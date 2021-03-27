//! 9-2 embedded-graphicsの基礎のサンプルコードです。
//! 緑色の線を1本、ホストPCのシミュレータに描画します。
//!
//! ## 実行方法
//! ```
//! $ cargo run --example line
//! ```

// `DrawTarget` や `Drawable` といったトレイトをまとめてインポートします
use embedded_graphics::prelude::*;
// RGB565 のカラーフォーマット、基本図形、スタイルをインポートします
use embedded_graphics::{pixelcolor::Rgb565, primitives::*, style::*};
// シミュレータをインポートします
use embedded_graphics_simulator::*;

fn main() {
    // 320x240ドットのRGB565カラーディスプレイを作成します
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(320, 240));

    // ディスプレイを表示するウィンドウを作成します
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("draw a line", &output_settings);

    // 緑色の線を1本描画します
    // `start`と`end`という2つの点を用意し、その2点を結ぶ直線を作ります
    // さらに、その直線に緑色で太さ1のstyleを適用します
    let start = Point::new(50, 20);
    let end = Point::new(270, 220);
    let style = PrimitiveStyle::with_stroke(Rgb565::GREEN, 1);
    // `Primitives::new(...).into_styled(...).draw(&mut display)`のパターン
    Line::new(start, end)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    // ウィンドウにディスプレイを表示します
    window.show_static(&display);
}
