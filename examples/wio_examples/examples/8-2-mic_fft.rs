//! 8-2 マイク音声の信号処理をする
//! マイクから入力した音声をフーリエ変換してパワースペクトラムを表示します
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 8-2-mic_fft --features app --release
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use heapless::consts::*;
use heapless::Vec;
use micromath::F32Ext;
use wio::entry;
use wio::hal::adc::{FreeRunning, InterruptAdc};
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::time::Hertz;
use wio::pac::{interrupt, CorePeripherals, Peripherals, ADC1};
use wio::prelude::*;
use wio::Pins;

use eg::{egrectangle, pixelcolor::Rgb565, primitive_style};
use eg::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics as eg;

// main() 関数とADCの割り込みハンドラで共有するリソース
struct Ctx {
    adc: InterruptAdc<ADC1, FreeRunning>,
    buffers: [SamplingBuffer; 2], // ADC結果のバッファ2面分
    // 現在ADC結果取り込み先のバッファへの参照
    sampling_buffer: Option<&'static mut SamplingBuffer>,
    // 現在信号処理中のバッファへの参照
    processing_buffer: Option<&'static mut SamplingBuffer>,
}

static mut CTX: Option<Ctx> = None;

const AVERAGING_FACTOR: u32 = 4; // 平均化フィルタのサンプル点数
const FFT_POINTS: usize = 256; // FFTをするサンプル点数
const ADC_SAMPLING_RATE: f32 = 83333.0; // ADCのサンプリングレート
#[allow(dead_code)]
// 平均化フィルタ後のサンプリングレート
const SAMPLING_RATE: f32 = ADC_SAMPLING_RATE / AVERAGING_FACTOR as f32;
const AMPLITUDE: f32 = 4096.0; // サンプル値の最大振幅

type SamplingBuffer = heapless::Vec<f32, U256>; //サンプリングバッファの型

// f32::max,f32::minが
// プラットフォームのライブラリとしてfmaxf,fminfがあることを前提としているが、
// 現在の環境にはfmaxf,fminfがないので、最低限のものを実装しておく
// Cから呼び出せる形式でなければならないので、`#[no_mangle]`を付ける
#[no_mangle]
fn fminf(a: f32, b: f32) -> f32 {
    match a.partial_cmp(&b) {
        None => a,
        Some(core::cmp::Ordering::Less) => a,
        Some(core::cmp::Ordering::Equal) => a,
        Some(core::cmp::Ordering::Greater) => b,
    }
}
#[no_mangle]
fn fmaxf(a: f32, b: f32) -> f32 {
    match a.partial_cmp(&b) {
        None => a,
        Some(core::cmp::Ordering::Less) => b,
        Some(core::cmp::Ordering::Equal) => b,
        Some(core::cmp::Ordering::Greater) => a,
    }
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut sets = Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // フリーランニングモードでADCを動かすようにInterruptAdc型を構築する
    let (microphone_adc, mut microphone_pin) = sets.microphone.init(
        peripherals.ADC1,
        &mut clocks,
        &mut peripherals.MCLK,
        &mut sets.port,
    );
    let mut microphone_adc: InterruptAdc<_, FreeRunning> =
        InterruptAdc::from(microphone_adc);

    // ADCの変換処理を開始する
    microphone_adc.start_conversion(&mut microphone_pin);

    // デバッグ用UARTを初期化する
    let mut serial = sets.uart.init(
        &mut clocks,
        Hertz(115200u32),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // 画面を初期化する
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            60.mhz(),
            &mut delay,
        )
        .unwrap();

    // 共有リソースを初期化する
    unsafe {
        CTX = Some(Ctx {
            adc: microphone_adc,
            buffers: [Vec::new(), Vec::new()],
            sampling_buffer: None,
            processing_buffer: None,
        });
        // 2面分のサンプリングバッファを取り込み用と処理用にそれぞれ割り当てる
        let mut ctx = CTX.as_mut().unwrap();
        let (first, rest) = ctx.buffers.split_first_mut().unwrap();
        ctx.sampling_buffer = Some(first);
        ctx.processing_buffer = Some(&mut rest[0]);
    }

    // ADC変換完了割り込み(RESRDY)を有効にしてサンプリングを開始する
    writeln!(&mut serial, "start").unwrap();

    unsafe { NVIC::unmask(interrupt::ADC1_RESRDY); }

    let button_restart =
        sets.buttons.button1.into_floating_input(&mut sets.port);
    let button_stop =
        sets.buttons.button2.into_floating_input(&mut sets.port);

    // FFTの窓関数としてHann窓を使うので係数を計算しておく
    // 振幅の正規化用に最大振幅で割っておく
    let mut hann_factor = [0f32; FFT_POINTS];
    for i in 0..FFT_POINTS {
        use core::f32::consts::PI;
        hann_factor[i] = 0.5f32
            * (1f32 - (PI * 2.0f32 * i as f32 / FFT_POINTS as f32).cos())
            / AMPLITUDE;
    }
    let hann_factor = hann_factor;

    // 画面のスペクトラム表示領域の内容を消す
    const SCREEN_WIDTH: i32 = 320;
    const SCREEN_HEIGHT: i32 = 240;
    fn clear_screen<T: embedded_graphics::DrawTarget<Rgb565>>(
        display: &mut T,
    ) -> Result<(), T::Error> {
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
            style = primitive_style!(fill_color = Rgb565::BLACK)
        )
        .draw(display)
    };
    clear_screen(&mut display).unwrap();

    const BAR_WIDTH: i32 = 2;
    const REAL_POINTS: usize = FFT_POINTS / 2;
    const NUMBER_OF_BARS: usize = REAL_POINTS;
    const DRAW_AREA_WIDTH: i32 =
        BAR_WIDTH * (NUMBER_OF_BARS as i32 + 1);
    let mut prev_bar_position = [0u8; NUMBER_OF_BARS as usize];
    let mut stop_req = false;
    let mut stop_ack = false;
    loop {
        if button_stop.is_low().unwrap() {
            // 停止ボタンが押されていたら、停止要求をする
            stop_req = true;
        }
        // safe: processing_bufferは、mainループでclearするまで
        //       割り込みハンドラが触らないので安全
        let processing_buffer = unsafe {
            let ctx = CTX.as_mut().unwrap();
            ctx.processing_buffer.as_mut().unwrap()
        };
        let len = processing_buffer.len();
        let cap = processing_buffer.capacity();

        // 処理対象バッファにFFT点数分のサンプルデータが入っている？
        if len == cap {
            for i in 0..FFT_POINTS {
                processing_buffer[i] *= hann_factor[i];
            }
            // 実部のみの入力に対する256点FFTを実行する
            let result =
                microfft::real::rfft_256(processing_buffer.as_mut());

            // スペクトラムを描画する
            let offset_top = 0;
            let offset_left = (SCREEN_WIDTH - DRAW_AREA_WIDTH) / 2;
            let area_height = SCREEN_HEIGHT;
            for (step, spectrum) in result.iter().enumerate() {
                // パワーの計算
                let power = spectrum.norm_sqr()
                    / (FFT_POINTS * FFT_POINTS) as f32;
                // 対数にする
                let relative_power = if power <= 0.0 {
                    core::f32::NEG_INFINITY
                } else {
                    power.log10() * 10.0
                };
                // 値からY座標を計算
                let height = -(((relative_power + 50.0) * 5.0)
                    .round()
                    .max(-area_height as f32)
                    .min(0.0) as i32);
                // Y座標から色を計算
                let intensity = (height * 255) / (SCREEN_HEIGHT / 2);
                let red = if height < SCREEN_HEIGHT / 2 {
                    255 - intensity
                } else {
                    0
                };
                let green = if height < SCREEN_HEIGHT / 2 {
                    intensity
                } else {
                    511 - intensity
                };
                let blue = if height < SCREEN_HEIGHT / 2 {
                    0
                } else {
                    intensity - 256
                };

                // 前回のバーを消す
                let start_x = offset_left + step as i32 * BAR_WIDTH;
                let end_x = offset_left + (step + 1) as i32 * BAR_WIDTH;
                let prev_y = prev_bar_position[step] as i32;
                egrectangle!(
                    top_left = (start_x, prev_y),
                    bottom_right =
                        (end_x, (prev_y + 2).min(area_height - 1)),
                    style =
                        primitive_style!(fill_color = Rgb565::BLACK)
                )
                .draw(&mut display)
                .unwrap();
                // 今回のバーを書く
                if stop_req {
                    // 停止要求時は見やすくするために棒グラフにする
                    egrectangle!(
                        top_left = (start_x, offset_top + height),
                        bottom_right = (end_x, area_height - 1),
                        style = primitive_style!(
                            fill_color = Rgb888::new(
                                red as u8,
                                green as u8,
                                blue as u8
                            )
                            .into()
                        )
                    )
                    .draw(&mut display)
                    .unwrap();
                } else {
                    // 普段は棒を描くのは遅いので点だけ描く
                    egrectangle!(
                        top_left = (start_x, offset_top + height),
                        bottom_right = (
                            end_x,
                            (offset_top + height + 2)
                                .min(area_height - 1)
                        ),
                        style = primitive_style!(
                            fill_color = Rgb888::new(
                                red as u8,
                                green as u8,
                                blue as u8
                            )
                            .into()
                        )
                    )
                    .draw(&mut display)
                    .unwrap();
                }
                prev_bar_position[step] = (offset_top + height) as u8;
            }

            // 処理が終わったので処理用バッファをクリアする
            processing_buffer.clear();

            // 停止要求が来ていたら、処理していたことを通知
            stop_ack = stop_req;
        }

        if stop_ack {
            // 停止要求に対する処理が完了したので
            // リスタートボタンが押されるまで停止
            stop_req = false;
            stop_ack = false;
            while !button_restart.is_low().unwrap() {}
            // 画面クリア
            clear_screen(&mut display).unwrap();
        }
    }
}

#[interrupt]
fn ADC1_RESRDY() {
    static mut AVERAGE: f32 = 0.0; // 平均値
    static mut AVERAGE_COUNT: u32 = 0; // 平均値計算時のサンプル数カウント
    unsafe {
        let ctx = CTX.as_mut().unwrap();
        if let Some(sample) = ctx.adc.service_interrupt_ready() {
            // サンプルデータがあれば平均値計算のために積算する
            *AVERAGE += sample as f32;
            *AVERAGE_COUNT += 1;
            if *AVERAGE_COUNT == AVERAGING_FACTOR {
                // 平均値計算回数分のサンプルデータを積算した
                let sampling_buffer =
                    ctx.sampling_buffer.as_mut().unwrap();
                if sampling_buffer.len() == sampling_buffer.capacity() {
                    // サンプリングバッファがいっぱいなので処理用バッファが空
                    // つまり処理が終わっているなら入れ替える
                    if ctx.processing_buffer.as_mut().unwrap().len()
                        == 0
                    {
                        core::mem::swap(
                            &mut ctx.processing_buffer,
                            &mut ctx.sampling_buffer,
                        );
                    }
                } else {
                    // サンプリングバッファに平均値を追加する
                    let _ = sampling_buffer
                        .push(*AVERAGE / (AVERAGING_FACTOR as f32));
                }
                // 積算カウントを0に戻す
                *AVERAGE_COUNT = 0;
                *AVERAGE = 0.0;
            }
        }
    }
}
