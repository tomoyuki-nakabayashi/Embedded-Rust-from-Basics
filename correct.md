# 正誤表

## 重要な正誤情報

### 第2刷で修正

サンプルコードの実行に支障のある正誤情報です。

- p.286 7-4 Wio TerminalのLCDにHello World!で `can't find crate for std` のビルドエラー
  - 正: `wio_splash/src/lib.rs` の一番上の行に `#![no_std]` を書く
- p.291 Webシミュレータで動作するWioSplashのコードがビルドエラーになる

誤
```rust
use embedded_graphics_web_simulator::*;
```
正
```rust
use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
```

## 些細な正誤情報

- p.208 第1段落 4行目
  - 誤: Rustの安全性の補償範囲外であるため
  - 正: Rustの安全性の保証範囲外であるため
- p.209 CountDownトレイトを説明するソースコード
  - 誤: カウントダウンタイマを現すトレイト
  - 正: カウントダウンタイマを表すトレイト
- p.214 第2段落 4行目
  - 誤: Wio Terminalファームウェアを書き込むと
  - 正: Wio Terminalにファームウェアを書き込むと
- p.224 第3段落 1行目
  - 誤: マイコンにはADCが内蔵されているもの多く
  - 正: マイコンにはADCが内蔵されているものも多く
- p.226 atsamd/boards/wio_terminal/src/sensors.rsのソースコード解説
  - 誤: PD1をACD入力として初期化し
  - 正: PD1をADC入力として初期化し
- p.232 第2段落 5行目
  - 誤: I2Cはクロック信号で同期を行う同期式のリアル通信方式です。
  - 正: I2Cはクロック信号で同期を行う同期式のシリアル通信方式です。
- p.283 サンプル実行のコマンド
  - 誤: cargo hf2 --example hello_lcd
  - 正: cargo hf2 --example 7-4-hello_lcd
- p.284 サンプル実行のコマンド
  - 誤: cargo hf2 --example hello_lcd --release
  - 正: cargo hf2 --example 7-4-hello_lcd --release

### 第2刷で修正

- p.51 第1段落 2行目
  - 誤: 値を変できない
  - 正: 値を変更できない
- p.84 第3段落 2行目
  - 誤: temparature
  - 正: temperature
- p.94 2つ目のソースコード中のコメント
  - 誤: i32 に型推論されため
  - 正: i32 に型推論されるため
- p.105 第2段落 1行目
  - p.43 でも説明したようように
  - p.43 でも説明したように
- p.227 ソースコードのタイトル
  - 誤: OneSho
  - 正: OneShot
