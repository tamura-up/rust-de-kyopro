#!/bin/bash

# コードを提出用にバンドルするスクリプト
# バンドル時のビルド等時間短縮のため、あらかじめ用意したバンドル用のプロジェクトディレクトリで cargo-equip でバンドルする。
# 
# usage
# `a.rs` をバンドルする例を以下に示す
# ## atcoder の場合
#    -a オプションをつける
#    contest-bundle.sh -a a
#
# ## atcoder 以外の場合
#    contest-bundle.sh a

set -e

# バンドルするためのコンテストディレクトリ
# このディレクトリの `/src/bin/a.rs` にソースファイルをコピーし、 cargo-equip でバンドルする
# バンドルした結果はクリップボードにコピーされる
bundle_dir=${HOME}/kyopro/codeforces/_bundle

while (( $# > 0 ))
do
  case $1 in
    # atcoder フラグ。指定がある場合は `--exclude-atcoder-202301-crates` を付与して、提出時に不要な crate のバンドルをしない
    -a | --atcoder)
      exclude_atcoder_crates="--exclude-atcoder-202301-crates"
      ;;
    -l | --leetcode)
      leetcode=1
      ;;
    *)
    # ファイル名受取り
      source_file=$(find ./ -name "${1}.rs" | head -n1) ;;
  esac
  shift
done

if [ ! -f ${source_file} ]; then
    echo "ファイルが存在しません" >&2
    exit 1
fi

# bundle 実行
cp ${source_file} ${bundle_dir}/src/bin/a.rs
pushd ${bundle_dir}
a=$(cargo equip $exclude_atcoder_crates --resolve-cfgs --remove docs comments --minify libs  --no-rustfmt --no-check  --bin a)

if [ -n "$leetcode" ]; then
    # main 関数を削除などの整形
    a=$(echo -e "$a" | sed -E '1s/^#!.*//' | sed -E 's/^fn main/fn _main/')
fi

echo -e "$a" | xsel --clipboard --input

echo ""
echo '""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""'
echo 'bundled soruce code:' ${source_file}
