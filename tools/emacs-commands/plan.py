"""「エディタの基本のコマンド」を, Lisp (`lisp/`) で実装する計画. generate.py が読み, FUNCTIONS.md に書く.

段階は番号で分ける. 前の段階を終えてから, 次の段階に進む (1 は土台で, 2 以降の全ての前提).
コマンドは, ライブラリごとに段階を決め, 例外だけ COMMAND_STEPS に書く. 段階が決まっていないコマンドが
あれば, generate.py が失敗する (Emacs が増やしたコマンドに, 段階を付け忘れないため).
"""

# 段階 (番号, 題, 内容). 内容は, 箇条書きの文.
STEPS = [
    (1, '土台: Rust のエディタの核 (ターミナルの ncurses でも動かすため)',
     ['目標は, 同じエディタを, 今の画面 (Tauri) と, ターミナル (ncurses) の両方で動かすこと。そのため, バッファ, カーソル, マーク, undo, '
      'kill-ring, 窓を, 画面 (TypeScript) でなく Rust の核に持つ。画面は, 核の状態を描いて, キーを核に渡すだけにする。'
      'コマンドは入れない (Emacs の C の関数に当たるものを作る)。',
      '【済】1.1 クレートを分ける: 画面に頼らない核のクレート (`core/`. 評価器 `scheme` も移す) と, それを使うアプリ (`src-tauri/`), ターミナルのアプリ (`tui/`)。'
      'Cargo のワークスペースにする。テストのカバレッジ 100% は, 核にも保つ。',
      '【Rust の部品は済. Lisp につなぐのは 1.9】1.2 バッファ (`core/src/buffer.rs`): 本文 (rope), カーソル, 変更の有無, ファイルの名前。`point`, `goto-char`, `insert`, `delete-region`, `char-after`, `char-before`, '
      '`forward-char`, `backward-char`, `forward-line`, `line-beginning-position`, `line-end-position`, `bobp`, `eobp`, `bolp`, `eolp`, `buffer-size`, '
      '`current-column`, `move-to-column`。位置は文字で数える (今の Lisp と同じ)。',
      '【Rust の部品は済. Lisp につなぐのは 1.9】1.3 マークとリージョン (`Buffer`): `mark`, `set-mark`, `push-mark`, `pop-mark`, `region-beginning`, `region-end`, `use-region-p`。',
      '【Rust の部品は済. Lisp につなぐのは 1.9】1.4 元に戻す (`Buffer`): 変更の履歴 (`undo-boundary`, `primitive-undo`)。今の CodeMirror の履歴と同じ動き (まとめ方, 重ねた undo) にする。',
      '【Rust の部品は済. Lisp につなぐのは 1.9】1.5 kill-ring (`core/src/kill_ring.rs`): `kill-region`, `current-kill`, `kill-append`, `insert-for-yank`。',
      '【Rust の部品は済 (`core/src/search.rs`, `core/src/syntax.rs`). Lisp につなぐのは 1.9】1.6 探す: `search-forward`, `search-backward`, `re-search-forward`, `re-search-backward`, `looking-at`, `match-beginning`, `match-end` '
      '(Emacs の正規表現の書き方に合わせる)。構文: `scan-lists`, `scan-sexps`, `syntax-ppss`, `char-syntax` (`src/lib/syntax.ts` を Rust に移した)。どちらも `emacs -Q --batch` の結果と全ての位置で比べる (`tools/syntax/run.sh`)。',
      '【Rust の部品は済 (`core/src/window.rs`, `core/src/editor.rs`). Lisp につなぐのは 1.9】1.7 窓とバッファの一覧: `selected-window`, `split-window`, `delete-window`, `window-list`, `set-window-buffer`, `set-buffer`, `get-buffer`, '
      '`generate-new-buffer`, `with-current-buffer`, バッファごとの変数 (`make-local-variable`)。',
      '【前置引数の部品は済 (`Editor::prefix_arg`, `prefix_numeric_value`). `interactive` は 1.9】1.8 前置引数: `current-prefix-arg`, `prefix-numeric-value`。`define-command` に, 引数の読み方 (Emacs の `interactive`) を足す。',
      '【Rust は済 (`core/src/scheme/editor_host.rs`: `Editor` を直接動かす `EditorHost`. 63 個の組み込み). 今の画面 (Tauri) をつなぐのは 1.11】1.9 評価器を核につなぐ: 今の `Host` (画面に頼んで返事を待つ) をやめ, 評価器が核を直接動かす。画面に頼むのは, 描くこと, ミニバッファの入力, '
      'ファイルの選択だけにする (`Host` を小さくする)。',
      '【済 (`tui/`. 動いている: 移動, 編集, kill, yank, undo, 窓の分割, ファイルを開く, 保存, M-x, 終了。基本のコマンドは `lisp/simple.scm`)】1.10 ターミナルのアプリの骨組み (`tui/`. ncurses): 核の状態を描き, キーを核に渡す。カーソルを動かし, 文字を入れ, 保存できるところまで。',
      '【段階 2 から 11 の後】1.11 今の画面 (Tauri) を核につなぎ替える: 本文の持ち主を CodeMirror から核に移す。CodeMirror は表示と入力にする。'
      '今の画面のコマンド (`src/lib/emacs.ts` など. 約 85 個) は, CodeMirror の状態を直接書き換えているので, 段階 2 から 11 で Lisp に移し終えるまでは, 持ち主を移せない。'
      '移す前の動きは, `tests/frontend/` のテストで保つ。',
      '【つないだ (`core/src/scheme/mirror.rs`. 環境変数 `RSEDITOR_NO_CORE` で、これまでの動きに戻せる)】1.11 の前につなぐ案 (Tauri の画面を, 先に核の組み込みで動かす): 評価のあいだだけ, 画面のバッファを核に写し (スナップショット), 終わったら変更を画面に返す '
      '(`EditorHost` を `Bridge` の上で動かし, 本文の操作は核で, バッファの出し入れとウィンドウは画面に頼む). 探す, 構文, マークなどの組み込みが, 今の画面の Lisp (howm, org) でも使える。'
      '画面の往復が減る代わりに, 画面と核の本文の同期が要る。実際の画面での確かめが要るので, 段階 2 以降の Lisp が画面で要るようになったときに行う。']),
    (2, '動き',
     ['【Lisp 版は済 (`lisp/simple.scm`). `emacs -Q --batch` の結果と, 本文の全ての位置で一致 (`core/tests/motion_test.rs`. 392 通り). '
      '画面 (Tauri) の TypeScript 版を消すのは, 画面をつないだ後 (段階 1.11 の前の案)】'
      '行, 単語, バッファの端, 左右の移動。核の上の Lisp で書く。カーソルを動かすので, 速さを比べてから Lisp に移す。']),
    (3, '消す, 切る, 貼る, 元に戻す',
     ['【Lisp 版は済 (`lisp/simple.scm`). `emacs -Q --batch` の結果と一致 (`core/tests/edit_test.rs`: 57 通りのコマンドを 7 種類の本文の全ての位置から, '
      '`core/tests/undo_test.rs`: undo と undo-redo の 10 通りの場面). 画面 (Tauri) の TypeScript 版を消すのは, 画面をつないだ後】'
      '違いが残るのは, `default-indent-new-line` (字下げは段階 6), `quoted-insert` (1 文字を表すキーだけ), `cycle-spacing` (最初の呼び出しだけを Emacs と比べた), '
      '`undo` が保存した状態に戻っても「変更なし」にならないこと。'
      'kill-ring を使うコマンドと, 空白と改行を整えるコマンド, 入れ替え, `undo`。']),
    (4, 'マーク, 前置引数, 実行',
     ['マークとリージョン, 前置引数 (`C-u`, `M-数字`), `M-x`, `M-:`, `C-g`, エラーの移動, 表示の切り替え。']),
    (5, 'S 式, 段落, ページ',
     ['括弧の移動と消去 (`C-M-f` など), defun, 文と段落, ページ。構文表 (段階 1.6) を使う。'
      '`src/lib/sexp.ts` と `src/lib/paragraph.ts` の計算は, ターミナルでも使うため, Rust に移す。']),
    (6, '字下げ, 整形, コメント',
     ['`TAB`, `C-M-\\`, `M-q`, `M-;`。字下げの決まり (`src/lib/lispIndent.ts`) と整形 (`src/lib/fill.ts`) は, ターミナルでも使うため, Rust に移す。'
      'エディタと整形の道具 (TODO の Phase 18) で, 決まりを 1 つにする。']),
    (7, '検索, 置換, 強調',
     ['isearch, `query-replace`, `occur`, `highlight-regexp`。検索の状態を核に持つ (描き方は, 画面ごと)。']),
    (8, '矩形, レジスタ, キーボードマクロ, 略語',
     ['矩形 (`C-x r`), レジスタ, キーボードマクロ (`F3`, `F4`, `C-x (`), 略語と `M-/`。']),
    (9, 'ファイル, バッファ, dired, ブックマーク, シェル',
     ['ファイルとバッファの残りのコマンド, `list-buffers`, dired, ブックマーク, `M-!`, `M-|`, `M-&`, バッファの複製。',
      '`save-buffers-kill-terminal` は, アプリを終える頼みを `Host` に足してから移す。']),
    (10, 'ウィンドウ',
     ['窓の分割, 削除, 移動, 大きさ, スクロール, 別の窓に出すコマンド。窓の木 (`src/lib/windows.ts`) は, 段階 1.7 で核に移したものを使う。']),
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
