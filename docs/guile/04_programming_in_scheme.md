# 4. Scheme でのプログラミング

> **原文**: [Guile Reference Manual - Programming in Scheme](https://www.gnu.org/software/guile/manual/guile.html#Programming-in-Scheme)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、Guile を使用して実際に Scheme プログラムを開発する方法を説明します。

## 4.1 Guile の Scheme 実装

Guile は R5RS、R6RS、R7RS Scheme 標準をサポートしており、多くの SRFI（Scheme Requests for Implementation）モジュールも提供しています。

### 標準化への対応

- **R5RS**: 基本的な Scheme 標準
- **R6RS**: より包括的な標準で、モジュールシステムを追加
- **R7RS**: R5RS と R6RS の中間的な標準
- **SRFI**: 実用的な拡張機能を定義する個別の提案

## 4.2 Guile の起動

### コマンドラインオプション

```bash
guile [options] [script] [args]
```

主要なオプション：

- `-s` - スクリプトファイルを実行
- `--` - オプションの終了、以降は引数として処理
- `-e` - 式を評価
- `--version` - バージョン表示
- `--help` - ヘルプ表示

### 環境変数

- `GUILE_LOAD_PATH` - モジュール検索パス
- `GUILE_LOAD_COMPILED_PATH` - コンパイル済みモジュールの検索パス

## 4.3 Guile スクリプティング

### スクリプトファイルの構造

```scheme
#!/usr/bin/guile -s
!#
(display "Hello, Guile!\n")
```

### メタスイッチ

`#!` で始まる最初の行により、Guile スクリプトの実行方法を制御できます。

### コマンドライン引数の処理

```scheme
#!/usr/bin/guile -s
!#
(define (main args)
  (format #t "Arguments: ~a\n" args))

(main (cdr (program-arguments)))
```

### スクリプティングの例

- ファイル処理
- テキスト変換
- システム管理タスク自動化

## 4.4 Guile を対話的に使用する

### init ファイル

`~/.guile` ファイルを作成すると、Guile 起動時に自動的に実行されます。

### Readline サポート

Guile は GNU Readline ライブラリをサポートし、行編集とコマンド履歴機能を提供しています。

### 値の履歴

前の評価結果は特殊な変数で参照できます：

```scheme
$1      ; 最初の結果
$2      ; 2 番目の結果
```

### REPL コマンド

REPL では以下の特殊コマンドが利用可能です：

- `,help` - ヘルプ表示
- `,module` - モジュール操作
- `,language` - 言語切り替え
- `,compile` - コンパイル設定
- `,profile` - プロファイリング
- `,debug` - デバッグコマンド

### エラーハンドリング

Guile は詳細なエラーメッセージとスタックトレースを提供し、デバッグを支援します。

### 対話的デバッグ

REPL でのデバッグ機能：

- ブレークポイント設定
- ステップ実行
- スタック検査

## 4.5 Emacs での Guile 使用

Guile は Emacs と統合でき、Emacs Lisp と Scheme の共存が可能です。

## 4.6 Guile ツール

Guile は様々なコマンドラインツールを提供しており、スクリプト開発と保守を支援します。

## 4.7 サイトパッケージのインストール

Scheme モジュールをシステムにインストールして、複数のプロジェクトで再利用できます。

## 4.8 Guile コードの配布

Scheme プログラムを他のユーザーに配布するための方法とベストプラクティス。

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
