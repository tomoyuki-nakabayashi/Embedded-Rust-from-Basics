//! 9-2 embedded-graphicsの基礎のサンプルコードです。
//! ferris の画像を描画します。
//!
//! ## 実行方法
//! ```
//! $ cargo run --example image_raw
//! ```

use embedded_graphics::{
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_graphics_simulator::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("ferris", &output_settings);

    let raw =
        ImageRawLE::new(include_bytes!("./assets/ferris.raw"), 86, 64);
    let image = Image::new(&raw, Point::new(32, 32));
    image.draw(&mut display)?;

    window.show_static(&display);
    Ok(())
}
