#!/bin/sh
# Emacs の組み込みのコマンドと, エディタの基本のコマンドと, その既定のキーの一覧を作り, docs/functions.md に書く.
#
# 使い方: tools/emacs-commands/run.sh
#   OUT        途中のファイルを置くディレクトリ (既定は一時ディレクトリ)
set -eu

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
out=${OUT:-$(mktemp -d)}
mkdir -p "$out"

# 既定 (emacs -Q) のコマンドとキー
DUMP_PREFIX="$out/q" emacs -Q --batch -l "$here/dump.el" 2>/dev/null

# rs-editor のコマンドとキーは, generate.py が core/src/keymap.rs と lisp/*.scm から読む
python3 "$here/generate.py" "$out" "$root/docs/functions.md"
