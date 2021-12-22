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

## ビルド

Web用

```
cargo make serve
```
ブラウザから、http://localhost:4000/で起動します。

ネイティブ用

```
cargo make run
```

## ライセンス

このプロジェクトのソースコードは、以下の２つのライセンスの下に公開されています。

* MIT License(http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

