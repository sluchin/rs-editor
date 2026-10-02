# 4. Scheme でのプログラミング

> **原文**: [Guile Reference Manual - Programming in Scheme](https://www.gnu.org/software/guile/manual/guile.html#Programming-in-Scheme)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、Guile を使用して実際に Scheme プログラムを開発する方法を説明します。

## 4.1 Guile の Scheme 実装

### 概要

Guile は複数の Scheme 標準規格をサポートしており、多くの SRFI（Scheme Requests for Implementation）モジュールも提供しています。

### 標準化への対応

```scheme
; R5RS 準拠のコード
(define (factorial n)
  (if (<= n 1) 1 (* n (factorial (- n 1)))))

; R6RS/R7RS の追加機能
(define-record-type <point>
  (make-point x y)
  point?
  (x point-x)
  (y point-y))

; SRFI-1（拡張リスト処理）
(use-modules (srfi srfi-1))
(map (lambda (x) (* x 2)) (iota 5))
```

### 標準規格の違い

- **R5RS**: 基本的な Scheme 標準、シンプルで汎用的
- **R6RS**: より包括的、モジュールシステムとライブラリを追加
- **R7RS**: R5RS と R6RS の中間、軽量で実用的
- **SRFI**: 特定の機能（リスト処理、正規表現など）を定義

## 4.2 Guile の起動

### 概要

Guile はコマンドラインから柔軟に起動でき、複数の起動オプションが提供されています。

### コマンドラインオプション

```bash
guile [options] [script] [args]
```

主要なオプション：

```bash
# インタラクティブモード（デフォルト）
guile

# スクリプトファイルを実行
guile script.scm

# 式を評価して終了
guile -c "(+ 2 3)"

# REPL を起動した後にスクリプトを実行
guile -l script.scm

# バージョン確認
guile --version
```

### 環境変数の設定

```bash
# モジュール検索パスを設定
export GUILE_LOAD_PATH=/path/to/modules:$GUILE_LOAD_PATH

# コンパイル済みモジュールのキャッシュ
export GUILE_LOAD_COMPILED_PATH=/path/to/compiled
```

## 4.3 Guile スクリプティング

### 概要

Scheme スクリプトファイルを直接実行可能にし、シェルスクリプトのように使用できます。

### スクリプトファイルの構造

```scheme
#!/usr/bin/guile -s
!#

; スクリプト本体
(define (main args)
  (format #t "Hello from Guile!~n"))

(main (cdr (program-arguments)))
```

### メタスイッチの説明

`#!/usr/bin/guile -s` 行：
- `#!` - シェバン（実行ビット付きファイルの指定）
- `/usr/bin/guile` - Guile インタプリタへのパス
- `-s` - スクリプトモード（以降のコードをスクリプトとして実行）
- `!#` - Guile メタブロックの終了マーク

### コマンドライン引数の処理

```scheme
#!/usr/bin/guile -s
!#

; 引数を処理する
(define (process-args args)
  (cond ((null? args) (display "No arguments\n"))
        ((> (length args) 1) (display "Multiple arguments\n"))
        (else (display (car args)) (newline))))

(process-args (cdr (program-arguments)))
```

### スクリプティングの実践例

#### ファイル処理

```scheme
#!/usr/bin/guile -s
!#

(define (count-lines filename)
  (let ((lines 0))
    (with-input-from-file filename
      (lambda ()
        (while (not (eof-object? (read-line)))
          (set! lines (+ lines 1)))))
    lines))

(let ((file (car (cdr (program-arguments)))))
  (format #t "Line count: ~a~n" (count-lines file)))
```

#### テキスト変換

```scheme
#!/usr/bin/guile -s
!#

(use-modules (ice-9 rdelim))

(define (transform-text)
  (let loop ()
    (let ((line (read-line)))
      (unless (eof-object? line)
        (display (string-upcase line))
        (newline)
        (loop)))))

(transform-text)
```

## 4.4 Guile を対話的に使用する

### 概要

REPL（Read-Eval-Print Loop）を使用して、対話的にコードを実行・テストできます。

### init ファイル

`~/.guile` ファイルを作成すると、Guile 起動時に自動的に実行されます。

```scheme
; ~/.guile
(display "Welcome to Guile!\n")

; 便利な関数を定義
(define (square x) (* x x))
(define (cube x) (* x x x))
```

### Readline サポート

Guile は GNU Readline ライブラリをサポートし、強力な行編集機能を提供：

```bash
scheme@(guile-user)> (+ 2 3)  ; 矢印キーで履歴を参照
5
scheme@(guile-user)> (define x 10)
scheme@(guile-user)> x
10
```

### 値の履歴

前の評価結果は特殊な変数で参照できます：

```scheme
scheme@(guile-user)> (+ 2 3)
5
scheme@(guile-user)> $1
5
scheme@(guile-user)> (* $1 2)
10
scheme@(guile-user)> $2
10
```

### REPL コマンド

REPL では以下の特殊コマンドが利用可能です：

```scheme
; ヘルプの表示
scheme@(guile-user)> ,help

; モジュール操作
scheme@(guile-user)> ,module (srfi srfi-1)

; 言語切り替え
scheme@(guile-user)> ,language scheme

; 式の検査
scheme@(guile-user)> ,describe map

; プロファイリング開始
scheme@(guile-user)> ,profile (compute-heavy-task)

; デバッグコマンド
scheme@(guile-user)> ,trace (function-name)
scheme@(guile-user)> ,break (lambda () ...)
```

### エラーハンドリング

Guile は詳細なエラーメッセージとスタックトレースを提供：

```scheme
scheme@(guile-user)> (car (cdr (cdr '(1 2))))
ERROR: In procedure car:
ERROR: Wrong type argument: ()

scheme@(guile-user)> (/ 1 0)
ERROR: In procedure /:
ERROR: Division by zero
```

### 対話的デバッグ

```scheme
; 関数の実行トレース
scheme@(guile-user)> (trace my-function)
scheme@(guile-user)> (my-function 5)
; my-function の呼び出しと戻り値が表示される

; トレース終了
scheme@(guile-user)> (untrace my-function)
```

## 4.5 Emacs での Guile 使用

### 概要

Guile は Emacs と統合でき、Emacs Lisp と Scheme の共存が可能です。

### Geiser インテグレーション

`geiser` パッケージを Emacs にインストールすると、Guile との対話が容易になります。

```elisp
; .emacs
(require 'geiser-guile)
(setq geiser-active-implementations '(guile))
```

### Scheme ファイルの編集と実行

Emacs で `.scm` ファイルを編集するとき：

- `C-c C-z` - Guile REPL を起動
- `C-c C-c` - 定義をコンパイル
- `C-c C-l` - ファイルをロード
- `C-c C-r` - リージョンを評価

## 4.6 Guile ツール

### 概要

Guile は開発・保守を支援する様々なコマンドラインツールを提供します。

### 主要なツール

```bash
# Scheme ファイルのコンパイル
guile-tools compile script.scm

# ドキュメントの表示
guile-tools doc procedure-name

# REPL の起動（フル機能）
guile

# スクリプト実行
guile script.scm arg1 arg2
```

### スクリプト管理

Guile スクリプトは実行ビットを付けることで直接実行可能：

```bash
chmod +x script.scm
./script.scm arg1 arg2
```

## 4.7 モジュールの配布とインストール

### 概要

Scheme モジュールをシステムにインストールして、複数のプロジェクトで再利用できます。

### モジュールの配置

```
my-project/
├── modules/
│   └── my-lib/
│       ├── utilities.scm
│       └── math.scm
├── configure.ac
└── Makefile.in
```

### インストール手順

```bash
./configure
make
make install

# インストール後、モジュールは自動的に検出可能
(use-modules (my-lib utilities))
```

### モジュール設定の例

```scheme
; modules/my-lib/utilities.scm
(define-module (my-lib utilities)
  #:use-module (srfi srfi-1)
  #:export (double triple process-list))

(define (double x) (* x 2))
(define (triple x) (* x 3))
(define (process-list lst) (map double lst))
```

## 4.8 Guile コードの配布

### 概要

Scheme プログラムを他のユーザーに配布するための方法とベストプラクティス。

### 配布パッケージの構成

```
package-name/
├── README
├── AUTHORS
├── COPYING
├── configure.ac
├── Makefile.in
├── src/
│   └── main.scm
├── tests/
│   └── test-main.scm
└── docs/
    └── manual.md
```

### ライセンスと著作権

- **GPL**: フリーソフトウェア、変更後も公開義務
- **LGPL**: ライブラリとしての使用に適切
- **BSD/MIT**: シンプルで寛容なライセンス

### ドキュメント作成

```scheme
; コメント付きコード例
(define (factorial n)
  "計算 n!（n の階乗）
   Args:
     n - 非負整数
   Returns:
     n の階乗"
  (if (<= n 1) 1
      (* n (factorial (- n 1)))))
```

### テストの含含

```bash
# テスト実行
make check

# カバレッジ測定
guile-tools coverage tests/test-*.scm
```

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
