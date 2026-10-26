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

## Emacs の組み込みのコマンド

Emacs 30.2 の組み込みのコマンド (C で書いたもの) と, その既定のキー。`tools/emacs-commands/run.sh` で作った一覧 (手で直さない。作り直すときは, `run.sh` を動かす)。

- **既定のキー**: `emacs -Q` の global-map のキー。global-map に無ければ, ほかのキーマップ (ミニバッファなど) のキーを, キーマップの名前と書く。
- **rs-editor**: rs-editor にあるコマンドは「実装済み」と, rs-editor のキーを書く。
- キーは Emacs の書き方 (`key-description`)。`C-c C-m` は `C-c RET`, `C-i` は `TAB` と書く。
- Lisp で書いたコマンド (`find-file` など) と, マイナーモードのコマンドは, ここには入れていない。

### 組み込みのコマンド (101 個. 実装済み 13 個)

| コマンド | 既定のキー | rs-editor | rs-editor のキー |
|---|---|---|---|
| `abort-minibuffers` | minibuffer-local-completion-map: `C-g`<br>minibuffer-local-isearch-map: `C-g`<br>minibuffer-local-map: `C-g` |  |  |
| `abort-recursive-edit` | `C-]` |  |  |
| `accelerate-menu` |  |  |  |
| `add-name-to-file` |  |  |  |
| `backward-char` | `C-b` | 実装済み | `C-b` |
| `base64-decode-region` |  |  |  |
| `base64-encode-region` |  |  |  |
| `base64url-encode-region` |  |  |  |
| `beginning-of-line` |  |  |  |
| `buffer-enable-undo` |  |  |  |
| `call-last-kbd-macro` |  | 実装済み | `C-x e` |
| `capitalize-region` |  |  |  |
| `capitalize-word` | `M-c` | 実装済み | `M-c` |
| `chmod` |  |  |  |
| `copy-file` |  |  |  |
| `decode-coding-region` |  |  |  |
| `defining-kbd-macro` |  |  |  |
| `delete-char` | `C-d` | 実装済み | `C-d` |
| `delete-frame` | `C-x 5 0` |  |  |
| `delete-other-windows-internal` |  |  |  |
| `delete-process` |  |  |  |
| `delete-region` |  |  |  |
| `do-auto-save` |  |  |  |
| `downcase-region` | `C-x C-l` | 実装済み | `C-x C-l` |
| `downcase-word` | `M-l` | 実装済み | `M-l` |
| `encode-coding-region` |  |  |  |
| `end-kbd-macro` |  | 実装済み | `C-x )` |
| `end-of-line` |  |  |  |
| `erase-buffer` |  |  |  |
| `eval-buffer` |  |  |  |
| `eval-region` |  |  |  |
| `exit-recursive-edit` | `C-M-c` |  |  |
| `forward-char` | `C-f` | 実装済み | `C-f` |
| `forward-line` |  |  |  |
| `forward-word` | `M-f`, `ESC <right>` | 実装済み | `M-f` |
| `garbage-collect` |  |  |  |
| `goto-char` | `M-g c` |  |  |
| `handle-save-session` | special-event-map: `<save-session>` |  |  |
| `handle-switch-frame` | `<switch-frame>` |  |  |
| `iconify-frame` |  |  |  |
| `indent-to` |  |  |  |
| `indent-to-column` |  |  |  |
| `insert-char` | `C-x 8 RET` |  |  |
| `kill-buffer` | `C-x k` | 実装済み | `C-x k` |
| `kill-emacs` |  |  |  |
| `kill-local-variable` |  |  |  |
| `kill-process` |  |  |  |
| `lossage-size` |  |  |  |
| `lower-frame` |  |  |  |
| `make-frame-invisible` |  |  |  |
| `make-frame-visible` |  |  |  |
| `make-indirect-buffer` |  |  |  |
| `make-local-variable` |  |  |  |
| `make-symbolic-link` |  |  |  |
| `make-variable-buffer-local` |  |  |  |
| `malloc-info` |  |  |  |
| `malloc-trim` |  |  |  |
| `modify-syntax-entry` |  |  |  |
| `move-to-column` | `M-g TAB` |  |  |
| `move-to-window-line` |  |  |  |
| `narrow-to-region` | `C-x n n` |  |  |
| `open-dribble-file` |  |  |  |
| `open-termscript` |  |  |  |
| `posix-search-backward` |  |  |  |
| `posix-search-forward` |  |  |  |
| `raise-frame` |  |  |  |
| `re-search-backward` |  |  |  |
| `re-search-forward` |  |  |  |
| `recenter` | kmacro-step-edit-map: `C-l`<br>multi-query-replace-map: `C-l`<br>query-replace-map: `C-l` |  |  |
| `recursive-edit` |  |  |  |
| `redirect-debugging-output` |  |  |  |
| `redraw-display` |  |  |  |
| `rename-buffer` | `C-x x r` |  |  |
| `rename-file` |  |  |  |
| `replace-buffer-contents` |  |  |  |
| `scroll-down` | kmacro-step-edit-map: `M-v`, `<prior>`<br>multi-query-replace-map: `M-v`, `<prior>`<br>query-replace-map: `M-v`, `<prior>` |  |  |
| `scroll-left` | `C-x <`, `C-<next>` |  |  |
| `scroll-right` | `C-x >`, `C-<prior>` |  |  |
| `scroll-up` | kmacro-step-edit-map: `C-v`, `<next>`<br>multi-query-replace-map: `C-v`, `<next>`<br>query-replace-map: `C-v`, `<next>` |  |  |
| `search-backward` |  |  |  |
| `search-backward-regexp` |  |  |  |
| `search-forward` | `<find>` |  |  |
| `search-forward-regexp` |  |  |  |
| `select-frame` |  |  |  |
| `self-insert-command` | 文字のキー (`a` など) |  |  |
| `set-file-modes` |  |  |  |
| `set-frame-height` |  |  |  |
| `set-frame-width` |  |  |  |
| `signal-process` |  |  |  |
| `start-kbd-macro` |  | 実装済み | `C-x (` |
| `suspend-emacs` |  |  |  |
| `top-level` |  |  |  |
| `transpose-regions` |  |  |  |
| `unix-sync` |  |  |  |
| `upcase-initials-region` |  |  |  |
| `upcase-region` | `C-x C-u` | 実装済み | `C-x C-u` |
| `upcase-word` | `M-u` | 実装済み | `M-u` |
| `widen` | `C-x n w` |  |  |
| `write-region` |  |  |  |
| `x-menu-bar-open-internal` |  |  |  |
| `x-print-frames-dialog` |  |  |  |

<!-- emacs-commands:end -->
