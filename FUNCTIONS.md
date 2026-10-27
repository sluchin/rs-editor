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
1. 比較演算子と制御フロー（`if`, `cond`）の実装
2. 変数バインディングと定義（`define`, `let`）
3. より高度なリスト操作
4. ラムダ式とクロージャのサポート

<!-- emacs-commands:begin -->

## Emacs の組み込みのコマンドと, エディタの基本のコマンド

Emacs 30.2 の組み込みのコマンド (C で書いたもの) と, エディタの基本のコマンド (Lisp で書いたもの) と, その既定のキー。`tools/emacs-commands/run.sh` で作った一覧 (手で直さない。作り直すときは, `run.sh` を動かす)。

- **既定のキー**: `emacs -Q` の global-map のキー。global-map に無ければ, ほかのキーマップ (ミニバッファなど) のキーを, キーマップの名前と書く。
- **rs-editor**: rs-editor にあるコマンドは「実装済み」と, rs-editor のキーを書く。
- **実装**: rs-editor で書いた言語とファイル。今は, ほとんどを TypeScript (画面) で書いている。いずれ Lisp (`lisp/`) に移す (移したら「Lisp」になる)。Emacs の「C で書いたもの」「Lisp で書いたもの」は, Emacs の中での区別で, rs-editor での書き方とは別。
- **Edwin**: MIT/GNU Scheme 12.1 の Edwin (Emacs に似たエディタ. `src/edwin` の `define-command`) にも, 同じ名前のコマンドがあれば ◯。名前の一覧は `tools/emacs-commands/edwin-commands.txt` (`tools/emacs-commands/edwin.sh` で作る)。
- **計画**: 「エディタの基本のコマンド」の表だけにある。Lisp で書く段階の番号 (下の「実装の計画」の段階)。「済」は, 既に Lisp で書いたもの。計画は `tools/emacs-commands/plan.py` に書く。
- キーは Emacs の書き方 (`key-description`)。`C-c C-m` は `C-c RET`, `C-i` は `TAB` と書く。
- 「エディタの基本のコマンド」は, Lisp で書いたコマンド (`find-file` など) のうち, 既定の global-map にキーがあり, エディタの基本のライブラリ (`simple`, `files`, `window`, `bindings`, `lisp`, `indent`, `paragraphs`, `page`, `fill`, `newcomment`, `replace`, `isearch`, `rect`, `register`, `kmacro`, `abbrev`, `dabbrev`, `buff-menu`, `dired`, `bookmark`, `xref`, `loaddefs`, `help`, `help-fns`, `info`) にあるもの。マイナーモードのコマンド (`-mode`) と, 版管理 (`vc-`) は, 入れていない。

### 組み込みのコマンド (101 個. 実装済み 15 個: Lisp 0 個, TypeScript 15 個. Edwin にもある: 41 個)

| コマンド | Edwin | 既定のキー | rs-editor | rs-editor のキー | 実装 |
|---|---|---|---|---|---|
| `abort-minibuffers` |  | minibuffer-local-completion-map: `C-g`<br>minibuffer-local-isearch-map: `C-g`<br>minibuffer-local-map: `C-g` | 実装済み |  | TypeScript (`src/App.tsx`) |
| `abort-recursive-edit` | ◯ | `C-]` | 実装済み | `C-]` | TypeScript (`src/App.tsx`) |
| `accelerate-menu` |  |  |  |  |  |
| `add-name-to-file` |  |  |  |  |  |
| `backward-char` | ◯ | `C-b` | 実装済み | `C-b` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) |
| `base64-decode-region` |  |  |  |  |  |
| `base64-encode-region` |  |  |  |  |  |
| `base64url-encode-region` |  |  |  |  |  |
| `beginning-of-line` | ◯ |  |  |  |  |
| `buffer-enable-undo` |  |  |  |  |  |
| `call-last-kbd-macro` | ◯ |  | 実装済み | `C-x e` | TypeScript (`src/lib/emacs.ts`) |
| `capitalize-region` | ◯ |  |  |  |  |
| `capitalize-word` | ◯ | `M-c` | 実装済み | `M-c` | TypeScript (`src/lib/emacs.ts`) |
| `chmod` |  |  |  |  |  |
| `copy-file` | ◯ |  |  |  |  |
| `decode-coding-region` |  |  |  |  |  |
| `defining-kbd-macro` |  |  |  |  |  |
| `delete-char` | ◯ | `C-d` | 実装済み | `C-d` | TypeScript (`src/lib/emacs.ts`) |
| `delete-frame` | ◯ | `C-x 5 0` |  |  |  |
| `delete-other-windows-internal` |  |  |  |  |  |
| `delete-process` |  |  |  |  |  |
| `delete-region` | ◯ |  |  |  |  |
| `do-auto-save` | ◯ |  |  |  |  |
| `downcase-region` | ◯ | `C-x C-l` | 実装済み | `C-x C-l` | TypeScript (`src/lib/emacs.ts`) |
| `downcase-word` | ◯ | `M-l` | 実装済み | `M-l` | TypeScript (`src/lib/emacs.ts`) |
| `encode-coding-region` |  |  |  |  |  |
| `end-kbd-macro` | ◯ |  | 実装済み | `C-x )` | TypeScript (`src/lib/emacs.ts`) |
| `end-of-line` | ◯ |  |  |  |  |
| `erase-buffer` |  |  |  |  |  |
| `eval-buffer` |  |  |  |  |  |
| `eval-region` | ◯ |  |  |  |  |
| `exit-recursive-edit` | ◯ | `C-M-c` |  |  |  |
| `forward-char` | ◯ | `C-f` | 実装済み | `C-f` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) |
| `forward-line` |  |  |  |  |  |
| `forward-word` | ◯ | `M-f`, `ESC <right>` | 実装済み | `M-f` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) |
| `garbage-collect` |  |  |  |  |  |
| `goto-char` | ◯ | `M-g c` |  |  |  |
| `handle-save-session` |  | special-event-map: `<save-session>` |  |  |  |
| `handle-switch-frame` |  | `<switch-frame>` |  |  |  |
| `iconify-frame` |  |  |  |  |  |
| `indent-to` |  |  |  |  |  |
| `indent-to-column` |  |  |  |  |  |
| `insert-char` |  | `C-x 8 RET` |  |  |  |
| `kill-buffer` | ◯ | `C-x k` | 実装済み | `C-x k` | TypeScript (`src/App.tsx`, `lisp/simple.scm`) |
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
| `narrow-to-region` | ◯ | `C-x n n` |  |  |  |
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
| `rename-buffer` | ◯ | `C-x x r` |  |  |  |
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
| `start-kbd-macro` | ◯ |  | 実装済み | `C-x (` | TypeScript (`src/lib/emacs.ts`) |
| `suspend-emacs` |  |  |  |  |  |
| `top-level` |  |  |  |  |  |
| `transpose-regions` |  |  |  |  |  |
| `unix-sync` |  |  |  |  |  |
| `upcase-initials-region` |  |  |  |  |  |
| `upcase-region` | ◯ | `C-x C-u` | 実装済み | `C-x C-u` | TypeScript (`src/lib/emacs.ts`) |
| `upcase-word` | ◯ | `M-u` | 実装済み | `M-u` | TypeScript (`src/lib/emacs.ts`) |
| `widen` | ◯ | `C-x n w` |  |  |  |
| `write-region` | ◯ |  |  |  |  |
| `x-menu-bar-open-internal` |  |  |  |  |  |
| `x-print-frames-dialog` |  |  |  |  |  |

### 実装の計画 (エディタの基本のコマンドを Lisp で書く)

エディタの基本のコマンドを, Lisp (`lisp/`) で書く計画。TODO.md の Phase 16 の続き。目標は, 今の画面 (Tauri) と, ターミナル (ncurses) の両方で動かすこと(そのため, 段階 1 で, エディタの核を Rust に作る)。下の表の「計画」の列の番号が, ここの段階の番号。「済」は, 既に Lisp で書いたもの。段階は番号の順に進める (1 は土台で, 2 以降の前提)。TypeScript で書いてあるコマンドは, Lisp に移したら TypeScript の方を消す。

| 段階 | 内容 | コマンド | 新しく書く | TypeScript から移す |
|---|---|---|---|---|
| 1 | 土台: Rust のエディタの核 (ターミナルの ncurses でも動かすため) | - | - | - |
| 2 | 動き | 10 | - | 10 |
| 3 | 消す, 切る, 貼る, 元に戻す | 18 | - | 18 |
| 4 | マーク, 前置引数, 実行 | 21 | 9 | 12 |
| 5 | S 式, 段落, ページ | 28 | 14 | 14 |
| 6 | 字下げ, 整形, コメント | 9 | 5 | 4 |
| 7 | 検索, 置換, 強調 | 16 | 10 | 6 |
| 8 | 矩形, レジスタ, キーボードマクロ, 略語 | 32 | 26 | 6 |
| 9 | ファイル, バッファ, dired, ブックマーク, シェル | 22 | 17 | 5 |
| 10 | ウィンドウ | 33 | 25 | 8 |
| 11 | ヘルプ, xref, info | 27 | 25 | 2 |
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
- 【Rust は済 (`core/src/scheme/editor_host.rs`: `Editor` を直接動かす `EditorHost`. 63 個の組み込み). 今の画面 (Tauri) をつなぐのは 1.11】1.9 評価器を核につなぐ: 今の `Host` (画面に頼んで返事を待つ) をやめ, 評価器が核を直接動かす。画面に頼むのは, 描くこと, ミニバッファの入力, ファイルの選択だけにする (`Host` を小さくする)。
- 【済 (`tui/`. 動いている: 移動, 編集, kill, yank, undo, 窓の分割, ファイルを開く, 保存, M-x, 終了。基本のコマンドは `lisp/simple.scm`)】1.10 ターミナルのアプリの骨組み (`tui/`. ncurses): 核の状態を描き, キーを核に渡す。カーソルを動かし, 文字を入れ, 保存できるところまで。
- 【段階 2 から 11 の後】1.11 今の画面 (Tauri) を核につなぎ替える: 本文の持ち主を CodeMirror から核に移す。CodeMirror は表示と入力にする。今の画面のコマンド (`src/lib/emacs.ts` など. 約 85 個) は, CodeMirror の状態を直接書き換えているので, 段階 2 から 11 で Lisp に移し終えるまでは, 持ち主を移せない。移す前の動きは, `tests/frontend/` のテストで保つ。
- 【つないだ (`core/src/scheme/mirror.rs`. 環境変数 `RSEDITOR_NO_CORE` で、これまでの動きに戻せる)】1.11 の前につなぐ案 (Tauri の画面を, 先に核の組み込みで動かす): 評価のあいだだけ, 画面のバッファを核に写し (スナップショット), 終わったら変更を画面に返す (`EditorHost` を `Bridge` の上で動かし, 本文の操作は核で, バッファの出し入れとウィンドウは画面に頼む). 探す, 構文, マークなどの組み込みが, 今の画面の Lisp (howm, org) でも使える。画面の往復が減る代わりに, 画面と核の本文の同期が要る。実際の画面での確かめが要るので, 段階 2 以降の Lisp が画面で要るようになったときに行う。

**2. 動き**

- 【Lisp 版は済 (`lisp/simple.scm`). `emacs -Q --batch` の結果と, 本文の全ての位置で一致 (`core/tests/motion_test.rs`. 392 通り). 画面 (Tauri) の TypeScript 版を消すのは, 画面をつないだ後 (段階 1.11 の前の案)】行, 単語, バッファの端, 左右の移動。核の上の Lisp で書く。カーソルを動かすので, 速さを比べてから Lisp に移す。

**3. 消す, 切る, 貼る, 元に戻す**

- 【Lisp 版は済 (`lisp/simple.scm`). `emacs -Q --batch` の結果と一致 (`core/tests/edit_test.rs`: 57 通りのコマンドを 7 種類の本文の全ての位置から, `core/tests/undo_test.rs`: undo と undo-redo の 10 通りの場面). 画面 (Tauri) の TypeScript 版を消すのは, 画面をつないだ後】違いが残るのは, `default-indent-new-line` (字下げは段階 6), `quoted-insert` (1 文字を表すキーだけ), `cycle-spacing` (最初の呼び出しだけを Emacs と比べた), `undo` が保存した状態に戻っても「変更なし」にならないこと。kill-ring を使うコマンドと, 空白と改行を整えるコマンド, 入れ替え, `undo`。

**4. マーク, 前置引数, 実行**

- マークとリージョン, 前置引数 (`C-u`, `M-数字`), `M-x`, `M-:`, `C-g`, エラーの移動, 表示の切り替え。

**5. S 式, 段落, ページ**

- 括弧の移動と消去 (`C-M-f` など), defun, 文と段落, ページ。構文表 (段階 1.6) を使う。`src/lib/sexp.ts` と `src/lib/paragraph.ts` の計算は, ターミナルでも使うため, Rust に移す。

**6. 字下げ, 整形, コメント**

- `TAB`, `C-M-\`, `M-q`, `M-;`。字下げの決まり (`src/lib/lispIndent.ts`) と整形 (`src/lib/fill.ts`) は, ターミナルでも使うため, Rust に移す。エディタと整形の道具 (TODO の Phase 18) で, 決まりを 1 つにする。

**7. 検索, 置換, 強調**

- isearch, `query-replace`, `occur`, `highlight-regexp`。検索の状態を核に持つ (描き方は, 画面ごと)。

**8. 矩形, レジスタ, キーボードマクロ, 略語**

- 矩形 (`C-x r`), レジスタ, キーボードマクロ (`F3`, `F4`, `C-x (`), 略語と `M-/`。

**9. ファイル, バッファ, dired, ブックマーク, シェル**

- ファイルとバッファの残りのコマンド, `list-buffers`, dired, ブックマーク, `M-!`, `M-|`, `M-&`, バッファの複製。
- `save-buffers-kill-terminal` は, アプリを終える頼みを `Host` に足してから移す。

**10. ウィンドウ**

- 窓の分割, 削除, 移動, 大きさ, スクロール, 別の窓に出すコマンド。窓の木 (`src/lib/windows.ts`) は, 段階 1.7 で核に移したものを使う。

**11. ヘルプ, xref, info**

- `C-h` のコマンドと, 定義へ移るコマンド (`M-.`), info。ヘルプは Phase 17 (docstring と多言語化) と一緒に進める。

**12. 入れない**

- rs-editor に当たる機能が無いので, 入れない。メール, フレーム (複数の窓枠), 日本語入力以外のテキスト変換, Emacs の案内の文書。

### エディタの基本のコマンド (Lisp で書いたもの. 262 個. 実装済み 108 個: Lisp 23 個, TypeScript 85 個. Edwin にもある: 135 個)

| コマンド | Edwin | ライブラリ | 既定のキー | rs-editor | rs-editor のキー | 実装 | 計画 |
|---|---|---|---|---|---|---|---|
| `analyze-text-conversion` |  | simple | `<text-conversion>` |  |  |  | 12 |
| `append-next-kill` | ◯ | simple | `C-M-w` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `async-shell-command` |  | simple | `M-&` |  |  |  | 9 |
| `back-to-indentation` | ◯ | simple | `M-m` | 実装済み | `M-m` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 2 |
| `backward-kill-word` | ◯ | simple | `M-DEL`, `C-<backspace>` | 実装済み | `M-DEL` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `backward-word` | ◯ | simple | `M-b`, `ESC <left>` | 実装済み | `M-b` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 2 |
| `beginning-of-buffer` | ◯ | simple | `M-<`, `<begin>`, `C-<home>` | 実装済み | `M-<` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 2 |
| `clone-buffer` |  | simple | `C-x x n` |  |  |  | 9 |
| `clone-indirect-buffer-other-window` |  | simple | `C-x 4 c` |  |  |  | 9 |
| `compose-mail` |  | simple | `C-x m` |  |  |  | 12 |
| `compose-mail-other-frame` |  | simple | `C-x 5 m` |  |  |  | 12 |
| `compose-mail-other-window` |  | simple | `C-x 4 m` |  |  |  | 12 |
| `count-words-region` |  | simple | `M-=` | 実装済み | `M-=` | TypeScript (`src/lib/emacs.ts`) | 4 |
| `cycle-spacing` |  | simple | `M-SPC` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `default-indent-new-line` |  | simple | `M-j`, `C-M-j` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `delete-backward-char` | ◯ | simple | `DEL` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `delete-blank-lines` | ◯ | simple | `C-x C-o` | 実装済み | `C-x C-o` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `delete-forward-char` |  | simple | `<deletechar>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `delete-horizontal-space` | ◯ | simple | `M-\` | 実装済み | `M-\` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `delete-indentation` | ◯ | simple | `M-^` | 実装済み | `M-^` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `digit-argument` | ◯ | simple | `C-0`, `C-1`, `C-2`, `C-3` ほか 16 | 実装済み | `M-0`, `M-1`, `M-2` ほか 7 | TypeScript (`src/lib/emacs.ts`) | 4 |
| `end-of-buffer` | ◯ | simple | `M->`, `C-<end>` | 実装済み | `M->` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 2 |
| `eval-expression` | ◯ | simple | `M-:`, `ESC M-:` | 実装済み | `M-:` | TypeScript (`src/hooks/useLisp.ts`, `lisp/simple.scm`) | 4 |
| `exchange-point-and-mark` | ◯ | simple | `C-x C-x` | 実装済み | `C-x C-x` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 4 |
| `execute-extended-command` | ◯ | simple | `M-x`, `<menu>`, `<execute>` | 実装済み | `M-x` | TypeScript (`src/App.tsx`, `lisp/simple.scm`) | 4 |
| `execute-extended-command-for-buffer` |  | simple | `M-X` |  |  |  | 4 |
| `goto-line` | ◯ | simple | `M-g g`, `M-g M-g` | 実装済み | `M-g g`, `M-g M-g` | TypeScript (`src/App.tsx`, `lisp/simple.scm`) | 2 |
| `goto-line-relative` |  | simple | `C-x n g` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `insert-buffer` | ◯ | simple | `C-x x i` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `keyboard-escape-quit` |  | simple | `ESC ESC ESC` |  |  |  | 4 |
| `keyboard-quit` | ◯ | simple | `C-g` | 実装済み | `C-g` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 4 |
| `kill-line` | ◯ | simple | `C-k`, `<deleteline>` | 実装済み | `C-k` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `kill-region` | ◯ | simple | `C-w`, `<cut>`, `S-<delete>` | 実装済み | `C-w` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `kill-ring-save` |  | simple | `M-w`, `<copy>`, `C-<insert>`, `C-<insertchar>` | 実装済み | `M-w` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `kill-whole-line` |  | simple | `C-S-<backspace>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `kill-word` | ◯ | simple | `M-d`, `C-<delete>` | 実装済み | `M-d` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `mark-whole-buffer` | ◯ | simple | `C-x h` | 実装済み | `C-x h` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 4 |
| `mark-word` | ◯ | simple | `M-@` | 実装済み | `M-@` | TypeScript (`src/lib/emacs.ts`) | 4 |
| `move-beginning-of-line` |  | simple | `C-a`, `<home>` | 実装済み | `C-a` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 2 |
| `move-end-of-line` |  | simple | `C-e`, `<end>` | 実装済み | `C-e` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 2 |
| `negative-argument` | ◯ | simple | `C--`, `M--`, `C-M--` |  |  |  | 4 |
| `newline` | ◯ | simple | `RET` | 実装済み | `RET`, `C-m` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `next-error` |  | simple | `` C-x ` ``, `M-g n`, `M-g M-n` | 実装済み | `M-g n`, `M-g M-n`, `` C-x ` `` | TypeScript (`src/hooks/useDirectories.ts`) | 4 |
| `next-line` | ◯ | simple | `C-n`, `<down>` | 実装済み | `C-n` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 2 |
| `open-line` | ◯ | simple | `C-o`, `<insertline>` | 実装済み | `C-o` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `pop-global-mark` |  | simple | `C-x C-@`, `C-x C-SPC` |  |  |  | 4 |
| `previous-error` |  | simple | `M-g p`, `M-g M-p` | 実装済み | `M-g p`, `M-g M-p` | TypeScript (`src/hooks/useDirectories.ts`) | 4 |
| `previous-line` | ◯ | simple | `C-p`, `<up>` | 実装済み | `C-p` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 2 |
| `quoted-insert` | ◯ | simple | `C-q` | 実装済み | `C-q` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `repeat-complex-command` | ◯ | simple | `<redo>`, `<again>`, `C-x M-:`, `C-x ESC ESC` |  |  |  | 4 |
| `set-fill-column` | ◯ | simple | `C-x f` |  |  |  | 4 |
| `set-goal-column` | ◯ | simple | `C-x C-n` | 実装済み | `C-x C-n` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 2 |
| `set-mark-command` | ◯ | simple | `C-@`, `C-SPC` | 実装済み | `C-SPC` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 4 |
| `set-selective-display` |  | simple | `C-x $` |  |  |  | 4 |
| `shell-command` | ◯ | simple | `M-!` |  |  |  | 9 |
| `shell-command-on-region` | ◯ | simple | `M-\|` |  |  |  | 9 |
| `split-line` | ◯ | simple | `C-M-o` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `toggle-truncate-lines` |  | simple | `C-x x t` |  |  |  | 4 |
| `transpose-chars` | ◯ | simple | `C-t` | 実装済み | `C-t` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `transpose-lines` | ◯ | simple | `C-x C-t` | 実装済み | `C-x C-t` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `transpose-sexps` | ◯ | simple | `C-M-t` |  |  |  | 5 |
| `transpose-words` | ◯ | simple | `M-t` | 実装済み | `M-t` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `undo` | ◯ | simple | `C-/`, `C-_`, `C-x u`, `<undo>` | 実装済み | `C-/`, `C-_`, `C-x u` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `undo-redo` |  | simple | `C-?`, `C-M-_` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `universal-argument` | ◯ | simple | `C-u` | 実装済み | `C-u` | TypeScript (`src/lib/emacs.ts`) | 4 |
| `what-cursor-position` | ◯ | simple | `C-x =` |  |  |  | 4 |
| `yank` | ◯ | simple | `C-y`, `<paste>`, `S-<insert>`, `S-<insertchar>` | 実装済み | `C-y` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `yank-pop` | ◯ | simple | `M-y` | 実装済み | `M-y` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `zap-to-char` |  | simple | `M-z` | 実装済み | `M-z` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 3 |
| `find-alternate-file` | ◯ | files | `C-x C-v` | 実装済み | `C-x C-v` | Lisp (`lisp/files.scm`) | 済 |
| `find-file` | ◯ | files | `<open>`, `C-x C-f` | 実装済み | `C-x C-f` | Lisp (`lisp/files.scm`) | 済 |
| `find-file-other-frame` | ◯ | files | `C-x 5 f`, `C-x 5 C-f` |  |  |  | 12 |
| `find-file-other-window` | ◯ | files | `C-x 4 f`, `C-x 4 C-f` |  |  |  | 9 |
| `find-file-read-only` |  | files | `C-x C-r` |  |  |  | 9 |
| `find-file-read-only-other-frame` |  | files | `C-x 5 r` |  |  |  | 12 |
| `find-file-read-only-other-window` |  | files | `C-x 4 r` |  |  |  | 9 |
| `insert-file` | ◯ | files | `C-x i` | 実装済み | `C-x i` | Lisp (`lisp/files.scm`) | 済 |
| `list-directory` | ◯ | files | `C-x C-d` |  |  |  | 9 |
| `not-modified` | ◯ | files | `M-~` |  |  |  | 9 |
| `rename-uniquely` |  | files | `C-x x u` |  |  |  | 9 |
| `revert-buffer-quick` |  | files | `C-x x g` |  |  |  | 9 |
| `save-buffer` | ◯ | files | `C-x C-s` | 実装済み | `C-x C-s` | Lisp (`lisp/files.scm`) | 済 |
| `save-buffers-kill-terminal` |  | files | `C-x C-c` | 実装済み | `C-x C-c` | TypeScript (`src/App.tsx`, `lisp/simple.scm`) | 9 |
| `save-some-buffers` | ◯ | files | `C-x s` | 実装済み | `C-x s` | Lisp (`lisp/files.scm`) | 済 |
| `write-file` | ◯ | files | `C-x C-w` | 実装済み | `C-x C-w` | Lisp (`lisp/files.scm`) | 済 |
| `balance-windows` |  | window | `C-x +` |  |  |  | 10 |
| `beginning-of-buffer-other-window` |  | window | `M-<home>`, `M-<begin>`, `ESC <home>`, `ESC <begin>` |  |  |  | 10 |
| `delete-other-windows` | ◯ | window | `C-x 1` | 実装済み | `C-x 1` | TypeScript (`src/hooks/useBuffers.ts`, `lisp/simple.scm`) | 10 |
| `delete-window` | ◯ | window | `C-x 0` | 実装済み | `C-x 0` | TypeScript (`src/hooks/useBuffers.ts`, `lisp/simple.scm`) | 10 |
| `delete-windows-on` |  | window | `C-x w 0` |  |  |  | 10 |
| `display-buffer` |  | window | `C-x 4 C-o` |  |  |  | 10 |
| `display-buffer-other-frame` |  | window | `C-x 5 C-o` |  |  |  | 12 |
| `end-of-buffer-other-window` |  | window | `M-<end>`, `ESC <end>` |  |  |  | 10 |
| `enlarge-window` | ◯ | window | `C-x ^` |  |  |  | 10 |
| `enlarge-window-horizontally` | ◯ | window | `C-x }` |  |  |  | 10 |
| `fit-window-to-buffer` |  | window | `C-x w -` |  |  |  | 10 |
| `handle-select-window` |  | window | `<select-window>` |  |  |  | 12 |
| `kill-buffer-and-window` | ◯ | window | `C-x 4 0` |  |  |  | 10 |
| `move-to-window-line-top-bottom` |  | window | `M-r` |  |  |  | 10 |
| `next-buffer` |  | window | `C-x <right>`, `<XF86Forward>`, `C-x C-<right>` |  |  |  | 10 |
| `other-window` | ◯ | window | `C-x o` | 実装済み | `C-x o` | TypeScript (`src/hooks/useBuffers.ts`, `lisp/simple.scm`) | 10 |
| `other-window-prefix` |  | window | `C-x 4 4` |  |  |  | 10 |
| `previous-buffer` |  | window | `<XF86Back>`, `C-x <left>`, `C-x C-<left>` |  |  |  | 10 |
| `quit-window` |  | window | `C-x w q` |  |  |  | 10 |
| `recenter-other-window` |  | window | `C-M-S-l` |  |  |  | 10 |
| `recenter-top-bottom` |  | window | `C-l` | 実装済み | `C-l` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 10 |
| `same-window-prefix` |  | window | `C-x 4 1` |  |  |  | 10 |
| `scroll-down-command` |  | window | `M-v`, `<prior>` | 実装済み | `M-v` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 10 |
| `scroll-other-window` | ◯ | window | `C-M-v`, `M-<next>`, `ESC <next>` |  |  |  | 10 |
| `scroll-other-window-down` | ◯ | window | `C-M-S-v`, `M-<prior>`, `ESC <prior>` |  |  |  | 10 |
| `scroll-up-command` |  | window | `C-v`, `<next>` | 実装済み | `C-v` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 10 |
| `shrink-window-horizontally` | ◯ | window | `C-x {` |  |  |  | 10 |
| `shrink-window-if-larger-than-buffer` | ◯ | window | `C-x -` |  |  |  | 10 |
| `split-root-window-below` |  | window | `C-x w 2` |  |  |  | 10 |
| `split-root-window-right` |  | window | `C-x w 3` |  |  |  | 10 |
| `split-window-below` |  | window | `C-x 2` | 実装済み | `C-x 2` | TypeScript (`src/hooks/useBuffers.ts`, `lisp/simple.scm`) | 10 |
| `split-window-right` |  | window | `C-x 3` | 実装済み | `C-x 3` | TypeScript (`src/hooks/useBuffers.ts`, `lisp/simple.scm`) | 10 |
| `switch-to-buffer` | ◯ | window | `C-x b` | 実装済み | `C-x b` | Lisp (`lisp/window.scm`) | 済 |
| `switch-to-buffer-other-frame` | ◯ | window | `C-x 5 b` |  |  |  | 12 |
| `switch-to-buffer-other-window` | ◯ | window | `C-x 4 b` |  |  |  | 10 |
| `toggle-window-dedicated` |  | window | `C-x w d` |  |  |  | 10 |
| `window-toggle-side-windows` |  | window | `C-x w s` |  |  |  | 10 |
| `complete-symbol` |  | bindings | `C-M-i` |  |  |  | 8 |
| `ignore-preserving-kill-region` |  | bindings | `<mouse-movement>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `left-char` |  | bindings | `<left>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `left-word` |  | bindings | `C-<left>`, `M-<left>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `right-char` |  | bindings | `<right>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `right-word` |  | bindings | `C-<right>`, `M-<right>` | 実装済み |  | Lisp (`lisp/simple.scm`) | 済 |
| `backward-kill-sexp` | ◯ | lisp | `C-M-<delete>`, `ESC C-<delete>`, `C-M-<backspace>`, `ESC C-<backspace>` |  |  |  | 5 |
| `backward-list` | ◯ | lisp | `C-M-p` |  |  |  | 5 |
| `backward-sexp` | ◯ | lisp | `C-M-b`, `C-M-<left>`, `ESC C-<left>` | 実装済み | `C-M-b` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `backward-up-list` | ◯ | lisp | `C-M-u`, `C-M-<up>`, `ESC C-<up>` | 実装済み | `C-M-u` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `beginning-of-defun` | ◯ | lisp | `C-M-a`, `C-M-<home>`, `ESC C-<home>` | 実装済み | `C-M-a` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `down-list` | ◯ | lisp | `C-M-d`, `C-M-<down>`, `ESC C-<down>` | 実装済み | `C-M-d` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `end-of-defun` | ◯ | lisp | `C-M-e`, `C-M-<end>`, `ESC C-<end>` | 実装済み | `C-M-e` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `forward-list` | ◯ | lisp | `C-M-n` |  |  |  | 5 |
| `forward-sexp` | ◯ | lisp | `C-M-f`, `C-M-<right>`, `ESC C-<right>` | 実装済み | `C-M-f` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `insert-parentheses` | ◯ | lisp | `M-(` |  |  |  | 5 |
| `kill-sexp` | ◯ | lisp | `C-M-k` | 実装済み | `C-M-k` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `mark-defun` | ◯ | lisp | `C-M-h` | 実装済み | `C-M-h` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `mark-sexp` | ◯ | lisp | `C-M-@`, `C-M-SPC` | 実装済み | `C-M-SPC` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `move-past-close-and-reindent` | ◯ | lisp | `M-)` |  |  |  | 5 |
| `narrow-to-defun` |  | lisp | `C-x n d` |  |  |  | 5 |
| `indent-for-tab-command` | ◯ | indent | `TAB` | 実装済み | `TAB`, `C-i` | TypeScript (`src/lib/emacs.ts`, `lisp/simple.scm`) | 6 |
| `indent-region` | ◯ | indent | `C-M-\` | 実装済み | `C-M-\` | TypeScript (`src/lib/emacs.ts`) | 6 |
| `indent-rigidly` | ◯ | indent | `C-x TAB` |  |  |  | 6 |
| `tab-to-tab-stop` |  | indent | `M-i` |  |  |  | 6 |
| `backward-kill-sentence` | ◯ | paragraphs | `C-x DEL` |  |  |  | 5 |
| `backward-paragraph` | ◯ | paragraphs | `M-{`, `C-<up>` | 実装済み | `M-{` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `backward-sentence` | ◯ | paragraphs | `M-a` | 実装済み | `M-a` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `forward-paragraph` | ◯ | paragraphs | `M-}`, `C-<down>` | 実装済み | `M-}` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `forward-sentence` | ◯ | paragraphs | `M-e` | 実装済み | `M-e` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `kill-sentence` | ◯ | paragraphs | `M-k` | 実装済み | `M-k` | TypeScript (`src/lib/emacs.ts`) | 5 |
| `mark-paragraph` | ◯ | paragraphs | `M-h` |  |  |  | 5 |
| `backward-page` | ◯ | page | `C-x [` |  |  |  | 5 |
| `count-lines-page` | ◯ | page | `C-x l` |  |  |  | 5 |
| `forward-page` | ◯ | page | `C-x ]` |  |  |  | 5 |
| `mark-page` | ◯ | page | `C-x C-p` |  |  |  | 5 |
| `narrow-to-page` | ◯ | page | `C-x n p` |  |  |  | 5 |
| `fill-paragraph` | ◯ | fill | `M-q` | 実装済み | `M-q` | TypeScript (`src/lib/emacs.ts`) | 6 |
| `set-fill-prefix` | ◯ | fill | `C-x .` |  |  |  | 6 |
| `comment-dwim` |  | newcomment | `M-;` | 実装済み | `M-;` | TypeScript (`src/lib/emacs.ts`) | 6 |
| `comment-line` |  | newcomment | `C-x C-;` |  |  |  | 6 |
| `comment-set-column` |  | newcomment | `C-x ;` |  |  |  | 6 |
| `occur` | ◯ | replace | `M-s o` |  |  |  | 7 |
| `query-replace` | ◯ | replace | `M-%` | 実装済み | `M-%` | TypeScript (`src/hooks/useSearch.ts`) | 7 |
| `query-replace-regexp` | ◯ | replace | `C-M-%` | 実装済み | `C-M-%` | TypeScript (`src/hooks/useSearch.ts`) | 7 |
| `isearch-backward` | ◯ | isearch | `C-r` | 実装済み | `C-r` | TypeScript (`src/hooks/useSearch.ts`) | 7 |
| `isearch-backward-regexp` | ◯ | isearch | `C-M-r` | 実装済み | `C-M-r` | TypeScript (`src/hooks/useSearch.ts`) | 7 |
| `isearch-forward` | ◯ | isearch | `C-s` | 実装済み | `C-s` | TypeScript (`src/hooks/useSearch.ts`) | 7 |
| `isearch-forward-regexp` | ◯ | isearch | `C-M-s` | 実装済み | `C-M-s` | TypeScript (`src/hooks/useSearch.ts`) | 7 |
| `isearch-forward-symbol` |  | isearch | `M-s _` |  |  |  | 7 |
| `isearch-forward-symbol-at-point` |  | isearch | `M-s .` |  |  |  | 7 |
| `isearch-forward-thing-at-point` |  | isearch | `M-s M-.` |  |  |  | 7 |
| `isearch-forward-word` |  | isearch | `M-s w` |  |  |  | 7 |
| `clear-rectangle` | ◯ | rect | `C-x r c` |  |  |  | 8 |
| `copy-rectangle-as-kill` |  | rect | `C-x r M-w` |  |  |  | 8 |
| `delete-rectangle` | ◯ | rect | `C-x r d` |  |  |  | 8 |
| `kill-rectangle` | ◯ | rect | `C-x r k` | 実装済み | `C-x r k` | TypeScript (`src/lib/emacs.ts`) | 8 |
| `open-rectangle` | ◯ | rect | `C-x r o` |  |  |  | 8 |
| `rectangle-number-lines` |  | rect | `C-x r N` |  |  |  | 8 |
| `string-rectangle` |  | rect | `C-x r t` | 実装済み | `C-x r t` | TypeScript (`src/App.tsx`) | 8 |
| `yank-rectangle` | ◯ | rect | `C-x r y` | 実装済み | `C-x r y` | TypeScript (`src/lib/emacs.ts`) | 8 |
| `copy-rectangle-to-register` |  | register | `C-x r r` |  |  |  | 8 |
| `copy-to-register` | ◯ | register | `C-x r s`, `C-x r x` | 実装済み | `C-x r s`, `C-x r x` | TypeScript (`src/lib/emacs.ts`) | 8 |
| `increment-register` | ◯ | register | `C-x r +` |  |  |  | 8 |
| `insert-register` | ◯ | register | `C-x r g`, `C-x r i` | 実装済み | `C-x r i`, `C-x r g` | TypeScript (`src/lib/emacs.ts`) | 8 |
| `jump-to-register` |  | register | `C-x r j` |  |  |  | 8 |
| `number-to-register` | ◯ | register | `C-x r n` |  |  |  | 8 |
| `point-to-register` | ◯ | register | `C-x r C-@`, `C-x r SPC`, `C-x r C-SPC` |  |  |  | 8 |
| `window-configuration-to-register` |  | register | `C-x r w` |  |  |  | 8 |
| `kmacro-end-and-call-macro` |  | kmacro | `C-x e` |  |  |  | 8 |
| `kmacro-end-call-mouse` |  | kmacro | `S-<mouse-3>` |  |  |  | 8 |
| `kmacro-end-macro` |  | kmacro | `C-x )` |  |  |  | 8 |
| `kmacro-end-or-call-macro` |  | kmacro | `<f4>` |  |  |  | 8 |
| `kmacro-name-last-macro` |  | kmacro | `C-x C-k n` |  |  |  | 8 |
| `kmacro-start-macro` |  | kmacro | `C-x (`, `C-x C-k s`, `C-x C-k C-s` |  |  |  | 8 |
| `kmacro-start-macro-or-insert-counter` |  | kmacro | `<f3>` |  |  |  | 8 |
| `abbrev-prefix-mark` | ◯ | abbrev | `M-'` |  |  |  | 8 |
| `add-global-abbrev` | ◯ | abbrev | `C-x a g` |  |  |  | 8 |
| `add-mode-abbrev` | ◯ | abbrev | `C-x a +`, `C-x a l`, `C-x a C-a` |  |  |  | 8 |
| `expand-abbrev` | ◯ | abbrev | `C-x '`, `C-x a '`, `C-x a e` |  |  |  | 8 |
| `inverse-add-global-abbrev` | ◯ | abbrev | `C-x a -`, `C-x a i g` |  |  |  | 8 |
| `inverse-add-mode-abbrev` | ◯ | abbrev | `C-x a i l` |  |  |  | 8 |
| `dabbrev-completion` |  | dabbrev | `C-M-/` |  |  |  | 8 |
| `dabbrev-expand` | ◯ | dabbrev | `M-/` | 実装済み | `M-/` | TypeScript (`src/lib/emacs.ts`) | 8 |
| `list-buffers` | ◯ | buff-menu | `C-x C-b` | 実装済み | `C-x C-b` | TypeScript (`src/App.tsx`) | 9 |
| `dired` | ◯ | dired | `C-x d` | 実装済み | `C-x d` | TypeScript (`src/hooks/useDirectories.ts`) | 9 |
| `dired-jump` |  | dired | `C-x C-j` |  |  |  | 9 |
| `dired-jump-other-window` |  | dired | `C-x 4 C-j` |  |  |  | 9 |
| `dired-other-frame` | ◯ | dired | `C-x 5 d` |  |  |  | 12 |
| `dired-other-tab` |  | dired | `C-x t d` |  |  |  | 12 |
| `dired-other-window` | ◯ | dired | `C-x 4 d` |  |  |  | 9 |
| `bookmark-bmenu-list` |  | bookmark | `C-x r l` |  |  |  | 9 |
| `bookmark-jump` |  | bookmark | `C-x r b` | 実装済み | `C-x r b` | TypeScript (`src/hooks/useBookmarks.ts`) | 9 |
| `bookmark-set` |  | bookmark | `C-x r m` | 実装済み | `C-x r m` | TypeScript (`src/hooks/useBookmarks.ts`) | 9 |
| `bookmark-set-no-overwrite` |  | bookmark | `C-x r M` |  |  |  | 9 |
| `xref-find-apropos` |  | xref | `C-M-.` |  |  |  | 11 |
| `xref-find-definitions` |  | xref | `M-.` |  |  |  | 11 |
| `xref-find-definitions-other-frame` |  | xref | `C-x 5 .` |  |  |  | 12 |
| `xref-find-definitions-other-window` |  | xref | `C-x 4 .` |  |  |  | 11 |
| `xref-find-references` |  | xref | `M-?` |  |  |  | 11 |
| `xref-go-back` |  | xref | `M-,` |  |  |  | 11 |
| `xref-go-forward` |  | xref | `C-M-,` |  |  |  | 11 |
| `highlight-lines-matching-regexp` |  | loaddefs | `M-s h l` |  |  |  | 7 |
| `highlight-phrase` |  | loaddefs | `M-s h p` |  |  |  | 7 |
| `highlight-regexp` |  | loaddefs | `M-s h r` |  |  |  | 7 |
| `highlight-symbol-at-point` |  | loaddefs | `M-s h .` |  |  |  | 7 |
| `unhighlight-regexp` |  | loaddefs | `M-s h u` |  |  |  | 7 |
| `describe-bindings` | ◯ | help | `C-h b`, `<f1> b`, `<help> b` | 実装済み | `C-h b` | TypeScript (`src/App.tsx`) | 11 |
| `describe-copying` |  | help | `C-h C-c`, `<f1> C-c`, `<help> C-c` |  |  |  | 12 |
| `describe-distribution` |  | help | `C-h C-o`, `<f1> C-o`, `<help> C-o` |  |  |  | 12 |
| `describe-gnu-project` |  | help | `C-h g`, `<f1> g`, `<help> g` |  |  |  | 12 |
| `describe-key` | ◯ | help | `C-h k`, `<f1> k`, `<help> k` | 実装済み | `C-h k` | TypeScript (`src/lib/emacs.ts`) | 11 |
| `describe-key-briefly` | ◯ | help | `C-h c`, `<f1> c`, `<help> c` |  |  |  | 11 |
| `describe-no-warranty` |  | help | `C-h C-w`, `<f1> C-w`, `<help> C-w` |  |  |  | 12 |
| `help-for-help` |  | help | `C-h ?`, `<f1> ?`, `C-h C-h`, `<f1> C-h` ほか 8 |  |  |  | 11 |
| `help-quick-toggle` |  | help | `C-h C-q`, `<f1> C-q`, `<help> C-q` |  |  |  | 11 |
| `help-quit` |  | help | `C-h q`, `<f1> q`, `<help> q` |  |  |  | 11 |
| `search-forward-help-for-help` |  | help | `C-h C-s`, `<f1> C-s`, `<help> C-s` |  |  |  | 11 |
| `view-echo-area-messages` |  | help | `C-h e`, `<f1> e`, `<help> e` |  |  |  | 11 |
| `view-emacs-FAQ` |  | help | `C-h C-f`, `<f1> C-f`, `<help> C-f` |  |  |  | 12 |
| `view-emacs-debugging` |  | help | `C-h C-d`, `<f1> C-d`, `<help> C-d` |  |  |  | 12 |
| `view-emacs-news` |  | help | `C-h n`, `<f1> n`, `C-h C-n`, `<f1> C-n` ほか 2 |  |  |  | 12 |
| `view-emacs-problems` |  | help | `C-h C-p`, `<f1> C-p`, `<help> C-p` |  |  |  | 12 |
| `view-emacs-todo` |  | help | `C-h C-t`, `<f1> C-t`, `<help> C-t` |  |  |  | 12 |
| `view-external-packages` |  | help | `C-h C-e`, `<f1> C-e`, `<help> C-e` |  |  |  | 12 |
| `view-lossage` | ◯ | help | `C-h l`, `<f1> l`, `<help> l` |  |  |  | 11 |
| `view-order-manuals` |  | help | `C-h RET`, `<f1> RET`, `<help> RET` |  |  |  | 12 |
| `where-is` | ◯ | help | `C-h w`, `<f1> w`, `<help> w` |  |  |  | 11 |
| `describe-command` |  | help-fns | `C-h x`, `<f1> x`, `<help> x` |  |  |  | 11 |
| `describe-function` | ◯ | help-fns | `C-h f`, `<f1> f`, `<help> f` | 実装済み | `C-h f` | Lisp (`lisp/help-fns.scm`) | 済 |
| `describe-symbol` |  | help-fns | `C-h o`, `<f1> o`, `<help> o` |  |  |  | 11 |
| `describe-syntax` | ◯ | help-fns | `C-h s`, `<f1> s`, `<help> s` |  |  |  | 11 |
| `describe-variable` | ◯ | help-fns | `C-h v`, `<f1> v`, `<help> v` |  |  |  | 11 |
| `help-find-source` |  | help-fns | `C-h 4 s`, `<f1> 4 s`, `<help> 4 s` |  |  |  | 11 |
| `Info-goto-emacs-command-node` |  | info | `C-h F`, `<f1> F`, `<help> F` |  |  |  | 11 |
| `Info-goto-emacs-key-command-node` |  | info | `C-h K`, `<f1> K`, `<help> K` |  |  |  | 11 |
| `info` | ◯ | info | `C-h i`, `<f1> i`, `<help> i` |  |  |  | 11 |
| `info-display-manual` |  | info | `C-h R`, `<f1> R`, `<help> R` |  |  |  | 11 |
| `info-emacs-manual` |  | info | `C-h r`, `<f1> r`, `<help> r` |  |  |  | 11 |
| `info-other-window` |  | info | `C-h 4 i`, `<f1> 4 i`, `<help> 4 i` |  |  |  | 11 |

<!-- emacs-commands:end -->
