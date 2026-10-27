#!/bin/sh
# Emacs で構文の走査と探す処理の結果を全ての位置で書き出し, core/tests/syntax.json, core/tests/search.json, core/tests/motion.json, core/tests/skip.json, core/tests/edit.json, core/tests/undo.json を作る
# (core/src/syntax.rs と core/src/search.rs の期待値).
#
# 使い方: tools/syntax/run.sh
set -eu

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
OUT="$root/core/tests/syntax.json" emacs -Q --batch -l "$here/dump.el"
OUT="$root/core/tests/search.json" emacs -Q --batch -l "$here/search.el"
OUT="$root/core/tests/motion.json" emacs -Q --batch -l "$here/motion.el" >/dev/null 2>&1
OUT="$root/core/tests/skip.json" emacs -Q --batch -l "$here/skip.el" >/dev/null 2>&1
OUT="$root/core/tests/edit.json" emacs -Q --batch -l "$here/edit.el" >/dev/null 2>&1
OUT="$root/core/tests/undo.json" emacs -Q --batch -l "$here/undo.el" >/dev/null 2>&1
