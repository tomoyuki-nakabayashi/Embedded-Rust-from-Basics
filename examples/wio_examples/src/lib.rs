#![no_std]
#![allow(dead_code)] // 使用しないメソッドでコンパイラが警告を出さないようにします

use wio::hal::gpio::*; // GPIOの構造体やトレイトをインポートします
use wio::prelude::*;
use wio_terminal as wio;

// Wio TerminalのユーザーLEDドライバです
pub struct Led {
    pin: Pa15<Output<PushPull>>,
}

impl Led {
    // デフォルトモードのPA15ピンを、出力モードに移行します
    pub fn new(pin: Pa15<Input<Floating>>, port: &mut Port) -> Led {
        Led {
            pin: pin.into_push_pull_output(port),
        }
    }

    // LEDを点灯します
    pub fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    // LEDを消灯します
    pub fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }

    // LEDが点灯しているときは消灯し、消灯しているときは点灯します
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}
