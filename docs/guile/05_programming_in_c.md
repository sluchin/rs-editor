# 5. C でのプログラミング

> **原文**: [Guile Reference Manual - Programming in C](https://www.gnu.org/software/guile/manual/guile.html#Programming-in-C)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、C または C++ プログラムに Guile を組み込み、Scheme スクリプト機能を追加する方法を説明します。

## 5.1 複数バージョンのインストール

Guile の複数バージョンを同時にシステムにインストールし、異なるプロジェクトで使い分けることができます。

## 5.2 プログラムを Guile とリンク

### 基本的な手順

1. Guile ライブラリをプロジェクトにリンク
2. Guile のヘッダファイルをインクルード
3. `scm_init_guile()` で初期化
4. Scheme コードを実行

### サンプル Guile メインプログラム

```c
#include <libguile.h>

static void *
inner_main(void *data)
{
  scm_c_eval_string("(display \"Hello, Guile!\n\")");
  return NULL;
}

int
main(int argc, char *argv[])
{
  scm_boot_guile(argc, argv, inner_main, NULL);
  return 0;
}
```

### Make でのビルド

Guile はコンパイラフラグとリンクフラグを取得するためのスクリプトを提供しています。

```bash
gcc `pkg-config --cflags guile-3.0` \
    myprogram.c \
    `pkg-config --libs guile-3.0` -o myprogram
```

### Autoconf でのビルド

大規模なプロジェクトでは、Autoconf と Automake を使用してビルドシステムを構築します。

## 5.3 Guile とライブラリをリンク

### Guile 拡張機能

C で書かれた拡張機能は、Scheme から手続きとして呼び出せます。

### サンプル Guile 拡張

```c
#include <libguile.h>

SCM
my_double(SCM x)
{
  return scm_from_int(2 * scm_to_int(x));
}

void
init_my_module(void)
{
  scm_c_define_gsubr("double", 1, 0, 0, my_double);
}
```

## 5.4 libguile 使用の一般的な概念

### 動的型付け

Guile は動的型付けであり、すべての値は `SCM` 型で表現されます。

### ガベージコレクション

Guile は自動ガベージコレクションを実装しており、メモリ管理を自動化しています。

### 制御フロー

C と Scheme 間の制御フローの管理が重要です。

### 非同期シグナル

Guile はシグナルを安全に処理できます。

### マルチスレッド

Guile はマルチスレッド環境をサポートしています。

## 5.5 新しい外部オブジェクト型の定義

### 外部オブジェクト型の定義

C で定義したデータ構造を Scheme から使用できるように、外部オブジェクト型を定義します。

### 外部オブジェクトの作成

```c
scm_c_make_foreign_object(type, slots)
```

### 外部オブジェクトの型チェック

```c
scm_assert_foreign_object_type(type, obj)
```

### メモリ管理

外部オブジェクトのメモリ解放を適切に管理します。

### Scheme での外部オブジェクトの使用

C で定義したオブジェクトを Scheme からメソッドのように呼び出せます。

## 5.6 関数スナーフィング

Scheme と C 間の関数定義を自動化するツール。

## 5.7 Guile プログラミングの概要

### Dia への Guile 統合例

画像編集アプリケーション Dia に Guile を統合した実例を紹介します。

### C より Scheme が扱いやすい理由

スクリプト言語の利点と柔軟性。

### テストベッドとしての Guile 使用

複雑なシステムのテストを簡素化。

### プログラミング選択肢

機能実装時に何を C で、何を Scheme で書くかの判断。

## 5.8 Autoconf サポート

### Autoconf の背景

GNU Autoconf ツールの基本概念。

### Autoconf マクロ

Guile 統合用の Autoconf マクロ。

### Autoconf マクロの使用方法

プロジェクトで Guile サポートを追加する手順。

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
