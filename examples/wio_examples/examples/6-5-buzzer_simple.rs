//! 6-5 ブザー/PWMのサンプルコードです。
//! 一定の音がなり続けます
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-5-buzzer_simple
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins};

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
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut pins = Pins::new(peripherals.PORT);
    // ブザーはPins構造体のbuzzer_ctrフィールドとして定義されている
    let mut buzzer =
        pins.buzzer_ctr.into_push_pull_output(&mut pins.port);

    loop {
        buzzer.toggle();
        delay.delay_ms(10u8);
    }
}
