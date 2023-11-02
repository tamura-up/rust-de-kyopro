競技プログラミングで学ぶ Rust

## libs

自作ライブラリは `/libs` に配置し、 cargo-equip でバンドルすることを想定して作成する。

### TODO
[usage](https://github.com/qryxip/cargo-equip#usage) の 6. のようにクレートに分割するようにしているが、作り方が正しいのかよくわかってない

### クレート作るときのメモ

クレートを cargo-equip でバンドルするためにはライセンス記載が必要

#### ライセンスの記載

最低限以下を行う

- LICENSE ファイルを配置
- `Cargo.toml` に `license = "CC0-1.0"` を 追加

#### cargo-equip links

- https://github.com/qryxip/cargo-equip
- https://qiita.com/qryxip/items/b945c0adbd62ce5f1f3d#%E3%83%A9%E3%82%A4%E3%83%96%E3%83%A9%E3%83%AA%E3%81%AE%E5%88%B6%E7%B4%84
