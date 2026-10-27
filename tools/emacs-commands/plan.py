"""「エディタの基本のコマンド」を, Lisp (`lisp/`) で実装する計画. generate.py が読み, FUNCTIONS.md に書く.

段階は番号で分ける. 前の段階を終えてから, 次の段階に進む (1 は土台で, 2 以降の全ての前提).
コマンドは, ライブラリごとに段階を決め, 例外だけ COMMAND_STEPS に書く. 段階が決まっていないコマンドが
あれば, generate.py が失敗する (Emacs が増やしたコマンドに, 段階を付け忘れないため).
"""

# 段階 (番号, 題, 内容). 内容は, 箇条書きの文.
STEPS = [
    (1, '土台: Lisp から使う組み込みの関数',
     ['コマンドを Lisp で書くために, 組み込み (Rust と画面の TypeScript) に足す。コマンドは入れない。'
      'Emacs の C の関数に当たるもの。',
      'カーソルと文字: `char-after`, `char-before`, `forward-char`, `backward-char`, `forward-line`, `line-beginning-position`, '
      '`line-end-position`, `bobp`, `eobp`, `bolp`, `eolp`, `buffer-size`, `delete-char`, `current-column`, `move-to-column`。',
      '探す: `search-forward`, `search-backward`, `re-search-forward`, `re-search-backward`, `looking-at`, `match-beginning`, `match-end`。',
      'マークとリージョン: `mark`, `set-mark`, `push-mark`, `pop-mark`, `region-beginning`, `region-end`, `use-region-p`。',
      '切り取りと貼り付け: `kill-region`, `current-kill`, `kill-append`, `insert-for-yank`。元に戻す: `undo-boundary`, `primitive-undo`。',
      '前置引数: `current-prefix-arg`, `prefix-numeric-value`。`define-command` に, 引数の読み方 (Emacs の `interactive`) を足す。',
      '窓とバッファ: `selected-window`, `split-window`, `delete-window`, `window-list`, `set-window-buffer`, `set-buffer`, '
      '`get-buffer`, `generate-new-buffer`, `with-current-buffer`, バッファごとの変数 (`make-local-variable`)。',
      '構文: `scan-lists`, `scan-sexps`, `syntax-ppss`, `char-syntax` (`src/lib/syntax.ts` の計算を Rust に移すか, 画面に頼む)。',
      '速さが要るもの (`self-insert-command`, カーソルを動かすもの) は, Lisp で書いて動かして比べてから, 画面に残すか決める。']),
    (2, '動き',
     ['行, 単語, バッファの端, 左右の移動。カーソルを動かすので, 速さを比べてから Lisp に移す。']),
    (3, '消す, 切る, 貼る, 元に戻す',
     ['kill-ring を使うコマンドと, 空白と改行を整えるコマンド, 入れ替え, `undo`。']),
    (4, 'マーク, 前置引数, 実行',
     ['マークとリージョン, 前置引数 (`C-u`, `M-数字`), `M-x`, `M-:`, `C-g`, エラーの移動, 表示の切り替え。']),
    (5, 'S 式, 段落, ページ',
     ['括弧の移動と消去 (`C-M-f` など), defun, 文と段落, ページ。構文表 (段階 1) を使う。']),
    (6, '字下げ, 整形, コメント',
     ['`TAB`, `C-M-\\`, `M-q`, `M-;`。字下げの決まりを, Lisp と画面で 1 つにする (TODO の Phase 18 と同じ)。']),
    (7, '検索, 置換, 強調',
     ['isearch, `query-replace`, `occur`, `highlight-regexp`。画面の検索の状態 (ミニバッファの 1 つのキーの問い) を Lisp から使えるようにする。']),
    (8, '矩形, レジスタ, キーボードマクロ, 略語',
     ['矩形 (`C-x r`), レジスタ, キーボードマクロ (`F3`, `F4`, `C-x (`), 略語と `M-/`。']),
    (9, 'ファイル, バッファ, dired, ブックマーク, シェル',
     ['ファイルとバッファの残りのコマンド, `list-buffers`, dired, ブックマーク, `M-!`, `M-|`, `M-&`, バッファの複製。',
      '`save-buffers-kill-terminal` は, 窓を閉じる組み込みを足してから移す。']),
    (10, 'ウィンドウ',
     ['窓の分割, 削除, 移動, 大きさ, スクロール, 別の窓に出すコマンド。窓の木 (`src/lib/windows.ts`) を操作する組み込みを使う。']),
    (11, 'ヘルプ, xref, info',
     ['`C-h` のコマンドと, 定義へ移るコマンド (`M-.`), info。ヘルプは Phase 17 (docstring と多言語化) と一緒に進める。']),
    (12, '入れない',
     ['rs-editor に当たる機能が無いので, 入れない。メール, フレーム (複数の窓枠), 日本語入力以外のテキスト変換, Emacs の案内の文書。']),
]

# ライブラリごとの, 既定の段階.
LIBRARY_STEPS = {
    'simple': 3,
    'files': 9,
    'window': 10,
    'bindings': 2,
    'lisp': 5,
    'indent': 6,
    'paragraphs': 5,
    'page': 5,
    'fill': 6,
    'newcomment': 6,
    'replace': 7,
    'isearch': 7,
    'rect': 8,
    'register': 8,
    'kmacro': 8,
    'abbrev': 8,
    'dabbrev': 8,
    'buff-menu': 9,
    'dired': 9,
    'bookmark': 9,
    'xref': 11,
    'loaddefs': 7,
    'help': 11,
    'help-fns': 11,
    'info': 11,
}

# ライブラリの既定と違うコマンドの段階.
COMMAND_STEPS = {
    # simple: 動き
    **dict.fromkeys([
        'back-to-indentation', 'backward-word', 'beginning-of-buffer', 'end-of-buffer', 'move-beginning-of-line',
        'move-end-of-line', 'next-line', 'previous-line', 'goto-line', 'goto-line-relative', 'set-goal-column',
    ], 2),
    # simple: マーク, 前置引数, 実行, 表示
    **dict.fromkeys([
        'set-mark-command', 'exchange-point-and-mark', 'mark-whole-buffer', 'mark-word', 'pop-global-mark',
        'digit-argument', 'negative-argument', 'universal-argument', 'keyboard-quit', 'keyboard-escape-quit',
        'what-cursor-position', 'count-words-region', 'set-fill-column', 'toggle-truncate-lines',
        'set-selective-display', 'repeat-complex-command', 'execute-extended-command',
        'execute-extended-command-for-buffer', 'eval-expression', 'next-error', 'previous-error',
    ], 4),
    # simple: S 式の入れ替え
    'transpose-sexps': 5,
    # simple: シェルとバッファの複製
    **dict.fromkeys([
        'shell-command', 'shell-command-on-region', 'async-shell-command', 'clone-buffer',
        'clone-indirect-buffer-other-window',
    ], 9),
    # simple: メールとテキスト変換
    **dict.fromkeys([
        'compose-mail', 'compose-mail-other-frame', 'compose-mail-other-window', 'analyze-text-conversion',
    ], 12),
    # bindings: 補完
    'complete-symbol': 8,
    # bindings: kill-ring
    'ignore-preserving-kill-region': 3,
    # help: Emacs の案内の文書
    **dict.fromkeys([
        'describe-copying', 'describe-distribution', 'describe-gnu-project', 'describe-no-warranty',
        'view-emacs-FAQ', 'view-emacs-debugging', 'view-emacs-news', 'view-emacs-problems', 'view-emacs-todo',
        'view-external-packages', 'view-order-manuals',
    ], 12),
    # window: フレーム
    'handle-select-window': 12,
}


def step_of(command, library):
    """コマンドの段階 (番号). フレームとタブに出すコマンドは 12. 決まっていなければ None.

    Args:
        command: コマンドの名前.
        library: Emacs でコマンドがあるライブラリ.
    """
    if command in COMMAND_STEPS:
        return COMMAND_STEPS[command]
    if 'frame' in command or command.endswith('-other-tab'):
        return 12
    return LIBRARY_STEPS.get(library)
