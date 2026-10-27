#!/bin/sh
# Emacs で構文の走査と探す処理の結果を全ての位置で書き出し, core/tests/syntax.json と core/tests/search.json を作る
# (core/src/syntax.rs と core/src/search.rs の期待値).
#
# 使い方: tools/syntax/run.sh
set -eu

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
OUT="$root/core/tests/syntax.json" emacs -Q --batch -l "$here/dump.el"
OUT="$root/core/tests/search.json" emacs -Q --batch -l "$here/search.el"
