//! 9-2 embedded-graphicsの基礎のサンプルコードです。
//! 様々な基本図形を描画します。
//!
//! ## 実行方法
//! ```
//! $ cargo run --example primitives
//! ```

// `DrawTarget` や `Drawable` といったトレイトをまとめてインポートします
use embedded_graphics::prelude::*;
// RGB565 のカラーフォーマット、基本図形、スタイルをインポートします
use embedded_graphics::{pixelcolor::Rgb565, primitives::*, style::*};
// シミュレータをインポートします
use embedded_graphics_simulator::*;

fn main() -> Result<(), core::convert::Infallible> {
    // シミュレータディスプレイを作成します
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("draw primitives", &output_settings);

    // 緑色の直線を描画します
    Line::new(Point::new(50, 20), Point::new(270, 220))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
        .draw(&mut display)?;

    // 赤色の太線で囲われた円を描画します
    // `Circle` は中心の点と、半径を引数で渡します
    Circle::new(Point::new(50, 200), 20)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5))
        .draw(&mut display)?;

    // 青色で塗りつぶされた三角形を描画します
    // `PrimitiveStyle::with_fill()` 関数を使って図形を塗りつぶします
    Triangle::new(
        Point::new(200, 20),
        Point::new(170, 45),
        Point::new(300, 150),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
    .draw(&mut display)?;

    // シアン色の太線で囲われていて、黄色で塗りつぶされた長方形を描画します
    // まず、複雑なスタイルを `PrimitiveStyleBuilder::new()` で作成します
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(10)
        .stroke_color(Rgb565::CYAN)
        .fill_color(Rgb565::YELLOW)
        .build();
    // `Rectangle` は、左上の点と右下の点の位置を引数に渡します
    Rectangle::new(Point::new(100, 100), Point::new(220, 140))
        .into_styled(style)
        .draw(&mut display)?;

    window.show_static(&display);

    Ok(())
}
