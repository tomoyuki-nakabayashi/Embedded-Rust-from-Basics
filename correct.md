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
