//! 6-7 加速度センサ/I2Cのサンプルコードです。
//! I2CでLIS3DHからデバイスIDを取得します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-7-i2c_read_device_id
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::gpio::*;
use wio::hal::sercom::*;
use wio::pac::Peripherals;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut sets = wio::Pins::new(peripherals.PORT).split();
    // UARTドライバオブジェクトを初期化する
    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // I2Cドライバオブジェクトを初期化する
    let gclk0 = &clocks.gclk0();
    let mut i2c: I2CMaster4<
        Sercom4Pad0<Pa13<PfD>>,
        Sercom4Pad1<Pa12<PfD>>,
    > = I2CMaster4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        400.khz(),
        peripherals.SERCOM4,
        &mut peripherals.MCLK,
        sets.accelerometer.sda.into_pad(&mut sets.port),
        sets.accelerometer.scl.into_pad(&mut sets.port),
    );

    // LIS3DHのデバイスIDを取得する
    let slave_addr = 0x18;
    let who_am_i_reg = 0x0F;
    let mut data: [u8; 1] = [0];
    // `slave_addr`に`who_am_i_reg`を書き込んで、`data`に1バイト受信する
    i2c.write_read(slave_addr, &[who_am_i_reg], &mut data)
        .unwrap();

    writeln!(&mut serial, "device id = 0x{:x}", data[0]).unwrap();

    loop {}
}
