# 6. API リファレンス

> **原文**: [Guile Reference Manual - API Reference](https://www.gnu.org/software/guile/manual/guile.html#API-Reference)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、Guile が提供する主要な API（Application Programming Interface）について説明します。詳細は公式マニュアルを参照してください。

## 6.1 Guile API の概要

Guile API は、以下の目的で設計されています：

- Scheme コード内での値の操作
- C コードから Scheme 関数の呼び出し
- Scheme から C 関数へのアクセス
- メモリ管理とガベージコレクション

## 6.2 非推奨機能

Guile は古い API の後方互換性のため、一部の関数を非推奨としています。新しいコードでは非推奨関数の使用を避けるべきです。

## 6.3 SCM 型

すべての Scheme 値は `SCM` 型で表現されます。

```c
SCM x = scm_from_int(42);      /* 整数を作成 */
int i = scm_to_int(x);          /* SCM から整数に変換 */
```

## 6.4 Guile の初期化

### 初期化関数

```c
scm_boot_guile(argc, argv, main_func, data)
scm_init_guile()
```

## 6.5 スナーフィングマクロ

C コードから Scheme 手続きを自動生成するマクロ。

## 6.6 データ型

### ブール値

```scheme
#t      ; 真
#f      ; 偽
```

### 数値データ型

- **整数**: 任意精度整数
- **実数**: 浮動小数点数
- **有理数**: 分数表現
- **複素数**: 複素数表現
- **正確数と非正確数**: 計算精度の区別

### 数値操作

```scheme
(+ 1 2)         ; 加算 => 3
(- 5 3)         ; 減算 => 2
(* 4 5)         ; 乗算 => 20
(/ 20 4)        ; 除算 => 5
(modulo 17 5)   ; 余り => 2
```

### 文字

```scheme
#\a             ; 文字 'a'
#\space         ; スペース
#\newline       ; 改行
```

### 文字集合

文字の集合を効率的に操作するためのデータ構造。

### 文字列

```scheme
"hello"                         ; 文字列
(string-append "hello" " " "world")  ; 連結
(string-length "hello")         ; 長さ => 5
(string-ref "hello" 0)          ; 最初の文字 => #\h
```

### シンボル

```scheme
'foo            ; シンボル
(symbol? 'foo)  ; #t
(symbol->string 'foo)  ; "foo"
```

### キーワード

```scheme
#:key           ; キーワード
```

### ペアとリスト

```scheme
(cons 1 2)              ; => (1 . 2)
(list 1 2 3)            ; => (1 2 3)
(car '(1 2 3))          ; => 1
(cdr '(1 2 3))          ; => (2 3)
(append '(1 2) '(3 4))  ; => (1 2 3 4)
```

### ベクトル

```scheme
#(1 2 3)                ; ベクトル
(vector-length #(1 2 3))  ; => 3
(vector-ref #(1 2 3) 0)   ; => 1
```

### ビットベクトル

論理値の集合を効率的に保存。

### バイトベクトル

バイト列の処理用。

### 配列

多次元配列構造。

## 6.7 手続き（プロシージャ）

### Lambda: 基本的な手続き作成

```scheme
(lambda (x y) (+ x y))
```

### プリミティブ手続き

Guile が提供する組み込み手続き。

### コンパイルされた手続き

パフォーマンス最適化のためコンパイルされた手続き。

### オプション引数

```scheme
(define (f a #:optional b) a)
```

### Case-lambda

異なる引数数に対応する手続き。

### 高階関数

手続きを引数や戻り値とする関数。

## 6.8 マクロ

### マクロの定義

```scheme
(define-syntax when
  (syntax-rules ()
    ((when test body ...)
     (if test (begin body ...)))))
```

### Syntax-rules マクロ

パターンマッチング方式のマクロ。

## 6.9 一般ユーティリティ関数

## 6.10 定義と変数バインディング

### トップレベル変数定義

```scheme
(define x 10)
```

### 局所変数バインディング

```scheme
(let ((x 1) (y 2)) (+ x y))
```

## 6.11 プログラム実行フローの制御

### シーケンシング

```scheme
(begin expr1 expr2 expr3)
```

### 条件分岐

```scheme
(if condition true-expr false-expr)
(cond (test1 expr1) (test2 expr2))
```

### 反復メカニズム

```scheme
(do ((i 0 (+ i 1))) ((>= i 10)) (display i))
```

### 例外処理

```scheme
(catch #t (lambda () ...) (lambda (key . args) ...))
```

## 6.12 入出力

### ポート

ファイルやストリームとの通信チャネル。

### テキスト入出力

```scheme
(display "hello")
(newline)
(read-line)
```

### バイナリ入出力

バイナリデータの読み書き。

## 6.13 正規表現

パターンマッチングとテキスト処理。

## 6.14 LALR(1) パーシング

構文解析ライブラリ。

## 6.15 PEG パーシング

パーサー生成文法ライブラリ。

## 6.16 Scheme コードの読み込みと評価

### Scheme コードの読み込み

```scheme
(read-string port)
(eval (read port) environment)
```

### Scheme コードのコンパイル

```scheme
(compile expr)
```

## 6.17 メモリ管理とガベージコレクション

Guile の自動メモリ管理について。

## 6.18 モジュール

### モジュール使用

```scheme
(use-modules (module-name))
```

### モジュール作成

```scheme
(define-module (my-module)
  #:export (exported-function))
```

## 6.19 外部関数インターフェース

C ライブラリとのバインディング。

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
