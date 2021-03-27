//! 7-4 Wio TerminalのLCDにHello World!のサンプルコードです。
//! Wio Terminal の LCD に `Hello Wio Terminal!` と表示します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 7-4-hello_lcd
//! ```

#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::{fonts::*, pixelcolor::*, prelude::*, primitives::*, style::*};
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

    // LCDを黒色で塗りつぶす
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    let background =
        Rectangle::new(Point::new(0, 0), Point::new(319, 239))
            .into_styled(style);
    background.draw(&mut display).unwrap();

    // 画面情報に「Hello Wio Terminal!」と表示する
    Text::new("Hello Wio Terminal!", Point::new(30, 30))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display)
        .unwrap();

    loop {}
}
