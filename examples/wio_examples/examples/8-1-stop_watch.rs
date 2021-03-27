//! 8-1 ストップウォッチをつくる のサンプルコードです。
//! Wio Terminalをストップウォッチとして使えます。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 8-1-stop_watch --features app --release
//! ```

#![no_std]
#![no_main]

use cortex_m::peripheral::NVIC;
use wio_terminal as wio;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use cortex_m::interrupt::{self as cortex_interrupt, Mutex};
use eg::{
    egrectangle, egtext, fonts::Font24x32, pixelcolor::Rgb565,
    prelude::*, primitive_style, text_style,
};
use embedded_graphics as eg;
use heapless::consts::*;
use heapless::String;
use wio::hal::gpio::{Pb26, Pb27, PfC};
use wio::hal::pwm::Channel;
use wio::hal::sercom::{Sercom2Pad0, Sercom2Pad1, UART2};
use wio::hal::time::Hertz;
use wio::hal::{clock::GenericClockController, timer::TimerCounter};
use wio::hal::{delay::Delay, pwm::Tcc0Pwm};
use wio::pac::{interrupt, CorePeripherals, Peripherals, TC3};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

struct Ctx {
    timer_counter: u32,
    tc3: TimerCounter<TC3>,
}
static mut CTX: Option<Ctx> = None;

// デバッグ用UART
static UART: Mutex<RefCell<Option<
    UART2<Sercom2Pad1<Pb27<PfC>>, Sercom2Pad0<Pb26<PfC>>, (), ()>
>>> = Mutex::new(RefCell::new(None));

enum State {
    Initializing, // 初期化処理
    Idle,         // ストップウォッチ停止中
    Running,      // ストップウォッチ動作中（時刻カウント中）
}

// ブザーを鳴らすヘルパー関数
fn beep<P: Into<Hertz>>(
    buzzer_pwm: &mut Tcc0Pwm,
    delay: &mut Delay,
    frequency: P,
    duration_ms: u16,
) {
    buzzer_pwm.set_period(frequency.into());
    buzzer_pwm.enable(Channel::_4);
    delay.delay_ms(duration_ms);
    buzzer_pwm.disable(Channel::_4);
}

const SCREEN_WIDTH: i32 = 320; // 画面幅
const SCREEN_HEIGHT: i32 = 240; // 画面高さ

// 描画処理を各ステートで呼び出せるようにしておく
fn draw<T>(display: &mut T) -> Result<(), T::Error>
where
    T: embedded_graphics::DrawTarget<Rgb565>,
{
    // カウント表示エリアをクリアする
    const FONT_WIDTH: i32 = 24;
    const FONT_HEIGHT: i32 = 32;
    egrectangle!(
        top_left = (0, 0),
        bottom_right = (SCREEN_WIDTH - 1, FONT_HEIGHT),
        style = primitive_style!(fill_color = Rgb565::BLACK)
    )
    .draw(display)?;

    // 現在のタイムスタンプを取得する
    let counter = unsafe { CTX.as_ref().unwrap().timer_counter };
    let elapsed_s = (counter as f32) / 16.0; // カウンタから経過秒数に変換

    // タイムスタンプを描画する
    let mut textbuffer = String::<U256>::new();
    write!(&mut textbuffer, "{:.2}", elapsed_s).unwrap();
    
    // 座標計算用に文字列の長さを取得
    let length = textbuffer.len();
    // 右詰描画用に左の座標計算
    let left = SCREEN_WIDTH - (length as i32) * FONT_WIDTH;
    egtext!(
        text = textbuffer.as_str(),
        top_left = (left, 0),
        style =
            text_style!(font = Font24x32, text_color = Rgb565::WHITE)
    )
    .draw(display)?;
    Ok(())
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

    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // パニックハンドラ用にUARTを初期化する（ストップウォッチの機能では不使用）
    let serial = sets.uart.init(
        &mut clocks,
        Hertz(115200u32),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );
    cortex_interrupt::free(|cs| UART.borrow(cs).replace(Some(serial)));

    // ブザーの初期化（TCC0を使ったPWM信号生成）
    let mut buzzer = sets.buzzer.init(
        &mut clocks,
        peripherals.TCC0,
        &mut peripherals.MCLK,
        &mut sets.port,
    );
    // デューティ比を0.5に音色変えたい場合は比率を変えるとよい
    let max_duty = buzzer.get_max_duty();
    buzzer.set_duty(Channel::_4, max_duty / 2);
    buzzer.disable(Channel::_4);

    // 時間を計測するためにタイマを初期化する
    // 正確に時間を計測するために、XOSC32K（外部32.768[kHz]水晶発振器）
    // を基準にしたクロックを使う必要がある
    // 以下の設定でGCLK6はXOSC32Kを基準とした32.768[kHz]のクロックとなる
    let gclk6 = clocks
        .configure_gclk_divider_and_source(
            wio::pac::gclk::pchctrl::GEN_A::GCLK6,
            1,
            wio::pac::gclk::genctrl::SRC_A::XOSC32K,
            false,
        )
        .unwrap();
    // GCLK6を使うTC2/TC3向けのクロック設定を構築
    let timer_clock = clocks.tc2_tc3(&gclk6).unwrap();
    // TC3を初期化
    let mut tc3 = wio::hal::timer::TimerCounter::tc3_(
        &timer_clock,
        peripherals.TC3,
        &mut peripherals.MCLK,
    );
    unsafe { NVIC::unmask(interrupt::TC3); }
    // 62.5[ms] = 1/16[s]周期のカウンタとしてTC3の動作を開始
    tc3.start(62500.us());

    unsafe { CTX = Some(Ctx { timer_counter: 0, tc3 }); }

    // LCDの初期化
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

    // LCDのクリア（全体を黒で塗りつぶす）
    egrectangle!(
        top_left = (0, 0),
        bottom_right = (SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
        style = primitive_style!(fill_color = Rgb565::BLACK)
    )
    .draw(&mut display)
    .unwrap();

    // ボタンのGPIOを初期化
    let button_start =
        sets.buttons.button3.into_floating_input(&mut sets.port);
    let button_stop =
        sets.buttons.button2.into_floating_input(&mut sets.port);
    let button_clear =
        sets.buttons.button1.into_floating_input(&mut sets.port);

    let mut state = State::Initializing;
    loop {
        match state {
            State::Initializing => {
                draw(&mut display).unwrap(); // 画面初期化
                state = State::Idle;// 停止中に移行
            }
            State::Idle => {
                if button_start.is_low().unwrap() {
                    // スタートボタンが押されたらTC3割り込みを有効にして
                    // ストップウォッチのカウントを開始する
                    unsafe {
                        CTX.as_mut().unwrap().tc3.enable_interrupt()
                    }
                    beep(&mut buzzer, &mut delay, 880.hz(), 200u16);
                    state = State::Running // 動作中に遷移
                } else if button_clear.is_low().unwrap() {
                    // クリアボタンが押されたらカウンタをクリアする
                    unsafe { CTX.as_mut().unwrap().timer_counter = 0 }
                    beep(&mut buzzer, &mut delay, 1760.hz(), 200u16);
                    draw(&mut display).unwrap(); // 画面更新
                }
            }
            State::Running => {
                if button_stop.is_low().unwrap() {
                    // 割り込みを無効にしてカウント停止する
                    unsafe {
                        CTX.as_mut().unwrap().tc3.disable_interrupt()
                    }
                    beep(&mut buzzer, &mut delay, 880.hz(), 50u16);
                    delay.delay_ms(50u16);
                    beep(&mut buzzer, &mut delay, 880.hz(), 100u16);
                    state = State::Idle; // 停止中に遷移
                }
                draw(&mut display).unwrap();
            }
        }
    }
}

/// TC3の割り込みハンドラ (62.5[ms]周期で呼ばれる)
#[interrupt]
fn TC3() {
    unsafe {
        let ctx = CTX.as_mut().unwrap();
        ctx.tc3.wait().ok();
        ctx.timer_counter += 1;
    }
}

use core::panic::PanicInfo;
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_interrupt::free(|cs| {
        if let Some(ref mut serial) =
            UART.borrow(cs).borrow_mut().deref_mut()
        {
            writeln!(serial, "panic: {}", info).ok();
        }
    });

    loop {}
}
