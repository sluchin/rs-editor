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
- キーは Emacs の書き方 (`key-description`)。`C-c C-m` は `C-c RET`, `C-i` は `TAB` と書く。
- 「エディタの基本のコマンド」は, Lisp で書いたコマンド (`find-file` など) のうち, 既定の global-map にキーがあり, エディタの基本のライブラリ (`simple`, `files`, `window`, `bindings`, `lisp`, `indent`, `paragraphs`, `page`, `fill`, `newcomment`, `replace`, `isearch`, `rect`, `register`, `kmacro`, `abbrev`, `dabbrev`, `buff-menu`, `dired`, `bookmark`, `xref`, `loaddefs`, `help`, `help-fns`, `info`) にあるもの。マイナーモードのコマンド (`-mode`) と, 版管理 (`vc-`) は, 入れていない。

### 組み込みのコマンド (101 個. 実装済み 15 個: Lisp 0 個, TypeScript 15 個)

| コマンド | 既定のキー | rs-editor | rs-editor のキー | 実装 |
|---|---|---|---|---|
| `abort-minibuffers` | minibuffer-local-completion-map: `C-g`<br>minibuffer-local-isearch-map: `C-g`<br>minibuffer-local-map: `C-g` | 実装済み |  | TypeScript (`src/App.tsx`) |
| `abort-recursive-edit` | `C-]` | 実装済み | `C-]` | TypeScript (`src/App.tsx`) |
| `accelerate-menu` |  |  |  |  |
| `add-name-to-file` |  |  |  |  |
| `backward-char` | `C-b` | 実装済み | `C-b` | TypeScript (`src/lib/emacs.ts`) |
| `base64-decode-region` |  |  |  |  |
| `base64-encode-region` |  |  |  |  |
| `base64url-encode-region` |  |  |  |  |
| `beginning-of-line` |  |  |  |  |
| `buffer-enable-undo` |  |  |  |  |
| `call-last-kbd-macro` |  | 実装済み | `C-x e` | TypeScript (`src/lib/emacs.ts`) |
| `capitalize-region` |  |  |  |  |
| `capitalize-word` | `M-c` | 実装済み | `M-c` | TypeScript (`src/lib/emacs.ts`) |
| `chmod` |  |  |  |  |
| `copy-file` |  |  |  |  |
| `decode-coding-region` |  |  |  |  |
| `defining-kbd-macro` |  |  |  |  |
| `delete-char` | `C-d` | 実装済み | `C-d` | TypeScript (`src/lib/emacs.ts`) |
| `delete-frame` | `C-x 5 0` |  |  |  |
| `delete-other-windows-internal` |  |  |  |  |
| `delete-process` |  |  |  |  |
| `delete-region` |  |  |  |  |
| `do-auto-save` |  |  |  |  |
| `downcase-region` | `C-x C-l` | 実装済み | `C-x C-l` | TypeScript (`src/lib/emacs.ts`) |
| `downcase-word` | `M-l` | 実装済み | `M-l` | TypeScript (`src/lib/emacs.ts`) |
| `encode-coding-region` |  |  |  |  |
| `end-kbd-macro` |  | 実装済み | `C-x )` | TypeScript (`src/lib/emacs.ts`) |
| `end-of-line` |  |  |  |  |
| `erase-buffer` |  |  |  |  |
| `eval-buffer` |  |  |  |  |
| `eval-region` |  |  |  |  |
| `exit-recursive-edit` | `C-M-c` |  |  |  |
| `forward-char` | `C-f` | 実装済み | `C-f` | TypeScript (`src/lib/emacs.ts`) |
| `forward-line` |  |  |  |  |
| `forward-word` | `M-f`, `ESC <right>` | 実装済み | `M-f` | TypeScript (`src/lib/emacs.ts`) |
| `garbage-collect` |  |  |  |  |
| `goto-char` | `M-g c` |  |  |  |
| `handle-save-session` | special-event-map: `<save-session>` |  |  |  |
| `handle-switch-frame` | `<switch-frame>` |  |  |  |
| `iconify-frame` |  |  |  |  |
| `indent-to` |  |  |  |  |
| `indent-to-column` |  |  |  |  |
| `insert-char` | `C-x 8 RET` |  |  |  |
| `kill-buffer` | `C-x k` | 実装済み | `C-x k` | TypeScript (`src/App.tsx`) |
| `kill-emacs` |  |  |  |  |
| `kill-local-variable` |  |  |  |  |
| `kill-process` |  |  |  |  |
| `lossage-size` |  |  |  |  |
| `lower-frame` |  |  |  |  |
| `make-frame-invisible` |  |  |  |  |
| `make-frame-visible` |  |  |  |  |
| `make-indirect-buffer` |  |  |  |  |
| `make-local-variable` |  |  |  |  |
| `make-symbolic-link` |  |  |  |  |
| `make-variable-buffer-local` |  |  |  |  |
| `malloc-info` |  |  |  |  |
| `malloc-trim` |  |  |  |  |
| `modify-syntax-entry` |  |  |  |  |
| `move-to-column` | `M-g TAB` |  |  |  |
| `move-to-window-line` |  |  |  |  |
| `narrow-to-region` | `C-x n n` |  |  |  |
| `open-dribble-file` |  |  |  |  |
| `open-termscript` |  |  |  |  |
| `posix-search-backward` |  |  |  |  |
| `posix-search-forward` |  |  |  |  |
| `raise-frame` |  |  |  |  |
| `re-search-backward` |  |  |  |  |
| `re-search-forward` |  |  |  |  |
| `recenter` | kmacro-step-edit-map: `C-l`<br>multi-query-replace-map: `C-l`<br>query-replace-map: `C-l` |  |  |  |
| `recursive-edit` |  |  |  |  |
| `redirect-debugging-output` |  |  |  |  |
| `redraw-display` |  |  |  |  |
| `rename-buffer` | `C-x x r` |  |  |  |
| `rename-file` |  |  |  |  |
| `replace-buffer-contents` |  |  |  |  |
| `scroll-down` | kmacro-step-edit-map: `M-v`, `<prior>`<br>multi-query-replace-map: `M-v`, `<prior>`<br>query-replace-map: `M-v`, `<prior>` |  |  |  |
| `scroll-left` | `C-x <`, `C-<next>` |  |  |  |
| `scroll-right` | `C-x >`, `C-<prior>` |  |  |  |
| `scroll-up` | kmacro-step-edit-map: `C-v`, `<next>`<br>multi-query-replace-map: `C-v`, `<next>`<br>query-replace-map: `C-v`, `<next>` |  |  |  |
| `search-backward` |  |  |  |  |
| `search-backward-regexp` |  |  |  |  |
| `search-forward` | `<find>` |  |  |  |
| `search-forward-regexp` |  |  |  |  |
| `select-frame` |  |  |  |  |
| `self-insert-command` | 文字のキー (`a` など) |  |  |  |
| `set-file-modes` |  |  |  |  |
| `set-frame-height` |  |  |  |  |
| `set-frame-width` |  |  |  |  |
| `signal-process` |  |  |  |  |
| `start-kbd-macro` |  | 実装済み | `C-x (` | TypeScript (`src/lib/emacs.ts`) |
| `suspend-emacs` |  |  |  |  |
| `top-level` |  |  |  |  |
| `transpose-regions` |  |  |  |  |
| `unix-sync` |  |  |  |  |
| `upcase-initials-region` |  |  |  |  |
| `upcase-region` | `C-x C-u` | 実装済み | `C-x C-u` | TypeScript (`src/lib/emacs.ts`) |
| `upcase-word` | `M-u` | 実装済み | `M-u` | TypeScript (`src/lib/emacs.ts`) |
| `widen` | `C-x n w` |  |  |  |
| `write-region` |  |  |  |  |
| `x-menu-bar-open-internal` |  |  |  |  |
| `x-print-frames-dialog` |  |  |  |  |

### エディタの基本のコマンド (Lisp で書いたもの. 262 個. 実装済み 92 個: Lisp 0 個, TypeScript 92 個)

| コマンド | ライブラリ | 既定のキー | rs-editor | rs-editor のキー | 実装 |
|---|---|---|---|---|---|
| `analyze-text-conversion` | simple | `<text-conversion>` |  |  |  |
| `append-next-kill` | simple | `C-M-w` |  |  |  |
| `async-shell-command` | simple | `M-&` |  |  |  |
| `back-to-indentation` | simple | `M-m` | 実装済み | `M-m` | TypeScript (`src/lib/emacs.ts`) |
| `backward-kill-word` | simple | `M-DEL`, `C-<backspace>` | 実装済み | `M-DEL` | TypeScript (`src/lib/emacs.ts`) |
| `backward-word` | simple | `M-b`, `ESC <left>` | 実装済み | `M-b` | TypeScript (`src/lib/emacs.ts`) |
| `beginning-of-buffer` | simple | `M-<`, `<begin>`, `C-<home>` | 実装済み | `M-<` | TypeScript (`src/lib/emacs.ts`) |
| `clone-buffer` | simple | `C-x x n` |  |  |  |
| `clone-indirect-buffer-other-window` | simple | `C-x 4 c` |  |  |  |
| `compose-mail` | simple | `C-x m` |  |  |  |
| `compose-mail-other-frame` | simple | `C-x 5 m` |  |  |  |
| `compose-mail-other-window` | simple | `C-x 4 m` |  |  |  |
| `count-words-region` | simple | `M-=` | 実装済み | `M-=` | TypeScript (`src/lib/emacs.ts`) |
| `cycle-spacing` | simple | `M-SPC` |  |  |  |
| `default-indent-new-line` | simple | `M-j`, `C-M-j` |  |  |  |
| `delete-backward-char` | simple | `DEL` |  |  |  |
| `delete-blank-lines` | simple | `C-x C-o` | 実装済み | `C-x C-o` | TypeScript (`src/lib/emacs.ts`) |
| `delete-forward-char` | simple | `<deletechar>` |  |  |  |
| `delete-horizontal-space` | simple | `M-\` | 実装済み | `M-\` | TypeScript (`src/lib/emacs.ts`) |
| `delete-indentation` | simple | `M-^` | 実装済み | `M-^` | TypeScript (`src/lib/emacs.ts`) |
| `digit-argument` | simple | `C-0`, `C-1`, `C-2`, `C-3` ほか 16 | 実装済み | `M-0`, `M-1`, `M-2` ほか 7 | TypeScript (`src/lib/emacs.ts`) |
| `end-of-buffer` | simple | `M->`, `C-<end>` | 実装済み | `M->` | TypeScript (`src/lib/emacs.ts`) |
| `eval-expression` | simple | `M-:`, `ESC M-:` | 実装済み | `M-:` | TypeScript (`src/hooks/useLisp.ts`) |
| `exchange-point-and-mark` | simple | `C-x C-x` | 実装済み | `C-x C-x` | TypeScript (`src/lib/emacs.ts`) |
| `execute-extended-command` | simple | `M-x`, `<menu>`, `<execute>` | 実装済み | `M-x` | TypeScript (`src/App.tsx`) |
| `execute-extended-command-for-buffer` | simple | `M-X` |  |  |  |
| `goto-line` | simple | `M-g g`, `M-g M-g` | 実装済み | `M-g g`, `M-g M-g` | TypeScript (`src/App.tsx`) |
| `goto-line-relative` | simple | `C-x n g` |  |  |  |
| `insert-buffer` | simple | `C-x x i` |  |  |  |
| `keyboard-escape-quit` | simple | `ESC ESC ESC` |  |  |  |
| `keyboard-quit` | simple | `C-g` | 実装済み | `C-g` | TypeScript (`src/lib/emacs.ts`) |
| `kill-line` | simple | `C-k`, `<deleteline>` | 実装済み | `C-k` | TypeScript (`src/lib/emacs.ts`) |
| `kill-region` | simple | `C-w`, `<cut>`, `S-<delete>` | 実装済み | `C-w` | TypeScript (`src/lib/emacs.ts`) |
| `kill-ring-save` | simple | `M-w`, `<copy>`, `C-<insert>`, `C-<insertchar>` | 実装済み | `M-w` | TypeScript (`src/lib/emacs.ts`) |
| `kill-whole-line` | simple | `C-S-<backspace>` |  |  |  |
| `kill-word` | simple | `M-d`, `C-<delete>` | 実装済み | `M-d` | TypeScript (`src/lib/emacs.ts`) |
| `mark-whole-buffer` | simple | `C-x h` | 実装済み | `C-x h` | TypeScript (`src/lib/emacs.ts`) |
| `mark-word` | simple | `M-@` | 実装済み | `M-@` | TypeScript (`src/lib/emacs.ts`) |
| `move-beginning-of-line` | simple | `C-a`, `<home>` | 実装済み | `C-a` | TypeScript (`src/lib/emacs.ts`) |
| `move-end-of-line` | simple | `C-e`, `<end>` | 実装済み | `C-e` | TypeScript (`src/lib/emacs.ts`) |
| `negative-argument` | simple | `C--`, `M--`, `C-M--` |  |  |  |
| `newline` | simple | `RET` | 実装済み | `RET`, `C-m` | TypeScript (`src/lib/emacs.ts`) |
| `next-error` | simple | `` C-x ` ``, `M-g n`, `M-g M-n` | 実装済み | `M-g n`, `M-g M-n`, `` C-x ` `` | TypeScript (`src/hooks/useDirectories.ts`) |
| `next-line` | simple | `C-n`, `<down>` | 実装済み | `C-n` | TypeScript (`src/lib/emacs.ts`) |
| `open-line` | simple | `C-o`, `<insertline>` | 実装済み | `C-o` | TypeScript (`src/lib/emacs.ts`) |
| `pop-global-mark` | simple | `C-x C-@`, `C-x C-SPC` |  |  |  |
| `previous-error` | simple | `M-g p`, `M-g M-p` | 実装済み | `M-g p`, `M-g M-p` | TypeScript (`src/hooks/useDirectories.ts`) |
| `previous-line` | simple | `C-p`, `<up>` | 実装済み | `C-p` | TypeScript (`src/lib/emacs.ts`) |
| `quoted-insert` | simple | `C-q` | 実装済み | `C-q` | TypeScript (`src/lib/emacs.ts`) |
| `repeat-complex-command` | simple | `<redo>`, `<again>`, `C-x M-:`, `C-x ESC ESC` |  |  |  |
| `set-fill-column` | simple | `C-x f` |  |  |  |
| `set-goal-column` | simple | `C-x C-n` | 実装済み | `C-x C-n` | TypeScript (`src/lib/emacs.ts`) |
| `set-mark-command` | simple | `C-@`, `C-SPC` | 実装済み | `C-SPC` | TypeScript (`src/lib/emacs.ts`) |
| `set-selective-display` | simple | `C-x $` |  |  |  |
| `shell-command` | simple | `M-!` |  |  |  |
| `shell-command-on-region` | simple | `M-\|` |  |  |  |
| `split-line` | simple | `C-M-o` |  |  |  |
| `toggle-truncate-lines` | simple | `C-x x t` |  |  |  |
| `transpose-chars` | simple | `C-t` | 実装済み | `C-t` | TypeScript (`src/lib/emacs.ts`) |
| `transpose-lines` | simple | `C-x C-t` | 実装済み | `C-x C-t` | TypeScript (`src/lib/emacs.ts`) |
| `transpose-sexps` | simple | `C-M-t` |  |  |  |
| `transpose-words` | simple | `M-t` | 実装済み | `M-t` | TypeScript (`src/lib/emacs.ts`) |
| `undo` | simple | `C-/`, `C-_`, `C-x u`, `<undo>` | 実装済み | `C-/`, `C-_`, `C-x u` | TypeScript (`src/lib/emacs.ts`) |
| `undo-redo` | simple | `C-?`, `C-M-_` |  |  |  |
| `universal-argument` | simple | `C-u` | 実装済み | `C-u` | TypeScript (`src/lib/emacs.ts`) |
| `what-cursor-position` | simple | `C-x =` |  |  |  |
| `yank` | simple | `C-y`, `<paste>`, `S-<insert>`, `S-<insertchar>` | 実装済み | `C-y` | TypeScript (`src/lib/emacs.ts`) |
| `yank-pop` | simple | `M-y` | 実装済み | `M-y` | TypeScript (`src/lib/emacs.ts`) |
| `zap-to-char` | simple | `M-z` | 実装済み | `M-z` | TypeScript (`src/lib/emacs.ts`) |
| `find-alternate-file` | files | `C-x C-v` | 実装済み | `C-x C-v` | TypeScript (`src/hooks/useFiles.ts`) |
| `find-file` | files | `<open>`, `C-x C-f` | 実装済み | `C-x C-f` | TypeScript (`src/hooks/useFiles.ts`) |
| `find-file-other-frame` | files | `C-x 5 f`, `C-x 5 C-f` |  |  |  |
| `find-file-other-window` | files | `C-x 4 f`, `C-x 4 C-f` |  |  |  |
| `find-file-read-only` | files | `C-x C-r` |  |  |  |
| `find-file-read-only-other-frame` | files | `C-x 5 r` |  |  |  |
| `find-file-read-only-other-window` | files | `C-x 4 r` |  |  |  |
| `insert-file` | files | `C-x i` | 実装済み | `C-x i` | TypeScript (`src/hooks/useFiles.ts`) |
| `list-directory` | files | `C-x C-d` |  |  |  |
| `not-modified` | files | `M-~` |  |  |  |
| `rename-uniquely` | files | `C-x x u` |  |  |  |
| `revert-buffer-quick` | files | `C-x x g` |  |  |  |
| `save-buffer` | files | `C-x C-s` | 実装済み | `C-x C-s` | TypeScript (`src/hooks/useFiles.ts`) |
| `save-buffers-kill-terminal` | files | `C-x C-c` | 実装済み | `C-x C-c` | TypeScript (`src/App.tsx`) |
| `save-some-buffers` | files | `C-x s` | 実装済み | `C-x s` | TypeScript (`src/hooks/useFiles.ts`) |
| `write-file` | files | `C-x C-w` | 実装済み | `C-x C-w` | TypeScript (`src/hooks/useFiles.ts`) |
| `balance-windows` | window | `C-x +` |  |  |  |
| `beginning-of-buffer-other-window` | window | `M-<home>`, `M-<begin>`, `ESC <home>`, `ESC <begin>` |  |  |  |
| `delete-other-windows` | window | `C-x 1` | 実装済み | `C-x 1` | TypeScript (`src/hooks/useBuffers.ts`) |
| `delete-window` | window | `C-x 0` | 実装済み | `C-x 0` | TypeScript (`src/hooks/useBuffers.ts`) |
| `delete-windows-on` | window | `C-x w 0` |  |  |  |
| `display-buffer` | window | `C-x 4 C-o` |  |  |  |
| `display-buffer-other-frame` | window | `C-x 5 C-o` |  |  |  |
| `end-of-buffer-other-window` | window | `M-<end>`, `ESC <end>` |  |  |  |
| `enlarge-window` | window | `C-x ^` |  |  |  |
| `enlarge-window-horizontally` | window | `C-x }` |  |  |  |
| `fit-window-to-buffer` | window | `C-x w -` |  |  |  |
| `handle-select-window` | window | `<select-window>` |  |  |  |
| `kill-buffer-and-window` | window | `C-x 4 0` |  |  |  |
| `move-to-window-line-top-bottom` | window | `M-r` |  |  |  |
| `next-buffer` | window | `C-x <right>`, `<XF86Forward>`, `C-x C-<right>` |  |  |  |
| `other-window` | window | `C-x o` | 実装済み | `C-x o` | TypeScript (`src/hooks/useBuffers.ts`) |
| `other-window-prefix` | window | `C-x 4 4` |  |  |  |
| `previous-buffer` | window | `<XF86Back>`, `C-x <left>`, `C-x C-<left>` |  |  |  |
| `quit-window` | window | `C-x w q` |  |  |  |
| `recenter-other-window` | window | `C-M-S-l` |  |  |  |
| `recenter-top-bottom` | window | `C-l` | 実装済み | `C-l` | TypeScript (`src/lib/emacs.ts`) |
| `same-window-prefix` | window | `C-x 4 1` |  |  |  |
| `scroll-down-command` | window | `M-v`, `<prior>` | 実装済み | `M-v` | TypeScript (`src/lib/emacs.ts`) |
| `scroll-other-window` | window | `C-M-v`, `M-<next>`, `ESC <next>` |  |  |  |
| `scroll-other-window-down` | window | `C-M-S-v`, `M-<prior>`, `ESC <prior>` |  |  |  |
| `scroll-up-command` | window | `C-v`, `<next>` | 実装済み | `C-v` | TypeScript (`src/lib/emacs.ts`) |
| `shrink-window-horizontally` | window | `C-x {` |  |  |  |
| `shrink-window-if-larger-than-buffer` | window | `C-x -` |  |  |  |
| `split-root-window-below` | window | `C-x w 2` |  |  |  |
| `split-root-window-right` | window | `C-x w 3` |  |  |  |
| `split-window-below` | window | `C-x 2` | 実装済み | `C-x 2` | TypeScript (`src/hooks/useBuffers.ts`) |
| `split-window-right` | window | `C-x 3` | 実装済み | `C-x 3` | TypeScript (`src/hooks/useBuffers.ts`) |
| `switch-to-buffer` | window | `C-x b` | 実装済み | `C-x b` | TypeScript (`src/App.tsx`) |
| `switch-to-buffer-other-frame` | window | `C-x 5 b` |  |  |  |
| `switch-to-buffer-other-window` | window | `C-x 4 b` |  |  |  |
| `toggle-window-dedicated` | window | `C-x w d` |  |  |  |
| `window-toggle-side-windows` | window | `C-x w s` |  |  |  |
| `complete-symbol` | bindings | `C-M-i` |  |  |  |
| `ignore-preserving-kill-region` | bindings | `<mouse-movement>` |  |  |  |
| `left-char` | bindings | `<left>` |  |  |  |
| `left-word` | bindings | `C-<left>`, `M-<left>` |  |  |  |
| `right-char` | bindings | `<right>` |  |  |  |
| `right-word` | bindings | `C-<right>`, `M-<right>` |  |  |  |
| `backward-kill-sexp` | lisp | `C-M-<delete>`, `ESC C-<delete>`, `C-M-<backspace>`, `ESC C-<backspace>` |  |  |  |
| `backward-list` | lisp | `C-M-p` |  |  |  |
| `backward-sexp` | lisp | `C-M-b`, `C-M-<left>`, `ESC C-<left>` | 実装済み | `C-M-b` | TypeScript (`src/lib/emacs.ts`) |
| `backward-up-list` | lisp | `C-M-u`, `C-M-<up>`, `ESC C-<up>` | 実装済み | `C-M-u` | TypeScript (`src/lib/emacs.ts`) |
| `beginning-of-defun` | lisp | `C-M-a`, `C-M-<home>`, `ESC C-<home>` | 実装済み | `C-M-a` | TypeScript (`src/lib/emacs.ts`) |
| `down-list` | lisp | `C-M-d`, `C-M-<down>`, `ESC C-<down>` | 実装済み | `C-M-d` | TypeScript (`src/lib/emacs.ts`) |
| `end-of-defun` | lisp | `C-M-e`, `C-M-<end>`, `ESC C-<end>` | 実装済み | `C-M-e` | TypeScript (`src/lib/emacs.ts`) |
| `forward-list` | lisp | `C-M-n` |  |  |  |
| `forward-sexp` | lisp | `C-M-f`, `C-M-<right>`, `ESC C-<right>` | 実装済み | `C-M-f` | TypeScript (`src/lib/emacs.ts`) |
| `insert-parentheses` | lisp | `M-(` |  |  |  |
| `kill-sexp` | lisp | `C-M-k` | 実装済み | `C-M-k` | TypeScript (`src/lib/emacs.ts`) |
| `mark-defun` | lisp | `C-M-h` | 実装済み | `C-M-h` | TypeScript (`src/lib/emacs.ts`) |
| `mark-sexp` | lisp | `C-M-@`, `C-M-SPC` | 実装済み | `C-M-SPC` | TypeScript (`src/lib/emacs.ts`) |
| `move-past-close-and-reindent` | lisp | `M-)` |  |  |  |
| `narrow-to-defun` | lisp | `C-x n d` |  |  |  |
| `indent-for-tab-command` | indent | `TAB` | 実装済み | `TAB`, `C-i` | TypeScript (`src/lib/emacs.ts`) |
| `indent-region` | indent | `C-M-\` | 実装済み | `C-M-\` | TypeScript (`src/lib/emacs.ts`) |
| `indent-rigidly` | indent | `C-x TAB` |  |  |  |
| `tab-to-tab-stop` | indent | `M-i` |  |  |  |
| `backward-kill-sentence` | paragraphs | `C-x DEL` |  |  |  |
| `backward-paragraph` | paragraphs | `M-{`, `C-<up>` | 実装済み | `M-{` | TypeScript (`src/lib/emacs.ts`) |
| `backward-sentence` | paragraphs | `M-a` | 実装済み | `M-a` | TypeScript (`src/lib/emacs.ts`) |
| `forward-paragraph` | paragraphs | `M-}`, `C-<down>` | 実装済み | `M-}` | TypeScript (`src/lib/emacs.ts`) |
| `forward-sentence` | paragraphs | `M-e` | 実装済み | `M-e` | TypeScript (`src/lib/emacs.ts`) |
| `kill-sentence` | paragraphs | `M-k` | 実装済み | `M-k` | TypeScript (`src/lib/emacs.ts`) |
| `mark-paragraph` | paragraphs | `M-h` |  |  |  |
| `backward-page` | page | `C-x [` |  |  |  |
| `count-lines-page` | page | `C-x l` |  |  |  |
| `forward-page` | page | `C-x ]` |  |  |  |
| `mark-page` | page | `C-x C-p` |  |  |  |
| `narrow-to-page` | page | `C-x n p` |  |  |  |
| `fill-paragraph` | fill | `M-q` | 実装済み | `M-q` | TypeScript (`src/lib/emacs.ts`) |
| `set-fill-prefix` | fill | `C-x .` |  |  |  |
| `comment-dwim` | newcomment | `M-;` | 実装済み | `M-;` | TypeScript (`src/lib/emacs.ts`) |
| `comment-line` | newcomment | `C-x C-;` |  |  |  |
| `comment-set-column` | newcomment | `C-x ;` |  |  |  |
| `occur` | replace | `M-s o` |  |  |  |
| `query-replace` | replace | `M-%` | 実装済み | `M-%` | TypeScript (`src/hooks/useSearch.ts`) |
| `query-replace-regexp` | replace | `C-M-%` | 実装済み | `C-M-%` | TypeScript (`src/hooks/useSearch.ts`) |
| `isearch-backward` | isearch | `C-r` | 実装済み | `C-r` | TypeScript (`src/hooks/useSearch.ts`) |
| `isearch-backward-regexp` | isearch | `C-M-r` | 実装済み | `C-M-r` | TypeScript (`src/hooks/useSearch.ts`) |
| `isearch-forward` | isearch | `C-s` | 実装済み | `C-s` | TypeScript (`src/hooks/useSearch.ts`) |
| `isearch-forward-regexp` | isearch | `C-M-s` | 実装済み | `C-M-s` | TypeScript (`src/hooks/useSearch.ts`) |
| `isearch-forward-symbol` | isearch | `M-s _` |  |  |  |
| `isearch-forward-symbol-at-point` | isearch | `M-s .` |  |  |  |
| `isearch-forward-thing-at-point` | isearch | `M-s M-.` |  |  |  |
| `isearch-forward-word` | isearch | `M-s w` |  |  |  |
| `clear-rectangle` | rect | `C-x r c` |  |  |  |
| `copy-rectangle-as-kill` | rect | `C-x r M-w` |  |  |  |
| `delete-rectangle` | rect | `C-x r d` |  |  |  |
| `kill-rectangle` | rect | `C-x r k` | 実装済み | `C-x r k` | TypeScript (`src/lib/emacs.ts`) |
| `open-rectangle` | rect | `C-x r o` |  |  |  |
| `rectangle-number-lines` | rect | `C-x r N` |  |  |  |
| `string-rectangle` | rect | `C-x r t` | 実装済み | `C-x r t` | TypeScript (`src/App.tsx`) |
| `yank-rectangle` | rect | `C-x r y` | 実装済み | `C-x r y` | TypeScript (`src/lib/emacs.ts`) |
| `copy-rectangle-to-register` | register | `C-x r r` |  |  |  |
| `copy-to-register` | register | `C-x r s`, `C-x r x` | 実装済み | `C-x r s`, `C-x r x` | TypeScript (`src/lib/emacs.ts`) |
| `increment-register` | register | `C-x r +` |  |  |  |
| `insert-register` | register | `C-x r g`, `C-x r i` | 実装済み | `C-x r i`, `C-x r g` | TypeScript (`src/lib/emacs.ts`) |
| `jump-to-register` | register | `C-x r j` |  |  |  |
| `number-to-register` | register | `C-x r n` |  |  |  |
| `point-to-register` | register | `C-x r C-@`, `C-x r SPC`, `C-x r C-SPC` |  |  |  |
| `window-configuration-to-register` | register | `C-x r w` |  |  |  |
| `kmacro-end-and-call-macro` | kmacro | `C-x e` |  |  |  |
| `kmacro-end-call-mouse` | kmacro | `S-<mouse-3>` |  |  |  |
| `kmacro-end-macro` | kmacro | `C-x )` |  |  |  |
| `kmacro-end-or-call-macro` | kmacro | `<f4>` |  |  |  |
| `kmacro-name-last-macro` | kmacro | `C-x C-k n` |  |  |  |
| `kmacro-start-macro` | kmacro | `C-x (`, `C-x C-k s`, `C-x C-k C-s` |  |  |  |
| `kmacro-start-macro-or-insert-counter` | kmacro | `<f3>` |  |  |  |
| `abbrev-prefix-mark` | abbrev | `M-'` |  |  |  |
| `add-global-abbrev` | abbrev | `C-x a g` |  |  |  |
| `add-mode-abbrev` | abbrev | `C-x a +`, `C-x a l`, `C-x a C-a` |  |  |  |
| `expand-abbrev` | abbrev | `C-x '`, `C-x a '`, `C-x a e` |  |  |  |
| `inverse-add-global-abbrev` | abbrev | `C-x a -`, `C-x a i g` |  |  |  |
| `inverse-add-mode-abbrev` | abbrev | `C-x a i l` |  |  |  |
| `dabbrev-completion` | dabbrev | `C-M-/` |  |  |  |
| `dabbrev-expand` | dabbrev | `M-/` | 実装済み | `M-/` | TypeScript (`src/lib/emacs.ts`) |
| `list-buffers` | buff-menu | `C-x C-b` | 実装済み | `C-x C-b` | TypeScript (`src/App.tsx`) |
| `dired` | dired | `C-x d` | 実装済み | `C-x d` | TypeScript (`src/hooks/useDirectories.ts`) |
| `dired-jump` | dired | `C-x C-j` |  |  |  |
| `dired-jump-other-window` | dired | `C-x 4 C-j` |  |  |  |
| `dired-other-frame` | dired | `C-x 5 d` |  |  |  |
| `dired-other-tab` | dired | `C-x t d` |  |  |  |
| `dired-other-window` | dired | `C-x 4 d` |  |  |  |
| `bookmark-bmenu-list` | bookmark | `C-x r l` |  |  |  |
| `bookmark-jump` | bookmark | `C-x r b` | 実装済み | `C-x r b` | TypeScript (`src/hooks/useBookmarks.ts`) |
| `bookmark-set` | bookmark | `C-x r m` | 実装済み | `C-x r m` | TypeScript (`src/hooks/useBookmarks.ts`) |
| `bookmark-set-no-overwrite` | bookmark | `C-x r M` |  |  |  |
| `xref-find-apropos` | xref | `C-M-.` |  |  |  |
| `xref-find-definitions` | xref | `M-.` |  |  |  |
| `xref-find-definitions-other-frame` | xref | `C-x 5 .` |  |  |  |
| `xref-find-definitions-other-window` | xref | `C-x 4 .` |  |  |  |
| `xref-find-references` | xref | `M-?` |  |  |  |
| `xref-go-back` | xref | `M-,` |  |  |  |
| `xref-go-forward` | xref | `C-M-,` |  |  |  |
| `highlight-lines-matching-regexp` | loaddefs | `M-s h l` |  |  |  |
| `highlight-phrase` | loaddefs | `M-s h p` |  |  |  |
| `highlight-regexp` | loaddefs | `M-s h r` |  |  |  |
| `highlight-symbol-at-point` | loaddefs | `M-s h .` |  |  |  |
| `unhighlight-regexp` | loaddefs | `M-s h u` |  |  |  |
| `describe-bindings` | help | `C-h b`, `<f1> b`, `<help> b` | 実装済み | `C-h b` | TypeScript (`src/App.tsx`) |
| `describe-copying` | help | `C-h C-c`, `<f1> C-c`, `<help> C-c` |  |  |  |
| `describe-distribution` | help | `C-h C-o`, `<f1> C-o`, `<help> C-o` |  |  |  |
| `describe-gnu-project` | help | `C-h g`, `<f1> g`, `<help> g` |  |  |  |
| `describe-key` | help | `C-h k`, `<f1> k`, `<help> k` | 実装済み | `C-h k` | TypeScript (`src/lib/emacs.ts`) |
| `describe-key-briefly` | help | `C-h c`, `<f1> c`, `<help> c` |  |  |  |
| `describe-no-warranty` | help | `C-h C-w`, `<f1> C-w`, `<help> C-w` |  |  |  |
| `help-for-help` | help | `C-h ?`, `<f1> ?`, `C-h C-h`, `<f1> C-h` ほか 8 |  |  |  |
| `help-quick-toggle` | help | `C-h C-q`, `<f1> C-q`, `<help> C-q` |  |  |  |
| `help-quit` | help | `C-h q`, `<f1> q`, `<help> q` |  |  |  |
| `search-forward-help-for-help` | help | `C-h C-s`, `<f1> C-s`, `<help> C-s` |  |  |  |
| `view-echo-area-messages` | help | `C-h e`, `<f1> e`, `<help> e` |  |  |  |
| `view-emacs-FAQ` | help | `C-h C-f`, `<f1> C-f`, `<help> C-f` |  |  |  |
| `view-emacs-debugging` | help | `C-h C-d`, `<f1> C-d`, `<help> C-d` |  |  |  |
| `view-emacs-news` | help | `C-h n`, `<f1> n`, `C-h C-n`, `<f1> C-n` ほか 2 |  |  |  |
| `view-emacs-problems` | help | `C-h C-p`, `<f1> C-p`, `<help> C-p` |  |  |  |
| `view-emacs-todo` | help | `C-h C-t`, `<f1> C-t`, `<help> C-t` |  |  |  |
| `view-external-packages` | help | `C-h C-e`, `<f1> C-e`, `<help> C-e` |  |  |  |
| `view-lossage` | help | `C-h l`, `<f1> l`, `<help> l` |  |  |  |
| `view-order-manuals` | help | `C-h RET`, `<f1> RET`, `<help> RET` |  |  |  |
| `where-is` | help | `C-h w`, `<f1> w`, `<help> w` |  |  |  |
| `describe-command` | help-fns | `C-h x`, `<f1> x`, `<help> x` |  |  |  |
| `describe-function` | help-fns | `C-h f`, `<f1> f`, `<help> f` |  |  |  |
| `describe-symbol` | help-fns | `C-h o`, `<f1> o`, `<help> o` |  |  |  |
| `describe-syntax` | help-fns | `C-h s`, `<f1> s`, `<help> s` |  |  |  |
| `describe-variable` | help-fns | `C-h v`, `<f1> v`, `<help> v` |  |  |  |
| `help-find-source` | help-fns | `C-h 4 s`, `<f1> 4 s`, `<help> 4 s` |  |  |  |
| `Info-goto-emacs-command-node` | info | `C-h F`, `<f1> F`, `<help> F` |  |  |  |
| `Info-goto-emacs-key-command-node` | info | `C-h K`, `<f1> K`, `<help> K` |  |  |  |
| `info` | info | `C-h i`, `<f1> i`, `<help> i` |  |  |  |
| `info-display-manual` | info | `C-h R`, `<f1> R`, `<help> R` |  |  |  |
| `info-emacs-manual` | info | `C-h r`, `<f1> r`, `<help> r` |  |  |  |
| `info-other-window` | info | `C-h 4 i`, `<f1> 4 i`, `<help> 4 i` |  |  |  |

<!-- emacs-commands:end -->
