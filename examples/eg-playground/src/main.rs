use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, Rectangle, Triangle},
    style::{PrimitiveStyle, PrimitiveStyleBuilder},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() {
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("draw primitives", &output_settings);

    Line::new(Point::new(50, 20), Point::new(270, 220))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
        .draw(&mut display)
        .unwrap();

    Circle::new(Point::new(50, 200), 20)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5))
        .draw(&mut display)
        .unwrap();

    Triangle::new(
        Point::new(200, 20),
        Point::new(170, 45),
        Point::new(300, 150),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
    .draw(&mut display)
    .unwrap();

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(10)
        .stroke_color(Rgb565::CYAN)
        .fill_color(Rgb565::YELLOW)
        .build();
    Rectangle::new(Point::new(100, 100), Point::new(220, 140))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    window.show_static(&display);
}
