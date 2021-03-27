//! 6-6 光センサ/ADCのサンプルコードです。
//! 光センサで読み取った値をシリアルターミナルに出力します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-6-light_sensor
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use nb;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    // クロックを初期化する
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

    // 光センサ読み取り用のADCとピンを初期化する
    let (mut light, mut pd1) = sets.light_sensor.init(
        peripherals.ADC1,
        &mut clocks,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // UARTドライバオブジェクトを初期化する
    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    loop {
        // ADC入力を1秒に1回取得して、UARTに出力する
        let value: u16 = nb::block!(light.read(&mut pd1)).unwrap();
        writeln!(&mut serial, "light sensor value: {}", value).unwrap();
        delay.delay_ms(1000u16);
    }
}
