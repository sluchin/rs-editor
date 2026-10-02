# 6. API リファレンス

> **原文**: [Guile Reference Manual - API Reference](https://www.gnu.org/software/guile/manual/guile.html#API-Reference)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、Guile が提供する主要な API（Application Programming Interface）について説明します。C プログラムから Guile を使用する際の関数・型定義が含まれます。

## 6.1 Guile API の概要

### 概要

Guile API は、以下の目的で設計されています：

- Scheme コード内での値の操作
- C コードから Scheme 関数の呼び出し
- Scheme から C 関数へのアクセス
- メモリ管理とガベージコレクション

### 主要な構成要素

#### 初期化とブート

```c
#include <libguile.h>

/* Guile 環境を初期化 */
scm_boot_guile(argc, argv, inner_main, NULL);
scm_init_guile();
```

#### SCM データ型

すべての Scheme 値は `SCM` 型で表現：

```c
SCM value;                /* Scheme の値 */
int c_int = scm_to_int(value);     /* 型変換 */
SCM scm_int = scm_from_int(42);    /* 逆変換 */
```

#### ガベージコレクション

```c
scm_gc_protect_object(object);      /* GC から保護 */
scm_gc_unprotect_object(object);    /* 保護解除 */
```

## 6.2 非推奨機能

### 概要

Guile は古い API の後方互換性のため、一部の関数を非推奨としています。新しいコードでは非推奨関数の使用を避けるべきです。

### 非推奨から新機能への移行

| 非推奨関数 | 新関数 | 注記 |
|-----------|--------|------|
| `SCM_INUMP(x)` | `scm_is_integer(x)` | 型チェック |
| `SCM_INUM(x)` | `scm_to_int(x)` | 値取得 |
| `SCM_MAKINUM(i)` | `scm_from_int(i)` | 値作成 |

## 6.3 SCM 型と値表現

### 概要

すべての Scheme 値は `SCM` 型で表現されます。SCM は不透明な値で、型情報をエンコード化しています。

### SCM 型の基本

```c
#include <libguile.h>

/* SCM 値の作成と変換 */
SCM x = scm_from_int(42);           /* 整数 42 を作成 */
int i = scm_to_int(x);              /* SCM から int に変換 */

/* 型チェック */
if (scm_is_integer(x)) {
  printf("x is an integer\n");
}

/* NULL 値（アンバインド）*/
SCM unbound = SCM_UNBOUND;
```

### C から Scheme への値変換

```c
/* 整数 */
SCM scm_int = scm_from_int(42);
SCM scm_long = scm_from_long(1000000L);

/* 浮動小数点数 */
SCM scm_double = scm_from_double(3.14);

/* 文字列 */
SCM scm_str = scm_from_locale_string("hello");

/* ブール値 */
SCM scm_true = scm_from_bool(1);
SCM scm_false = scm_from_bool(0);

/* シンボル */
SCM scm_sym = scm_from_locale_symbol("my-symbol");
```

### Scheme から C への値変換

```c
/* 整数取得 */
int i = scm_to_int(scm_value);
long l = scm_to_long(scm_value);

/* 浮動小数点数取得 */
double d = scm_to_double(scm_value);

/* 文字列取得 */
char *str = scm_to_locale_string(scm_value);
free(str);  /* 自分で解放する必要あり */

/* ブール値チェック */
int is_true = scm_is_true(scm_value);
```

## 6.4 Guile の初期化

### 概要

Guile を使用する前に、適切に初期化する必要があります。

### 初期化関数

```c
/* メイン関数内から Guile を初期化 */
static void *
inner_main(void *data)
{
  /* ここで Guile を使用 */
  SCM result = scm_c_eval_string("(+ 2 3)");
  printf("Result: %d\n", scm_to_int(result));
  return NULL;
}

int
main(int argc, char *argv[])
{
  /* Guile 環境を初期化（argc と argv を渡す）*/
  scm_boot_guile(argc, argv, inner_main, NULL);
  return 0;
}
```

### スレッド化された使用

```c
/* マルチスレッド対応初期化 */
scm_init_guile();

/* スレッド内で Guile を使用 */
void *
thread_proc(void *data)
{
  scm_c_eval_string("(display \"From thread\\n\")");
  return NULL;
}
```

### 環境変数の影響

```c
/* 環境変数で初期化をカスタマイズ */
setenv("GUILE_LOAD_PATH", "/custom/path", 1);
scm_boot_guile(argc, argv, inner_main, NULL);
```

## 6.5 スナーフィングマクロ

### 概要

C コードから Scheme 手続きを自動生成するマクロ。

### SCM_DEFINE マクロ

```c
#include <libguile.h>

/* Scheme から呼び出し可能な C 関数 */
SCM_DEFINE(my_double, "my-double", 1, 0, 0,
           (SCM x),
           "引数を 2 倍にする")
{
  return scm_from_int(2 * scm_to_int(x));
}

/* 初期化時に以下を呼び出す */
#include "my_module.x"  /* スナーフィング生成コード */
```

### ドキュメント文字列

```c
SCM_DEFINE(process_data, "process-data", 2, 1, 0,
           (SCM input, SCM mode, SCM options),
           "データを処理する\n\n"
           "引数:\n"
           "  input - 処理対象のデータ\n"
           "  mode - 処理モード（'fast または 'slow）\n"
           "  options - オプション（省略可能）\n"
           "戻り値: 処理結果")
{
  /* 実装 */
  return input;
}
```

## 6.6 基本データ型

### 概要

Guile で提供される基本的なデータ型とその操作方法。

### ブール値

```c
/* C での操作 */
SCM scm_true = scm_from_bool(1);   /* #t */
SCM scm_false = scm_from_bool(0);  /* #f */

int is_true = scm_is_true(scm_value);
int is_false = scm_is_false(scm_value);

/* SCM_BOOL_T、SCM_BOOL_F マクロ */
SCM result = SCM_BOOL_T;
```

```scheme
; Scheme での操作
#t              ; 真
#f              ; 偽
(boolean? #t)   ; => #t
(not #f)        ; => #t
```

### 数値データ型

#### 整数

```c
/* C での操作 */
SCM scm_int = scm_from_int(42);
SCM scm_long = scm_from_long(1000000L);
int i = scm_to_int(scm_int);
```

```scheme
; Scheme での操作
42              ; 整数リテラル
(integer? 42)   ; => #t
(+ 1 2)         ; => 3
(* 5 6)         ; => 30
(quotient 17 5) ; => 3
(remainder 17 5); => 2
```

#### 浮動小数点数

```scheme
3.14                    ; 実数
(real? 3.14)            ; => #t
(+ 1.5 2.5)             ; => 4.0
(sqrt 16.0)             ; => 4.0
(sin 0.0)               ; => 0.0
```

#### 有理数

```scheme
(rational? 1/3)         ; => #t
(+ 1/2 1/3)             ; => 5/6
(denominator 5/6)       ; => 6
(numerator 5/6)         ; => 5
```

#### 複素数

```scheme
3+4i                    ; 複素数
(complex? 3+4i)         ; => #t
(real-part 3+4i)        ; => 3
(imag-part 3+4i)        ; => 4
```

#### 正確数と非正確数

```scheme
; 正確数（計算精度を保証）
(exact? 1/3)            ; => #t
(exact->inexact 1/3)    ; => 0.333333...

; 非正確数（浮動小数点数）
(inexact? 0.5)          ; => #t
(inexact->exact 0.5)    ; => 1/2
```

### 文字と文字列

#### 文字

```c
/* C での操作 */
SCM scm_char = scm_c_make_char('a');
char c = scm_to_char(scm_char);
```

```scheme
; Scheme での操作
#\a             ; 文字 'a'
#\space         ; スペース文字
#\newline       ; 改行
(char? #\a)     ; => #t
(char->integer #\A)  ; => 65
(char-upcase #\a)    ; => #\A
```

#### 文字列

```c
/* C での操作 */
SCM scm_str = scm_from_locale_string("hello");
char *str = scm_to_locale_string(scm_str);
/* scm_to_locale_string から取得したメモリは
   scm_gc_free で解放 */
size_t len = scm_c_string_length(scm_str);
```

```scheme
; Scheme での操作
"hello"                         ; 文字列リテラル
(string? "hello")               ; => #t
(string-append "hello" " " "world")  ; => "hello world"
(string-length "hello")         ; => 5
(string-ref "hello" 0)          ; => #\h
(substring "hello" 1 4)         ; => "ell"
(string-upcase "hello")         ; => "HELLO"
```

### シンボル

```c
/* C での操作 */
SCM scm_sym = scm_from_locale_symbol("my-symbol");
char *sym_name = scm_symbol_to_string(scm_sym);
```

```scheme
; Scheme での操作
'foo                    ; シンボル
(symbol? 'foo)          ; => #t
(symbol->string 'foo)   ; => "foo"
(string->symbol "bar")  ; => bar
(gensym "x")            ; => x1（ユニークなシンボル）
```

### キーワード

```scheme
#:key                   ; キーワード
(keyword? #:name)       ; => #t
(keyword->symbol #:foo) ; => foo
```

### ペアとリスト

```c
/* C での操作 */
SCM pair = scm_cons(scm_from_int(1), scm_from_int(2));
SCM car_val = scm_car(pair);
SCM cdr_val = scm_cdr(pair);

SCM list = scm_list_3(
  scm_from_int(1),
  scm_from_int(2),
  scm_from_int(3));
size_t len = scm_to_size_t(scm_length(list));
```

```scheme
; Scheme での操作
(cons 1 2)              ; => (1 . 2)
(list 1 2 3)            ; => (1 2 3)
(car '(1 2 3))          ; => 1
(cdr '(1 2 3))          ; => (2 3)
(cadr '(1 2 3))         ; => 2
(null? '())             ; => #t
(append '(1 2) '(3 4))  ; => (1 2 3 4)
(reverse '(1 2 3))      ; => (3 2 1)
(length '(1 2 3))       ; => 3
```

### ベクトル

```c
/* C での操作 */
SCM vec = scm_make_vector(scm_from_int(3), 
                          scm_from_int(0));
scm_c_vector_set_x(vec, 0, scm_from_int(10));
SCM val = scm_c_vector_ref(vec, 0);
size_t len = scm_c_vector_length(vec);
```

```scheme
; Scheme での操作
#(1 2 3)                      ; ベクトルリテラル
(vector? #(1 2 3))            ; => #t
(make-vector 5 0)             ; 5 要素、初期値 0
(vector-length #(1 2 3))      ; => 3
(vector-ref #(1 2 3) 0)       ; => 1
(vector-set! #(1 2 3) 0 10)   ; 0 番目を 10 に設定
```

### バイトベクトル

```scheme
#u8(1 2 3 4 5)               ; バイトベクトル
(bytevector? #u8(1 2 3))     ; => #t
(bytevector-length #u8(1 2)) ; => 2
(bytevector-u8-ref #u8(1 2) 0)  ; => 1
(bytevector-u8-set! bv 0 255)   ; バイト設定
```

### 配列

```scheme
; 多次元配列
(define arr (make-array 0 3 4))
(array-set! arr 42 0 0)
(array-ref arr 0 0)          ; => 42
```

## 6.7 手続き（プロシージャ）

### 概要

手続きは Scheme の中心的な概念で、複数の方法で定義・呼び出しが可能です。

### Lambda: 基本的な手続き作成

```c
/* C での手続き呼び出し */
SCM proc = scm_c_eval_string("(lambda (x y) (+ x y))");
SCM arg1 = scm_from_int(2);
SCM arg2 = scm_from_int(3);
SCM result = scm_call_2(proc, arg1, arg2);
```

```scheme
; Scheme での手続き定義
(lambda (x y) (+ x y))

; 複数の引数
((lambda (a b c) (+ a b c)) 1 2 3)  ; => 6

; ネストされた手続き
((lambda (x) (lambda (y) (+ x y))) 5 3)  ; => 8
```

### プリミティブ手続き

Guile が提供する組み込み手続き：

```scheme
+, -, *, /          ; 算術演算
car, cdr, cons      ; リスト操作
display, write      ; 出力
read                ; 入力
map, apply, fold    ; 高階関数
```

### 手続きの呼び出し

```c
/* 可変引数での呼び出し */
SCM proc = scm_c_eval_string("+");
SCM args = scm_list_3(
  scm_from_int(1),
  scm_from_int(2),
  scm_from_int(3));
SCM result = scm_apply(proc, args, SCM_EOL);
```

```scheme
; 手続きの呼び出し
(+ 1 2)                 ; => 3
(apply + '(1 2 3))      ; => 6
(map (lambda (x) (* x 2)) '(1 2 3))  ; => (2 4 6)
```

### オプション引数と キーワード引数

```scheme
; #:optional を使用
(define (greet name #:optional (greeting "Hello"))
  (string-append greeting " " name))

(greet "Alice")              ; => "Hello Alice"
(greet "Bob" #:greeting "Hi"); => "Hi Bob"

; #:key を使用
(define (make-server #:key (host "localhost") (port 8080))
  (list host port))

(make-server)
(make-server #:host "example.com")
(make-server #:port 9000)
```

### Case-lambda

異なる引数数に対応する手続き：

```scheme
(define length-flexible
  (case-lambda
    ((x) (length x))
    ((x y) (+ (length x) (length y)))
    ((x y z) (+ (length x) (length y) (length z)))))

(length-flexible '(1 2 3))          ; => 3
(length-flexible '(a) '(b c))       ; => 3
```

### 高階関数

```scheme
; 関数を返す高階関数
(define (make-adder n)
  (lambda (x) (+ x n)))

(define add5 (make-adder 5))
(add5 10)                   ; => 15

; 関数を引数に取る高階関数
(define (apply-twice f x)
  (f (f x)))

(apply-twice (lambda (x) (* 2 x)) 3)  ; => 12
```

## 6.8 マクロ

### 概要

マクロはコード変換のための強力な機能です。

### マクロの定義

```c
/* C から定義した Scheme マクロ */
scm_c_eval_string(
  "(define-syntax when\n"
  "  (syntax-rules ()\n"
  "    ((when test body ...)\n"
  "     (if test (begin body ...)))))\n");
```

### Syntax-rules マクロ

パターンマッチング方式のマクロ：

```scheme
; if-not マクロ（if の否定版）
(define-syntax if-not
  (syntax-rules ()
    ((if-not test then-expr else-expr)
     (if (not test) then-expr else-expr))))

(if-not #f "yes" "no")  ; => "yes"

; unless マクロ
(define-syntax unless
  (syntax-rules ()
    ((unless test body ...)
     (if (not test) (begin body ...)))))

(unless (> 1 2)
  (display "1 is not greater than 2")
  (newline))
```

### Syntax-case マクロ

より複雑なマクロの定義：

```scheme
(define-syntax my-cond
  (lambda (x)
    (syntax-case x ()
      ((my-cond (test . body) ...)
       #'(cond (test . body) ...)))))
```

## 6.9 変数とスコープ

### 概要

変数定義とスコープ管理。

### トップレベル変数定義

```c
/* C から定義 */
scm_c_eval_string("(define x 42)");
SCM x = scm_c_eval_string("x");
```

```scheme
; Scheme で定義
(define x 10)
(define y 20)
(define (square x) (* x x))
```

### 局所変数バインディング

```scheme
; let：並列バインディング
(let ((x 1) (y 2))
  (+ x y))                   ; => 3

; let*：順序付きバインディング
(let* ((x 5)
       (y (* x 2)))
  (+ x y))                   ; => 15

; letrec：相互再帰用
(letrec ((is-even? (lambda (n)
                     (if (= n 0) #t
                         (is-odd? (- n 1)))))
         (is-odd? (lambda (n)
                    (if (= n 0) #f
                        (is-even? (- n 1))))))
  (is-even? 4))              ; => #t
```

### グローバル変数の変更

```scheme
; set! で変更
(define counter 0)
(set! counter 1)

; ハッシュテーブルでのキー値変更
(hash-set! table 'name "new-value")
```

## 6.10 制御フロー

### 概要

プログラム実行の流れを制御する構文。

### シーケンシング

```scheme
; begin で複数の式を順番に実行
(begin
  (display "First")
  (newline)
  (display "Second")
  (newline))
```

### 条件分岐

```scheme
; if 式
(if (> x 0) "positive" "non-positive")

; cond：複数条件
(cond
  ((< x 0) "negative")
  ((= x 0) "zero")
  (else "positive"))

; case：値による分岐
(case (car lst)
  ((+ -) "arithmetic")
  ((* /) "multiplication/division")
  (else "unknown"))
```

### 反復とループ

```scheme
; do ループ
(do ((i 0 (+ i 1)))
    ((>= i 10) "done")
  (display i)
  (newline))

; for-each：副作用のための反復
(for-each (lambda (x) (display x))
          '(1 2 3 4 5))
```

### 例外処理

```scheme
; catch で例外をキャッチ
(catch 'my-error
  (lambda ()
    (throw 'my-error "Something went wrong"))
  (lambda (key msg)
    (display "Caught: ")
    (display msg)))

; with-exception-handler
(with-exception-handler
  (lambda (exn)
    (display "Error: ")
    (display (exception:message exn)))
  (lambda ()
    (error "An error occurred")))
```

## 6.11 入出力

### 概要

ファイルとストリームの操作。

### ポート

```scheme
; 標準ポート
(current-input-port)       ; 標準入力
(current-output-port)      ; 標準出力
(current-error-port)       ; エラー出力

; ファイルポートの開閉
(with-input-from-file "input.txt"
  (lambda ()
    (read-line)))

(with-output-to-file "output.txt"
  (lambda ()
    (display "Hello, file!")))
```

### テキスト入出力

```scheme
; 出力
(display "Hello")          ; 出力
(write '(1 2 3))           ; S式を出力
(format #t "~a ~d~n" "Number:" 42)

; 入力
(read)                     ; S式を読み込み
(read-line)                ; 1 行を文字列で読み込み
(get-char)                 ; 1 文字を読み込み
```

### バイナリ入出力

```scheme
; バイナリモードでのファイル操作
(call-with-input-file "data.bin"
  (lambda (port)
    (get-bytevector-all port)))
```

## 6.12 高度なトピック

### 正規表現

```scheme
(use-modules (ice-9 regex))

(string-match "^[0-9]+$" "12345")    ; マッチ
(string-match "^[a-z]+$" "hello")    ; マッチ

; マッチ結果から部分文字列を抽出
(let ((match (string-match "(\\w+)@(\\w+)" "user@host")))
  (match:substring match 1))  ; => "user"
```

### Scheme コードの読み込みと評価

```c
/* C からのコード実行 */
SCM result = scm_c_eval_string("(+ 2 3)");
int val = scm_to_int(result);
```

```scheme
; Scheme 内でのコード実行
(eval (read port) (interaction-environment))

(eval '(+ 1 2) (null-environment 5))  ; => 3
```

### メモリ管理とガベージコレクション

```c
/* GC からの保護 */
static SCM my_important_object;

void
init_module(void)
{
  my_important_object = scm_from_int(42);
  scm_gc_protect_object(my_important_object);
}

/* 後で保護を解除 */
scm_gc_unprotect_object(my_important_object);
```

## 6.13 モジュールシステム

### モジュール使用

```c
/* C からモジュールを使用 */
scm_c_eval_string("(use-modules (srfi srfi-1))");
SCM result = scm_c_eval_string("(map (lambda (x) (* x 2)) '(1 2 3))");
```

```scheme
; Scheme でモジュール使用
(use-modules (srfi srfi-1))          ; SRFI-1 をロード
(use-modules (ice-9 regex))          ; 正規表現ライブラリ
(use-modules (my-module utils))      ; カスタムモジュール
```

### モジュール作成と管理

```scheme
; モジュールの定義
(define-module (my-project utils)
  #:use-module (srfi srfi-1)
  #:export (double triple process-list))

(define (double x) (* x 2))
(define (triple x) (* x 3))
(define (internal-func x) x)  ; エクスポートされない

(define (process-list lst)
  (map double lst))
```

### モジュール内での再エクスポート

```scheme
(define-module (my-project extended)
  #:use-module (my-project utils)
  #:export-syntax (my-macro)
  #:re-export (double triple))

(define-syntax my-macro
  (syntax-rules ()
    ((my-macro x) (double x))))
```

## 6.14 外部関数インターフェース（FFI）

### C ライブラリの直接呼び出し

```scheme
(use-modules (system foreign))

; C の strlen 関数をラップ
(define libc (dynamic-link "libc.so.6"))
(define strlen
  (pointer->procedure size_t
    (dynamic-func "strlen" libc)
    (list '*)))

(strlen (string->pointer "hello"))   ; => 5
```

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
