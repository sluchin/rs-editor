# 2 Hello Guile!

> **原文**: [Guile Reference Manual - Hello Guile!](https://www.gnu.org/software/guile/manual/html_node/Hello-Guile_0021.html)
>
> このドキュメントは GNU Free Documentation License の下で公開されている原文の翻訳です。

この章では、Guile を使用できるあらゆる方法を駆け足で紹介します。Guile のソース配布物の `examples/` ディレクトリには、追加の例があります。また、見つけた問題を報告する最善の方法についても説明します。

以下の例では、Guile が `/usr/local/` にインストールされていることを前提としています。

- Guile を対話的に実行する
- Guile スクリプトを実行する
- Guile をプログラムにリンクする
- Guile 拡張を書く
- Guile モジュールシステムを使う
- バグの報告

## 2.1 Guile を対話的に実行する

最も単純な形では、Guile は Scheme プログラミング言語の対話的インタプリタとして動作し、ユーザーが端末から入力した Scheme 式を読み取って評価します。以下は Guile とユーザーとの対話の例です。ユーザーの入力は `$` および `scheme@(guile-user)>` プロンプトの後に表示されます。

```
$ guile
scheme@(guile-user)> (+ 1 2 3)                ; いくつかの数を足す
$1 = 6
scheme@(guile-user)> (define (factorial n)    ; 関数を定義する
                       (if (zero? n) 1 (* n (factorial (- n 1)))))
scheme@(guile-user)> (factorial 20)
$2 = 2432902008176640000
scheme@(guile-user)> (getpwnam "root")        ; /etc/passwd を調べる
$3 = #("root" "x" 0 0 "root" "/root" "/bin/bash")
scheme@(guile-user)> C-d
$
```

## 2.2 Guile スクリプトを実行する

AWK や Perl、あるいは任意のシェルと同様に、Guile はスクリプトファイルを解釈できます。Guile スクリプトとは、単に Scheme コードのファイルの先頭に、オペレーティングシステムに Guile の起動方法を伝え、次に Guile に Scheme コードの扱い方を伝える追加情報を付けたものです。

以下は簡単な Guile スクリプトです。詳細については「Guile スクリプティング」を参照してください。

```scheme
#!/usr/local/bin/guile -s
!#
(display "Hello, world!")
(newline)
```

## 2.3 Guile をプログラムにリンクする

Guile インタプリタはオブジェクトライブラリとして利用でき、Scheme を設定言語または拡張言語として使用するアプリケーションにリンクできます。

以下は、完全な Guile インタプリタを生成するプログラムのソースコード `simple-guile.c` です。Guile が提供する通常の関数すべてに加えて、関数 `my-hostname` も提供します。

```c
#include <stdlib.h>
#include <libguile.h>

static SCM
my_hostname (void)
{
  char *s = getenv ("HOSTNAME");
  if (s == NULL)
    return SCM_BOOL_F;
  else
    return scm_from_locale_string (s);
}

static void
inner_main (void *data, int argc, char **argv)
{
  scm_c_define_gsubr ("my-hostname", 0, 0, 0, my_hostname);
  scm_shell (argc, argv);
}

int
main (int argc, char **argv)
{
  scm_boot_guile (argc, argv, inner_main, 0);
  return 0; /* never reached */
}
```

Guile がシステムに正しくインストールされていれば、上記のプログラムは次のようにコンパイルおよびリンクできます。

```
$ gcc -o simple-guile simple-guile.c \
    `pkg-config --cflags --libs guile-3.0`
```

これを実行すると、新しい `my-hostname` 関数も呼び出せる点を除けば、`guile` プログラムとまったく同じように動作します。

```
$ ./simple-guile
scheme@(guile-user)> (+ 1 2 3)
$1 = 6
scheme@(guile-user)> (my-hostname)
"burns"
```

## 2.4 Guile 拡張を書く

Guile をプログラムにリンクして、プログラムのユーザーが Scheme を使えるようにすることができます。また、ライブラリを Guile にリンクして、その機能を Guile のすべてのユーザーが利用できるようにすることもできます。

Guile にリンクされるライブラリは**拡張**（extension）と呼ばれますが、実際には普通のオブジェクトライブラリにすぎません。

次の例は、`j0` 関数を Scheme コードから利用できるようにする、Guile 用の簡単な拡張の書き方を示しています。

```c
#include <math.h>
#include <libguile.h>

SCM
j0_wrapper (SCM x)
{
  return scm_from_double (j0 (scm_to_double (x)));
}

void
init_bessel ()
{
  scm_c_define_gsubr ("j0", 1, 0, 0, j0_wrapper);
}
```

この C ソースファイルは共有ライブラリにコンパイルする必要があります。GNU/Linux での方法は次のとおりです。

```
gcc `pkg-config --cflags guile-3.0` \
  -shared -o libguile-bessel.so -fPIC bessel.c
```

共有ライブラリを移植性のある方法で作成するには、GNU Libtool の使用を推奨します（『GNU Libtool』の「Introduction」を参照）。

共有ライブラリは、関数 `load-extension` を使って実行中の Guile プロセスに読み込むことができます。すると、`j0` はすぐに利用可能になります。

```
$ guile
scheme@(guile-user)> (load-extension "./libguile-bessel" "init_bessel")
scheme@(guile-user)> (j0 2)
$1 = 0.223890779141236
```

拡張をインストールする方法の詳細については、「サイトパッケージのインストール」を参照してください。

## 2.5 Guile モジュールシステムを使う

Guile はプログラムをモジュールに分割することをサポートしています。モジュールを使うことで、関連するコードをまとめてグループ化し、ほぼ独立した部品から完全なプログラムを構成することを管理できます。

この入門的な内容を超えるモジュールシステムの詳細については、「モジュール」を参照してください。

- モジュールを使う
- 新しいモジュールを書く
- 拡張をモジュールに入れる

### 2.5.1 モジュールを使う

Guile には、たとえば文字列処理やコマンドライン解析のための、多くの有用なモジュールが付属しています。さらに、他の Guile ハッカーが書いた多くの Guile モジュールも存在しますが、それらは手動でインストールする必要があります。

以下は、パイプを介して他のプロセスと通信する手段を提供する `(ice-9 popen)` モジュールを、関数 `read-line` を提供する `(ice-9 rdelim)` モジュールと一緒に使用する方法を示す対話セッションの例です。[^1]

```
$ guile
scheme@(guile-user)> (use-modules (ice-9 popen))
scheme@(guile-user)> (use-modules (ice-9 rdelim))
scheme@(guile-user)> (define p (open-input-pipe "ls -l"))
scheme@(guile-user)> (read-line p)
$1 = "total 30"
scheme@(guile-user)> (read-line p)
$2 = "drwxr-sr-x    2 mgrabmue mgrabmue     1024 Mar 29 19:57 CVS"
```

[^1]: 「ice-9」は、Kurt Vonnegut の小説『猫のゆりかご』（Cat's Cradle）に登場する架空の物質への言及です（「Status, or: Your Help Needed」を参照）。

### 2.5.2 新しいモジュールを書く

構文形式 `define-module` を使って新しいモジュールを作成できます。この形式に続く、次の `define-module` までのすべての定義は、新しいモジュールに置かれます。

通常、1つのモジュールは1つのファイルに置かれ、そのファイルは Guile が自動的に見つけられる場所にインストールされます。次のセッションは簡単な例を示しています。

```
$ cat /usr/local/share/guile/site/foo/bar.scm

(define-module (foo bar)
  #:export (frob))

(define (frob x) (* 2 x))

$ guile
scheme@(guile-user)> (use-modules (foo bar))
scheme@(guile-user)> (frob 12)
$1 = 24
```

モジュールをインストールする方法の詳細については、「サイトパッケージのインストール」を参照してください。

### 2.5.3 拡張をモジュールに入れる

Scheme コードに加えて、C で定義されたものもモジュールに入れることができます。

これを行うには、モジュールを定義し、モジュールの本体で直接 `load-extension` を呼び出す小さな Scheme ファイルを書きます。

```
$ cat /usr/local/share/guile/site/math/bessel.scm

(define-module (math bessel)
  #:export (j0))

(load-extension "libguile-bessel" "init_bessel")

$ file /usr/local/lib/guile/3.0/extensions/libguile-bessel.so
… ELF 32-bit LSB shared object …
$ guile
scheme@(guile-user)> (use-modules (math bessel))
scheme@(guile-user)> (j0 2)
$1 = 0.223890779141236
```

詳細については「外部拡張」を参照してください。

## 2.6 バグの報告

インストールに関する問題は、bug-guile@gnu.org に報告してください。

Guile にバグを見つけた場合は、Guile 開発者が修正できるよう、彼らに報告してください。あなた自身がバグ修正を適用したり、新しいバージョンの Guile をインストールしたりできない場合には、回避策を提案してもらえることもあります。

バグ報告を送る前に、本当にバグを見つけたのかどうか、次のリストで確認してください。

- ドキュメントと実際の動作が異なる場合は、ドキュメントかプログラムのどちらかに、確実にバグを見つけたことになります。
- Guile がクラッシュする場合、それはバグです。
- Guile がハングする、またはタスクの完了に永遠に時間がかかる場合、それはバグです。
- 計算が誤った結果を生成する場合、それはバグです。
- 正しい Scheme プログラムに対して Guile がエラーを通知する場合、それはバグです。
- 正しくない Scheme プログラムに対して Guile がエラーを通知しない場合、明示的に文書化されていない限り、それはバグかもしれません。
- ドキュメントのある部分が明確でなく、その節を読み直しても意味が分からない場合、それはバグです。

バグを報告する前に、`.guile` ファイルを含め、Guile に読み込んだプログラムが、Guile の機能に影響を与える可能性のある変数を設定していないか確認してください。また、`.guile` ファイルを読み込まずに新しく起動した Guile でも問題が発生するかどうかを確認してください（初期化ファイルの読み込みを防ぐには、`-q` スイッチを付けて Guile を起動します）。その場合に問題が発生しないのであれば、問題を発生させるために Guile に読み込む必要のあるプログラムの正確な内容を報告しなければなりません。

バグ報告を書くときは、以下で説明する情報をできるだけ多く報告に含めるようにしてください。一部の項目が分からなくても問題はありませんが、得られる情報が多いほど、バグを診断して修正できる可能性が高くなります。

- Guile のバージョン番号。この情報は、シェルで「`guile --version`」を実行するか、Guile 内から `(version)` を呼び出すことで取得できます。
- `config.guess` シェルスクリプトによって判定されるマシンの種類。Guile のチェックアウトがあれば、このファイルは `build-aux` にあります。そうでなければ、最新版を http://git.savannah.gnu.org/gitweb/?p=config.git;a=blob_plain;f=config.guess;hb=HEAD から取得できます。

  ```
  $ build-aux/config.guess
  x86_64-unknown-linux-gnu
  ```

- バイナリパッケージから Guile をインストールした場合は、そのパッケージのバージョン。RPM を使用するシステムでは `rpm -qa | grep guile` を使用します。DPKG を使用するシステムでは `dpkg -l | grep guile` を使用します。
- Guile を自分でビルドした場合は、使用したビルド設定。

  ```
  $ ./config.status --config
  '--enable-error-on-warning' '--disable-deprecated'...
  ```

- バグを再現する方法の完全な説明。

  バグを引き起こす Scheme プログラムがある場合は、それをバグ報告に含めてください。プログラムが大きすぎて含められない場合は、コードを最小限のテストケースに縮小するよう試みてください。

  REPL で問題を再現できるなら、それが最善です。REPL で入力した式の記録を提供してください。

- 誤った動作の説明。たとえば、「Guile プロセスが致命的なシグナルを受け取る」、あるいは「結果の出力は次のとおりで、これは間違っていると思う」など。

  バグの現れ方が Guile のエラーメッセージである場合は、エラーメッセージの正確なテキストと、Scheme プログラムがどのようにしてエラーに至ったかを示すバックトレースを報告することが重要です。これは Guile のデバッガの `,backtrace` コマンドを使って行えます。

バグによって Guile がクラッシュする場合は、GDB などの低レベルデバッガからの追加情報が役立つかもしれません。Guile を自分でビルドした場合は、`meta/gdb-uninstalled-guile` スクリプトを介して GDB の下で Guile を実行できます。通常どおり Guile を起動する代わりにこのラッパースクリプトを起動し、`run` と入力してプロセスを開始し、クラッシュが起きたら `backtrace` と入力します。そのバックトレースを報告に含めてください。

---

> **ライセンス**: この翻訳は GNU Free Documentation License v1.3 以降に基づいて作成されています。
> 原文の著作権: Copyright (C) 1996-2023 Free Software Foundation, Inc.
