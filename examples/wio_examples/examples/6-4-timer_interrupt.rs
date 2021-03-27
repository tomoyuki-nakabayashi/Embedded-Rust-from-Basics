//! 6-4 タイマ/割り込みのサンプルコードです。
//! 割り込みでLチカしながら、ホストPCのシリアルターミナルに入力した内容をそのまま出力します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-4-timer_interrupt
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use cortex_m::peripheral::NVIC;
use wio::hal::clock::GenericClockController;
use wio::hal::hal::serial::*;
use wio::hal::timer::TimerCounter;
use wio::pac::{interrupt, Peripherals, TC3};
use wio::prelude::*;
use wio::{entry, Pins, Sets};
use wio_examples::Led;

// main()関数と割り込みハンドラとで共有するリソース
struct Ctx {
    led: Led,
    tc3: TimerCounter<TC3>,
}
static mut CTX: Option<Ctx> = None;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    // クロックを初期化する
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    // UARTドライバオブジェクトを初期化する
    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // 2MHzのクロックを取得する
    let gclk5 = clocks
        .get_gclk(wio::pac::gclk::pchctrl::GEN_A::GCLK5)
        .unwrap();
    // TC3へのクロックを2MHzにする
    let timer_clock = clocks.tc2_tc3(&gclk5).unwrap();
    // TC3ドライバオブジェクトを初期化する
    let mut tc3 = TimerCounter::tc3_(
        &timer_clock,
        peripherals.TC3,
        &mut peripherals.MCLK,
    );

    // 割り込みコントローラで、TC3の割り込み通知を有効化する
    unsafe {
        NVIC::unmask(interrupt::TC3);
    }

    // 1秒のカウントを開始して、TC3が割り込みが発生するようにする
    tc3.start(1.s());
    tc3.enable_interrupt();

    // 割り込みハンドラと共有するリソースを格納する
    unsafe {
        CTX = Some(Ctx {
            led: Led::new(sets.user_led, &mut sets.port),
            tc3,
        });
    }

    // シリアルターミナルにechoし続ける
    loop {
        // データを1ワード受信するとifブロック内に入る
        if let Ok(c) = nb::block!(serial.read()) {
            // 受信したデータをそのまま送信する
            nb::block!(serial.write(c)).unwrap();
        }
    }
}

// TC3の割り込みハンドラ
// 1秒ごとに呼ばれる
#[interrupt]
fn TC3() {
    unsafe {
        let ctx = CTX.as_mut().unwrap();
        // 次のカウントを開始する
        ctx.tc3.wait().unwrap();
        ctx.led.toggle();
    }
}
