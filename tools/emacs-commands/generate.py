#!/usr/bin/env python3
"""Emacs の組み込みのコマンド (C で書いたもの) と, エディタの基本のコマンド (Lisp で書いたもの) と,
その既定のキーの一覧を作り, FUNCTIONS.md の印の間に書く.

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

sys.path.insert(0, str(Path(__file__).resolve().parent))
import plan  # noqa: E402

BEGIN = '<!-- emacs-commands:begin -->'
END = '<!-- emacs-commands:end -->'


# エディタの基本のライブラリ (Lisp で書いたコマンドのうち, ここのものを載せる)
ESSENTIAL_LIBRARIES = [
    'simple', 'files', 'window', 'bindings', 'lisp', 'indent', 'paragraphs', 'page', 'fill', 'newcomment',
    'replace', 'isearch', 'rect', 'register', 'kmacro', 'abbrev', 'dabbrev', 'buff-menu', 'dired',
    'bookmark', 'xref', 'loaddefs', 'help', 'help-fns', 'info',
]


def load_commands(path):
    """コマンド -> ライブラリ ((C) は, C で書いたもの)."""
    commands = {}
    for line in path.read_text().splitlines():
        name, library = (line.split('\t') + [''])[:2]
        commands[name] = library
    return commands


def essential_commands(commands, keys):
    """エディタの基本の, Lisp で書いたコマンド (既定の global-map にキーがあり, 基本のライブラリにあるもの.
    マイナーモードと, 版管理 (vc-) は除く). ライブラリの順."""
    names = [
        c for c, library in commands.items()
        if library in ESSENTIAL_LIBRARIES
        and any(m == 'global-map' for m, _ in keys.get(c, []))
        and not c.endswith('-mode') and not c.startswith('vc-')
    ]
    return sorted(names, key=lambda c: (ESSENTIAL_LIBRARIES.index(commands[c]), c))


def load_edwin(path):
    """Edwin (MIT/GNU Scheme のエディタ) にあるコマンドの名前の集まり."""
    return {
        line.strip()
        for line in path.read_text().splitlines()
        if line.strip() and not line.startswith('#')
    }


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
    """rs-editor のコマンド -> (キー (Emacs の書き方), 実装したファイル (リポジトリからのパス))."""
    data = json.loads((out / 'rs-editor.json').read_text())
    keys = defaultdict(list)
    for key, command in data['keys']:
        keys[command].append(key)
    places = defaultdict(list)
    # エディタのコマンド (createCommands)
    for name in data['commands']:
        places[name].append('src/lib/emacs.ts')
    # 画面のコマンド (ファイル, バッファ, 評価など. useMinibuffer.ts は, 入力の履歴の名前で, コマンドでない)
    hooks = [p for p in sorted((root / 'src' / 'hooks').glob('*.ts')) if p.name != 'useMinibuffer.ts']
    for path in hooks + [root / 'src' / 'App.tsx']:
        for name in sorted(screen_commands(path.read_text())):
            places[name].append(str(path.relative_to(root)))
    # Lisp で書いたコマンド (define-command)
    for path in sorted((root / 'lisp').glob('*.scm')):
        for name in sorted(set(re.findall(r"^\(define-command \(([^\s()]+)", path.read_text(), re.M))):
            places[name].append(str(path.relative_to(root)))
    # 起動のときに読む Lisp (loadup.rs) で割り当てたキー (global-set-key)
    loadup = (root / 'core' / 'src' / 'scheme' / 'loadup.rs').read_text()
    for file in re.findall(r'"(lisp/[^"]+\.scm)"', loadup):
        text = (root / file).read_text()
        for key, name in re.findall(r"^\(global-set-key \"([^\"]+)\" '([^\s()]+)\)", text, re.M):
            keys[name].append(key)
    # キーだけで動くもの (前置引数. キーを受ける所で扱う)
    for name in keys:
        if not places[name]:
            places[name].append('src/lib/emacs.ts')
    names = set(places) | set(keys)
    return {n: (keys.get(n, []), places.get(n, [])) for n in names}


def screen_commands(text):
    """画面のコマンドの名前 (`'name': ` の形と, `actions: {` の中の `name: ` の形)."""
    names = set(re.findall(r"^\s+'([a-z][a-z0-9*-]*)': ", text, re.M))
    for block in re.findall(r"^(\s*)actions: \{\n(.*?)^\1\}", text, re.M | re.S):
        names |= set(re.findall(r"^\s+([a-z][a-z0-9]*): ", block[1], re.M))
    return names


def place_cell(places):
    """実装したファイル (Lisp か TypeScript か). 無ければ, 空."""
    if not places:
        return ''
    language = 'Lisp' if all(p.endswith('.scm') for p in places) else 'TypeScript'
    return f'{language} ({", ".join(code(p) for p in places)})'



def plan_cell(command, library, places):
    """計画の段階 (番号). 既に Lisp で書いてあれば「済」."""
    if places and all(p.endswith('.scm') for p in places):
        return '済'
    step = plan.step_of(command, library)
    if step is None:
        sys.exit(f'{command} ({library}): tools/emacs-commands/plan.py に段階を書いてください')
    return str(step)


def plan_lines(essential, all_commands, rs):
    """実装の計画 (段階の一覧と, 段階ごとの内容)."""
    counts = {number: [0, 0, 0] for number, _, _ in plan.STEPS}  # 全部, 新しく書く, TypeScript から移す
    for c in essential:
        cell = plan_cell(c, all_commands[c], rs[c][1] if c in rs else [])
        if cell == '済':
            continue
        counts[int(cell)][0] += 1
        counts[int(cell)][1 if c not in rs else 2] += 1
    lines = ['### 実装の計画 (エディタの基本のコマンドを Lisp で書く)', '']
    lines.append('エディタの基本のコマンドを, Lisp (`lisp/`) で書く計画。TODO.md の Phase 16 の続き。目標は, 今の画面 (Tauri) と, ターミナル (ncurses) の両方で動かすこと'
                 '(そのため, 段階 1 で, エディタの核を Rust に作る)。'
                 '下の表の「計画」の列の番号が, ここの段階の番号。「済」は, 既に Lisp で書いたもの。'
                 '段階は番号の順に進める (1 は土台で, 2 以降の前提)。'
                 'TypeScript で書いてあるコマンドは, Lisp に移したら TypeScript の方を消す。')
    lines.append('')
    lines.append('| 段階 | 内容 | コマンド | 新しく書く | TypeScript から移す |')
    lines.append('|---|---|---|---|---|')
    for number, title, _ in plan.STEPS:
        total, new, move = counts[number]
        lines.append(f'| {number} | {title} | {total or "-"} | {new or "-"} | {move or "-"} |')
    lines.append('')
    for number, title, body in plan.STEPS:
        lines.append(f'**{number}. {title}**')
        lines.append('')
        lines.extend(f'- {text}' for text in body)
        lines.append('')
    return lines


def main():
    out, target = Path(sys.argv[1]), Path(sys.argv[2])
    root = target.parent
    all_commands = load_commands(out / 'q-commands.tsv')
    commands = sorted(c for c, library in all_commands.items() if library == '(C)')
    keys = load_keys(out / 'q-keys.tsv')
    edwin = load_edwin(Path(__file__).resolve().parent / 'edwin-commands.txt')
    essential = essential_commands(all_commands, keys)
    rs = rs_editor(root, out)
    implemented = sum(1 for c in commands if c in rs)
    essential_implemented = sum(1 for c in essential if c in rs)

    def languages(names):
        """実装済みのうち, Lisp と TypeScript の数."""
        cells = [place_cell(rs[c][1]) for c in names if c in rs]
        lisp = sum(1 for cell in cells if cell.startswith('Lisp'))
        return f'Lisp {lisp} 個, TypeScript {len(cells) - lisp} 個'


    lines = [BEGIN, '']
    w = lines.append
    w('## Emacs の組み込みのコマンドと, エディタの基本のコマンド')
    w('')
    w('Emacs 30.2 の組み込みのコマンド (C で書いたもの) と, エディタの基本のコマンド (Lisp で書いたもの) と, その既定のキー。'
      '`tools/emacs-commands/run.sh` で作った一覧 (手で直さない。作り直すときは, `run.sh` を動かす)。')
    w('')
    w('- **既定のキー**: `emacs -Q` の global-map のキー。global-map に無ければ, ほかのキーマップ (ミニバッファなど) のキーを, キーマップの名前と書く。')
    w('- **rs-editor**: rs-editor にあるコマンドは「実装済み」と, rs-editor のキーを書く。')
    w('- **実装**: rs-editor で書いた言語とファイル。今は, ほとんどを TypeScript (画面) で書いている。'
      'いずれ Lisp (`lisp/`) に移す (移したら「Lisp」になる)。Emacs の「C で書いたもの」「Lisp で書いたもの」は, '
      'Emacs の中での区別で, rs-editor での書き方とは別。')
    w('- **Edwin**: MIT/GNU Scheme 12.1 の Edwin (Emacs に似たエディタ. `src/edwin` の `define-command`) にも, 同じ名前のコマンドがあれば ◯。'
      '名前の一覧は `tools/emacs-commands/edwin-commands.txt` (`tools/emacs-commands/edwin.sh` で作る)。')
    w('- **計画**: 「エディタの基本のコマンド」の表だけにある。Lisp で書く段階の番号 (下の「実装の計画」の段階)。「済」は, 既に Lisp で書いたもの。'
      '計画は `tools/emacs-commands/plan.py` に書く。')
    w('- キーは Emacs の書き方 (`key-description`)。`C-c C-m` は `C-c RET`, `C-i` は `TAB` と書く。')
    w('- 「エディタの基本のコマンド」は, Lisp で書いたコマンド (`find-file` など) のうち, 既定の global-map にキーがあり, '
      'エディタの基本のライブラリ (' + ', '.join(f'`{l}`' for l in ESSENTIAL_LIBRARIES) + ') にあるもの。'
      'マイナーモードのコマンド (`-mode`) と, 版管理 (`vc-`) は, 入れていない。')
    w('')
    edwin_builtin = sum(1 for c in commands if c in edwin)
    w(f'### 組み込みのコマンド ({len(commands)} 個. 実装済み {implemented} 個: {languages(commands)}. Edwin にもある: {edwin_builtin} 個)')
    w('')
    w('| コマンド | Edwin | 既定のキー | rs-editor | rs-editor のキー | 実装 |')
    w('|---|---|---|---|---|---|')
    for c in commands:
        done = c in rs
        w(f'| {code(c)} | {"◯" if c in edwin else ""} | {default_keys_cell(c, keys.get(c, []))} | {"実装済み" if done else ""} | '
          f'{keys_cell(rs[c][0], 3) if done else ""} | {place_cell(rs[c][1]) if done else ""} |')
    w('')
    for line in plan_lines(essential, all_commands, rs):
        w(line)
    edwin_essential = sum(1 for c in essential if c in edwin)
    w(f'### エディタの基本のコマンド (Lisp で書いたもの. {len(essential)} 個. '
      f'実装済み {essential_implemented} 個: {languages(essential)}. Edwin にもある: {edwin_essential} 個)')
    w('')
    w('| コマンド | Edwin | ライブラリ | 既定のキー | rs-editor | rs-editor のキー | 実装 | 計画 |')
    w('|---|---|---|---|---|---|---|---|')
    for c in essential:
        done = c in rs
        w(f'| {code(c)} | {"◯" if c in edwin else ""} | {all_commands[c]} | {default_keys_cell(c, keys.get(c, []))} | {"実装済み" if done else ""} | '
          f'{keys_cell(rs[c][0], 3) if done else ""} | {place_cell(rs[c][1]) if done else ""} | '
          f'{plan_cell(c, all_commands[c], rs[c][1] if c in rs else [])} |')
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
    print(f'{target}: {len(commands)} builtin commands ({implemented} in rs-editor), '
          f'{len(essential)} essential commands ({essential_implemented} in rs-editor)')


if __name__ == '__main__':
    main()
