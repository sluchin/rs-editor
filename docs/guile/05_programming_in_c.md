# 5. C でのプログラミング

> **原文**: [Guile Reference Manual - Programming in C](https://www.gnu.org/software/guile/manual/guile.html#Programming-in-C)
> 
> このドキュメントはGNU Free Documentation Licenseの下で公開されています。

このセクションでは、C または C++ プログラムに Guile を組み込み、Scheme スクリプト機能を追加する方法を説明します。

## 5.1 複数バージョンのインストール

### 概要

Guile の複数バージョンを同時にシステムにインストールし、異なるプロジェクトで使い分けることができます。

### インストール例

```bash
# Guile 3.0 をインストール
./configure --prefix=/opt/guile-3.0
make install

# Guile 2.2 をインストール（別プレフィックス）
./configure --prefix=/opt/guile-2.2
make install

# 使用時にパスを指定
export PKG_CONFIG_PATH=/opt/guile-3.0/lib/pkgconfig:$PKG_CONFIG_PATH
```

### バージョンの確認

```bash
guile --version
guile-3.0 --version
guile-2.2 --version
```

## 5.2 プログラムを Guile とリンク

### 概要

C プログラムに Guile を組み込むための基本的な手順を説明します。

### 基本的な手順

1. Guile ライブラリをプロジェクトにリンク
2. Guile のヘッダファイルをインクルード
3. `scm_boot_guile()` で初期化
4. Scheme コードを実行

### サンプル Guile メインプログラム

```c
#include <libguile.h>

static void *
inner_main(void *data)
{
  /* Scheme コードを実行 */
  scm_c_eval_string("(display \"Hello, Guile!\n\")");
  
  /* Scheme 関数を呼び出し */
  SCM result = scm_c_eval_string("(+ 2 3)");
  printf("Result: %d\n", scm_to_int(result));
  
  return NULL;
}

int
main(int argc, char *argv[])
{
  /* Guile 環境を初期化して inner_main を実行 */
  scm_boot_guile(argc, argv, inner_main, NULL);
  return 0;
}
```

### ビルド方法（pkg-config 使用）

```bash
gcc `pkg-config --cflags guile-3.0` \
    myprogram.c \
    `pkg-config --libs guile-3.0` -o myprogram
```

### ビルド方法（guile-config 使用）

```bash
gcc `guile-config compile` \
    myprogram.c \
    `guile-config link` -o myprogram
```

### Makefile の例

```makefile
GUILE_CFLAGS = $(shell pkg-config --cflags guile-3.0)
GUILE_LIBS = $(shell pkg-config --libs guile-3.0)

myprogram: myprogram.c
	gcc $(GUILE_CFLAGS) -o myprogram myprogram.c $(GUILE_LIBS)

clean:
	rm -f myprogram
```

### Autoconf でのビルド

大規模なプロジェクトでは、Autoconf と Automake を使用：

```bash
# configure.ac
AC_INIT([myproject], [1.0])
AC_PROG_CC
GUILE_PKG([3.0])
AC_OUTPUT([Makefile])

# Makefile.in
all: myprogram
myprogram: myprogram.c
	$(CC) $(GUILE_CFLAGS) -o myprogram myprogram.c $(GUILE_LIBS)
```

## 5.3 Guile とライブラリをリンク

### 概要

C で書かれた拡張機能は、Scheme から手続きとして呼び出せます。

### Guile 拡張機能の実装

```c
#include <libguile.h>

/* C の関数 */
SCM
my_double(SCM x)
{
  /* SCM 値を C の int に変換 */
  int c_x = scm_to_int(x);
  
  /* 計算を実行 */
  int result = 2 * c_x;
  
  /* 結果を SCM 値に変換して返す */
  return scm_from_int(result);
}

/* モジュールの初期化 */
void
init_my_module(void)
{
  /* Scheme 関数を定義
     引数: ("double", 必須1個, オプション0個, VarArgs, C関数) */
  scm_c_define_gsubr("double", 1, 0, 0, my_double);
}
```

### 拡張機能のビルド

```bash
# 共有ライブラリとしてコンパイル
gcc -fPIC -shared `pkg-config --cflags guile-3.0` \
    my_module.c -o libmymodule.so \
    `pkg-config --libs guile-3.0`
```

### Scheme での使用

```scheme
; 拡張機能をロード
(load-extension "libmymodule" "init_my_module")

; 定義された関数を使用
(double 5)   ; => 10
(double 21)  ; => 42
```

## 5.4 libguile 使用の一般的な概念

### 概要

libguile を使用する際の重要な概念を説明します。

### SCM 型と値変換

```c
/* 整数の変換 */
SCM scm_int = scm_from_int(42);
int c_int = scm_to_int(scm_int);

/* 文字列の変換 */
SCM scm_str = scm_from_locale_string("hello");
char *c_str = scm_to_locale_string(scm_str);

/* ブール値の変換 */
SCM scm_bool = scm_from_bool(1);
int c_bool = scm_is_true(scm_bool);

/* リストの操作 */
SCM list = scm_list_1(scm_from_int(42));
SCM car_val = scm_car(list);
SCM cdr_val = scm_cdr(list);
```

### ガベージコレクション

Guile は自動ガベージコレクションを実装：

```c
/* SCM_PROTECT_RELEASE を使用して GC 対象からの保護 */
SCM my_global_var;

void
init_globals(void)
{
  my_global_var = scm_from_int(42);
  scm_gc_protect_object(my_global_var);  /* GC から保護 */
}

void
cleanup(void)
{
  scm_gc_unprotect_object(my_global_var);  /* 保護を解除 */
}
```

### 制御フロー

C と Scheme 間の制御フローの管理：

```c
static void *
inner_main(void *data)
{
  /* Scheme コード実行 */
  SCM proc = scm_c_eval_string(
    "(lambda (x) (* x 2))");
  
  /* Scheme 関数を呼び出し */
  SCM arg = scm_from_int(5);
  SCM result = scm_call_1(proc, arg);
  
  printf("Result: %d\n", scm_to_int(result));
  return NULL;
}

int
main(int argc, char *argv[])
{
  scm_boot_guile(argc, argv, inner_main, NULL);
  return 0;
}
```

### 非同期シグナルハンドリング

Guile はシグナルを安全に処理：

```c
#include <signal.h>
#include <libguile.h>

static int should_exit = 0;

void
signal_handler(int signum)
{
  should_exit = 1;
}

static void *
inner_main(void *data)
{
  signal(SIGINT, signal_handler);
  
  while (!should_exit) {
    scm_c_eval_string("(process-event)");
  }
  
  return NULL;
}
```

### マルチスレッド対応

Guile はマルチスレッド環境をサポート：

```c
#include <pthread.h>
#include <libguile.h>

void *
thread_proc(void *data)
{
  /* Guile をスレッド化 */
  scm_init_guile();
  
  /* Scheme コードを実行 */
  scm_c_eval_string("(display \"Hello from thread\n\")");
  
  return NULL;
}

int
main(void)
{
  pthread_t thread;
  pthread_create(&thread, NULL, thread_proc, NULL);
  pthread_join(thread, NULL);
  return 0;
}
```

## 5.5 新しい外部オブジェクト型の定義

### 概要

C で定義したデータ構造を Scheme から使用できるように、外部オブジェクト型を定義します。

### 外部オブジェクト型の定義

```c
#include <libguile.h>

/* C のデータ構造 */
typedef struct {
  int x;
  int y;
} Point;

static scm_t_bits point_tag;

/* 生成関数 */
SCM
make_point(SCM x_scm, SCM y_scm)
{
  Point *point = malloc(sizeof(Point));
  point->x = scm_to_int(x_scm);
  point->y = scm_to_int(y_scm);
  
  SCM smob = scm_new_smob(point_tag, (scm_t_bits)point);
  return smob;
}

/* フィールドアクセス */
SCM
point_x(SCM point_smob)
{
  Point *point = (Point *)SCM_SMOB_DATA(point_smob);
  return scm_from_int(point->x);
}

SCM
point_y(SCM point_smob)
{
  Point *point = (Point *)SCM_SMOB_DATA(point_smob);
  return scm_from_int(point->y);
}

/* メモリ解放 */
size_t
free_point(SCM point_smob)
{
  Point *point = (Point *)SCM_SMOB_DATA(point_smob);
  free(point);
  return 0;
}

/* モジュール初期化 */
void
init_point_module(void)
{
  point_tag = scm_make_smob_type("point", 0);
  scm_set_smob_free(point_tag, free_point);
  
  scm_c_define_gsubr("make-point", 2, 0, 0, make_point);
  scm_c_define_gsubr("point-x", 1, 0, 0, point_x);
  scm_c_define_gsubr("point-y", 1, 0, 0, point_y);
}
```

### Scheme での使用

```scheme
(load-extension "libpoint" "init_point_module")

(define p (make-point 10 20))
(point-x p)  ; => 10
(point-y p)  ; => 20
```

## 5.6 関数スナーフィング

### 概要

Scheme と C 間の関数定義を自動化するツール。

### スナーフィングの使用

```c
/* my_functions.c */

/* 素数をチェック（自動登録用） */
SCM_DEFINE(is_prime_p, "is-prime?", 1, 0, 0,
           (SCM n),
           "素数かどうかをチェック")
{
  int num = scm_to_int(n);
  if (num < 2) return SCM_BOOL_F;
  
  for (int i = 2; i * i <= num; i++) {
    if (num % i == 0) return SCM_BOOL_F;
  }
  return SCM_BOOL_T;
}

void
init_my_functions(void)
{
  #include "my_functions.x"  /* 自動生成コード */
}
```

## 5.7 実践的なプログラミング例

### 概要

C と Scheme の組み合わせによる実践的な例。

### 例1: C での計算、Scheme でのロジック

```c
/* 高速な C 関数 */
SCM
c_compute(SCM input)
{
  int result = expensive_calculation(scm_to_int(input));
  return scm_from_int(result);
}

/* Scheme で高レベルロジックを実装 */
(use-modules (my-c-lib))

(define (process-data data)
  (let ((computed (c-compute data)))
    (if (> computed 1000)
        (display "Large result")
        (display "Small result"))))
```

### 例2: C でのデータ構造、Scheme でのアルゴリズム

```c
/* C で高速なデータ構造を実装 */
typedef struct {
  int *data;
  int size;
} Array;

SCM
create_array(SCM size_scm)
{
  /* ... 実装 ... */
}

/* Scheme で高レベルアルゴリズムを記述 */
(define (array-sum arr)
  (let loop ((i 0) (sum 0))
    (if (>= i (array-length arr))
        sum
        (loop (+ i 1) 
              (+ sum (array-ref arr i))))))
```

### テストベッドとしての Guile

```c
/* C の関数をテスト */
int
my_algorithm(int input)
{
  return input * 2 + 1;
}

SCM
test_my_algorithm(SCM input)
{
  return scm_from_int(my_algorithm(scm_to_int(input)));
}

/* Scheme でテストスイート */
(use-modules (srfi srfi-64))
(use-modules (my-c-lib))

(test-begin "algorithm")
(test-equal "test 1" 3 (test-my-algorithm 1))
(test-equal "test 5" 11 (test-my-algorithm 5))
(test-end "algorithm")
```

## 5.8 Autoconf サポート

### 概要

GNU Autoconf ツールを使用した Guile 統合。

### configure.ac の設定

```autoconf
AC_INIT([myproject], [1.0])
AM_INIT_AUTOMAKE([foreign])
AC_PROG_CC
AC_PROG_LIBTOOL

# Guile 3.0 の要求
GUILE_PKG([3.0])

AC_CONFIG_FILES([
  Makefile
  src/Makefile
])
AC_OUTPUT
```

### Autoconf マクロ

Guile が提供する便利なマクロ：

```autoconf
# Guile 3.0 以上を必須に
GUILE_PKG([3.0])

# Guile モジュールをチェック
GUILE_MODULE_REQUIRED([srfi srfi-1])

# Guile のフラグを自動取得
GUILE_CFLAGS
GUILE_LIBS
```

### Makefile.in の例

```makefile
bin_PROGRAMS = myprogram
myprogram_SOURCES = myprogram.c
myprogram_CFLAGS = $(GUILE_CFLAGS)
myprogram_LDFLAGS = $(GUILE_LIBS)

lib_LTLIBRARIES = libmyextension.la
libmyextension_la_SOURCES = my_extension.c
libmyextension_la_CFLAGS = $(GUILE_CFLAGS)
libmyextension_la_LDFLAGS = -module -version-info 0:0:0
```

---

> **ライセンス**: このドキュメント内の翻訳は、GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
