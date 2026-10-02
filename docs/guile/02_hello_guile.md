2 こんにちは、Guile! [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello-Guile_0021-1)
------------------------------------------------------------------------------------------

この章では、Guileの様々な使用方法を簡単に紹介します。Guileのソースコード配布パッケージに含まれるexamples/ディレクトリには、さらに多くの使用例が用意されています。また、問題を発見した場合の最適な報告方法についても説明します。

以下の例は、Guile が `/usr/local/` にインストールされていることを前提としています。

* [Guileをインタラクティブに実行する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Running-Guile-Interactively)
* [Guileスクリプトの実行](https://doc.guix.gnu.org/guile/latest/en/guile.html#Running-Guile-Scripts)
* [Guileをプログラムにリンクする](https://doc.guix.gnu.org/guile/latest/en/guile.html#Linking-Guile-into-Programs)
* [Guile拡張機能の作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-Guile-Extensions)
* [Guileモジュールシステムの使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-the-Guile-Module-System)
* [バグの報告](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reporting-Bugs)

* * *

次へ: [Guile スクリプトの実行](https://doc.guix.gnu.org/guile/latest/en/guile.html#Running-Guile-Scripts)、上へ: [Hello Guile!](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello-Guile_0021) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 2.1 Guile を対話的に実行する [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Running-Guile-Interactively-1)

Guileは、最もシンプルな形では、Schemeプログラミング言語の対話型インタープリタとして機能し、ユーザーが端末から入力したScheme式を読み込んで評価します。以下は、Guileとユーザー間のやり取りの例です。ユーザーの入力は、`$`と`scheme@(guile-user)>`のプロンプトの後に表示されます。

ガイル
scheme@(guile-user)> (+ 1 2 3) ; 数字をいくつか追加します
1ドル＝6
scheme@(guile-user)> (define (factorial n) ; 関数を定義します
(もし (ゼロ? n) 1 (\* n (階乗 (- n 1)))))
scheme@(guile-user)> (20の階乗)
2ドル = 2432902008176640000
scheme@(guile-user)> (getpwnam "root") ; /etc/passwd を確認してください
$3 = #("root" "x" 0 0 "root" "/root" "/bin/bash")
scheme@(guile-user)> Cd
$

* * *

次へ: [Guile をプログラムにリンクする](https://doc.guix.gnu.org/guile/latest/en/guile.html#Linking-Guile-into-Programs)、前: [Guile を対話的に実行する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Running-Guile-Interactively)、上: [こんにちは Guile!](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello-Guile_0021) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 2.2 Guile スクリプトの実行 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Running-Guile-Scripts-1)

AWK、Perl、あるいは他のシェルと同様に、Guileはスクリプトファイルを解釈できます。Guileスクリプトとは、Schemeコードのファイルであり、冒頭にGuileの起動方法をオペレーティングシステムに指示する追加情報と、GuileがSchemeコードを処理する方法を指示する情報が記述されています。

以下は簡単なGuileスクリプトです。詳細については、[Guileスクリプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting)を参照してください。

#!/usr/local/bin/guile -s
!#
（「こんにちは、世界！」と表示）
（改行）

* * *

次へ: [Guile拡張機能の作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-Guile-Extensions)、前: [Guileスクリプトの実行](https://doc.guix.gnu.org/guile/latest/en/guile.html#Running-Guile-Scripts)、上: [Hello Guile!](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello-Guile_0021) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 2.3 Guile をプログラムにリンクする [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Linking-Guile-into-Programs-1)

Guileインタープリタはオブジェクトライブラリとして提供されており、Schemeを構成言語または拡張言語として使用するアプリケーションにリンクすることができます。

ここにsimple-guile.cというソースコードを示します。これは、完全なGuileインタープリタを生成するプログラムです。Guileが提供する通常の機能に加えて、`my-hostname`という機能も提供します。

#include <stdlib.h>
#include <libguile.h>

静的SCM
my_hostname (void)
{
char \*s = getenv ("HOSTNAME");
if (s == NULL)
return SCM\_BOOL\_F;
それ以外
return scm_from_locale_string (s);
}

static void
inner_main (void \*data, int argc, char \**argv)
{
scm\_c\_define\_gsubr ("my-hostname", 0, 0, 0, my\_hostname);
scm_shell (argc, argv);
}

整数
main (int argc, char \**argv)
{
scm\_boot\_guile (argc, argv, inner\_main, 0);
return 0; /\* 到達しなかった \*/
}

Guileがシステムに正しくインストールされている場合、上記のプログラムは次のようにコンパイルおよびリンクできます。

$ gcc -o simple-guile simple-guile.c \\
`pkg-config --cflags --libs guile-3.0`

実行すると、新しい `my-hostname` 関数を呼び出すことができる点を除いて、`guile` プログラムとまったく同じように動作します。

$ ./simple-guile
scheme@(guile-user)> (+ 1 2 3)
1ドル＝6
scheme@(guile-user)> (my-hostname)
「火傷」

* * *

次へ: [Guile モジュール システムの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-the-Guile-Module-System)、前: [Guile をプログラムにリンクする](https://doc.guix.gnu.org/guile/latest/en/guile.html#Linking-Guile-into-Programs)、上: [Hello Guile!](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello-Guile_0021) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 2.4 Guile拡張機能の作成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-Guile-Extensions-1)

Guileをプログラムにリンクすることで、プログラムのユーザーがSchemeを利用できるようになります。また、ライブラリをGuileにリンクすることで、その機能をGuileのすべてのユーザーが利用できるようになります。

Guileにリンクされるライブラリは「拡張機能」と呼ばれますが、実際には単なる普通のオブジェクトライブラリです。

以下の例は、Guile の `j0` 関数を Scheme コードで使用できるようにする簡単な拡張機能の書き方を示しています。

#include <math.h>
#include <libguile.h>

SCM
j0\_wrapper (SCM x)
{
return scm_from_double (j0 (scm_to_double (x)));
}

空所
init_bessel()
{
scm\_c\_define\_gsubr ("j0", 1, 0, 0, j0\_wrapper);
}

このC言語のソースファイルを共有ライブラリにコンパイルする必要があります。GNU/Linuxでコンパイルする方法は以下のとおりです。

gcc \`pkg-config --cflags guile-3.0\` \\
-shared -o libguile-bessel.so -fPIC bessel.c

移植性の高い共有ライブラリを作成するには、GNU Libtool の使用をお勧めします（GNU Libtool の [概要](https://www.gnu.org/software/libtool/manual/libtool.html#Top) を参照してください）。

共有ライブラリは、`load-extension`関数を使用して実行中のGuileプロセスにロードできます。すると、`j0`がすぐに利用可能になります。

ガイル
scheme@(guile-user)> (load-extension "./libguile-bessel" "init\_bessel")
scheme@(guile-user)> (j0 2)
1ドル＝0.223890779141236

拡張機能のインストール方法の詳細については、[サイトパッケージのインストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Installing-Site-Packages)を参照してください。

* * *

次へ: [バグの報告](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reporting-Bugs)、前: [Guile拡張機能の作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-Guile-Extensions)、上: [Hello Guile!](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello-Guile_0021) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 2.5 Guileモジュールシステムの使用 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-the-Guile-Module-System-1)

Guileはプログラムをモジュールに分割する機能をサポートしています。モジュールを使用することで、関連するコードをグループ化し、ほぼ独立した部分から完全なプログラムを構成する際の管理が容易になります。

この入門資料を超えるモジュールシステムの詳細については、[モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Modules)を参照してください。

* [モジュールの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Modules)
* [新しいモジュールの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-new-Modules)
* [拡張機能をモジュールに組み込む](https://doc.guix.gnu.org/guile/latest/en/guile.html#Putting-Extensions-into-Modules)

* * *

次へ: [新しいモジュールの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-new-Modules)、上へ: [Guile モジュール システムの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-the-Guile-Module-System) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 2.5.1 モジュールの使用 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Modules-1)

Guileには、文字列処理やコマンドライン解析など、多くの便利なモジュールが付属しています。さらに、他のGuile開発者によって作成されたGuileモジュールも多数存在しますが、これらは手動でインストールする必要があります。

以下に、パイプを介して他のプロセスと通信する手段を提供する `(ice-9 popen)` モジュールと、関数 `read-line` を提供する `(ice-9 rdelim)` モジュールの使用方法を示すサンプル対話型セッションを示します。[1](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT1)

ガイル
scheme@(guile-user)> (use-modules (ice-9 popen))
scheme@(guile-user)> (use-modules (ice-9 rdelim))
scheme@(guile-user)> (define p (open-input-pipe "ls -l"))
scheme@(guile-user)> (read-line p)
1ドル＝「合計30」
scheme@(guile-user)> (read-line p)
$2 = "drwxr-sr-x 2 mgrabmue mgrabmue 1024 Mar 29 19:57 CVS"

* * *

次へ: [拡張機能をモジュールに組み込む](https://doc.guix.gnu.org/guile/latest/en/guile.html#Putting-Extensions-into-Modules)、前: [モジュールの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Modules)、上: [Guile モジュール システムの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-the-Guile-Module-System) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 2.5.2 新しいモジュールの作成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-new-Modules-1)

構文形式 `define-module` を使用して新しいモジュールを作成できます。この形式に続く定義は、次の `define-module` まですべて新しいモジュール内に記述されます。

通常、1つのモジュールは1つのファイルに格納され、そのファイルはGuileが自動的に検出できる場所にインストールされます。次のセッションでは、簡単な例を示します。

$ cat /usr/local/share/guile/site/foo/bar.scm

(define-module (foo bar)
#:export (frob))

(define (frob x) (* 2 x))

ガイル
scheme@(guile-user)> (use-modules (foo bar))
scheme@(guile-user)> (frob 12)
1ドル＝24

モジュールのインストール方法の詳細については、[サイトパッケージのインストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Installing-Site-Packages)を参照してください。

* * *

前へ: [新しいモジュールの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Writing-new-Modules)、上へ: [Guile モジュール システムの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-the-Guile-Module-System) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 2.5.3 拡張機能をモジュールに組み込む [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Putting-Extensions-into-Modules-1)

Schemeコードに加えて、C言語で定義されたものもモジュールに含めることができます。

これを行うには、モジュールを定義する小さな Scheme ファイルを作成し、モジュールの本体内で `load-extension` を直接呼び出します。

$ cat /usr/local/share/guile/site/math/bessel.scm

(define-module (math bessel)
#:export (j0))

(load-extension "libguile-bessel" "init\_bessel")

$ file /usr/local/lib/guile/3.0/extensions/libguile-bessel.so
... ELF 32ビットLSB共有オブジェクト ...
ガイル
scheme@(guile-user)> (use-modules (math bessel))
scheme@(guile-user)> (j0 2)
1ドル＝0.223890779141236

詳細については、[外部拡張機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Extensions)を参照してください。

* * *

前へ: [Guileモジュールシステムの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-the-Guile-Module-System)、上へ: [Hello Guile!](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello-Guile_0021) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 2.6 バグの報告 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reporting-Bugs-1)

インストールに関する問題はすべて、[bug-guile@gnu.org](mailto:bug-guile@gnu.org)まで報告してください。

Guileでバグを見つけた場合は、Guileの開発者に報告してください。開発者が修正してくれます。バグ修正を適用したり、Guileの新しいバージョンをインストールしたりすることが難しい場合は、開発者が回避策を提案してくれる場合もあります。

バグ報告を送信する前に、以下のリストと照らし合わせて、本当にバグを発見したかどうかを確認してください。

ドキュメントと実際の動作が異なる場合は、ドキュメントまたはプログラムのどちらかにバグが見つかったことになります。
* Guileがクラッシュする場合は、バグです。
* Guileがフリーズしたり、タスクの完了に非常に時間がかかったりする場合は、バグです。
計算結果が間違っている場合は、バグです。
* Guile が有効な Scheme プログラムに対してエラーを通知する場合、それはバグです。
* Guile が無効な Scheme プログラムに対してエラーを通知しない場合、明示的に文書化されていない限り、バグである可能性があります。
* ドキュメントの一部が不明瞭で、そのセクションを読み返しても理解できない場合は、バグです。

バグを報告する前に、.guile ファイルを含め、Guile にロードしたプログラムが Guile の動作に影響を与える可能性のある変数を設定していないか確認してください。また、.guile ファイルを読み込まずに Guile を新規起動した場合（初期化ファイルの読み込みを防止するために、`-q` オプションを付けて Guile を起動してください）、問題が発生するかどうかも確認してください。問題が発生しない場合は、問題発生の原因となる Guile にロードする必要のあるプログラムの正確な内容を報告してください。

バグ報告を作成する際は、下記の情報をできる限り多く含めてください。一部の項目が分からない場合でも問題ありませんが、情報が多いほど、バグの診断と修正の可能性が高まります。

* Guile のバージョン番号。この情報は、シェルで「guile --version」を実行するか、Guile 内から「(version)」を呼び出すことで取得できます。
* `config.guess` シェルスクリプトによって判定されるマシンタイプ。Guile をチェックアウトしている場合は、このファイルは `build-aux` にあります。そうでない場合は、[http://git.savannah.gnu.org/gitweb/?p=config.git;a=blob\_plain;f=config.guess;hb=HEAD](http://git.savannah.gnu.org/gitweb/?p=config.git;a=blob_plain;f=config.guess;hb=HEAD) から最新バージョンを取得できます。
    
$ build-aux/config.guess
x86\_64-不明なLinux-GNU
    
* Guile をバイナリ パッケージからインストールした場合は、そのパッケージのバージョン。RPM を使用するシステムでは、`rpm -qa | grep guile` を使用します。DPKG を使用するシステムでは、`dpkg -l | grep guile` を使用します。
* Guileを自分でビルドした場合、使用したビルド構成は次のとおりです。
    
$ ./config.status --config
'--enable-error-on-warning' '--disable-deprecated'...
    
* バグを再現するための詳細な手順。
    
バグが発生するSchemeプログラムをお持ちの場合は、バグレポートにそのプログラムを含めてください。プログラムが大きすぎて含めることができない場合は、コードを最小限のテストケースに縮小してみてください。
    
REPLで問題を再現できるのであれば、それが最善です。REPLで入力した式を書き起こしてください。
    
* 誤った動作の説明。例えば、「Guileプロセスが致命的なシグナルを受け取りました」や「結果として得られた出力は以下のとおりですが、これは間違っていると思います。」など。
    
バグの症状がGuileのエラーメッセージとして現れる場合は、エラーメッセージの正確なテキストと、Schemeプログラムがどのようにしてそのエラーに至ったかを示すバックトレースを報告することが重要です。これは、Guileのデバッガにある`,backtrace`コマンドを使用して実行できます。
    

バグによって Guile がクラッシュする場合は、GDB などの低レベルデバッガからの追加情報が役立つ可能性があります。Guile を自分でビルドした場合は、`meta/gdb-uninstalled-guile` スクリプトを使用して GDB 上で Guile を実行できます。通常の Guile を起動する代わりに、ラッパー スクリプトを起動し、`run` と入力してプロセスを開始し、クラッシュが発生したら `backtrace` と入力してください。そのバックトレースをレポートに含めてください。

* * *

次へ: [Programming in Scheme](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme)、前: [Hello Guile!](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hello- Guile_0021)、上: [The Guile Reference Manual](https://doc.guix.gnu.org/guile/latest/en/guile.html#Top) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]
