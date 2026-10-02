# 1. はじめに

> **原文**: [Guile Reference Manual - Introduction](https://www.gnu.org/software/guile/manual/guile.html#Introduction)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

Guile は GNU プロジェクトの一部として開発された、洗練された拡張可能プログラミング言語です。Scheme 言語の実装であり、GNU エコシステム全体で使用されています。

## 1.1 Guile と Scheme

### 概要

Guile は Scheme プログラミング言語の完全な実装です。Scheme は Lisp の優雅で概念的にシンプルな方言で、Guy Steele と Gerald Sussman によって創始されました。

### R5RS への対応

Guile は Revised^5 Report on the Algorithmic Language Scheme（通常は R5RS と呼ばれる）で説明されている Scheme を実装しており、クリーンで汎用的なデータ構造と制御構造を提供しています。

```scheme
; 基本的な Scheme コード例
(+ 1 2)                         ; => 3
(define (factorial n)
  (if (<= n 1) 1
      (* n (factorial (- n 1)))))
(factorial 5)                   ; => 120
```

### R5RS を超えた機能

R5RS の仕様に加えて、Guile は実世界のプログラミングに必要な以下の機能で拡張されています：

- **モジュールシステム** - コードの整理と再利用
- **POSIX システムコール** - オペレーティングシステムとの対話
- **ネットワークサポート** - TCP/UDP 通信機能
- **マルチスレッド** - 並行プログラミング
- **動的リンク** - 実行時にライブラリをロード
- **外部関数呼び出し（FFI）** - C ライブラリとの連携
- **強力な文字列処理** - テキスト操作機能
- **オブジェクト指向プログラミング** - GOOPS システム
- **その他の実用的機能** - 実世界のプログラミングに必要な機能

### R6RS への対応

2007年、Scheme コミュニティは R6RS（Revised^6 Report）を発表しました。R6RS は RnRS シリーズの重要な一部であり、コア Scheme 言語を拡張し、多くの非コア関数を標準化しました。

```scheme
; R6RS の特徴
; - より包括的な標準
; - モジュールシステムの標準化
; - ライブラリシステム
; - より多くのプリミティブ関数
```

Guile は R6RS の機能をほぼすべて取り込み、既存の機能を R6RS 仕様に適合させるように更新されています。

### SRFI（Scheme Requests for Implementation）

R6RS の公式化に並行して、SRFI プロセス（http://srfi.schemers.org/）が、マルチスレッドプログラミングや多次元配列などの実用的なニーズのためのインターフェースを標準化しています。Guile は多くの SRFI をサポートしており、詳細は「SRFI サポートモジュール」で記載されています。

### Scheme コミュニティの分裂と R7RS

R6RS 標準に至るプロセスは、Scheme コミュニティの分裂を表面化させました：

- **R6RS 支持派**: R5RS では有用で移植可能なプログラムを書くのが不可能であり、野心的な標準のみがこの問題を解決できると考えた
- **R7RS 支持派**: R6RS は範囲が広すぎると考え、より最小限の Scheme 実装では採用されないコンポーネントが含まれていると主張

2013年、R7RS 派が公式 Scheme 標準化トラックの制御を取得し、より限定的な R7RS を発表しました。R7RS は本質的に R5RS と、モジュールシステムの追加からなり、より軽量なアプローチを採用しています。

### Guile の標準対応まとめ

Guile は以下のすべての標準をサポート：

- **R5RS**: 基本的かつ最も広く採用されている Scheme 標準
- **R6RS**: より包括的な標準、オブジェクト指向機能を含む
- **R7RS**: R5RS と R6RS の中間、軽量で実用的
- **SRFI**: 特定の機能領域を標準化する提案

> **推奨事項**: ほとんどのユーザーにとって、クロス実装移植性の必要性が特定されるまで、問題を解決する際に役立つ Guile の部分を使用することをお勧めします。標準から派生したものであっても、Guile 固有のものであってもかまいません。

## 1.2 C コードとの統合

### 概要

Guile の最大の特徴の一つは、既存の C コードとシームレスに統合できることです。これにより、C アプリケーションに Scheme スクリプト機能を追加できます。

### 統合のアプローチ

#### シェルとしての使用

シェルと同様に、Guile は以下の方法で実行できます：

```bash
# インタラクティブモード
guile

# スクリプトインタプリタ
guile script.scm

# 式を評価
guile -c "(+ 2 3)"
```

#### ライブラリとしての使用

Guile は `libguile` オブジェクトライブラリを提供し、他のアプリケーションが完全な Scheme インタプリタを容易に組み込むことができます：

```c
#include <libguile.h>

static void *
inner_main(void *data)
{
  scm_c_eval_string("(display \"Hello from Guile!\n\")");
  return NULL;
}

int
main(int argc, char *argv[])
{
  scm_boot_guile(argc, argv, inner_main, NULL);
  return 0;
}
```

### C と Scheme の相互作用

C コードから Scheme コードを呼び出し、その逆も同様に容易です：

```c
/* Scheme 関数を呼び出し */
SCM scheme_func = scm_c_eval_string("(lambda (x) (* x 2))");
SCM result = scm_call_1(scheme_func, scm_from_int(5));
int c_result = scm_to_int(result);  /* => 10 */
```

```scheme
; C 関数を呼び出し（拡張機能として定義）
(load-extension "libmy-extension" "init_my_extension")
(my-c-function 42)
```

### Guile デザインの4つの側面

C と Scheme の統合を容易にする Guile の設計には4つの重要な側面があります：

1. **拡張言語としての設計**: Guile は常に拡張言語として開発されてきたため、C API の重要性が高く、それに応じて発展しています

2. **保守的ガベージコレクション**: Guile は保守的なガベージコレクションを使用するため、ほとんどの既存 C コードを Guile に変更なしでグルーできます

3. **継続の実装**: Guile は Scheme の継続の概念を C スタックの複製と再確立で実装し、奇妙な Scheme 実行フローに対応する必要がありません

4. **モジュールシステム**: モジュールシステムにより、拡張機能が相互に干渉することなく共存できます

### ドメイン固有言語（DSL）の作成

アプリケーションは Guile を通じて以下をすることができます：

- 新しい関数を追加
- 新しいデータ型を定義
- 新しい制御構造を実装
- 新しい構文を定義

これらにより、タスク向けに調整されたドメイン固有言語を作成できます：

```scheme
; カスタム DSL の例
(define-syntax with-output-to-file
  (syntax-rules ()
    ((with-output-to-file filename body ...)
     (call-with-output-file filename
       (lambda (port)
         (let ((old-port (current-output-port)))
           (set-current-output-port port)
           (dynamic-wind
             (lambda () #f)
             (lambda () body ...)
             (lambda () (set-current-output-port old-port)))))))))
```

### モジュールシステムの利点

Guile のモジュールシステムにより、大規模プログラムを管理可能なセクションに分割でき、セクション間に明確なインターフェースを定義できます：

- モジュールは解釈コードとコンパイルコードの混合を含められます
- Guile は静的またはダイナミックリンクを使用してコンパイルコードを組み込めます
- モジュールは開発者が再利用可能なルーチン集をパッケージ化して配布するよう促します

## 1.3 Guile と GNU プロジェクト

### 概要

Guile は GNU プロジェクトが Emacs Lisp の拡張言語としての素晴らしい成功に続いて構想しました。

### 構想

Emacs Lisp が Emacs 環境内で完全で予期しないアプリケーションの記述を可能にしたのと同様に、Guile も他の GNU プロジェクトアプリケーションに対して同じことを行うべきという考え方です。この哲学は現在も変わっていません。

### 拡張性とソフトウェアの自由

拡張性の概念は GNU プロジェクトの主要目標である**ソフトウェアの自由**と密接に関連しています。

**ソフトウェアの自由**とは、ソフトウェアパッケージを受け取った人々が以下を行える権利です：

- ソフトウェアを修正または強化
- ソフトウェアの元の開発者が想定していなかった方法で使用

コンパイル言語（C など）で書かれたプログラムの場合、この自由は C コードの修正と再構築をカバーします。しかし、プログラムが拡張言語も提供する場合、これは通常、ユーザーが自分の変更を開始するための、より親切で低い参入障壁です。

### GNU アプリケーションでの Guile の使用

Guile は現在、以下の GNU プロジェクトアプリケーションで使用されています：

- **AutoGen** - コードジェネレーション
- **Lilypond** - 音楽譜作成
- **Denemo** - 音楽編集
- **Mailutils** - メール処理
- **TeXmacs** - 科学文書編集
- **Gnucash** - 財務管理

将来、さらに多くの GNU アプリケーションで Guile が採用されることが期待されています。

## 1.4 対話的プログラミング

### 概要

Guile は対話的プログラミング環境として設計されており、これは多くの Scheme 実装と異なる特徴です。

### ソフトウェアの自由と対話性

非フリーソフトウェアは、ユーザーがその仕組みを見ることに関心がありません。ユーザーは単にそれを受け入れるか、問題を報告して、ソースコードの所有者が対処することを望むだけです。

**フリーソフトウェア**は、非フリーソフトウェアと同じくらい確実に動作することを目指しますが、その仕組みを利用可能にすることで、ユーザーを支援すべきです。これは以下の理由から有用です：

- **教育**: ソースコードを研究してプログラミングを学ぶ
- **監査**: セキュリティと正確性を検証
- **拡張**: 新機能を追加
- **デバッグ**: 問題のトラブルシューティング

### 理想的なフリーソフトウェアシステム

理想的なフリーソフトウェアシステムは以下の特性を持ちます：

1. **ソースコードへのアクセス**: 使用している機能のソースコードを見ることが簡単
2. **ステップスルーデバッグ**: ソースコードをステップバイステップで実行を追跡可能
3. **ホットリロード**: ソースコードの一部を修正し、実行中のプログラムに再ロードして即座に反映

**例**:
- Emacs のヘルプシステムのソースコードハイパーリンク
- edebug によるインタラクティブデバッグ

### Guile の設計哲学

Guile は対話的プログラミングのために設計されており、これは多くの Scheme 実装と区別します。他の多くの Scheme 実装は、固定 Scheme プログラムをできるだけ高速に実行することを優先化しています。

**トレードオフ**: 実装者の観点からは、パフォーマンスと実行中プログラムの部分的な修正能力の間にはトレードオフがあります。より高速な Scheme 実装は存在しますが、Guile は GNU プロジェクトであるため、GNU プログラミング自由と実験のビジョンを優先化しています。

## 1.5 複数言語のサポート

### 概要

Guile 2.0 リリース以来、Guile のアーキテクチャはあらゆる言語をコア仮想マシンバイトコードにコンパイルをサポートし、Scheme はサポートされている言語の1つに過ぎません。

### サポート言語

Guile が現在サポートしている言語：

- **Scheme** - メインの言語
- **Emacs Lisp** - Emacs との互換性のための Lisp 方言
- **ECMAScript** - JavaScript として一般的に知られる
- **Brainfuck** - 難解なプログラミング言語

進行中または検討中の言語：

- **Lua** - スクリプト言語
- **Ruby** - オブジェクト指向スクリプト言語
- **Python** - 人気のあるスクリプト言語

### ユーザーの選択

この設計により、Guile を使用するアプリケーションのユーザーは、アプリケーション作成者の好みを押し付けられるのではなく、自分の選択言語でプログラムできます。

## 1.6 Guile の入手とインストール

### ダウンロード

Guile は主要な GNU アーカイブサイト ftp://ftp.gnu.org またはそのミラーから入手できます。ファイルは `guile-version.tar.gz` という名前です。

```bash
# 現在のバージョン（3.0.11）をダウンロード
ftp://ftp.gnu.org/gnu/guile/guile-3.0.11.tar.gz
```

### アンバンドル

```bash
zcat guile-3.0.11.tar.gz | tar xvf -
```

これにより、すべてのソースを含む `guile-3.0.11` というディレクトリが作成されます。

### ビルドとインストール

`INSTALL` ファイルを参照して詳細な手順を確認できますが、通常は以下で十分です：

```bash
cd guile-3.0.11
./configure
make
make install
```

### インストール内容

このコマンドは以下をインストール：

- Guile 実行可能ファイル（`guile`）
- Guile ライブラリ（`libguile`）
- 関連するヘッダファイルとサポートライブラリ
- Guile リファレンスマニュアル

### R5RS 標準の含含

このマニュアルは Scheme 標準（R5RS）を頻繁に参照するため、Guile 配布に R5RS レポートが含まれています。

```bash
# インストール後、info ディレクトリで利用可能
info guile
info r5rs
```

### ディストリビューション別インストール

```bash
# Debian/Ubuntu
apt install guile-3.0

# Fedora/RHEL
dnf install guile

# Arch Linux
pacman -S guile

# macOS
brew install guile

# Windows（WSL または MinGW）
# WSL で上記の Linux コマンドを使用
```

## 1.7 このマニュアルの構成

### 概要

このマニュアルの残りは以下の章で構成されています。すべてのセクションは対応する章で再度、より詳細に記載されています。

### 主要な章

#### Chapter 2: Guile を始める（Hello Guile!）

概要を示します：

- Guile を対話的に使用する方法
- Guile をスクリプトインタプリタとして使用する方法
- Guile を独自のアプリケーションにリンクする方法
- 解釈およびコンパイルされたコードのモジュールを作成する方法

#### Chapter 3: Scheme へようこそ（Hello Scheme!）

Scheme 初心者向けの章：

- Scheme 言語の基本的な考え方の紹介
- このマテリアルはあらゆる Scheme 実装に適用可能
- Guile 固有の内容は含まれない

#### Chapter 4: Scheme でのプログラミング（Programming in Scheme）

Guile での Scheme プログラミングの概要：

- `guile` プログラムをコマンドラインから呼び出す方法
- Scheme でスクリプトを書く方法
- Guile が標準 Scheme を超えて提供する拡張

#### Chapter 5: C でのプログラミング（Programming in C）

Guile を C プログラムで使用する方法の概要：

- Guile にアクセスするために理解すべき基本概念
- 動的型とガベージコレクタ
- 新しいデータ型と関数を定義する方法
- チュートリアル形式の説明

#### Chapter 6: Guile API リファレンス

Guile API の詳細なドキュメント：

- 機能ベースのグループに整理
- Scheme と C インターフェースを並行して表示

#### Chapter 7: Guile モジュール

Guile 配布の一部として配布される重要なモジュール：

- Guile Scheme コアの機能を拡張するモジュール

#### Chapter 8: GOOPS

GOOPS（Guile Object-Oriented Programming System）の説明：

- Guile のオブジェクト指向拡張
- クラス、複数継承、ジェネリック関数

## 1.8 表記規則

### 概要

このマニュアルでは、Scheme 式の評価と結果を示すために特定の表記を使用します。

### 戻り値の表示

評価結果を示すのに `⇒` 記号を使用します：

```scheme
(+ 1 2)
⇒ 3
```

### 出力と戻り値の区別

手続きが戻り値に加えて出力を生成する場合、`⊣` 記号で出力を示します：

```scheme
(begin (display 1) (newline) 'hooray)
⊣ 1
⇒ hooray
```

この例では：
- `⊣ 1` - `display` で出力される 1
- `⇒ hooray` - 式が返す値

### コード例での表記

#### エラーメッセージ

エラーが発生した場合の表記：

```scheme
(define x y)
;; Error: Unbound variable: y
```

#### REPL プロンプト

Guile REPL での対話例：

```scheme
scheme@(guile-user)> (+ 2 3)
$1 = 5
scheme@(guile-user)> (* 3 4)
$2 = 12
```

#### パラメータ記述

手続きの説明での表記：

- `procedure (arg1 arg2 ...)` - 必須引数
- `procedure [optional-arg]` - オプション引数
- `procedure arg ...` - 可変長引数

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
