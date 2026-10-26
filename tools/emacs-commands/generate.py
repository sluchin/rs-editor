#!/usr/bin/env python3
"""Emacs のコマンドとキーの割り当ての一覧を作り, FUNCTIONS.md の印の間に書く.

使い方: generate.py <出力のディレクトリ> <初期設定ファイル> <FUNCTIONS.md>

出力のディレクトリには, run.sh が作ったファイルを置く.
  q-*         emacs -Q (標準のコマンド)
  default-*   初期設定で読む機能を, 設定をせずに読んだもの (変更前)
  custom-*    初期設定を読んだもの (変更後)
  rs-editor.json  rs-editor のコマンドとキー
"""

import json
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

BEGIN = '<!-- emacs-commands:begin -->'
END = '<!-- emacs-commands:end -->'

# プレフィックスを付け直したときの, 下のキーをまとめる (変更前のキーマップの名前)
PREFIX_NAMES = {'<f1>': 'help-map', '<f2>': '2C-command', 'M-g': 'goto-map', 'C-\\': 'toggle-input-method'}


def load_commands(path):
    """コマンド -> (ライブラリ, ファイル)."""
    commands = {}
    for line in path.read_text().splitlines():
        name, library, file = (line.split('\t') + ['', ''])[:3]
        commands[name] = (library, file)
    return commands


def load_keys(path):
    """(キーマップ, キー) -> コマンド."""
    keys = {}
    for line in path.read_text().splitlines():
        keymap, key, command = line.split('\t', 2)
        keys[(keymap, key)] = command
    return keys


def global_keys(keys):
    """コマンド -> global-map のキー (短い順)."""
    by_command = defaultdict(list)
    for (keymap, key), command in keys.items():
        if keymap == 'global-map':
            by_command[command].append(key)
    return {c: sorted(ks, key=lambda k: (len(k), k)) for c, ks in by_command.items()}


def code(text):
    """Markdown のコード (` を含めば, `` で囲む)."""
    if text is None or text == '':
        return ''
    text = text.replace('|', '\\|')
    return f'`` {text} ``' if '`' in text else f'`{text}`'


def keys_cell(keys, limit=4):
    if not keys:
        return ''
    shown = ', '.join(code(k) for k in keys[:limit])
    return shown + (f' ほか {len(keys) - limit}' if len(keys) > limit else '')


def normalize_keys(kbd_strings):
    """kbd の書き方を, key-description の書き方にする (Emacs で)."""
    if not kbd_strings:
        return {}
    expr = '(dolist (k (list %s)) (princ (format "%%s\\t%%s\\n" k (key-description (kbd k)))))' % ' '.join(
        json.dumps(k) for k in kbd_strings
    )
    out = subprocess.run(['emacs', '-Q', '--batch', '--eval', expr], capture_output=True, text=True, check=True)
    return dict(line.split('\t', 1) for line in out.stdout.splitlines())


def init_global_bindings(init_file):
    """初期設定ファイルの (define-key global-map ...) の, (行, キー, 関数)."""
    lines = Path(init_file).read_text(errors='replace').splitlines()
    found = []
    pattern = re.compile(r'\(define-key global-map (\(kbd "((?:[^"\\]|\\.)*)"\)|\[\(([a-z]+) ([a-z0-9]+)\)\]|\(string tty-erase-char\))\s*(.*)$')
    for n, line in enumerate(lines, 1):
        m = pattern.search(line)
        if not m:
            continue
        if m.group(2) is not None:
            key = m.group(2).encode().decode('unicode_escape')
        elif m.group(3):
            mod = {'control': 'C-', 'meta': 'M-'}[m.group(3)]
            key = f'{mod}<{m.group(4)}>'
        else:
            key = None  # tty の消すキー
        # 関数が次の行にあれば, 続けて読む
        rest = ' '.join([m.group(5)] + [l.strip() for l in lines[n:n + 6]]).strip()
        command = re.match(r"'?([^\s)]+)", rest).group(1) if rest else ''
        if rest.startswith('(lambda'):
            command = LAMBDA + lambda_summary(rest)
        found.append((n, key, command))
    names = normalize_keys(sorted({k for _, k, _ in found if k}))
    return [(n, names.get(k, k) if k else '(tty の erase)', c) for n, k, c in found]


LAMBDA = '(lambda)'


def first_sexp(text):
    """text の始めの S 式 (括弧の対を数える. 文字列の中の括弧は数えない)."""
    depth, in_string, escape = 0, False, False
    for i, ch in enumerate(text):
        if in_string:
            if escape:
                escape = False
            elif ch == '\\':
                escape = True
            elif ch == '"':
                in_string = False
        elif ch == '"':
            in_string = True
        elif ch == '(':
            depth += 1
        elif ch == ')':
            depth -= 1
            if depth == 0:
                return text[: i + 1]
    return text


def lambda_summary(text):
    """(lambda ...) の中身の要約 (説明があれば説明, 無ければ (interactive) の後の式)."""
    doc = re.match(r'\(lambda \([^)]*\)\s*"((?:[^"\\]|\\.)*)"', text)
    if doc:
        return ' ' + doc.group(1)
    body = re.search(r'\(interactive[^)]*\)\s*(.*)', text)
    return ' ' + first_sexp(body.group(1))[:60] if body else ''


def last_definitions(bindings):
    """キー -> 最後の (行, 関数) (同じキーを付け直したら, 後のもの)."""
    return {k: (n, c) for n, k, c in bindings}


def rs_editor(root, out):
    """rs-editor のコマンド -> キー (Emacs の書き方). howm.scm のコマンドも."""
    data = json.loads((out / 'rs-editor.json').read_text())
    keys = defaultdict(list)
    for key, command in data['keys']:
        keys[command].append(key)
    names = set(data['commands']) | set(keys)
    for path in list((root / 'src' / 'hooks').glob('*.ts')) + [root / 'src' / 'App.tsx']:
        names |= set(re.findall(r"^\s+'([a-z][a-z0-9*-]*)': ", path.read_text(), re.M))
    howm = set(re.findall(r'define-command \(([a-z0-9-]+)', (root / 'lisp' / 'howm.scm').read_text()))
    return {n: keys.get(n, []) for n in names}, howm


def rs_cell(command, rs, howm):
    if command in rs:
        return '✓ ' + keys_cell(rs[command], 3) if rs[command] else '✓'
    if command in howm:
        return '✓ (howm.scm)'
    return ''


def library_kind(file):
    if not file:
        return '不明'
    if '/usr/share/emacs/' in file:
        return '組み込み'
    return 'パッケージ'


def main():
    out, init_file, target = Path(sys.argv[1]), sys.argv[2], Path(sys.argv[3])
    root = target.parent
    q = load_commands(out / 'q-commands.tsv')
    custom = load_commands(out / 'custom-commands.tsv')
    before = load_keys(out / 'default-keys.tsv')
    after = load_keys(out / 'custom-keys.tsv')
    maps_before = set((out / 'default-maps.txt').read_text().split())
    maps_after = set((out / 'custom-maps.txt').read_text().split())
    rs, howm = rs_editor(root, out)
    keys_before, keys_after = global_keys(before), global_keys(after)
    rs_by_key = {k: c for c, ks in rs.items() for k in ks}
    bindings = init_global_bindings(init_file)
    init_keys = last_definitions(bindings)

    lines = [BEGIN, '']
    w = lines.append
    w('## Emacs のコマンドとキーの割り当て')
    w('')
    w('Emacs 30.2 から, `tools/emacs-commands/run.sh` で作った一覧 (手で直さない。作り直すときは, `run.sh` を動かす)。')
    w('')
    w('- **変更前 (デフォルト)**: `emacs -Q` で, 初期設定ファイルが読む機能 (ライブラリ) を, 設定をせずに読んだもの。')
    w(f'- **変更後 (カスタマイズ)**: `emacs --batch` で, 初期設定ファイル (`{init_file.replace(str(Path.home()), "~")}`) を, フォームごとに評価したもの (失敗したフォームは飛ばす)。')
    w('  `--batch` なので, 窓 (`window-system`) を使う設定と, 読めなかったパッケージの設定は, 入っていない (「batch で入らなかった global-map の設定」)。')
    w('- **rs-editor**: rs-editor にあるコマンド (✓) と, そのキー。`✓ (howm.scm)` は, `lisp/howm.scm` のコマンド。')
    w('- キーは Emacs の書き方 (`key-description`)。`C-c C-m` は `C-c RET`, `C-i` は `TAB` と書く。')
    w('')

    # 1. キーの割り当ての変更
    diff = sorted(
        (m, k, before.get((m, k)), after.get((m, k)))
        for (m, k) in set(before) | set(after)
        if before.get((m, k)) != after.get((m, k))
        and m in maps_before and m in maps_after and 'menu' not in m
    )
    by_map = defaultdict(list)
    for m, k, b, a in diff:
        by_map[m].append((k, b, a))
    w('### 1. キーの割り当ての変更 (変更前 → 変更後)')
    w('')
    w('変更前と変更後で, 割り当てが違うキー (足した, 変えた, 消した)。プレフィックスキーにコマンドを割り当てて, 下のキーが無くなったものは, 1 行にまとめた。')
    w('')
    for m in ['global-map'] + sorted(x for x in by_map if x != 'global-map'):
        rows = by_map.get(m, [])
        if not rows:
            continue
        prefixes = {k for k, b, a in rows if a is not None and any(r[0].startswith(k + ' ') and r[2] is None for r in rows)}
        w(f'#### {m} ({len(rows)} 個)')
        w('')
        if m == 'global-map':
            w('| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | 初期設定の行 | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |')
            w('|---|---|---|---|---|---|')
        else:
            w('| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |')
            w('|---|---|---|---|---|')
        for k, b, a in rows:
            if any(k.startswith(p + ' ') for p in prefixes):
                continue
            if k in prefixes:
                hidden = [r for r in rows if r[0].startswith(k + ' ')]
                b = f'プレフィックス ({PREFIX_NAMES.get(k, "")} の {len(hidden)} 個のキー)'.replace(' ( の', ' (')
                before_cell = b
            else:
                before_cell = code(b) if b else '(無し)'
            after_cell = code(a) if a else '(無し)'
            rs_cell_after = rs_cell(a, rs, howm) if a else ''
            if m == 'global-map':
                line, function = init_keys.get(k, ('', ''))
                if a == '<lambda>' and function.startswith(LAMBDA):
                    after_cell = f'{after_cell}: {function[len(LAMBDA):].strip()}'
                w(f'| {code(k)} | {before_cell} | {after_cell} | {line} | {code(rs_by_key.get(k, ""))} | {rs_cell_after} |')
            else:
                w(f'| {code(k)} | {before_cell} | {after_cell} |  | {rs_cell_after} |')
        w('')

    # 2. batch で入らなかった設定
    def applied(k, c):
        got = after.get(('global-map', k))
        if c.startswith(LAMBDA):
            return got == '<lambda>'
        if c == 'nil':
            return got is None
        return got == c
    missing = sorted((n, k, c) for k, (n, c) in init_keys.items() if not applied(k, c))
    w('### 2. batch で入らなかった global-map の設定')
    w('')
    w('初期設定ファイルの `(define-key global-map ...)` (同じキーを付け直したものは, 最後のもの) のうち, 変更後に入っていないもの (パッケージを読めなかった, 窓が要る, `(when ...)` の条件に合わなかった, など)。')
    w('')
    w('| 行 | キー | 関数 | 変更後の, そのキー | rs-editor |')
    w('|---|---|---|---|---|')
    for n, k, c in missing:
        w(f'| {n} | {code(k)} | {code(c)} | {code(after.get(("global-map", k)))} | {rs_cell(c, rs, howm)} |')
    w('')

    # 3. カスタマイズで増えたコマンド
    extra = sorted((c for c in custom if c not in q), key=lambda c: (library_kind(custom[c][1]), custom[c][0], c))
    w(f'### 3. カスタマイズで増えたコマンド ({len(extra)} 個)')
    w('')
    w('`emacs -Q` には無く, 初期設定ファイルを読むと使えるコマンド。種類は, 初期設定ファイルで定義したもの, Emacs に付いているライブラリ, パッケージ (`~/.emacs.d` など)。')
    w('')
    w('| コマンド | 種類 | ライブラリ | 変更後のキー | rs-editor |')
    w('|---|---|---|---|---|')
    for c in extra:
        library, file = custom[c]
        kind = '初期設定' if library in ('load-init', '') or file.endswith(Path(init_file).name) else library_kind(file)
        w(f'| {code(c)} | {kind} | {library if kind != "初期設定" else ""} | {keys_cell(keys_after.get(c))} | {rs_cell(c, rs, howm)} |')
    w('')

    # 4. 標準のコマンド
    standard = sorted(q, key=lambda c: (q[c][0], c))
    implemented = sum(1 for c in standard if c in rs)
    w(f'### 4. Emacs の標準のコマンド ({len(standard)} 個. rs-editor にあるもの {implemented} 個)')
    w('')
    w('`emacs -Q` で `M-x` で使える全てのコマンド (読んであるものと, autoload)。ライブラリの順。変更後のキーは, 変更前と同じなら「同じ」と書く。キーは global-map のもの (モードのキーマップは, 1. を見る)。')
    w('')
    w('| コマンド | ライブラリ | 変更前のキー | 変更後のキー | rs-editor |')
    w('|---|---|---|---|---|')
    for c in standard:
        kb, ka = keys_before.get(c, []), keys_after.get(c, [])
        after_cell = '同じ' if kb == ka and kb else keys_cell(ka)
        w(f'| {code(c)} | {q[c][0]} | {keys_cell(kb)} | {after_cell} | {rs_cell(c, rs, howm)} |')
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
    print(f'{target}: {len(diff)} changed keys, {len(missing)} missing, {len(extra)} extra, {len(standard)} standard ({implemented} in rs-editor)')


if __name__ == '__main__':
    main()
