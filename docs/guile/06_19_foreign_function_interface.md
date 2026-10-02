### 6.19 外部関数インターフェース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface-1)

C言語やRust、あるいはScheme以外の言語で書かれたライブラリを使用する必要がある場合があります。さらに稀なケースとして、Guileを拡張するためにC言語でコードを書く必要があるかもしれません。このセクションでは、これらの「外部ライブラリ」をロードする方法、ライブラリ内のデータや関数を検索する方法などについて説明します。

* [外国のライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Libraries)
* [外部拡張機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Extensions)
* [外部ポインタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Pointers)
* [外部型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Types)
* [外部関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Functions)
* [ボイドポインタとバイトアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Void-Pointers-and-Byte-Access)
* [外部構造体](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Structs)
* [その他の外部関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#More-Foreign-Functions)

* * *

次へ: [外部拡張機能](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Extensions)、上: [外部関数インターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.19.1 外国図書館 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Libraries-1)

Guileは実行時にSchemeライブラリをロードできるのと同様に、C言語やその他の低レベル言語で書かれたシステムライブラリもロードできます。これらの動的にロード可能なモジュールを「外部ライブラリ」と呼び、SchemeやGuileが実装する他の言語で書かれたネイティブライブラリと区別します。

外部ライブラリは通常、2つの形態で存在します。圧縮ライブラリ`libz`のように、オペレーティングシステムの一部となっている外部ライブラリもあります。これらの共有ライブラリは、多くのプログラムがコードを重複させることなくその機能を利用できるように構築されています。C言語で書かれたプログラムを作成する際、特定の共有ライブラリセットを使用することを宣言できます。プログラムが実行されると、オペレーティングシステムが共有ライブラリの場所を特定し、ロードします。

プログラム実行時に共有ライブラリを動的にロードおよびリンクできるオペレーティングシステムコンポーネントは、プログラムの実行中にプログラムからアクセスすることもできます。これはGuileにとって最も便利なインターフェースであり、Guileで「動的リンク」と言うときに意味するものです。実行時の動的リンクは、プログラム起動時に行われる動的リンクと区別するために、「dlopening」と呼ばれることもあります。

もう1つの種類の外部ライブラリは、モジュール、プラグイン、バンドル、または拡張機能と呼ばれることもあります。これらの外部ライブラリは、Cプログラムからリンクされることを意図したものではなく、実行時に動的にロードされることを目的としています。つまり、メインプログラムに機能を追加するものであり、それ自体で動作するものではありません。Guileライブラリの中には、ロード可能なモジュールとして一部の機能を実装しているものもあります。

いずれの場合も、Guile側のインターフェースは同じです。`load-foreign-library`を使用してインターフェースをロードします。生成された外部ライブラリオブジェクトは、シンプルなルックアップインターフェースを実装しており、ユーザーはライブラリによってエクスポートされたデータやコードのアドレスを取得できます。外部ライブラリを検査する機能はありません。検査する前に、ライブラリの内容を事前に把握しておく必要があります。

外部ライブラリをロードし、その内容にアクセスするためのルーチンは、`(system foreign-library)` モジュールに実装されています。

(use-modules (system foreign-library))

Scheme Procedure: **load-foreign-library** \[library\] \[#:extensions=system-library-extensions\] \[#:search-ltdl-library-path?=#t\] \[#:search-path=search-path\] \[#:search-system-paths?=#t\] \[#:lazy?=#t\] \[#:global=#f\] \[#:host-type-rename?=#t\] \[#:allow-dll-version-suffix?=#t\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002dforeign_002dlibrary)

このプロシージャは、library（文字列）で指定された共有ライブラリを検索し、実行中のGuileアプリケーションに動的にリンクします。成功した場合、このプロシージャはリンクされたオブジェクトファイルを表すのに適したSchemeオブジェクトを返します。失敗した場合は、エラーが発生します。

一般的な使用法では、ライブラリパラメータはパスも拡張子も指定しないファイル名（例：`.so`、`.dylib`、`.dll`）です。このプロシージャは、OSで一般的に使用される拡張子を用いて、一連の標準的な場所からライブラリを検索します。オプションパラメータによって、この動作をカスタマイズできます。

ライブラリにディレクトリ要素やファイル名拡張子が含まれている場合、より絞り込んだ検索が実行されます。

Guileは、システムごとにデフォルトの拡張子セットを用意しており、それを使って読み込みを試みます。GNUシステムではデフォルトの拡張子セットは`.so`のみです。Windowsでは`.dll`のみです。Darwin（Mac OS）では`.bundle`、`.so`、`.dylib`です。デフォルトの拡張子リストを上書きするには、`#:extensions extensions`を渡してください。ライブラリに拡張子のいずれかが含まれている場合、拡張子は試されません。そのため、読み込むファイルが正確にわかっている場合は、拡張子を指定できます。

ライブラリが絶対ファイル名を示すか、ディレクトリ区切り文字（`/`、Windows では `\`）を含まない限り、Guile は search-paths にリストされているディレクトリ内でライブラリを検索します。デフォルトの検索パスは 3 つのコンポーネントで構成されており、これらはすべてコロン（Windows ではセミコロン）で区切られた環境変数によって上書きできます。

`GUILE_EXTENSIONS_PATH`

これは、ユーザーがGuile拡張機能を含むディレクトリを検索パスに追加するための環境変数です。デフォルト値にはエントリがありません。この環境変数はGuile 3.0.6で追加されました。

`LTDL_LIBRARY_PATH`

search-ltdl-library-path? が true の場合、この環境変数を使用して検索パスにディレクトリを追加することもできます。この環境変数で指定されたディレクトリごとに、指定されたディレクトリ (たとえば `D`) と `.libs` サブディレクトリ (`D/.libs`) の 2 つのディレクトリが検索パスに追加されます。

根拠の詳細については、下記の注記を参照してください。

`GUILE_SYSTEM_EXTENSIONS_PATH`

Guile の検索パスの最後のパスは Guile 自身のパスであり、デフォルトでは libdir と extensiondir の順になります。たとえば、/opt/guile にインストールした場合、これらはそれぞれ /opt/guile/lib と `/opt/guile/lib/guile/3.0/extensions` になります。`extensionsdir` の詳細については、[並列インストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Parallel-Installations) を参照してください。

DLLを使用するシステムの場合、libdirではなくbindirが検索されるため、この例では/opt/guile/binが検索対象となります。

最後に、検索パスにライブラリが見つからず、かつライブラリが絶対パスではなくディレクトリ区切り文字も含まれておらず、さらに search-system-paths? が true の場合、オペレーティングシステムはライブラリの場所を特定するための独自のロジックを持っている可能性があります。たとえば、GNU では、デフォルトのパスセット (多くの場合 /usr/lib と /lib ですが、システムによって異なります) があり、`LD_LIBRARY_PATH` 環境変数で追加のパスを追加できます。DLL を使用するシステムでは、`PATH` が検索されます。他のオペレーティングシステムでは、別の慣習があります。

検索をオペレーティングシステムに頼るのは、通常は良い方法ではありません。あるマシンでは動作するが、別のマシンでは動作しないプログラムを生み出す原因となるからです。しかし、システムライブラリをラップする場合、それが唯一、プログラムを動作させる方法となることもあります。

lazy? が true (デフォルト) の場合、Guile はロードされたライブラリで使用されるシンボルが最初に使用されるときに、オペレーティングシステムにシンボルの解決を要求します。global? が true の場合、ロードされたライブラリによって定義されたシンボルは、他のモジュールがシンボルを解決する必要があるときに利用可能になります。デフォルトは `#f` で、シンボルはローカルに保持されます。

host-type-rename? が true (デフォルト) の場合、ライブラリ名は現在の `%host-type` に基づいて変更されることがあります。Cygwin ホストでは、検索動作が変更され、Cygwin の慣例に従って、「lib」で始まるファイル名は「cyg」という名前で検索されます。同様に、MSYS ホストでは、「lib」は「msys-」になります。

dll-version-suffix? が true (デフォルト) の場合、検索動作が変更され、DLL を検索する際にバージョンサフィックス付きの DLL も検索対象となります。たとえば、libtiff.dll を検索すると、libtiff-1.dll も検索対象となります。バージョン指定のない DLL が見つからず、複数のバージョン指定付き DLL が存在する場合は、最もバージョンの高いバージョン指定付き DLL が返されます。検索時にはディレクトリが優先されることに注意してください。すべての検索ディレクトリをまとめて最もバージョンの高い DLL を返すのではなく、DLL が存在する最初のディレクトリにある最もバージョンの高い DLL を返します。

ライブラリ引数が省略された場合、デフォルト値は`#f`になります。`library`がfalseの場合、作成される外部ライブラリは、現在実行中の実行ファイルで動的リンクに使用可能なすべてのシンボルへのアクセスを提供します。

上記の環境変数は、外部ライブラリモジュールが最初にロードされ、パラメータにバインドされる際に解析されます。パスの構成要素がnullの場合、例えば`GUILE_SYSTEM_EXTENSIONS_PATH="::"`の3つの構成要素は無視されます。

スキームパラメータ: **guile-extensions-path** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-guile_002dextensions_002dpath)

スキームパラメータ: **ltdl-library-path** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ltdl_002dlibrary_002dpath)

スキームパラメータ: **guile-system-extensions-path** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-guile_002dsystem_002dextensions_002dpath)

初期値がそれぞれ `GUILE_EXTENSIONS_PATH`、`LTDL_LIBRARY_PATH`、および `GUILE_SYSTEM_EXTENSIONS_PATH` から取得されるパラメータ。詳細は [Parameters](https://doc.guix.gnu.org/guile/latest/en/guile.html#Parameters) を参照してください。呼び出し元が明示的に `#:search-path` 引数を渡さない限り、`load-foreign-library` が呼び出されたときに検索パスを構築する際に、これらのパラメータの現在の値が使用されます。

スキーム手順: **foreign-library?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-foreign_002dlibrary_003f)

objが外部ライブラリの場合は`#t`を返し、それ以外の場合は`#f`を返します。

Guile 3.0.6より前のバージョンでは、Guileはlibtoolが提供する動的ライブラリローダーである`libltdl`を使用して外部ライブラリをロードしていました。このローダーは`LTDL_LIBRARY_PATH`を使用しており、後方互換性のために現在もそのパスをサポートしています。

しかし、`libltdl` は `.so` (または `.dll` など) ファイルだけでなく、libtool によって作成された `.la` ファイルも開きます。インストール済みのライブラリ (`make install` のターゲットディレクトリにあるライブラリ) では、`.la` ファイルは決して必要ありません。そのため、ほとんどの GNU/Linux ディストリビューションでは、`.la` ファイルを完全に削除しています。`.so` (または `.dll` など) ファイルを読み込むだけで十分です。これらのファイルは常に `.la` ファイルと同じディレクトリにあります。

しかし、ビルドツリーにあるようなインストールされていない動的ライブラリの場合、状況は少し複雑です。ライブラリのビルドに libtool を使用するプロジェクト (Guile や autotools を使用するほとんどのプロジェクトがこれに該当します) があり、ディレクトリ D で foo.so をビルドすると、libtool は D に foo.la を配置しますが、foo.so は D/.libs に配置されます。

`libltdl`には、インストールされていないビルドツリーからでも`.la`ファイルを読み取って`.so`の場所を把握できる特別なロジックがあり、.libファイルの存在がユーザーに漏れるのを防ぐため、ユーザーはほとんどこの状況に気づいていませんでした。

柔軟性とエラー報告の観点から、現在は libltdl は使用していません。しかし、この古いユースケースが機能し続けるように、search-ltdl-library-path? が true の場合、`LTDL_LIBRARY_PATH` の各エントリをデフォルトの拡張機能ロードパスに追加し、さらに、.la ファイルの代わりに .so ファイルが存在する場合に備えて、各エントリの .libs サブディレクトリも追加します。

* * *

次へ: [外部ポインタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Pointers)、前: [外部ライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Libraries)、上: [外部関数インターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.19.2 外部拡張機能 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Extensions-1)

共有ライブラリを使用する方法の一つは、Guileを拡張することです。このようなロード可能なモジュールは通常、1つの特別な初期化関数を定義しており、この関数が呼び出されると、`libguile` APIを使用して現在のモジュール内のプロシージャを定義します。

具体的には、Guileにベッセル関数の実装である`j0`を追加することで拡張できます。

#include <math.h>
#include <libguile.h>

SCM
j0\_wrapper (SCM x)
{
return scm_from_double (j0 (scm_to_double (x, "j0")));
}

空所
init_math_bessel (void)
{
scm\_c\_define\_gsubr ("j0", 1, 0, 0, j0\_wrapper);
}

次に、C言語のソースファイルを共有ライブラリにコンパイルする必要があります。GNU/Linuxでは、コンパイラの呼び出しは次のようになります。

gcc -shared -o bessel.so -fPIC bessel.c

Guile を拡張する共有ライブラリを配置するのに最適なデフォルトの場所は、extensions ディレクトリです。コマンドラインまたはビルドスクリプトから、`pkg-config --variable=extensionsdir guile-3.0` を実行すると、extensions ディレクトリが表示されます。詳細については、[並列インストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Parallel-Installations) を参照してください。

Guileは`load-extension`を介して`bessel.so`をロードできます。

Scheme手順: **load-extension** lib init [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002dextension)

C 関数: **scm\_load\_extension** (lib、init) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fload_005fextension)

LIBとINITで指定された拡張機能をロードして初期化します。

拡張機能を使用する一般的な方法は、モジュールを定義する小さな Scheme ファイルを作成し、そのモジュールに拡張機能をロードすることです。モジュールが自動ロードされると、拡張機能もロードされます。例:

(define-module (math bessel)
#:export (j0))

([load-extension](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load_002dextension) "bessel" "init\_math\_bessel")

この `load-extension` 呼び出しは、`(load-foreign-library "bessel")` を介して `bessel` ライブラリをロードし、次にライブラリ内で `init_math_bessel` シンボルを検索し、それを引数なしの関数として扱い、その関数を呼び出します。

拡張機能を `load-foreign-library` のデフォルトの検索パス外に配置することにした場合、Scheme モジュールを修正して絶対パスを指定する必要があるでしょう。たとえば、`automake` を使用して拡張機能をビルドし、それを `$(pkglibdir)` に配置する場合、ビルドシステムによって作成される build-parameters モジュールを定義することができます。

(define-module (math config)
#:export (extensiondir))
(define extensiondir "PKGLIBDIR")

このファイルは`config.scm.in`になります。インストールされたファイル名の絶対値を代入する`make`ルールを定義します。

config.scm: config.scm.in
sed 's|PKGLIBDIR|$(pkglibdir)|' <$< >$

すると、`(math bessel)` は `(math config)` をインポートし、次に `(load-extension (in-vicinity extensiondir "bessel") "init_math_bessel")` を実行します。

別の方法としては、`guile-extensions-path` パラメータ、またはそれに対応する環境変数を再バインドする方法がありますが、これらのパラメータを変更すると、`load-foreign-library` の他のユーザーにも影響することに注意してください。

拡張機能が `scm_c_define_gsubr` ([プリミティブ手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Primitive-Procedures)を参照) またはその他のメカニズムを使用して Guile に追加する新しいプリミティブは、`scm_c_define_gsubr` が実行されたときにカレントなモジュールに配置されるため、何がどこに配置されるかを明確にするには、上記のように `load-extension` をモジュールに含めるのが最善です。あるいは、C コードで `scm_c_define_module` を使用して、作成するモジュールを指定することもできます。

static void
do_init (void \*unused)
{
scm\_c\_define\_gsubr ("j0", 1, 0, 0, j0\_wrapper);
scm\_c\_export ("j0", NULL);
}

空所
init_math_bessel()
{
scm_c_define_module ("math bessel", do_init, NULL);
}

しかし...もし私たちが単に `j0` 関数だけを求めているのであれば、Guile ユーザーがそれを呼び出せるように、初期化関数とラッパー モジュールを備えた Guile 専用のラッパー ライブラリをコンパイルする必要があるというのは、非常に面倒な手続きのように思えます。別の方法もありますが、そのためにはまず関数ポインタと関数型について説明しなければなりません。[外部関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Functions) を参照して、重要な部分だけを確認してください。

* * *

次へ: [外部型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Types)、前: [外部拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Extensions)、上: [外部関数インターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.19.3 外部ポインタ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Pointers-1)

外部ライブラリは基本的にキーと値のマッピングであり、キーは定義の名前、値はその定義のアドレスです。定義のアドレスを調べるには、`(system foreign-library)`モジュールの`foreign-library-pointer`を使用します。

Scheme Procedure: **foreign-library-pointer** ライブラリ名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-foreign_002dlibrary_002dpointer)

lib が参照する共有オブジェクト内のシンボル名に対応する「ラップドポインタ」を返します。返されるポインタは C オブジェクトを指します。

便宜上、lib が外部ライブラリでない場合は、`load-foreign-library` に渡されます。

先ほどの `bessel.so` の例を続けると、`init_math_bessel` 関数のアドレスは次のように取得できます。

(use-modules (system foreign-library))
(define init (foreign-library-pointer "bessel" "init\_math\_bessel"))
初期化
⇒ #<ポインタ 0x7fb35b1b4688>

`foreign-library-pointer` が返す値は、C ポインタの Scheme ラッパーです。ポインタはGuile におけるデータ型であり、他のすべての型とは区別されます。次のセクションではポインタの逆参照の方法について説明しますが、その前に、一般的な型述語などについて説明します。

なお、このセクションの残りのインターフェースは、`(system foreign)`ライブラリの一部です。

(use-modules (system foreign))

Scheme手順: **ポインタアドレス** ポインタ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pointer_002daddress)

C 関数: **scm\_pointer\_address** (ポインタ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fpointer_005faddress)

ポインタの数値を返します。

（ポインタアドレス初期化）
⇒ 139984413364296 ; 結果は人によって異なります

Scheme手順: **make-pointer** アドレス \[finalizer\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dpointer)

アドレスを指す外部ポインタオブジェクトを返します。ファイナライザが渡された場合は、ポインタオブジェクトが到達不能になったときに呼び出される、引数1つのC関数へのポインタである必要があります。

Scheme手順: **ポインタ?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pointer_003f)

objがポインタオブジェクトの場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme変数: **%null-pointer** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0025null_002dpointer)

値が0の外部ポインタ。

Scheme手順: **nullポインタ？** ポインタ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-null_002dpointer_003f)

ポインタがヌルポインタの場合は`#t`を返し、それ以外の場合は`#f`を返します。

SCM値を外部関数に直接渡したり、外部関数がSCM値を返したりできるようにするため、Guileは安全でない型変換演算子もいくつかサポートしています。

Scheme手順: **scm->pointer** scm [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002d_003epointer)

scmの`object-address`を持つ外部ポインタオブジェクトを返します。

Scheme手順: **pointer->scm** ポインタ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pointer_002d_003escm)

Schemeオブジェクトへのポインタを安全でない方法でキャストしました。幸運を祈りましょう！

C言語の拡張機能から動的FFIにアクセスできるようにしたい場合があります。その際、「ポインタ」という名称は、ポインタをラップする`SCM`オブジェクトを指す場合もあれば、`void*`値を指す場合もあるため、混乱を招くことがあります。そこで、Schemeオブジェクトを指す場合は「ポインタオブジェクト」、`void*`値を指す場合は「ポインタ値」という用語を使用することにします。

C 関数: `SCM` **scm\_from\_pointer** `(void *ptr, void (*finalizer) (void*))` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005fpointer)

ポインタ値からポインタオブジェクトを作成します。

ファイナライザがnullでない場合、Guileはポインタオブジェクトが収集可能になった後のいずれかの時点で、ポインタ値に対してファイナライザを呼び出すように手配します。

C 関数: `void*` **scm\_to\_pointer** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fto_005fpointer)

ポインタオブジェクトからポインタ値を解凍します。

* * *

次へ: [外部関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Functions)、前: [外部ポインタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Pointers)、上: [外部関数インターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.19.4 外部型 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Types-1)

Schemeの観点から見ると、外部ポインタは混沌の断片です。ユーザーは任意のアドレスに対して外部ポインタを作成し、それを自由に操作できます。全体に秩序感を与える唯一の要素は、特定の記憶場所に特定の型があるという共通の認識だけです。外部インターフェース用のSchemeラッパーを作成する際には、パラメータとフィールドのデータ型を明示的に表現することで、この混沌を隠蔽します。

これらの「外部型値」は、次のようにロードできる`(system foreign)`モジュールの定数とプロシージャを使用して構築できます。

(use-modules (system foreign))

`(system foreign)` は、基本的な C 型を表すいくつかの値をエクスポートします。

Scheme変数: **int8** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-int8)

Scheme変数: **uint8** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uint8)

Scheme変数: **uint16** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uint16)

Scheme変数: **int16** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-int16)

Scheme変数: **uint32** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uint32)

Scheme変数: **int32** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-int32)

Scheme変数: **uint64** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uint64)

Scheme変数: **int64** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-int64)

Scheme変数: **float** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-float)

Scheme変数: **double** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-double)

Scheme変数: **complex-double** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-complex_002ddouble)

Scheme変数: **complex-float** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-complex_002dfloat)

これらの値は、指定されたサイズと符号を持つC言語の数値型を表します。`complex-float`と`complex-double`は、それぞれC99の`float_Complex`と`double_Complex`を表します。

さらに、プラットフォームに依存するサイズの種類を示すための便利なバインディングもいくつか用意されています。

Scheme変数: **int** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-int)

Scheme変数: **unsigned-int** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unsigned_002dint)

Scheme変数: **long** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-long)

Scheme変数: **unsigned-long** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unsigned_002dlong)

Scheme変数: **short** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-short)

Scheme変数: **unsigned-short** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unsigned_002dshort)

スキーム変数: **size\_t** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-size_005ft-1)

スキーム変数: **ssize\_t** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ssize_005ft)

スキーム変数: **ptrdiff\_t** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ptrdiff_005ft)

Scheme変数: **intptr\_t** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-intptr_005ft)

Scheme変数: **uintptr\_t** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-uintptr_005ft)

`(system foreign)`モジュールによってエクスポートされる値で、C言語の数値型を表します。例えば、`long`は64ビットプラットフォームでは`int64`と`equal?`になる場合があります。

Scheme変数: **void** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-void)

`void`型。`pointer->procedure`の最初の引数として使用することで、何も返さないC関数をラップできます。

さらに、慣例として、ポインタ型を表す記号として「*」が使用されます。以下のセクションで詳述する「pointer->procedure」などのプロシージャは、これを型記述子として受け入れます。

* * *

次へ: [Void ポインタとバイト アクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Void-Pointers-and-Byte-Access)、前: [外部型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Types)、上: [外部関数インターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.19.5 外部関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Functions-1)

動的ライブラリを使う上で最も自然なことは、関数ポインタ、つまり_外部関数_を探すことです。これらのSchemeインターフェースを使用するには、`(system foreign)`モジュールをロードしてください。

(use-modules (system foreign))

Scheme プロシージャ: **pointer->procedure** return\_type func\_ptr arg\_types \[#:return-errno?=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pointer_002d_003eprocedure)

C 関数: **scm_pointer_to_procedure** (return_type, func_ptr, arg_types) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fpointer_005fto_005fprocedure)

C 関数: **scm\_pointer\_to\_procedure\_with\_errno** (return\_type, func\_ptr, arg\_types) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fpointer_005fto_005fprocedure_005fwith_005ferrno)

外部関数を作成する。

外部関数のvoidポインタfunc_ptr、その引数型arg_types、および戻り値型return_typeが与えられたとき、引数を外部関数に渡して適切な値を返す手続きを返してください。

`arg_types` は外部型のリストである必要があります。`return_type` も外部型である必要があります。外部型の詳細については、[Foreign Types](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Types) を参照してください。

return-errno? が true の場合、または `scm_pointer_to_procedure_with_errno` を呼び出す場合、返されるプロシージャは 2 つの値を返し、2 番目の値は `errno` になります。

最後に、`(system foreign-library)`には、`foreign-library-pointer`と`pointer->procedure`を結合する便利なラッパー関数があります。

Scheme 手順: **foreign-library-function** ライブラリ名 \[#:return-type=void\] \[#:arg-types='()\] \[#:return-errno?=#f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-foreign_002dlibrary_002dfunction)

lib から name のアドレスをロードし、引数 arg-types を受け取り、戻り値型を返す関数として扱います。オプションで errno も使用します。

`foreign-library-function` の呼び出しは、以下と完全に同等です。

（ポインター→プロシージャ戻り値型）
（外部ライブラリポインタ ライブラリ名）
引数の型
#:return-errno? return-errno?)。

これらをまとめると、`(数学ベッセル)`のより良い定義は次のようになります。

(define-module (math bessel)
#:use-module (system foreign)
#:use-module (system foreign-library)
#:export (j0))

(j0 を定義)
(外部ライブラリ関数 "libm" "j0"
#:戻り値の型 double
#:引数の型 (リスト double)))

以上です！Cは全くありません。

より詳細な例に進む前に、次の 2 つのセクションでは、たとえば `int8` よりも複雑なデータの扱い方について説明します。外部関数の例については、[More Foreign Functions](https://doc.guix.gnu.org/guile/latest/en/guile.html#More-Foreign-Functions) を参照してください。

* * *

次へ: [外部構造体](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Structs)、前: [外部関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Functions)、上: [外部関数インターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.19.6 ボイドポインタとバイトアクセス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Void-Pointers-and-Byte-Access-1)

ラップされたポインタは型指定されていないため、実質的には C 言語の `void` ポインタと同等です。C 言語と同様に、ポインタが指すメモリ領域にはバイトレベルでアクセスできます。これは _bytevectors_ を使用して実現されます ([Bytevectors](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) を参照)。`(rnrs bytevectors)` モジュールには、バイト シーケンスを文字列、浮動小数点数、整数などの Scheme オブジェクトに変換するために使用できるプロシージャが含まれています。

これらの Scheme インターフェースを使用するには、`(system foreign)` モジュールをロードしてください。

(use-modules (system foreign))

Scheme手順: **pointer->bytevector** pointer len \[offset \[uvec\_type\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pointer_002d_003ebytevector)

C 関数: **scm\_pointer\_to\_bytevector** (pointer, len, offset, uvec\_type) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fpointer_005fto_005fbytevector)

ポインタが指す len バイトをエイリアスしたバイトベクターを返します。

ユーザーは、uvec_type引数を渡すことで、メモリが指定された型の要素の配列であることを示す代替のデフォルト解釈を指定できます。uvec_typeには、`array-type`が返す値（`f32`や`s16`など）を指定する必要があります。

offset が渡された場合、それは返されるバイトベクターによってエイリアスされたメモリ領域のポインタからのバイト単位のオフセットを指定します。

返されたバイトベクターを変更すると、ポインタが指すメモリも変更されるため、シートベルトをしっかり締めてください。

Scheme手順: **bytevector->pointer** bv \[offset\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002d_003epointer)

C 関数: **scm\_bytevector\_to\_pointer** (bv, offset) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fto_005fpointer)

bv が指すメモリ領域へのエイリアスポインタを返すか、offset が渡された場合は bv の後の offset バイトを返す。

これらの基本機能に加えて、便利な手続きも利用可能です。

Scheme手順: **dereference-pointer** ポインタ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-dereference_002dpointer)

ポインタがポインタを保持するメモリ領域を指していると仮定して、そのポインタを返します。

Scheme手順: **string->pointer** string \[encoding\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003epointer)

指定されたエンコーディングでヌル終端された文字列のコピーへの外部ポインタを返します。デフォルトは現在のロケールエンコーディングです。返された外部ポインタが到達不能になった時点で、C文字列は解放されます。

これは、Scheme における `scm_to_stringn` に相当するものです。

Scheme手順: **pointer->string** pointer \[length\] \[encoding\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pointer_002d_003estring)

ポインタが指すC文字列を表す文字列を返します。lengthが省略されている場合、または`-1`が指定されている場合は、文字列はヌル終端されているとみなされます。それ以外の場合は、lengthはポインタが指すメモリ内のバイト数です。C文字列は指定されたエンコーディングであるとみなされ、デフォルトは現在のロケールエンコーディングになります。

これは、Schemeにおける`scm_from_stringn`に相当するものです。

ほとんどのオブジェクト指向Cライブラリは、特定のデータ構造へのポインタを使用してオブジェクトを識別します。このような場合、異なるポインタ型を互いに素なScheme型として具体化することが有効です。`define-wrapped-pointer-type`マクロはこの処理を簡素化します。

Scheme構文: **define-wrapped-pointer-type** type-name pred wrap unwrap print [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002dwrapped_002dpointer_002dtype)

ポインタオブジェクトを、型が異なるSchemeオブジェクトにラップするためのヘルパープロシージャを定義します。具体的には、このマクロは以下を定義します。

* pred、新しいScheme型の述語。
* wrap、ポインタオブジェクトを受け取り、predを満たすオブジェクトを返すプロシージャ。
* unwrapは、その逆の操作を行います。

wrap はポインタの同一性を保持します。`equal?` である 2 つのポインタ オブジェクト p1 と p2 については、`(eq? (wrap p1) (wrap p2)) ⇒ #t` となります。

最後に、print 関数には、そのようなオブジェクトを出力するためのユーザー定義プロシージャの名前を指定する必要があります。このプロシージャには、ラップされたオブジェクトと書き込み先のポートが渡されます。

例えば、型 `bottle_t` と、`bottle_t *` ポインタを渡して操作できる関数を定義する C ライブラリをラップしているとします。次のように記述できます。

(define-wrapped-pointer-type bottle
ボトル？
ボトルを包む ボトルを包まない
（ラムダ（bp））
(書式 p "#<ボトル ~a ~x>"
（ボトルの中身b）
(ポインタアドレス (アンラップボトル b)))))

(グラブボトルを定義する)
;; `bottle\_t *grab (void)` のラッパー。
(let ((grab (foreign-library-function libbottle "grab\_bottle"
#:戻り値の型 '\*)))
(ラムダ()
「新しいボトルを返却してください。」
（ラップボトル（掴む）））））

（ボトルの内容を定義する）
;; `const char \*bottle\_contents (bottle\_t \*)` のラッパー。
(let ((contents (foreign-library-function libbottle "bottle\_contents"
#:戻り値の型 '\*
#:arg-types '(\*))))
(ラムダ(b)
「Bの中身を返してください。」
(ポインタ->文字列 (内容 (unwrap-bottle b))))))

（（ボトルをつかむ）と書く）
⇒ #<シャトー・オー・ブリオン 803d36 ボトル>

この例では、`grab-bottle` は `bottle?` を満たす真の `bottle` オブジェクトを返すことが保証されています。同様に、`bottle-contents` は引数が真の `bottle` オブジェクトでない場合、エラーになります。

別の例として、現在GuileにはAPIの一部として`scm_numptob`という変数があります。これはC言語の`long`型として宣言されています。したがって、その値を読み取るには、次のようにします。

(use-modules (system foreign))
(use-modules (rnrs bytevectors))
(numptob を定義)
(外部ライブラリポインタ #f "scm\_numptob"))
numptob
(bytevector-uint-ref (pointer->bytevector numptob (sizeof long))
0（ネイティブエンディアン）
(long のサイズ)
⇒ 8

Guileの内部状態を破壊したい場合は、`scm_numptob`に別の値を設定できますが、この変数は設定することを想定していないため、そうすべきではありません。実際、この点はより広く当てはまります。C APIは危険な場所です。値を設定するとプログラムがクラッシュする可能性があるだけでなく、ダングリングポインタなどが指すデータにアクセスするだけでも、同様に深刻な問題を引き起こす可能性があります。

* * *

次へ: [その他の外部関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#More-Foreign-Functions)、前: [ボイドポインタとバイトアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Void-Pointers-and-Byte-Access)、上: [外部関数インターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.19.7 外部構造体 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Structs-1)

最後に、外部関数を呼び出す前に、外部値に関する最後の注意点を述べておきます。C言語の構造体を扱う必要がある場合があり、その場合は構造体の各要素を型、オフセット、アライメントに基づいて解釈する必要があります。`(system foreign)`モジュールには、これをサポートするプリミティブがいくつか用意されています。

(use-modules (system foreign))

Scheme プロシージャ: **sizeof** 型 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sizeof)

C 関数: **scm_sizeof** (型) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsizeof)

型のサイズをバイト単位で返します。

type は、`int` のような有効な C 型である必要があります。あるいは、type はシンボル `*` でも構いません。その場合は、ポインタのサイズが返されます。type は型のリストでも構いません。その場合は、ABI に準拠したパッキングによる `struct` のサイズが返されます。

Scheme プロシージャ: **alignof** 型[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alignof)

C 関数: **scm\_alignof** (型) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005falignof)

型の配置をバイト単位で返します。

type は、`int` のような有効な C 型である必要があります。あるいは、type はシンボル `*` でも構いません。その場合は、ポインタのアライメントが返されます。type は型のリストでも構いません。その場合は、ABI に準拠したパッキングによる `struct` のアライメントが返されます。

Guileは、C言語の構造体とバイトベクトルを効率的に読み書きするための便利な構文も提供しています。

Scheme構文: **read-c-struct** bv offset
((フィールドタイプ) …) k [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002dc_002dstruct)

バイトベクター bv のオフセット offset から、型 type... のフィールドを持つ C 構造体を読み込みます。フィールドを識別子 field... にバインドし、`(k field ...)` を返します。

クロスコンパイルを行わない限り、フィールド型はマクロ展開時に評価されます。これにより、結果として得られるバイトベクトルアクセサとサイズ／アライメント計算を完全にインライン化できます。

Scheme構文: **write-c-struct** bv offset
((フィールドタイプ) …) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-write_002dc_002dstruct)

型 type... のフィールド field... を持つ C 構造体を、バイトベクトル bv のオフセット offset に書き込みます。ゼロ値を返します。

上記の`write-c-struct`と同様に、クロスコンパイルでない限り、フィールド型はマクロ展開時に評価されます。

例えば、`struct { int64_t a; uint8_t b; }` に相当するもののパーサーとシリアライザを定義するには、次のようにします。

(use-modules (system foreign) (rnrs bytevectors))

(構文ルールの定義)
(define-serialization (reader writer) (field type) ...)
（始める
(define (reader bv offset)
(read-c-struct bv offset ((field type) ...) values))
(define (writer bv offset field ...)
(write-c-struct bv offset ((field type) ...)))))

(define-serialization (read-struct write-struct)
(a int64) (b uint8)

(define bv (make-bytevector (sizeof (list int64 uint8))))

(write-struct bv 0 300 43)
(call-with-values (lambda () (read-struct bv 0))
リスト）
⇒ (300 43)

また、`read-c-struct` および `write-c-struct` とほぼ同等の古いインターフェースも存在しますが、こちらは実行時ディスパッチを使用し、バイトベクターではなく外部ポインタを操作します。

Scheme手順: **parse-c-struct** 外部型 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-parse_002dc_002dstruct)

C言語の構造体への外部ポインタを解析し、値のリストを返します。

`types`はC言語の型のリストである必要があります。

`struct { int64_t a; uint8_t b; }` のパーサーとシリアライザーの例は、次のようになります。

(parse-c-struct (make-c-struct (list int64 uint8)
(リスト300 43)
(int64 uint8 のリスト)
⇒ (300 43)

現時点では、Guileには従来型の構造体をサポートする便利なルーチンしかありません。しかし、`bytevector->pointer`と`pointer->bytevector`ルーチンを使用すれば、密にパックされた構造体や共用体を手動で作成および解析できます。詳細は`(system foreign)`のコードを参照してください。

* * *

前へ: [外部構造体](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Structs)、上へ: [外部関数インターフェース](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.19.8 その他の外部関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#More-Foreign-Functions-1)

外部関数へのポインタを渡したり、ポインタを返したりすることが可能です。その場合、引数または戻り値の型はポインタを示す記号「*」にする必要があります。例えば、次のコードは`memcpy`をSchemeで使用できるようにします。

(use-modules (system foreign))
(define memcpy
(外部ライブラリ関数 #f "memcpy"
#:戻り値の型 '\*
#:arg-types (list '\* '\* size\_t)))

`memcpy`を呼び出すには、外部ポインタを渡す必要があります。

(use-modules (rnrs bytevectors))

(src-bits を定義)
(u8-list->bytevector '(0 1 2 3 4 5 6 7)))
(define src
(バイトベクトル->ポインタ src-bits)
(define dest
(bytevector->pointer (make-bytevector 16 0)))

(memcpy dest src (bytevector-length src-bits))

(bytevector->u8-list (pointer->bytevector dest 16))
⇒ (0 1 2 3 4 5 6 7 0 0 0 0 0 0 0 0)

構造体を値として渡すことも、外部ポインタとして渡すこともできます。構造体の型と値の表現方法の詳細については、[外部構造体](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Structs)を参照してください。

「 Out」引数は外部ポインタとして渡されます。外部ポインタが指すメモリ領域は、その場で変更されます。

;; struct timeval {
;; time\_t tv\_sec; /\* 秒 \*/
;; suseconds\_t tv\_usec; /\* マイクロ秒 \*/
;; };
;; フィールドが「long」型であると仮定します

(gettimeofday を定義)
(let ((f (foreign\-library\-function #f "gettimeofday"
#:戻り値の型 int
#:arg-types (list '\* '\*)))
(テレビタイプ (リストロングロング)))
(ラムダ()
(let\* ((timeval (make-c-struct tv-type (list 0 0)))
(ret (f timeval %null-pointer)))
(もし (ゼロ? ret) の場合)
(値を適用する (parse-c-struct timeval tv-type))
(エラー「gettimeofday がエラーを返しました」ret))))))

(gettimeofday)
⇒ 1270587589
⇒ 499553

ご覧のとおり、外部関数へのこのインターフェースは非常に低レベルで、やや危険なレベルにあります[22](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT22)。

FFIは逆方向にも機能し、Schemeの手続きをC言語から呼び出し可能にすることができます。これにより、Schemeの手続きをC言語の関数が期待する「コールバック」として使用することが可能になります。

Scheme プロシージャ: **procedure->pointer** 戻り値の型 proc 引数の型 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_002d_003epointer)

C 関数: **scm\_procedure\_to\_pointer** (return\_type, proc, arg\_types) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprocedure_005fto_005fpointer)

引数の型が arg-types (リスト) で、プロシージャ proc のプロキシとして機能する、戻り値の型が return-type の C 関数へのポインタを返します。したがって、proc の引数の数、サポートされる引数の型、および戻り値の型は、return-type および arg-types と一致する必要があります。

例として、Cライブラリの配列ソート関数`qsort`をSchemeからアクセスできるようにする方法を以下に示します（GNU Cライブラリリファレンスマニュアルの[`qsort`](https://doc.guix.gnu.org/libc/latest/en/libc.html#Array-Sort-Function)を参照）。

(qsort を定義します!
(let ((qsort (foreign-library-function
#f "qsort" #:arg-types (list '\* size\_t size\_t '\*))))
(ラムダ (bv比較)
;; 比較に基づいてバイトベクトル BV をその場でソートする
;; プロシージャ COMPARE。
(let ((ptr (procedure->pointer int
(ラムダ (xy)
;; XとYはポインタなので、
;; 便宜上、逆参照
;; COMPARE を呼び出す前にそれらを実行します。
(比較 (dereference-uint8\* x)
(参照解除-uint8\* y)))
(リスト '\* '\*))))
(qsort (bytevector->pointer bv)
(バイトベクトル長 bv) 1 ;; バイトをソートしています
ptr)))))

(define (dereference-uint8\* ptr)
;; ヘルパー関数: PTR が指すバイトを逆参照します。
(let ((b (pointer->bytevector ptr 1)))
(バイトベクトル-u8-ref b 0)))

(bv を定義)
;; ソートされていないバイト配列。
(u8-list->bytevector '(7 1 127 3 5 4 77 2 9 0)))

;; BVをソートします。
(qsort! bv (lambda (xy) (- xy)))

;; ソートされた配列がどのようなものか見てみましょう。
(バイトベクター->u8リスト bv)
⇒ (0 1 2 3 4 5 7 9 77 127)

そして、ほら！

`procedure->pointer` は、一部の特殊なアーキテクチャではサポートされておらず（定義もされていません）、そのため、ユーザーコードでは `(defined? 'procedure->pointer)` を確認する必要がある場合があります。ただし、libffi 3.0.9 の時点では、x86、ia64、SPARC、PowerPC、ARM、MIPS など、多くのアーキテクチャで利用可能です。

* * *

次へ: [Smobs](https://doc.guix.gnu.org/guile/latest/en/guile.html#Smobs)、前: [Foreign Function Interface](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Function-Interface)、上: [API Reference](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]
