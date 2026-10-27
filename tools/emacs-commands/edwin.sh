#!/bin/sh
# MIT/GNU Scheme の Edwin (Emacs に似たエディタ) の, コマンドの名前の一覧を作る (edwin-commands.txt).
# ソースは, 一時ディレクトリに取り出すだけで, 動かさない.
#
# 使い方: tools/emacs-commands/edwin.sh [版]   (既定は 12.1)
set -eu

version=${1:-12.1}
here=$(cd "$(dirname "$0")" && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

curl -fsSL -o "$work/src.tar.gz" "https://ftp.gnu.org/gnu/mit-scheme/stable.pkg/$version/mit-scheme-$version.tar.gz"
mkdir "$work/src"
tar -xzf "$work/src.tar.gz" -C "$work/src" --wildcards '*/src/edwin/*.scm'

python3 - "$work/src" "$here/edwin-commands.txt" "$version" <<'PY'
import glob
import re
import sys

root, out, version = sys.argv[1:4]
names = set()
for path in glob.glob(f'{root}/*/src/edwin/*.scm'):
    text = open(path, errors='replace').read()
    # (define-command 名前 ...) と, (define-command '名前 ...)
    for match in re.finditer(r"\(define-command\s+'?([^\s()']+)", text):
        names.add(match.group(1))
with open(out, 'w') as f:
    f.write(f'# MIT/GNU Scheme {version} の Edwin (src/edwin/*.scm) で define-command したコマンドの名前 (tools/emacs-commands/edwin.sh で作る)\n')
    f.write('\n'.join(sorted(names)) + '\n')
print(f'{out}: {len(names)} commands')
PY
