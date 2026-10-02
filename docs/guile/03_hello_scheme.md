# 3. Scheme へようこそ

> **原文**: [Guile Reference Manual - Hello Scheme!](https://www.gnu.org/software/guile/manual/guile.html#Hello-Scheme)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、Scheme プログラミング言語の基本的な概念と構文を紹介します。

## 3.1 データ型、値、変数

### 遅延型付け（Latent Typing）

Scheme は動的型付け言語です。つまり、変数の型は実行時に決定されます。

```scheme
scheme@(guile-user)> (define x 42)         ; x は数値
scheme@(guile-user)> (define y "hello")    ; y は文字列
scheme@(guile-user)> (define z '(1 2 3))   ; z はリスト
```

### 値と変数

- **値**: 数値、文字列、リスト、ブール値など、プログラムが操作するデータ
- **変数**: 値を保持する名前付きの場所

### 変数の定義と設定

```scheme
; 変数の定義
(define answer 42)

; 変数の値を変更
(set! answer 100)
```

## 3.2 手続き（プロシージャ）の表現と使用

### 値としての手続き

Scheme では手続き（関数）も値として扱われます。

```scheme
; 組み込み手続き
scheme@(guile-user)> +
#<procedure + (#:optional _ _ . _)>

; 手続きの呼び出し
scheme@(guile-user)> (+ 2 3)
5
```

### 簡単な手続き呼び出し

```scheme
; 基本的な算術演算
(+ 10 20)           ; 30
(* 3 4)             ; 12
(/ 10 2)            ; 5
(- 100 25)          ; 75
```

### 新しい手続きの作成

```scheme
; lambda で無名手続きを作成
(define square (lambda (x) (* x x)))
scheme@(guile-user)> (square 5)
25

; define で直接定義
(define (double x) (* 2 x))
scheme@(guile-user)> (double 7)
14
```

### Lambda の代替方法

```scheme
; let を使用した局所的なバインディング
(let ((x 5) (y 10))
  (+ x y))

; let* を使用（前の変数を次の定義で参照可能）
(let* ((x 5)
       (y (* x 2)))
  (+ x y))
```

## 3.3 式と評価

### 式の評価

Scheme プログラムは式の評価で構成されています。各式は値に評価されます。

### リテラルデータの評価

```scheme
42                          ; 数値 => 42
"hello"                     ; 文字列 => "hello"
#t                          ; ブール値 => #t（真）
'(1 2 3)                    ; クォートされたリスト => (1 2 3)
```

### 変数参照の評価

```scheme
(define x 10)
x                           ; 変数参照 => 10
```

### 手続き呼び出し式の評価

```scheme
(+ 2 3)                     ; 手続き + を呼び出し => 5
(* (+ 2 3) 4)               ; ネストされた呼び出し => 20
```

### 特殊構文式の評価

```scheme
(if (> x 5)                 ; 条件分岐
    "x is large"
    "x is small")

(cond ((< x 0) "negative")  ; 複数の条件
      ((= x 0) "zero")
      (else "positive"))
```

### 尾呼び（Tail Calls）

Scheme は尾呼び最適化をサポートしており、末尾の手続き呼び出しはスタック領域を消費しません。

```scheme
(define (factorial n acc)
  (if (= n 0)
      acc
      (factorial (- n 1) (* n acc))))  ; 尾呼び
```

### Guile REPL の使用

```bash
$ guile
scheme@(guile-user)> (define x 5)
scheme@(guile-user)> (+ x 3)
8
scheme@(guile-user)> (exit)
```

### 一般的な構文のまとめ

- `(define name value)` - 変数を定義
- `(lambda (x) expr)` - 無名手続きを作成
- `(if condition true-expr false-expr)` - 条件分岐
- `(cond (test1 expr1) (test2 expr2) ...)` - 複数条件
- `(let ((x val)) expr)` - 局所変数をバインド

## 3.4 クロージャの概念

### 名前、位置、値、環境

Scheme では、変数は環境内の位置に結合されます。環境はスコープを定義します。

### 局所変数と環境

```scheme
(define (make-counter)
  (let ((count 0))
    (lambda ()
      (set! count (+ count 1))
      count)))

(define counter (make-counter))
(counter)  ; 1
(counter)  ; 2
(counter)  ; 3
```

### クロージャ

クロージャは、定義時の環境を「捉える」手続きです。この例では、`lambda` で作成された手続きが `count` 変数を捉えています。

### 環境の連鎖

ネストされた関数定義では、内部の関数が外部の関数の変数にアクセスできます。

### 字句スコープ（Lexical Scope）

Scheme のデフォルトスコープは字句スコープです。つまり、変数のスコープはそれが定義されたコード内に限定されます。

### 実例

クロージャを使用した実装例：

1. **シリアル番号ジェネレータ**: 呼び出すたびに番号をインクリメント
2. **永続的な状態管理**: クロージャで状態を保持
3. **コールバック**: 外部のコンテキストにアクセスするコールバック関数
4. **オブジェクト指向プログラミング**: クロージャでオブジェクトの状態と振る舞いをカプセル化

## 3.5 参考資料

Scheme の学習についてさらに詳しく知りたい場合：

- **入門書**: "Structure and Interpretation of Computer Programs" (SICP)
- **公式レポート**: R7RS Scheme 標準
- **Guile ドキュメント**: https://www.gnu.org/software/guile/manual/

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
