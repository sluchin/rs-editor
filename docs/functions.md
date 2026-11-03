# Scheme 組み込み関数リスト

このファイルは、rs-editor に実装されている Scheme インタプリタの組み込み関数一覧をまとめたものです。

後半の「Emacs のコマンドとキーの割り当て」は、Emacs の全てのコマンドと、初期設定ファイル (`~/.emacs.el`) で変えたキーの割り当てを、rs-editor と比べた一覧です (`tools/emacs-commands/run.sh` で作る)。

## 算術演算

### `(+ num1 num2 ...)`

複数の数値を加算します。

**例:**

```scheme
(+ 1 2 3)    ; => 6
(+ 10.5 0.5) ; => 11
(+)          ; => 0
```

### `(- num1 num2 ...)`

最初の数値から残りの数値を減算します。単一引数の場合は負数化します。

**例:**

```scheme
(- 10 3)     ; => 7
(- 5)        ; => -5
(- 100 20 10) ; => 70
```

### `(* num1 num2 ...)`

複数の数値を乗算します。

**例:**

```scheme
(* 2 3 4)    ; => 24
(* 5 0)      ; => 0
(*)          ; => 1
```

### `(/ num1 num2 ...)`

最初の数値を残りの数値で順番に除算します。ゼロ除算はエラーになります。

**例:**

```scheme
(/ 20 4)     ; => 5
(/ 100 2 5)  ; => 10
(/ 1 0)      ; => エラー: 0で除算することはできません
```

## リスト操作

### `(quote expr)`

式を評価せずにそのまま返します。リテラルとしてリストやシンボルを取得するのに使用します。

**例:**

```scheme
(quote (+ 1 2))  ; => (+ 1 2)
(quote hello)    ; => hello
'(1 2 3)         ; => (1 2 3) [クォート記法]
```

### `(list elem1 elem2 ...)`

与えられた要素からリストを構築します。各要素は評価されます。

**例:**

```scheme
(list 1 2 3)              ; => (1 2 3)
(list (+ 1 1) (+ 2 1))   ; => (2 3)
(list)                    ; => ()
```

## 実装予定の関数（TODO）

以下の関数は .claude/CLAUDE.md の TODO リストで実装予定です：

### 比較演算子

- `(> num1 num2 ...)`
- `(< num1 num2 ...)`
- `(= num1 num2 ...)`
- `(>= num1 num2 ...)`
- `(<= num1 num2 ...)`

### 制御フロー

- `(if condition then-expr else-expr)`
- `(cond (test1 expr1) (test2 expr2) ...)`
- `(case expr (key1 expr1) (key2 expr2) ...)`

### 変数バインディング

- `(define name value)`
- `(let ((var1 val1) (var2 val2)) body)`
- `(let* ((var1 val1) (var2 val2)) body)`
- `(letrec ((var1 val1) (var2 val2)) body)`

### ラムダ / 関数定義

- `(lambda (args) body)`
- `(define-syntax name transformer)`

### 基本的なリスト操作

- `(car lst)` - リストの先頭要素を返す
- `(cdr lst)` - リストの残りを返す
- `(cons elem lst)` - 要素をリストの先頭に追加
- `(append lst1 lst2 ...)` - リストを結合
- `(length lst)` - リストの要素数を返す
- `(map func lst)` - リストの各要素に関数を適用

### 型チェック

- `(integer? expr)`
- `(number? expr)`
- `(string? expr)`
- `(symbol? expr)`
- `(list? expr)`
- `(boolean? expr)`

## 型システム

rs-editor の Scheme インタプリタがサポートしている型：

- **Number**: 浮動小数点数（内部表現は f64）
- **String**: ダブルクォートで囲まれたテキスト
- **Symbol**: クォートなしの識別子
- **Boolean**: `#t` (true) / `#f` (false)
- **List**: S式形式のリスト `(elem1 elem2 ...)`

## 今後の拡張予定

詳細は `.claude/CLAUDE.md` の TODO と `TODO.md` のフェーズ分けを参照してください。

次のステップ：

1. 比較演算子と制御フロー（`if`、`cond`）の実装
2. 変数バインディングと定義（`define`、`let`）
3. より高度なリスト操作
4. ラムダ式とクロージャのサポート

<!-- emacs-commands:begin -->

## Emacs の組み込みのコマンドと, エディタの基本のコマンド

Emacs 30.2 の組み込みのコマンド (C で書いたもの) と, エディタの基本のコマンド (Lisp で書いたもの) と, その既定のキー。`tools/emacs-commands/run.sh` で作った一覧 (手で直さない。作り直すときは, `run.sh` を動かす)。

- **既定のキー**: `emacs -Q` の global-map のキー。global-map に無ければ, ほかのキーマップ (ミニバッファなど) のキーを, キーマップの名前と書く。
- **rs-editor**: rs-editor にあるコマンドは「実装済み」と, rs-editor のキーを書く。
- **実装**: rs-editor で書いた言語とファイル。コマンドは Lisp (`lisp/`) で書き, キーだけで動くもの (前置引数など) は Rust の核のコマンドループで扱う。Emacs の「C で書いたもの」「Lisp で書いたもの」は, Emacs の中での区別で, rs-editor での書き方とは別。
- **Edwin**: MIT/GNU Scheme 12.1 の Edwin (Emacs に似たエディタ. `src/edwin` の `define-command`) にも, 同じ名前のコマンドがあれば ◯。名前の一覧は `tools/emacs-commands/edwin-commands.txt` (`tools/emacs-commands/edwin.sh` で作る)。
- **計画**: 「エディタの基本のコマンド」の表だけにある。Lisp で書く段階の番号 (下の「実装の計画」の段階)。「済」は, 既に Lisp で書いたもの。計画は `tools/emacs-commands/plan.py` に書く。
- キーは Emacs の書き方 (`key-description`)。`C-c C-m` は `C-c RET`, `C-i` は `TAB` と書く。
- 「エディタの基本のコマンド」は, Lisp で書いたコマンド (`find-file` など) のうち, 既定の global-map にキーがあり, エディタの基本のライブラリ (`simple`, `files`, `window`, `bindings`, `lisp`, `indent`, `paragraphs`, `page`, `fill`, `newcomment`, `replace`, `isearch`, `rect`, `register`, `kmacro`, `abbrev`, `dabbrev`, `buff-menu`, `dired`, `bookmark`, `xref`, `loaddefs`, `help`, `help-fns`, `info`) にあるもの。マイナーモードのコマンド (`-mode`) と, 版管理 (`vc-`) は, 入れていない。

### 組み込みのコマンド (101 個. 実装済み 25 個: Lisp 25 個, Rust 0 個. Edwin にもある: 41 個)

| コマンド | Edwin | 既定のキー | rs-editor | rs-editor のキー | 実装 |
|---|---|---|---|---|---|
| `abort-minibuffers` |  | minibuffer-local-completion-map: `C-g`<br>minibuffer-local-isearch-map: `C-g`<br>minibuffer-local-map: `C-g` |  |  |  |
| `abort-recursive-edit` | ◯ | `C-]` | 実装済み | `C-]` | Lisp (`lisp/simple.scm`) |
| `accelerate-menu` |  |  |  |  |  |
| `add-name-to-file` |  |  |  |  |  |
| `backward-char` | ◯ | `C-b` | 実装済み | `C-b`, `<left>` | Lisp (`lisp/simple.scm`) |
| `base64-decode-region` |  |  |  |  |  |
| `base64-encode-region` |  |  |  |  |  |
| `base64url-encode-region` |  |  |  |  |  |
| `beginning-of-line` | ◯ |  | 実装済み |  | Lisp (`lisp/simple.scm`) |
| `buffer-enable-undo` |  |  |  |  |  |
| `call-last-kbd-macro` | ◯ |  | 実装済み |  | Lisp (`lisp/simple.scm`) |
| `capitalize-region` | ◯ |  | 実装済み |  | Lisp (`lisp/simple.scm`) |
| `capitalize-word` | ◯ | `M-c` | 実装済み | `M-c` | Lisp (`lisp/simple.scm`) |
| `chmod` |  |  |  |  |  |
| `copy-file` | ◯ |  | 実装済み |  | Lisp (`lisp/files.scm`) |
| `decode-coding-region` |  |  |  |  |  |
| `defining-kbd-macro` |  |  |  |  |  |
| `delete-char` | ◯ | `C-d` | 実装済み | `C-d` | Lisp (`lisp/simple.scm`) |
| `delete-frame` | ◯ | `C-x 5 0` |  |  |  |
| `delete-other-windows-internal` |  |  |  |  |  |
| `delete-process` |  |  |  |  |  |
| `delete-region` | ◯ |  |  |  |  |
| `do-auto-save` | ◯ |  | 実装済み |  | Lisp (`lisp/files.scm`) |
| `downcase-region` | ◯ | `C-x C-l` | 実装済み | `C-x C-l` | Lisp (`lisp/simple.scm`) |
| `downcase-word` | ◯ | `M-l` | 実装済み | `M-l` | Lisp (`lisp/simple.scm`) |
| `encode-coding-region` |  |  |  |  |  |
| `end-kbd-macro` | ◯ |  | 実装済み |  | Lisp (`lisp/simple.scm`) |
| `end-of-line` | ◯ |  | 実装済み |  | Lisp (`lisp/simple.scm`) |
| `erase-buffer` |  |  |  |  |  |
| `eval-buffer` |  |  |  |  |  |
| `eval-region` | ◯ |  |  |  |  |
| `exit-recursive-edit` | ◯ | `C-M-c` |  |  |  |
| `forward-char` | ◯ | `C-f` | 実装済み | `C-f`, `<right>` | Lisp (`lisp/simple.scm`) |
| `forward-line` |  |  |  |  |  |
| `forward-word` | ◯ | `M-f`, `ESC <right>` | 実装済み | `M-f` | Lisp (`lisp/simple.scm`) |
| `garbage-collect` |  |  |  |  |  |
| `goto-char` | ◯ | `M-g c` |  |  |  |
| `handle-save-session` |  | special-event-map: `<save-session>` |  |  |  |
| `handle-switch-frame` |  | `<switch-frame>` |  |  |  |
| `iconify-frame` |  |  |  |  |  |
| `indent-to` |  |  |  |  |  |
| `indent-to-column` |  |  |  |  |  |
| `insert-char` |  | `C-x 8 RET` | 実装済み |  | Lisp (`lisp/simple.scm`) |
| `kill-buffer` | ◯ | `C-x k` | 実装済み | `C-x k` | Lisp (`lisp/window.scm`) |
| `kill-emacs` |  |  |  |  |  |
| `kill-local-variable` | ◯ |  |  |  |  |
| `kill-process` |  |  |  |  |  |
| `lossage-size` |  |  |  |  |  |
| `lower-frame` |  |  |  |  |  |
| `make-frame-invisible` |  |  |  |  |  |
| `make-frame-visible` |  |  |  |  |  |
| `make-indirect-buffer` |  |  |  |  |  |
| `make-local-variable` | ◯ |  |  |  |  |
| `make-symbolic-link` |  |  |  |  |  |
| `make-variable-buffer-local` |  |  |  |  |  |
| `malloc-info` |  |  |  |  |  |
| `malloc-trim` |  |  |  |  |  |
| `modify-syntax-entry` |  |  |  |  |  |
| `move-to-column` |  | `M-g TAB` |  |  |  |
| `move-to-window-line` | ◯ |  |  |  |  |
| `narrow-to-region` | ◯ | `C-x n n` | 実装済み | `C-x n n` | Lisp (`lisp/simple.scm`) |
| `open-dribble-file` |  |  |  |  |  |
| `open-termscript` |  |  |  |  |  |
| `posix-search-backward` |  |  |  |  |  |
| `posix-search-forward` |  |  |  |  |  |
| `raise-frame` |  |  |  |  |  |
| `re-search-backward` | ◯ |  |  |  |  |
| `re-search-forward` | ◯ |  |  |  |  |
| `recenter` | ◯ | kmacro-step-edit-map: `C-l`<br>multi-query-replace-map: `C-l`<br>query-replace-map: `C-l` |  |  |  |
| `recursive-edit` |  |  |  |  |  |
| `redirect-debugging-output` |  |  |  |  |  |
| `redraw-display` | ◯ |  |  |  |  |
| `rename-buffer` | ◯ | `C-x x r` | 実装済み | `C-x x r` | Lisp (`lisp/simple.scm`) |
| `rename-file` | ◯ |  |  |  |  |
| `replace-buffer-contents` |  |  |  |  |  |
| `scroll-down` | ◯ | kmacro-step-edit-map: `M-v`, `<prior>`<br>multi-query-replace-map: `M-v`, `<prior>`<br>query-replace-map: `M-v`, `<prior>` |  |  |  |
| `scroll-left` |  | `C-x <`, `C-<next>` |  |  |  |
| `scroll-right` |  | `C-x >`, `C-<prior>` |  |  |  |
| `scroll-up` | ◯ | kmacro-step-edit-map: `C-v`, `<next>`<br>multi-query-replace-map: `C-v`, `<next>`<br>query-replace-map: `C-v`, `<next>` |  |  |  |
| `search-backward` | ◯ |  |  |  |  |
| `search-backward-regexp` |  |  |  |  |  |
| `search-forward` | ◯ | `<find>` |  |  |  |
| `search-forward-regexp` |  |  |  |  |  |
| `select-frame` |  |  |  |  |  |
| `self-insert-command` | ◯ | 文字のキー (`a` など) |  |  |  |
| `set-file-modes` |  |  |  |  |  |
| `set-frame-height` |  |  |  |  |  |
| `set-frame-width` |  |  |  |  |  |
| `signal-process` |  |  |  |  |  |
| `start-kbd-macro` | ◯ |  | 実装済み |  | Lisp (`lisp/simple.scm`) |
| `suspend-emacs` |  |  |  |  |  |
| `top-level` |  |  |  |  |  |
| `transpose-regions` |  |  |  |  |  |
| `unix-sync` |  |  |  |  |  |
| `upcase-initials-region` |  |  | 実装済み |  | Lisp (`lisp/simple.scm`) |
| `upcase-region` | ◯ | `C-x C-u` | 実装済み | `C-x C-u` | Lisp (`lisp/simple.scm`) |
| `upcase-word` | ◯ | `M-u` | 実装済み | `M-u` | Lisp (`lisp/simple.scm`) |
| `widen` | ◯ | `C-x n w` | 実装済み | `C-x n w` | Lisp (`lisp/simple.scm`) |
| `write-region` | ◯ |  | 実装済み |  | Lisp (`lisp/files.scm`) |
| `x-menu-bar-open-internal` |  |  |  |  |  |
| `x-print-frames-dialog` |  |  |  |  |  |

### 実装の計画 (エディタの基本のコマンドを Lisp で書く)

エディタの基本のコマンドを, Lisp (`lisp/`) で書く計画。TODO.md の Phase 16 の続き。目標は, 今の画面 (Tauri) と, ターミナル (ncurses) の両方で動かすこと(そのため, 段階 1 で, エディタの核を Rust に作る)。下の表の「計画」の列の番号が, ここの段階の番号。「済」は, 既に Lisp で書いたもの。段階は番号の順に進める (1 は土台で, 2 以降の前提)。画面 (Tauri) の TypeScript のコマンドは, 全て消した (画面は核の frame を描くだけ。`docs/core_migration.md`)。

| 段階 | 内容 | コマンド | 新しく書く | Rust から移す |
|---|---|---|---|---|
| 1 | 土台: Rust のエディタの核 (ターミナルの ncurses でも動かすため) | - | - | - |
| 2 | 動き | - | - | - |
| 3 | 消す, 切る, 貼る, 元に戻す | - | - | - |
| 4 | マーク, 前置引数, 実行 | - | - | - |
| 5 | S 式, 段落, ページ | - | - | - |
| 6 | 字下げ, 整形, コメント | - | - | - |
| 7 | 検索, 置換, 強調 | - | - | - |
| 8 | 矩形, レジスタ, キーボードマクロ, 略語 | 1 | 1 | - |
| 9 | ファイル, バッファ, dired, ブックマーク, シェル | - | - | - |
| 10 | ウィンドウ | 4 | 4 | - |
| 11 | ヘルプ, xref, info | - | - | - |
| 12 | 入れない | 23 | 23 | - |

**1. 土台: Rust のエディタの核 (ターミナルの ncurses でも動かすため)**

- 目標は, 同じエディタを, 今の画面 (Tauri) と, ターミナル (ncurses) の両方で動かすこと。そのため, バッファ, カーソル, マーク, undo, kill-ring, 窓を, 画面 (TypeScript) でなく Rust の核に持つ。画面は, 核の状態を描いて, キーを核に渡すだけにする。コマンドは入れない (Emacs の C の関数に当たるものを作る)。
- 【済】1.1 クレートを分ける: 画面に頼らない核のクレート (`core/`. 評価器 `scheme` も移す) と, それを使うアプリ (`src-tauri/`), ターミナルのアプリ (`tui/`)。Cargo のワークスペースにする。テストのカバレッジ 100% は, 核にも保つ。
- 【Rust の部品は済. Lisp につなぐのは 1.9】1.2 バッファ (`core/src/buffer.rs`): 本文 (rope), カーソル, 変更の有無, ファイルの名前。`point`, `goto-char`, `insert`, `delete-region`, `char-after`, `char-before`, `forward-char`, `backward-char`, `forward-line`, `line-beginning-position`, `line-end-position`, `bobp`, `eobp`, `bolp`, `eolp`, `buffer-size`, `current-column`, `move-to-column`。位置は文字で数える (今の Lisp と同じ)。
- 【Rust の部品は済. Lisp につなぐのは 1.9】1.3 マークとリージョン (`Buffer`): `mark`, `set-mark`, `push-mark`, `pop-mark`, `region-beginning`, `region-end`, `use-region-p`。
- 【Rust の部品は済. Lisp につなぐのは 1.9】1.4 元に戻す (`Buffer`): 変更の履歴 (`undo-boundary`, `primitive-undo`)。今の CodeMirror の履歴と同じ動き (まとめ方, 重ねた undo) にする。
- 【Rust の部品は済. Lisp につなぐのは 1.9】1.5 kill-ring (`core/src/kill_ring.rs`): `kill-region`, `current-kill`, `kill-append`, `insert-for-yank`。
- 【Rust の部品は済 (`core/src/search.rs`, `core/src/syntax.rs`). Lisp につなぐのは 1.9】1.6 探す: `search-forward`, `search-backward`, `re-search-forward`, `re-search-backward`, `looking-at`, `match-beginning`, `match-end` (Emacs の正規表現の書き方に合わせる)。構文: `scan-lists`, `scan-sexps`, `syntax-ppss`, `char-syntax` (`src/lib/syntax.ts` を Rust に移した)。どちらも `emacs -Q --batch` の結果と全ての位置で比べる (`tools/syntax/run.sh`)。
- 【Rust の部品は済 (`core/src/window.rs`, `core/src/editor.rs`). Lisp につなぐのは 1.9】1.7 窓とバッファの一覧: `selected-window`, `split-window`, `delete-window`, `window-list`, `set-window-buffer`, `set-buffer`, `get-buffer`, `generate-new-buffer`, `with-current-buffer`, バッファごとの変数 (`make-local-variable`)。
- 【前置引数の部品は済 (`Editor::prefix_arg`, `prefix_numeric_value`). `interactive` は 1.9】1.8 前置引数: `current-prefix-arg`, `prefix-numeric-value`。`define-command` に, 引数の読み方 (Emacs の `interactive`) を足す。
- 【Rust は済 (`core/src/scheme/editor_host.rs`: `Editor` を直接動かす `EditorHost`. 63 個の組み込み). 画面 (Tauri) も 1.11 でつないだ】1.9 評価器を核につなぐ: 今の `Host` (画面に頼んで返事を待つ) をやめ, 評価器が核を直接動かす。画面に頼むのは, 描くこと, ミニバッファの入力, ファイルの選択だけにする (`Host` を小さくする)。
- 【済 (`tui/`. 動いている: 移動, 編集, kill, yank, undo, 窓の分割, ファイルを開く, 保存, M-x, 終了。基本のコマンドは `lisp/simple.scm`)】1.10 ターミナルのアプリの骨組み (`tui/`. ncurses): 核の状態を描き, キーを核に渡す。カーソルを動かし, 文字を入れ, 保存できるところまで。
- 【済 (`docs/core_migration.md`. 画面は `src-tauri/src/session.rs` の `Session` で核を動かし, frame を描く)】1.11 今の画面 (Tauri) を核につなぎ替える: 本文の持ち主を CodeMirror から核に移す。CodeMirror は表示と入力にする。画面の TypeScript のコマンドは全て消した。

**2. 動き**

- 【Lisp 版は済 (`lisp/simple.scm`). `emacs -Q --batch` の結果と, 本文の全ての位置で一致 (`core/tests/test_motion.rs`. 392 通り). 画面 (Tauri) の TypeScript 版は消した (1.11)】行, 単語, バッファの端, 左右の移動。核の上の Lisp で書く。カーソルを動かすので, 速さを比べてから Lisp に移す。

**3. 消す, 切る, 貼る, 元に戻す**

- 【Lisp 版は済 (`lisp/simple.scm`). `emacs -Q --batch` の結果と一致 (`core/tests/test_edit.rs`: 57 通りのコマンドを 7 種類の本文の全ての位置から, `core/tests/test_undo.rs`: undo と undo-redo の 10 通りの場面). 画面 (Tauri) の TypeScript 版は消した (1.11)】違いが残るのは, `default-indent-new-line` (字下げは段階 6), `quoted-insert` (1 文字を表すキーだけ), `cycle-spacing` (最初の呼び出しだけを Emacs と比べた), `undo` が保存した状態に戻っても「変更なし」にならないこと。kill-ring を使うコマンドと, 空白と改行を整えるコマンド, 入れ替え, `undo`。

**4. マーク, 前置引数, 実行**

- 【Lisp 版とターミナルのアプリは済 (`lisp/simple.scm`, `tui/`). マークは `emacs -Q --batch` の結果と一致 (`core/tests/test_edit.rs`). `next-error`, `previous-error`, `first-error`, `compile-goto-error`, `grep` は `lisp/compile.scm` (`core/tests/test_compile.rs`). `grep` は外のコマンドを使わず Lisp の `rust-grep` で探し, `*grep*` と `*xref*` の `ファイル:行:行の文字` の行をたどる. 違いが残るのは, 前置引数で進む数を言えないことと, compilation-mode が無いこと (`compile` はまだ無い), ターミナルのアプリでは `local-set-key` が何もせず `*grep*` で `RET` が `compile-goto-error` にならないこと。画面 (Tauri) の TypeScript 版は消した (1.11)】マークとリージョン, 前置引数 (`C-u`, `M-数字`), `M-x`, `M-X`, `M-:`, `C-x M-:`, `C-g`, 表示の切り替え (`truncate-lines`, `selective-display`)。

**5. S 式, 段落, ページ**

- 【式, defun, 段落, 文は済 (`lisp/simple.scm`, 計算は `core/src/sexp.rs` と `core/src/paragraph.rs`). `emacs -Q --batch` の結果と一致 (`core/tests/test_lisp.rs`: 3 つのモードで, 50 通りのコマンドを全ての位置から, 対話的に). ページ (`forward-page`, `mark-page`) と入れ替え (`transpose-sexps`, `transpose-sentences`, `transpose-paragraphs`) も済 (`transpose-sexps` は, バックスラッシュで quote した文字と `##` `,@` の前後だけ Emacs と違う)。`insert-parentheses`、`move-past-close-and-reindent`、`count-lines-page` も済 (`move-past-close-and-reindent` は、scheme-mode の `{}` と `|sym (|` を含む本文の字下げだけ Emacs と違う)。絞り込みは、核に作った (`Buffer::narrow_to_region`、`widen`、`save-restriction`、`buffer-narrowed-p`。点、移動、検索、構文の走査、`buffer-string`、`erase-buffer`、窓の描画が、見える範囲だけを扱う。`goto-line` は、絞り込みの外の行なら絞り込みをやめる)。`narrow-to-region`、`widen`、`narrow-to-page`、`narrow-to-defun` は、Emacs と一致 (`core/tests/test_lisp.rs`、`core/tests/test_narrow.rs`)。違いが残るのは、段落、文、字下げ、整形の計算 (`forward-paragraph` など) が、絞り込みを見ずに本文の全体を読むこと、画面 (Tauri) がまだ絞り込めないこと。画面 (Tauri) の TypeScript 版は消した (1.11)】括弧の移動と消去 (`C-M-f` など), defun, 文と段落, ページ。構文表 (段階 1.6) を使う。

**6. 字下げ, 整形, コメント**

- 【Lisp 版は済 (`lisp/simple.scm`, 計算は字下げが `core/src/indent.rs`, 整形が `core/src/fill.rs`, コメントが `core/src/comment.rs`). `emacs -Q --batch` の結果と一致 (`core/tests/test_lisp.rs`: `indent-for-tab-command`, `indent-region`, `indent-sexp`, `newline`, `indent-rigidly`, `tab-to-tab-stop`, `set-fill-prefix`, `fill-paragraph`, `comment-dwim`, `comment-line` を, 本文の全ての位置から. 日本語の折り (禁則) は Emacs と別の決まりで, `core/tests/test_fill.rs` で試す). 違いが残るのは, Lisp の `fill-paragraph` が文字列と `#| |#` の中の段落を詰めないこと, `comment-dwim` が `#| |#` に対応しないこと, `comment-dwim` と `comment-line` のリージョンが行の途中で始まるか終わるとき (Emacs は途中から入れる. こちらは行ごと), `indent-rigidly` が前置引数で動かすだけで, 矢印キーの続きの入力は無いこと, `comment-set-column` が桁を決めるだけで, コメントの置き直しをしないこと。画面 (Tauri) の TypeScript 版は消した (1.11)】`TAB`, `C-M-\`, `M-q`, `M-;`。字下げの決まりと整形は, ターミナルでも使うため, Rust に移した。エディタと整形の道具 (TODO の Phase 18) で, 決まりを 1 つにする。

**7. 検索, 置換, 強調**

- 【Lisp 版は済 (`lisp/isearch.scm`, `lisp/replace.scm`. ターミナルのアプリが読む). `emacs -Q --batch` の結果と一致: `replace-string`, `replace-regexp`, `query-replace`, `query-replace-regexp` (`core/tests/test_replace.rs`), `occur` (`core/tests/test_occur.rs`), `isearch-forward` と `isearch-backward` と正規表現の版 (`core/tests/test_isearch.rs`: Emacs のコマンドを, キーの順に呼んだ結果と, 全ての位置から比べる)、`isearch-forward-thing-at-point` と `isearch-forward-symbol-at-point` (カーソルの所の物を初めの文字列にする。物は `region`、`symbol`、`sexp` で、`url` は無い。変数 `isearch-forward-thing-at-point` は、同じ名前の命令があるので `isearch-forward-thing-at-point-things`)。強調 (`highlight-regexp`, `unhighlight-regexp`, `highlight-phrase`, `highlight-symbol-at-point`, `highlight-lines-matching-regexp`) は, 核が正規表現を持ち (`Buffer::highlights`), ターミナルは下線で描く (`core/tests/test_highlight.rs`). 違いが残るのは, 記号の検索 (`isearch-forward-symbol`) で空白や正規表現の特別な文字を打つこと, 空白を打つと, 続く空白の全てに合う (lax whitespace) 動きが無いこと, isearch が, 別のキーで終えたときに, そのキーのコマンドを動かさないこと, `\#` `\,` `\?` の置き換えと, 置き換えの取り消し (`u`, `^`) と, 編集 (`e`) が無いこと, `occur` が *Occur* の色と, 複数のバッファを扱わないこと, 強調の色の名前を聞かないこと。画面 (Tauri) の TypeScript 版は消した (1.11)】isearch, `query-replace`, `occur`, `highlight-regexp`。検索の状態を核に持つ (描き方は, 画面ごと)。

**8. 矩形, レジスタ, キーボードマクロ, 略語**

- 矩形 (`C-x r`), レジスタ, キーボードマクロ (`F3`, `F4`, `C-x (`), 略語と `M-/`。
- 【矩形とレジスタとキーボードマクロは Lisp 版が済 (`lisp/rect.scm`, `lisp/register.scm`, `lisp/simple.scm`). `emacs -Q --batch` の結果と一致: 矩形 (`clear-rectangle`, `delete-rectangle`, `open-rectangle`, `rectangle-number-lines`, `kill-rectangle`, `copy-rectangle-as-kill` と `yank-rectangle`) と, レジスタ (`copy-rectangle-to-register`, `number-to-register`, `increment-register`, `append-to-register`, `point-to-register`, `jump-to-register`) (`core/tests/test_lisp.rs`, `core/tests/test_edit.rs`). ウィンドウ構成のレジスタ (`window-configuration-to-register`) は, バッチの Emacs に窓が無いので Rust のテストだけ (`core/tests/test_register.rs`, `core/tests/test_editor.rs`). キーボードマクロの `kmacro-*` (カウンター, `F3`, `F4`, 名前を付ける) も, 同じ理由で `tui/tests/test_app.rs` で試す. 違いが残るのは, 幅の違う文字 (全角, 曖昧幅) を含む行の矩形の桁の数え方, `kmacro-end-call-mouse` (マウス) と, キーボードマクロのリング (`kmacro-cycle-ring-next` など) が無いこと。略語 (`lisp/abbrev.scm`: `expand-abbrev`, `abbrev-prefix-mark`, `add-global-abbrev`, `add-mode-abbrev`, `inverse-add-global-abbrev`, `inverse-add-mode-abbrev`) も済で, `emacs -Q --batch` の結果と一致 (`core/tests/test_edit.rs`: 大文字と小文字の展開, 前置引数, 取り消し, 作り直しの問い, 接頭辞の印を, 本文の全ての位置から). 違いが残るのは, 略語の表がメジャーモードごとではなく, 全てのバッファで 1 つのローカルの表と, グローバルの表だけなこと, `abbrev-mode` (打つと自動で展開する) と, 略語の保存 (`write-abbrev-file`) と, 表の親 (`:parents`), 展開のフックの `no-self-insert` が無いこと。`dabbrev-completion` (`lisp/dabbrev.scm`) と, 補完の土台 (`completion-at-point`, `lisp/minibuffer.scm`) も済で, `dabbrev-completion` は `emacs -Q --batch` の結果と一致 (`core/tests/test_edit.rs`: 本文の全ての位置から, 補完した本文と, カーソルと, 出したメッセージ). `complete-symbol` は, 補完する名前が Emacs と別 (Emacs は Emacs Lisp の記号, こちらは評価器に定義された Scheme の名前) なので, `core/tests/test_minibuffer.rs` で試す。補完の違いは, 候補の選び方が先頭の一致だけ (partial-completion などの方式が無い), `*Completions*` が 1 行ずつの簡単な一覧 (列に並べず, 選んで確定もできない), `dabbrev-completion` が他のバッファを探さない (前置引数を付けても今のバッファだけ) こと。`dabbrev-expand` の違いは, 展開の後に空白を入れて続けると次の語を取る機能と, 他のバッファを探す機能が無いこと。略語の違いを無くすには, (1) メジャーモードごとの略語の表と `abbrev-mode`: エディタにメジャーモードの仕組み (バッファごとのモードと, 変数 `local-abbrev-table`) と, 文字を打つたびに略語を展開するか決める処理 (Emacs の `self-insert-command` の後の `abbrev-mode` の判定) を足す必要があり, エディタ側の変更が大きい。 (2) 略語の保存 (`write-abbrev-file`, `read-abbrev-file`) と表の親 (`:parents`), 展開のフックの `no-self-insert` は, Lisp だけで書ける。】

**9. ファイル, バッファ, dired, ブックマーク, シェル**

- ファイルとバッファの残りのコマンド, `list-buffers`, dired, ブックマーク, `M-!`, `M-|`, `M-&`, バッファの複製。
- `save-buffers-kill-terminal` は, アプリを終える頼みを `Host` に足してから移す。
- 【シェルとバッファの命令は Lisp 版が済 (`lisp/simple.scm`, `lisp/files.scm`). `emacs -Q --batch` の結果と一致: `shell-command` と `shell-command-on-region` (出力のバッファ, 今のバッファに入れる, 領域の置き換え, エコー領域か別のバッファか, 終了コードのメッセージ), `clone-buffer` (本文, 点, マーク, 絞り込み), `not-modified`, 読み取り専用のバッファでの編集の失敗 (`insert`, `delete-char`, `kill-word`, `upcase-region` など) (`core/tests/test_edit.rs`). `async-shell-command`, `rename-buffer`, `rename-uniquely`, `read-only-mode`, `find-file-read-only`, `revert-buffer-quick`, `list-directory`, `clone-indirect-buffer-other-window` は 名前, ファイル, 問いが絡むので Rust で試す (`core/tests/test_simple_buffers.rs`, `core/tests/test_editor.rs`, `core/tests/test_buffer.rs`). 違いが残るのは, 非同期のシェルのコマンド (`&` で終わる命令と `async-shell-command`) を終わりまで動かしてから次を読むこと (プロセスのバッファ, 途中の出力, 止めることが無い), 標準エラーを別のバッファに分けられないこと (`error-buffer` は使わず, 出力に混ぜる), 領域が不連続のとき (`region-noncontiguous-p`) と `shell-command-dont-erase-buffer` が無いこと, `clone-indirect-buffer-other-window` が本文を共有する間接バッファではなく, 本文を写した双子のバッファになること, `clone-buffer` がメジャーモードとバッファごとの変数を写さないこと (モードが無い), `read-only-mode` がマイナーモードではなく命令で, 文字ごとの `read-only` の属性と `inhibit-read-only` と `view-mode` が無いこと, `list-directory` の一覧が `ls` の出力のままで (`ls -CF` の列の幅は端末の幅で決まる), `*Directory*` が help-mode でなく, 取り消しの記録を止めないこと, `revert-buffer-quick` が自動保存のファイルからの読み直し (前置引数) と `revert-buffer-quick-short-answers` に対応しないこと, `find-file-read-only` がワイルドカードを展開しないことと, `rename-buffer` が空白で始まる名前に乱数を付けないこと。画面 (Tauri) の TypeScript 版は消した (1.11)】

**10. ウィンドウ**

- 窓の分割, 削除, 移動, 大きさ, スクロール, 別の窓に出すコマンド。窓の木 (`src/lib/windows.ts`) は, 段階 1.7 で核に移したものを使う。【Lisp 版とターミナルのアプリは済 (`lisp/window.scm`, `lisp/simple.scm`, `core/src/window.rs`, `tui/`). バッチの Emacs には窓が 1 つしか無いので, `emacs -Q --batch` との比べは無く, Rust のテストで試す (`core/tests/test_window_commands.rs`, `core/tests/test_window.rs`, `core/tests/test_editor.rs`, `tui/tests/test_app.rs`). 窓の木に分ける割合を持たせ (`enlarge-window`, `shrink-window`, `balance-windows`, `fit-window-to-buffer` などが変える), 画面の大きさを核に知らせて (`Editor::set_screen_size`), 窓の大きさを行と桁で数える。違いが残るのは, 窓ごとの点 (`window-point`) と先頭の行 (`window-start`) が無く, 点はバッファごとなので, 同じバッファを出した窓では `scroll-other-window` が選んだ窓の点も動かすこと, `next-buffer` と `previous-buffer` が, 窓ごとの前のバッファの履歴ではなく, バッファの一覧の順に替えること, `quit-window` と `delete-windows-on` がバッファを後ろに回さないこと (`bury-buffer` が無い), 大きさを変えるときに, 隣の窓の最小の高さを守らず, 割合を 5% から 95% に収めるだけなこと, `fit-window-to-buffer` が高さだけを合わせること (幅, `fit-window-to-buffer-horizontally`, 最大の高さの変数が無い), `enlarge-window` が行の数で言った量にぴったり合わないことがある (割合から丸めるため) こと, `same-window-prefix`, `other-window-prefix`, `toggle-window-dedicated`, `window-toggle-side-windows` (窓を固定する, 横の窓) が無いこと, `move-to-window-line-top-bottom` が `scroll-margin` を見ないこと。画面 (Tauri) の TypeScript 版は消した (1.11)】

**11. ヘルプ, xref, info**

- `C-h` のコマンドと, 定義へ移るコマンド (`M-.`), info。ヘルプは Phase 17 (docstring と多言語化) と一緒に進める。【Lisp 版とターミナルのアプリは済 (`lisp/help.scm`, `lisp/xref.scm`, `lisp/info.scm`, `tui/`). `emacs -Q --batch` には窓も info のマニュアルも無いので, Rust のテストで試す (`core/tests/test_help.rs`, `tui/tests/test_app.rs`). `describe-variable`, `describe-command`, `describe-symbol`, `describe-key`, `where-is`, `describe-bindings`, `view-lossage`, `view-echo-area-messages`, `describe-syntax`, `xref-find-definitions`, `xref-find-references`, `xref-find-apropos`, `xref-go-back`, `info` などがある。違いが残るのは, `*Help*` が help-mode でなく (リンクも履歴も無い), 定義を探すのが `grep` の正規表現で (etags や LSP の backend が無い), `xref-find-references` の結果が `*xref*` の簡単な一覧なこと, `info` が Info のノードの移動 (`n`, `p`, `u`, 索引) を持たず, マニュアルの文章をそのまま出すことと, `describe-char`, `describe-face`, `apropos-command` などが無いこと。画面 (Tauri) の TypeScript 版は消した (1.11)】

**12. 入れない**

- rs-editor に当たる機能が無いので, 入れない。メール, フレーム (複数の窓枠), 日本語入力以外のテキスト変換, Emacs の案内の文書。

### エディタの基本のコマンド (Lisp で書いたもの. 262 個. 実装済み 234 個: Lisp 234 個, Rust 0 個. Edwin にもある: 135 個)

| コマンド | Edwin | ライブラリ | 既定のキー | rs-editor | rs-editor のキー | 実装 | 計画 |
|---|---|---|---|---|---|---|---|
| `analyze-text-conversion` |  | simple | `<text-conversion>` |  |  |  | 12 |
| `append-next-kill` | ◯ | simple | `C-M-w` | 実装済み | `C-M-w` | Lisp (`lisp/simple.scm`) | 済 |
| `async-shell-command` |  | simple | `M-&` | 実装済み | `M-&` | Lisp (`lisp/simple.scm`) | 済 |
| `back-to-indentation` | ◯ | simple | `M-m` | 実装済み | `M-m` | Lisp (`lisp/simple.scm`) | 済 |
| `backward-kill-word` | ◯ | simple | `M-DEL`, `C-<backspace>` | 実装済み | `M-DEL` | Lisp (`lisp/simple.scm`) | 済 |
| `backward-word` | ◯ | simple | `M-b`, `ESC <left>` | 実装済み | `M-b` | Lisp (`lisp/simple.scm`) | 済 |
| `beginning-of-buffer` | ◯ | simple | `M-<`, `<begin>`, `C-<home>` | 実装済み | `M-<` | Lisp (`lisp/simple.scm`) | 済 |
| `clone-buffer` |  | simple | `C-x x n` | 実装済み | `C-x x n` | Lisp (`lisp/simple.scm`) | 済 |
| `clone-indirect-buffer-other-window` |  | simple | `C-x 4 c` | 実装済み | `C-x 4 c` | Lisp (`lisp/simple.scm`) | 済 |
| `compose-mail` |  | simple | `C-x m` |  |  |  | 12 |
| `compose-mail-other-frame` |  | simple | `C-x 5 m` |  |  |  | 12 |
| `compose-mail-other-window` |  | simple | `C-x 4 m` |  |  |  | 12 |
| `count-words-region` |  | simple | `M-=` | 実装済み | `M-=` | Lisp (`lisp/simple.scm`) | 済 |
| `cycle-spacing` |  | simple | `M-SPC` | 実装済み | `M-SPC` | Lisp (`lisp/simple.scm`) | 済 |
| `default-indent-new-line` |  | simple | `M-j`, `C-M-j` | 実装済み | `M-j`, `C-M-j` | Lisp (`lisp/simple.scm`) | 済 |
| `delete-backward-char` | ◯ | simple | `DEL` | 実装済み | `DEL` | Lisp (`lisp/simple.scm`) | 済 |
| `delete-blank-lines` | ◯ | simple | `C-x C-o` | 実装済み | `C-x C-o` | Lisp (`lisp/simple.scm`) | 済 |
| `delete-forward-char` |  | simple | `<deletechar>` | 実装済み | `<delete>` | Lisp (`lisp/simple.scm`) | 済 |
| `delete-horizontal-space` | ◯ | simple | `M-\` | 実装済み | `M-\\` | Lisp (`lisp/simple.scm`) | 済 |
| `delete-indentation` | ◯ | simple | `M-^` | 実装済み | `M-^` | Lisp (`lisp/simple.scm`) | 済 |
| `digit-argument` | ◯ | simple | `C-0`, `C-1`, `C-2`, `C-3` ほか 16 | 実装済み | `M-0`, `M-1`, `M-2` ほか 7 | Lisp (`lisp/simple.scm`) | 済 |
| `end-of-buffer` | ◯ | simple | `M->`, `C-<end>` | 実装済み | `M->` | Lisp (`lisp/simple.scm`) | 済 |
| `eval-expression` | ◯ | simple | `M-:`, `ESC M-:` | 実装済み | `M-:` | Lisp (`lisp/simple.scm`) | 済 |
| `exchange-point-and-mark` | ◯ | simple | `C-x C-x` | 実装済み | `C-x C-x` | Lisp (`lisp/simple.scm`) | 済 |
| `execute-extended-command` | ◯ | simple | `M-x`, `<menu>`, `<execute>` | 実装済み | `M-x` | Lisp (`lisp/simple.scm`) | 済 |
| `execute-extended-command-for-buffer` |  | simple | `M-X` | 実装済み | `M-X` | Lisp (`lisp/simple.scm`) | 済 |
| `goto-line` | ◯ | simple | `M-g g`, `M-g M-g` | 実装済み | `M-g g`, `M-g M-g` | Lisp (`lisp/simple.scm`) | 済 |
| `goto-line-relative` |  | simple | `C-x n g` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `insert-buffer` | ◯ | simple | `C-x x i` | 実装済み | `C-x x i` | Lisp (`lisp/simple.scm`) | 済 |
| `keyboard-escape-quit` |  | simple | `ESC ESC ESC` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `keyboard-quit` | ◯ | simple | `C-g` | 実装済み | `C-g` | Lisp (`lisp/simple.scm`) | 済 |
| `kill-line` | ◯ | simple | `C-k`, `<deleteline>` | 実装済み | `C-k` | Lisp (`lisp/simple.scm`) | 済 |
| `kill-region` | ◯ | simple | `C-w`, `<cut>`, `S-<delete>` | 実装済み | `C-w` | Lisp (`lisp/simple.scm`) | 済 |
| `kill-ring-save` |  | simple | `M-w`, `<copy>`, `C-<insert>`, `C-<insertchar>` | 実装済み | `M-w` | Lisp (`lisp/simple.scm`) | 済 |
| `kill-whole-line` |  | simple | `C-S-<backspace>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `kill-word` | ◯ | simple | `M-d`, `C-<delete>` | 実装済み | `M-d` | Lisp (`lisp/simple.scm`) | 済 |
| `mark-whole-buffer` | ◯ | simple | `C-x h` | 実装済み | `C-x h` | Lisp (`lisp/simple.scm`) | 済 |
| `mark-word` | ◯ | simple | `M-@` | 実装済み | `M-@` | Lisp (`lisp/simple.scm`) | 済 |
| `move-beginning-of-line` |  | simple | `C-a`, `<home>` | 実装済み | `C-a`, `<home>` | Lisp (`lisp/simple.scm`) | 済 |
| `move-end-of-line` |  | simple | `C-e`, `<end>` | 実装済み | `C-e`, `<end>` | Lisp (`lisp/simple.scm`) | 済 |
| `negative-argument` | ◯ | simple | `C--`, `M--`, `C-M--` | 実装済み | `M--`, `C--` | Lisp (`lisp/simple.scm`) | 済 |
| `newline` | ◯ | simple | `RET` | 実装済み | `RET`, `C-m` | Lisp (`lisp/simple.scm`) | 済 |
| `next-error` |  | simple | `` C-x ` ``, `M-g n`, `M-g M-n` | 実装済み | `M-g n`, `M-g M-n`, `` C-x ` `` | Lisp (`lisp/compile.scm`) | 済 |
| `next-line` | ◯ | simple | `C-n`, `<down>` | 実装済み | `C-n`, `<down>` | Lisp (`lisp/simple.scm`) | 済 |
| `open-line` | ◯ | simple | `C-o`, `<insertline>` | 実装済み | `C-o` | Lisp (`lisp/simple.scm`) | 済 |
| `pop-global-mark` |  | simple | `C-x C-@`, `C-x C-SPC` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `previous-error` |  | simple | `M-g p`, `M-g M-p` | 実装済み | `M-g p`, `M-g M-p` | Lisp (`lisp/compile.scm`) | 済 |
| `previous-line` | ◯ | simple | `C-p`, `<up>` | 実装済み | `C-p`, `<up>` | Lisp (`lisp/simple.scm`) | 済 |
| `quoted-insert` | ◯ | simple | `C-q` | 実装済み | `C-q` | Lisp (`lisp/simple.scm`) | 済 |
| `repeat-complex-command` | ◯ | simple | `<redo>`, `<again>`, `C-x M-:`, `C-x ESC ESC` | 実装済み | `C-x M-:` | Lisp (`lisp/simple.scm`) | 済 |
| `set-fill-column` | ◯ | simple | `C-x f` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `set-goal-column` | ◯ | simple | `C-x C-n` | 実装済み | `C-x C-n` | Lisp (`lisp/simple.scm`) | 済 |
| `set-mark-command` | ◯ | simple | `C-@`, `C-SPC` | 実装済み | `C-SPC`, `C-@` | Lisp (`lisp/simple.scm`) | 済 |
| `set-selective-display` |  | simple | `C-x $` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `shell-command` | ◯ | simple | `M-!` | 実装済み | `M-!` | Lisp (`lisp/simple.scm`) | 済 |
| `shell-command-on-region` | ◯ | simple | `M-\|` | 実装済み | `M-\|` | Lisp (`lisp/simple.scm`) | 済 |
| `split-line` | ◯ | simple | `C-M-o` | 実装済み | `C-M-o` | Lisp (`lisp/simple.scm`) | 済 |
| `toggle-truncate-lines` |  | simple | `C-x x t` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `transpose-chars` | ◯ | simple | `C-t` | 実装済み | `C-t` | Lisp (`lisp/simple.scm`) | 済 |
| `transpose-lines` | ◯ | simple | `C-x C-t` | 実装済み | `C-x C-t` | Lisp (`lisp/simple.scm`) | 済 |
| `transpose-sexps` | ◯ | simple | `C-M-t` | 実装済み | `C-M-t` | Lisp (`lisp/simple.scm`) | 済 |
| `transpose-words` | ◯ | simple | `M-t` | 実装済み | `M-t` | Lisp (`lisp/simple.scm`) | 済 |
| `undo` | ◯ | simple | `C-/`, `C-_`, `C-x u`, `<undo>` | 実装済み | `C-/`, `C-_`, `C-x u` | Lisp (`lisp/simple.scm`) | 済 |
| `undo-redo` |  | simple | `C-?`, `C-M-_` | 実装済み | `C-M-_` | Lisp (`lisp/simple.scm`) | 済 |
| `universal-argument` | ◯ | simple | `C-u` | 実装済み | `C-u` | Lisp (`lisp/simple.scm`) | 済 |
| `what-cursor-position` | ◯ | simple | `C-x =` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `yank` | ◯ | simple | `C-y`, `<paste>`, `S-<insert>`, `S-<insertchar>` | 実装済み | `C-y` | Lisp (`lisp/simple.scm`) | 済 |
| `yank-pop` | ◯ | simple | `M-y` | 実装済み | `M-y` | Lisp (`lisp/simple.scm`) | 済 |
| `zap-to-char` |  | simple | `M-z` | 実装済み | `M-z` | Lisp (`lisp/simple.scm`) | 済 |
| `find-alternate-file` | ◯ | files | `C-x C-v` | 実装済み | `C-x C-v` | Lisp (`lisp/files.scm`) | 済 |
| `find-file` | ◯ | files | `<open>`, `C-x C-f` | 実装済み | `C-x C-f` | Lisp (`lisp/files.scm`) | 済 |
| `find-file-other-frame` | ◯ | files | `C-x 5 f`, `C-x 5 C-f` |  |  |  | 12 |
| `find-file-other-window` | ◯ | files | `C-x 4 f`, `C-x 4 C-f` | 実装済み | `C-x 4 f`, `C-x 4 C-f` | Lisp (`lisp/files.scm`) | 済 |
| `find-file-read-only` |  | files | `C-x C-r` | 実装済み | `C-x C-r` | Lisp (`lisp/files.scm`) | 済 |
| `find-file-read-only-other-frame` |  | files | `C-x 5 r` |  |  |  | 12 |
| `find-file-read-only-other-window` |  | files | `C-x 4 r` | 実装済み | `C-x 4 r` | Lisp (`lisp/files.scm`) | 済 |
| `insert-file` | ◯ | files | `C-x i` | 実装済み | `C-x i` | Lisp (`lisp/files.scm`) | 済 |
| `list-directory` | ◯ | files | `C-x C-d` | 実装済み | `C-x C-d` | Lisp (`lisp/files.scm`) | 済 |
| `not-modified` | ◯ | files | `M-~` | 実装済み | `M-~` | Lisp (`lisp/files.scm`) | 済 |
| `rename-uniquely` |  | files | `C-x x u` | 実装済み | `C-x x u` | Lisp (`lisp/files.scm`) | 済 |
| `revert-buffer-quick` |  | files | `C-x x g` | 実装済み | `C-x x g` | Lisp (`lisp/files.scm`) | 済 |
| `save-buffer` | ◯ | files | `C-x C-s` | 実装済み | `C-x C-s` | Lisp (`lisp/files.scm`) | 済 |
| `save-buffers-kill-terminal` |  | files | `C-x C-c` | 実装済み | `C-x C-c` | Lisp (`lisp/simple.scm`) | 済 |
| `save-some-buffers` | ◯ | files | `C-x s` | 実装済み | `C-x s` | Lisp (`lisp/files.scm`) | 済 |
| `write-file` | ◯ | files | `C-x C-w` | 実装済み | `C-x C-w` | Lisp (`lisp/files.scm`) | 済 |
| `balance-windows` |  | window | `C-x +` | 実装済み | `C-x +` | Lisp (`lisp/window.scm`) | 済 |
| `beginning-of-buffer-other-window` |  | window | `M-<home>`, `M-<begin>`, `ESC <home>`, `ESC <begin>` | 実装済み | `M-<home>` | Lisp (`lisp/window.scm`) | 済 |
| `delete-other-windows` | ◯ | window | `C-x 1` | 実装済み | `C-x 1` | Lisp (`lisp/simple.scm`) | 済 |
| `delete-window` | ◯ | window | `C-x 0` | 実装済み | `C-x 0` | Lisp (`lisp/simple.scm`) | 済 |
| `delete-windows-on` |  | window | `C-x w 0` | 実装済み | `C-x w 0` | Lisp (`lisp/window.scm`) | 済 |
| `display-buffer` |  | window | `C-x 4 C-o` | 実装済み | `C-x 4 C-o` | Lisp (`lisp/window.scm`) | 済 |
| `display-buffer-other-frame` |  | window | `C-x 5 C-o` |  |  |  | 12 |
| `end-of-buffer-other-window` |  | window | `M-<end>`, `ESC <end>` | 実装済み | `M-<end>` | Lisp (`lisp/window.scm`) | 済 |
| `enlarge-window` | ◯ | window | `C-x ^` | 実装済み | `C-x ^` | Lisp (`lisp/window.scm`) | 済 |
| `enlarge-window-horizontally` | ◯ | window | `C-x }` | 実装済み | `C-x }` | Lisp (`lisp/window.scm`) | 済 |
| `fit-window-to-buffer` |  | window | `C-x w -` | 実装済み | `C-x w -` | Lisp (`lisp/window.scm`) | 済 |
| `handle-select-window` |  | window | `<select-window>` |  |  |  | 12 |
| `kill-buffer-and-window` | ◯ | window | `C-x 4 0` | 実装済み | `C-x 4 0` | Lisp (`lisp/window.scm`) | 済 |
| `move-to-window-line-top-bottom` |  | window | `M-r` | 実装済み | `M-r` | Lisp (`lisp/window.scm`) | 済 |
| `next-buffer` |  | window | `C-x <right>`, `<XF86Forward>`, `C-x C-<right>` | 実装済み | `C-x <right>`, `C-x C-<right>` | Lisp (`lisp/window.scm`) | 済 |
| `other-window` | ◯ | window | `C-x o` | 実装済み | `C-x o` | Lisp (`lisp/simple.scm`) | 済 |
| `other-window-prefix` |  | window | `C-x 4 4` |  |  |  | 10 |
| `previous-buffer` |  | window | `<XF86Back>`, `C-x <left>`, `C-x C-<left>` | 実装済み | `C-x <left>`, `C-x C-<left>` | Lisp (`lisp/window.scm`) | 済 |
| `quit-window` |  | window | `C-x w q` | 実装済み | `C-x w q` | Lisp (`lisp/window.scm`) | 済 |
| `recenter-other-window` |  | window | `C-M-S-l` | 実装済み |  | Lisp (`lisp/window.scm`) | 済 |
| `recenter-top-bottom` |  | window | `C-l` | 実装済み | `C-l` | Lisp (`lisp/simple.scm`) | 済 |
| `same-window-prefix` |  | window | `C-x 4 1` |  |  |  | 10 |
| `scroll-down-command` |  | window | `M-v`, `<prior>` | 実装済み | `M-v`, `<prior>` | Lisp (`lisp/simple.scm`) | 済 |
| `scroll-other-window` | ◯ | window | `C-M-v`, `M-<next>`, `ESC <next>` | 実装済み | `C-M-v` | Lisp (`lisp/window.scm`) | 済 |
| `scroll-other-window-down` | ◯ | window | `C-M-S-v`, `M-<prior>`, `ESC <prior>` | 実装済み |  | Lisp (`lisp/window.scm`) | 済 |
| `scroll-up-command` |  | window | `C-v`, `<next>` | 実装済み | `C-v`, `<next>` | Lisp (`lisp/simple.scm`) | 済 |
| `shrink-window-horizontally` | ◯ | window | `C-x {` | 実装済み | `C-x {` | Lisp (`lisp/window.scm`) | 済 |
| `shrink-window-if-larger-than-buffer` | ◯ | window | `C-x -` | 実装済み | `C-x -` | Lisp (`lisp/window.scm`) | 済 |
| `split-root-window-below` |  | window | `C-x w 2` | 実装済み | `C-x w 2` | Lisp (`lisp/window.scm`) | 済 |
| `split-root-window-right` |  | window | `C-x w 3` | 実装済み | `C-x w 3` | Lisp (`lisp/window.scm`) | 済 |
| `split-window-below` |  | window | `C-x 2` | 実装済み | `C-x 2` | Lisp (`lisp/simple.scm`) | 済 |
| `split-window-right` |  | window | `C-x 3` | 実装済み | `C-x 3` | Lisp (`lisp/simple.scm`) | 済 |
| `switch-to-buffer` | ◯ | window | `C-x b` | 実装済み | `C-x b` | Lisp (`lisp/window.scm`) | 済 |
| `switch-to-buffer-other-frame` | ◯ | window | `C-x 5 b` |  |  |  | 12 |
| `switch-to-buffer-other-window` | ◯ | window | `C-x 4 b` | 実装済み | `C-x 4 b` | Lisp (`lisp/window.scm`) | 済 |
| `toggle-window-dedicated` |  | window | `C-x w d` |  |  |  | 10 |
| `window-toggle-side-windows` |  | window | `C-x w s` |  |  |  | 10 |
| `complete-symbol` |  | bindings | `C-M-i` | 実装済み | `C-M-i` | Lisp (`lisp/minibuffer.scm`) | 済 |
| `ignore-preserving-kill-region` |  | bindings | `<mouse-movement>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `left-char` |  | bindings | `<left>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `left-word` |  | bindings | `C-<left>`, `M-<left>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `right-char` |  | bindings | `<right>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `right-word` |  | bindings | `C-<right>`, `M-<right>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `backward-kill-sexp` | ◯ | lisp | `C-M-<delete>`, `ESC C-<delete>`, `C-M-<backspace>`, `ESC C-<backspace>` | 実装済み | `C-M-DEL` | Lisp (`lisp/simple.scm`) | 済 |
| `backward-list` | ◯ | lisp | `C-M-p` | 実装済み | `C-M-p` | Lisp (`lisp/simple.scm`) | 済 |
| `backward-sexp` | ◯ | lisp | `C-M-b`, `C-M-<left>`, `ESC C-<left>` | 実装済み | `C-M-b` | Lisp (`lisp/simple.scm`) | 済 |
| `backward-up-list` | ◯ | lisp | `C-M-u`, `C-M-<up>`, `ESC C-<up>` | 実装済み | `C-M-u` | Lisp (`lisp/simple.scm`) | 済 |
| `beginning-of-defun` | ◯ | lisp | `C-M-a`, `C-M-<home>`, `ESC C-<home>` | 実装済み | `C-M-a` | Lisp (`lisp/simple.scm`) | 済 |
| `down-list` | ◯ | lisp | `C-M-d`, `C-M-<down>`, `ESC C-<down>` | 実装済み | `C-M-d` | Lisp (`lisp/simple.scm`) | 済 |
| `end-of-defun` | ◯ | lisp | `C-M-e`, `C-M-<end>`, `ESC C-<end>` | 実装済み | `C-M-e` | Lisp (`lisp/simple.scm`) | 済 |
| `forward-list` | ◯ | lisp | `C-M-n` | 実装済み | `C-M-n` | Lisp (`lisp/simple.scm`) | 済 |
| `forward-sexp` | ◯ | lisp | `C-M-f`, `C-M-<right>`, `ESC C-<right>` | 実装済み | `C-M-f` | Lisp (`lisp/simple.scm`) | 済 |
| `insert-parentheses` | ◯ | lisp | `M-(` | 実装済み | `M-(` | Lisp (`lisp/simple.scm`) | 済 |
| `kill-sexp` | ◯ | lisp | `C-M-k` | 実装済み | `C-M-k` | Lisp (`lisp/simple.scm`) | 済 |
| `mark-defun` | ◯ | lisp | `C-M-h` | 実装済み | `C-M-h` | Lisp (`lisp/simple.scm`) | 済 |
| `mark-sexp` | ◯ | lisp | `C-M-@`, `C-M-SPC` | 実装済み | `C-M-SPC`, `C-M-@` | Lisp (`lisp/simple.scm`) | 済 |
| `move-past-close-and-reindent` | ◯ | lisp | `M-)` | 実装済み | `M-)` | Lisp (`lisp/simple.scm`) | 済 |
| `narrow-to-defun` |  | lisp | `C-x n d` | 実装済み | `C-x n d` | Lisp (`lisp/simple.scm`) | 済 |
| `indent-for-tab-command` | ◯ | indent | `TAB` | 実装済み | `TAB`, `C-i` | Lisp (`lisp/simple.scm`) | 済 |
| `indent-region` | ◯ | indent | `C-M-\` | 実装済み | `C-M-\\` | Lisp (`lisp/simple.scm`) | 済 |
| `indent-rigidly` | ◯ | indent | `C-x TAB` | 実装済み | `C-x TAB` | Lisp (`lisp/simple.scm`) | 済 |
| `tab-to-tab-stop` |  | indent | `M-i` | 実装済み | `M-i` | Lisp (`lisp/simple.scm`) | 済 |
| `backward-kill-sentence` | ◯ | paragraphs | `C-x DEL` | 実装済み | `C-x DEL` | Lisp (`lisp/simple.scm`) | 済 |
| `backward-paragraph` | ◯ | paragraphs | `M-{`, `C-<up>` | 実装済み | `M-{` | Lisp (`lisp/simple.scm`) | 済 |
| `backward-sentence` | ◯ | paragraphs | `M-a` | 実装済み | `M-a` | Lisp (`lisp/simple.scm`) | 済 |
| `forward-paragraph` | ◯ | paragraphs | `M-}`, `C-<down>` | 実装済み | `M-}` | Lisp (`lisp/simple.scm`) | 済 |
| `forward-sentence` | ◯ | paragraphs | `M-e` | 実装済み | `M-e` | Lisp (`lisp/simple.scm`) | 済 |
| `kill-sentence` | ◯ | paragraphs | `M-k` | 実装済み | `M-k` | Lisp (`lisp/simple.scm`) | 済 |
| `mark-paragraph` | ◯ | paragraphs | `M-h` | 実装済み | `M-h` | Lisp (`lisp/simple.scm`) | 済 |
| `backward-page` | ◯ | page | `C-x [` | 実装済み | `C-x [` | Lisp (`lisp/simple.scm`) | 済 |
| `count-lines-page` | ◯ | page | `C-x l` | 実装済み | `C-x l` | Lisp (`lisp/simple.scm`) | 済 |
| `forward-page` | ◯ | page | `C-x ]` | 実装済み | `C-x ]` | Lisp (`lisp/simple.scm`) | 済 |
| `mark-page` | ◯ | page | `C-x C-p` | 実装済み | `C-x C-p` | Lisp (`lisp/simple.scm`) | 済 |
| `narrow-to-page` | ◯ | page | `C-x n p` | 実装済み | `C-x n p` | Lisp (`lisp/simple.scm`) | 済 |
| `fill-paragraph` | ◯ | fill | `M-q` | 実装済み | `M-q` | Lisp (`lisp/simple.scm`) | 済 |
| `set-fill-prefix` | ◯ | fill | `C-x .` | 実装済み | `C-x .` | Lisp (`lisp/simple.scm`) | 済 |
| `comment-dwim` |  | newcomment | `M-;` | 実装済み | `M-;` | Lisp (`lisp/simple.scm`) | 済 |
| `comment-line` |  | newcomment | `C-x C-;` | 実装済み | `C-x C-;` | Lisp (`lisp/simple.scm`) | 済 |
| `comment-set-column` |  | newcomment | `C-x ;` | 実装済み | `C-x ;` | Lisp (`lisp/simple.scm`) | 済 |
| `occur` | ◯ | replace | `M-s o` | 実装済み | `M-s o` | Lisp (`lisp/replace.scm`) | 済 |
| `query-replace` | ◯ | replace | `M-%` | 実装済み | `M-%` | Lisp (`lisp/replace.scm`) | 済 |
| `query-replace-regexp` | ◯ | replace | `C-M-%` | 実装済み | `C-M-%` | Lisp (`lisp/replace.scm`) | 済 |
| `isearch-backward` | ◯ | isearch | `C-r` | 実装済み | `C-r` | Lisp (`lisp/isearch.scm`) | 済 |
| `isearch-backward-regexp` | ◯ | isearch | `C-M-r` | 実装済み | `C-M-r` | Lisp (`lisp/isearch.scm`) | 済 |
| `isearch-forward` | ◯ | isearch | `C-s` | 実装済み | `C-s` | Lisp (`lisp/isearch.scm`) | 済 |
| `isearch-forward-regexp` | ◯ | isearch | `C-M-s` | 実装済み | `C-M-s` | Lisp (`lisp/isearch.scm`) | 済 |
| `isearch-forward-symbol` |  | isearch | `M-s _` | 実装済み | `M-s _` | Lisp (`lisp/isearch.scm`) | 済 |
| `isearch-forward-symbol-at-point` |  | isearch | `M-s .` | 実装済み | `M-s .` | Lisp (`lisp/isearch.scm`) | 済 |
| `isearch-forward-thing-at-point` |  | isearch | `M-s M-.` | 実装済み | `M-s M-.` | Lisp (`lisp/isearch.scm`) | 済 |
| `isearch-forward-word` |  | isearch | `M-s w` | 実装済み | `M-s w` | Lisp (`lisp/isearch.scm`) | 済 |
| `clear-rectangle` | ◯ | rect | `C-x r c` | 実装済み | `C-x r c` | Lisp (`lisp/rect.scm`) | 済 |
| `copy-rectangle-as-kill` |  | rect | `C-x r M-w` | 実装済み | `C-x r M-w` | Lisp (`lisp/rect.scm`) | 済 |
| `delete-rectangle` | ◯ | rect | `C-x r d` | 実装済み | `C-x r d` | Lisp (`lisp/rect.scm`) | 済 |
| `kill-rectangle` | ◯ | rect | `C-x r k` | 実装済み | `C-x r k` | Lisp (`lisp/rect.scm`) | 済 |
| `open-rectangle` | ◯ | rect | `C-x r o` | 実装済み | `C-x r o` | Lisp (`lisp/rect.scm`) | 済 |
| `rectangle-number-lines` |  | rect | `C-x r N` | 実装済み | `C-x r N` | Lisp (`lisp/rect.scm`) | 済 |
| `string-rectangle` |  | rect | `C-x r t` | 実装済み | `C-x r t` | Lisp (`lisp/rect.scm`) | 済 |
| `yank-rectangle` | ◯ | rect | `C-x r y` | 実装済み | `C-x r y` | Lisp (`lisp/rect.scm`) | 済 |
| `copy-rectangle-to-register` |  | register | `C-x r r` | 実装済み | `C-x r r` | Lisp (`lisp/register.scm`) | 済 |
| `copy-to-register` | ◯ | register | `C-x r s`, `C-x r x` | 実装済み | `C-x r s`, `C-x r x` | Lisp (`lisp/register.scm`) | 済 |
| `increment-register` | ◯ | register | `C-x r +` | 実装済み | `C-x r +` | Lisp (`lisp/register.scm`) | 済 |
| `insert-register` | ◯ | register | `C-x r g`, `C-x r i` | 実装済み | `C-x r i`, `C-x r g` | Lisp (`lisp/register.scm`) | 済 |
| `jump-to-register` |  | register | `C-x r j` | 実装済み | `C-x r j` | Lisp (`lisp/register.scm`) | 済 |
| `number-to-register` | ◯ | register | `C-x r n` | 実装済み | `C-x r n` | Lisp (`lisp/register.scm`) | 済 |
| `point-to-register` | ◯ | register | `C-x r C-@`, `C-x r SPC`, `C-x r C-SPC` | 実装済み | `C-x r SPC`, `C-x r C-SPC`, `C-x r C-@` | Lisp (`lisp/register.scm`) | 済 |
| `window-configuration-to-register` |  | register | `C-x r w` | 実装済み | `C-x r w` | Lisp (`lisp/register.scm`) | 済 |
| `kmacro-end-and-call-macro` |  | kmacro | `C-x e` | 実装済み | `C-x C-k e`, `C-x e` | Lisp (`lisp/simple.scm`) | 済 |
| `kmacro-end-call-mouse` |  | kmacro | `S-<mouse-3>` |  |  |  | 8 |
| `kmacro-end-macro` |  | kmacro | `C-x )` | 実装済み | `C-x )` | Lisp (`lisp/simple.scm`) | 済 |
| `kmacro-end-or-call-macro` |  | kmacro | `<f4>` | 実装済み | `<f4>` | Lisp (`lisp/simple.scm`) | 済 |
| `kmacro-name-last-macro` |  | kmacro | `C-x C-k n` | 実装済み | `C-x C-k n` | Lisp (`lisp/simple.scm`) | 済 |
| `kmacro-start-macro` |  | kmacro | `C-x (`, `C-x C-k s`, `C-x C-k C-s` | 実装済み | `C-x (`, `C-x C-k s`, `C-x C-k C-s` | Lisp (`lisp/simple.scm`) | 済 |
| `kmacro-start-macro-or-insert-counter` |  | kmacro | `<f3>` | 実装済み | `<f3>` | Lisp (`lisp/simple.scm`) | 済 |
| `abbrev-prefix-mark` | ◯ | abbrev | `M-'` | 実装済み | `M-'` | Lisp (`lisp/abbrev.scm`) | 済 |
| `add-global-abbrev` | ◯ | abbrev | `C-x a g` | 実装済み | `C-x a g` | Lisp (`lisp/abbrev.scm`) | 済 |
| `add-mode-abbrev` | ◯ | abbrev | `C-x a +`, `C-x a l`, `C-x a C-a` | 実装済み | `C-x a +`, `C-x a l`, `C-x a C-a` | Lisp (`lisp/abbrev.scm`) | 済 |
| `expand-abbrev` | ◯ | abbrev | `C-x '`, `C-x a '`, `C-x a e` | 実装済み | `C-x '`, `C-x a '`, `C-x a e` | Lisp (`lisp/abbrev.scm`) | 済 |
| `inverse-add-global-abbrev` | ◯ | abbrev | `C-x a -`, `C-x a i g` | 実装済み | `C-x a -`, `C-x a i g` | Lisp (`lisp/abbrev.scm`) | 済 |
| `inverse-add-mode-abbrev` | ◯ | abbrev | `C-x a i l` | 実装済み | `C-x a i l` | Lisp (`lisp/abbrev.scm`) | 済 |
| `dabbrev-completion` |  | dabbrev | `C-M-/` | 実装済み | `C-M-/` | Lisp (`lisp/dabbrev.scm`) | 済 |
| `dabbrev-expand` | ◯ | dabbrev | `M-/` | 実装済み | `M-/` | Lisp (`lisp/dabbrev.scm`) | 済 |
| `list-buffers` | ◯ | buff-menu | `C-x C-b` | 実装済み | `C-x C-b` | Lisp (`lisp/buff-menu.scm`) | 済 |
| `dired` | ◯ | dired | `C-x d` | 実装済み | `C-x d` | Lisp (`lisp/dired.scm`) | 済 |
| `dired-jump` |  | dired | `C-x C-j` | 実装済み | `C-x C-j` | Lisp (`lisp/dired.scm`) | 済 |
| `dired-jump-other-window` |  | dired | `C-x 4 C-j` | 実装済み | `C-x 4 C-j` | Lisp (`lisp/dired.scm`) | 済 |
| `dired-other-frame` | ◯ | dired | `C-x 5 d` |  |  |  | 12 |
| `dired-other-tab` |  | dired | `C-x t d` |  |  |  | 12 |
| `dired-other-window` | ◯ | dired | `C-x 4 d` | 実装済み | `C-x 4 d` | Lisp (`lisp/dired.scm`) | 済 |
| `bookmark-bmenu-list` |  | bookmark | `C-x r l` | 実装済み | `C-x r l` | Lisp (`lisp/bookmark.scm`) | 済 |
| `bookmark-jump` |  | bookmark | `C-x r b` | 実装済み | `C-x r b` | Lisp (`lisp/bookmark.scm`) | 済 |
| `bookmark-set` |  | bookmark | `C-x r m` | 実装済み | `C-x r m` | Lisp (`lisp/bookmark.scm`) | 済 |
| `bookmark-set-no-overwrite` |  | bookmark | `C-x r M` | 実装済み | `C-x r M` | Lisp (`lisp/bookmark.scm`) | 済 |
| `xref-find-apropos` |  | xref | `C-M-.` | 実装済み | `C-M-.` | Lisp (`lisp/xref.scm`) | 済 |
| `xref-find-definitions` |  | xref | `M-.` | 実装済み | `M-.` | Lisp (`lisp/xref.scm`) | 済 |
| `xref-find-definitions-other-frame` |  | xref | `C-x 5 .` |  |  |  | 12 |
| `xref-find-definitions-other-window` |  | xref | `C-x 4 .` | 実装済み | `C-x 4 .` | Lisp (`lisp/xref.scm`) | 済 |
| `xref-find-references` |  | xref | `M-?` | 実装済み | `M-?` | Lisp (`lisp/xref.scm`) | 済 |
| `xref-go-back` |  | xref | `M-,` | 実装済み | `M-,` | Lisp (`lisp/xref.scm`) | 済 |
| `xref-go-forward` |  | xref | `C-M-,` | 実装済み | `C-M-,` | Lisp (`lisp/xref.scm`) | 済 |
| `highlight-lines-matching-regexp` |  | loaddefs | `M-s h l` | 実装済み | `M-s h l` | Lisp (`lisp/replace.scm`) | 済 |
| `highlight-phrase` |  | loaddefs | `M-s h p` | 実装済み | `M-s h p` | Lisp (`lisp/replace.scm`) | 済 |
| `highlight-regexp` |  | loaddefs | `M-s h r` | 実装済み | `M-s h r` | Lisp (`lisp/replace.scm`) | 済 |
| `highlight-symbol-at-point` |  | loaddefs | `M-s h .` | 実装済み | `M-s h .` | Lisp (`lisp/replace.scm`) | 済 |
| `unhighlight-regexp` |  | loaddefs | `M-s h u` | 実装済み | `M-s h u` | Lisp (`lisp/replace.scm`) | 済 |
| `describe-bindings` | ◯ | help | `C-h b`, `<f1> b`, `<help> b` | 実装済み | `C-h b` | Lisp (`lisp/help.scm`) | 済 |
| `describe-copying` |  | help | `C-h C-c`, `<f1> C-c`, `<help> C-c` |  |  |  | 12 |
| `describe-distribution` |  | help | `C-h C-o`, `<f1> C-o`, `<help> C-o` |  |  |  | 12 |
| `describe-gnu-project` |  | help | `C-h g`, `<f1> g`, `<help> g` |  |  |  | 12 |
| `describe-key` | ◯ | help | `C-h k`, `<f1> k`, `<help> k` | 実装済み | `C-h k` | Lisp (`lisp/help.scm`) | 済 |
| `describe-key-briefly` | ◯ | help | `C-h c`, `<f1> c`, `<help> c` | 実装済み | `C-h c` | Lisp (`lisp/help.scm`) | 済 |
| `describe-no-warranty` |  | help | `C-h C-w`, `<f1> C-w`, `<help> C-w` |  |  |  | 12 |
| `help-for-help` |  | help | `C-h ?`, `<f1> ?`, `C-h C-h`, `<f1> C-h` ほか 8 | 実装済み | `C-h ?`, `C-h C-h` | Lisp (`lisp/help.scm`) | 済 |
| `help-quick-toggle` |  | help | `C-h C-q`, `<f1> C-q`, `<help> C-q` | 実装済み | `C-h C-q` | Lisp (`lisp/help.scm`) | 済 |
| `help-quit` |  | help | `C-h q`, `<f1> q`, `<help> q` | 実装済み | `C-h q` | Lisp (`lisp/help.scm`) | 済 |
| `search-forward-help-for-help` |  | help | `C-h C-s`, `<f1> C-s`, `<help> C-s` | 実装済み | `C-h C-s` | Lisp (`lisp/help.scm`) | 済 |
| `view-echo-area-messages` |  | help | `C-h e`, `<f1> e`, `<help> e` | 実装済み | `C-h e` | Lisp (`lisp/help.scm`) | 済 |
| `view-emacs-FAQ` |  | help | `C-h C-f`, `<f1> C-f`, `<help> C-f` |  |  |  | 12 |
| `view-emacs-debugging` |  | help | `C-h C-d`, `<f1> C-d`, `<help> C-d` |  |  |  | 12 |
| `view-emacs-news` |  | help | `C-h n`, `<f1> n`, `C-h C-n`, `<f1> C-n` ほか 2 |  |  |  | 12 |
| `view-emacs-problems` |  | help | `C-h C-p`, `<f1> C-p`, `<help> C-p` |  |  |  | 12 |
| `view-emacs-todo` |  | help | `C-h C-t`, `<f1> C-t`, `<help> C-t` |  |  |  | 12 |
| `view-external-packages` |  | help | `C-h C-e`, `<f1> C-e`, `<help> C-e` |  |  |  | 12 |
| `view-lossage` | ◯ | help | `C-h l`, `<f1> l`, `<help> l` | 実装済み | `C-h l` | Lisp (`lisp/help.scm`) | 済 |
| `view-order-manuals` |  | help | `C-h RET`, `<f1> RET`, `<help> RET` |  |  |  | 12 |
| `where-is` | ◯ | help | `C-h w`, `<f1> w`, `<help> w` | 実装済み | `C-h w` | Lisp (`lisp/help.scm`) | 済 |
| `describe-command` |  | help-fns | `C-h x`, `<f1> x`, `<help> x` | 実装済み | `C-h x` | Lisp (`lisp/help.scm`) | 済 |
| `describe-function` | ◯ | help-fns | `C-h f`, `<f1> f`, `<help> f` | 実装済み | `C-h f`, `C-h f` | Lisp (`lisp/help-fns.scm`) | 済 |
| `describe-symbol` |  | help-fns | `C-h o`, `<f1> o`, `<help> o` | 実装済み | `C-h o` | Lisp (`lisp/help.scm`) | 済 |
| `describe-syntax` | ◯ | help-fns | `C-h s`, `<f1> s`, `<help> s` | 実装済み | `C-h s` | Lisp (`lisp/help.scm`) | 済 |
| `describe-variable` | ◯ | help-fns | `C-h v`, `<f1> v`, `<help> v` | 実装済み | `C-h v` | Lisp (`lisp/help.scm`) | 済 |
| `help-find-source` |  | help-fns | `C-h 4 s`, `<f1> 4 s`, `<help> 4 s` | 実装済み | `C-h 4 s` | Lisp (`lisp/xref.scm`) | 済 |
| `Info-goto-emacs-command-node` |  | info | `C-h F`, `<f1> F`, `<help> F` | 実装済み | `C-h F` | Lisp (`lisp/info.scm`) | 済 |
| `Info-goto-emacs-key-command-node` |  | info | `C-h K`, `<f1> K`, `<help> K` | 実装済み | `C-h K` | Lisp (`lisp/info.scm`) | 済 |
| `info` | ◯ | info | `C-h i`, `<f1> i`, `<help> i` | 実装済み | `C-h i` | Lisp (`lisp/info.scm`) | 済 |
| `info-display-manual` |  | info | `C-h R`, `<f1> R`, `<help> R` | 実装済み | `C-h R` | Lisp (`lisp/info.scm`) | 済 |
| `info-emacs-manual` |  | info | `C-h r`, `<f1> r`, `<help> r` | 実装済み | `C-h r` | Lisp (`lisp/info.scm`) | 済 |
| `info-other-window` |  | info | `C-h 4 i`, `<f1> 4 i`, `<help> 4 i` | 実装済み | `C-h 4 i` | Lisp (`lisp/info.scm`) | 済 |

<!-- emacs-commands:end -->
