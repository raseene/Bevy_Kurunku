# くるくるアクションパズル　くるんくる～ぱ

Rust製のゲームエンジン [Bevy](https://bevyengine.org)で作られたミニゲームです。

![Screenshot](http://raseene.asablo.jp/blog/img/2021/12/19/616840.jpg)

## Web対応版

　https://raseene.github.io/Bevy_Kurunku/

ブラウザ等、環境によっては動かないかもしれません。

## 必要な環境

* cargo-make：タスクランナー

```
cargo install cargo-make
```

* WASMサポートを追加

```
rustup target install wasm32-unknown-unknown
```

* wasm-server-runner：ブラウザで実行

```
cargo install wasm-server-runner
```

## ビルド

ネイティブ用

```
cargo run
```

Web用（デバッグ）

```
cargo run --target wasm32-unknown-unknown
```
表示されたアドレスを、ブラウザから起動します。

Web用（リリース）

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/kurunku.wasm
```
index.html
out/kurunku.js
out/kurunku_bg.wasm
assets/*.*
以上のファイルを、Web サーバーに配置します。

## ライセンス

このプロジェクトのソースコードは、以下の２つのライセンスの下に公開されています。

* MIT License(http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

