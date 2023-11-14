#!/bin/bash
# バンドル用ディレクトリの Cargo.toml から dependencies セクションの内容を出力します

# 取得元の Cargo.toml ファイル
contest_cargo_toml=${HOME}/kyopro/codeforces/_bundle/Cargo.toml 

# dependencies を取得
top=$(grep -En "^\[dependencies\]" ${contest_cargo_toml} | sed -e "s/^\([0-9]\+\).*/\1/g") 
top=$(($top + 1))
dependencies=$(tail -n +${top} ${contest_cargo_toml} )
bottom=$(echo "${dependencies}" | grep -En "^\[" | sed -e "s/^\([0-9]\+\).*/\1/g")

if [ -n "$bottom" ]; then
    echo "${dependencies}" | head -n $(($bottom - 1))
else
    echo "${dependencies}"
fi
