# 2. Guile を始める

> **原文**: [Guile Reference Manual - Hello Guile!](https://www.gnu.org/software/guile/manual/guile.html#Hello-Guile)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、Guile を実際に使い始めるための基本的な方法を紹介します。

## 2.1 Guile を対話的に実行する

### 概要
Guile はインタラクティブなコマンドラインツール（REPL: Read-Eval-Print Loop）を提供しています。これにより、Scheme コードをリアルタイムで実行・テストできます。

### 起動方法

```bash
guile
```

### 基本的な使用例

```scheme
scheme@(guile-user)> (+ 2 3)
5

scheme@(guile-user)> (define x 10)

scheme@(guile-user)> (* x 2)
20

scheme@(guile-user)> (display "Hello!\n")
Hello!

scheme@(guile-user)> (exit)
```

### REPL の終了

```scheme
(exit)
```

## 2.2 Guile スクリプトの実行

### 概要
Scheme コードをスクリプトファイルとして実行することができます。ファイルには実行ビットを付けて、直接実行することも、`guile` コマンドで実行することも可能です。

### スクリプトファイルの作成

`hello.scm` という名前で以下のファイルを作成：

```scheme
#!/usr/bin/guile -s
!#

(display "Hello, Guile!\n")
```

### スクリプトの実行方法

**方法1: guile コマンドで実行**
```bash
guile hello.scm
```

**方法2: スクリプトとして直接実行**
```bash
chmod +x hello.scm
./hello.scm
```

### スクリプトの構造

- `#!/usr/bin/guile -s` - Guile インタプリタのパス
- `!#` - スクリプトのメタデータ終了マーク
- 以降 - Scheme コード

## 2.3 Guile をプログラムにリンク

### 概要
Guile は共有ライブラリとして利用可能で、C または C++ で書かれたプログラムに Scheme インタープリタを組み込むことができます。これにより、既存のアプリケーションにスクリプト機能を追加できます。

### 基本的な構造

1. Guile ライブラリをリンク
2. C コードで `scm_init_guile()` を呼び出す
3. `scm_c_eval_string()` などの関数を使用して Scheme コードを実行

### サンプルコード（C）

```c
#include <libguile.h>

static void *
inner_main(void *data)
{
  scm_c_eval_string("(display \"Hello from C!\n\")");
  return NULL;
}

int
main(int argc, char *argv[])
{
  scm_boot_guile(argc, argv, inner_main, NULL);
  return 0;
}
```

### ビルド方法

```bash
gcc `pkg-config --cflags guile-3.0` \
    program.c \
    `pkg-config --libs guile-3.0` -o program
```

## 2.4 Guile 拡張機能の開発

### 概要
独自の Guile 拡張機能を C で開発することもできます。これにより、パフォーマンスが必要な部分を C で実装し、Scheme から呼び出すことができます。

### 拡張機能の基本

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

### 使用方法

```scheme
(load-extension "libmymodule" "init_my_module")
(double 5)  ; => 10
```

## 2.5 Guile モジュールシステムの使用

### 概要
Guile はモジュールシステムを提供し、コードの再利用と整理が容易になります。

### 2.5.1 モジュールの使用

```scheme
(use-modules (srfi srfi-1))  ; SRFI-1 (リスト処理) をインポート
(use-modules (ice-9 regex))  ; 正規表現ライブラリをインポート
```

### 2.5.2 新しいモジュールの作成

```scheme
(define-module (my-module)
  #:export (my-function my-constant))

(define my-constant 42)

(define (my-function x)
  (* x 2))
```

### 2.5.3 モジュール内の拡張機能の配置

開発した拡張機能をモジュールにまとめることで、再利用可能で保守性の高いコードを作成できます。

```scheme
(define-module (my-project utils)
  #:use-module (srfi srfi-1)
  #:export (helper-function))

(define (helper-function lst)
  (map (lambda (x) (* x 2)) lst))
```

## 2.6 バグ報告

### 概要
Guile の使用中に問題が発生した場合は、GNU Guile プロジェクトに報告できます。

### 報告先

**公式ウェブサイト**: https://www.gnu.org/software/guile/

### 報告時に含めるべき情報

- Guile のバージョン：`guile --version`
- 使用しているオペレーティングシステム
- エラーメッセージの全文
- 問題を再現するコード例（可能な限り簡潔に）
- 予期される動作と実際の動作の違い

### バグ報告の例

```
Guile version: 3.0.11
OS: Linux 5.10.0
Error: Unbound variable: undefined-function
Code example:
  (define (test) (undefined-function 42))
  (test)
```

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
