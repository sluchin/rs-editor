#!/usr/bin/env python3
"""Emacs の組み込みのコマンド (C で書いたもの) と, その既定のキーの一覧を作り, FUNCTIONS.md の印の間に書く.

使い方: generate.py <出力のディレクトリ> <FUNCTIONS.md>

出力のディレクトリには, run.sh が作ったファイルを置く.
  q-*             emacs -Q (既定のコマンドとキー)
  rs-editor.json  rs-editor のコマンドとキー
"""

import json
import re
import sys
from collections import defaultdict
from pathlib import Path

BEGIN = '<!-- emacs-commands:begin -->'
END = '<!-- emacs-commands:end -->'


def load_builtin_commands(path):
    """組み込みのコマンド (ライブラリが (C) のもの) の名前."""
    names = []
    for line in path.read_text().splitlines():
        name, library = (line.split('\t') + [''])[:2]
        if library == '(C)':
            names.append(name)
    return sorted(names)


def load_keys(path):
    """コマンド -> [(キーマップ, キー)] (短いキーの順)."""
    keys = defaultdict(list)
    for line in path.read_text().splitlines():
        keymap, key, command = line.split('\t', 2)
        keys[command].append((keymap, key))
    return {c: sorted(ks, key=lambda mk: (len(mk[1]), mk[1])) for c, ks in keys.items()}


def code(text):
    """Markdown のコード (` を含めば, `` で囲む)."""
    if not text:
        return ''
    text = text.replace('|', '\\|')
    return f'`` {text} ``' if '`' in text else f'`{text}`'


def keys_cell(keys, limit=4):
    """キーを並べる (多ければ, 残りの数を書く)."""
    shown = ', '.join(code(k) for k in keys[:limit])
    return shown + (f' ほか {len(keys) - limit}' if len(keys) > limit else '')


def default_keys_cell(command, keys):
    """既定のキー. global-map のキーが無ければ, ほかのキーマップのキーを, キーマップの名前と書く."""
    global_keys = [k for m, k in keys if m == 'global-map']
    if command == 'self-insert-command':
        # 文字のキーは, 範囲で割り当てていて, 書き出していない
        return '文字のキー (`a` など)'

    if global_keys:
        return keys_cell(global_keys)
    by_map = defaultdict(list)
    for m, k in keys:
        by_map[m].append(k)
    return '<br>'.join(f'{m}: {keys_cell(ks, 2)}' for m, ks in sorted(by_map.items())[:3])


def rs_editor(root, out):
    """rs-editor のコマンド -> キー (Emacs の書き方)."""
    data = json.loads((out / 'rs-editor.json').read_text())
    keys = defaultdict(list)
    for key, command in data['keys']:
        keys[command].append(key)
    names = set(data['commands']) | set(keys)
    # 画面のコマンド (ファイル, バッファ, 評価など)
    for path in list((root / 'src' / 'hooks').glob('*.ts')) + [root / 'src' / 'App.tsx']:
        names |= set(re.findall(r"^\s+'([a-z][a-z0-9*-]*)': ", path.read_text(), re.M))
    return {n: keys.get(n, []) for n in names}


def main():
    out, target = Path(sys.argv[1]), Path(sys.argv[2])
    root = target.parent
    commands = load_builtin_commands(out / 'q-commands.tsv')
    keys = load_keys(out / 'q-keys.tsv')
    rs = rs_editor(root, out)
    implemented = sum(1 for c in commands if c in rs)

    lines = [BEGIN, '']
    w = lines.append
    w('## Emacs の組み込みのコマンド')
    w('')
    w('Emacs 30.2 の組み込みのコマンド (C で書いたもの) と, その既定のキー。'
      '`tools/emacs-commands/run.sh` で作った一覧 (手で直さない。作り直すときは, `run.sh` を動かす)。')
    w('')
    w('- **既定のキー**: `emacs -Q` の global-map のキー。global-map に無ければ, ほかのキーマップ (ミニバッファなど) のキーを, キーマップの名前と書く。')
    w('- **rs-editor**: rs-editor にあるコマンドは「実装済み」と, rs-editor のキーを書く。')
    w('- キーは Emacs の書き方 (`key-description`)。`C-c C-m` は `C-c RET`, `C-i` は `TAB` と書く。')
    w('- Lisp で書いたコマンド (`find-file` など) と, マイナーモードのコマンドは, ここには入れていない。')
    w('')
    w(f'### 組み込みのコマンド ({len(commands)} 個. 実装済み {implemented} 個)')
    w('')
    w('| コマンド | 既定のキー | rs-editor | rs-editor のキー |')
    w('|---|---|---|---|')
    for c in commands:
        done = c in rs
        w(f'| {code(c)} | {default_keys_cell(c, keys.get(c, []))} | {"実装済み" if done else ""} | '
          f'{keys_cell(rs[c], 3) if done else ""} |')
    w('')
    w(END)

    text = target.read_text()
    block = '\n'.join(lines)
    if BEGIN in text:
        start, end = text.index(BEGIN), text.index(END) + len(END)
        text = text[:start] + block + text[end:]
    else:
        text = text.rstrip('\n') + '\n\n' + block + '\n'
    target.write_text(text)
    print(f'{target}: {len(commands)} builtin commands ({implemented} in rs-editor)')


if __name__ == '__main__':
    main()
