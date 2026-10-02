# 2. Guile を始める

> **原文**: [Guile Reference Manual - Hello Guile!](https://www.gnu.org/software/guile/manual/guile.html#Hello-Guile)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、Guile を実際に使い始めるための基本的な方法を紹介します。

## 2.1 Guile を対話的に実行する

Guile はインタラクティブなコマンドラインツール（REPL: Read-Eval-Print Loop）を提供しています。

### 起動方法

```bash
guile
```

### 基本的な使用例

REPL で実行：

```scheme
scheme@(guile-user)> (+ 2 3)
5

scheme@(guile-user)> (define x 10)

scheme@(guile-user)> (* x 2)
20

scheme@(guile-user)> (exit)
```

## 2.2 Guile スクリプトの実行

Scheme コードをスクリプトファイルとして実行することもできます。

### スクリプトファイルの作成

`hello.scm` という名前で以下のファイルを作成：

```scheme
#!/usr/bin/guile -s
!#

(display "Hello, Guile!\n")
```

### スクリプトの実行

```bash
guile hello.scm
```

または、ファイルに実行権限を付与して直接実行：

```bash
chmod +x hello.scm
./hello.scm
```

## 2.3 Guile をプログラムにリンク

Guile は共有ライブラリとして利用可能で、C または C++ で書かれたプログラムに Scheme インタープリタを組み込むことができます。これにより、既存のアプリケーションにスクリプト機能を追加できます。

基本的な構造：

1. Guile ライブラリをリンク
2. C コードで `scm_init_guile()` を呼び出す
3. `scm_c_eval_string()` などの関数を使用して Scheme コードを実行

## 2.4 Guile 拡張機能の開発

独自の Guile 拡張機能を C で開発することもできます。これにより、パフォーマンスが必要な部分を C で実装し、Scheme から呼び出すことができます。

## 2.5 Guile モジュールシステムの使用

Guile は モジュールシステムを提供し、コードの再利用と整理が容易になります。

### モジュールの使用

```scheme
(use-modules (srfi srfi-1))  ; SRFI-1 (リスト処理) をインポート
```

### 新しいモジュールの作成

```scheme
(define-module (my-module)
  #:export (my-function))

(define (my-function x)
  (* x 2))
```

### モジュール内の拡張機能の配置

開発した拡張機能をモジュールにまとめることで、再利用可能で保守性の高いコードを作成できます。

## 2.6 バグ報告

Guile の使用中に問題が発生した場合は、GNU Guile プロジェクトに報告できます。

**報告先**: https://www.gnu.org/software/guile/

報告時には以下の情報を含めることが推奨されます：

- Guile のバージョン
- 使用しているオペレーティングシステム
- エラーメッセージの全文
- 問題を再現するコード例

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
