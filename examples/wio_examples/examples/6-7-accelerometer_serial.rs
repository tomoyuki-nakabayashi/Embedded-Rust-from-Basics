//! 6-7 加速度センサ/I2Cのサンプルコードです。
//! 1秒ごとに加速度センサから値を読み出します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-7-accelerometer_serial
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use accelerometer::{vector::F32x3, Accelerometer};
use core::fmt::Write;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

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

    let mut sets = wio::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // UARTドライバオブジェクトを初期化する
    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // 加速度センサドライバオブジェクトを初期化する
    let mut accel = sets.accelerometer.init(
        &mut clocks,
        peripherals.SERCOM4,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // デバイスIDを取得、0x33が格納されている
    let accel_id = accel.get_device_id().unwrap();
    writeln!(&mut serial, "Accelerometer ID: 0x{:X}", accel_id).unwrap();

    // 1秒ごとに加速度センサから読み取った値をシリアルに出力する
    loop {
        let F32x3 { x, y, z } = accel.accel_norm().unwrap();
        writeln!(&mut serial, "X:{:.2}, Y:{:.2}, Z:{:.2}", x, y, z)
            .unwrap();
        delay.delay_ms(1000u16);
    }
}
