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

## Emacs のコマンドとキーの割り当て

Emacs 30.2 から, `tools/emacs-commands/run.sh` で作った一覧 (手で直さない。作り直すときは, `run.sh` を動かす)。

- **変更前 (デフォルト)**: `emacs -Q` で, 初期設定ファイルが読む機能 (ライブラリ) を, 設定をせずに読んだもの。
- **変更後 (カスタマイズ)**: `emacs --batch` で, 初期設定ファイル (`~/.emacs.el`) を, フォームごとに評価したもの (失敗したフォームは飛ばす)。
  `--batch` なので, 窓 (`window-system`) を使う設定と, 読めなかったパッケージの設定は, 入っていない (「batch で入らなかった global-map の設定」)。
- **rs-editor**: rs-editor にあるコマンド (✓) と, そのキー。`✓ (howm.scm)` は, `lisp/howm.scm` のコマンド。
- キーは Emacs の書き方 (`key-description`)。`C-c C-m` は `C-c RET`, `C-i` は `TAB` と書く。

### 1. キーの割り当ての変更 (変更前 → 変更後)

変更前と変更後で, 割り当てが違うキー (足した, 変えた, 消した)。プレフィックスキーにコマンドを割り当てて, 下のキーが無くなったものは, 1 行にまとめた。

#### global-map (135 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | 初期設定の行 | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|---|
| `<f10>` | `menu-bar-open` | `other-window-or-split` | 1440 |  |  |
| `<f12>` | (無し) | `recentf-open-files` | 2661 |  |  |
| `<f1>` | プレフィックス (help-map の 51 個のキー) | `<lambda>`: (manual-entry (current-word)) | 826 |  |  |
| `<f2>` | プレフィックス (2C-command の 4 個のキー) | `toggle-debug-on-error` | 1262 |  |  |
| `<f6>` | (無し) | `speedbar-get-focus` | 1958 |  |  |
| `<f9>` | (無し) | `desktop-recover` | 1346 |  |  |
| `<help> TAB` | (無し) | `info-lookup-symbol` |  |  |  |
| `<left>` | `left-char` | `backward-char` | 6532 |  | ✓ `C-b` |
| `<right>` | `right-char` | `forward-char` | 6531 |  | ✓ `C-f` |
| `C-;` | (無し) | `hippie-expand` | 1367 |  |  |
| `C-<insert>` | `kill-ring-save` | `clipboard-kill-ring-save` | 1406 |  |  |
| `C-<left>` | `left-word` | `backward-word` | 6536 |  | ✓ `M-b` |
| `C-<right>` | `right-word` | `forward-word` | 6535 |  | ✓ `M-f` |
| `C-S-k` | (無し) | `just-one-space` | 1424 |  | ✓ `M-SPC` |
| `C-S-t` | (無し) | `transpose-chars` | 1429 |  | ✓ `C-t` |
| `C-\` | `toggle-input-method` | (無し) | 3793 |  |  |
| `C-^` | (無し) | `global-whitespace-mode` | 1403 |  |  |
| `C-c ;` | (無し) | `comment-or-uncomment-region` | 1446 |  |  |
| `C-c C-f` | (無し) | `file-cache-choice` | 2729 |  |  |
| `C-c C-g` | (無し) | `<lambda>`: (message nil) | 1421 |  |  |
| `C-c C-l` | (無し) | `toggle-truncate-lines` | 1418 |  |  |
| `C-c C-s` | (無し) | `mark-string` | 2989 |  |  |
| `C-c C-w` | (無し) | `mark-word*` | 2988 |  |  |
| `C-c C-x` | (無し) | `recentf-open-files` | 2660 |  |  |
| `C-c RET` | (無し) | `mark-symbol` | 2987 |  |  |
| `C-c TAB` | (無し) | `info-apropos-region-or-word` | 779 |  |  |
| `C-c c` | (無し) | `compile` |  |  |  |
| `C-c d` | (無し) | `calendar-datetime-choice` | 2156 |  |  |
| `C-c f` | (無し) | `default-browser-or-vlc-choice` | 1339 |  |  |
| `C-c p` | (無し) | `pomodoro-choice` | 4098 |  |  |
| `C-c r` | (無し) | `bookmark-choice` | 1063 |  |  |
| `C-c t` | (無し) | `tabbar-mode` | 3529 |  |  |
| `C-c v` | (無し) | `vc-choice` | 4882 |  |  |
| `C-c x` | (無し) | `speedbar-get-focus` | 1957 |  |  |
| `C-h TAB` | (無し) | `info-lookup-symbol` | 780 |  |  |
| `C-t` | `transpose-chars` | (無し) | 1428 | `transpose-chars` |  |
| `C-x %` | (無し) | `<lambda>`: Go to the matching parenthesis if on parenthesis. | 1269 |  |  |
| `C-x 4 F` | (無し) | `find-function-other-window` |  |  |  |
| `C-x 4 K` | (無し) | `find-function-on-key-other-window` |  |  |  |
| `C-x 4 L` | (無し) | `find-library-other-window` |  |  |  |
| `C-x 4 V` | (無し) | `find-variable-other-window` |  |  |  |
| `C-x 5 F` | (無し) | `find-function-other-frame` |  |  |  |
| `C-x 5 K` | (無し) | `find-function-on-key-other-frame` |  |  |  |
| `C-x 5 L` | (無し) | `find-library-other-frame` |  |  |  |
| `C-x 5 V` | (無し) | `find-variable-other-frame` |  |  |  |
| `C-x ;` | `comment-set-column` | `hippie-expand` | 1368 |  |  |
| `C-x C-o` | `delete-blank-lines` | `occur-by-moccur` | 1123 | `delete-blank-lines` |  |
| `C-x F` | (無し) | `find-function` |  |  |  |
| `C-x K` | (無し) | `find-function-on-key` |  |  |  |
| `C-x L` | (無し) | `find-library` |  |  |  |
| `C-x M-d` | (無し) | `dired-at-point` | 1400 |  |  |
| `C-x M-f` | (無し) | `find-file-at-point` | 1399 |  |  |
| `C-x M-l` | (無し) | `find-file-at-point-goto-line` | 1398 |  |  |
| `C-x V` | (無し) | `find-variable` |  |  |  |
| `C-x j` | (無し) | `dired-jump-other-window` | 2676 |  |  |
| `M-0` | (無し) | `digit-argument` |  | `digit-argument` | ✓ `M-0`, `M-1`, `M-2` ほか 7 |
| `M-1` | (無し) | `toggle-which-func-mode` | 631 | `digit-argument` |  |
| `M-2` | (無し) | `se/make-summary-buffer` | 5230 | `digit-argument` |  |
| `M-3` | (無し) | `delete-mark-ring` | 1221 | `digit-argument` |  |
| `M-<f1>` | (無し) | `iman` | 2963 |  |  |
| `M-[ A` | (無し) | `previous-line` |  |  | ✓ `C-p` |
| `M-[ B` | (無し) | `next-line` |  |  | ✓ `C-n` |
| `M-[ C` | (無し) | `forward-char` |  |  | ✓ `C-f` |
| `M-[ D` | (無し) | `backward-char` |  |  | ✓ `C-b` |
| `M-g` | プレフィックス (goto-map の 9 個のキー) | `goto-line` | 1443 |  | ✓ `M-g g`, `M-g M-g` |
| `RET` | `newline` | `newline-and-indent` | 1382 | `newline` |  |
| `S-<f1>` | (無し) | `<lambda>`: (woman (current-word)) | 871 |  |  |
| `S-<f9>` | (無し) | `desktop-save*` | 1353 |  |  |
| `S-<insert>` | `yank` | `clipboard-yank` | 1412 |  |  |
| `S-DEL` | (無し) | `clipboard-kill-region` | 1409 |  |  |
| `TAB` | `indent-for-tab-command` | `self-insert-command` | 1432 | `indent-for-tab-command` |  |

#### emacs-lisp-mode-map (3 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `<f3>` | (無し) | `eval-current-buffer` |  |  |
| `C-c C-p` | (無し) | `mark-up-list` |  |  |
| `TAB` | (無し) | `lisp-complete-symbol` |  |  |

#### help-mode-map (2 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `M-<left>` | (無し) | `help-go-back` |  |  |
| `M-<right>` | (無し) | `help-go-forward` |  |  |

#### isearch-mode-map (2 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `M-O` | (無し) | `isearch-moccur-all` |  |  |
| `M-o` | (無し) | `isearch-moccur` |  |  |

#### lisp-interaction-mode-map (1 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `TAB` | (無し) | `lisp-complete-symbol` |  |  |

#### lisp-mode-map (1 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `TAB` | (無し) | `lisp-complete-symbol` |  |  |

#### minibuffer-local-completion-map (1 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `C-c C-c` | (無し) | `file-cache-minibuffer-complete` |  |  |

#### minibuffer-local-must-match-map (1 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `C-c C-c` | (無し) | `file-cache-minibuffer-complete` |  |  |

#### read--expression-map (1 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `TAB` | `completion-at-point` | `lisp-complete-symbol` |  |  |

#### read-expression-map (1 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `TAB` | `completion-at-point` | `lisp-complete-symbol` |  |  |

#### recentf-dialog-mode-map (2 個)

| キー | 変更前 (デフォルト) | 変更後 (カスタマイズ) | rs-editor の, そのキー | rs-editor に, 変更後のコマンド |
|---|---|---|---|---|
| `s` | (無し) | `recentf-sort-files` |  |  |
| `w` | (無し) | `recentf-edit-list` |  |  |

### 2. batch で入らなかった global-map の設定

初期設定ファイルの `(define-key global-map ...)` (同じキーを付け直したものは, 最後のもの) のうち, 変更後に入っていないもの (パッケージを読めなかった, 窓が要る, `(when ...)` の条件に合わなかった, など)。

| 行 | キー | 関数 | 変更後の, そのキー | rs-editor |
|---|---|---|---|---|
| 763 | `<f11>` | `resize-frame-interactively` | `toggle-frame-fullscreen` |  |
| 923 | `C-c m` | `w3m-gtk-lookup` |  |  |
| 1957 | `C-c x` | `sr-speedbar-toggle` | `speedbar-get-focus` |  |
| 1958 | `<f6>` | `sr-speedbar-toggle` | `speedbar-get-focus` |  |
| 2559 | `C-c o` | `org-choice` |  |  |
| 2675 | `C-<return>` | `cua-set-rectangle-mark` |  |  |
| 2676 | `C-x j` | `cua-set-rectangle-mark` | `dired-jump-other-window` |  |
| 3001 | `C-c C-j` | `direx:jump-to-directory-other-window` |  |  |
| 3126 | `C-?` | `redo` | `undo-redo` | ✓ `C-x C-/` |
| 3207 | `M-\` | `bm-toggle` | `delete-horizontal-space` |  |
| 3208 | `M-[` | `bm-previous` |  |  |
| 3209 | `M-]` | `bm-next` |  |  |
| 3238 | `<f7>` | `point-undo` |  |  |
| 3239 | `S-<f7>` | `point-redo` |  |  |
| 3244 | `<f8>` | `goto-last-change` |  |  |
| 3245 | `S-<f8>` | `goto-last-change-reverse` |  |  |
| 3417 | `C-c a` | `(lambda) (cond ((eq major-mode 'eshell-mode) (call-interactively 'any` |  |  |
| 3424 | `C-x a` | `anything-filelist+` |  |  |
| 3499 | `C-c s` | `(lambda) (if (eq major-mode 'eshell-mode) (helm-choice-for-eshell) (h` |  |  |
| 3517 | `C-c n` | `helm-mini` |  |  |
| 3623 | `C-c 2` | `navi2ch` |  |  |
| 3682 | `C-c h` | `howm-menu` |  | ✓ (howm.scm) |
| 3793 | `C-\` | `skk-mode` |  |  |
| 3990 | `C-x C-z` | `open-junk-file` | `suspend-frame` |  |
| 4231 | `C-c w` | `sdcv-search-input` |  |  |
| 4233 | `C-c i` | `sdcv-search-pointer+` |  |  |
| 4594 | `C-c 3` | `w3m-choice` |  |  |
| 4668 | `C-c e` | `evernote-choice` |  |  |
| 5142 | `C-<f3>` | `cscope-set-initial-directory` |  |  |
| 5143 | `C-<f4>` | `cscope-unset-initial-directory` |  |  |
| 5144 | `C-<f5>` | `cscope-find-this-symbol` |  |  |
| 5145 | `C-<f6>` | `cscope-find-global-definition` |  |  |
| 5146 | `C-<f7>` | `cscope-find-global-definition-no-prompting` |  |  |
| 5147 | `C-<f8>` | `cscope-pop-mark` |  |  |
| 5148 | `C-<f9>` | `cscope-history-forward-line` |  |  |
| 5149 | `C-<f10>` | `cscope-history-forward-file` | `buffer-menu-open` |  |
| 5150 | `C-<f11>` | `cscope-history-backward-line` |  |  |
| 5151 | `C-<f12>` | `cscope-history-backward-file` |  |  |
| 5152 | `M-<f9>` | `cscope-display-buffer` |  |  |
| 5153 | `M-<f10>` | `cscope-display-buffer-toggle` | `toggle-frame-maximized` |  |
| 5265 | `<f4>` | `toggle-auto-complete-mode` | `kmacro-end-or-call-macro` |  |
| 5410 | `<f5>` | `toggle-yas-minor-mode` |  |  |
| 5467 | `C-c C-h` | `hyperspec-lookup` |  |  |

### 3. カスタマイズで増えたコマンド (465 個)

`emacs -Q` には無く, 初期設定ファイルを読むと使えるコマンド。種類は, 初期設定ファイルで定義したもの, Emacs に付いているライブラリ, パッケージ (`~/.emacs.d` など)。

| コマンド | 種類 | ライブラリ | 変更後のキー | rs-editor |
|---|---|---|---|---|
| `annotate-mode` | パッケージ | annotate |  |  |
| `isearch-moccur` | パッケージ | color-moccur |  |  |
| `isearch-moccur-all` | パッケージ | color-moccur |  |  |
| `moccur-mode-edit-set-key` | パッケージ | color-moccur |  |  |
| `occur-by-moccur` | パッケージ | color-moccur | `C-x C-o` |  |
| `completing-help-mode` | パッケージ | completing-help |  |  |
| `turn-off-completing-help-mode` | パッケージ | completing-help |  |  |
| `turn-on-completing-help-mode` | パッケージ | completing-help |  |  |
| `fold-this` | パッケージ | fold-this-all |  |  |
| `hide/show-comments` | パッケージ | hide-comnt |  |  |
| `iman` | パッケージ | iman | `M-<f1>` |  |
| `align-repeat` | 初期設定 |  |  |  |
| `apt-get-install` | 初期設定 |  |  |  |
| `apt-get-install-all` | 初期設定 |  |  |  |
| `apt-get-update` | 初期設定 |  |  |  |
| `bookmark-choice` | 初期設定 |  | `C-c r` |  |
| `byte-compile-directory` | 初期設定 |  |  |  |
| `calendar-datetime-choice` | 初期設定 |  | `C-c d` |  |
| `calendar-info` | 初期設定 |  |  |  |
| `calendar-ja-info` | 初期設定 |  |  |  |
| `calendar-reference` | 初期設定 |  |  |  |
| `cedet-info` | 初期設定 |  |  |  |
| `clear-mark-ring` | 初期設定 |  |  |  |
| `copy-executable-files` | 初期設定 |  |  |  |
| `default-browser-google-search` | 初期設定 |  |  |  |
| `default-browser-or-vlc-choice` | 初期設定 |  | `C-c f` |  |
| `default-browser-url-at-point` | 初期設定 |  |  |  |
| `default-browser-wikipedia-search` | 初期設定 |  |  |  |
| `delete-mark-ring` | 初期設定 |  | `M-3` |  |
| `desktop-recover` | 初期設定 |  | `<f9>` |  |
| `desktop-save*` | 初期設定 |  | `S-<f9>` |  |
| `dired-info` | 初期設定 |  |  |  |
| `dired-ja-info` | 初期設定 |  |  |  |
| `el-get-install-all` | 初期設定 |  |  |  |
| `elisp-info` | 初期設定 |  |  |  |
| `elisp-ja-info` | 初期設定 |  |  |  |
| `emacs-info` | 初期設定 |  |  |  |
| `emacs-ja-info` | 初期設定 |  |  |  |
| `emacs-w3m-info` | 初期設定 |  |  |  |
| `emacs-w3m-ja-info` | 初期設定 |  |  |  |
| `enable-ajc-java-complete` | 初期設定 |  |  |  |
| `enable-malabar-mode` | 初期設定 |  |  |  |
| `eshell-chdir-up` | 初期設定 |  |  |  |
| `execute-indent` | 初期設定 |  |  |  |
| `file-cache-add-dir-from-file` | 初期設定 |  |  |  |
| `file-cache-choice` | 初期設定 |  | `C-c C-f` |  |
| `file-cache-recovery-from-file` | 初期設定 |  |  |  |
| `file-cache-save-to-file` | 初期設定 |  |  |  |
| `find-file-at-point-goto-line` | 初期設定 |  | `C-x M-l` |  |
| `find-file-for-sudo` | 初期設定 |  |  |  |
| `fold-this--this-or-all` | 初期設定 |  |  |  |
| `font-back` | 初期設定 |  |  |  |
| `font-to-monapo` | 初期設定 |  |  |  |
| `gauche-refe-info` | 初期設定 |  |  |  |
| `gauche-refj-info` | 初期設定 |  |  |  |
| `global-display-line-numbers-except-mode` | 初期設定 |  |  |  |
| `gtd` | 初期設定 |  |  |  |
| `gtd-home` | 初期設定 |  |  |  |
| `guile-2.0-info` | 初期設定 |  |  |  |
| `indent-all` | 初期設定 |  |  |  |
| `indent-for-vc-state` | 初期設定 |  |  |  |
| `info-apropos-region-or-word` | 初期設定 |  | `C-c TAB` |  |
| `insert-date` | 初期設定 |  |  |  |
| `insert-date-time` | 初期設定 |  |  |  |
| `install-el-get` | 初期設定 |  |  |  |
| `kill-all-buffer` | 初期設定 |  |  |  |
| `kill-org-all-buffer` | 初期設定 |  |  |  |
| `kill-org-buffer` | 初期設定 |  |  |  |
| `libc-info` | 初期設定 |  |  |  |
| `make-ctags-exuberant` | 初期設定 |  |  |  |
| `make-etags` | 初期設定 |  |  |  |
| `make-gtags` | 初期設定 |  |  |  |
| `mew.jis-info` | 初期設定 |  |  |  |
| `navi2ch-reference` | 初期設定 |  |  |  |
| `org-dired` | 初期設定 |  |  |  |
| `org-find-file-feed` | 初期設定 |  |  |  |
| `org-ja-info` | 初期設定 |  |  |  |
| `org-reference` | 初期設定 |  |  |  |
| `other-window-or-split` | 初期設定 |  | `<f10>` |  |
| `package-install-all` | 初期設定 |  |  |  |
| `perl-ja-info` | 初期設定 |  |  |  |
| `pomodoro-choice` | 初期設定 |  | `C-c p` |  |
| `print-direction-word` | 初期設定 |  |  |  |
| `print-files` | 初期設定 |  |  |  |
| `print-load-file` | 初期設定 |  |  |  |
| `print-require-benchmark` | 初期設定 |  |  |  |
| `python-info` | 初期設定 |  |  |  |
| `recentf-sort-files` | 初期設定 |  |  |  |
| `semantic-info` | 初期設定 |  |  |  |
| `semantic-user-info` | 初期設定 |  |  |  |
| `sync-skkdic` | 初期設定 |  |  |  |
| `tcl-info` | 初期設定 |  |  |  |
| `toggle-which-func-mode` | 初期設定 |  | `M-1` |  |
| `tramp_ja-info` | 初期設定 |  |  |  |
| `update-el-get` | 初期設定 |  |  |  |
| `update-emacswiki` | 初期設定 |  |  |  |
| `vc-choice` | 初期設定 |  | `C-c v` |  |
| `vlc-check-directory` | 初期設定 |  |  |  |
| `vlc-check-location` | 初期設定 |  |  |  |
| `vlc-url-at-point` | 初期設定 |  |  |  |
| `vlc-xml2csv-file` | 初期設定 |  |  |  |
| `zsh-info` | 初期設定 |  |  |  |
| `moccur-edit-mode-in` | パッケージ | moccur-edit |  |  |
| `pomodoro-pause` | パッケージ | pomodoro-technique |  |  |
| `pomodoro-reset` | パッケージ | pomodoro-technique |  |  |
| `pomodoro-restart` | パッケージ | pomodoro-technique |  |  |
| `pomodoro-save` | パッケージ | pomodoro-technique |  |  |
| `pomodoro-start` | パッケージ | pomodoro-technique |  |  |
| `pomodoro-stop` | パッケージ | pomodoro-technique |  |  |
| `session-initialize` | パッケージ | session |  |  |
| `session-jump-to-last-change` | パッケージ | session |  |  |
| `se/make-summary-buffer` | パッケージ | summarye | `M-2` |  |
| `tabbar-mode` | パッケージ | tabbar | `C-c t` |  |
| `turn-on-tempbuf-mode` | パッケージ | tempbuf |  |  |
| `backward-defun` | パッケージ | thingopt |  |  |
| `backward-string` | パッケージ | thingopt |  |  |
| `copy-thing` | パッケージ | thingopt |  |  |
| `forward-defun` | パッケージ | thingopt |  |  |
| `forward-string` | パッケージ | thingopt |  |  |
| `kill-region-dwim` | パッケージ | thingopt |  |  |
| `kill-ring-save-dwim` | パッケージ | thingopt |  |  |
| `kill-thing` | パッケージ | thingopt |  |  |
| `mark-thing` | パッケージ | thingopt |  |  |
| `upward-bounds-of-thing` | パッケージ | thingopt |  |  |
| `upward-isearch-thing` | パッケージ | thingopt |  |  |
| `upward-mark-thing` | パッケージ | thingopt |  |  |
| `copy-buffer` | 初期設定 |  |  |  |
| `copy-button` | 初期設定 |  |  |  |
| `copy-char` | 初期設定 |  |  |  |
| `copy-comment` | 初期設定 |  |  |  |
| `copy-defun` | 初期設定 |  |  |  |
| `copy-email` | 初期設定 |  |  |  |
| `copy-existing-filename` | 初期設定 |  |  |  |
| `copy-filename` | 初期設定 |  |  |  |
| `copy-line` | 初期設定 |  |  |  |
| `copy-list` | 初期設定 |  |  |  |
| `copy-number` | 初期設定 |  |  |  |
| `copy-page` | 初期設定 |  |  |  |
| `copy-paragraph` | 初期設定 |  |  |  |
| `copy-region` | 初期設定 |  |  |  |
| `copy-sentence` | 初期設定 |  |  |  |
| `copy-sexp` | 初期設定 |  |  |  |
| `copy-string` | 初期設定 |  |  |  |
| `copy-symbol` | 初期設定 |  |  |  |
| `copy-thing*` | 初期設定 |  |  |  |
| `copy-up-list` | 初期設定 |  |  |  |
| `copy-url` | 初期設定 |  |  |  |
| `copy-uuid` | 初期設定 |  |  |  |
| `copy-whitespace` | 初期設定 |  |  |  |
| `copy-word` | 初期設定 |  |  |  |
| `kill-buffer*` | 初期設定 |  |  |  |
| `kill-button` | 初期設定 |  |  |  |
| `kill-char` | 初期設定 |  |  |  |
| `kill-comment*` | 初期設定 |  |  |  |
| `kill-defun` | 初期設定 |  |  |  |
| `kill-email` | 初期設定 |  |  |  |
| `kill-existing-filename` | 初期設定 |  |  |  |
| `kill-filename` | 初期設定 |  |  |  |
| `kill-line*` | 初期設定 |  |  |  |
| `kill-list` | 初期設定 |  |  |  |
| `kill-number` | 初期設定 |  |  |  |
| `kill-page` | 初期設定 |  |  |  |
| `kill-paragraph*` | 初期設定 |  |  |  |
| `kill-region*` | 初期設定 |  |  |  |
| `kill-sentence*` | 初期設定 |  |  |  |
| `kill-sexp*` | 初期設定 |  |  |  |
| `kill-string` | 初期設定 |  |  |  |
| `kill-symbol` | 初期設定 |  |  |  |
| `kill-thing*` | 初期設定 |  |  |  |
| `kill-up-list` | 初期設定 |  |  |  |
| `kill-url` | 初期設定 |  |  |  |
| `kill-uuid` | 初期設定 |  |  |  |
| `kill-whitespace` | 初期設定 |  |  |  |
| `kill-word*` | 初期設定 |  |  |  |
| `mark-buffer` | 初期設定 |  |  |  |
| `mark-button` | 初期設定 |  |  |  |
| `mark-char` | 初期設定 |  |  |  |
| `mark-comment` | 初期設定 |  |  |  |
| `mark-defun*` | 初期設定 |  |  |  |
| `mark-email` | 初期設定 |  |  |  |
| `mark-existing-filename` | 初期設定 |  |  |  |
| `mark-filename` | 初期設定 |  |  |  |
| `mark-line` | 初期設定 |  |  |  |
| `mark-list` | 初期設定 |  |  |  |
| `mark-number` | 初期設定 |  |  |  |
| `mark-page*` | 初期設定 |  |  |  |
| `mark-paragraph*` | 初期設定 |  |  |  |
| `mark-region` | 初期設定 |  |  |  |
| `mark-sentence` | 初期設定 |  |  |  |
| `mark-sexp*` | 初期設定 |  |  |  |
| `mark-string` | 初期設定 |  | `C-c C-s` |  |
| `mark-symbol` | 初期設定 |  | `C-c RET` |  |
| `mark-thing*` | 初期設定 |  |  |  |
| `mark-up-list` | 初期設定 |  |  |  |
| `mark-url` | 初期設定 |  |  |  |
| `mark-uuid` | 初期設定 |  |  |  |
| `mark-whitespace` | 初期設定 |  |  |  |
| `mark-word*` | 初期設定 |  | `C-c C-w` |  |
| `widget-beginning-of-line` | 組み込み | (C) |  |  |
| `auth-source-forget-all-cached` | 組み込み | auth-source |  |  |
| `read-passwd-mode` | 組み込み | auth-source |  |  |
| `read-passwd-toggle-visibility` | 組み込み | auth-source |  |  |
| `auto-revert-set-timer` | 組み込み | autorevert |  |  |
| `browse-url-button-copy` | 組み込み | browse-url |  |  |
| `browse-url-chrome` | 組み込み | browse-url |  |  |
| `browse-url-default-macosx-browser` | 組み込み | browse-url |  |  |
| `browse-url-default-windows-browser` | 組み込み | browse-url |  |  |
| `browse-url-epiphany` | 組み込み | browse-url |  |  |
| `browse-url-man` | 組み込み | browse-url |  |  |
| `byte-recompile-file` | 組み込み | bytecomp |  |  |
| `emacs-lisp-compilation-mode` | 組み込み | bytecomp |  |  |
| `emacs-lisp-compilation-recompile` | 組み込み | bytecomp |  |  |
| `cl-describe-type` | 組み込み | cl-extra |  |  |
| `native-compile-prune-cache` | 組み込み | comp |  |  |
| `native-comp-limple-mode` | 組み込み | comp-common |  |  |
| `Custom-buffer-done` | 組み込み | cus-edit |  |  |
| `Custom-dirlocals-menu` | 組み込み | cus-edit |  |  |
| `Custom-dirlocals-revert-buffer` | 組み込み | cus-edit |  |  |
| `Custom-dirlocals-save` | 組み込み | cus-edit |  |  |
| `Custom-goto-parent` | 組み込み | cus-edit |  |  |
| `Custom-help` | 組み込み | cus-edit |  |  |
| `Custom-mode` | 組み込み | cus-edit |  |  |
| `Custom-mode-menu` | 組み込み | cus-edit |  |  |
| `Custom-newline` | 組み込み | cus-edit |  |  |
| `Custom-no-edit` | 組み込み | cus-edit |  |  |
| `Custom-reset-current` | 組み込み | cus-edit |  |  |
| `Custom-reset-saved` | 組み込み | cus-edit |  |  |
| `Custom-reset-standard` | 組み込み | cus-edit |  |  |
| `Custom-save` | 組み込み | cus-edit |  |  |
| `Custom-set` | 組み込み | cus-edit |  |  |
| `custom-toggle-hide-all-widgets` | 組み込み | cus-edit |  |  |
| `customize-customized` | 組み込み | cus-edit |  |  |
| `diff-add-change-log-entries-other-window` | 組み込み | diff-mode |  |  |
| `diff-apply-buffer` | 組み込み | diff-mode |  |  |
| `diff-apply-hunk` | 組み込み | diff-mode |  |  |
| `diff-auto-refine-mode` | 組み込み | diff-mode |  |  |
| `diff-context->unified` | 組み込み | diff-mode |  |  |
| `diff-delete-trailing-whitespace` | 組み込み | diff-mode |  |  |
| `diff-ediff-patch` | 組み込み | diff-mode |  |  |
| `diff-file-kill` | 組み込み | diff-mode |  |  |
| `diff-file-next` | 組み込み | diff-mode |  |  |
| `diff-file-prev` | 組み込み | diff-mode |  |  |
| `diff-fixup-modifs` | 組み込み | diff-mode |  |  |
| `diff-goto-source` | 組み込み | diff-mode |  |  |
| `diff-hunk-kill` | 組み込み | diff-mode |  |  |
| `diff-hunk-next` | 組み込み | diff-mode |  |  |
| `diff-hunk-prev` | 組み込み | diff-mode |  |  |
| `diff-ignore-whitespace-hunk` | 組み込み | diff-mode |  |  |
| `diff-kill-applied-hunks` | 組み込み | diff-mode |  |  |
| `diff-kill-junk` | 組み込み | diff-mode |  |  |
| `diff-mode-menu` | 組み込み | diff-mode |  |  |
| `diff-mouse-goto-source` | 組み込み | diff-mode |  |  |
| `diff-next-complex-hunk` | 組み込み | diff-mode |  |  |
| `diff-refine-hunk` | 組み込み | diff-mode |  |  |
| `diff-refresh-hunk` | 組み込み | diff-mode |  |  |
| `diff-restrict-view` | 組み込み | diff-mode |  |  |
| `diff-reverse-direction` | 組み込み | diff-mode |  |  |
| `diff-split-hunk` | 組み込み | diff-mode |  |  |
| `diff-tell-file-name` | 組み込み | diff-mode |  |  |
| `diff-test-hunk` | 組み込み | diff-mode |  |  |
| `diff-undo` | 組み込み | diff-mode |  |  |
| `diff-unified->context` | 組み込み | diff-mode |  |  |
| `diff-wiggle` | 組み込み | diff-mode |  |  |
| `eieio-version` | 組み込み | eieio |  |  |
| `file-cache-clear-cache` | 組み込み | filecache |  |  |
| `file-notify-rm-all-watches` | 組み込み | filenotify |  |  |
| `flyspell-word` | 組み込み | flyspell |  |  |
| `help-customize` | 組み込み | help-mode |  |  |
| `help-follow` | 組み込み | help-mode |  |  |
| `help-follow-mouse` | 組み込み | help-mode |  |  |
| `help-follow-symbol` | 組み込み | help-mode |  |  |
| `help-go-back` | 組み込み | help-mode |  |  |
| `help-go-forward` | 組み込み | help-mode |  |  |
| `help-goto-info` | 組み込み | help-mode |  |  |
| `help-goto-lispref-info` | 組み込み | help-mode |  |  |
| `help-goto-next-page` | 組み込み | help-mode |  |  |
| `help-goto-previous-page` | 組み込み | help-mode |  |  |
| `help-mode-menu` | 組み込み | help-mode |  |  |
| `help-view-source` | 組み込み | help-mode |  |  |
| `hide-ifdefs` | 組み込み | hideif |  |  |
| `icomplete-backward-completions` | 組み込み | icomplete |  |  |
| `icomplete-fido-backward-updir` | 組み込み | icomplete |  |  |
| `icomplete-fido-delete-char` | 組み込み | icomplete |  |  |
| `icomplete-fido-exit` | 組み込み | icomplete |  |  |
| `icomplete-fido-kill` | 組み込み | icomplete |  |  |
| `icomplete-fido-ret` | 組み込み | icomplete |  |  |
| `icomplete-force-complete` | 組み込み | icomplete |  |  |
| `icomplete-force-complete-and-exit` | 組み込み | icomplete |  |  |
| `icomplete-forward-completions` | 組み込み | icomplete |  |  |
| `icomplete-ret` | 組み込み | icomplete |  |  |
| `icomplete-vertical-goto-first` | 組み込み | icomplete |  |  |
| `icomplete-vertical-goto-last` | 組み込み | icomplete |  |  |
| `ido-bury-buffer-at-head` | 組み込み | ido |  |  |
| `ido-complete` | 組み込み | ido |  |  |
| `ido-complete-space` | 組み込み | ido |  |  |
| `ido-completion-auto-help` | 組み込み | ido |  |  |
| `ido-completion-help` | 組み込み | ido |  |  |
| `ido-copy-current-file-name` | 組み込み | ido |  |  |
| `ido-copy-current-word` | 組み込み | ido |  |  |
| `ido-delete-backward-updir` | 組み込み | ido |  |  |
| `ido-delete-backward-word-updir` | 組み込み | ido |  |  |
| `ido-delete-file-at-head` | 組み込み | ido |  |  |
| `ido-edit-input` | 組み込み | ido |  |  |
| `ido-enter-dired` | 組み込み | ido |  |  |
| `ido-enter-find-file` | 組み込み | ido |  |  |
| `ido-enter-insert-buffer` | 組み込み | ido |  |  |
| `ido-enter-insert-file` | 組み込み | ido |  |  |
| `ido-enter-switch-buffer` | 組み込み | ido |  |  |
| `ido-everywhere` | 組み込み | ido |  |  |
| `ido-exit-minibuffer` | 組み込み | ido |  |  |
| `ido-fallback-command` | 組み込み | ido |  |  |
| `ido-forget-work-directory` | 組み込み | ido |  |  |
| `ido-init-completion-maps` | 組み込み | ido |  |  |
| `ido-kill-buffer-at-head` | 組み込み | ido |  |  |
| `ido-list-directory` | 組み込み | ido |  |  |
| `ido-load-history` | 組み込み | ido |  |  |
| `ido-magic-backward-char` | 組み込み | ido |  |  |
| `ido-magic-delete-char` | 組み込み | ido |  |  |
| `ido-magic-forward-char` | 組み込み | ido |  |  |
| `ido-make-directory` | 組み込み | ido |  |  |
| `ido-merge-work-directories` | 組み込み | ido |  |  |
| `ido-next-match` | 組み込み | ido |  |  |
| `ido-next-match-dir` | 組み込み | ido |  |  |
| `ido-next-work-directory` | 組み込み | ido |  |  |
| `ido-next-work-file` | 組み込み | ido |  |  |
| `ido-pop-dir` | 組み込み | ido |  |  |
| `ido-prev-match` | 組み込み | ido |  |  |
| `ido-prev-match-dir` | 組み込み | ido |  |  |
| `ido-prev-work-directory` | 組み込み | ido |  |  |
| `ido-prev-work-file` | 組み込み | ido |  |  |
| `ido-push-dir` | 組み込み | ido |  |  |
| `ido-push-dir-first` | 組み込み | ido |  |  |
| `ido-reread-directory` | 組み込み | ido |  |  |
| `ido-restrict-to-matches` | 組み込み | ido |  |  |
| `ido-save-history` | 組み込み | ido |  |  |
| `ido-select-text` | 組み込み | ido |  |  |
| `ido-switch-to-completions` | 組み込み | ido |  |  |
| `ido-take-first-match` | 組み込み | ido |  |  |
| `ido-toggle-case` | 組み込み | ido |  |  |
| `ido-toggle-ignore` | 組み込み | ido |  |  |
| `ido-toggle-literal` | 組み込み | ido |  |  |
| `ido-toggle-prefix` | 組み込み | ido |  |  |
| `ido-toggle-regexp` | 組み込み | ido |  |  |
| `ido-toggle-trace` | 組み込み | ido |  |  |
| `ido-toggle-vc` | 組み込み | ido |  |  |
| `ido-toggle-virtual-buffers` | 組み込み | ido |  |  |
| `ido-undo-merge-work-directory` | 組み込み | ido |  |  |
| `ido-up-directory` | 組み込み | ido |  |  |
| `ido-wash-history` | 組み込み | ido |  |  |
| `ido-wide-find-dir` | 組み込み | ido |  |  |
| `ido-wide-find-dir-or-delete-dir` | 組み込み | ido |  |  |
| `ido-wide-find-file` | 組み込み | ido |  |  |
| `ido-wide-find-file-or-pop-dir` | 組み込み | ido |  |  |
| `json-pretty-print` | 組み込み | json |  |  |
| `json-pretty-print-buffer` | 組み込み | json |  |  |
| `json-pretty-print-buffer-ordered` | 組み込み | json |  |  |
| `json-pretty-print-ordered` | 組み込み | json |  |  |
| `mailcap-parse-mailcaps` | 組み込み | mailcap |  |  |
| `mailcap-parse-mimetypes` | 組み込み | mailcap |  |  |
| `mailcap-view-file` | 組み込み | mailcap |  |  |
| `org-mobile-pull` | 組み込み | org-mobile |  |  |
| `org-mobile-push` | 組み込み | org-mobile |  |  |
| `package-browse-url` | 組み込み | package |  |  |
| `package-delete` | 組み込み | package |  |  |
| `package-isolate` | 組み込み | package |  |  |
| `package-list-packages-no-fetch` | 組み込み | package |  |  |
| `package-menu-backup-unmark` | 組み込み | package |  |  |
| `package-menu-clear-filter` | 組み込み | package |  |  |
| `package-menu-describe-package` | 組み込み | package |  |  |
| `package-menu-execute` | 組み込み | package |  |  |
| `package-menu-filter` | 組み込み | package |  |  |
| `package-menu-filter-by-archive` | 組み込み | package |  |  |
| `package-menu-filter-by-description` | 組み込み | package |  |  |
| `package-menu-filter-by-keyword` | 組み込み | package |  |  |
| `package-menu-filter-by-name` | 組み込み | package |  |  |
| `package-menu-filter-by-name-or-description` | 組み込み | package |  |  |
| `package-menu-filter-by-status` | 組み込み | package |  |  |
| `package-menu-filter-by-version` | 組み込み | package |  |  |
| `package-menu-filter-marked` | 組み込み | package |  |  |
| `package-menu-filter-upgradable` | 組み込み | package |  |  |
| `package-menu-hide-package` | 組み込み | package |  |  |
| `package-menu-mark-delete` | 組み込み | package |  |  |
| `package-menu-mark-install` | 組み込み | package |  |  |
| `package-menu-mark-obsolete-for-deletion` | 組み込み | package |  |  |
| `package-menu-mark-unmark` | 組み込み | package |  |  |
| `package-menu-mark-upgrades` | 組み込み | package |  |  |
| `package-menu-mode-menu` | 組み込み | package |  |  |
| `package-menu-quick-help` | 組み込み | package |  |  |
| `package-menu-refresh` | 組み込み | package |  |  |
| `package-menu-toggle-hiding` | 組み込み | package |  |  |
| `package-quickstart-refresh` | 組み込み | package |  |  |
| `package-show-package-list` | 組み込み | package |  |  |
| `password-reset` | 組み込み | password-cache |  |  |
| `pp-28` | 組み込み | pp |  |  |
| `pp-fill` | 組み込み | pp |  |  |
| `recentf-cancel-dialog` | 組み込み | recentf |  |  |
| `recentf-cleanup` | 組み込み | recentf |  |  |
| `recentf-edit-list` | 組み込み | recentf |  |  |
| `recentf-edit-list-validate` | 組み込み | recentf |  |  |
| `recentf-load-list` | 組み込み | recentf |  |  |
| `recentf-open-files` | 組み込み | recentf | `<f12>`, `C-c C-x` |  |
| `recentf-open-more-files` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-0` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-1` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-2` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-3` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-4` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-5` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-6` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-7` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-8` | 組み込み | recentf |  |  |
| `recentf-open-most-recent-file-9` | 組み込み | recentf |  |  |
| `recentf-save-list` | 組み込み | recentf |  |  |
| `savehist-save` | 組み込み | savehist |  |  |
| `save-place-forget-unreadable-files` | 組み込み | saveplace |  |  |
| `toggle-save-place` | 組み込み | saveplace |  |  |
| `server-edit` | 組み込み | server | `C-x #` |  |
| `server-edit-abort` | 組み込み | server |  |  |
| `server-generate-key` | 組み込み | server |  |  |
| `global-srecode-minor-mode` | 組み込み | srecode |  |  |
| `display-time-next-load-average` | 組み込み | time |  |  |
| `world-clock-copy-time-as-kill` | 組み込み | time |  |  |
| `tree-widget-button-click` | 組み込み | tree-widget |  |  |
| `url-cookie-delete` | 組み込み | url-cookie |  |  |
| `url-cookie-list` | 組み込み | url-cookie |  |  |
| `url-cookie-mode` | 組み込み | url-cookie |  |  |
| `url-cookie-parse-file-netscape` | 組み込み | url-cookie |  |  |
| `url-cookie-setup-save-timer` | 組み込み | url-cookie |  |  |
| `url-cookie-undo` | 組み込み | url-cookie |  |  |
| `url-history-save-history` | 組み込み | url-history |  |  |
| `url-history-setup-save-timer` | 組み込み | url-history |  |  |
| `vc-finish-logentry` | 組み込み | vc-dispatcher |  |  |
| `vc-revert-buffer-internal` | 組み込み | vc-dispatcher |  |  |
| `vc-dir-git-mode` | 組み込み | vc-git |  |  |
| `vc-git-log-edit-mode` | 組み込み | vc-git |  |  |
| `vc-git-log-edit-toggle-amend` | 組み込み | vc-git |  |  |
| `vc-git-log-edit-toggle-no-verify` | 組み込み | vc-git |  |  |
| `vc-git-log-edit-toggle-signoff` | 組み込み | vc-git |  |  |
| `vc-git-log-view-mode` | 組み込み | vc-git |  |  |
| `vc-git-region-history-mode` | 組み込み | vc-git |  |  |
| `vc-git-stash` | 組み込み | vc-git |  |  |
| `vc-git-stash-apply` | 組み込み | vc-git |  |  |
| `vc-git-stash-apply-at-point` | 組み込み | vc-git |  |  |
| `vc-git-stash-delete` | 組み込み | vc-git |  |  |
| `vc-git-stash-delete-at-point` | 組み込み | vc-git |  |  |
| `vc-git-stash-menu` | 組み込み | vc-git |  |  |
| `vc-git-stash-pop` | 組み込み | vc-git |  |  |
| `vc-git-stash-pop-at-point` | 組み込み | vc-git |  |  |
| `vc-git-stash-show` | 組み込み | vc-git |  |  |
| `vc-git-stash-show-at-point` | 組み込み | vc-git |  |  |
| `vc-git-stash-snapshot` | 組み込み | vc-git |  |  |
| `advertised-widget-backward` | 組み込み | wid-edit |  |  |
| `widget-backward` | 組み込み | wid-edit |  |  |
| `widget-button-click` | 組み込み | wid-edit |  |  |
| `widget-button-press` | 組み込み | wid-edit |  |  |
| `widget-complete` | 組み込み | wid-edit |  |  |
| `widget-describe` | 組み込み | wid-edit |  |  |
| `widget-end-of-line` | 組み込み | wid-edit |  |  |
| `widget-field-activate` | 組み込み | wid-edit |  |  |
| `widget-forward` | 組み込み | wid-edit |  |  |
| `widget-key-sequence-read-event` | 組み込み | wid-edit |  |  |
| `widget-kill-line` | 組み込み | wid-edit |  |  |
| `widget-move-and-invoke` | 組み込み | wid-edit |  |  |
| `widget-narrow-to-field` | 組み込み | wid-edit |  |  |

### 4. Emacs の標準のコマンド (3051 個. rs-editor にあるもの 115 個)

`emacs -Q` で `M-x` で使える全てのコマンド (読んであるものと, autoload)。ライブラリの順。変更後のキーは, 変更前と同じなら「同じ」と書く。キーは global-map のもの (モードのキーマップは, 1. を見る)。

| コマンド | ライブラリ | 変更前のキー | 変更後のキー | rs-editor |
|---|---|---|---|---|
| `abort-minibuffers` | (C) |  |  |  |
| `abort-recursive-edit` | (C) | `C-]` | 同じ |  |
| `accelerate-menu` | (C) |  |  |  |
| `add-name-to-file` | (C) |  |  |  |
| `backward-char` | (C) | `C-b` | `C-b`, `M-[ D`, `<left>` | ✓ `C-b` |
| `base64-decode-region` | (C) |  |  |  |
| `base64-encode-region` | (C) |  |  |  |
| `base64url-encode-region` | (C) |  |  |  |
| `beginning-of-line` | (C) |  |  |  |
| `buffer-enable-undo` | (C) |  |  |  |
| `call-last-kbd-macro` | (C) |  |  | ✓ `C-x e` |
| `capitalize-region` | (C) |  |  |  |
| `capitalize-word` | (C) | `M-c` | 同じ | ✓ `M-c` |
| `chmod` | (C) |  |  |  |
| `copy-file` | (C) |  |  |  |
| `decode-coding-region` | (C) |  |  |  |
| `defining-kbd-macro` | (C) |  |  |  |
| `delete-char` | (C) | `C-d` | 同じ | ✓ `C-d` |
| `delete-frame` | (C) | `C-x 5 0` | 同じ |  |
| `delete-other-windows-internal` | (C) |  |  |  |
| `delete-process` | (C) |  |  |  |
| `delete-region` | (C) |  |  |  |
| `do-auto-save` | (C) |  |  |  |
| `downcase-region` | (C) | `C-x C-l` | 同じ | ✓ `C-x C-l` |
| `downcase-word` | (C) | `M-l` | 同じ | ✓ `M-l` |
| `encode-coding-region` | (C) |  |  |  |
| `end-kbd-macro` | (C) |  |  | ✓ `C-x )` |
| `end-of-line` | (C) |  |  |  |
| `erase-buffer` | (C) |  |  |  |
| `eval-buffer` | (C) |  |  |  |
| `eval-region` | (C) |  |  |  |
| `exit-recursive-edit` | (C) | `C-M-c` | 同じ |  |
| `forward-char` | (C) | `C-f` | `C-f`, `M-[ C`, `<right>` | ✓ `C-f` |
| `forward-line` | (C) |  |  |  |
| `forward-word` | (C) | `M-f`, `ESC <right>` | `M-f`, `C-<right>`, `ESC <right>` | ✓ `M-f` |
| `garbage-collect` | (C) |  |  |  |
| `goto-char` | (C) | `M-g c` |  |  |
| `handle-save-session` | (C) |  |  |  |
| `handle-switch-frame` | (C) | `<switch-frame>` | 同じ |  |
| `iconify-frame` | (C) |  |  |  |
| `indent-to` | (C) |  |  |  |
| `indent-to-column` | (C) |  |  |  |
| `insert-char` | (C) | `C-x 8 RET` | 同じ |  |
| `kill-buffer` | (C) | `C-x k` | 同じ | ✓ `C-x k` |
| `kill-emacs` | (C) |  |  |  |
| `kill-local-variable` | (C) |  |  |  |
| `kill-process` | (C) |  |  |  |
| `lossage-size` | (C) |  |  |  |
| `lower-frame` | (C) |  |  |  |
| `make-frame-invisible` | (C) |  |  |  |
| `make-frame-visible` | (C) |  |  |  |
| `make-indirect-buffer` | (C) |  |  |  |
| `make-local-variable` | (C) |  |  |  |
| `make-symbolic-link` | (C) |  |  |  |
| `make-variable-buffer-local` | (C) |  |  |  |
| `malloc-info` | (C) |  |  |  |
| `malloc-trim` | (C) |  |  |  |
| `modify-syntax-entry` | (C) |  |  |  |
| `move-to-column` | (C) | `M-g TAB` |  |  |
| `move-to-window-line` | (C) |  |  |  |
| `narrow-to-region` | (C) | `C-x n n` | 同じ |  |
| `open-dribble-file` | (C) |  |  |  |
| `open-termscript` | (C) |  |  |  |
| `posix-search-backward` | (C) |  |  |  |
| `posix-search-forward` | (C) |  |  |  |
| `raise-frame` | (C) |  |  |  |
| `re-search-backward` | (C) |  |  |  |
| `re-search-forward` | (C) |  |  |  |
| `recenter` | (C) |  |  |  |
| `recursive-edit` | (C) |  |  |  |
| `redirect-debugging-output` | (C) |  |  |  |
| `redraw-display` | (C) |  |  |  |
| `rename-buffer` | (C) | `C-x x r` | 同じ |  |
| `rename-file` | (C) |  |  |  |
| `replace-buffer-contents` | (C) |  |  |  |
| `scroll-down` | (C) |  |  |  |
| `scroll-left` | (C) | `C-x <`, `C-<next>` | 同じ |  |
| `scroll-right` | (C) | `C-x >`, `C-<prior>` | 同じ |  |
| `scroll-up` | (C) |  |  |  |
| `search-backward` | (C) |  |  |  |
| `search-backward-regexp` | (C) |  |  |  |
| `search-forward` | (C) | `<find>` | 同じ |  |
| `search-forward-regexp` | (C) |  |  |  |
| `select-frame` | (C) |  |  |  |
| `self-insert-command` | (C) |  | `TAB` |  |
| `set-file-modes` | (C) |  |  |  |
| `set-frame-height` | (C) |  |  |  |
| `set-frame-width` | (C) |  |  |  |
| `signal-process` | (C) |  |  |  |
| `start-kbd-macro` | (C) |  |  | ✓ `C-x (` |
| `suspend-emacs` | (C) |  |  |  |
| `top-level` | (C) |  |  |  |
| `transpose-regions` | (C) |  |  |  |
| `unix-sync` | (C) |  |  |  |
| `upcase-initials-region` | (C) |  |  |  |
| `upcase-region` | (C) | `C-x C-u` | 同じ | ✓ `C-x C-u` |
| `upcase-word` | (C) | `M-u` | 同じ | ✓ `M-u` |
| `widen` | (C) | `C-x n w` | 同じ |  |
| `write-region` | (C) |  |  |  |
| `x-menu-bar-open-internal` | (C) |  |  |  |
| `x-print-frames-dialog` | (C) |  |  |  |
| `5x5` | 5x5 |  |  |  |
| `5x5-crack` | 5x5 |  |  |  |
| `5x5-crack-mutating-best` | 5x5 |  |  |  |
| `5x5-crack-mutating-current` | 5x5 |  |  |  |
| `5x5-crack-randomly` | 5x5 |  |  |  |
| `5x5-crack-xor-mutate` | 5x5 |  |  |  |
| `abbrev-edit-save-buffer` | abbrev |  |  |  |
| `abbrev-edit-save-to-file` | abbrev |  |  |  |
| `abbrev-mode` | abbrev |  |  |  |
| `abbrev-prefix-mark` | abbrev | `M-'` | 同じ |  |
| `abbrev-suggest-show-report` | abbrev |  |  |  |
| `add-global-abbrev` | abbrev | `C-x a g` | 同じ |  |
| `add-mode-abbrev` | abbrev | `C-x a +`, `C-x a l`, `C-x a C-a` | 同じ |  |
| `define-abbrevs` | abbrev |  |  |  |
| `define-global-abbrev` | abbrev |  |  |  |
| `define-mode-abbrev` | abbrev |  |  |  |
| `edit-abbrevs` | abbrev |  |  |  |
| `edit-abbrevs-redefine` | abbrev |  |  |  |
| `expand-abbrev` | abbrev | `C-x '`, `C-x a '`, `C-x a e` | 同じ |  |
| `expand-region-abbrevs` | abbrev |  |  |  |
| `insert-abbrevs` | abbrev |  |  |  |
| `inverse-add-global-abbrev` | abbrev | `C-x a -`, `C-x a i g` | 同じ |  |
| `inverse-add-mode-abbrev` | abbrev | `C-x a i l` | 同じ |  |
| `kill-all-abbrevs` | abbrev |  |  |  |
| `list-abbrevs` | abbrev |  |  |  |
| `read-abbrev-file` | abbrev |  |  |  |
| `unexpand-abbrev` | abbrev |  |  |  |
| `write-abbrev-file` | abbrev |  |  |  |
| `add-change-log-entry` | add-log |  |  |  |
| `add-change-log-entry-other-window` | add-log | `C-x 4 a` | 同じ |  |
| `change-log-merge` | add-log |  |  |  |
| `change-log-mode` | add-log |  |  |  |
| `ad-activate` | advice |  |  |  |
| `ad-disable-advice` | advice |  |  |  |
| `ad-enable-advice` | advice |  |  |  |
| `align` | align |  |  |  |
| `align-current` | align |  |  |  |
| `align-entire` | align |  |  |  |
| `align-highlight-rule` | align |  |  |  |
| `align-newline-and-indent` | align |  |  |  |
| `align-regexp` | align |  |  |  |
| `align-unhighlight-rule` | align |  |  |  |
| `allout-mode` | allout |  |  |  |
| `allout-outlinify-sticky` | allout |  |  |  |
| `allout-widgets-mode` | allout-widgets |  |  |  |
| `ange-ftp-reread-dir` | ange-ftp |  |  |  |
| `animate-birthday-present` | animate |  |  |  |
| `ansi-color-for-comint-mode-on` | ansi-color |  |  |  |
| `antlr-mode` | antlr-mode |  |  |  |
| `antlr-show-makefile-rules` | antlr-mode |  |  |  |
| `appt-activate` | appt |  |  |  |
| `appt-add` | appt |  |  |  |
| `apropos` | apropos |  |  |  |
| `apropos-command` | apropos | `C-h a`, `<f1> a`, `<help> a` | `C-h a`, `<help> a` |  |
| `apropos-documentation` | apropos | `C-h d`, `<f1> d`, `<help> d` | `C-h d`, `<help> d` |  |
| `apropos-function` | apropos |  |  |  |
| `apropos-library` | apropos |  |  |  |
| `apropos-local-value` | apropos |  |  |  |
| `apropos-local-variable` | apropos |  |  |  |
| `apropos-user-option` | apropos |  |  |  |
| `apropos-value` | apropos |  |  |  |
| `apropos-variable` | apropos |  |  |  |
| `array-mode` | array |  |  |  |
| `artist-mode` | artist |  |  |  |
| `asm-mode` | asm-mode |  |  |  |
| `authinfo-mode` | auth-source |  |  |  |
| `autoconf-mode` | autoconf |  |  |  |
| `auto-insert` | autoinsert |  |  |  |
| `auto-insert-mode` | autoinsert |  |  |  |
| `make-directory-autoloads` | autoload |  |  |  |
| `update-directory-autoloads` | autoload |  |  |  |
| `auto-revert-mode` | autorevert |  |  |  |
| `auto-revert-tail-mode` | autorevert |  |  |  |
| `global-auto-revert-mode` | autorevert |  |  |  |
| `mouse-avoidance-mode` | avoid |  |  |  |
| `bat-mode` | bat-mode |  |  |  |
| `battery` | battery |  |  |  |
| `display-battery-mode` | battery |  |  |  |
| `benchmark` | benchmark |  |  |  |
| `bibtex-initialize` | bibtex |  |  |  |
| `bibtex-mode` | bibtex |  |  |  |
| `bibtex-search-entry` | bibtex |  |  |  |
| `bibtex-style-mode` | bibtex-style |  |  |  |
| `describe-personal-keybindings` | bind-key |  |  |  |
| `complete-symbol` | bindings | `C-M-i` | 同じ |  |
| `ignore-preserving-kill-region` | bindings | `<mouse-movement>` | 同じ |  |
| `left-char` | bindings | `<left>` |  |  |
| `left-word` | bindings | `C-<left>`, `M-<left>` | `M-<left>` |  |
| `mode-line-bury-buffer` | bindings |  |  |  |
| `mode-line-change-eol` | bindings |  |  |  |
| `mode-line-minor-mode-help` | bindings |  |  |  |
| `mode-line-next-buffer` | bindings |  |  |  |
| `mode-line-other-buffer` | bindings |  |  |  |
| `mode-line-previous-buffer` | bindings |  |  |  |
| `mode-line-toggle-modified` | bindings |  |  |  |
| `mode-line-toggle-read-only` | bindings |  |  |  |
| `mode-line-unbury-buffer` | bindings |  |  |  |
| `mode-line-widen` | bindings |  |  |  |
| `right-char` | bindings | `<right>` |  |  |
| `right-word` | bindings | `C-<right>`, `M-<right>` | `M-<right>` |  |
| `binhex-decode-region` | binhex |  |  |  |
| `binhex-decode-region-external` | binhex |  |  |  |
| `binhex-decode-region-internal` | binhex |  |  |  |
| `blackbox` | blackbox |  |  |  |
| `bookmark-bmenu-list` | bookmark | `C-x r l` | 同じ |  |
| `bookmark-bmenu-search` | bookmark |  |  |  |
| `bookmark-delete` | bookmark |  |  |  |
| `bookmark-delete-all` | bookmark |  |  |  |
| `bookmark-insert` | bookmark |  |  |  |
| `bookmark-insert-location` | bookmark |  |  |  |
| `bookmark-jump` | bookmark | `C-x r b` | 同じ | ✓ `C-x r b` |
| `bookmark-jump-other-frame` | bookmark |  |  |  |
| `bookmark-jump-other-window` | bookmark |  |  |  |
| `bookmark-load` | bookmark |  |  |  |
| `bookmark-relocate` | bookmark |  |  |  |
| `bookmark-rename` | bookmark |  |  |  |
| `bookmark-save` | bookmark |  |  |  |
| `bookmark-set` | bookmark | `C-x r m` | 同じ | ✓ `C-x r m` |
| `bookmark-set-no-overwrite` | bookmark | `C-x r M` | 同じ |  |
| `bookmark-write` | bookmark |  |  |  |
| `browse-url` | browse-url |  |  |  |
| `browse-url-at-mouse` | browse-url |  |  |  |
| `browse-url-at-point` | browse-url |  |  |  |
| `browse-url-button-open` | browse-url |  |  |  |
| `browse-url-chromium` | browse-url |  |  |  |
| `browse-url-conkeror` | browse-url |  |  |  |
| `browse-url-default-android-browser` | browse-url |  |  |  |
| `browse-url-default-haiku-browser` | browse-url |  |  |  |
| `browse-url-elinks` | browse-url |  |  |  |
| `browse-url-emacs` | browse-url |  |  |  |
| `browse-url-firefox` | browse-url |  |  |  |
| `browse-url-generic` | browse-url |  |  |  |
| `browse-url-gnome-moz` | browse-url |  |  |  |
| `browse-url-kde` | browse-url |  |  |  |
| `browse-url-mail` | browse-url |  |  |  |
| `browse-url-mozilla` | browse-url |  |  |  |
| `browse-url-of-buffer` | browse-url |  |  |  |
| `browse-url-of-dired-file` | browse-url |  |  |  |
| `browse-url-of-file` | browse-url |  |  |  |
| `browse-url-of-region` | browse-url |  |  |  |
| `browse-url-text-emacs` | browse-url |  |  |  |
| `browse-url-text-xterm` | browse-url |  |  |  |
| `browse-url-w3` | browse-url |  |  |  |
| `browse-url-w3-gnudoit` | browse-url |  |  |  |
| `browse-url-webpositive` | browse-url |  |  |  |
| `browse-url-with-browser-kind` | browse-url |  |  |  |
| `browse-url-xdg-open` | browse-url |  |  |  |
| `bs-customize` | bs |  |  |  |
| `bs-cycle-next` | bs |  |  |  |
| `bs-cycle-previous` | bs |  |  |  |
| `bs-show` | bs |  |  |  |
| `bubbles` | bubbles |  |  |  |
| `Buffer-menu-1-window` | buff-menu |  |  |  |
| `Buffer-menu-2-window` | buff-menu |  |  |  |
| `Buffer-menu-backup-unmark` | buff-menu |  |  |  |
| `Buffer-menu-bury` | buff-menu |  |  |  |
| `Buffer-menu-delete` | buff-menu |  |  |  |
| `Buffer-menu-delete-backwards` | buff-menu |  |  |  |
| `Buffer-menu-execute` | buff-menu |  |  |  |
| `Buffer-menu-isearch-buffers` | buff-menu |  |  |  |
| `Buffer-menu-isearch-buffers-regexp` | buff-menu |  |  |  |
| `Buffer-menu-mark` | buff-menu |  |  |  |
| `Buffer-menu-mode-menu` | buff-menu |  |  |  |
| `Buffer-menu-mouse-select` | buff-menu |  |  |  |
| `Buffer-menu-multi-occur` | buff-menu |  |  |  |
| `Buffer-menu-not-modified` | buff-menu |  |  |  |
| `Buffer-menu-other-window` | buff-menu |  |  |  |
| `Buffer-menu-save` | buff-menu |  |  |  |
| `Buffer-menu-select` | buff-menu |  |  |  |
| `Buffer-menu-sort` | buff-menu |  |  |  |
| `Buffer-menu-switch-other-window` | buff-menu |  |  |  |
| `Buffer-menu-this-window` | buff-menu |  |  |  |
| `Buffer-menu-toggle-files-only` | buff-menu |  |  |  |
| `Buffer-menu-toggle-internal` | buff-menu |  |  |  |
| `Buffer-menu-toggle-read-only` | buff-menu |  |  |  |
| `Buffer-menu-unmark` | buff-menu |  |  |  |
| `Buffer-menu-unmark-all` | buff-menu |  |  |  |
| `Buffer-menu-unmark-all-buffers` | buff-menu |  |  |  |
| `Buffer-menu-view` | buff-menu |  |  |  |
| `Buffer-menu-view-other-window` | buff-menu |  |  |  |
| `Buffer-menu-visit-tags-table` | buff-menu |  |  |  |
| `buffer-menu` | buff-menu |  |  |  |
| `buffer-menu-other-window` | buff-menu |  |  |  |
| `list-buffers` | buff-menu | `C-x C-b` | 同じ | ✓ `C-x C-b` |
| `bug-reference-mode` | bug-reference |  |  |  |
| `bug-reference-prog-mode` | bug-reference |  |  |  |
| `backward-button` | button |  |  |  |
| `button-describe` | button |  |  |  |
| `button-mode` | button |  |  |  |
| `forward-button` | button |  |  |  |
| `push-button` | button |  |  |  |
| `byte-compile-file` | bytecomp |  |  |  |
| `byte-force-recompile` | bytecomp |  |  |  |
| `byte-recompile-directory` | bytecomp |  |  |  |
| `compile-defun` | bytecomp |  |  |  |
| `display-call-tree` | bytecomp |  |  |  |
| `c++-ts-mode` | c-ts-mode |  |  |  |
| `c-or-c++-ts-mode` | c-ts-mode |  |  |  |
| `c-ts-mode` | c-ts-mode |  |  |  |
| `calendar-hebrew-list-yahrzeits` | cal-hebrew |  |  |  |
| `calc` | calc |  |  |  |
| `calc-dispatch` | calc | `C-x *` | 同じ |  |
| `calc-embedded` | calc |  |  |  |
| `calc-embedded-activate` | calc |  |  |  |
| `calc-grab-rectangle` | calc |  |  |  |
| `calc-grab-region` | calc |  |  |  |
| `calc-grab-sum-across` | calc |  |  |  |
| `calc-grab-sum-down` | calc |  |  |  |
| `calc-keypad` | calc |  |  |  |
| `full-calc` | calc |  |  |  |
| `full-calc-keypad` | calc |  |  |  |
| `quick-calc` | calc |  |  |  |
| `calc-undo` | calc-undo |  |  |  |
| `calculator` | calculator |  |  |  |
| `calendar` | calendar |  |  |  |
| `canlock-verify` | canlock |  |  |  |
| `describe-buffer-case-table` | case-table |  |  |  |
| `c-guess` | cc-guess |  |  |  |
| `c-guess-buffer` | cc-guess |  |  |  |
| `c-guess-buffer-no-install` | cc-guess |  |  |  |
| `c-guess-install` | cc-guess |  |  |  |
| `c-guess-no-install` | cc-guess |  |  |  |
| `c-guess-region` | cc-guess |  |  |  |
| `c-guess-region-no-install` | cc-guess |  |  |  |
| `awk-mode` | cc-mode |  |  |  |
| `c++-mode` | cc-mode |  |  |  |
| `c-mode` | cc-mode |  |  |  |
| `c-or-c++-mode` | cc-mode |  |  |  |
| `idl-mode` | cc-mode |  |  |  |
| `java-mode` | cc-mode |  |  |  |
| `objc-mode` | cc-mode |  |  |  |
| `pike-mode` | cc-mode |  |  |  |
| `c-add-style` | cc-styles |  |  |  |
| `c-set-offset` | cc-styles |  |  |  |
| `c-set-style` | cc-styles |  |  |  |
| `cfengine-auto-mode` | cfengine |  |  |  |
| `cfengine2-mode` | cfengine |  |  |  |
| `cfengine3-mode` | cfengine |  |  |  |
| `describe-char-fold-equivalences` | char-fold |  |  |  |
| `check-declare-directory` | check-declare |  |  |  |
| `check-declare-file` | check-declare |  |  |  |
| `checkdoc` | checkdoc |  |  |  |
| `checkdoc-comments` | checkdoc |  |  |  |
| `checkdoc-continue` | checkdoc |  |  |  |
| `checkdoc-current-buffer` | checkdoc |  |  |  |
| `checkdoc-defun` | checkdoc |  |  |  |
| `checkdoc-dired` | checkdoc |  |  |  |
| `checkdoc-eval-current-buffer` | checkdoc |  |  |  |
| `checkdoc-eval-defun` | checkdoc |  |  |  |
| `checkdoc-interactive` | checkdoc |  |  |  |
| `checkdoc-ispell` | checkdoc |  |  |  |
| `checkdoc-ispell-comments` | checkdoc |  |  |  |
| `checkdoc-ispell-continue` | checkdoc |  |  |  |
| `checkdoc-ispell-current-buffer` | checkdoc |  |  |  |
| `checkdoc-ispell-defun` | checkdoc |  |  |  |
| `checkdoc-ispell-interactive` | checkdoc |  |  |  |
| `checkdoc-ispell-message-interactive` | checkdoc |  |  |  |
| `checkdoc-ispell-message-text` | checkdoc |  |  |  |
| `checkdoc-ispell-start` | checkdoc |  |  |  |
| `checkdoc-message-interactive` | checkdoc |  |  |  |
| `checkdoc-message-text` | checkdoc |  |  |  |
| `checkdoc-minor-mode` | checkdoc |  |  |  |
| `checkdoc-package-keywords` | checkdoc |  |  |  |
| `checkdoc-rogue-spaces` | checkdoc |  |  |  |
| `checkdoc-start` | checkdoc |  |  |  |
| `decode-hz-buffer` | china-util |  |  |  |
| `decode-hz-region` | china-util |  |  |  |
| `encode-hz-buffer` | china-util |  |  |  |
| `encode-hz-region` | china-util |  |  |  |
| `command-history` | chistory |  |  |  |
| `list-command-history` | chistory |  |  |  |
| `repeat-matching-complex-command` | chistory |  |  |  |
| `cl-font-lock-built-in-mode` | cl-font-lock |  |  |  |
| `cl-old-struct-compat-mode` | cl-lib |  |  |  |
| `c-macro-expand` | cmacexp |  |  |  |
| `cmake-ts-mode` | cmake-ts-mode |  |  |  |
| `run-scheme` | cmuscheme |  |  |  |
| `comint-redirect-send-command` | comint |  |  |  |
| `comint-redirect-send-command-to-process` | comint |  |  |  |
| `comint-run` | comint |  |  |  |
| `compare-windows` | compare-w |  |  |  |
| `compilation-minor-mode` | compile |  |  |  |
| `compilation-mode` | compile |  |  |  |
| `compilation-next-error-function` | compile |  |  |  |
| `compilation-shell-minor-mode` | compile |  |  |  |
| `compile` | compile |  | `C-c c` |  |
| `dynamic-completion-mode` | completion |  |  |  |
| `completion-preview-mode` | completion-preview |  |  |  |
| `global-completion-preview-mode` | completion-preview |  |  |  |
| `auto-composition-mode` | composite |  |  |  |
| `compose-last-chars` | composite |  |  |  |
| `compose-region` | composite |  |  |  |
| `decompose-region` | composite |  |  |  |
| `global-auto-composition-mode` | composite |  |  |  |
| `toggle-auto-composition` | composite |  |  |  |
| `conf-colon-mode` | conf-mode |  |  |  |
| `conf-desktop-mode` | conf-mode |  |  |  |
| `conf-javaprop-mode` | conf-mode |  |  |  |
| `conf-mode` | conf-mode |  |  |  |
| `conf-ppd-mode` | conf-mode |  |  |  |
| `conf-space-keywords` | conf-mode |  |  |  |
| `conf-space-mode` | conf-mode |  |  |  |
| `conf-toml-mode` | conf-mode |  |  |  |
| `conf-unix-mode` | conf-mode |  |  |  |
| `conf-windows-mode` | conf-mode |  |  |  |
| `conf-xdefaults-mode` | conf-mode |  |  |  |
| `cookie` | cookie1 |  |  |  |
| `copyright` | copyright |  |  |  |
| `copyright-fix-years` | copyright |  |  |  |
| `copyright-update` | copyright |  |  |  |
| `copyright-update-directory` | copyright |  |  |  |
| `cperl-mode` | cperl-mode |  |  |  |
| `cperl-perldoc` | cperl-mode |  |  |  |
| `cperl-perldoc-at-point` | cperl-mode |  |  |  |
| `cpp-highlight-buffer` | cpp |  |  |  |
| `cpp-parse-edit` | cpp |  |  |  |
| `csharp-mode` | csharp-mode |  |  |  |
| `csharp-ts-mode` | csharp-mode |  |  |  |
| `css-lookup-symbol` | css-mode |  |  |  |
| `css-mode` | css-mode |  |  |  |
| `css-ts-mode` | css-mode |  |  |  |
| `scss-mode` | css-mode |  |  |  |
| `cua-mode` | cua-base |  |  |  |
| `cua-selection-mode` | cua-base |  |  |  |
| `cua-rectangle-mark-mode` | cua-rect |  |  |  |
| `cursor-intangible-mode` | cursor-sensor |  |  |  |
| `cursor-sensor-mode` | cursor-sensor |  |  |  |
| `customize` | cus-edit |  |  |  |
| `customize-apropos` | cus-edit |  |  |  |
| `customize-apropos-faces` | cus-edit |  |  |  |
| `customize-apropos-groups` | cus-edit |  |  |  |
| `customize-apropos-options` | cus-edit |  |  |  |
| `customize-browse` | cus-edit |  |  |  |
| `customize-changed` | cus-edit |  |  |  |
| `customize-dirlocals` | cus-edit |  |  |  |
| `customize-face` | cus-edit |  |  |  |
| `customize-face-other-window` | cus-edit |  |  |  |
| `customize-group` | cus-edit |  |  |  |
| `customize-group-other-window` | cus-edit |  |  |  |
| `customize-icon` | cus-edit |  |  |  |
| `customize-mode` | cus-edit |  |  |  |
| `customize-option` | cus-edit |  |  |  |
| `customize-option-other-window` | cus-edit |  |  |  |
| `customize-rogue` | cus-edit |  |  |  |
| `customize-save-customized` | cus-edit |  |  |  |
| `customize-save-variable` | cus-edit |  |  |  |
| `customize-saved` | cus-edit |  |  |  |
| `customize-set-value` | cus-edit |  |  |  |
| `customize-set-variable` | cus-edit |  |  |  |
| `customize-toggle-option` | cus-edit |  |  |  |
| `customize-unsaved` | cus-edit |  |  |  |
| `custom-theme-visit-theme` | cus-theme |  |  |  |
| `customize-create-theme` | cus-theme |  |  |  |
| `customize-themes` | cus-theme |  |  |  |
| `describe-theme` | cus-theme |  |  |  |
| `disable-theme` | custom |  |  |  |
| `enable-theme` | custom |  |  |  |
| `load-theme` | custom |  |  |  |
| `theme-choose-variant` | custom |  |  |  |
| `toggle-theme` | custom |  |  |  |
| `cvs-status-mode` | cvs-status |  |  |  |
| `cwarn-mode` | cwarn |  |  |  |
| `global-cwarn-mode` | cwarn |  |  |  |
| `standard-display-cyrillic-translit` | cyril-util |  |  |  |
| `dabbrev-completion` | dabbrev | `C-M-/` | 同じ |  |
| `dabbrev-expand` | dabbrev | `M-/` | 同じ | ✓ `M-/` |
| `dbus-handle-event` | dbus |  |  |  |
| `dbus-monitor` | dbus |  |  |  |
| `dcl-mode` | dcl-mode |  |  |  |
| `cancel-debug-on-entry` | debug |  |  |  |
| `cancel-debug-on-variable-change` | debug |  |  |  |
| `debug` | debug |  |  |  |
| `debug-on-entry` | debug |  |  |  |
| `debug-on-variable-change` | debug |  |  |  |
| `decipher` | decipher |  |  |  |
| `decipher-mode` | decipher |  |  |  |
| `delimit-columns-customize` | delim-col |  |  |  |
| `delimit-columns-rectangle` | delim-col |  |  |  |
| `delimit-columns-region` | delim-col |  |  |  |
| `delete-active-region` | delsel |  |  |  |
| `delete-selection-mode` | delsel |  |  |  |
| `describe-char` | descr-text |  |  |  |
| `describe-text-properties` | descr-text |  |  |  |
| `desktop-change-dir` | desktop |  |  |  |
| `desktop-clear` | desktop |  |  |  |
| `desktop-read` | desktop |  |  |  |
| `desktop-remove` | desktop |  |  |  |
| `desktop-revert` | desktop |  |  |  |
| `desktop-save` | desktop |  |  |  |
| `desktop-save-in-desktop-dir` | desktop |  |  |  |
| `desktop-save-mode` | desktop |  |  |  |
| `gnus-article-outlook-deuglify-article` | deuglify |  |  |  |
| `gnus-article-outlook-rearrange-citation` | deuglify |  |  |  |
| `gnus-article-outlook-repair-attribution` | deuglify |  |  |  |
| `gnus-article-outlook-unwrap-lines` | deuglify |  |  |  |
| `gnus-outlook-deuglify-article` | deuglify |  |  |  |
| `diary` | diary-lib |  |  |  |
| `diary-mail-entries` | diary-lib |  |  |  |
| `diary-mode` | diary-lib |  |  |  |
| `dictionary` | dictionary |  |  |  |
| `dictionary-lookup-definition` | dictionary |  |  |  |
| `dictionary-match-words` | dictionary |  |  |  |
| `dictionary-mode` | dictionary |  |  |  |
| `dictionary-mouse-popup-matching-words` | dictionary |  |  |  |
| `dictionary-popup-matching-words` | dictionary |  |  |  |
| `dictionary-search` | dictionary |  |  |  |
| `dictionary-tooltip-mode` | dictionary |  |  |  |
| `global-dictionary-tooltip-mode` | dictionary |  |  |  |
| `diff` | diff |  |  |  |
| `diff-backup` | diff |  |  |  |
| `diff-buffer-with-file` | diff |  |  |  |
| `diff-buffers` | diff |  |  |  |
| `diff-minor-mode` | diff-mode |  |  |  |
| `diff-mode` | diff-mode |  |  |  |
| `dig` | dig |  |  |  |
| `dired` | dired | `C-x d` | 同じ | ✓ `C-x d` |
| `dired-jump` | dired | `C-x C-j` | 同じ |  |
| `dired-jump-other-window` | dired | `C-x 4 C-j` | `C-x j`, `C-x 4 C-j` |  |
| `dired-other-frame` | dired | `C-x 5 d` | 同じ |  |
| `dired-other-tab` | dired | `C-x t d` | 同じ |  |
| `dired-other-window` | dired | `C-x 4 d` | 同じ |  |
| `dirtrack-mode` | dirtrack |  |  |  |
| `disassemble` | disass |  |  |  |
| `describe-current-display-table` | disp-table |  |  |  |
| `standard-display-by-replacement-char` | disp-table |  |  |  |
| `display-fill-column-indicator-mode` | display-fill-column-indicator |  |  |  |
| `global-display-fill-column-indicator-mode` | display-fill-column-indicator |  |  |  |
| `display-line-numbers-mode` | display-line-numbers |  |  |  |
| `global-display-line-numbers-mode` | display-line-numbers |  |  |  |
| `header-line-indent-mode` | display-line-numbers |  |  |  |
| `dissociated-press` | dissociate |  |  |  |
| `dns-mode` | dns-mode |  |  |  |
| `dns-mode-soa-increment-serial` | dns-mode |  |  |  |
| `doc-view-minor-mode` | doc-view |  |  |  |
| `doc-view-mode` | doc-view |  |  |  |
| `dockerfile-ts-mode` | dockerfile-ts-mode |  |  |  |
| `doctor` | doctor |  |  |  |
| `double-mode` | double |  |  |  |
| `dunnet` | dunnet |  |  |  |
| `dynamic-setting-handle-config-changed-event` | dynamic-setting |  |  |  |
| `easy-menu-add` | easymenu |  |  |  |
| `easy-menu-remove` | easymenu |  |  |  |
| `ebnf-apply-style` | ebnf2ps |  |  |  |
| `ebnf-customize` | ebnf2ps |  |  |  |
| `ebnf-delete-style` | ebnf2ps |  |  |  |
| `ebnf-eps-buffer` | ebnf2ps |  |  |  |
| `ebnf-eps-directory` | ebnf2ps |  |  |  |
| `ebnf-eps-file` | ebnf2ps |  |  |  |
| `ebnf-eps-region` | ebnf2ps |  |  |  |
| `ebnf-find-style` | ebnf2ps |  |  |  |
| `ebnf-insert-style` | ebnf2ps |  |  |  |
| `ebnf-merge-style` | ebnf2ps |  |  |  |
| `ebnf-pop-style` | ebnf2ps |  |  |  |
| `ebnf-print-buffer` | ebnf2ps |  |  |  |
| `ebnf-print-directory` | ebnf2ps |  |  |  |
| `ebnf-print-file` | ebnf2ps |  |  |  |
| `ebnf-print-region` | ebnf2ps |  |  |  |
| `ebnf-push-style` | ebnf2ps |  |  |  |
| `ebnf-reset-style` | ebnf2ps |  |  |  |
| `ebnf-spool-buffer` | ebnf2ps |  |  |  |
| `ebnf-spool-directory` | ebnf2ps |  |  |  |
| `ebnf-spool-file` | ebnf2ps |  |  |  |
| `ebnf-spool-region` | ebnf2ps |  |  |  |
| `ebnf-syntax-buffer` | ebnf2ps |  |  |  |
| `ebnf-syntax-directory` | ebnf2ps |  |  |  |
| `ebnf-syntax-file` | ebnf2ps |  |  |  |
| `ebnf-syntax-region` | ebnf2ps |  |  |  |
| `ebrowse-back-in-position-stack` | ebrowse |  |  |  |
| `ebrowse-electric-choose-tree` | ebrowse |  |  |  |
| `ebrowse-electric-position-menu` | ebrowse |  |  |  |
| `ebrowse-forward-in-position-stack` | ebrowse |  |  |  |
| `ebrowse-member-mode` | ebrowse |  |  |  |
| `ebrowse-save-tree` | ebrowse |  |  |  |
| `ebrowse-save-tree-as` | ebrowse |  |  |  |
| `ebrowse-statistics` | ebrowse |  |  |  |
| `ebrowse-tags-complete-symbol` | ebrowse |  |  |  |
| `ebrowse-tags-find-declaration` | ebrowse |  |  |  |
| `ebrowse-tags-find-declaration-other-frame` | ebrowse |  |  |  |
| `ebrowse-tags-find-declaration-other-window` | ebrowse |  |  |  |
| `ebrowse-tags-find-definition` | ebrowse |  |  |  |
| `ebrowse-tags-find-definition-other-frame` | ebrowse |  |  |  |
| `ebrowse-tags-find-definition-other-window` | ebrowse |  |  |  |
| `ebrowse-tags-loop-continue` | ebrowse |  |  |  |
| `ebrowse-tags-query-replace` | ebrowse |  |  |  |
| `ebrowse-tags-search` | ebrowse |  |  |  |
| `ebrowse-tags-search-member-use` | ebrowse |  |  |  |
| `ebrowse-tags-view-declaration` | ebrowse |  |  |  |
| `ebrowse-tags-view-definition` | ebrowse |  |  |  |
| `ebrowse-tags-view-definition-other-frame` | ebrowse |  |  |  |
| `ebrowse-tags-view-definition-other-window` | ebrowse |  |  |  |
| `ebrowse-tree-mode` | ebrowse |  |  |  |
| `electric-buffer-list` | ebuff-menu |  |  |  |
| `Electric-command-history-redo-expression` | echistory |  |  |  |
| `global-ede-mode` | ede |  |  |  |
| `edebug-all-defs` | edebug |  |  |  |
| `edebug-all-forms` | edebug |  |  |  |
| `edebug-eval-top-level-form` | edebug |  |  |  |
| `ediff-backup` | ediff |  |  |  |
| `ediff-buffers` | ediff |  |  |  |
| `ediff-buffers3` | ediff |  |  |  |
| `ediff-current-file` | ediff |  |  |  |
| `ediff-directories` | ediff |  |  |  |
| `ediff-directories3` | ediff |  |  |  |
| `ediff-directory-revisions` | ediff |  |  |  |
| `ediff-documentation` | ediff |  |  |  |
| `ediff-files` | ediff |  |  |  |
| `ediff-files3` | ediff |  |  |  |
| `ediff-merge-buffers` | ediff |  |  |  |
| `ediff-merge-buffers-with-ancestor` | ediff |  |  |  |
| `ediff-merge-directories` | ediff |  |  |  |
| `ediff-merge-directories-with-ancestor` | ediff |  |  |  |
| `ediff-merge-directory-revisions` | ediff |  |  |  |
| `ediff-merge-directory-revisions-with-ancestor` | ediff |  |  |  |
| `ediff-merge-files` | ediff |  |  |  |
| `ediff-merge-files-with-ancestor` | ediff |  |  |  |
| `ediff-merge-revisions` | ediff |  |  |  |
| `ediff-merge-revisions-with-ancestor` | ediff |  |  |  |
| `ediff-patch-buffer` | ediff |  |  |  |
| `ediff-patch-file` | ediff |  |  |  |
| `ediff-regions-linewise` | ediff |  |  |  |
| `ediff-regions-wordwise` | ediff |  |  |  |
| `ediff-revision` | ediff |  |  |  |
| `ediff-version` | ediff |  |  |  |
| `ediff-windows-linewise` | ediff |  |  |  |
| `ediff-windows-wordwise` | ediff |  |  |  |
| `ediff-customize` | ediff-help |  |  |  |
| `ediff-show-registry` | ediff-mult |  |  |  |
| `ediff-toggle-multiframe` | ediff-util |  |  |  |
| `ediff-toggle-use-toolbar` | ediff-util |  |  |  |
| `editorconfig-mode` | editorconfig |  |  |  |
| `editorconfig-conf-mode` | editorconfig-conf-mode |  |  |  |
| `editorconfig-apply` | editorconfig-tools |  |  |  |
| `editorconfig-display-current-properties` | editorconfig-tools |  |  |  |
| `editorconfig-find-current-editorconfig` | editorconfig-tools |  |  |  |
| `edit-kbd-macro` | edmacro | `C-x C-k e` | 同じ |  |
| `edit-last-kbd-macro` | edmacro |  |  |  |
| `edit-named-kbd-macro` | edmacro |  |  |  |
| `read-kbd-macro` | edmacro |  |  |  |
| `edt-emulation-on` | edt |  |  |  |
| `edt-set-scroll-margins` | edt |  |  |  |
| `eglot` | eglot |  |  |  |
| `eglot-upgrade-eglot` | eglot |  |  |  |
| `eieio-browse` | eieio-opt |  |  |  |
| `eldoc` | eldoc |  |  |  |
| `eldoc-doc-buffer` | eldoc |  |  |  |
| `eldoc-mode` | eldoc |  |  |  |
| `eldoc-print-current-symbol-info` | eldoc |  |  |  |
| `global-eldoc-mode` | eldoc |  |  |  |
| `electric-pair-local-mode` | elec-pair |  |  |  |
| `electric-pair-mode` | elec-pair |  |  |  |
| `electric-indent-just-newline` | electric |  |  |  |
| `electric-indent-local-mode` | electric |  |  |  |
| `electric-indent-mode` | electric |  |  |  |
| `electric-layout-local-mode` | electric |  |  |  |
| `electric-layout-mode` | electric |  |  |  |
| `electric-newline-and-maybe-indent` | electric | `C-j` | 同じ | ✓ `C-j` |
| `electric-quote-local-mode` | electric |  |  |  |
| `electric-quote-mode` | electric |  |  |  |
| `elide-head` | elide-head |  |  |  |
| `elide-head-mode` | elide-head |  |  |  |
| `elint-current-buffer` | elint |  |  |  |
| `elint-defun` | elint |  |  |  |
| `elint-directory` | elint |  |  |  |
| `elint-file` | elint |  |  |  |
| `elint-initialize` | elint |  |  |  |
| `elisp-byte-code-mode` | elisp-mode |  |  |  |
| `elisp-byte-compile-buffer` | elisp-mode |  |  |  |
| `elisp-byte-compile-file` | elisp-mode |  |  |  |
| `elisp-enable-lexical-binding` | elisp-mode |  |  |  |
| `elisp-eval-region-or-buffer` | elisp-mode |  |  |  |
| `elisp-flymake--batch-compile-for-flymake` | elisp-mode |  |  |  |
| `elisp-last-sexp-toggle-display` | elisp-mode |  |  |  |
| `emacs-lisp-byte-compile` | elisp-mode |  |  |  |
| `emacs-lisp-byte-compile-and-load` | elisp-mode |  |  |  |
| `emacs-lisp-macroexpand` | elisp-mode |  |  |  |
| `emacs-lisp-mode` | elisp-mode |  |  |  |
| `emacs-lisp-mode-menu` | elisp-mode |  |  |  |
| `emacs-lisp-native-compile` | elisp-mode |  |  |  |
| `emacs-lisp-native-compile-and-load` | elisp-mode |  |  |  |
| `eval-defun` | elisp-mode |  |  |  |
| `eval-last-sexp` | elisp-mode | `C-x C-e` | 同じ | ✓ `C-x C-e` |
| `eval-print-last-sexp` | elisp-mode |  |  |  |
| `lisp-interaction-mode` | elisp-mode |  |  |  |
| `lisp-interaction-mode-menu` | elisp-mode |  |  |  |
| `elixir-ts-mode` | elixir-ts-mode |  |  |  |
| `elp-instrument-function` | elp |  |  |  |
| `elp-instrument-list` | elp |  |  |  |
| `elp-instrument-package` | elp |  |  |  |
| `elp-results` | elp |  |  |  |
| `emacs-authors-mode` | emacs-authors-mode |  |  |  |
| `emacs-lock-mode` | emacs-lock |  |  |  |
| `emacs-news-mode` | emacs-news-mode |  |  |  |
| `emacs-news-view-mode` | emacs-news-mode |  |  |  |
| `emacs-build-description` | emacsbug |  |  |  |
| `report-emacs-bug` | emacsbug |  |  |  |
| `submit-emacs-patch` | emacsbug |  |  |  |
| `emerge-buffers` | emerge |  |  |  |
| `emerge-buffers-with-ancestor` | emerge |  |  |  |
| `emerge-files` | emerge |  |  |  |
| `emerge-files-with-ancestor` | emerge |  |  |  |
| `emerge-merge-directories` | emerge |  |  |  |
| `emerge-revisions` | emerge |  |  |  |
| `emerge-revisions-with-ancestor` | emerge |  |  |  |
| `emoji-describe` | emoji | `C-x 8 e d` | 同じ |  |
| `emoji-insert` | emoji | `C-x 8 e e`, `C-x 8 e i` | 同じ |  |
| `emoji-list` | emoji | `C-x 8 e l` | 同じ |  |
| `emoji-list-select` | emoji |  |  |  |
| `emoji-recent` | emoji | `C-x 8 e r` | 同じ |  |
| `emoji-search` | emoji | `C-x 8 e s` | 同じ |  |
| `emoji-zoom-decrease` | emoji | `C-x 8 e -` | 同じ |  |
| `emoji-zoom-increase` | emoji | `C-x 8 e +` | 同じ |  |
| `emoji-zoom-reset` | emoji | `C-x 8 e 0` | 同じ |  |
| `enriched-mode` | enriched |  |  |  |
| `getenv` | env |  |  |  |
| `setenv` | env |  |  |  |
| `epa-decrypt-armor-in-region` | epa |  |  |  |
| `epa-decrypt-file` | epa |  |  |  |
| `epa-decrypt-region` | epa |  |  |  |
| `epa-delete-keys` | epa |  |  |  |
| `epa-encrypt-file` | epa |  |  |  |
| `epa-encrypt-region` | epa |  |  |  |
| `epa-export-keys` | epa |  |  |  |
| `epa-import-armor-in-region` | epa |  |  |  |
| `epa-import-keys` | epa |  |  |  |
| `epa-import-keys-region` | epa |  |  |  |
| `epa-insert-keys` | epa |  |  |  |
| `epa-list-keys` | epa |  |  |  |
| `epa-list-secret-keys` | epa |  |  |  |
| `epa-sign-file` | epa |  |  |  |
| `epa-sign-region` | epa |  |  |  |
| `epa-verify-cleartext-in-region` | epa |  |  |  |
| `epa-verify-file` | epa |  |  |  |
| `epa-verify-region` | epa |  |  |  |
| `epa-dired-do-decrypt` | epa-dired |  |  |  |
| `epa-dired-do-encrypt` | epa-dired |  |  |  |
| `epa-dired-do-sign` | epa-dired |  |  |  |
| `epa-dired-do-verify` | epa-dired |  |  |  |
| `epa-file-disable` | epa-file |  |  |  |
| `epa-file-enable` | epa-file |  |  |  |
| `auto-encryption-mode` | epa-hook |  |  |  |
| `epa-file-name-regexp-update` | epa-hook |  |  |  |
| `epa-search-keys` | epa-ks |  |  |  |
| `epa-global-mail-mode` | epa-mail |  |  |  |
| `epa-mail-decrypt` | epa-mail |  |  |  |
| `epa-mail-encrypt` | epa-mail |  |  |  |
| `epa-mail-import-keys` | epa-mail |  |  |  |
| `epa-mail-mode` | epa-mail |  |  |  |
| `epa-mail-sign` | epa-mail |  |  |  |
| `epa-mail-verify` | epa-mail |  |  |  |
| `erc` | erc |  |  |  |
| `erc-server-select` | erc |  |  |  |
| `erc-tls` | erc |  |  |  |
| `ert-describe-test` | ert |  |  |  |
| `ert-run-tests-interactively` | ert |  |  |  |
| `ert-kill-all-test-buffers` | ert-x |  |  |  |
| `erts-mode` | erts-mode |  |  |  |
| `eshell-mode` | esh-mode |  |  |  |
| `eshell` | eshell |  |  |  |
| `eshell-command` | eshell |  |  |  |
| `complete-tag` | etags |  |  |  |
| `find-tag` | etags |  |  |  |
| `find-tag-noselect` | etags |  |  |  |
| `find-tag-other-frame` | etags |  |  |  |
| `find-tag-other-window` | etags |  |  |  |
| `find-tag-regexp` | etags |  |  |  |
| `list-tags` | etags |  |  |  |
| `select-tags-table` | etags |  |  |  |
| `tags-apropos` | etags |  |  |  |
| `tags-loop-continue` | etags |  |  |  |
| `tags-next-file` | etags |  |  |  |
| `tags-query-replace` | etags |  |  |  |
| `tags-reset-tags-tables` | etags |  |  |  |
| `tags-search` | etags |  |  |  |
| `tags-table-mode` | etags |  |  |  |
| `visit-tags-table` | etags |  |  |  |
| `etags-regen-mode` | etags-regen |  |  |  |
| `ethio-fidel-to-sera-buffer` | ethio-util |  |  |  |
| `ethio-fidel-to-sera-marker` | ethio-util |  |  |  |
| `ethio-fidel-to-sera-region` | ethio-util |  |  |  |
| `ethio-fidel-to-tex-buffer` | ethio-util |  |  |  |
| `ethio-input-special-character` | ethio-util |  |  |  |
| `ethio-insert-ethio-space` | ethio-util |  |  |  |
| `ethio-modify-vowel` | ethio-util |  |  |  |
| `ethio-replace-space` | ethio-util |  |  |  |
| `ethio-sera-to-fidel-buffer` | ethio-util |  |  |  |
| `ethio-sera-to-fidel-marker` | ethio-util |  |  |  |
| `ethio-sera-to-fidel-region` | ethio-util |  |  |  |
| `ethio-tex-to-fidel-buffer` | ethio-util |  |  |  |
| `eudc-expand-inline` | eudc |  |  |  |
| `eudc-expand-try-all` | eudc |  |  |  |
| `eudc-get-email` | eudc |  |  |  |
| `eudc-get-phone` | eudc |  |  |  |
| `eudc-load-eudc` | eudc |  |  |  |
| `eudc-query-form` | eudc |  |  |  |
| `eudc-set-server` | eudc |  |  |  |
| `eudc-insert-record-at-point-into-bbdb` | eudc-export |  |  |  |
| `eudc-try-bbdb-insert` | eudc-export |  |  |  |
| `eudc-edit-hotlist` | eudc-hotlist |  |  |  |
| `eww` | eww |  |  |  |
| `eww-browse` | eww |  |  |  |
| `eww-list-bookmarks` | eww |  |  |  |
| `eww-mode` | eww |  |  |  |
| `eww-open-file` | eww |  |  |  |
| `eww-search-words` | eww | `M-s M-w` | 同じ |  |
| `executable-interpret` | executable |  |  |  |
| `executable-set-magic` | executable |  |  |  |
| `expand-jump-to-next-slot` | expand | `C-x a n` | 同じ |  |
| `expand-jump-to-previous-slot` | expand | `C-x a p` | 同じ |  |
| `f90-mode` | f90 |  |  |  |
| `buffer-face-mode` | face-remap |  |  |  |
| `buffer-face-set` | face-remap |  |  |  |
| `buffer-face-toggle` | face-remap |  |  |  |
| `global-text-scale-adjust` | face-remap | `C-x C-M-+`, `C-x C-M--`, `C-x C-M-0`, `C-x C-M-=` | 同じ |  |
| `text-scale-adjust` | face-remap | `C-x C-+`, `C-x C--`, `C-x C-0`, `C-x C-=` | 同じ |  |
| `text-scale-decrease` | face-remap |  |  |  |
| `text-scale-increase` | face-remap |  |  |  |
| `text-scale-pinch` | face-remap | `<pinch>` | 同じ |  |
| `text-scale-set` | face-remap |  |  |  |
| `variable-pitch-mode` | face-remap |  |  |  |
| `list-colors-display` | facemenu |  |  |  |
| `invert-face` | faces |  |  |  |
| `list-faces-display` | faces |  |  |  |
| `make-empty-face` | faces |  |  |  |
| `make-face` | faces |  |  |  |
| `make-face-bold` | faces |  |  |  |
| `make-face-bold-italic` | faces |  |  |  |
| `make-face-italic` | faces |  |  |  |
| `make-face-unbold` | faces |  |  |  |
| `make-face-unitalic` | faces |  |  |  |
| `modify-face` | faces |  |  |  |
| `read-color` | faces |  |  |  |
| `set-face-background` | faces |  |  |  |
| `set-face-background-pixmap` | faces |  |  |  |
| `set-face-extend` | faces |  |  |  |
| `set-face-font` | faces |  |  |  |
| `set-face-foreground` | faces |  |  |  |
| `set-face-inverse-video` | faces |  |  |  |
| `set-face-inverse-video-p` | faces |  |  |  |
| `set-face-stipple` | faces |  |  |  |
| `set-face-underline` | faces |  |  |  |
| `faceup-clean-buffer` | faceup |  |  |  |
| `faceup-render-view-buffer` | faceup |  |  |  |
| `faceup-view-buffer` | faceup |  |  |  |
| `faceup-write-file` | faceup |  |  |  |
| `feedmail-queue-reminder` | feedmail |  |  |  |
| `feedmail-run-the-queue` | feedmail |  |  |  |
| `feedmail-run-the-queue-global-prompt` | feedmail |  |  |  |
| `feedmail-run-the-queue-no-prompts` | feedmail |  |  |  |
| `dired-at-point` | ffap |  | `C-x M-d` |  |
| `ffap-at-mouse` | ffap |  |  |  |
| `ffap-bindings` | ffap |  |  |  |
| `ffap-menu` | ffap |  |  |  |
| `ffap-next` | ffap |  |  |  |
| `find-file-at-point` | ffap |  | `C-x M-f` |  |
| `file-cache-add-directory` | filecache |  |  |  |
| `file-cache-add-directory-list` | filecache |  |  |  |
| `file-cache-add-directory-recursively` | filecache |  |  |  |
| `file-cache-add-directory-using-find` | filecache |  |  |  |
| `file-cache-add-directory-using-locate` | filecache |  |  |  |
| `file-cache-add-file` | filecache |  |  |  |
| `file-cache-minibuffer-complete` | filecache |  |  |  |
| `file-notify-handle-event` | filenotify |  |  |  |
| `append-to-file` | files |  |  |  |
| `auto-save-visited-mode` | files |  |  |  |
| `basic-save-buffer` | files |  |  |  |
| `cd` | files |  |  |  |
| `copy-directory` | files |  |  |  |
| `delete-directory` | files |  |  |  |
| `delete-file` | files |  |  |  |
| `find-alternate-file` | files | `C-x C-v` | 同じ | ✓ `C-x C-v` |
| `find-alternate-file-other-window` | files |  |  |  |
| `find-file` | files | `<open>`, `C-x C-f` | 同じ | ✓ `C-x C-f` |
| `find-file-existing` | files |  |  |  |
| `find-file-literally` | files |  |  |  |
| `find-file-other-frame` | files | `C-x 5 f`, `C-x 5 C-f` | 同じ |  |
| `find-file-other-window` | files | `C-x 4 f`, `C-x 4 C-f` | 同じ |  |
| `find-file-read-only` | files | `C-x C-r` | 同じ |  |
| `find-file-read-only-other-frame` | files | `C-x 5 r` | 同じ |  |
| `find-file-read-only-other-window` | files | `C-x 4 r` | 同じ |  |
| `find-sibling-file` | files |  |  |  |
| `insert-file` | files | `C-x i` | 同じ | ✓ `C-x i` |
| `insert-file-literally` | files |  |  |  |
| `kill-matching-buffers` | files |  |  |  |
| `kill-matching-buffers-no-ask` | files |  |  |  |
| `kill-some-buffers` | files |  |  |  |
| `list-directory` | files | `C-x C-d` | 同じ |  |
| `load-file` | files |  |  |  |
| `load-library` | files |  |  |  |
| `lock-file-mode` | files |  |  |  |
| `make-directory` | files |  |  |  |
| `make-empty-file` | files |  |  |  |
| `move-file-to-trash` | files |  |  |  |
| `normal-mode` | files |  |  |  |
| `not-modified` | files | `M-~` | 同じ |  |
| `pwd` | files |  |  |  |
| `recode-file-name` | files |  |  |  |
| `recover-file` | files |  |  |  |
| `recover-session` | files |  |  |  |
| `recover-session-finish` | files |  |  |  |
| `recover-this-file` | files |  |  | ✓ |
| `rename-uniquely` | files | `C-x x u` | 同じ |  |
| `rename-visited-file` | files |  |  |  |
| `restart-emacs` | files |  |  |  |
| `revert-buffer` | files |  |  | ✓ |
| `revert-buffer-quick` | files | `C-x x g` | 同じ |  |
| `revert-buffer-with-fine-grain` | files |  |  |  |
| `save-buffer` | files | `C-x C-s` | 同じ | ✓ `C-x C-s` |
| `save-buffers-kill-emacs` | files |  |  |  |
| `save-buffers-kill-terminal` | files | `C-x C-c` | 同じ | ✓ `C-x C-c` |
| `save-some-buffers` | files | `C-x s` | 同じ | ✓ `C-x s` |
| `set-visited-file-name` | files |  |  |  |
| `write-file` | files | `C-x C-w` | 同じ | ✓ `C-x C-w` |
| `add-dir-local-variable` | files-x |  |  |  |
| `add-file-local-variable` | files-x |  |  |  |
| `add-file-local-variable-prop-line` | files-x |  |  |  |
| `copy-dir-locals-to-file-locals` | files-x |  |  |  |
| `copy-dir-locals-to-file-locals-prop-line` | files-x |  |  |  |
| `copy-file-locals-to-dir-locals` | files-x |  |  |  |
| `delete-dir-local-variable` | files-x |  |  |  |
| `delete-file-local-variable` | files-x |  |  |  |
| `delete-file-local-variable-prop-line` | files-x |  |  |  |
| `canonically-space-region` | fill |  |  |  |
| `fill-individual-paragraphs` | fill |  |  |  |
| `fill-nonuniform-paragraphs` | fill |  |  |  |
| `fill-paragraph` | fill | `M-q` | 同じ | ✓ `M-q` |
| `fill-region` | fill |  |  |  |
| `fill-region-as-paragraph` | fill |  |  |  |
| `justify-current-line` | fill |  |  |  |
| `set-fill-prefix` | fill | `C-x .` | 同じ |  |
| `set-justification` | fill |  |  |  |
| `set-justification-center` | fill |  |  |  |
| `set-justification-full` | fill |  |  |  |
| `set-justification-left` | fill |  |  |  |
| `set-justification-none` | fill |  |  |  |
| `set-justification-right` | fill |  |  |  |
| `find-dired` | find-dired |  |  |  |
| `find-dired-with-command` | find-dired |  |  |  |
| `find-grep-dired` | find-dired |  |  |  |
| `find-name-dired` | find-dired |  |  |  |
| `ff-find-other-file` | find-file |  |  |  |
| `ff-find-other-file-other-window` | find-file |  |  |  |
| `ff-get-other-file` | find-file |  |  |  |
| `find-face-definition` | find-func |  |  |  |
| `find-function` | find-func |  | `C-x F` |  |
| `find-function-at-point` | find-func |  |  |  |
| `find-function-on-key` | find-func |  | `C-x K` |  |
| `find-function-on-key-other-frame` | find-func |  | `C-x 5 K` |  |
| `find-function-on-key-other-window` | find-func |  | `C-x 4 K` |  |
| `find-function-other-frame` | find-func |  | `C-x 5 F` |  |
| `find-function-other-window` | find-func |  | `C-x 4 F` |  |
| `find-library` | find-func |  | `C-x L` |  |
| `find-library-other-frame` | find-func |  | `C-x 5 L` |  |
| `find-library-other-window` | find-func |  | `C-x 4 L` |  |
| `find-variable` | find-func |  | `C-x V` |  |
| `find-variable-at-point` | find-func |  |  |  |
| `find-variable-other-frame` | find-func |  | `C-x 5 V` |  |
| `find-variable-other-window` | find-func |  | `C-x 4 V` |  |
| `find-lisp-find-dired` | find-lisp |  |  |  |
| `find-lisp-find-dired-filter` | find-lisp |  |  |  |
| `find-lisp-find-dired-subdirectories` | find-lisp |  |  |  |
| `find-lisp-find-dired-subdirs-other-window` | find-lisp |  |  |  |
| `finder-by-keyword` | finder | `C-h p`, `<f1> p`, `<help> p` | `C-h p`, `<help> p` |  |
| `finder-commentary` | finder |  |  |  |
| `finder-list-keywords` | finder |  |  |  |
| `enable-flow-control` | flow-ctrl |  |  |  |
| `flymake-mode` | flymake |  |  |  |
| `flyspell-buffer` | flyspell |  |  |  |
| `flyspell-mode` | flyspell |  |  |  |
| `flyspell-prog-mode` | flyspell |  |  |  |
| `flyspell-region` | flyspell |  |  |  |
| `follow-delete-other-windows-and-split` | follow |  |  |  |
| `follow-mode` | follow |  |  |  |
| `follow-scroll-down` | follow |  |  |  |
| `follow-scroll-down-window` | follow |  |  |  |
| `follow-scroll-up` | follow |  |  |  |
| `follow-scroll-up-window` | follow |  |  |  |
| `font-lock-mode` | font-core |  |  |  |
| `global-font-lock-mode` | font-core |  |  |  |
| `font-lock-after-fontify-buffer` | font-lock |  |  |  |
| `font-lock-after-unfontify-buffer` | font-lock |  |  |  |
| `font-lock-debug-fontify` | font-lock |  |  |  |
| `font-lock-fontify-block` | font-lock |  |  |  |
| `font-lock-fontify-buffer` | font-lock |  |  |  |
| `font-lock-update` | font-lock | `C-x x f` | 同じ |  |
| `footnote-mode` | footnote |  |  |  |
| `format-decode-buffer` | format |  |  |  |
| `format-decode-region` | format |  |  |  |
| `format-encode-buffer` | format |  |  |  |
| `format-encode-region` | format |  |  |  |
| `format-find-file` | format |  |  |  |
| `format-insert-file` | format |  |  |  |
| `format-write-file` | format |  |  |  |
| `forms-find-file` | forms |  |  |  |
| `forms-find-file-other-window` | forms |  |  |  |
| `forms-mode` | forms |  |  |  |
| `fortran-mode` | fortran |  |  |  |
| `fortune` | fortune |  |  |  |
| `fortune-add-fortune` | fortune |  |  |  |
| `fortune-compile` | fortune |  |  |  |
| `fortune-from-region` | fortune |  |  |  |
| `fortune-message` | fortune |  |  |  |
| `fortune-to-signature` | fortune |  |  |  |
| `auto-lower-mode` | frame |  |  |  |
| `auto-raise-mode` | frame |  |  |  |
| `blink-cursor-mode` | frame |  |  |  |
| `clone-frame` | frame | `C-x 5 c` | 同じ |  |
| `close-display-connection` | frame |  |  |  |
| `delete-other-frames` | frame | `C-x 5 1` | 同じ |  |
| `handle-delete-frame` | frame | `<delete-frame>` | 同じ |  |
| `handle-focus-in` | frame |  |  |  |
| `handle-focus-out` | frame |  |  |  |
| `handle-move-frame` | frame |  |  |  |
| `iconify-or-deiconify-frame` | frame |  |  |  |
| `make-frame` | frame |  |  |  |
| `make-frame-command` | frame | `C-x 5 2` | 同じ |  |
| `make-frame-on-current-monitor` | frame |  |  |  |
| `make-frame-on-display` | frame |  |  |  |
| `make-frame-on-monitor` | frame |  |  |  |
| `next-multiframe-window` | frame |  |  |  |
| `next-window-any-frame` | frame |  |  |  |
| `other-frame` | frame | `C-x 5 o` | 同じ |  |
| `other-frame-prefix` | frame | `C-x 5 5` | 同じ |  |
| `previous-multiframe-window` | frame |  |  |  |
| `previous-window-any-frame` | frame |  |  |  |
| `select-frame-by-name` | frame |  |  |  |
| `set-background-color` | frame |  |  |  |
| `set-border-color` | frame |  |  |  |
| `set-cursor-color` | frame |  |  |  |
| `set-foreground-color` | frame |  |  |  |
| `set-frame-font` | frame |  |  |  |
| `set-frame-name` | frame |  |  |  |
| `set-mouse-color` | frame |  |  |  |
| `suspend-frame` | frame | `C-z`, `C-x C-z` | 同じ |  |
| `toggle-frame-fullscreen` | frame | `<f11>` | 同じ |  |
| `toggle-frame-maximized` | frame | `M-<f10>`, `ESC <f10>` | 同じ |  |
| `undelete-frame` | frame | `C-x 5 u` | 同じ |  |
| `undelete-frame-mode` | frame |  |  |  |
| `window-divider-mode` | frame |  |  |  |
| `frameset-to-register` | frameset | `C-x r f` | 同じ |  |
| `fringe-mode` | fringe |  |  |  |
| `set-fringe-style` | fringe |  |  |  |
| `gdb` | gdb-mi |  |  |  |
| `gdb-enable-debug` | gdb-mi |  |  |  |
| `generic-mode` | generic |  |  |  |
| `glasses-mode` | glasses |  |  |  |
| `glyphless-display-mode` | glyphless-mode |  |  |  |
| `gnus` | gnus |  |  |  |
| `gnus-child` | gnus |  |  |  |
| `gnus-child-no-server` | gnus |  |  |  |
| `gnus-no-server` | gnus |  |  |  |
| `gnus-other-frame` | gnus |  |  |  |
| `gnus-slave` | gnus |  |  |  |
| `gnus-slave-no-server` | gnus |  |  |  |
| `gnus-agent-batch` | gnus-agent |  |  |  |
| `gnus-agent-batch-fetch` | gnus-agent |  |  |  |
| `gnus-agent-regenerate` | gnus-agent |  |  |  |
| `gnus-agentize` | gnus-agent |  |  |  |
| `gnus-child-unplugged` | gnus-agent |  |  |  |
| `gnus-plugged` | gnus-agent |  |  |  |
| `gnus-slave-unplugged` | gnus-agent |  |  |  |
| `gnus-unplugged` | gnus-agent |  |  |  |
| `gnus-bookmark-bmenu-list` | gnus-bookmark |  |  |  |
| `gnus-bookmark-jump` | gnus-bookmark |  |  |  |
| `gnus-bookmark-set` | gnus-bookmark |  |  |  |
| `gnus-cache-generate-active` | gnus-cache |  |  |  |
| `gnus-cache-generate-nov-databases` | gnus-cache |  |  |  |
| `gnus-jog-cache` | gnus-cache |  |  |  |
| `gnus-delay-article` | gnus-delay |  |  |  |
| `gnus-delay-send-queue` | gnus-delay |  |  |  |
| `turn-on-gnus-dired-mode` | gnus-dired |  |  |  |
| `gnus-draft-reminder` | gnus-draft |  |  |  |
| `gnus-face-from-file` | gnus-fun |  |  |  |
| `gnus-insert-random-x-face-header` | gnus-fun |  |  |  |
| `gnus-random-face` | gnus-fun |  |  |  |
| `gnus-random-x-face` | gnus-fun |  |  |  |
| `gnus-x-face-from-file` | gnus-fun |  |  |  |
| `gnus-treat-from-gravatar` | gnus-gravatar |  |  |  |
| `gnus-treat-mail-gravatar` | gnus-gravatar |  |  |  |
| `gnus-fetch-group` | gnus-group |  |  |  |
| `gnus-fetch-group-other-frame` | gnus-group |  |  |  |
| `gnus-read-ephemeral-emacs-bug-group` | gnus-group |  |  |  |
| `gnus-batch-score` | gnus-kill |  |  |  |
| `gnus-mailing-list-insinuate` | gnus-ml |  |  |  |
| `gnus-mailing-list-mode` | gnus-ml |  |  |  |
| `gnus-group-split-setup` | gnus-mlspl |  |  |  |
| `gnus-group-split-update` | gnus-mlspl |  |  |  |
| `gnus-button-reply` | gnus-msg |  |  |  |
| `gnus-msg-mail` | gnus-msg |  |  |  |
| `gnus-treat-from-picon` | gnus-picon |  |  |  |
| `gnus-treat-mail-picon` | gnus-picon |  |  |  |
| `gnus-treat-newsgroups-picon` | gnus-picon |  |  |  |
| `gnus-registry-initialize` | gnus-registry |  |  |  |
| `gnus-sieve-article-add-rule` | gnus-sieve |  |  |  |
| `gnus-sieve-generate` | gnus-sieve |  |  |  |
| `gnus-sieve-update` | gnus-sieve |  |  |  |
| `gnus-update-format` | gnus-spec |  |  |  |
| `go-mod-ts-mode` | go-ts-mode |  |  |  |
| `go-ts-mode` | go-ts-mode |  |  |  |
| `gomoku` | gomoku |  |  |  |
| `global-goto-address-mode` | goto-addr |  |  |  |
| `goto-address` | goto-addr |  |  |  |
| `goto-address-at-point` | goto-addr |  |  |  |
| `goto-address-mode` | goto-addr |  |  |  |
| `goto-address-prog-mode` | goto-addr |  |  |  |
| `bovine-grammar-mode` | grammar |  |  |  |
| `wisent-grammar-mode` | grammar |  |  |  |
| `grep` | grep |  |  |  |
| `grep-find` | grep |  |  |  |
| `lgrep` | grep |  |  |  |
| `rgrep` | grep |  |  |  |
| `zrgrep` | grep |  |  |  |
| `dbx` | gud |  |  |  |
| `gdb-script-mode` | gud |  |  |  |
| `gud-gdb` | gud |  |  |  |
| `gud-tooltip-mode` | gud |  |  |  |
| `guiler` | gud |  |  |  |
| `jdb` | gud |  |  |  |
| `lldb` | gud |  |  |  |
| `pdb` | gud |  |  |  |
| `perldb` | gud |  |  |  |
| `sdb` | gud |  |  |  |
| `xdb` | gud |  |  |  |
| `handwrite` | handwrite |  |  |  |
| `hanoi` | hanoi |  |  |  |
| `hanoi-unix` | hanoi |  |  |  |
| `hanoi-unix-64` | hanoi |  |  |  |
| `hashcash-insert-payment` | hashcash |  |  |  |
| `hashcash-insert-payment-async` | hashcash |  |  |  |
| `mail-add-payment` | hashcash |  |  |  |
| `mail-add-payment-async` | hashcash |  |  |  |
| `mail-check-payment` | hashcash |  |  |  |
| `heex-ts-mode` | heex-ts-mode |  |  |  |
| `cheat-sheet` | help |  |  |  |
| `describe-bindings` | help | `C-h b`, `<f1> b`, `<help> b` | `C-h b`, `<help> b` | ✓ `C-h b` |
| `describe-copying` | help | `C-h C-c`, `<f1> C-c`, `<help> C-c` | `C-h C-c`, `<help> C-c` |  |
| `describe-distribution` | help | `C-h C-o`, `<f1> C-o`, `<help> C-o` | `C-h C-o`, `<help> C-o` |  |
| `describe-gnu-project` | help | `C-h g`, `<f1> g`, `<help> g` | `C-h g`, `<help> g` |  |
| `describe-key` | help | `C-h k`, `<f1> k`, `<help> k` | `C-h k`, `<help> k` | ✓ `C-h k` |
| `describe-key-briefly` | help | `C-h c`, `<f1> c`, `<help> c` | `C-h c`, `<help> c` |  |
| `describe-minor-mode` | help |  |  |  |
| `describe-minor-mode-from-indicator` | help |  |  |  |
| `describe-minor-mode-from-symbol` | help |  |  |  |
| `describe-no-warranty` | help | `C-h C-w`, `<f1> C-w`, `<help> C-w` | `C-h C-w`, `<help> C-w` |  |
| `describe-prefix-bindings` | help |  |  |  |
| `help` | help |  |  |  |
| `help-for-help` | help | `C-h ?`, `<f1> ?`, `C-h C-h`, `<f1> C-h` ほか 8 | `C-h ?`, `C-h C-h`, `<help> ?`, `C-h <f1>` ほか 4 |  |
| `help-for-help-internal` | help |  |  |  |
| `help-quick` | help |  |  |  |
| `help-quick-toggle` | help | `C-h C-q`, `<f1> C-q`, `<help> C-q` | `C-h C-q`, `<help> C-q` |  |
| `help-quit` | help | `C-h q`, `<f1> q`, `<help> q` | `C-h q`, `<help> q` |  |
| `search-forward-help-for-help` | help | `C-h C-s`, `<f1> C-s`, `<help> C-s` | `C-h C-s`, `<help> C-s` |  |
| `temp-buffer-resize-mode` | help |  |  |  |
| `view-echo-area-messages` | help | `C-h e`, `<f1> e`, `<help> e` | `C-h e`, `<help> e` |  |
| `view-emacs-FAQ` | help | `C-h C-f`, `<f1> C-f`, `<help> C-f` | `C-h C-f`, `<help> C-f` |  |
| `view-emacs-debugging` | help | `C-h C-d`, `<f1> C-d`, `<help> C-d` | `C-h C-d`, `<help> C-d` |  |
| `view-emacs-news` | help | `C-h n`, `<f1> n`, `C-h C-n`, `<f1> C-n` ほか 2 | `C-h n`, `C-h C-n`, `<help> n`, `<help> C-n` |  |
| `view-emacs-problems` | help | `C-h C-p`, `<f1> C-p`, `<help> C-p` | `C-h C-p`, `<help> C-p` |  |
| `view-emacs-todo` | help | `C-h C-t`, `<f1> C-t`, `<help> C-t` | `C-h C-t`, `<help> C-t` |  |
| `view-external-packages` | help | `C-h C-e`, `<f1> C-e`, `<help> C-e` | `C-h C-e`, `<help> C-e` |  |
| `view-lossage` | help | `C-h l`, `<f1> l`, `<help> l` | `C-h l`, `<help> l` |  |
| `view-order-manuals` | help | `C-h RET`, `<f1> RET`, `<help> RET` | `C-h RET`, `<help> RET` |  |
| `where-is` | help | `C-h w`, `<f1> w`, `<help> w` | `C-h w`, `<help> w` |  |
| `display-local-help` | help-at-pt | `C-h .`, `<f1> .`, `<help> .` | `C-h .`, `<help> .` |  |
| `help-at-pt-cancel-timer` | help-at-pt |  |  |  |
| `help-at-pt-set-timer` | help-at-pt |  |  |  |
| `scan-buf-next-region` | help-at-pt |  |  |  |
| `scan-buf-previous-region` | help-at-pt |  |  |  |
| `describe-categories` | help-fns |  |  |  |
| `describe-command` | help-fns | `C-h x`, `<f1> x`, `<help> x` | `C-h x`, `<help> x` |  |
| `describe-face` | help-fns |  |  |  |
| `describe-function` | help-fns | `C-h f`, `<f1> f`, `<help> f` | `C-h f`, `<help> f` |  |
| `describe-keymap` | help-fns |  |  |  |
| `describe-mode` | help-fns | `C-h m`, `<f1> m`, `<help> m` | `C-h m`, `<help> m` |  |
| `describe-symbol` | help-fns | `C-h o`, `<f1> o`, `<help> o` | `C-h o`, `<help> o` |  |
| `describe-syntax` | help-fns | `C-h s`, `<f1> s`, `<help> s` | `C-h s`, `<help> s` |  |
| `describe-variable` | help-fns | `C-h v`, `<f1> v`, `<help> v` | `C-h v`, `<help> v` |  |
| `describe-widget` | help-fns |  |  |  |
| `doc-file-to-info` | help-fns |  |  |  |
| `doc-file-to-man` | help-fns |  |  |  |
| `help-find-source` | help-fns | `C-h 4 s`, `<f1> 4 s`, `<help> 4 s` | `C-h 4 s`, `<help> 4 s` |  |
| `help-make-xrefs` | help-mode |  |  |  |
| `help-mode` | help-mode |  |  |  |
| `Helper-describe-bindings` | helper |  |  |  |
| `Helper-help` | helper |  |  |  |
| `hexl-find-file` | hexl |  |  |  |
| `hexl-mode` | hexl |  |  |  |
| `hexlify-buffer` | hexl |  |  |  |
| `htmlfontify-load-rgb-file` | hfy-cmap |  |  |  |
| `global-hi-lock-mode` | hi-lock |  |  |  |
| `hi-lock-face-buffer` | hi-lock |  |  |  |
| `hi-lock-face-phrase-buffer` | hi-lock |  |  |  |
| `hi-lock-face-symbol-at-point` | hi-lock |  |  |  |
| `hi-lock-find-patterns` | hi-lock | `M-s h f` | 同じ |  |
| `hi-lock-line-face-buffer` | hi-lock |  |  |  |
| `hi-lock-mode` | hi-lock |  |  |  |
| `hi-lock-unface-buffer` | hi-lock |  |  |  |
| `hi-lock-write-interactive-patterns` | hi-lock | `M-s h w` | 同じ |  |
| `hide-ifdef-mode` | hideif |  |  |  |
| `hs-minor-mode` | hideshow |  |  |  |
| `global-highlight-changes-mode` | hilit-chg |  |  |  |
| `highlight-changes-mode` | hilit-chg |  |  |  |
| `highlight-changes-next-change` | hilit-chg |  |  |  |
| `highlight-changes-previous-change` | hilit-chg |  |  |  |
| `highlight-changes-remove-highlight` | hilit-chg |  |  |  |
| `highlight-changes-rotate-faces` | hilit-chg |  |  |  |
| `highlight-changes-visible-mode` | hilit-chg |  |  |  |
| `highlight-compare-buffers` | hilit-chg |  |  |  |
| `highlight-compare-with-file` | hilit-chg |  |  |  |
| `hippie-expand` | hippie-exp |  | `C-;`, `C-x ;` |  |
| `global-hl-line-mode` | hl-line |  |  |  |
| `hl-line-mode` | hl-line |  |  |  |
| `holidays` | holidays |  |  |  |
| `list-holidays` | holidays |  |  |  |
| `html-ts-mode` | html-ts-mode |  |  |  |
| `htmlfontify-buffer` | htmlfontify |  |  |  |
| `htmlfontify-copy-and-link-dir` | htmlfontify |  |  |  |
| `ibuffer` | ibuffer |  |  |  |
| `ibuffer-jump` | ibuffer |  |  |  |
| `ibuffer-list-buffers` | ibuffer |  |  |  |
| `ibuffer-other-window` | ibuffer |  |  |  |
| `icalendar-export-file` | icalendar |  |  |  |
| `icalendar-export-region` | icalendar |  |  |  |
| `icalendar-import-buffer` | icalendar |  |  |  |
| `icalendar-import-file` | icalendar |  |  |  |
| `fido-mode` | icomplete |  |  |  |
| `fido-vertical-mode` | icomplete |  |  |  |
| `icomplete-mode` | icomplete |  |  |  |
| `icomplete-vertical-mode` | icomplete |  |  |  |
| `icon-mode` | icon |  |  |  |
| `describe-icon` | icons |  |  |  |
| `idlwave-shell` | idlw-shell |  |  |  |
| `idlwave-mode` | idlwave |  |  |  |
| `ido-dired` | ido |  |  |  |
| `ido-dired-other-frame` | ido |  |  |  |
| `ido-dired-other-window` | ido |  |  |  |
| `ido-display-buffer` | ido |  |  |  |
| `ido-display-buffer-other-frame` | ido |  |  |  |
| `ido-display-file` | ido |  |  |  |
| `ido-find-alternate-file` | ido |  |  |  |
| `ido-find-alternate-file-other-window` | ido |  |  |  |
| `ido-find-file` | ido |  |  |  |
| `ido-find-file-in-dir` | ido |  |  |  |
| `ido-find-file-other-frame` | ido |  |  |  |
| `ido-find-file-other-window` | ido |  |  |  |
| `ido-find-file-read-only` | ido |  |  |  |
| `ido-find-file-read-only-other-frame` | ido |  |  |  |
| `ido-find-file-read-only-other-window` | ido |  |  |  |
| `ido-insert-buffer` | ido |  |  |  |
| `ido-insert-file` | ido |  |  |  |
| `ido-kill-buffer` | ido |  |  |  |
| `ido-mode` | ido |  |  |  |
| `ido-switch-buffer` | ido |  |  |  |
| `ido-switch-buffer-other-frame` | ido |  |  |  |
| `ido-switch-buffer-other-window` | ido |  |  |  |
| `ido-write-file` | ido |  |  |  |
| `ielm` | ielm |  |  |  |
| `iimage-mode` | iimage |  |  |  |
| `image-decrease-size` | image |  |  |  |
| `image-flip-horizontally` | image |  |  |  |
| `image-flip-vertically` | image |  |  |  |
| `image-increase-size` | image |  |  |  |
| `image-mouse-decrease-size` | image |  |  |  |
| `image-mouse-increase-size` | image |  |  |  |
| `image-rotate` | image |  |  |  |
| `image-save` | image |  |  |  |
| `image-crop` | image-crop |  |  |  |
| `image-cut` | image-crop |  |  |  |
| `image-dired-dired-with-window-configuration` | image-dired |  |  |  |
| `image-dired-display-thumbs` | image-dired |  |  |  |
| `image-dired-show-all-from-dir` | image-dired |  |  |  |
| `image-dired-dired-display-external` | image-dired-dired |  |  |  |
| `image-dired-dired-display-image` | image-dired-dired |  |  |  |
| `image-dired-dired-toggle-marked-thumbs` | image-dired-dired |  |  |  |
| `image-dired-display-thumb` | image-dired-dired |  |  |  |
| `image-dired-display-thumbs-append` | image-dired-dired |  |  |  |
| `image-dired-jump-thumbnail-buffer` | image-dired-dired |  |  |  |
| `image-dired-mark-tagged-files` | image-dired-dired |  |  |  |
| `image-dired-minor-mode` | image-dired-dired |  |  |  |
| `image-dired-delete-tag` | image-dired-tags |  |  |  |
| `image-dired-dired-comment-files` | image-dired-tags |  |  |  |
| `image-dired-dired-edit-comment-and-tags` | image-dired-tags |  |  |  |
| `image-dired-tag-files` | image-dired-tags |  |  |  |
| `auto-image-file-mode` | image-file |  |  |  |
| `image-minor-mode` | image-mode |  |  |  |
| `image-mode` | image-mode |  |  |  |
| `imenu` | imenu | `M-g i` |  |  |
| `imenu-add-menubar-index` | imenu |  |  |  |
| `imenu-add-to-menubar` | imenu |  |  |  |
| `indian-2-column-to-ucs-region` | ind-util |  |  |  |
| `indian-compose-region` | ind-util |  |  |  |
| `beginning-of-line-text` | indent |  |  |  |
| `decrease-left-margin` | indent |  |  |  |
| `decrease-right-margin` | indent |  |  |  |
| `edit-tab-stops` | indent |  |  |  |
| `edit-tab-stops-note-changes` | indent |  |  |  |
| `increase-left-margin` | indent |  |  |  |
| `increase-right-margin` | indent |  |  |  |
| `indent-according-to-mode` | indent |  |  |  |
| `indent-for-tab-command` | indent | `TAB` |  | ✓ `TAB`, `C-i` |
| `indent-region` | indent | `C-M-\` | 同じ | ✓ `C-M-\` |
| `indent-relative` | indent |  |  |  |
| `indent-relative-first-indent-point` | indent |  |  |  |
| `indent-relative-maybe` | indent |  |  |  |
| `indent-rigidly` | indent | `C-x TAB` | 同じ |  |
| `indent-rigidly-left` | indent |  |  |  |
| `indent-rigidly-left-to-tab-stop` | indent |  |  |  |
| `indent-rigidly-right` | indent |  |  |  |
| `indent-rigidly-right-to-tab-stop` | indent |  |  |  |
| `move-to-left-margin` | indent |  |  |  |
| `move-to-tab-stop` | indent |  |  |  |
| `set-left-margin` | indent |  |  |  |
| `set-right-margin` | indent |  |  |  |
| `tab-to-tab-stop` | indent | `M-i` | 同じ |  |
| `kill-ring-deindent-mode` | indent-aux |  |  |  |
| `inferior-lisp` | inf-lisp |  |  |  |
| `lisp-eval-defun` | inf-lisp |  |  |  |
| `Info-directory` | info |  |  |  |
| `Info-goto-emacs-command-node` | info | `C-h F`, `<f1> F`, `<help> F` | `C-h F`, `<help> F` |  |
| `Info-goto-emacs-key-command-node` | info | `C-h K`, `<f1> K`, `<help> K` | `C-h K`, `<help> K` |  |
| `Info-index` | info |  |  |  |
| `Info-mode` | info |  |  |  |
| `Info-on-current-buffer` | info |  |  |  |
| `Info-speedbar-browser` | info |  |  |  |
| `info` | info | `C-h i`, `<f1> i`, `<help> i` | `C-h i`, `<help> i` |  |
| `info-apropos` | info |  |  |  |
| `info-display-manual` | info | `C-h R`, `<f1> R`, `<help> R` | `C-h R`, `<help> R` |  |
| `info-emacs-bug` | info |  |  |  |
| `info-emacs-manual` | info | `C-h r`, `<f1> r`, `<help> r` | `C-h r`, `<help> r` |  |
| `info-finder` | info |  |  |  |
| `info-other-window` | info | `C-h 4 i`, `<f1> 4 i`, `<help> 4 i` | `C-h 4 i`, `<help> 4 i` |  |
| `info-complete-file` | info-look |  |  |  |
| `info-complete-symbol` | info-look |  |  |  |
| `info-lookup-file` | info-look |  |  |  |
| `info-lookup-reset` | info-look |  |  |  |
| `info-lookup-symbol` | info-look | `C-h S`, `<f1> S`, `<help> S` | `C-h S`, `C-h TAB`, `<help> S`, `<help> TAB` |  |
| `info-xref-check` | info-xref |  |  |  |
| `info-xref-check-all` | info-xref |  |  |  |
| `info-xref-check-all-custom` | info-xref |  |  |  |
| `info-xref-docstrings` | info-xref |  |  |  |
| `Info-split` | informat |  |  |  |
| `Info-tagify` | informat |  |  |  |
| `Info-validate` | informat |  |  |  |
| `isearch-abort` | isearch |  |  |  |
| `isearch-backward` | isearch | `C-r` | 同じ | ✓ `C-r` |
| `isearch-backward-regexp` | isearch | `C-M-r` | 同じ | ✓ `C-M-r` |
| `isearch-beginning-of-buffer` | isearch |  |  |  |
| `isearch-cancel` | isearch |  |  |  |
| `isearch-char-by-name` | isearch |  |  |  |
| `isearch-complete` | isearch |  |  |  |
| `isearch-complete-edit` | isearch |  |  |  |
| `isearch-del-char` | isearch |  |  |  |
| `isearch-delete-char` | isearch |  |  |  |
| `isearch-describe-bindings` | isearch |  |  |  |
| `isearch-describe-key` | isearch |  |  |  |
| `isearch-describe-mode` | isearch |  |  |  |
| `isearch-edit-string` | isearch |  |  |  |
| `isearch-emoji-by-name` | isearch |  |  |  |
| `isearch-end-of-buffer` | isearch |  |  |  |
| `isearch-exit` | isearch |  |  |  |
| `isearch-fold-quotes-mode` | isearch |  |  |  |
| `isearch-forward` | isearch | `C-s` | 同じ | ✓ `C-s` |
| `isearch-forward-exit-minibuffer` | isearch |  |  |  |
| `isearch-forward-regexp` | isearch | `C-M-s` | 同じ | ✓ `C-M-s` |
| `isearch-forward-symbol` | isearch | `M-s _` | 同じ |  |
| `isearch-forward-symbol-at-point` | isearch | `M-s .` | 同じ |  |
| `isearch-forward-thing-at-point` | isearch | `M-s M-.` | 同じ |  |
| `isearch-forward-word` | isearch | `M-s w` | 同じ |  |
| `isearch-help-for-help` | isearch |  |  |  |
| `isearch-help-for-help-internal` | isearch |  |  |  |
| `isearch-highlight-lines-matching-regexp` | isearch |  |  |  |
| `isearch-highlight-regexp` | isearch |  |  |  |
| `isearch-menu-bar-map` | isearch |  |  |  |
| `isearch-mode-help` | isearch |  |  |  |
| `isearch-mouse-2` | isearch |  |  |  |
| `isearch-nonincremental-exit-minibuffer` | isearch |  |  |  |
| `isearch-occur` | isearch |  |  |  |
| `isearch-printing-char` | isearch |  |  |  |
| `isearch-query-replace` | isearch |  |  |  |
| `isearch-query-replace-regexp` | isearch |  |  |  |
| `isearch-quote-char` | isearch |  |  |  |
| `isearch-repeat-backward` | isearch |  |  |  |
| `isearch-repeat-forward` | isearch |  |  |  |
| `isearch-reverse-exit-minibuffer` | isearch |  |  |  |
| `isearch-ring-advance` | isearch |  |  |  |
| `isearch-ring-retreat` | isearch |  |  |  |
| `isearch-tmm-menubar` | isearch |  |  |  |
| `isearch-toggle-case-fold` | isearch |  |  |  |
| `isearch-toggle-char-fold` | isearch |  |  |  |
| `isearch-toggle-invisible` | isearch |  |  |  |
| `isearch-toggle-lax-whitespace` | isearch |  |  |  |
| `isearch-toggle-regexp` | isearch |  |  |  |
| `isearch-toggle-symbol` | isearch |  |  |  |
| `isearch-toggle-word` | isearch |  |  |  |
| `isearch-xterm-paste` | isearch |  |  |  |
| `isearch-yank-char` | isearch |  |  |  |
| `isearch-yank-char-in-minibuffer` | isearch |  |  |  |
| `isearch-yank-from-kill-ring` | isearch |  |  |  |
| `isearch-yank-kill` | isearch |  |  |  |
| `isearch-yank-line` | isearch |  |  |  |
| `isearch-yank-pop` | isearch |  |  |  |
| `isearch-yank-pop-only` | isearch |  |  |  |
| `isearch-yank-symbol-or-char` | isearch |  |  |  |
| `isearch-yank-until-char` | isearch |  |  |  |
| `isearch-yank-word` | isearch |  |  |  |
| `isearch-yank-word-or-char` | isearch |  |  |  |
| `isearch-yank-x-selection` | isearch |  |  |  |
| `lazy-highlight-cleanup` | isearch |  |  |  |
| `word-search-backward` | isearch |  |  |  |
| `word-search-backward-lax` | isearch |  |  |  |
| `word-search-forward` | isearch |  |  |  |
| `word-search-forward-lax` | isearch |  |  |  |
| `isearch-toggle-input-method` | isearch-x |  |  |  |
| `isearch-toggle-specified-input-method` | isearch-x |  |  |  |
| `isearch-transient-input-method` | isearch-x |  |  |  |
| `isearchb-activate` | isearchb |  |  |  |
| `iso-cvt-define-menu` | iso-cvt |  |  |  |
| `iso-cvt-read-only` | iso-cvt |  |  |  |
| `iso-cvt-write-only` | iso-cvt |  |  |  |
| `iso-german` | iso-cvt |  |  |  |
| `iso-gtex2iso` | iso-cvt |  |  |  |
| `iso-iso2duden` | iso-cvt |  |  |  |
| `iso-iso2gtex` | iso-cvt |  |  |  |
| `iso-iso2sgml` | iso-cvt |  |  |  |
| `iso-iso2tex` | iso-cvt |  |  |  |
| `iso-sgml2iso` | iso-cvt |  |  |  |
| `iso-spanish` | iso-cvt |  |  |  |
| `iso-tex2iso` | iso-cvt |  |  |  |
| `iso-transl-set-language` | iso-transl |  |  |  |
| `ispell` | ispell |  |  |  |
| `ispell-buffer` | ispell |  |  |  |
| `ispell-buffer-with-debug` | ispell |  |  |  |
| `ispell-change-dictionary` | ispell |  |  |  |
| `ispell-comment-or-string-at-point` | ispell |  |  |  |
| `ispell-comments-and-strings` | ispell |  |  |  |
| `ispell-complete-word` | ispell |  |  |  |
| `ispell-complete-word-interior-frag` | ispell |  |  |  |
| `ispell-continue` | ispell |  |  |  |
| `ispell-kill-ispell` | ispell |  |  |  |
| `ispell-message` | ispell |  |  |  |
| `ispell-minor-mode` | ispell |  |  |  |
| `ispell-pdict-save` | ispell |  |  |  |
| `ispell-region` | ispell |  |  |  |
| `ispell-word` | ispell | `M-$` | 同じ |  |
| `iswitchb-mode` | iswitchb |  |  |  |
| `japanese-hankaku-region` | japan-util |  |  |  |
| `japanese-hiragana-region` | japan-util |  |  |  |
| `japanese-katakana-region` | japan-util |  |  |  |
| `japanese-zenkaku-region` | japan-util |  |  |  |
| `java-ts-mode` | java-ts-mode |  |  |  |
| `jit-lock-debug-mode` | jit-lock |  |  |  |
| `auto-compression-mode` | jka-cmpr-hook |  |  |  |
| `js-base-mode` | js |  |  |  |
| `js-json-mode` | js |  |  |  |
| `js-jsx-mode` | js |  |  |  |
| `js-mode` | js |  |  |  |
| `js-ts-mode` | js |  |  |  |
| `json-ts-mode` | json-ts-mode |  |  |  |
| `keymap-global-lookup` | keymap |  |  |  |
| `keymap-global-set` | keymap |  |  |  |
| `keymap-global-unset` | keymap |  |  |  |
| `keymap-local-set` | keymap |  |  |  |
| `keymap-local-unset` | keymap |  |  |  |
| `kkc-region` | kkc |  |  |  |
| `kmacro-call-macro` | kmacro |  |  |  |
| `kmacro-end-and-call-macro` | kmacro | `C-x e` | 同じ |  |
| `kmacro-end-call-mouse` | kmacro | `S-<mouse-3>` | 同じ |  |
| `kmacro-end-macro` | kmacro | `C-x )` | 同じ |  |
| `kmacro-end-or-call-macro` | kmacro | `<f4>` | 同じ |  |
| `kmacro-keymap` | kmacro |  |  |  |
| `kmacro-name-last-macro` | kmacro | `C-x C-k n` | 同じ |  |
| `kmacro-start-macro` | kmacro | `C-x (`, `C-x C-k s`, `C-x C-k C-s` | 同じ |  |
| `kmacro-start-macro-or-insert-counter` | kmacro | `<f3>` | 同じ |  |
| `list-keyboard-macros` | kmacro |  |  |  |
| `lao-compose-region` | lao-util |  |  |  |
| `latin1-display-ucs-per-lynx` | latin1-disp |  |  |  |
| `ld-script-mode` | ld-script |  |  |  |
| `less-css-mode` | less-css-mode |  |  |  |
| `life` | life |  |  |  |
| `backward-kill-sexp` | lisp | `C-M-<delete>`, `ESC C-<delete>`, `C-M-<backspace>`, `ESC C-<backspace>` | 同じ |  |
| `backward-list` | lisp | `C-M-p` | 同じ |  |
| `backward-sexp` | lisp | `C-M-b`, `C-M-<left>`, `ESC C-<left>` | 同じ | ✓ `C-M-b` |
| `backward-up-list` | lisp | `C-M-u`, `C-M-<up>`, `ESC C-<up>` | 同じ | ✓ `C-M-u` |
| `beginning-of-defun` | lisp | `C-M-a`, `C-M-<home>`, `ESC C-<home>` | 同じ | ✓ `C-M-a` |
| `beginning-of-defun-comments` | lisp |  |  |  |
| `beginning-of-defun-raw` | lisp |  |  |  |
| `check-parens` | lisp |  |  |  |
| `delete-pair` | lisp |  |  |  |
| `down-list` | lisp | `C-M-d`, `C-M-<down>`, `ESC C-<down>` | 同じ | ✓ `C-M-d` |
| `end-of-defun` | lisp | `C-M-e`, `C-M-<end>`, `ESC C-<end>` | 同じ | ✓ `C-M-e` |
| `forward-list` | lisp | `C-M-n` | 同じ |  |
| `forward-sexp` | lisp | `C-M-f`, `C-M-<right>`, `ESC C-<right>` | 同じ | ✓ `C-M-f` |
| `insert-pair` | lisp |  |  |  |
| `insert-parentheses` | lisp | `M-(` | 同じ |  |
| `kill-backward-up-list` | lisp |  |  |  |
| `kill-sexp` | lisp | `C-M-k` | 同じ | ✓ `C-M-k` |
| `lisp-complete-symbol` | lisp |  |  |  |
| `mark-defun` | lisp | `C-M-h` | 同じ | ✓ `C-M-h` |
| `mark-sexp` | lisp | `C-M-@`, `C-M-SPC` | 同じ | ✓ `C-M-SPC` |
| `move-past-close-and-reindent` | lisp | `M-)` | 同じ |  |
| `narrow-to-defun` | lisp | `C-x n d` | 同じ |  |
| `raise-sexp` | lisp |  |  |  |
| `up-list` | lisp |  |  | ✓ |
| `common-lisp-mode` | lisp-mode |  |  |  |
| `indent-code-rigidly` | lisp-mode |  |  |  |
| `indent-pp-sexp` | lisp-mode |  |  |  |
| `indent-sexp` | lisp-mode |  |  | ✓ `C-M-q` |
| `lisp-data-mode` | lisp-mode |  |  |  |
| `lisp-fill-paragraph` | lisp-mode |  |  |  |
| `lisp-indent-line` | lisp-mode |  |  |  |
| `lisp-mode` | lisp-mode |  |  |  |
| `lisp-mode-menu` | lisp-mode |  |  |  |
| `LaTeX-mode` | loaddefs |  |  |  |
| `TeX-mode` | loaddefs |  |  |  |
| `ange-ftp-re-read-dir` | loaddefs |  |  |  |
| `bookmark-locate` | loaddefs |  |  |  |
| `browse-web` | loaddefs |  |  |  |
| `cancel-debug-watch` | loaddefs |  |  |  |
| `capitalized-words-mode` | loaddefs |  |  |  |
| `close-rectangle` | loaddefs |  |  |  |
| `command-apropos` | loaddefs |  |  |  |
| `customize-changed-options` | loaddefs |  |  |  |
| `customize-variable` | loaddefs |  |  |  |
| `customize-variable-other-window` | loaddefs |  |  |  |
| `debug-watch` | loaddefs |  |  |  |
| `delphi-mode` | loaddefs |  |  |  |
| `describe-editorconfig-properties` | loaddefs |  |  |  |
| `display-time-world` | loaddefs |  |  |  |
| `ebnf-despool` | loaddefs |  |  |  |
| `ebuffers` | loaddefs |  |  |  |
| `ebuffers3` | loaddefs |  |  |  |
| `edebug-defun` | loaddefs |  |  |  |
| `ediff` | loaddefs |  |  |  |
| `ediff-merge` | loaddefs |  |  |  |
| `ediff-merge-with-ancestor` | loaddefs |  |  |  |
| `ediff3` | loaddefs |  |  |  |
| `edir-merge-revisions` | loaddefs |  |  |  |
| `edir-merge-revisions-with-ancestor` | loaddefs |  |  |  |
| `edir-revisions` | loaddefs |  |  |  |
| `edirs` | loaddefs |  |  |  |
| `edirs-merge` | loaddefs |  |  |  |
| `edirs-merge-with-ancestor` | loaddefs |  |  |  |
| `edirs3` | loaddefs |  |  |  |
| `edit-bookmarks` | loaddefs |  |  |  |
| `edit-picture` | loaddefs |  |  |  |
| `eglot-update` | loaddefs |  |  |  |
| `epatch` | loaddefs |  |  |  |
| `epatch-buffer` | loaddefs |  |  |  |
| `erc-select` | loaddefs |  |  |  |
| `eregistry` | loaddefs |  |  |  |
| `erevision` | loaddefs |  |  |  |
| `ert` | loaddefs |  |  |  |
| `etc-authors-mode` | loaddefs |  |  |  |
| `ff-find-related-file` | loaddefs |  |  |  |
| `ff-mouse-find-other-file` | loaddefs |  |  |  |
| `ff-mouse-find-other-file-other-window` | loaddefs |  |  |  |
| `ffap` | loaddefs |  |  |  |
| `find-grep` | loaddefs |  |  |  |
| `gnus-batch-kill` | loaddefs |  |  |  |
| `help-xref-interned` | loaddefs |  |  |  |
| `highlight-lines-matching-regexp` | loaddefs | `M-s h l` | 同じ |  |
| `highlight-phrase` | loaddefs | `M-s h p` | 同じ |  |
| `highlight-regexp` | loaddefs | `M-s h r` | 同じ |  |
| `highlight-symbol-at-point` | loaddefs | `M-s h .` | 同じ |  |
| `holiday-list` | loaddefs |  |  |  |
| `image-dired` | loaddefs |  |  |  |
| `image-dired-setup-dired-keybindings` | loaddefs |  |  |  |
| `inhibit-remote-files` | loaddefs |  |  |  |
| `irc` | loaddefs |  |  |  |
| `javascript-mode` | loaddefs |  |  |  |
| `kmacro-menu` | loaddefs |  |  |  |
| `list-bookmarks` | loaddefs |  |  |  |
| `manual-entry` | loaddefs |  |  |  |
| `modula-2-mode` | loaddefs |  |  |  |
| `name-last-kbd-macro` | loaddefs |  |  |  |
| `next-file` | loaddefs |  |  |  |
| `outlineify-sticky` | loaddefs |  |  |  |
| `outlinify-sticky` | loaddefs |  |  |  |
| `package-list-packages` | loaddefs |  |  |  |
| `pending-delete-mode` | loaddefs |  |  |  |
| `plain-TeX-mode` | loaddefs |  |  |  |
| `pop-tag-mark` | loaddefs |  |  |  |
| `recentf` | loaddefs |  |  |  |
| `regexp-builder` | loaddefs |  |  |  |
| `replace-rectangle` | loaddefs |  |  |  |
| `run-lisp` | loaddefs |  |  |  |
| `run-octave` | loaddefs |  |  |  |
| `rzgrep` | loaddefs |  |  |  |
| `shell-script-mode` | loaddefs |  |  |  |
| `shortdoc` | loaddefs |  |  |  |
| `speedbar` | loaddefs |  |  |  |
| `srt-mode` | loaddefs |  |  |  |
| `toggle-option` | loaddefs |  |  |  |
| `trace-function` | loaddefs |  |  |  |
| `tramp-unload-tramp` | loaddefs |  |  |  |
| `tumme` | loaddefs |  |  |  |
| `unhighlight-regexp` | loaddefs | `M-s h u` | 同じ |  |
| `url-mail` | loaddefs |  |  |  |
| `vc-resolve-conflicts` | loaddefs |  |  |  |
| `vc-update` | loaddefs | `C-x v +` | 同じ |  |
| `xml-mode` | loaddefs |  |  |  |
| `xref-pop-marker-stack` | loaddefs |  |  |  |
| `zone-mode` | loaddefs |  |  |  |
| `unload-feature` | loadhist |  |  |  |
| `locate` | locate |  |  |  |
| `locate-with-filter` | locate |  |  |  |
| `log-view-mode` | log-view |  |  |  |
| `lpr-buffer` | lpr |  |  |  |
| `lpr-region` | lpr |  |  |  |
| `print-buffer` | lpr |  |  |  |
| `print-region` | lpr |  |  |  |
| `lua-ts-inferior-lua` | lua-ts-mode |  |  |  |
| `lua-ts-mode` | lua-ts-mode |  |  |  |
| `lunar-phases` | lunar |  |  |  |
| `m4-mode` | m4-mode |  |  |  |
| `apply-macro-to-region-lines` | macros | `C-x C-k r` | 同じ |  |
| `insert-kbd-macro` | macros |  |  |  |
| `kbd-macro-query` | macros | `C-x q`, `C-x C-k q` | 同じ |  |
| `what-domain` | mail-extr |  |  |  |
| `mail-quote-printable-region` | mail-utils |  |  |  |
| `mail-unquote-printable-region` | mail-utils |  |  |  |
| `define-mail-abbrev` | mailabbrev |  |  |  |
| `mail-abbrevs-mode` | mailabbrev |  |  |  |
| `define-mail-alias` | mailalias |  |  |  |
| `expand-mail-aliases` | mailalias |  |  |  |
| `mairix-edit-saved-searches` | mairix |  |  |  |
| `mairix-edit-saved-searches-customize` | mairix |  |  |  |
| `mairix-search` | mairix |  |  |  |
| `mairix-search-from-this-article` | mairix |  |  |  |
| `mairix-search-thread-this-article` | mairix |  |  |  |
| `mairix-update-database` | mairix |  |  |  |
| `mairix-use-saved-search` | mairix |  |  |  |
| `mairix-widget-search` | mairix |  |  |  |
| `mairix-widget-search-based-on-article` | mairix |  |  |  |
| `makefile-automake-mode` | make-mode |  |  |  |
| `makefile-bsdmake-mode` | make-mode |  |  |  |
| `makefile-gmake-mode` | make-mode |  |  |  |
| `makefile-imake-mode` | make-mode |  |  |  |
| `makefile-makepp-mode` | make-mode |  |  |  |
| `makefile-mode` | make-mode |  |  |  |
| `man` | man |  |  |  |
| `man-follow` | man |  |  |  |
| `master-mode` | master |  |  |  |
| `minibuffer-depth-indicate-mode` | mb-depth |  |  |  |
| `memory-report` | memory-report |  |  |  |
| `buffer-menu-open` | menu-bar | `C-<f10>` | 同じ |  |
| `clipboard-kill-region` | menu-bar |  | `S-DEL` |  |
| `clipboard-kill-ring-save` | menu-bar |  | `C-<insert>` |  |
| `clipboard-yank` | menu-bar |  | `S-<insert>` |  |
| `elisp-index-search` | menu-bar |  |  |  |
| `emacs-index-search` | menu-bar |  |  |  |
| `help-with-tutorial-spec-language` | menu-bar |  |  |  |
| `kill-this-buffer` | menu-bar |  |  |  |
| `menu-bar--display-line-numbers-mode-absolute` | menu-bar |  |  |  |
| `menu-bar--display-line-numbers-mode-none` | menu-bar |  |  |  |
| `menu-bar--display-line-numbers-mode-relative` | menu-bar |  |  |  |
| `menu-bar--display-line-numbers-mode-visual` | menu-bar |  |  |  |
| `menu-bar--toggle-truncate-long-lines` | menu-bar |  |  |  |
| `menu-bar--visual-line-mode-enable` | menu-bar |  |  |  |
| `menu-bar--wrap-long-lines-window-edge` | menu-bar |  |  |  |
| `menu-bar-bottom-and-right-window-divider` | menu-bar |  |  |  |
| `menu-bar-bottom-window-divider` | menu-bar |  |  |  |
| `menu-bar-enable-clipboard` | menu-bar |  |  |  |
| `menu-bar-left-scroll-bar` | menu-bar |  |  |  |
| `menu-bar-mode` | menu-bar |  |  |  |
| `menu-bar-no-scroll-bar` | menu-bar |  |  |  |
| `menu-bar-no-window-divider` | menu-bar |  |  |  |
| `menu-bar-open` | menu-bar | `<f10>` |  |  |
| `menu-bar-open-mouse` | menu-bar |  |  |  |
| `menu-bar-options-save` | menu-bar |  |  |  |
| `menu-bar-read-lispintro` | menu-bar |  |  |  |
| `menu-bar-read-lispref` | menu-bar |  |  |  |
| `menu-bar-read-mail` | menu-bar |  |  |  |
| `menu-bar-right-scroll-bar` | menu-bar |  |  |  |
| `menu-bar-right-window-divider` | menu-bar |  |  |  |
| `menu-bar-select-buffer` | menu-bar |  |  |  |
| `menu-bar-select-yank` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-ind-box` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-ind-customize` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-ind-left` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-ind-mixed` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-ind-none` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-ind-right` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-menu-customize` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-menu-customize-disable` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-menu-customize-left` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-menu-customize-reset` | menu-bar |  |  |  |
| `menu-bar-showhide-fringe-menu-customize-right` | menu-bar |  |  |  |
| `menu-bar-showhide-tool-bar-menu-customize-disable` | menu-bar |  |  |  |
| `menu-bar-showhide-tool-bar-menu-customize-enable-bottom` | menu-bar |  |  |  |
| `menu-bar-showhide-tool-bar-menu-customize-enable-left` | menu-bar |  |  |  |
| `menu-bar-showhide-tool-bar-menu-customize-enable-right` | menu-bar |  |  |  |
| `menu-bar-showhide-tool-bar-menu-customize-enable-top` | menu-bar |  |  |  |
| `menu-bar-window-divider-customize` | menu-bar |  |  |  |
| `menu-find-file-existing` | menu-bar |  |  |  |
| `menu-set-font` | menu-bar |  |  |  |
| `nonincremental-re-search-backward` | menu-bar |  |  |  |
| `nonincremental-re-search-forward` | menu-bar |  |  |  |
| `nonincremental-repeat-search-backward` | menu-bar |  |  |  |
| `nonincremental-repeat-search-forward` | menu-bar |  |  |  |
| `nonincremental-search-backward` | menu-bar |  |  |  |
| `nonincremental-search-forward` | menu-bar |  |  |  |
| `search-emacs-glossary` | menu-bar |  |  |  |
| `toggle-case-fold-search` | menu-bar |  |  |  |
| `toggle-debug-on-error` | menu-bar |  | `<f2>` |  |
| `toggle-debug-on-quit` | menu-bar |  |  |  |
| `toggle-indicate-empty-lines` | menu-bar |  |  |  |
| `toggle-menu-bar-mode-from-frame` | menu-bar |  |  |  |
| `toggle-save-desktop-globally` | menu-bar |  |  |  |
| `toggle-save-place-globally` | menu-bar |  |  |  |
| `toggle-uniquify-buffer-names` | menu-bar |  |  |  |
| `toggle-use-system-font` | menu-bar |  |  |  |
| `message-bold-region` | message |  |  |  |
| `message-bounce` | message |  |  |  |
| `message-cancel-news` | message |  |  |  |
| `message-followup` | message |  |  |  |
| `message-forward` | message |  |  |  |
| `message-goto-eoh` | message |  |  |  |
| `message-insinuate-rmail` | message |  |  |  |
| `message-mail` | message |  |  |  |
| `message-mail-other-frame` | message |  |  |  |
| `message-mail-other-window` | message |  |  |  |
| `message-mailto` | message |  |  |  |
| `message-mode` | message |  |  |  |
| `message-news` | message |  |  |  |
| `message-news-other-frame` | message |  |  |  |
| `message-news-other-window` | message |  |  |  |
| `message-recover` | message |  |  |  |
| `message-reply` | message |  |  |  |
| `message-resend` | message |  |  |  |
| `message-supersede` | message |  |  |  |
| `message-unbold-region` | message |  |  |  |
| `message-wide-reply` | message |  |  |  |
| `metafont-mode` | meta-mode |  |  |  |
| `metapost-mode` | meta-mode |  |  |  |
| `mh-fully-kill-draft` | mh-comp |  |  |  |
| `mh-send-letter` | mh-comp |  |  |  |
| `mh-smail` | mh-comp |  |  |  |
| `mh-smail-other-window` | mh-comp |  |  |  |
| `mh-version` | mh-e |  |  |  |
| `mh-folder-mode` | mh-folder |  |  |  |
| `mh-nmail` | mh-folder |  |  |  |
| `mh-rmail` | mh-folder |  |  |  |
| `mhtml-mode` | mhtml-mode |  |  |  |
| `clean-buffer-list` | midnight |  |  |  |
| `midnight-mode` | midnight |  |  |  |
| `minibuffer-electric-default-mode` | minibuf-eldef |  |  |  |
| `completion-at-point` | minibuffer |  |  |  |
| `completion-help-at-point` | minibuffer |  |  |  |
| `completion-in-region-mode` | minibuffer |  |  |  |
| `delete-minibuffer-contents` | minibuffer |  |  |  |
| `exit-minibuffer` | minibuffer |  |  |  |
| `minibuffer-beginning-of-buffer` | minibuffer |  |  |  |
| `minibuffer-choose-completion` | minibuffer |  |  |  |
| `minibuffer-choose-completion-or-exit` | minibuffer |  |  |  |
| `minibuffer-complete` | minibuffer |  |  |  |
| `minibuffer-complete-and-exit` | minibuffer |  |  |  |
| `minibuffer-complete-defaults` | minibuffer |  |  |  |
| `minibuffer-complete-history` | minibuffer |  |  |  |
| `minibuffer-complete-word` | minibuffer |  |  |  |
| `minibuffer-completion-help` | minibuffer |  |  |  |
| `minibuffer-force-complete` | minibuffer |  |  |  |
| `minibuffer-force-complete-and-exit` | minibuffer |  |  |  |
| `minibuffer-hide-completions` | minibuffer |  |  |  |
| `minibuffer-inactive-mode` | minibuffer |  |  |  |
| `minibuffer-insert-file-name-at-point` | minibuffer |  |  |  |
| `minibuffer-next-completion` | minibuffer |  |  |  |
| `minibuffer-next-line-completion` | minibuffer |  |  |  |
| `minibuffer-previous-completion` | minibuffer |  |  |  |
| `minibuffer-previous-line-completion` | minibuffer |  |  |  |
| `minibuffer-recenter-top-bottom` | minibuffer |  |  |  |
| `minibuffer-regexp-mode` | minibuffer |  |  |  |
| `minibuffer-scroll-down-command` | minibuffer |  |  |  |
| `minibuffer-scroll-other-window` | minibuffer |  |  |  |
| `minibuffer-scroll-other-window-down` | minibuffer |  |  |  |
| `minibuffer-scroll-up-command` | minibuffer |  |  |  |
| `self-insert-and-exit` | minibuffer |  |  |  |
| `backward-to-word` | misc |  |  |  |
| `butterfly` | misc |  |  |  |
| `copy-from-above-command` | misc |  |  |  |
| `duplicate-dwim` | misc |  |  |  |
| `duplicate-line` | misc |  |  |  |
| `forward-to-word` | misc |  |  |  |
| `list-dynamic-libraries` | misc |  |  |  |
| `mark-beginning-of-buffer` | misc |  |  |  |
| `mark-end-of-buffer` | misc |  |  |  |
| `upcase-char` | misc |  |  |  |
| `zap-up-to-char` | misc |  |  |  |
| `multi-file-replace-regexp-as-diff` | misearch |  |  |  |
| `multi-isearch-buffers` | misearch |  |  |  |
| `multi-isearch-buffers-regexp` | misearch |  |  |  |
| `multi-isearch-files` | misearch |  |  |  |
| `multi-isearch-files-regexp` | misearch |  |  |  |
| `replace-regexp-as-diff` | misearch |  |  |  |
| `mixal-mode` | mixal-mode |  |  |  |
| `mml-attach-file` | mml |  |  |  |
| `m2-mode` | modula2 |  |  |  |
| `denato-region` | morse |  |  |  |
| `morse-region` | morse |  |  |  |
| `nato-region` | morse |  |  |  |
| `unmorse-region` | morse |  |  |  |
| `context-menu-mode` | mouse |  |  |  |
| `context-menu-open` | mouse | `S-<f10>` | 同じ |  |
| `minor-mode-menu-from-indicator` | mouse |  |  |  |
| `mouse-appearance-menu` | mouse | `S-<down-mouse-1>` | 同じ |  |
| `mouse-buffer-menu` | mouse | `C-<down-mouse-1>` | 同じ |  |
| `mouse-delete-other-windows` | mouse |  |  |  |
| `mouse-delete-window` | mouse |  |  |  |
| `mouse-drag-and-drop-region` | mouse |  |  |  |
| `mouse-drag-bottom-edge` | mouse | `<bottom-edge> <down-mouse-1>` | 同じ |  |
| `mouse-drag-bottom-left-corner` | mouse | `<bottom-left-corner> <down-mouse-1>` | 同じ |  |
| `mouse-drag-bottom-right-corner` | mouse | `<bottom-right-corner> <down-mouse-1>` | 同じ |  |
| `mouse-drag-header-line` | mouse |  |  |  |
| `mouse-drag-left-edge` | mouse | `<left-edge> <down-mouse-1>` | 同じ |  |
| `mouse-drag-mode-line` | mouse | `<bottom-divider> <down-mouse-1>` | 同じ |  |
| `mouse-drag-region` | mouse | `<down-mouse-1>` | 同じ |  |
| `mouse-drag-region-rectangle` | mouse | `C-M-<down-mouse-1>` | 同じ |  |
| `mouse-drag-right-edge` | mouse | `<right-edge> <down-mouse-1>` | 同じ |  |
| `mouse-drag-secondary` | mouse | `M-<down-mouse-1>` | 同じ |  |
| `mouse-drag-tab-line` | mouse | `<tab-line> <down-mouse-1>` | 同じ |  |
| `mouse-drag-top-edge` | mouse | `<top-edge> <down-mouse-1>` | 同じ |  |
| `mouse-drag-top-left-corner` | mouse | `<top-left-corner> <down-mouse-1>` | 同じ |  |
| `mouse-drag-top-right-corner` | mouse | `<top-right-corner> <down-mouse-1>` | 同じ |  |
| `mouse-drag-vertical-line` | mouse | `<right-divider> <down-mouse-1>` | 同じ |  |
| `mouse-kill` | mouse |  |  |  |
| `mouse-kill-ring-save` | mouse |  |  |  |
| `mouse-kill-secondary` | mouse |  |  |  |
| `mouse-minor-mode-menu` | mouse |  |  |  |
| `mouse-save-then-kill` | mouse | `<mouse-3>` | 同じ |  |
| `mouse-secondary-save-then-kill` | mouse | `M-<mouse-3>` | 同じ |  |
| `mouse-select-font` | mouse |  |  |  |
| `mouse-select-window` | mouse | `<tab-line> <mouse-1>` | 同じ |  |
| `mouse-set-font` | mouse |  |  |  |
| `mouse-set-mark` | mouse |  |  |  |
| `mouse-set-point` | mouse | `<mouse-1>`, `C-M-<mouse-1>` | 同じ |  |
| `mouse-set-region` | mouse | `<drag-mouse-1>` | 同じ |  |
| `mouse-set-secondary` | mouse | `M-<drag-mouse-1>` | 同じ |  |
| `mouse-split-window-horizontally` | mouse | `<bottom-divider> C-<mouse-2>` | 同じ |  |
| `mouse-split-window-vertically` | mouse | `<right-divider> C-<mouse-2>` | 同じ |  |
| `mouse-start-secondary` | mouse | `M-<mouse-1>` | 同じ |  |
| `mouse-tear-off-window` | mouse |  |  |  |
| `mouse-yank-at-click` | mouse |  |  |  |
| `mouse-yank-primary` | mouse | `<mouse-2>` | 同じ |  |
| `mouse-yank-secondary` | mouse | `M-<mouse-2>` | 同じ |  |
| `tear-off-window` | mouse | `C-x w ^ f` | 同じ |  |
| `mouse-drag-drag` | mouse-drag |  |  |  |
| `mouse-drag-throw` | mouse-drag |  |  |  |
| `mpc` | mpc |  |  |  |
| `mpuz` | mpuz |  |  |  |
| `msb-mode` | msb |  |  |  |
| `mspools-show` | mspools |  |  |  |
| `recode-region` | mule |  |  |  |
| `revert-buffer-with-coding-system` | mule | `C-x RET r` | 同じ |  |
| `set-buffer-file-coding-system` | mule | `C-x RET f` | 同じ |  |
| `set-buffer-process-coding-system` | mule | `C-x RET p` | 同じ |  |
| `set-clipboard-coding-system` | mule |  |  |  |
| `set-file-name-coding-system` | mule | `C-x RET F` | 同じ |  |
| `set-keyboard-coding-system` | mule | `C-x RET k` | 同じ |  |
| `set-next-selection-coding-system` | mule | `C-x RET X` | 同じ |  |
| `set-selection-coding-system` | mule | `C-x RET x` | 同じ |  |
| `set-terminal-coding-system` | mule | `C-x RET t` | 同じ |  |
| `translate-region` | mule |  |  |  |
| `activate-transient-input-method` | mule-cmds | `C-x \` | 同じ |  |
| `describe-input-method` | mule-cmds | `C-h I`, `<f1> I`, `C-h C-\`, `<f1> C-\` ほか 2 | `C-h I`, `C-h C-\`, `<help> I`, `<help> C-\` |  |
| `describe-language-environment` | mule-cmds | `C-h L`, `<f1> L`, `<help> L` | `C-h L`, `<help> L` |  |
| `describe-specified-language-support` | mule-cmds |  |  |  |
| `highlight-confusing-reorderings` | mule-cmds |  |  |  |
| `prefer-coding-system` | mule-cmds |  |  |  |
| `reset-language-environment` | mule-cmds |  |  |  |
| `search-unencodable-char` | mule-cmds |  |  |  |
| `set-input-method` | mule-cmds | `C-x RET C-\` | 同じ |  |
| `set-language-environment` | mule-cmds | `C-x RET l` | 同じ |  |
| `set-locale-environment` | mule-cmds |  |  |  |
| `setup-specified-language-environment` | mule-cmds |  |  |  |
| `toggle-enable-multibyte-characters` | mule-cmds |  |  |  |
| `toggle-input-method` | mule-cmds | `C-\` |  |  |
| `universal-coding-system-argument` | mule-cmds | `C-x RET c` | 同じ |  |
| `view-hello-file` | mule-cmds | `C-h h`, `<f1> h`, `<help> h` | `C-h h`, `<help> h` |  |
| `describe-character-set` | mule-diag |  |  |  |
| `describe-coding-system` | mule-diag | `C-h C`, `<f1> C`, `<help> C` | `C-h C`, `<help> C` |  |
| `describe-current-coding-system` | mule-diag |  |  |  |
| `describe-current-coding-system-briefly` | mule-diag |  |  |  |
| `describe-font` | mule-diag |  |  |  |
| `describe-fontset` | mule-diag |  |  |  |
| `font-show-log` | mule-diag |  |  |  |
| `list-character-sets` | mule-diag |  |  |  |
| `list-charset-chars` | mule-diag |  |  |  |
| `list-coding-systems` | mule-diag |  |  |  |
| `list-fontsets` | mule-diag |  |  |  |
| `list-input-methods` | mule-diag |  |  |  |
| `mule-diag` | mule-diag |  |  |  |
| `list-multisession-values` | multisession |  |  |  |
| `mouse-wheel-global-text-scale` | mwheel | `C-M-<mouse-4>`, `C-M-<mouse-5>`, `C-M-<wheel-up>`, `C-M-<wheel-down>` | 同じ |  |
| `mouse-wheel-mode` | mwheel |  |  |  |
| `mouse-wheel-text-scale` | mwheel | `C-<mouse-4>`, `C-<mouse-5>`, `C-<wheel-up>`, `C-<wheel-down>` | 同じ |  |
| `mwheel-scroll` | mwheel | `<mouse-4>`, `<mouse-5>`, `<mouse-6>`, `<mouse-7>` ほか 52 | 同じ |  |
| `advice-remove` | nadvice |  |  |  |
| `arp` | net-utils |  |  |  |
| `dns-lookup-host` | net-utils |  |  |  |
| `finger` | net-utils |  |  |  |
| `ftp` | net-utils |  |  |  |
| `ifconfig` | net-utils |  |  |  |
| `iwconfig` | net-utils |  |  |  |
| `netstat` | net-utils |  |  |  |
| `network-connection` | net-utils |  |  |  |
| `network-connection-to-service` | net-utils |  |  |  |
| `nslookup` | net-utils |  |  |  |
| `nslookup-host` | net-utils |  |  |  |
| `ping` | net-utils |  |  |  |
| `route` | net-utils |  |  |  |
| `run-dig` | net-utils |  |  |  |
| `traceroute` | net-utils |  |  |  |
| `whois` | net-utils |  |  |  |
| `whois-reverse-lookup` | net-utils |  |  |  |
| `comment-box` | newcomment |  |  |  |
| `comment-dwim` | newcomment | `M-;` | 同じ | ✓ `M-;` |
| `comment-indent` | newcomment |  |  |  |
| `comment-indent-new-line` | newcomment |  |  |  |
| `comment-kill` | newcomment |  |  |  |
| `comment-line` | newcomment | `C-x C-;` | 同じ |  |
| `comment-or-uncomment-region` | newcomment |  | `C-c ;` |  |
| `comment-region` | newcomment |  |  |  |
| `comment-set-column` | newcomment | `C-x ;` |  |  |
| `indent-for-comment` | newcomment |  |  |  |
| `indent-new-comment-line` | newcomment |  |  |  |
| `kill-comment` | newcomment |  |  |  |
| `set-comment-column` | newcomment |  |  |  |
| `uncomment-region` | newcomment |  |  |  |
| `newsticker-start` | newst-backend |  |  |  |
| `newsticker-plainview` | newst-plainview |  |  |  |
| `newsticker-show-news` | newst-reader |  |  |  |
| `newsticker-start-ticker` | newst-ticker |  |  |  |
| `newsticker-treeview` | newst-treeview |  |  |  |
| `nndiary-generate-nov-databases` | nndiary |  |  |  |
| `nnfolder-generate-active-file` | nnfolder |  |  |  |
| `nnml-generate-nov-databases` | nnml |  |  |  |
| `disable-command` | novice |  |  |  |
| `enable-command` | novice |  |  |  |
| `nroff-mode` | nroff-mode |  |  |  |
| `nxml-mode` | nxml-mode |  |  |  |
| `org-cite-insert` | oc |  |  |  |
| `inferior-octave` | octave |  |  |  |
| `octave-mode` | octave |  |  |  |
| `opascal-mode` | opascal |  |  |  |
| `org-babel-load-file` | org |  |  |  |
| `org-customize` | org |  |  |  |
| `org-cycle-agenda-files` | org |  |  |  |
| `org-mode` | org |  |  |  |
| `org-open-at-point-global` | org |  |  |  |
| `org-reload` | org |  |  |  |
| `org-submit-bug-report` | org |  |  |  |
| `org-switchb` | org |  |  |  |
| `org-version` | org |  |  |  |
| `org-agenda` | org-agenda |  |  |  |
| `org-agenda-list` | org-agenda |  |  |  |
| `org-agenda-list-stuck-projects` | org-agenda |  |  |  |
| `org-agenda-set-restriction-lock` | org-agenda |  |  |  |
| `org-agenda-to-appt` | org-agenda |  |  |  |
| `org-calendar-goto-agenda` | org-agenda |  |  |  |
| `org-search-view` | org-agenda |  |  |  |
| `org-store-agenda-views` | org-agenda |  |  |  |
| `org-tags-view` | org-agenda |  |  |  |
| `org-todo-list` | org-agenda |  |  |  |
| `org-toggle-sticky-agenda` | org-agenda |  |  |  |
| `org-capture` | org-capture |  |  |  |
| `org-capture-import-remember-templates` | org-capture |  |  |  |
| `org-capture-string` | org-capture |  |  |  |
| `org-decrypt-entries` | org-crypt |  |  |  |
| `org-decrypt-entry` | org-crypt |  |  |  |
| `org-encrypt-entries` | org-crypt |  |  |  |
| `org-encrypt-entry` | org-crypt |  |  |  |
| `org-cycle` | org-cycle |  |  |  |
| `org-cycle-global` | org-cycle |  |  |  |
| `outline-minor-mode` | outline |  |  |  |
| `outline-mode` | outline |  |  |  |
| `org-koma-letter-export-as-latex` | ox-koma-letter |  |  |  |
| `org-koma-letter-export-to-latex` | ox-koma-letter |  |  |  |
| `org-koma-letter-export-to-pdf` | ox-koma-letter |  |  |  |
| `describe-package` | package | `C-h P`, `<f1> P`, `<help> P` | `C-h P`, `<help> P` |  |
| `list-packages` | package |  |  |  |
| `package-autoremove` | package |  |  |  |
| `package-import-keyring` | package |  |  |  |
| `package-initialize` | package |  |  |  |
| `package-install` | package |  |  |  |
| `package-install-file` | package |  |  |  |
| `package-install-from-buffer` | package |  |  |  |
| `package-install-selected-packages` | package |  |  |  |
| `package-recompile` | package |  |  |  |
| `package-recompile-all` | package |  |  |  |
| `package-refresh-contents` | package |  |  |  |
| `package-reinstall` | package |  |  |  |
| `package-report-bug` | package |  |  |  |
| `package-upgrade` | package |  |  |  |
| `package-upgrade-all` | package |  |  |  |
| `package-vc-checkout` | package-vc |  |  |  |
| `package-vc-install` | package-vc |  |  |  |
| `package-vc-install-from-checkout` | package-vc |  |  |  |
| `package-vc-install-selected-packages` | package-vc |  |  |  |
| `package-vc-prepare-patch` | package-vc |  |  |  |
| `package-vc-rebuild` | package-vc |  |  |  |
| `package-vc-upgrade` | package-vc |  |  |  |
| `package-vc-upgrade-all` | package-vc |  |  |  |
| `package-upload-file` | package-x |  |  |  |
| `backward-page` | page | `C-x [` | 同じ |  |
| `count-lines-page` | page | `C-x l` | 同じ |  |
| `forward-page` | page | `C-x ]` | 同じ |  |
| `mark-page` | page | `C-x C-p` | 同じ |  |
| `narrow-to-page` | page | `C-x n p` | 同じ |  |
| `what-page` | page |  |  |  |
| `backward-kill-paragraph` | paragraphs |  |  |  |
| `backward-kill-sentence` | paragraphs | `C-x DEL` | 同じ |  |
| `backward-paragraph` | paragraphs | `M-{`, `C-<up>` | 同じ | ✓ `M-{` |
| `backward-sentence` | paragraphs | `M-a` | 同じ | ✓ `M-a` |
| `forward-paragraph` | paragraphs | `M-}`, `C-<down>` | 同じ | ✓ `M-}` |
| `forward-sentence` | paragraphs | `M-e` | 同じ | ✓ `M-e` |
| `kill-paragraph` | paragraphs |  |  |  |
| `kill-sentence` | paragraphs | `M-k` | 同じ | ✓ `M-k` |
| `mark-end-of-sentence` | paragraphs |  |  |  |
| `mark-paragraph` | paragraphs | `M-h` | 同じ |  |
| `repunctuate-sentences` | paragraphs |  |  |  |
| `transpose-paragraphs` | paragraphs |  |  |  |
| `transpose-sentences` | paragraphs |  |  |  |
| `use-hard-newlines` | paragraphs |  |  |  |
| `show-paren-local-mode` | paren |  |  |  |
| `show-paren-mode` | paren |  |  |  |
| `pascal-mode` | pascal |  |  |  |
| `pcomplete` | pcomplete |  |  |  |
| `pcomplete-continue` | pcomplete |  |  |  |
| `pcomplete-expand` | pcomplete |  |  |  |
| `pcomplete-expand-and-complete` | pcomplete |  |  |  |
| `pcomplete-help` | pcomplete |  |  |  |
| `pcomplete-list` | pcomplete |  |  |  |
| `pcomplete-reverse` | pcomplete |  |  |  |
| `cvs-checkout` | pcvs |  |  |  |
| `cvs-examine` | pcvs |  |  |  |
| `cvs-quickdir` | pcvs |  |  |  |
| `cvs-status` | pcvs |  |  |  |
| `cvs-update` | pcvs |  |  |  |
| `perl-mode` | perl-mode |  |  |  |
| `php-ts-mode` | php-ts-mode |  |  |  |
| `php-ts-mode-run-php-webserver` | php-ts-mode |  |  |  |
| `run-php` | php-ts-mode |  |  |  |
| `picture-mode` | picture |  |  |  |
| `pixel-scroll-mode` | pixel-scroll |  |  |  |
| `pixel-scroll-precision-mode` | pixel-scroll |  |  |  |
| `plstore-mode` | plstore |  |  |  |
| `pong` | pong |  |  |  |
| `pp-buffer` | pp |  |  |  |
| `pp-eval-expression` | pp |  |  |  |
| `pp-eval-last-sexp` | pp |  |  |  |
| `pp-macroexpand-expression` | pp |  |  |  |
| `pp-macroexpand-last-sexp` | pp |  |  |  |
| `lpr-customize` | printing |  |  |  |
| `pr-customize` | printing |  |  |  |
| `pr-despool-preview` | printing |  |  |  |
| `pr-despool-print` | printing |  |  |  |
| `pr-despool-ps-print` | printing |  |  |  |
| `pr-despool-using-ghostscript` | printing |  |  |  |
| `pr-help` | printing |  |  |  |
| `pr-interface` | printing |  |  |  |
| `pr-printify-buffer` | printing |  |  |  |
| `pr-printify-directory` | printing |  |  |  |
| `pr-printify-region` | printing |  |  |  |
| `pr-ps-buffer-preview` | printing |  |  |  |
| `pr-ps-buffer-print` | printing |  |  |  |
| `pr-ps-buffer-ps-print` | printing |  |  |  |
| `pr-ps-buffer-using-ghostscript` | printing |  |  |  |
| `pr-ps-directory-preview` | printing |  |  |  |
| `pr-ps-directory-print` | printing |  |  |  |
| `pr-ps-directory-ps-print` | printing |  |  |  |
| `pr-ps-directory-using-ghostscript` | printing |  |  |  |
| `pr-ps-fast-fire` | printing |  |  |  |
| `pr-ps-file-preview` | printing |  |  |  |
| `pr-ps-file-print` | printing |  |  |  |
| `pr-ps-file-ps-print` | printing |  |  |  |
| `pr-ps-file-up-preview` | printing |  |  |  |
| `pr-ps-file-up-ps-print` | printing |  |  |  |
| `pr-ps-file-using-ghostscript` | printing |  |  |  |
| `pr-ps-mode-preview` | printing |  |  |  |
| `pr-ps-mode-print` | printing |  |  |  |
| `pr-ps-mode-ps-print` | printing |  |  |  |
| `pr-ps-mode-using-ghostscript` | printing |  |  |  |
| `pr-ps-name` | printing |  |  |  |
| `pr-ps-region-preview` | printing |  |  |  |
| `pr-ps-region-print` | printing |  |  |  |
| `pr-ps-region-ps-print` | printing |  |  |  |
| `pr-ps-region-using-ghostscript` | printing |  |  |  |
| `pr-ps-utility` | printing |  |  |  |
| `pr-show-lpr-setup` | printing |  |  |  |
| `pr-show-pr-setup` | printing |  |  |  |
| `pr-show-ps-setup` | printing |  |  |  |
| `pr-toggle-duplex` | printing |  |  |  |
| `pr-toggle-faces` | printing |  |  |  |
| `pr-toggle-file-duplex` | printing |  |  |  |
| `pr-toggle-file-landscape` | printing |  |  |  |
| `pr-toggle-file-tumble` | printing |  |  |  |
| `pr-toggle-ghostscript` | printing |  |  |  |
| `pr-toggle-header` | printing |  |  |  |
| `pr-toggle-header-frame` | printing |  |  |  |
| `pr-toggle-landscape` | printing |  |  |  |
| `pr-toggle-line` | printing |  |  |  |
| `pr-toggle-lock` | printing |  |  |  |
| `pr-toggle-mode` | printing |  |  |  |
| `pr-toggle-region` | printing |  |  |  |
| `pr-toggle-spool` | printing |  |  |  |
| `pr-toggle-tumble` | printing |  |  |  |
| `pr-toggle-upside-down` | printing |  |  |  |
| `pr-toggle-zebra` | printing |  |  |  |
| `pr-txt-buffer` | printing |  |  |  |
| `pr-txt-directory` | printing |  |  |  |
| `pr-txt-fast-fire` | printing |  |  |  |
| `pr-txt-mode` | printing |  |  |  |
| `pr-txt-name` | printing |  |  |  |
| `pr-txt-region` | printing |  |  |  |
| `proced` | proced |  |  |  |
| `profiler-find-profile` | profiler |  |  |  |
| `profiler-find-profile-other-frame` | profiler |  |  |  |
| `profiler-find-profile-other-window` | profiler |  |  |  |
| `profiler-start` | profiler |  |  |  |
| `global-prettify-symbols-mode` | prog-mode |  |  |  |
| `prettify-symbols-mode` | prog-mode |  |  |  |
| `prog-fill-reindent-defun` | prog-mode |  |  |  |
| `prog-indent-sexp` | prog-mode |  |  |  |
| `prog-mode` | prog-mode |  |  |  |
| `project-any-command` | project | `C-x p o` | 同じ |  |
| `project-async-shell-command` | project | `C-x p &` | 同じ |  |
| `project-compile` | project | `C-x p c` | 同じ |  |
| `project-dired` | project | `C-x p D` | 同じ |  |
| `project-display-buffer` | project |  |  |  |
| `project-display-buffer-other-frame` | project |  |  |  |
| `project-eshell` | project | `C-x p e` | 同じ |  |
| `project-execute-extended-command` | project | `C-x p x` | 同じ |  |
| `project-find-dir` | project | `C-x p d` | 同じ |  |
| `project-find-file` | project | `C-x p f` | 同じ |  |
| `project-find-regexp` | project | `C-x p g` | 同じ |  |
| `project-forget-project` | project |  |  |  |
| `project-kill-buffers` | project | `C-x p k` | 同じ |  |
| `project-list-buffers` | project | `C-x p C-b` | 同じ |  |
| `project-or-external-find-file` | project | `C-x p F` | 同じ |  |
| `project-or-external-find-regexp` | project | `C-x p G` | 同じ |  |
| `project-other-frame-command` | project | `C-x 5 p` | 同じ |  |
| `project-other-tab-command` | project | `C-x t p` | 同じ |  |
| `project-other-window-command` | project | `C-x 4 p` | 同じ |  |
| `project-prefix-or-any-command` | project |  |  |  |
| `project-query-replace-regexp` | project | `C-x p r` | 同じ |  |
| `project-search` | project |  |  |  |
| `project-shell` | project | `C-x p s` | 同じ |  |
| `project-shell-command` | project | `C-x p !` | 同じ |  |
| `project-switch-project` | project | `C-x p p` | 同じ |  |
| `project-switch-to-buffer` | project | `C-x p b` | 同じ |  |
| `project-vc-dir` | project | `C-x p v` | 同じ |  |
| `mercury-mode` | prolog |  |  |  |
| `prolog-mode` | prolog |  |  |  |
| `run-prolog` | prolog |  |  |  |
| `ps-mode` | ps-mode |  |  |  |
| `ps-despool` | ps-print |  |  |  |
| `ps-line-lengths` | ps-print |  |  |  |
| `ps-nb-pages-buffer` | ps-print |  |  |  |
| `ps-nb-pages-region` | ps-print |  |  |  |
| `ps-print-buffer` | ps-print |  |  |  |
| `ps-print-buffer-with-faces` | ps-print |  |  |  |
| `ps-print-customize` | ps-print |  |  |  |
| `ps-print-region` | ps-print |  |  |  |
| `ps-print-region-with-faces` | ps-print |  |  |  |
| `ps-spool-buffer` | ps-print |  |  |  |
| `ps-spool-buffer-with-faces` | ps-print |  |  |  |
| `ps-spool-region` | ps-print |  |  |  |
| `ps-spool-region-with-faces` | ps-print |  |  |  |
| `python-add-import` | python |  |  |  |
| `python-base-mode` | python |  |  |  |
| `python-fix-imports` | python |  |  |  |
| `python-import-symbol-at-point` | python |  |  |  |
| `python-mode` | python |  |  |  |
| `python-remove-import` | python |  |  |  |
| `python-sort-imports` | python |  |  |  |
| `python-ts-mode` | python |  |  |  |
| `run-python` | python |  |  |  |
| `quoted-printable-decode-region` | qp |  |  |  |
| `quail-set-keyboard-layout` | quail |  |  |  |
| `quail-show-keyboard-layout` | quail |  |  |  |
| `quail-update-leim-list-file` | quail |  |  |  |
| `rcirc` | rcirc |  |  |  |
| `rcirc-track-minor-mode` | rcirc |  |  |  |
| `re-builder` | re-builder |  |  |  |
| `recentf-mode` | recentf |  |  |  |
| `recentf-open` | recentf |  |  |  |
| `clear-rectangle` | rect | `C-x r c` | 同じ |  |
| `copy-rectangle-as-kill` | rect | `C-x r M-w` | 同じ |  |
| `delete-rectangle` | rect | `C-x r d` | 同じ |  |
| `delete-whitespace-rectangle` | rect |  |  |  |
| `kill-rectangle` | rect | `C-x r k` | 同じ | ✓ `C-x r k` |
| `open-rectangle` | rect | `C-x r o` | 同じ |  |
| `rectangle-mark-mode` | rect | `C-x SPC` | 同じ |  |
| `rectangle-number-lines` | rect | `C-x r N` | 同じ |  |
| `string-insert-rectangle` | rect |  |  |  |
| `string-rectangle` | rect | `C-x r t` | 同じ | ✓ `C-x r t` |
| `yank-rectangle` | rect | `C-x r y` | 同じ | ✓ `C-x r y` |
| `refill-mode` | refill |  |  |  |
| `reftex-mode` | reftex |  |  |  |
| `reftex-citation` | reftex-cite |  |  |  |
| `reftex-isearch-minor-mode` | reftex-global |  |  |  |
| `reftex-index-phrases-mode` | reftex-index |  |  |  |
| `append-to-register` | register |  |  |  |
| `copy-rectangle-to-register` | register | `C-x r r` | 同じ |  |
| `copy-to-register` | register | `C-x r s`, `C-x r x` | 同じ | ✓ `C-x r s`, `C-x r x` |
| `frame-configuration-to-register` | register |  |  |  |
| `increment-register` | register | `C-x r +` | 同じ |  |
| `insert-register` | register | `C-x r g`, `C-x r i` | 同じ | ✓ `C-x r i`, `C-x r g` |
| `jump-to-register` | register | `C-x r j` | 同じ |  |
| `list-registers` | register |  |  |  |
| `number-to-register` | register | `C-x r n` | 同じ |  |
| `point-to-register` | register | `C-x r C-@`, `C-x r SPC`, `C-x r C-SPC` | 同じ |  |
| `prepend-to-register` | register |  |  |  |
| `register-preview-next` | register |  |  |  |
| `register-preview-previous` | register |  |  |  |
| `register-to-point` | register |  |  |  |
| `view-register` | register |  |  |  |
| `window-configuration-to-register` | register | `C-x r w` | 同じ |  |
| `remember` | remember |  |  |  |
| `remember-clipboard` | remember |  |  |  |
| `remember-notes` | remember |  |  |  |
| `remember-other-frame` | remember |  |  |  |
| `repeat` | repeat | `C-x z` | 同じ |  |
| `repeat-exit` | repeat |  |  |  |
| `repeat-mode` | repeat |  |  |  |
| `copy-matching-lines` | replace |  |  |  |
| `count-matches` | replace |  |  |  |
| `delete-matching-lines` | replace |  |  |  |
| `delete-non-matching-lines` | replace |  |  |  |
| `flush-lines` | replace |  |  |  |
| `how-many` | replace |  |  |  |
| `keep-lines` | replace |  |  |  |
| `kill-matching-lines` | replace |  |  |  |
| `list-matching-lines` | replace |  |  |  |
| `map-query-replace-regexp` | replace |  |  |  |
| `multi-occur` | replace |  |  |  |
| `multi-occur-in-matching-buffers` | replace |  |  |  |
| `occur` | replace | `M-s o` | 同じ |  |
| `occur-cease-edit` | replace |  |  |  |
| `occur-edit-mode` | replace |  |  |  |
| `occur-menu-map` | replace |  |  |  |
| `occur-mode` | replace |  |  |  |
| `occur-mode-display-occurrence` | replace |  |  |  |
| `occur-mode-goto-occurrence` | replace |  |  |  |
| `occur-mode-goto-occurrence-other-window` | replace |  |  |  |
| `occur-mode-mouse-goto` | replace |  |  |  |
| `occur-next` | replace |  |  |  |
| `occur-next-error` | replace |  |  |  |
| `occur-prev` | replace |  |  |  |
| `occur-rename-buffer` | replace |  |  |  |
| `occur-symbol-at-mouse` | replace |  |  |  |
| `occur-word-at-mouse` | replace |  |  |  |
| `query-replace` | replace | `M-%` | 同じ | ✓ `M-%` |
| `query-replace-regexp` | replace | `C-M-%` | 同じ | ✓ `C-M-%` |
| `read-regexp-toggle-case-fold` | replace |  |  |  |
| `replace-regexp` | replace |  |  | ✓ |
| `replace-string` | replace |  |  | ✓ |
| `reposition-window` | reposition | `C-M-l` | 同じ |  |
| `global-reveal-mode` | reveal |  |  |  |
| `reveal-mode` | reveal |  |  |  |
| `file-name-shadow-mode` | rfn-eshadow |  |  |  |
| `rmail` | rmail |  |  |  |
| `rmail-input` | rmail |  |  |  |
| `rmail-mode` | rmail |  |  |  |
| `rmail-set-remote-password` | rmail |  |  |  |
| `rmail-edit-current-message` | rmailedit |  |  |  |
| `rmail-add-label` | rmailkwd |  |  |  |
| `rmail-kill-label` | rmailkwd |  |  |  |
| `rmail-next-labeled-message` | rmailkwd |  |  |  |
| `rmail-previous-labeled-message` | rmailkwd |  |  |  |
| `rmail-mime` | rmailmm |  |  |  |
| `set-rmail-inbox-list` | rmailmsc |  |  |  |
| `rmail-output` | rmailout |  |  |  |
| `rmail-output-as-seen` | rmailout |  |  |  |
| `rmail-output-body-to-file` | rmailout |  |  |  |
| `rmail-sort-by-author` | rmailsort |  |  |  |
| `rmail-sort-by-correspondent` | rmailsort |  |  |  |
| `rmail-sort-by-date` | rmailsort |  |  |  |
| `rmail-sort-by-labels` | rmailsort |  |  |  |
| `rmail-sort-by-lines` | rmailsort |  |  |  |
| `rmail-sort-by-recipient` | rmailsort |  |  |  |
| `rmail-sort-by-subject` | rmailsort |  |  |  |
| `rmail-summary` | rmailsum |  |  |  |
| `rmail-summary-by-labels` | rmailsum |  |  |  |
| `rmail-summary-by-recipients` | rmailsum |  |  |  |
| `rmail-summary-by-regexp` | rmailsum |  |  |  |
| `rmail-summary-by-senders` | rmailsum |  |  |  |
| `rmail-summary-by-thread` | rmailsum |  |  |  |
| `rmail-summary-by-topic` | rmailsum |  |  |  |
| `rng-nxml-mode-init` | rng-nxml |  |  |  |
| `rng-validate-mode` | rng-valid |  |  |  |
| `rot13-other-window` | rot13 |  |  |  |
| `rot13-region` | rot13 |  |  |  |
| `toggle-rot13-mode` | rot13 |  |  |  |
| `rst-minor-mode` | rst |  |  |  |
| `rst-mode` | rst |  |  |  |
| `ruby-base-mode` | ruby-mode |  |  |  |
| `ruby-mode` | ruby-mode |  |  |  |
| `ruby-ts-mode` | ruby-ts-mode |  |  |  |
| `ruler-mode` | ruler-mode |  |  |  |
| `rust-ts-mode` | rust-ts-mode |  |  |  |
| `savehist-mode` | savehist |  |  |  |
| `save-place-local-mode` | saveplace |  |  |  |
| `save-place-mode` | saveplace |  |  |  |
| `dsssl-mode` | scheme |  |  |  |
| `scheme-mode` | scheme |  |  |  |
| `gnus-score-mode` | score-mode |  |  |  |
| `scroll-all-mode` | scroll-all |  |  |  |
| `horizontal-scroll-bar-mode` | scroll-bar |  |  |  |
| `scroll-bar-drag` | scroll-bar |  |  |  |
| `scroll-bar-horizontal-drag` | scroll-bar |  |  |  |
| `scroll-bar-maybe-set-window-start` | scroll-bar |  |  |  |
| `scroll-bar-mode` | scroll-bar |  |  |  |
| `scroll-bar-scroll-down` | scroll-bar |  |  |  |
| `scroll-bar-scroll-up` | scroll-bar |  |  |  |
| `scroll-bar-set-window-start` | scroll-bar |  |  |  |
| `scroll-bar-toolkit-horizontal-scroll` | scroll-bar |  |  |  |
| `scroll-bar-toolkit-scroll` | scroll-bar |  |  |  |
| `toggle-horizontal-scroll-bar` | scroll-bar |  |  |  |
| `toggle-scroll-bar` | scroll-bar |  |  |  |
| `scroll-lock-mode` | scroll-lock | `<Scroll_Lock>` | 同じ |  |
| `secrets-show-secrets` | secrets |  |  |  |
| `gui-set-selection` | select |  |  |  |
| `lost-selection-mode` | select |  |  |  |
| `x-set-selection` | select |  |  |  |
| `semantic-mode` | semantic |  |  |  |
| `mail` | sendmail |  |  |  |
| `mail-mode` | sendmail |  |  |  |
| `mail-other-frame` | sendmail |  |  |  |
| `mail-other-window` | sendmail |  |  |  |
| `mail-send-and-exit` | sendmail |  |  |  |
| `server-force-delete` | server |  |  |  |
| `server-mode` | server |  |  |  |
| `server-start` | server |  |  |  |
| `ses-mode` | ses |  |  |  |
| `html-mode` | sgml-mode |  |  |  |
| `sgml-mode` | sgml-mode |  |  |  |
| `bash-ts-mode` | sh-script |  |  |  |
| `sh-base-mode` | sh-script |  |  |  |
| `sh-mode` | sh-script |  |  |  |
| `list-load-path-shadows` | shadow |  |  |  |
| `shadow-define-cluster` | shadowfile |  |  |  |
| `shadow-define-literal-group` | shadowfile |  |  |  |
| `shadow-define-regexp-group` | shadowfile |  |  |  |
| `shadow-initialize` | shadowfile |  |  |  |
| `shell` | shell |  |  |  |
| `shortdoc-display-group` | shortdoc |  |  |  |
| `shr-render-region` | shr |  |  |  |
| `sieve-manage` | sieve |  |  |  |
| `sieve-upload` | sieve |  |  |  |
| `sieve-upload-and-bury` | sieve |  |  |  |
| `sieve-upload-and-kill` | sieve |  |  |  |
| `sieve-mode` | sieve-mode |  |  |  |
| `advertised-undo` | simple |  |  |  |
| `analyze-text-conversion` | simple | `<text-conversion>` | 同じ |  |
| `append-next-kill` | simple | `C-M-w` | 同じ |  |
| `append-to-buffer` | simple |  |  |  |
| `async-shell-command` | simple | `M-&` | 同じ |  |
| `auto-fill-mode` | simple |  |  |  |
| `auto-save-mode` | simple |  |  |  |
| `back-to-indentation` | simple | `M-m` | 同じ | ✓ `M-m` |
| `backward-delete-char-untabify` | simple |  |  |  |
| `backward-kill-word` | simple | `M-DEL`, `C-<backspace>` | 同じ | ✓ `M-DEL` |
| `backward-to-indentation` | simple |  |  |  |
| `backward-word` | simple | `M-b`, `ESC <left>` | `M-b`, `C-<left>`, `ESC <left>` | ✓ `M-b` |
| `beginning-of-buffer` | simple | `M-<`, `<begin>`, `C-<home>` | 同じ | ✓ `M-<` |
| `beginning-of-visual-line` | simple |  |  |  |
| `binary-overwrite-mode` | simple |  |  |  |
| `blink-matching-open` | simple |  |  |  |
| `buffer-disable-undo` | simple |  |  |  |
| `capitalize-dwim` | simple |  |  |  |
| `choose-completion` | simple |  |  |  |
| `clean-mode` | simple |  |  |  |
| `clone-buffer` | simple | `C-x x n` | 同じ |  |
| `clone-indirect-buffer` | simple |  |  |  |
| `clone-indirect-buffer-other-window` | simple | `C-x 4 c` | 同じ |  |
| `column-number-mode` | simple |  |  |  |
| `completion-list-mode` | simple |  |  |  |
| `compose-mail` | simple | `C-x m` | 同じ |  |
| `compose-mail-other-frame` | simple | `C-x 5 m` | 同じ |  |
| `compose-mail-other-window` | simple | `C-x 4 m` | 同じ |  |
| `copy-region-as-kill` | simple |  |  |  |
| `copy-to-buffer` | simple |  |  |  |
| `count-words` | simple |  |  | ✓ |
| `count-words-region` | simple | `M-=` | 同じ | ✓ `M-=` |
| `cursor-face-highlight-mode` | simple |  |  |  |
| `cycle-spacing` | simple | `M-SPC` | 同じ |  |
| `default-indent-new-line` | simple | `M-j`, `C-M-j` | 同じ |  |
| `delete-all-space` | simple |  |  |  |
| `delete-backward-char` | simple | `DEL` | 同じ |  |
| `delete-blank-lines` | simple | `C-x C-o` |  | ✓ `C-x C-o` |
| `delete-completion-window` | simple |  |  |  |
| `delete-forward-char` | simple | `<deletechar>` | 同じ |  |
| `delete-horizontal-space` | simple | `M-\` | 同じ | ✓ `M-\` |
| `delete-indentation` | simple | `M-^` | 同じ | ✓ `M-^` |
| `delete-trailing-whitespace` | simple |  |  |  |
| `digit-argument` | simple | `C-0`, `C-1`, `C-2`, `C-3` ほか 16 | `C-0`, `C-1`, `C-2`, `C-3` ほか 17 | ✓ `M-0`, `M-1`, `M-2` ほか 7 |
| `downcase-dwim` | simple |  |  |  |
| `end-of-buffer` | simple | `M->`, `C-<end>` | 同じ | ✓ `M->` |
| `end-of-visual-line` | simple |  |  |  |
| `eval-expression` | simple | `M-:`, `ESC M-:` | 同じ | ✓ `M-:` |
| `exchange-point-and-mark` | simple | `C-x C-x` | 同じ | ✓ `C-x C-x` |
| `execute-extended-command` | simple | `M-x`, `<menu>`, `<execute>` | 同じ | ✓ `M-x` |
| `execute-extended-command-cycle` | simple |  |  |  |
| `execute-extended-command-for-buffer` | simple | `M-X` | 同じ |  |
| `first-completion` | simple |  |  |  |
| `first-error` | simple |  |  |  |
| `fixup-whitespace` | simple |  |  |  |
| `forward-to-indentation` | simple |  |  |  |
| `fundamental-mode` | simple |  |  |  |
| `global-visual-line-mode` | simple |  |  |  |
| `goto-history-element` | simple |  |  |  |
| `goto-line` | simple | `M-g g`, `M-g M-g` | `M-g` | ✓ `M-g g`, `M-g M-g` |
| `goto-line-relative` | simple | `C-x n g` | 同じ |  |
| `goto-next-locus` | simple |  |  |  |
| `indent-tabs-mode` | simple |  |  |  |
| `insert-buffer` | simple | `C-x x i` | 同じ |  |
| `join-line` | simple |  |  |  |
| `just-one-space` | simple |  | `C-S-k` | ✓ `M-SPC` |
| `keyboard-escape-quit` | simple | `ESC ESC ESC` | 同じ |  |
| `keyboard-quit` | simple | `C-g` | 同じ | ✓ `C-g` |
| `kill-current-buffer` | simple |  |  |  |
| `kill-line` | simple | `C-k`, `<deleteline>` | 同じ | ✓ `C-k` |
| `kill-region` | simple | `C-w`, `<cut>`, `S-<delete>` | 同じ | ✓ `C-w` |
| `kill-ring-save` | simple | `M-w`, `<copy>`, `C-<insert>`, `C-<insertchar>` | `M-w`, `<copy>`, `C-<insertchar>` | ✓ `M-w` |
| `kill-visual-line` | simple |  |  |  |
| `kill-whole-line` | simple | `C-S-<backspace>` | 同じ |  |
| `kill-word` | simple | `M-d`, `C-<delete>` | 同じ | ✓ `M-d` |
| `last-completion` | simple |  |  |  |
| `line-number-mode` | simple |  |  |  |
| `list-processes` | simple |  |  |  |
| `mark-whole-buffer` | simple | `C-x h` | 同じ | ✓ `C-x h` |
| `mark-word` | simple | `M-@` | 同じ | ✓ `M-@` |
| `messages-buffer-mode` | simple |  |  |  |
| `move-beginning-of-line` | simple | `C-a`, `<home>` | 同じ | ✓ `C-a` |
| `move-end-of-line` | simple | `C-e`, `<end>` | 同じ | ✓ `C-e` |
| `negative-argument` | simple | `C--`, `M--`, `C-M--` | 同じ |  |
| `newline` | simple | `RET` |  | ✓ `RET`, `C-m` |
| `newline-and-indent` | simple |  | `RET` |  |
| `next-complete-history-element` | simple |  |  |  |
| `next-completion` | simple |  |  |  |
| `next-error` | simple | `` C-x ` ``, `M-g n`, `M-g M-n` | `` C-x ` `` | ✓ `M-g n`, `M-g M-n`, `` C-x ` `` |
| `next-error-follow-minor-mode` | simple |  |  |  |
| `next-error-no-select` | simple |  |  |  |
| `next-error-select-buffer` | simple |  |  |  |
| `next-history-element` | simple |  |  |  |
| `next-line` | simple | `C-n`, `<down>` | `C-n`, `M-[ B`, `<down>` | ✓ `C-n` |
| `next-line-completion` | simple |  |  |  |
| `next-line-or-history-element` | simple |  |  |  |
| `next-logical-line` | simple |  |  |  |
| `next-match` | simple |  |  |  |
| `next-matching-history-element` | simple |  |  |  |
| `normal-erase-is-backspace-mode` | simple |  |  |  |
| `open-line` | simple | `C-o`, `<insertline>` | 同じ | ✓ `C-o` |
| `overwrite-mode` | simple | `<insert>`, `<insertchar>` | 同じ |  |
| `play-sound-file` | simple |  |  |  |
| `pop-global-mark` | simple | `C-x C-@`, `C-x C-SPC` | 同じ |  |
| `pop-to-mark-command` | simple |  |  |  |
| `prepend-to-buffer` | simple |  |  |  |
| `previous-complete-history-element` | simple |  |  |  |
| `previous-completion` | simple |  |  |  |
| `previous-error` | simple | `M-g p`, `M-g M-p` |  | ✓ `M-g p`, `M-g M-p` |
| `previous-error-no-select` | simple |  |  |  |
| `previous-history-element` | simple |  |  |  |
| `previous-line` | simple | `C-p`, `<up>` | `C-p`, `<up>`, `M-[ A` | ✓ `C-p` |
| `previous-line-completion` | simple |  |  |  |
| `previous-line-or-history-element` | simple |  |  |  |
| `previous-logical-line` | simple |  |  |  |
| `previous-matching-history-element` | simple |  |  |  |
| `process-menu-delete-process` | simple |  |  |  |
| `process-menu-mode` | simple |  |  |  |
| `push-mark-command` | simple |  |  |  |
| `quoted-insert` | simple | `C-q` | 同じ | ✓ `C-q` |
| `read--expression-try-read` | simple |  |  |  |
| `read-expression-switch-to-completions` | simple |  |  |  |
| `read-extended-command-mode` | simple |  |  |  |
| `read-only-mode` | simple | `C-x C-q` | 同じ |  |
| `recenter-current-error` | simple |  |  |  |
| `reindent-then-newline-and-indent` | simple |  |  |  |
| `repeat-complex-command` | simple | `<redo>`, `<again>`, `C-x M-:`, `C-x ESC ESC` | 同じ |  |
| `rotate-yank-pointer` | simple |  |  |  |
| `scratch-buffer` | simple |  |  |  |
| `set-fill-column` | simple | `C-x f` | 同じ |  |
| `set-goal-column` | simple | `C-x C-n` | 同じ | ✓ `C-x C-n` |
| `set-mark-command` | simple | `C-@`, `C-SPC` | 同じ | ✓ `C-SPC` |
| `set-selective-display` | simple | `C-x $` | 同じ |  |
| `set-variable` | simple |  |  |  |
| `shell-command` | simple | `M-!` | 同じ |  |
| `shell-command-on-region` | simple | `M-\|` | 同じ |  |
| `size-indication-mode` | simple |  |  |  |
| `special-mode` | simple |  |  |  |
| `split-line` | simple | `C-M-o` | 同じ |  |
| `switch-to-completions` | simple |  |  |  |
| `switch-to-minibuffer` | simple |  |  |  |
| `toggle-truncate-lines` | simple | `C-x x t` | `C-c C-l`, `C-x x t` |  |
| `toggle-word-wrap` | simple |  |  |  |
| `transient-mark-mode` | simple |  |  |  |
| `transpose-chars` | simple | `C-t` | `C-S-t` | ✓ `C-t` |
| `transpose-lines` | simple | `C-x C-t` | 同じ | ✓ `C-x C-t` |
| `transpose-sexps` | simple | `C-M-t` | 同じ |  |
| `transpose-words` | simple | `M-t` | 同じ | ✓ `M-t` |
| `undo` | simple | `C-/`, `C-_`, `C-x u`, `<undo>` | 同じ | ✓ `C-/`, `C-_`, `C-x u` |
| `undo-only` | simple |  |  |  |
| `undo-redo` | simple | `C-?`, `C-M-_` | 同じ |  |
| `universal-argument` | simple | `C-u` | 同じ | ✓ `C-u` |
| `universal-argument-more` | simple |  |  |  |
| `upcase-dwim` | simple |  |  |  |
| `visible-mode` | simple |  |  |  |
| `visual-line-mode` | simple |  |  |  |
| `what-cursor-position` | simple | `C-x =` | 同じ |  |
| `what-line` | simple |  |  |  |
| `yank` | simple | `C-y`, `<paste>`, `S-<insert>`, `S-<insertchar>` | `C-y`, `<paste>`, `S-<insertchar>` | ✓ `C-y` |
| `yank-from-kill-ring` | simple |  |  |  |
| `yank-in-context` | simple |  |  |  |
| `yank-pop` | simple | `M-y` | 同じ | ✓ `M-y` |
| `zap-to-char` | simple | `M-z` | 同じ | ✓ `M-z` |
| `simula-mode` | simula |  |  |  |
| `skeleton-pair-insert-maybe` | skeleton |  |  |  |
| `smerge-ediff` | smerge-mode |  |  |  |
| `smerge-mode` | smerge-mode |  |  |  |
| `smerge-start-session` | smerge-mode |  |  |  |
| `smiley-buffer` | smiley |  |  |  |
| `smiley-region` | smiley |  |  |  |
| `smtpmail-send-queued-mail` | smtpmail |  |  |  |
| `snake` | snake |  |  |  |
| `snmp-mode` | snmp-mode |  |  |  |
| `snmpv2-mode` | snmp-mode |  |  |  |
| `global-so-long-mode` | so-long |  |  |  |
| `so-long` | so-long |  |  |  |
| `so-long-commentary` | so-long |  |  |  |
| `so-long-customize` | so-long |  |  |  |
| `so-long-enable` | so-long |  |  |  |
| `so-long-minor-mode` | so-long |  |  |  |
| `so-long-mode` | so-long |  |  |  |
| `sunrise-sunset` | solar |  |  |  |
| `solitaire` | solitaire |  |  |  |
| `delete-duplicate-lines` | sort |  |  |  |
| `reverse-region` | sort |  |  |  |
| `sort-columns` | sort |  |  |  |
| `sort-fields` | sort |  |  |  |
| `sort-lines` | sort |  |  |  |
| `sort-numeric-fields` | sort |  |  |  |
| `sort-pages` | sort |  |  |  |
| `sort-paragraphs` | sort |  |  |  |
| `sort-regexp-fields` | sort |  |  |  |
| `spam-initialize` | spam |  |  |  |
| `spam-report-agentize` | spam-report |  |  |  |
| `spam-report-deagentize` | spam-report |  |  |  |
| `spam-report-process-queue` | spam-report |  |  |  |
| `speedbar-frame-mode` | speedbar |  |  |  |
| `speedbar-get-focus` | speedbar |  | `<f6>`, `C-c x` |  |
| `spook` | spook |  |  |  |
| `sql-connect` | sql |  |  |  |
| `sql-db2` | sql |  |  |  |
| `sql-informix` | sql |  |  |  |
| `sql-ingres` | sql |  |  |  |
| `sql-interbase` | sql |  |  |  |
| `sql-linter` | sql |  |  |  |
| `sql-mariadb` | sql |  |  |  |
| `sql-mode` | sql |  |  |  |
| `sql-ms` | sql |  |  |  |
| `sql-mysql` | sql |  |  |  |
| `sql-oracle` | sql |  |  |  |
| `sql-postgres` | sql |  |  |  |
| `sql-product-interactive` | sql |  |  |  |
| `sql-solid` | sql |  |  |  |
| `sql-sqlite` | sql |  |  |  |
| `sql-sybase` | sql |  |  |  |
| `sql-vertica` | sql |  |  |  |
| `sqlite-mode-open-file` | sqlite-mode |  |  |  |
| `srecode-template-mode` | srt-mode |  |  |  |
| `about-emacs` | startup | `C-h C-a`, `<f1> C-a`, `<help> C-a` | `C-h C-a`, `<help> C-a` |  |
| `display-about-screen` | startup |  |  |  |
| `exit-splash-screen` | startup |  |  |  |
| `strokes-compose-complex-stroke` | strokes |  |  |  |
| `strokes-decode-buffer` | strokes |  |  |  |
| `strokes-describe-stroke` | strokes |  |  |  |
| `strokes-do-complex-stroke` | strokes |  |  |  |
| `strokes-do-stroke` | strokes |  |  |  |
| `strokes-global-set-stroke` | strokes |  |  |  |
| `strokes-help` | strokes |  |  |  |
| `strokes-list-strokes` | strokes |  |  |  |
| `strokes-load-user-strokes` | strokes |  |  |  |
| `strokes-mode` | strokes |  |  |  |
| `studlify-buffer` | studly |  |  |  |
| `studlify-region` | studly |  |  |  |
| `studlify-word` | studly |  |  |  |
| `backward-delete-char` | subr |  |  |  |
| `ensure-empty-lines` | subr |  |  |  |
| `fetch-bytecode` | subr |  |  |  |
| `forward-same-syntax` | subr |  |  |  |
| `forward-symbol` | subr |  |  |  |
| `forward-whitespace` | subr |  |  |  |
| `global-set-key` | subr |  |  |  |
| `global-unset-key` | subr |  |  |  |
| `ignore` | subr | `<touch-end>`, `<XF86WakeUp>`, `<iconify-frame>`, `C-M-<drag-mouse-1>` ほか 11 | 同じ |  |
| `local-set-key` | subr |  |  |  |
| `local-unset-key` | subr |  |  |  |
| `locate-library` | subr |  |  |  |
| `mkdir` | subr |  |  |  |
| `read-char-from-minibuffer-insert-char` | subr |  |  |  |
| `read-char-from-minibuffer-insert-other` | subr |  |  |  |
| `remove-hook` | subr |  |  |  |
| `undefined` | subr |  |  |  |
| `y-or-n-p-insert-n` | subr |  |  |  |
| `y-or-n-p-insert-other` | subr |  |  |  |
| `y-or-n-p-insert-y` | subr |  |  |  |
| `global-subword-mode` | subword |  |  |  |
| `global-superword-mode` | subword |  |  |  |
| `subword-mode` | subword |  |  |  |
| `superword-mode` | subword |  |  |  |
| `gpm-mouse-mode` | t-mouse |  |  |  |
| `find-file-other-tab` | tab-bar | `C-x t f`, `C-x t C-f` | 同じ |  |
| `find-file-read-only-other-tab` | tab-bar | `C-x t C-r` | 同じ |  |
| `other-tab-prefix` | tab-bar | `C-x t t` | 同じ |  |
| `switch-to-buffer-other-tab` | tab-bar | `C-x t b` | 同じ |  |
| `tab-bar-change-tab-group` | tab-bar |  |  |  |
| `tab-bar-close-group-tabs` | tab-bar |  |  |  |
| `tab-bar-close-other-tabs` | tab-bar |  |  |  |
| `tab-bar-close-tab` | tab-bar |  |  |  |
| `tab-bar-close-tab-by-name` | tab-bar |  |  |  |
| `tab-bar-detach-tab` | tab-bar |  |  |  |
| `tab-bar-duplicate-tab` | tab-bar |  |  |  |
| `tab-bar-history-back` | tab-bar |  |  |  |
| `tab-bar-history-forward` | tab-bar |  |  |  |
| `tab-bar-history-mode` | tab-bar |  |  |  |
| `tab-bar-menu-bar` | tab-bar |  |  |  |
| `tab-bar-mode` | tab-bar |  |  |  |
| `tab-bar-mouse-1` | tab-bar |  |  |  |
| `tab-bar-mouse-close-tab` | tab-bar |  |  |  |
| `tab-bar-mouse-context-menu` | tab-bar |  |  |  |
| `tab-bar-mouse-down-1` | tab-bar |  |  |  |
| `tab-bar-mouse-move-tab` | tab-bar |  |  |  |
| `tab-bar-move-tab` | tab-bar |  |  |  |
| `tab-bar-move-tab-backward` | tab-bar |  |  |  |
| `tab-bar-move-tab-to` | tab-bar |  |  |  |
| `tab-bar-move-tab-to-frame` | tab-bar |  |  |  |
| `tab-bar-move-tab-to-group` | tab-bar |  |  |  |
| `tab-bar-move-window-to-tab` | tab-bar |  |  |  |
| `tab-bar-new-tab` | tab-bar |  |  |  |
| `tab-bar-new-tab-to` | tab-bar |  |  |  |
| `tab-bar-rename-tab` | tab-bar |  |  |  |
| `tab-bar-rename-tab-by-name` | tab-bar |  |  |  |
| `tab-bar-select-tab` | tab-bar |  |  |  |
| `tab-bar-select-tab-by-name` | tab-bar |  |  |  |
| `tab-bar-switch-to-last-tab` | tab-bar |  |  |  |
| `tab-bar-switch-to-next-tab` | tab-bar |  |  |  |
| `tab-bar-switch-to-prev-tab` | tab-bar |  |  |  |
| `tab-bar-switch-to-recent-tab` | tab-bar |  |  |  |
| `tab-bar-switch-to-tab` | tab-bar |  |  |  |
| `tab-bar-touchscreen-begin` | tab-bar |  |  |  |
| `tab-bar-undo-close-tab` | tab-bar |  |  |  |
| `tab-close` | tab-bar | `C-x t 0` | 同じ |  |
| `tab-close-group` | tab-bar |  |  |  |
| `tab-close-other` | tab-bar | `C-x t 1` | 同じ |  |
| `tab-detach` | tab-bar | `C-x t ^ f` | 同じ |  |
| `tab-duplicate` | tab-bar | `C-x t n` | 同じ |  |
| `tab-group` | tab-bar | `C-x t G` | 同じ |  |
| `tab-last` | tab-bar |  |  |  |
| `tab-list` | tab-bar |  |  |  |
| `tab-move` | tab-bar | `C-x t m` | 同じ |  |
| `tab-move-to` | tab-bar | `C-x t M` | 同じ |  |
| `tab-new` | tab-bar | `C-x t 2` | 同じ |  |
| `tab-new-to` | tab-bar | `C-x t N` | 同じ |  |
| `tab-next` | tab-bar | `C-x t o` | 同じ |  |
| `tab-previous` | tab-bar | `C-x t O` | 同じ |  |
| `tab-recent` | tab-bar |  |  |  |
| `tab-rename` | tab-bar | `C-x t r` | 同じ |  |
| `tab-select` | tab-bar |  |  |  |
| `tab-switch` | tab-bar | `C-x t RET` | 同じ |  |
| `tab-switcher` | tab-bar |  |  |  |
| `tab-switcher-backup-unmark` | tab-bar |  |  |  |
| `tab-switcher-delete` | tab-bar |  |  |  |
| `tab-switcher-delete-backwards` | tab-bar |  |  |  |
| `tab-switcher-execute` | tab-bar |  |  |  |
| `tab-switcher-mode` | tab-bar |  |  |  |
| `tab-switcher-mouse-select` | tab-bar |  |  |  |
| `tab-switcher-next-line` | tab-bar |  |  |  |
| `tab-switcher-prev-line` | tab-bar |  |  |  |
| `tab-switcher-select` | tab-bar |  |  |  |
| `tab-switcher-unmark` | tab-bar |  |  |  |
| `tab-undo` | tab-bar | `C-x t u` | 同じ |  |
| `tab-window-detach` | tab-bar | `C-x w ^ t` | 同じ |  |
| `toggle-frame-tab-bar` | tab-bar |  |  |  |
| `toggle-tab-bar-mode-from-frame` | tab-bar |  |  |  |
| `global-tab-line-mode` | tab-line |  |  |  |
| `tab-line-mode` | tab-line |  |  |  |
| `tabify` | tabify |  |  |  |
| `untabify` | tabify |  |  |  |
| `table-backward-cell` | table |  |  |  |
| `table-capture` | table |  |  |  |
| `table-delete-column` | table |  |  |  |
| `table-delete-row` | table |  |  |  |
| `table-fixed-width-mode` | table |  |  |  |
| `table-forward-cell` | table |  |  |  |
| `table-generate-source` | table |  |  |  |
| `table-heighten-cell` | table |  |  |  |
| `table-insert` | table |  |  |  |
| `table-insert-column` | table |  |  |  |
| `table-insert-row` | table |  |  |  |
| `table-insert-row-column` | table |  |  |  |
| `table-insert-sequence` | table |  |  |  |
| `table-justify` | table |  |  |  |
| `table-justify-cell` | table |  |  |  |
| `table-justify-column` | table |  |  |  |
| `table-justify-row` | table |  |  |  |
| `table-narrow-cell` | table |  |  |  |
| `table-query-dimension` | table |  |  |  |
| `table-recognize` | table |  |  |  |
| `table-recognize-cell` | table |  |  |  |
| `table-recognize-region` | table |  |  |  |
| `table-recognize-table` | table |  |  |  |
| `table-release` | table |  |  |  |
| `table-shorten-cell` | table |  |  |  |
| `table-span-cell` | table |  |  |  |
| `table-split-cell` | table |  |  |  |
| `table-split-cell-horizontally` | table |  |  |  |
| `table-split-cell-vertically` | table |  |  |  |
| `table-unrecognize` | table |  |  |  |
| `table-unrecognize-cell` | table |  |  |  |
| `table-unrecognize-region` | table |  |  |  |
| `table-unrecognize-table` | table |  |  |  |
| `table-widen-cell` | table |  |  |  |
| `tabulated-list-col-sort` | tabulated-list |  |  |  |
| `tabulated-list-mode` | tabulated-list |  |  |  |
| `tabulated-list-narrow-current-column` | tabulated-list |  |  |  |
| `tabulated-list-next-column` | tabulated-list |  |  |  |
| `tabulated-list-previous-column` | tabulated-list |  |  |  |
| `tabulated-list-revert` | tabulated-list |  |  |  |
| `tabulated-list-sort` | tabulated-list |  |  |  |
| `tabulated-list-widen-current-column` | tabulated-list |  |  |  |
| `talk` | talk |  |  |  |
| `talk-connect` | talk |  |  |  |
| `tar-mode` | tar-mode |  |  |  |
| `inferior-tcl` | tcl |  |  |  |
| `tcl-help-on-word` | tcl |  |  |  |
| `tcl-mode` | tcl |  |  |  |
| `rsh` | telnet |  |  |  |
| `telnet` | telnet |  |  |  |
| `ansi-term` | term |  |  |  |
| `serial-term` | term |  |  |  |
| `term` | term |  |  |  |
| `testcover-start` | testcover |  |  |  |
| `testcover-this-defun` | testcover |  |  |  |
| `tetris` | tetris |  |  |  |
| `doctex-mode` | tex-mode |  |  |  |
| `latex-mode` | tex-mode |  |  |  |
| `plain-tex-mode` | tex-mode |  |  |  |
| `slitex-mode` | tex-mode |  |  |  |
| `tex-mode` | tex-mode |  |  |  |
| `texi2info` | texinfmt |  |  |  |
| `texinfo-format-buffer` | texinfmt |  |  |  |
| `texinfo-format-region` | texinfmt |  |  |  |
| `texinfo-mode` | texinfo |  |  |  |
| `center-line` | text-mode |  |  |  |
| `center-paragraph` | text-mode |  |  |  |
| `center-region` | text-mode |  |  |  |
| `indented-text-mode` | text-mode |  |  |  |
| `paragraph-indent-minor-mode` | text-mode |  |  |  |
| `paragraph-indent-text-mode` | text-mode |  |  |  |
| `text-mode` | text-mode |  |  |  |
| `text-mode-menu` | text-mode |  |  |  |
| `toggle-text-mode-auto-fill` | text-mode |  |  |  |
| `thai-compose-buffer` | thai-util |  |  |  |
| `thai-compose-region` | thai-util |  |  |  |
| `list-threads` | thread |  |  |  |
| `thread-handle-event` | thread |  |  |  |
| `tibetan-compose-buffer` | tibet-util |  |  |  |
| `tibetan-compose-region` | tibet-util |  |  |  |
| `tibetan-decompose-buffer` | tibet-util |  |  |  |
| `tibetan-decompose-region` | tibet-util |  |  |  |
| `tildify-buffer` | tildify |  |  |  |
| `tildify-mode` | tildify |  |  |  |
| `tildify-region` | tildify |  |  |  |
| `tildify-space` | tildify |  |  |  |
| `display-time` | time |  |  |  |
| `display-time-mode` | time |  |  |  |
| `emacs-init-time` | time |  |  |  |
| `emacs-uptime` | time |  |  |  |
| `world-clock` | time |  |  |  |
| `time-stamp` | time-stamp |  |  |  |
| `time-stamp-toggle-active` | time-stamp |  |  |  |
| `timeclock-change` | timeclock |  |  |  |
| `timeclock-in` | timeclock |  |  |  |
| `timeclock-mode-line-display` | timeclock |  |  |  |
| `timeclock-out` | timeclock |  |  |  |
| `timeclock-reread-log` | timeclock |  |  |  |
| `timeclock-status-string` | timeclock |  |  |  |
| `timeclock-when-to-leave-string` | timeclock |  |  |  |
| `timeclock-workday-elapsed-string` | timeclock |  |  |  |
| `timeclock-workday-remaining-string` | timeclock |  |  |  |
| `cancel-function-timers` | timer |  |  |  |
| `run-at-time` | timer |  |  |  |
| `run-with-idle-timer` | timer |  |  |  |
| `run-with-timer` | timer |  |  |  |
| `list-timers` | timer-list |  |  |  |
| `tmm-menubar` | tmm | `` M-` `` | 同じ |  |
| `tmm-menubar-mouse` | tmm |  |  |  |
| `todo-archive-mode` | todo-mode |  |  |  |
| `todo-filtered-items-mode` | todo-mode |  |  |  |
| `todo-mode` | todo-mode |  |  |  |
| `todo-show` | todo-mode |  |  |  |
| `toml-ts-mode` | toml-ts-mode |  |  |  |
| `modifier-bar-mode` | tool-bar |  |  |  |
| `toggle-tool-bar-mode-from-frame` | tool-bar |  |  |  |
| `tool-bar-mode` | tool-bar |  |  |  |
| `tooltip-mode` | tooltip |  |  |  |
| `touch-screen-drag` | touch-screen | `<touchscreen-drag>` | 同じ |  |
| `touch-screen-handle-touch` | touch-screen |  |  |  |
| `touch-screen-hold` | touch-screen | `<touchscreen-hold>` | 同じ |  |
| `touch-screen-pinch` | touch-screen | `<touchscreen-pinch>` | 同じ |  |
| `touch-screen-preview-select` | touch-screen |  |  |  |
| `touch-screen-restart-drag` | touch-screen | `<touchscreen-restart-drag>` | 同じ |  |
| `touch-screen-scroll` | touch-screen | `<touchscreen-scroll>` | 同じ |  |
| `trace-function-background` | trace |  |  |  |
| `trace-function-foreground` | trace |  |  |  |
| `treesit-install-language-grammar` | treesit |  |  |  |
| `help-with-tutorial` | tutorial | `C-h t`, `<f1> t`, `<help> t` | `C-h t`, `<help> t` |  |
| `2C-associate-buffer` | two-column | `<f2> b`, `C-x 6 b` | `C-x 6 b` |  |
| `2C-command` | two-column |  |  |  |
| `2C-split` | two-column | `<f2> s`, `C-x 6 s` | `C-x 6 s` |  |
| `2C-two-columns` | two-column | `<f2> 2`, `C-x 6 2`, `<f2> <f2>`, `C-x 6 <f2>` | `C-x 6 2`, `C-x 6 <f2>` |  |
| `type-break` | type-break |  |  |  |
| `type-break-guesstimate-keystroke-threshold` | type-break |  |  |  |
| `type-break-mode` | type-break |  |  |  |
| `type-break-statistics` | type-break |  |  |  |
| `tsx-ts-mode` | typescript-ts-mode |  |  |  |
| `typescript-ts-base-mode` | typescript-ts-mode |  |  |  |
| `typescript-ts-mode` | typescript-ts-mode |  |  |  |
| `underline-region` | underline |  |  |  |
| `ununderline-region` | underline |  |  |  |
| `undigestify-rmail-message` | undigest |  |  |  |
| `unforward-rmail-message` | undigest |  |  |  |
| `uniquify-rationalize-file-buffer-names` | uniquify |  |  |  |
| `unrmail` | unrmail |  |  |  |
| `url-gateway-nslookup-host` | url-gw |  |  |  |
| `url-handler-mode` | url-handlers |  |  |  |
| `url-setup-privacy-info` | url-privacy |  |  |  |
| `url-view-url` | url-util |  |  |  |
| `use-package-jump-to-package-form` | use-package-jump |  |  |  |
| `use-package-lint` | use-package-lint |  |  |  |
| `uudecode-decode-region-external` | uudecode |  |  |  |
| `uudecode-decode-region-internal` | uudecode |  |  |  |
| `vc-change-backend` | vc |  |  |  |
| `vc-create-branch` | vc | `C-x v b c` | 同じ |  |
| `vc-create-tag` | vc | `C-x v s` | 同じ |  |
| `vc-delete-file` | vc | `C-x v x` | 同じ |  |
| `vc-diff` | vc | `C-x v =` | 同じ |  |
| `vc-diff-mergebase` | vc | `C-x v M D` | 同じ |  |
| `vc-ediff` | vc |  |  |  |
| `vc-edit-next-command` | vc | `C-x v !` | 同じ |  |
| `vc-find-conflicted-file` | vc |  |  |  |
| `vc-ignore` | vc | `C-x v G` | 同じ |  |
| `vc-insert-headers` | vc |  |  |  |
| `vc-log-incoming` | vc | `C-x v I` | 同じ |  |
| `vc-log-mergebase` | vc | `C-x v M L` | 同じ |  |
| `vc-log-outgoing` | vc | `C-x v O` | 同じ |  |
| `vc-log-search` | vc |  |  |  |
| `vc-merge` | vc | `C-x v m` | 同じ |  |
| `vc-next-action` | vc | `C-x v v` | 同じ |  |
| `vc-prepare-patch` | vc |  |  |  |
| `vc-print-branch-log` | vc | `C-x v b l` | 同じ |  |
| `vc-print-log` | vc | `C-x v l` | 同じ |  |
| `vc-print-root-log` | vc | `C-x v L` | 同じ |  |
| `vc-pull` | vc |  |  |  |
| `vc-pull-and-push` | vc |  |  |  |
| `vc-push` | vc | `C-x v P` | 同じ |  |
| `vc-region-history` | vc | `C-x v h` | 同じ |  |
| `vc-register` | vc | `C-x v i` | 同じ |  |
| `vc-rename-file` | vc |  |  |  |
| `vc-retrieve-tag` | vc | `C-x v r` | 同じ |  |
| `vc-revert` | vc | `C-x v u` | 同じ |  |
| `vc-revision-other-window` | vc | `C-x v ~` | 同じ |  |
| `vc-root-diff` | vc | `C-x v D` | 同じ |  |
| `vc-root-version-diff` | vc |  |  |  |
| `vc-switch-branch` | vc | `C-x v b s` | 同じ |  |
| `vc-update-change-log` | vc | `C-x v a` | 同じ |  |
| `vc-version-diff` | vc |  |  |  |
| `vc-version-ediff` | vc |  |  |  |
| `vc-annotate` | vc-annotate | `C-x v g` | 同じ |  |
| `vc-dir` | vc-dir | `C-x v d` | 同じ |  |
| `vc-dir-root` | vc-dir |  |  |  |
| `vc-git-grep` | vc-git |  |  |  |
| `vc-find-file-hook` | vc-hooks |  |  |  |
| `vc-mode-line` | vc-hooks |  |  |  |
| `vc-refresh-state` | vc-hooks |  |  |  |
| `vera-mode` | vera-mode |  |  |  |
| `verilog-mode` | verilog-mode |  |  |  |
| `emacs-version` | version |  |  |  |
| `version` | version |  |  |  |
| `vhdl-mode` | vhdl-mode |  |  |  |
| `viet-decode-viqr-buffer` | viet-util |  |  |  |
| `viet-decode-viqr-region` | viet-util |  |  |  |
| `viet-encode-viqr-buffer` | viet-util |  |  |  |
| `viet-encode-viqr-region` | viet-util |  |  |  |
| `View-exit-and-edit` | view |  |  |  |
| `view-buffer` | view |  |  |  |
| `view-buffer-other-frame` | view |  |  |  |
| `view-buffer-other-window` | view |  |  |  |
| `view-file` | view |  |  |  |
| `view-file-other-frame` | view |  |  |  |
| `view-file-other-window` | view |  |  |  |
| `view-mode` | view |  |  |  |
| `toggle-viper-mode` | viper |  |  |  |
| `viper-mode` | viper |  |  |  |
| `global-visual-wrap-prefix-mode` | visual-wrap |  |  |  |
| `visual-wrap-prefix-mode` | visual-wrap |  |  |  |
| `wallpaper-set` | wallpaper |  |  |  |
| `wdired-change-to-wdired-mode` | wdired |  |  |  |
| `webjump` | webjump |  |  |  |
| `which-function-mode` | which-func |  |  |  |
| `which-key-C-h-dispatch` | which-key |  |  |  |
| `which-key-dump-bindings` | which-key |  |  |  |
| `which-key-mode` | which-key |  |  |  |
| `which-key-setup-minibuffer` | which-key |  |  |  |
| `which-key-setup-side-window-bottom` | which-key |  |  |  |
| `which-key-setup-side-window-right` | which-key |  |  |  |
| `which-key-setup-side-window-right-bottom` | which-key |  |  |  |
| `which-key-show-full-keymap` | which-key |  |  |  |
| `which-key-show-full-major-mode` | which-key |  |  |  |
| `which-key-show-full-minor-mode-keymap` | which-key |  |  |  |
| `which-key-show-keymap` | which-key |  |  |  |
| `which-key-show-major-mode` | which-key |  |  |  |
| `which-key-show-minor-mode-keymap` | which-key |  |  |  |
| `which-key-show-next-page-cycle` | which-key |  |  |  |
| `which-key-show-next-page-no-cycle` | which-key |  |  |  |
| `which-key-show-previous-page-cycle` | which-key |  |  |  |
| `which-key-show-previous-page-no-cycle` | which-key |  |  |  |
| `which-key-show-standard-help` | which-key |  |  |  |
| `which-key-show-top-level` | which-key |  |  |  |
| `which-key-undo-key` | which-key |  |  |  |
| `global-whitespace-mode` | whitespace |  | `C-^` |  |
| `global-whitespace-newline-mode` | whitespace |  |  |  |
| `global-whitespace-toggle-options` | whitespace |  |  |  |
| `whitespace-cleanup` | whitespace |  |  |  |
| `whitespace-cleanup-region` | whitespace |  |  |  |
| `whitespace-mode` | whitespace |  |  |  |
| `whitespace-newline-mode` | whitespace |  |  |  |
| `whitespace-report` | whitespace |  |  |  |
| `whitespace-report-region` | whitespace |  |  |  |
| `whitespace-toggle-options` | whitespace |  |  |  |
| `widget-browse` | wid-browse |  |  |  |
| `widget-browse-at` | wid-browse |  |  |  |
| `widget-browse-other-window` | wid-browse |  |  |  |
| `widget-minor-mode` | wid-browse |  |  |  |
| `windmove-default-keybindings` | windmove |  |  |  |
| `windmove-delete-default-keybindings` | windmove |  |  |  |
| `windmove-delete-down` | windmove |  |  |  |
| `windmove-delete-left` | windmove |  |  |  |
| `windmove-delete-right` | windmove |  |  |  |
| `windmove-delete-up` | windmove |  |  |  |
| `windmove-display-default-keybindings` | windmove |  |  |  |
| `windmove-display-down` | windmove |  |  |  |
| `windmove-display-left` | windmove |  |  |  |
| `windmove-display-new-frame` | windmove |  |  |  |
| `windmove-display-new-tab` | windmove |  |  |  |
| `windmove-display-right` | windmove |  |  |  |
| `windmove-display-same-window` | windmove |  |  |  |
| `windmove-display-up` | windmove |  |  |  |
| `windmove-down` | windmove |  |  |  |
| `windmove-left` | windmove |  |  |  |
| `windmove-mode` | windmove |  |  |  |
| `windmove-right` | windmove |  |  |  |
| `windmove-swap-states-default-keybindings` | windmove |  |  |  |
| `windmove-swap-states-down` | windmove |  |  |  |
| `windmove-swap-states-left` | windmove |  |  |  |
| `windmove-swap-states-right` | windmove |  |  |  |
| `windmove-swap-states-up` | windmove |  |  |  |
| `windmove-up` | windmove |  |  |  |
| `balance-windows` | window | `C-x +` | 同じ |  |
| `balance-windows-area` | window |  |  |  |
| `beginning-of-buffer-other-window` | window | `M-<home>`, `M-<begin>`, `ESC <home>`, `ESC <begin>` | 同じ |  |
| `bury-buffer` | window |  |  |  |
| `delete-other-windows` | window | `C-x 1` | 同じ | ✓ `C-x 1` |
| `delete-other-windows-vertically` | window |  |  |  |
| `delete-window` | window | `C-x 0` | 同じ | ✓ `C-x 0` |
| `delete-windows-on` | window | `C-x w 0` | 同じ |  |
| `display-buffer` | window | `C-x 4 C-o` | 同じ |  |
| `display-buffer-other-frame` | window | `C-x 5 C-o` | 同じ |  |
| `end-of-buffer-other-window` | window | `M-<end>`, `ESC <end>` | 同じ |  |
| `enlarge-window` | window | `C-x ^` | 同じ |  |
| `enlarge-window-horizontally` | window | `C-x }` | 同じ |  |
| `fit-frame-to-buffer` | window |  |  |  |
| `fit-window-to-buffer` | window | `C-x w -` | 同じ |  |
| `handle-select-window` | window | `<select-window>` | 同じ |  |
| `kill-buffer-and-window` | window | `C-x 4 0` | 同じ |  |
| `maximize-window` | window |  |  |  |
| `minimize-window` | window |  |  |  |
| `move-to-window-line-top-bottom` | window | `M-r` | 同じ |  |
| `next-buffer` | window | `C-x <right>`, `<XF86Forward>`, `C-x C-<right>` | 同じ |  |
| `other-window` | window | `C-x o` | 同じ | ✓ `C-x o` |
| `other-window-prefix` | window | `C-x 4 4` | 同じ |  |
| `pop-to-buffer` | window |  |  |  |
| `previous-buffer` | window | `<XF86Back>`, `C-x <left>`, `C-x C-<left>` | 同じ |  |
| `quit-window` | window | `C-x w q` | 同じ |  |
| `quit-windows-on` | window |  |  |  |
| `recenter-other-window` | window | `C-M-S-l` | 同じ |  |
| `recenter-top-bottom` | window | `C-l` | 同じ | ✓ `C-l` |
| `replace-buffer-in-windows` | window |  |  |  |
| `same-window-prefix` | window | `C-x 4 1` | 同じ |  |
| `scroll-down-command` | window | `M-v`, `<prior>` | 同じ | ✓ `M-v` |
| `scroll-down-line` | window |  |  |  |
| `scroll-other-window` | window | `C-M-v`, `M-<next>`, `ESC <next>` | 同じ |  |
| `scroll-other-window-down` | window | `C-M-S-v`, `M-<prior>`, `ESC <prior>` | 同じ |  |
| `scroll-up-command` | window | `C-v`, `<next>` | 同じ | ✓ `C-v` |
| `scroll-up-line` | window |  |  |  |
| `shrink-window` | window |  |  |  |
| `shrink-window-horizontally` | window | `C-x {` | 同じ |  |
| `shrink-window-if-larger-than-buffer` | window | `C-x -` | 同じ |  |
| `split-root-window-below` | window | `C-x w 2` | 同じ |  |
| `split-root-window-right` | window | `C-x w 3` | 同じ |  |
| `split-window-below` | window | `C-x 2` | 同じ | ✓ `C-x 2` |
| `split-window-horizontally` | window |  |  |  |
| `split-window-right` | window | `C-x 3` | 同じ | ✓ `C-x 3` |
| `split-window-vertically` | window |  |  |  |
| `switch-to-buffer` | window | `C-x b` | 同じ | ✓ `C-x b` |
| `switch-to-buffer-other-frame` | window | `C-x 5 b` | 同じ |  |
| `switch-to-buffer-other-window` | window | `C-x 4 b` | 同じ |  |
| `switch-to-next-buffer` | window |  |  |  |
| `switch-to-prev-buffer` | window |  |  |  |
| `toggle-window-dedicated` | window | `C-x w d` | 同じ |  |
| `unbury-buffer` | window |  |  |  |
| `window-swap-states` | window |  |  |  |
| `window-toggle-side-windows` | window | `C-x w s` | 同じ |  |
| `global-window-tool-bar-mode` | window-tool-bar |  |  |  |
| `window-tool-bar-mode` | window-tool-bar |  |  |  |
| `winner-mode` | winner |  |  |  |
| `woman` | woman |  |  |  |
| `woman-dired-find-file` | woman |  |  |  |
| `woman-find-file` | woman |  |  |  |
| `global-word-wrap-whitespace-mode` | word-wrap-mode |  |  |  |
| `word-wrap-whitespace-mode` | word-wrap-mode |  |  |  |
| `x-dnd-handle-drag-n-drop-event` | x-dnd |  |  |  |
| `x-clipboard-yank` | x-win |  |  |  |
| `x-menu-bar-open` | x-win |  |  |  |
| `x-preedit-text` | x-win |  |  |  |
| `xref-find-apropos` | xref | `C-M-.` | 同じ |  |
| `xref-find-definitions` | xref | `M-.` | 同じ |  |
| `xref-find-definitions-at-mouse` | xref |  |  |  |
| `xref-find-definitions-other-frame` | xref | `C-x 5 .` | 同じ |  |
| `xref-find-definitions-other-window` | xref | `C-x 4 .` | 同じ |  |
| `xref-find-references` | xref | `M-?` | 同じ |  |
| `xref-find-references-at-mouse` | xref |  |  |  |
| `xref-go-back` | xref | `M-,` | 同じ |  |
| `xref-go-forward` | xref | `C-M-,` | 同じ |  |
| `xterm-mouse-mode` | xt-mouse |  |  |  |
| `xwidget-webkit-browse-url` | xwidget |  |  |  |
| `yaml-ts-mode` | yaml-ts-mode |  |  |  |
| `yank-media` | yank-media |  |  |  |
| `yenc-decode-region` | yenc |  |  |  |
| `zone` | zone |  |  |  |

<!-- emacs-commands:end -->
