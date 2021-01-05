# mori97.github.io

自動デプロイ：
[![Build Status](https://travis-ci.com/mori97/mori97.github.io.svg?branch=main)](https://travis-ci.com/mori97/mori97.github.io)

簡単なCVや論文リストなどを載せるための自己紹介用の[ホームページ](https://mori97.github.io/)．
[Yew](https://yew.rs/docs/ja/)というRustのフロントエンドフレームワークで書きました．

## ビルド手順

ローカルでビルドするには`wasm-pack`をインストールする必要があります．

```
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f
```

ビルド：

```
$ wasm-pack build --target web --out-name wasm --out-dir ./static
```

## デプロイ

Travis CIで自動ビルド＆デプロイしています．(See `.travis.yml`)

