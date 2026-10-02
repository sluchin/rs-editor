# 3. Scheme へようこそ

> **原文**: [Guile Reference Manual - Hello Scheme!](https://www.gnu.org/software/guile/manual/guile.html#Hello-Scheme)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、Scheme プログラミング言語の基本的な概念と構文を紹介します。

## 3.1 データ型、値、変数

### 3.1.1 遅延型付け（Latent Typing）

Scheme は動的型付け言語です。つまり、変数の型は実行時に決定されます。同じ変数に異なる型の値を割り当てることができます。

```scheme
(define x 42)        ; x は数値
(set! x "hello")     ; x は文字列に変更
(set! x '(1 2 3))    ; x はリストに変更
```

### 3.1.2 値と変数

- **値**: 数値、文字列、リスト、ブール値など、プログラムが操作するデータ
- **変数**: 値を保持する名前付きの場所

### 3.1.3 変数の定義と設定

```scheme
; 変数の定義（初期化）
(define answer 42)

; 変数の値を変更
(set! answer 100)

; 複数の変数定義
(define x 10)
(define y 20)
```

## 3.2 手続き（プロシージャ）の表現と使用

### 3.2.1 値としての手続き

Scheme では手続き（関数）も値として扱われます。手続きは変数に格納したり、他の手続きに引数として渡したり、戻り値として返したりできます。

```scheme
; 組み込み手続きの確認
scheme@(guile-user)> +
#<procedure + (#:optional _ _ . _)>

; 手続きの呼び出し
scheme@(guile-user)> (+ 2 3)
5
```

### 3.2.2 簡単な手続き呼び出し

```scheme
; 基本的な算術演算
(+ 10 20)           ; 30
(* 3 4)             ; 12
(/ 10 2)            ; 5
(- 100 25)          ; 75

; リスト処理
(length '(1 2 3))   ; 3
(car '(1 2 3))      ; 1
(cdr '(1 2 3))      ; (2 3)
```

### 3.2.3 新しい手続きの作成

```scheme
; lambda で無名手続きを作成
(define square (lambda (x) (* x x)))
(square 5)          ; 25

; define で直接定義
(define (double x) (* 2 x))
(double 7)          ; 14

; 複数の引数
(define (add-three a b c) (+ a b c))
(add-three 1 2 3)   ; 6
```

### 3.2.4 Lambda の代替方法

```scheme
; let を使用した局所的なバインディング
(let ((x 5) (y 10))
  (+ x y))          ; 15

; let* を使用（前の変数を次の定義で参照可能）
(let* ((x 5)
       (y (* x 2)))
  (+ x y))          ; 15

; letrec を使用（再帰関数用）
(letrec ((fact (lambda (n)
                 (if (<= n 1) 1
                     (* n (fact (- n 1)))))))
  (fact 5))         ; 120
```

## 3.3 式と評価

### 3.3.1 評価される式の種類

#### リテラルデータの評価

```scheme
42                          ; 数値 => 42
"hello"                     ; 文字列 => "hello"
#t                          ; ブール値（真） => #t
#f                          ; ブール値（偽） => #f
'(1 2 3)                    ; クォートされたリスト => (1 2 3)
```

#### 3.3.1.2 変数参照の評価

```scheme
(define x 10)
x                           ; 変数参照 => 10
(define pi 3.14159)
pi                          ; => 3.14159
```

#### 3.3.1.3 手続き呼び出し式の評価

```scheme
(+ 2 3)                     ; 手続き + を呼び出し => 5
(* (+ 2 3) 4)               ; ネストされた呼び出し => 20
(length (list 1 2 3))       ; => 3
```

#### 3.3.1.4 特殊構文式の評価

```scheme
(if (> x 5) "large" "small")   ; 条件分岐

(cond ((< x 0) "negative")     ; 複数の条件
      ((= x 0) "zero")
      (else "positive"))
```

### 3.3.2 尾呼び（Tail Calls）

Scheme は尾呼び最適化をサポートしており、末尾の手続き呼び出しはスタック領域を消費しません。

```scheme
; 尾呼び最適化版の階乗
(define (factorial n acc)
  (if (= n 0)
      acc
      (factorial (- n 1) (* n acc))))  ; 尾呼び

(factorial 5 1)         ; 120
```

### 3.3.3 Guile REPL の使用

```bash
$ guile
scheme@(guile-user)> (define x 5)
scheme@(guile-user)> (+ x 3)
8
scheme@(guile-user)> (exit)
```

### 3.3.4 一般的な構文のまとめ

| 構文 | 説明 | 例 |
|------|------|-----|
| `(define name value)` | 変数を定義 | `(define x 10)` |
| `(lambda (x) expr)` | 無名手続きを作成 | `(lambda (x) (* x 2))` |
| `(if test true false)` | 条件分岐 | `(if (> x 5) "yes" "no")` |
| `(cond (t1 e1) (t2 e2))` | 複数条件 | `(cond ((< x 0) "neg") (else "pos"))` |
| `(let ((x val)) expr)` | 局所変数をバインド | `(let ((x 1)) (+ x 2))` |
| `(quote x)` または `'x` | クォート | `'(1 2 3)` |

## 3.4 クロージャの概念

### 3.4.1 名前、位置、値、環境

Scheme では、変数は環境内の位置に結合されます。環境はスコープを定義し、変数の有効範囲を決めます。

### 3.4.2 局所変数と環境

```scheme
(define (outer)
  (let ((x 10))
    (lambda () x)))        ; x の値を返すクロージャ

(define f (outer))
(f)                        ; 10
```

### 3.4.3 環境の連鎖

ネストされた関数定義では、内部の関数が外部の関数の変数にアクセスできます。

```scheme
(define (make-adder n)
  (lambda (x) (+ x n)))    ; n を使用するクロージャ

(define add5 (make-adder 5))
(add5 3)                   ; 8
```

### 3.4.4 字句スコープ（Lexical Scope）

Scheme のデフォルトスコープは字句スコープです。つまり、変数のスコープはそれが定義されたコード内に限定されます。

### 3.4.5 クロージャ

クロージャは、定義時の環境を「捉える」手続きです。

```scheme
(define (make-counter)
  (let ((count 0))
    (lambda ()
      (set! count (+ count 1))
      count)))

(define counter (make-counter))
(counter)                  ; 1
(counter)                  ; 2
(counter)                  ; 3
```

### 3.4.6 例1: シリアル番号ジェネレータ

```scheme
(define make-serial-number
  (let ((count 0))
    (lambda ()
      (set! count (+ count 1))
      count)))

(make-serial-number)       ; 1
(make-serial-number)       ; 2
```

### 3.4.7 例2: 永続的な状態管理

```scheme
(define (make-bank-account initial-balance)
  (let ((balance initial-balance))
    (lambda (amount)
      (if (>= balance amount)
          (begin
            (set! balance (- balance amount))
            balance)
          "不十分です"))))

(define account (make-bank-account 100))
(account 30)               ; 70
(account 50)               ; 20
(account 100)              ; "不十分です"
```

### 3.4.8 例3: コールバッククロージャ問題

クロージャを使用してコールバック関数を作成する際、変数の更新に注意が必要です。

### 3.4.9 例4: オブジェクト指向プログラミング

クロージャでオブジェクトの状態と振る舞いをカプセル化できます。

```scheme
(define (make-point x y)
  (define (get-x) x)
  (define (get-y) y)
  (define (move dx dy)
    (make-point (+ x dx) (+ y dy)))
  
  (lambda (msg)
    (cond ((eq? msg 'x) (get-x))
          ((eq? msg 'y) (get-y))
          ((eq? msg 'move) move))))

(define p (make-point 10 20))
((p 'x))                  ; 10
((p 'y))                  ; 20
```

## 3.5 参考資料

Scheme の学習についてさらに詳しく知りたい場合：

- **入門書**: "Structure and Interpretation of Computer Programs" (SICP)
- **公式レポート**: R7RS Scheme 標準
- **Guile ドキュメント**: https://www.gnu.org/software/guile/manual/
- **オンラインリソース**: https://www.schemers.org/

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
