#!/bin/sh
# Emacs のコマンドとキーの割り当ての一覧を作り, FUNCTIONS.md に書く.
#
# 使い方: tools/emacs-commands/run.sh
#   INIT_FILE  比べる初期設定ファイル (既定は ~/.emacs.el)
#   OUT        途中のファイルを置くディレクトリ (既定は一時ディレクトリ)
set -eu

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
init=${INIT_FILE:-$HOME/.emacs.el}
out=${OUT:-$(mktemp -d)}
mkdir -p "$out"

# 標準 (emacs -Q), 変更後 (初期設定を読む), 変更前 (同じ機能を, 設定をせずに読む)
DUMP_PREFIX="$out/q" emacs -Q --batch -l "$here/dump.el" 2>/dev/null
INIT_FILE="$init" DUMP_PREFIX="$out/custom" emacs --batch -l "$here/load-init.el" -l "$here/dump.el" 2>/dev/null
DUMP_DIR="$out" DUMP_PREFIX="$out/default" emacs -Q --batch -l "$here/default.el" -l "$here/dump.el" 2>/dev/null

# rs-editor のコマンドとキー (src/lib/emacs.ts を読むため, 一時的なテストで書き出す)
test_file="$root/tests/zz-emacs-commands-dump.test.ts"
trap 'rm -f "$test_file"' EXIT
cat >"$test_file" <<EOF
import { it } from 'vitest'
import { writeFileSync } from 'node:fs'
import { createCommands, KEY_BINDINGS, toEmacsKey } from '@/lib/emacs'

it('dump', () => {
  const commands = Object.keys(
    createCommands({ killRing: [] as never, message: () => {}, macro: {} as never }),
  )
  const keys = KEY_BINDINGS.map(([key, command]) => [toEmacsKey(key), command])
  writeFileSync('$out/rs-editor.json', JSON.stringify({ commands, keys }))
})
EOF
(cd "$root" && npx vitest run "$test_file" >/dev/null)

python3 "$here/generate.py" "$out" "$init" "$root/FUNCTIONS.md"
