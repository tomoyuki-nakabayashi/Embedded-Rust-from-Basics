//! 7-4 Wio TerminalのLCDにHello World!のサンプルコードです。
//! Wio Terminal の LCD にスプラッシュ画面を表示します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 7-4-splash --features="splash" --release
//! ```

#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::{image::*, pixelcolor::*, prelude::*};
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins};
// WioSplash をインポートする
use wio_splash::WioSplash;

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
    let mut sets = Pins::new(peripherals.PORT).split();

    // ディスプレイドライバを初期化する
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            58.mhz(),
            &mut delay,
        )
        .unwrap();

    // スプラッシュ画面を描画する
    let raw =
        ImageRawLE::new(include_bytes!("./assets/ferris.raw"), 86, 64);
    let splash = WioSplash::new(Rgb565::GREEN, raw);
    splash.draw(&mut display).unwrap();

    loop {}
}
