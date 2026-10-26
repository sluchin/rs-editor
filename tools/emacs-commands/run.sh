#!/bin/sh
# Emacs の組み込みのコマンドと, その既定のキーの一覧を作り, FUNCTIONS.md に書く.
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

python3 "$here/generate.py" "$out" "$root/FUNCTIONS.md"
