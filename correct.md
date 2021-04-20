# 正誤表

## 重要な正誤情報

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

- p.227 ソースコードのタイトル
  - 誤: OneSho
  - 正: OneShot
