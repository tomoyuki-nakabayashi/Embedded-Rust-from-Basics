//! 6-3 シリアル入出力/UARTのサンプルコードです。
//! ホストPCのシリアルターミナルに
//! ```text
//! hello world
//! this is UART example!
//! ```
//! と出力します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-3-uart
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use wio::hal::clock::GenericClockController;
use wio::pac::Peripherals;
use wio::prelude::*;
use wio::{entry, Pins, Sets};

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

    // 「hello world」と出力する
    for c in b"hello world\n".iter() {
        nb::block!(serial.write(*c)).unwrap();
    }
    // 「this is UART example!」と出力する
    writeln!(&mut serial, "this is {} example!", "UART").unwrap();

    loop {}
}
