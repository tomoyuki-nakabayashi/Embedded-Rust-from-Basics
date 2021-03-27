//! 9-2 embedded-graphicsの基礎のサンプルコードです。
//! `hello world!`を描画します。
//!
//! ## 実行方法
//! ```
//! $ cargo run --example hello_world
//! ```

use embedded_graphics::{
    fonts::*, pixelcolor::Rgb565, prelude::*, style::*,
};
use embedded_graphics_simulator::*;

fn main() -> Result<(), core::convert::Infallible> {
    // シミュレータディスプレイを作成します
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("hello world", &output_settings);

    // Text は描画する文字列と、描画開始位置となる左上の点を引数に渡します
    Text::new("hello world!", Point::new(0, 0))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display)?;

    window.show_static(&display);
    Ok(())
}
