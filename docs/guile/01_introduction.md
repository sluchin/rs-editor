# 1. はじめに

> **原文**: [Guile Reference Manual - Introduction](https://www.gnu.org/software/guile/manual/guile.html#Introduction)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

Guile は Scheme プログラミング言語の実装です。Scheme は Lisp の優雅で概念的にシンプルな方言で、Guy Steele と Gerald Sussman によって創始され、RnRS（Revised^n Reports on Scheme）として知られる一連のレポートによって進化してきました。

## 1.1 Guile と Scheme

Guile は Revised^5 Report on the Algorithmic Language Scheme（R5RS）で説明されている Scheme を実装しており、クリーンで汎用的なデータとコントロール構造を提供しています。R5RS の仕様に加えて、Guile は以下の機能で拡張されています：

- **モジュールシステム** - コードの整理と再利用
- **POSIX システムコール** - オペレーティングシステムとの対話
- **ネットワークサポート** - TCP/UDP通信機能
- **マルチスレッド** - 並行プログラミング
- **動的リンク** - 実行時にライブラリをロード
- **外部関数呼び出し** - C ライブラリとの連携
- **強力な文字列処理** - テキスト操作機能
- **その他の実用的機能** - 実世界のプログラミングに必要な機能

### R6RS と R7RS の対応

- **R6RS（2007年）**: より包括的な標準で、モジュールシステムと多くの標準ライブラリを追加
- **R7RS（2013年）**: R5RS と R6RS の中間的な標準として、コミュニティの分裂を解決
- **SRFI（Scheme Requests for Implementation）**: 標準以外の実用的な拡張機能を提供

## 1.2 C コードとの結合

Guile は C コードとの緊密な統合が可能に設計されており、以下が実現できます：

- 既存の C プログラムに Scheme スクリプト機能を追加
- Scheme から C ライブラリを呼び出し
- C で高速な処理を実装し、Scheme から利用
- C と Scheme のデータ構造を相互に変換

## 1.3 Guile と GNU プロジェクト

Guile は GNU プロジェクトの公式拡張言語として位置づけられており、以下の特徴があります：

- GNU エコシステムとの深い統合
- GNU ツール（Emacs、GCC など）とのシームレスな連携
- フリーソフトウェアの理念に基づいた開発

## 1.4 対話型プログラミング

Guile は対話型のプログラミング環境（REPL: Read-Eval-Print Loop）を提供し、以下を可能にします：

- コードをリアルタイムで実行・テスト
- プログラムの段階的な開発
- 迅速なプロトタイピング
- インタラクティブなデバッグ

## 1.5 複数の言語のサポート

Guile は単なる Scheme インタプリタではなく、複数の言語をサポートします：

- **Scheme** - メインの言語
- **Emacs Lisp** - Emacs と互換性のある Lisp 方言
- **ECMAScript** - JavaScript に似たスクリプト言語
- 他の言語の拡張も可能

## 1.6 Guile の入手とインストール

Guile は複数の方法で入手できます：

### ディストリビューション別インストール

- **Debian/Ubuntu**: `apt install guile-3.0`
- **Fedora/RHEL**: `dnf install guile`
- **Arch Linux**: `pacman -S guile`
- **macOS**: `brew install guile`
- **Windows**: MinGW または WSL で利用可能

### ソースからのビルド

```bash
./configure
make
make install
```

詳細は公式ウェブサイト（https://www.gnu.org/software/guile/）を参照してください。

## 1.7 このマニュアルの構成

このマニュアルの主要な部分：

### Part 1: 基本と使用法
- **Chapter 1: Introduction** - Guile の概要と特徴
- **Chapter 2: Hello Guile!** - Guile を始める方法
- **Chapter 3: Hello Scheme!** - Scheme 言語の基礎

### Part 2: プログラミング
- **Chapter 4: Programming in Scheme** - Scheme でのプログラミング
- **Chapter 5: Programming in C** - C との統合
- **Chapter 6: API Reference** - Guile API リファレンス

### Part 3: 高度なトピック
- モジュールシステム
- 外部関数インターフェース
- スレッド処理
- デバッグとプロファイリング

## 1.8 表記規則

このマニュアルで使用する表記規則：

| 表記 | 意味 |
|------|------|
| `code` | プログラムコードまたは関数名 |
| `filename` | ファイル名またはパス |
| **太字** | 重要な概念や用語 |
| *イタリック* | 強調や参照 |
| `(+ 2 3)` | Scheme コード例 |
| `$ command` | シェルコマンド |
| `=>` | 評価結果を示す矢印 |

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
