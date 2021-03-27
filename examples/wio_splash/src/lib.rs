#![no_std]

use embedded_graphics::{
    egcircle, egrectangle, egtext,
    fonts::Font8x16,
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
    primitive_style, text_style,
};

// ケースの色
const CASE_COLOR: Rgb565 = Rgb565::WHITE;
// ボタンの色
const BUTTON_COLOR: Rgb565 = Rgb565::BLUE;
// ケース、ボタンを除く部分の色
const BG_COLOR: Rgb565 = Rgb565::BLACK;

/// Wio TerminalのLCDにスプラッシュ画面を描画するオブジェクトです。
/// この実装は`Rgb565`カラーフォーマット専用の実装です。
pub struct WioSplash<'a> {
    // テキストの色です。
    text_color: Rgb565,
    // LCDの中央に表示する画像です。
    image: ImageRawLE<'a, Rgb565>,
}

impl<'a> WioSplash<'a> {
    /// 新しいスプラッシュ画面オブジェクトを作成します。
    /// 好みの色、画像を指定できます。
    pub fn new(
        text_color: Rgb565,
        image: ImageRawLE<'a, Rgb565>,
    ) -> Self {
        Self { text_color, image }
    }

    // ケースを描画するヘルパー関数
    fn draw_case<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Rgb565>,
    {
        // 筐体とディスプレイの塗りつぶし
        egrectangle!(
            top_left = (20, 20),
            bottom_right = (300, 220),
            style = primitive_style!(
                stroke_width = 5,
                stroke_color = CASE_COLOR,
                fill_color = BG_COLOR,
            )
        )
        .draw(display)?;
        // 筐体下側は白い部分が太い
        egrectangle!(
            top_left = (20, 180),
            bottom_right = (300, 220),
            style = primitive_style!(fill_color = CASE_COLOR)
        )
        .draw(display)?;

        // スピーカー用の穴を4つ描画
        for i in 0..4 {
            egrectangle!(
                top_left = (40 + i * 15, 190),
                bottom_right = (45 + i * 15, 210),
                style = primitive_style!(fill_color = BG_COLOR)
            )
            .draw(display)?;
        }

        Ok(())
    }

    // ボタンを描画するヘルパー関数
    fn draw_buttons<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Rgb565>,
    {
        // 上部のボタンを3つ描画
        for i in 0..3 {
            egrectangle!(
                top_left = (40 + i * 60, 15),
                bottom_right = (80 + i * 60, 20),
                style = primitive_style!(fill_color = BUTTON_COLOR)
            )
            .draw(display)?;
        }

        // 5-wayスイッチを描画
        egcircle!(
            center = (260, 180),
            radius = 20,
            style = primitive_style!(fill_color = BUTTON_COLOR)
        )
        .draw(display)
    }

    // 画像を画面中央に描画するヘルパー関数
    fn draw_image<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Rgb565>,
    {
        // 画面サイズと画像サイズから、画像を中央に表示するための座標を計算
        use core::convert::TryFrom;
        let (w, h) = display.size().into();
        let (iw, ih) = (self.image.width(), self.image.height());
        let (x, y) = (
            i32::try_from(w / 2 - iw / 2).unwrap(),
            i32::try_from(h / 2 - ih / 2).unwrap(),
        );
        let top_left = Point::new(x, y);
        let image = Image::new(&self.image, top_left);
        image.draw(display)
    }
}

/// Drawableトレイトを実装します。
/// `Rgb565`専用の実装です。
impl<'a> Drawable<Rgb565> for WioSplash<'a> {
    fn draw<D>(self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Rgb565>,
    {
        self.draw_case(display)?;
        self.draw_buttons(display)?;
        self.draw_image(display)?;

        egtext!(
            text = "Booting Wio Terminal...",
            top_left = (30, 30),
            style = text_style!(
                font = Font8x16,
                text_color = self.text_color,
            )
        )
        .draw(display)
    }
}
