# 6. API リファレンス

> **原文**: [Guile Reference Manual - API Reference](https://www.gnu.org/software/guile/manual/guile.html#API-Reference)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、Guile が提供する包括的な API（Application Programming Interface）について詳細に説明します。C プログラムから Guile を使用する際の関数・型定義が含まれます。

## 6.1 Guile API の概要

### 概要

Guile API は、C プログラムが Scheme インタプリタと相互作用するための統一的なインターフェースです。Scheme 値の操作、関数呼び出し、データ型変換、メモリ管理などをサポートします。

### API の構成

Guile API は以下のレイヤーで構成されています：

- **低レベル API**: SCM 型と基本的な型変換
- **プロシージャ呼び出し**: Scheme 関数の呼び出し
- **型システム**: データ型の表現と変換
- **メモリ管理**: ガベージコレクション
- **モジュールシステム**: コードの整理と再利用

### 主要な機能

```c
/* ヘッダファイルのインクルード */
#include <libguile.h>

/* 初期化 */
scm_boot_guile(argc, argv, main_func, NULL);

/* 値の作成と変換 */
SCM scm_val = scm_from_int(42);
int c_val = scm_to_int(scm_val);

/* 関数呼び出し */
SCM result = scm_call_1(proc, arg);

/* GC 保護 */
scm_gc_protect_object(important_obj);
```

## 6.2 非推奨機能

### 概要

Guile は古い API との後方互換性を提供しながら、新しい API への移行を推奨しています。

### 非推奨マクロと置き換え関数

非推奨のマクロは SCM 値の内部表現に直接アクセスしていました：

| 非推奨 | 新API | 説明 |
|--------|-------|------|
| `SCM_INUMP(x)` | `scm_is_integer(x)` | 整数型チェック |
| `SCM_INUM(x)` | `scm_to_int(x)` | 整数値取得 |
| `SCM_MAKINUM(i)` | `scm_from_int(i)` | 整数値作成 |
| `SCM_STRINGP(x)` | `scm_is_string(x)` | 文字列型チェック |
| `SCM_STRING_CHARS(x)` | `scm_to_locale_string(x)` | 文字列取得 |

### 新 API への移行例

```c
/* 非推奨: */
if (SCM_INUMP(x)) {
  int val = SCM_INUM(x);
}

/* 新API: */
if (scm_is_integer(x)) {
  int val = scm_to_int(x);
}
```

## 6.3 SCM 型

### 概要

SCM は Guile における全 Scheme 値の表現型です。SCM は不透明なポインタ型で、内部的に型情報とデータをエンコードしています。

### SCM 型の特性

```c
/* SCM は単純なデータ型 */
typedef unsigned long SCM;

/* あらゆる Scheme 値は SCM で表現される */
SCM boolean = scm_from_bool(1);      /* #t */
SCM integer = scm_from_int(42);      /* 42 */
SCM string = scm_from_locale_string("hello");  /* "hello" */
SCM list = scm_list_2(integer, string);  /* (42 "hello") */

/* 型情報の埋め込み */
if (scm_is_integer(value)) {
  /* value は整数型 */
}
```

### 特殊な SCM 値

```c
/* 未定義値 */
SCM undefined = SCM_UNBOUND;

/* false 値（違いに注意） */
SCM false = SCM_BOOL_F;
SCM eol = SCM_EOL;

/* その他の重要な値 */
SCM true = SCM_BOOL_T;
```

## 6.4 Guile の初期化

### 概要

Guile を C プログラムに組み込む場合、初期化手順が重要です。

### 初期化関数

#### scm_boot_guile

メイン関数から Guile 環境を初期化する標準的な方法：

```c
static void *
inner_main(void *data)
{
  /* Guile が初期化された状態でここが実行される */
  SCM result = scm_c_eval_string("(+ 2 3)");
  printf("Result: %d\n", scm_to_int(result));
  
  return NULL;  /* inner_main の戻り値 */
}

int
main(int argc, char *argv[])
{
  /* Guile 環境を初期化して inner_main を実行 */
  scm_boot_guile(argc, argv, inner_main, NULL);
  return 0;
}
```

#### scm_init_guile

スレッド化環境またはライブラリコンテキストで初期化：

```c
/* ライブラリコンテキストでの初期化 */
scm_init_guile();

/* 初期化後、Guile を使用可能 */
SCM value = scm_c_eval_string("(define x 42)");
```

### 初期化時の引数処理

```c
/* プログラム引数を Guile に渡す */
int
main(int argc, char *argv[])
{
  scm_boot_guile(argc, argv, inner_main, NULL);
  return 0;
}

/* inner_main 内から引数にアクセス */
static void *
inner_main(void *data)
{
  SCM args = scm_program_arguments();
  /* args は (program-name arg1 arg2 ...) のリスト */
  return NULL;
}
```

## 6.5 スナーフィングマクロ

### 概要

スナーフィングは、C から定義された関数を自動的に Scheme の手続きとして登録するツール。

### SCM_DEFINE マクロ

```c
#include <libguile.h>

/* C 関数をスナーフィングで定義 */
SCM_DEFINE(my_double, "my-double", 1, 0, 0,
           (SCM x),
           "引数を 2 倍にする関数\n\n"
           "Args:\n"
           "  x - 整数\n"
           "Returns: x * 2")
{
  return scm_from_int(2 * scm_to_int(x));
}

/* 初期化関数 */
void
init_my_module(void)
{
  #include "my_module.x"  /* スナーフィング生成ファイル */
}
```

### スナーフィング処理

スナーフィングプロセスは以下を行います：

1. C ソース内の `SCM_DEFINE` を検索
2. Scheme の手続き定義を生成
3. メタデータ（ドキュメント、アリティ）を含める
4. `.x` ファイルに出力

使用方法：

```bash
guile-tools snarf my_module.c > my_module.x
gcc -c -I. my_module.c
```

## 6.6 データ型

### 6.6.1 ブール値

```c
/* C での操作 */
SCM true = scm_from_bool(1);
SCM false = scm_from_bool(0);

int is_true = scm_is_true(value);     /* #t に類する */
int is_false = scm_is_false(value);   /* #f のみ */

/* マクロ定義 */
#define SCM_BOOL_T  /* #t */
#define SCM_BOOL_F  /* #f */
```

```scheme
; Scheme での操作
#t                  ; 真
#f                  ; 偽
(boolean? #t)       ; => #t
(not #f)            ; => #t
(if #f 1 2)         ; => 2（#f は偽として扱われる）
```

### 6.6.2 数値データ型

#### 数値タワー

Scheme は多層的な数値型をサポート：

```
複素数
├ 実数
  ├ 有理数
    └ 整数
```

#### 整数操作

```c
/* C での整数操作 */
SCM scm_int = scm_from_int(42);
SCM scm_long = scm_from_long(1000000L);
SCM scm_uint = scm_from_uint(42U);

int i = scm_to_int(scm_int);
long l = scm_to_long(scm_long);

/* 任意精度整数 */
SCM big_int = scm_c_eval_string("999999999999999999");
```

```scheme
; Scheme での整数
42                  ; 整数リテラル
#b101010            ; 2進表記（42）
#o52                ; 8進表記（42）
#x2a                ; 16進表記（42）

(integer? 42)       ; => #t
(odd? 5)            ; => #t
(even? 4)           ; => #t
(prime? 17)         ; => #t
```

#### 実数と有理数

```scheme
; 実数（浮動小数点数）
3.14                ; 実数リテラル
1.5e2               ; 指数表記（150.0）
(real? 3.14)        ; => #t
(inexact? 3.14)     ; => #t（浮動小数点数は非正確）

; 有理数（正確な分数）
1/2                 ; 有理数
(rational? 1/3)     ; => #t
(exact? 1/3)        ; => #t
(+ 1/2 1/3)         ; => 5/6
(denominator 5/6)   ; => 6
```

#### 複素数

```scheme
3+4i                ; 複素数リテラル
(complex? 3+4i)     ; => #t
(real-part 3+4i)    ; => 3
(imag-part 3+4i)    ; => 4
(magnitude 3+4i)    ; => 5.0
(angle 3+4i)        ; => 角度（ラジアン）
```

#### 数値演算

```scheme
; 基本演算
(+ 1 2 3)           ; => 6
(- 10 3)            ; => 7
(* 2 3 4)           ; => 24
(/ 10 3)            ; => 10/3（正確）
(quotient 17 5)     ; => 3
(remainder 17 5)    ; => 2
(modulo 17 5)       ; => 2

; 数学関数
(sqrt 16)           ; => 4
(expt 2 3)          ; => 8
(sin 0)             ; => 0.0
(cos 0)             ; => 1.0
(log 1)             ; => 0.0
(exp 1)             ; => e
(ceiling 3.2)       ; => 4
(floor 3.9)         ; => 3
(round 3.5)         ; => 4
(truncate 3.9)      ; => 3
```

### 6.6.3 文字

```c
/* C での文字操作 */
SCM scm_char = scm_c_make_char('a');
char c = scm_to_char(scm_char);

/* 大文字・小文字変換 */
SCM upper = scm_char_upcase(scm_char);
SCM lower = scm_char_downcase(scm_char);
```

```scheme
; Scheme での文字
#\a                 ; 文字 'a'
#\A                 ; 大文字 'A'
#\space             ; スペース
#\newline           ; 改行
#\null              ; ヌル文字
#\tab               ; タブ
#\alarm             ; ベル
#\backspace         ; バックスペース
#\delete            ; DEL 文字

(char? #\a)         ; => #t
(char=? #\a #\a)    ; => #t
(char<? #\a #\b)    ; => #t
(char-upcase #\a)   ; => #\A
(char-downcase #\A) ; => #\a
(char->integer #\A) ; => 65
(integer->char 65)  ; => #\A
```

### 6.6.4 文字集合

文字集合は複数の文字をコンパクトに管理するデータ構造：

```scheme
(use-modules (srfi srfi-14))

; 文字集合の作成
(char-set #\a #\b #\c)         ; 具体的な文字
(char-set-union cs1 cs2)        ; 共和
(char-set-intersection cs1 cs2) ; 交差
(char-set-complement cs)        ; 補集合

; 標準的な文字集合
char-set:lower-case             ; a-z
char-set:upper-case             ; A-Z
char-set:digit                  ; 0-9
char-set:whitespace             ; 空白文字
char-set:punctuation            ; 句読点
```

### 6.6.5 文字列

#### 文字列の作成と操作

```c
/* C での文字列操作 */
SCM scm_str = scm_from_locale_string("hello");
char *c_str = scm_to_locale_string(scm_str);
free(c_str);  /* 必ず解放 */

size_t len = scm_c_string_length(scm_str);
```

```scheme
; Scheme での文字列操作
"hello"                         ; 文字列リテラル
(string? "hello")               ; => #t
(string-length "hello")         ; => 5
(string-ref "hello" 0)          ; => #\h
(string-set! str 0 #\H)         ; 文字を変更
(substring "hello" 1 4)         ; => "ell"
(string-append "hello" " " "world")  ; => "hello world"

; 文字列変換
(string-upcase "hello")         ; => "HELLO"
(string-downcase "HELLO")       ; => "hello"
(string-capitalize "hello world")  ; => "Hello world"

; 文字列検索
(string-contains "hello" "ll")  ; => 2（位置）
(string-index "hello" #\l)      ; => 2
(string-rindex "hello" #\l)     ; => 3

; 文字列分割と結合
(string-split "a,b,c" #\,)      ; => ("a" "b" "c")
(string-join '("a" "b" "c") ",")  ; => "a,b,c"
```

### 6.6.6 シンボル

```c
/* C でのシンボル操作 */
SCM sym = scm_from_locale_symbol("my-symbol");
char *name = scm_symbol_to_string(sym);
```

```scheme
; Scheme でのシンボル
'foo                ; シンボル foo
(symbol? 'foo)      ; => #t
(symbol->string 'foo)  ; => "foo"
(string->symbol "bar")  ; => bar

; ユニークなシンボル
(gensym)            ; => g1（毎回異なる）
(gensym "x")        ; => x2
```

### 6.6.7 ペアとリスト

```c
/* C でのペア操作 */
SCM pair = scm_cons(scm_from_int(1), scm_from_int(2));
SCM car_val = scm_car(pair);      /* 1 */
SCM cdr_val = scm_cdr(pair);      /* 2 */

/* リスト操作 */
SCM list = scm_list_3(
  scm_from_int(1),
  scm_from_int(2),
  scm_from_int(3));
```

```scheme
; Scheme でのペアとリスト
(cons 1 2)              ; => (1 . 2)   ペア
(list 1 2 3)            ; => (1 2 3)   リスト
(car '(1 2 3))          ; => 1
(cdr '(1 2 3))          ; => (2 3)
(cadr '(1 2 3))         ; => 2
(caddr '(1 2 3))        ; => 3

; リスト操作
(null? '())             ; => #t
(length '(1 2 3))       ; => 3
(append '(1 2) '(3 4))  ; => (1 2 3 4)
(reverse '(1 2 3))      ; => (3 2 1)
(member 2 '(1 2 3))     ; => (2 3)
(nth 1 '(a b c))        ; => b
(take '(1 2 3 4) 2)     ; => (1 2)
(drop '(1 2 3 4) 2)     ; => (3 4)
```

### 6.6.8 ベクトル

```c
/* C でのベクトル操作 */
SCM vec = scm_make_vector(scm_from_int(3), SCM_UNSPECIFIED);
scm_c_vector_set_x(vec, 0, scm_from_int(10));
SCM val = scm_c_vector_ref(vec, 0);
size_t len = scm_c_vector_length(vec);
```

```scheme
; Scheme でのベクトル
#(1 2 3)                      ; ベクトルリテラル
(vector? #(1 2 3))            ; => #t
(make-vector 5 0)             ; 5 要素のベクトル
(vector-length #(1 2 3))      ; => 3
(vector-ref #(1 2 3) 0)       ; => 1
(vector-set! #(1 2 3) 0 10)   ; 0 番目を 10 に設定
(vector->list #(1 2 3))       ; => (1 2 3)
(list->vector '(1 2 3))       ; => #(1 2 3)
```

### 6.6.9 バイトベクトル

```scheme
; バイトベクトル（SRFI-4）
#u8(1 2 3 4 5)                  ; u8vector（0-255）
#s8(1 -2 3 -4)                  ; s8vector（-128-127）
#u16(256 512)                   ; u16vector
#f64(1.5 2.5 3.5)               ; f64vector（浮動小数点数）

(bytevector? #u8(1 2 3))        ; => #t
(bytevector-length #u8(1 2))    ; => 2
(bytevector-u8-ref #u8(1 2) 0)  ; => 1
(bytevector-u8-set! bv 0 255)   ; 値を設定
```

## 6.7 手続き（プロシージャ）

### 概要

手続きは Scheme の最も基本的な概念で、計算を実行するユニット。

### C から手続きの呼び出し

```c
/* 引数なし */
SCM result = scm_call_0(proc);

/* 1 引数 */
SCM result = scm_call_1(proc, arg1);

/* 2 引数 */
SCM result = scm_call_2(proc, arg1, arg2);

/* 3 引数 */
SCM result = scm_call_3(proc, arg1, arg2, arg3);

/* 可変引数 */
SCM args = scm_list_2(arg1, arg2);
SCM result = scm_apply_0(proc, args);
```

```scheme
; Scheme での手続き定義
(lambda (x y) (+ x y))          ; 無名手続き
((lambda (x) (* x 2)) 5)        ; => 10

; 名前付き手続き
(define (square x) (* x x))
(square 5)                       ; => 25

; 高階手続き
(define (apply-twice f x)
  (f (f x)))
(apply-twice (lambda (x) (* 2 x)) 3)  ; => 12

; map と apply
(map (lambda (x) (* x 2)) '(1 2 3))  ; => (2 4 6)
(apply + '(1 2 3))              ; => 6
```

### 高階関数とクロージャ

```scheme
; クロージャ：環境をキャプチャ
(define (make-counter)
  (let ((count 0))
    (lambda ()
      (set! count (+ count 1))
      count)))

(define counter (make-counter))
(counter)                       ; => 1
(counter)                       ; => 2

; 関数の部分適用
(define add (lambda (x y) (+ x y)))
(define add5 (lambda (x) (add x 5)))
(add5 10)                       ; => 15
```

## 6.8 マクロ

### 概要

マクロはコード変換のための強力な機能。Scheme は多様なマクロシステムをサポート。

### Syntax-rules マクロ

パターンマッチング方式の基本的なマクロ：

```scheme
(define-syntax when
  (syntax-rules ()
    ((when test body ...)
     (if test (begin body ...)))))

(when (> x 5)
  (display "x is large")
  (newline))
```

### Syntax-case マクロ

より詳細な制御が必要な場合：

```scheme
(define-syntax my-let
  (lambda (x)
    (syntax-case x ()
      ((my-let ((var expr) ...) body ...)
       #'((lambda (var ...) body ...) expr ...)))))

(my-let ((x 1) (y 2))
  (+ x y))                      ; => 3
```

## 6.9 変数バインディング

### トップレベル定義

```c
/* C から定義 */
scm_c_eval_string("(define x 42)");
SCM x = scm_c_eval_string("x");
```

```scheme
; トップレベル変数
(define x 10)
(define y 20)
(set! x 50)                     ; 変更
```

### 局所変数バインディング

```scheme
; let：並列バインディング
(let ((x 1) (y 2))
  (+ x y))                      ; => 3

; let*：順序依存バインディング
(let* ((x 5)
       (y (* x 2)))
  (+ x y))                      ; => 15

; letrec：相互再帰
(letrec ((even? (lambda (n)
                  (if (= n 0) #t (odd? (- n 1)))))
         (odd? (lambda (n)
                 (if (= n 0) #f (even? (- n 1))))))
  (even? 4))                    ; => #t
```

## 6.10 制御フロー

### 条件分岐

```scheme
; if 式
(if (> x 0) "positive" "non-positive")

; cond 式
(cond ((< x 0) "negative")
      ((= x 0) "zero")
      (else "positive"))

; case 式
(case (car lst)
  ((+ -) "arithmetic")
  ((* /) "multiplication")
  (else "unknown"))
```

### ループと反復

```scheme
; do ループ
(do ((i 0 (+ i 1)))
    ((>= i 10) "done")
  (display i)
  (newline))

; for-each
(for-each (lambda (x) (display x) (newline))
          '(1 2 3 4 5))

; map
(map (lambda (x) (* x 2)) '(1 2 3))  ; => (2 4 6)
```

### 例外処理

```scheme
; catch で例外をキャッチ
(catch 'my-error
  (lambda ()
    (throw 'my-error "error occurred"))
  (lambda (key msg)
    (display "Caught: ")
    (display msg)))

; with-exception-handler
(with-exception-handler
  (lambda (ex)
    (display "Error occurred"))
  (lambda ()
    (error "something went wrong")))
```

## 6.11 入出力（I/O）

### ポート操作

```c
/* C でのポート操作 */
SCM input = scm_open_file(scm_from_locale_string("input.txt"),
                          scm_from_locale_string("r"));
SCM output = scm_open_file(scm_from_locale_string("output.txt"),
                           scm_from_locale_string("w"));
scm_close_port(input);
scm_close_port(output);
```

```scheme
; Scheme でのポート操作
(current-input-port)            ; 標準入力
(current-output-port)           ; 標準出力
(current-error-port)            ; エラー出力

; ファイル操作
(open-input-file "input.txt")
(open-output-file "output.txt")
(with-input-from-file "data.txt"
  (lambda ()
    (read-line)))
(with-output-to-file "output.txt"
  (lambda ()
    (display "Hello, file!")))
```

### テキスト入出力

```scheme
; 出力
(display "Hello")               ; 人間向け
(write '(1 2 3))                ; マシン向け
(format #t "~a = ~d~n" "x" 42)  ; フォーマット出力
(newline)                       ; 改行

; 入力
(read)                          ; S式を読み込み
(read-line)                     ; 行を読み込み
(get-char)                      ; 1 文字読み込み
(peek-char)                     ; 1 文字先読み
```

### format 関数

```scheme
; format の例
(format #t "Number: ~d~n" 42)           ; => Number: 42
(format #t "String: ~s~n" "hello")      ; => String: "hello"
(format #t "Hex: ~x~n" 255)             ; => Hex: ff
(format #t "Padded: ~5d~n" 42)          ; => Padded:    42
(format #f "~a + ~a = ~a" 2 3 5)        ; => "2 + 3 = 5"
```

## 6.12 正規表現

```scheme
(use-modules (ice-9 regex))

; 基本的なマッチング
(string-match "^[0-9]+$" "12345")       ; マッチ
(string-match "^[0-9]+$" "abc")         ; #f（不一致）

; マッチ結果から抽出
(let ((m (string-match "(\\w+)@(\\w+)" "user@host")))
  (match:substring m 1))                ; => "user"

; 置換
(regexp-substitute #f
  (string-match "(.+)@(.+)" "user@host")
  'pre 2 "@" 1 'post)                   ; => "host@user"
```

## 6.13 Scheme コードの評価

### 動的評価

```c
/* C からコードを評価 */
SCM result = scm_c_eval_string("(+ 2 3)");
int val = scm_to_int(result);
```

```scheme
; eval で S式を評価
(eval '(+ 1 2) (interaction-environment))  ; => 3

; eval-string でコード文字列を評価
(use-modules (ice-9 eval-string))
(eval-string "(+ 1 2)")                 ; => 3

; compile でコンパイル
(compile '(+ 1 2))                      ; コンパイル結果
```

## 6.14 メモリ管理とガベージコレクション

### GC 保護

```c
/* グローバル変数を GC から保護 */
static SCM important_object;

void
init_module(void)
{
  important_object = scm_from_int(42);
  scm_gc_protect_object(important_object);
}

/* 後で保護を解除 */
scm_gc_unprotect_object(important_object);
```

### 弱参照

```scheme
; 弱参照：GC の対象になる可能性がある参照
(make-weak-vector size init)
(weak-vector-ref wvec i default)
```

## 6.15 モジュールシステム

### モジュール使用

```c
/* C からモジュール操作 */
scm_c_eval_string("(use-modules (srfi srfi-1))");
```

```scheme
; モジュール使用
(use-modules (srfi srfi-1))             ; SRFI-1
(use-modules (ice-9 regex))             ; 正規表現
(use-modules (my-project utils))        ; カスタムモジュール
```

### モジュール作成

```scheme
(define-module (my-project utils)
  #:use-module (srfi srfi-1)
  #:export (double triple process))

(define (double x) (* x 2))
(define (triple x) (* x 3))
(define (process lst) (map double lst))
```

## 6.16 読み込みと評価

### コード読み込み

```scheme
; Scheme ファイルをロード
(load "my-file.scm")

; モジュールのロード
(load-extension "libmy" "init_my")
(use-modules (my-lib utils))
```

## 6.17 外部関数インターフェース（FFI）

```scheme
(use-modules (system foreign))

; C ライブラリの関数をラップ
(define libc (dynamic-link "libc.so.6"))
(define strlen-proc
  (pointer->procedure size_t
    (dynamic-func "strlen" libc)
    (list '*)))

(strlen-proc (string->pointer "hello"))  ; => 5
```

## 6.18 スレッドと並行処理

```scheme
(use-modules (ice-9 threads))

; スレッド作成
(define t (make-thread (lambda () (display "Thread\n"))))
(thread-join! t)

; ミューテックス
(define mutex (make-mutex))
(with-mutex mutex (display "Protected"))

; 条件変数
(define cond-var (make-condition-variable))
(condition-variable-wait! cond-var mutex)
```

## 6.19 デバッグ

### トレース

```scheme
; 関数をトレース
(trace square)
(square 5)                              ; 呼び出しと戻り値を表示
(untrace square)

; ブレークポイント
(break)                                 ; デバッガーに入る
```

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
