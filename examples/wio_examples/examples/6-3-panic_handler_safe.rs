//! 6-3 シリアル入出力/UARTのサンプルコードです。
//! MutexとRefCellを使って安全にグローバル変数を共有するパニックハンドラ実装です。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-3-panic_handler_safe
//! ```

#![no_std]
#![no_main]

use wio_terminal as wio;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use core::panic::PanicInfo;
use cortex_m::interrupt::{self, Mutex};
use wio::hal::clock::GenericClockController;
use wio::hal::gpio::*;
use wio::hal::sercom::*;
use wio::pac::Peripherals;
use wio::prelude::*;
use wio::{entry, Pins, Sets};

// 絶対に初期化しないといけないので、いったんNoneを持つRefCellで初期化する
static UART: Mutex<RefCell<Option<
    UART2<Sercom2Pad1<Pb27<PfC>>, Sercom2Pad0<Pb26<PfC>>, (), ()>
>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // interrupt::free()関数でクリティカルセクション（Critical Section）に
    // 入らないとMutexの中身を使えない
    // `cs: &CriticalSection`はクロージャ内でのみ使えるマーカーオブジェクト
    // クリティカルセクションで実行する処理はクロージャで渡している
    // 1行目はNoneをSomeで置き換える
    interrupt::free(|cs| UART.borrow(cs).replace(Some(serial)));
    // グローバル変数UARTに格納したUARTドライバオブジェクトの可変参照を取得して
    // hello worldを出力する
    interrupt::free(|cs| {
        if let Some(ref mut serial) =
            UART.borrow(cs).borrow_mut().deref_mut()
        {
            writeln!(serial, "hello world").unwrap();
        }
    });

    let none: Option<usize> = None;
    none.unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::free(|cs| {
        if let Some(ref mut serial) =
            UART.borrow(cs).borrow_mut().deref_mut()
        {
            // パニックハンドラ内でさらにパニックしないように、unwrap()しない
            let _ = writeln!(serial, "panic: {}", info);
        }
    });

    loop {}
}
