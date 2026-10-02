4 Scheme でのプログラミング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme-1)
--------------------------------------------------------------------------------------------------------

Guileのコア言語はSchemeであり、C言語のコードを深く掘り下げる必要なく、Guileを使ってSchemeプログラムを記述・実行するだけで多くのことが実現できます。このマニュアルのこの部分では、このモードでGuileを使用する方法、およびスクリプトの作成、デバッグ、配布用のプログラムのパッケージ化を支援するGuileが提供するツールについて説明します。

Guileのアプリケーションプログラミングインターフェース（API）を構成する変数、関数などに関する詳細なリファレンス情報については、[APIリファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference)を参照してください。

* [GuileによるSchemeの実装](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scheme)
* [Guileの呼び出し](https://doc.guix.gnu.org/guile/latest/en/guile.html#Invoking-Guile)
* [Guile スクリプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting)
* [Guileを対話的に使用する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively)
* [EmacsでGuileを使用する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-in-Emacs)
* [Guile Tools の使用方法](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Tools)
* [サイトパッケージのインストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Installing-Site-Packages)
* [Guileコードの配布](https://doc.guix.gnu.org/guile/latest/en/guile.html#Distributing-Guile-Code)

* * *

次へ: [Guile の呼び出し](https://doc.guix.gnu.org/guile/latest/en/guile.html#Invoking- Guile)、上へ: [Scheme でのプログラミング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 4.1 Guile による Scheme の実装 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile_0027s-Implementation-of-Scheme)

Guile のコア言語は Scheme であり、これは _RnRS_ と呼ばれる一連のレポートで規定および説明されています。_RnRS_ は _Revised^n Report on the Algorithmic Language Scheme_ の略称です。Guile は R5RS に完全に準拠しており (R5RS の [Introduction](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Top) を参照)、R6RS および R7RS にもほぼ準拠しています。

Guileには、これらのレポート以外にも多くの拡張機能があります。Guileが標準のSchemeを拡張している分野には、以下のようなものがあります。

* Guileのインタラクティブなドキュメントシステム
* GuileによるPOSIX準拠のネットワークプログラミングのサポート
* GOOPS – Guileが開発したオブジェクト指向プログラミングのためのフレームワーク。

* * *

次へ: [Guile スクリプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting)、前: [Guile による Scheme の実装](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scheme)、上: [Scheme でのプログラミング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 4.2 Guile の呼び出し [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Invoking-Guile-1)

Guileの多くの機能は、ユーザーがGuileの起動前または起動時に提供する情報に依存しており、またその情報によって変更される可能性があります。以下に、提供すべき情報とその提供方法について説明します。

* [コマンドラインオプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command_002dline-Options)
* [環境変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Environment-Variables)

* * *

次へ: [環境変数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Environment-Variables)、上: [Guile の呼び出し](https://doc.guix.gnu.org/guile/latest/en/guile.html#Invoking-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.2.1 コマンドラインオプション [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command_002dline-Options-1)

ここでは、Guile のコマンドライン処理について詳しく説明します。Guile は引数を左から右に処理し、以下に説明するスイッチを認識します。例については、[スクリプトの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scripting-Examples) を参照してください。

`script arg...` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-script-mode)

`-s スクリプト引数...`

デフォルトでは、Guile はコマンドラインで指定されたファイルをスクリプトとして読み込みます。script に続くコマンドライン引数 arg... は、スクリプトの引数になります。`command-line`関数は、`(script arg...)` の形式の文字列のリストを返します。

ファイル名の先頭にハイフンを付けることも可能です。例えば、\-myfile.scm のように指定します。この場合、ファイル名の前に \-s を付けて、Guile に（スクリプト）ファイルとして指定されていることを知らせる必要があります。

スクリプトは、`load`関数と同様にSchemeソースコードとして読み込まれ、評価されます。スクリプトの読み込みが完了すると、Guileは終了します。

`-c expr arg...` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-evaluate-expression_002c-command_002dline-argument)

expr を Scheme コードとして評価し、終了します。expr に続くコマンドライン引数 arg... はコマンドライン引数になります。`command-line` 関数は、`(guile arg...)` の形式の文字列のリストを返します。ここで、guile は Guile 実行可能ファイルのパスです。

`-- 引数...`

対話型で実行し、ユーザーに式を入力するように促して評価します。 \-- の後に続くコマンドライン引数 arg... は、対話型セッションのコマンドライン引数になります。`command-line` 関数は、`(guile arg...)` の形式の文字列のリストを返します。ここで、guile は Guile 実行可能ファイルのパスです。

`-L ディレクトリ`

Guileのモジュールロードパスの先頭にディレクトリを追加します。指定されたディレクトリは、コマンドラインで指定された順序で、環境変数`GUILE_LOAD_PATH`に指定されたディレクトリよりも前に検索されます。ここで追加されたパスは、ユーザーの.guileファイルの実行時には有効になりません。

`-C ディレクトリ`

-L と同様ですが、コンパイル済みファイルの読み込みパスを調整します。

`-x 拡張機能`

Guile のロード拡張機能リストの先頭に拡張機能を追加します ([`%load- extensions`](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths) を参照)。指定された拡張機能は、コマンドラインで指定された順序で、デフォルトのロード拡張機能よりも前に試行されます。ここで追加された拡張機能は、ユーザーの .guile ファイルの実行中は有効になりません。

`-l ファイル`

ファイルからSchemeのソースコードを読み込み、コマンドラインの処理を続行します。

`-e function`

関数をスクリプトの_エントリポイント_にします。スクリプトファイルをロードした後（-s オプションを使用）、または式を評価した後（-c オプションを使用）、プログラム名とコマンドライン引数を含むリスト（`command-line` 関数によって提供されるリスト）に関数を適用します。

-e スイッチは引数リストのどこにでも指定できますが、Guile は常にその関数を最後に実行します。これは奇妙に思えるかもしれませんが、POSIX におけるスクリプト呼び出しの仕組み上、-s オプションは常にリストの最後に指定する必要があります。

関数は、多くの場合、スクリプト内で定義されている関数の名前を表す単純なシンボルです。また、`(@ モジュール名 シンボル)` の形式にすることもできます。その場合、シンボルは module-name という名前のモジュール内で検索されます。

省略記法として、`(symbol ...)` という形式を使用できます。これは、`@` で始まらないシンボルのみのリストです。これは、`(@ module-name main)` と同等です。ここで、`module-name` は `(symbol ...)` 形式です。[Guile モジュールの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Modules) および [スクリプトの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scripting-Examples) を参照してください。

`-ds`

最後に \-s オプションが指定された場合は、コマンドラインのこの位置で指定されたものとして扱います。ここでスクリプトを読み込みます。

このスイッチが必要な理由は、POSIX スクリプト呼び出しメカニズムでは-s オプションが最後に指定される必要があるものの、プログラマーはコマンドラインで要求された他のアクションよりも前にスクリプトを実行したい場合があるためです。例については、[スクリプトの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scripting-Examples) を参照してください。

`\`

スクリプトファイルの2行目から始まるコマンドライン引数の詳細については、[メタスイッチ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Meta-Switch)を参照してください。

`--use-srfi=list` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-loading-srfi-modules-_0028command-line_0029)

オプション \--use-srfi は、カンマ区切りの数値リストを想定しています。各数値は、スクリプト ファイルの評価または REPL の起動前にインタープリタにロードされる SRFI モジュールを表します。さらに、このオプションを使用すると、ロードされた SRFI の機能識別子がプロシージャ `cond-expand` によって認識されます。

以下は、GUILEインタープリタの起動前にモジュールSRFI-8（「receive」）とSRFI-13（「string library」）をロードする例です。

guile --use-srfi=8,13

`--r6rs` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-r6rs-_0028command-line_0029)

Guileの初期環境をR6RSのサポートを向上させるように調整します。注意点については、[R6RSとの非互換性](https://doc.guix.gnu.org/guile/latest/en/guile.html#R6RS-Incompatibilities)を参照してください。

`--r7rs` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-r7rs-_0028command-line_0029)

Guileの初期環境をR7RSをより適切にサポートするように調整します。注意点については、[R7RSとの非互換性](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Incompatibilities)を参照してください。

`--debug` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-debugging-virtual-machine-_0028command-line_0029)

デバッグ用仮想マシン (VM) エンジンから始めましょう。デバッグ用 VM を使用すると、トレース、ブレークポイント、プロファイリング時の正確な呼び出し回数に必要な VM フックがサポートされます。ただし、デバッグ用 VM は通常の VM より約 10% 遅くなります。詳細については、[VM フック](https://doc.guix.gnu.org/guile/latest/en/guile.html#VM-Hooks) を参照してください。

デフォルトでは、デバッグ用VMエンジンは対話型セッションに入るときにのみ使用されます。-sまたは-cオプションを指定してスクリプトを実行する場合は、デフォルトで通常の高速なVMが使用されます。

`--no-debug` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-debugging-virtual-machine-_0028command-line_0029-1)

対話型セッションに入る場合でも、デバッグ用VMエンジンを使用しないでください。

名前とは裏腹に、Guile を --no-debug オプションで実行した場合でも、エラー発生時に詳細なバックトレースを出力するなど、通常のデバッグ機能はサポートされます。--debug オプションとの唯一の違いは、VM フックとその関連機能がサポートされない点です（上記参照）。

`-q` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-init-file_002c-not-loading)

初期化ファイル .guile をロードしません。このオプションは対話的に実行する場合にのみ有効です。スクリプトを実行する場合は .guile ファイルはロードされません。[初期化ファイル ~/.guile](https://doc.guix.gnu.org/guile/latest/en/guile.html#Init-File) を参照してください。

`--listen[=p]`

このプログラムの実行中は、REPLクライアントのためにローカルポートまたはパスで待機します。pが数字で始まる場合は、待機するローカルポートとみなされます。スラッシュで始まる場合は、待機するUNIXドメインソケットのファイル名とみなされます。

p が指定されていない場合、デフォルトはローカルポート 37146 です。逆さまに見ると、ほぼ「Guile」と読めます。netcat がインストールされている場合は、nc localhost 37146 を実行すると Guile プロンプトが表示されます。または、Emacs を起動してプロセスに接続することもできます。詳細については、[Emacs での Guile の使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-in-Emacs) を参照してください。

> **注:** ポートを開くと、そのポートに接続できるユーザーは誰でも、Guile プロセスが実行されているユーザーとして Guile ができることすべてを実行できるようになります。マルチユーザー マシンでは \--listen を使用しないでください。もちろん、Guile に \--listen を渡さなければ、ポートは開かれません。
>
Guileは、[HTTPプロトコル間エクスプロイト攻撃](https://en.wikipedia.org/wiki/Inter-protocol_exploitation)から保護します。これは、攻撃者がHTMLページを介して、Webブラウザにループバックインターフェースまたはプライベートネットワークで待機しているTCPサーバーにデータを送信させるシナリオです。ただし、可能な限り`--listen=/some/local/file`のようにUNIXドメインソケットを使用することをお勧めします。

とはいえ、--listen は対話型のデバッグや開発には非常に便利です。

`--statprof=style`

statprof プロファイラ内でスクリプトや式を実行します。結果をスタイル付きで表示します。STYLE の指定可能な値については、[Statprof](https://doc.guix.gnu.org/guile/latest/en/guile.html#Statprof) を参照してください。REPL 内では、[Profile Commands](https://doc.guix.gnu.org/guile/latest/en/guile.html#Profile-Commands) を使用してください。

`--auto-compile`

ソースファイルを自動的にコンパイルします（デフォルトの動作）。

`--fresh-auto-compile`

自動コンパイルキャッシュを無効として扱い、再コンパイルを強制します。

`--no-auto-compile`

ソースファイルの自動コンパイルを無効にします。

`--language=lang`

コマンドライン引数の残りの部分については、`-l` で指定されたファイルと `-c` で渡された式は lang で記述されているものとみなします。lang は、コンパイラがサポートする言語のいずれかの名前である必要があります ([コンパイラ タワー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiler-Tower) を参照)。対話的に実行する場合は、REPL の言語を lang に設定してください ([Guile の対話的使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively) を参照)。

デフォルトの言語は`scheme`です。その他の興味深い値としては、`elisp`（Emacs Lisp用）や`ecmascript`などがあります。

以下の例は、Scheme、Emacs Lisp、およびECMAScriptにおける式の評価を示しています。

guile -c "(apply + '(1 2))"
guile --language\=elisp -c "(= (funcall (symbol-function '+) 1 2) 3)"
guile --language\=ecmascript -c '(function (x) { return x \* x; })(2);'

Schemeで書かれたファイルとEmacs Lispで書かれたファイルを読み込み、Scheme REPLを起動するには、次のように入力します。

guile -l foo.scm --language=elisp -l foo.el --language=scheme

`-h, --help`

Guileの起動方法に関するヘルプを表示してから終了します。

`-v, --version`

Guileの現在のバージョンを表示し、終了します。

* * *

前へ: [コマンドラインオプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command_002dline-Options)、上へ: [Guile の起動](https://doc.guix.gnu.org/guile/latest/en/guile.html#Invoking-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.2.2 環境変数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Environment-Variables-1)

_環境_はオペレーティングシステムの機能であり、名前と値を持つ変数の集合で構成されます。各変数は_環境変数_（または「シェル変数」）と呼ばれ、環境変数名は大文字と小文字を区別し、慣例としてすべて大文字を使用します。値はすべてテキスト文字列であり、数値として記述されているものも同様です。（ここで言及しているのは、Guileが呼び出されるオペレーティングシステムのシェルで定義されている名前と値です。これは、実行中のGuileインスタンス内で定義されているScheme環境とは異なります。Scheme環境の説明については、[名前、場所、値、および環境](https://doc.guix.gnu.org/guile/latest/en/guile.html#About-Environments)を参照してください。）

Guile を起動する前に環境変数を設定する方法は、オペレーティングシステム、特に使用しているシェルによって異なります。たとえば、Bash を使用して `GUILE_WARN_DEPRECATED` を設定することで、Guile に非推奨機能に関する詳細な警告メッセージを表示するように指示する方法は次のとおりです。

$ export GUILE\_WARN\_DEPRECATED="detailed"
ガイル

または、以下の方法で単一の呼び出しに対して詳細な警告を有効にすることもできます。

$ env GUILE\_WARN\_DEPRECATED="detailed" guile

Guile の実行中のインスタンス内から、Guile の実行時動作に影響を与えるシェル環境変数の値を取得または変更する場合は、[ランタイム環境](https://doc.guix.gnu.org/guile/latest/en/guile.html#Runtime-Environment) を参照してください。

Guileの実行時動作に影響を与える環境変数は以下のとおりです。

`GUILE_AUTO_COMPILE` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-GUILE_005fAUTO_005fCOMPILE)

これは、Guile に対して Scheme ソースファイルを自動的にコンパイルするかどうかを指示するために使用できるフラグです。Guile 2.0 以降では、Scheme ソースファイルはデフォルトで自動的にコンパイルされます。

.scm ファイルに対応するコンパイル済み (.go) ファイルが見つからない場合、または .scm ファイルよりも新しいバージョンでない場合、.scm ファイルは実行時にコンパイルされ、生成された .go ファイルが保存されます。コンソールに警告メッセージが表示されます。

コンパイルされたファイルは、$XDG\_CACHE\_HOME/guile/ccache ディレクトリに保存されます。ここで、`XDG_CACHE_HOME` のデフォルト値は $HOME/.cache ディレクトリです。このディレクトリが存在しない場合は作成されます。

この仕組みは、.go ファイルのタイムスタンプが .scm ファイルのタイムスタンプよりも新しいことを前提としています。インストール後に .scm または .go ファイルを移動する場合は、元のタイムスタンプが保持されるように注意してください。

Scheme ファイルの自動コンパイルを防止するには、`GUILE_AUTO_COMPILE` をゼロ (0) に設定します。コンパイル済みのファイルよりも新しいかどうかに関わらず、Scheme ファイルをコンパイルするように Guile に指示するには、この変数を「fresh」に設定します。

[Schemeコードのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compilation)を参照してください。

`GUILE_HISTORY` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-GUILE_005fHISTORY-1)

この変数は、Guile REPLコマンドの履歴を格納するファイルの名前を指定します。この環境変数を設定することで、別の履歴ファイルを指定できます。デフォルトでは、履歴ファイルは$HOME/.guile\_historyです。

`GUILE_INSTALL_LOCALE` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-GUILE_005fINSTALL_005fLOCALE)

これは、`(setlocale LC_ALL "")`[3](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT3) を呼び出すことで、起動時に現在のロケールをインストールするかどうかを Guile に伝えるために使用できるフラグです。ロケールの詳細については、[Locales](https://doc.guix.gnu.org/guile/latest/en/guile.html#Locales) を参照してください。

`GUILE_INSTALL_LOCALE`を`0`に設定することで、ロケールをインストールしないことを明示的に示すことができます。また、この変数を`1`に設定することで、ロケールを明示的に有効にすることもできます。

通常、現在のロケールをインストールするのが正しい方法です。これにより、Guileは非ASCII文字を含む文字列を正しく解析して出力できるようになります。そのため、このオプションはデフォルトで有効になっています。

`GUILE_LOAD_COMPILED_PATH` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-GUILE_005fLOAD_005fCOMPILED_005fPATH)

この変数は、コンパイル済みの Scheme ファイル (.go ファイル) をロードする際に検索されるパスを拡張するために使用できます。その値は、コロンで区切られたディレクトリのリストである必要があります。特別なパス要素 `...` (省略記号) が含まれている場合は、省略記号の代わりにデフォルト パスが挿入され、そうでない場合はデフォルト パスが末尾に追加されます。結果は `%load-compiled-path` に格納されます ([Load Paths](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths) を参照)。

以下は、Bashシェルを使用して現在のディレクトリ「.」と相対ディレクトリ「../my-library」を`%load-compiled-path`に追加する例です。

$ export GUILE\_LOAD\_COMPILED\_PATH=".:../my-library"
$ guile -c '(display %load-compiled-path) (newline)'
(. ../my-library /usr/local/lib/guile/3.0/ccache)

`GUILE_LOAD_PATH` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-GUILE_005fLOAD_005fPATH)

この変数は、Scheme ファイルの読み込み時に検索されるパスを拡張するために使用できます。その値は、コロンで区切られたディレクトリのリストである必要があります。特別なパス要素 `...` (省略記号) が含まれている場合は、省略記号の代わりにデフォルト パスが挿入され、そうでない場合はデフォルト パスが末尾に追加されます。結果は `%load-path` に格納されます ([Load Paths](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths) を参照)。

以下は、Bashシェルを使用して現在のディレクトリを`%load-path`の先頭に追加し、相対ディレクトリ../srfiを末尾に追加する例です。

$ env GUILE\_LOAD\_PATH=".:...:../srfi" \\
guile -c '(display %load-path) (newline)'
(. /usr/local/share/guile/3.0 \\
/usr/local/share/guile/site/3.0 \\
/usr/local/share/guile/site \\
/usr/local/share/guile \\
../srfi)

（注：上記の改行は説明のためのものであり、実際の例では必須ではありません。）

`GUILE_EXTENSIONS_PATH` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-GUILE_005fEXTENSIONS_005fPATH)

この変数は、`load-extension`、`dynamic-link`、`load-foreign-library`などを介して外部ライブラリを検索するパスを拡張するために使用できます。その値は、コロン（Windowsではセミコロン）で区切られたディレクトリのリストである必要があります。[外部ライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Libraries)を参照してください。

`GUILE_WARN_DEPRECATED` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- GUILE_005fWARN_005fDEPRECATED)

Guile の進化に伴い、一部の機能は削除されるか、新しい機能に置き換えられます。この進化に合わせてユーザーがコードを移行できるように、Guile は最終的に削除される予定の機能を使用しているコードについて警告メッセージを表示します。`GUILE_WARN_DEPRECATED` を「no」に設定すると、Guile はこれらの警告メッセージを表示しません。また、「detailed」に設定すると、Guile は警告をより詳細に説明するメッセージを表示します。[非推奨](https://doc.guix.gnu.org/guile/latest/en/guile.html#Deprecation) を参照してください。

`HOME` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-HOME)

Guile は、環境変数 `HOME`（ホーム ディレクトリの名前）を使用して、.guile や .guile\_history などのさまざまなファイルを見つけます。

`GUILE_JIT_THRESHOLD` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-GUILE_005fJIT_005fTHRESHOLD)

Guile には、Guile コードの実行を高速化するジャストインタイム (JIT) コードジェネレータがあります。詳細については、[Just-In-Time Native Code](https://doc.guix.gnu.org/guile/latest/en/guile.html#Just_002dIn_002dTime-Native-Code) を参照してください。コード生成の単位は関数です。各関数には独自のカウンタがあり、関数が呼び出されたときと関数内のループの各イテレーションでインクリメントされます。カウンタが `GUILE_JIT_THRESHOLD` を超えると、関数は JIT コンパイルされます。JIT コンパイルを無効にするには `GUILE_JIT_THRESHOLD` を `-1` に設定し、関数が最初に見つかったときにすぐに JIT コンパイルするには `0` に設定します。

`GUILE_JIT_LOG` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-GUILE_005fJIT_005fLOG)

JITコンパイルイベントのログ出力レベルを段階的に上げるには、`1`、`2`、または`3`に設定してください。デバッグに使用します。

`GUILE_JIT_STOP_AFTER` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- GUILE_005fJIT_005fSTOP_005fAFTER)

JITコンパイラは可能な限りテストを行いましたが、バグが存在する可能性はあります。GuileのJITコンパイラが原因でプログラムが失敗していると思われる場合は、`GUILE_JIT_STOP_AFTER`にJITコンパイルする関数の最大数を示す正の整数を設定してください。`GUILE_JIT_STOP_AFTER`の値に基づいて二分探索を行うことで、コンパイルエラーが発生している関数を特定できます。

`GUILE_JIT_PAUSE_WHEN_STOPPING` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-GUILE_005fJIT_005fPAUSE_005fWHEN_005fSTOPPING)

JITコンパイラのデバッグでは、実行中のプロセスを分析する必要がある場合があります。`GUILE_JIT_PAUSE_WHEN_STOPPING`を設定すると、JITが停止したときにプロセスが一時停止し、デバッガを接続できるようになります。また、次のようなメッセージが表示されます。

要求どおり、自動JITコンパイルを停止します。
30秒間スリープします。デバッグするには：
gdb -p 133646

* * *

次へ: [Guile を対話的に使用する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively)、前: [Guile を呼び出す](https://doc.guix.gnu.org/guile/latest/en/guile.html#Invoking-Guile)、上: [Scheme でのプログラミング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 4.3 Guile スクリプト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting-1)

AWK、Perl、あるいは他のシェルと同様に、Guileはスクリプトファイルを解釈できます。Guileスクリプトとは、Schemeコードのファイルであり、冒頭にGuileの起動方法をオペレーティングシステムに指示する追加情報と、GuileがSchemeコードを処理する方法を指示する情報が記述されています。

* [スクリプトファイルの先頭](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Top-of-a-Script-File)
* [メタスイッチ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Meta-Switch)
* [コマンドライン処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command-Line-Handling)
* [スクリプトの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scripting-Examples)

* * *

次へ: [メタスイッチ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Meta-Switch)、上: [Guile スクリプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.3.1 スクリプトファイルの先頭 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Top-of-a-Script-File-1)

Guileスクリプトの最初の行では、オペレーティングシステムに対してスクリプトの評価にGuileを使用するように指示し、次にGuileに対してその方法を指示する必要があります。最も単純な例を以下に示します。

* ファイルの最初の 2 文字は「#!」でなければなりません。
    
オペレーティングシステムは、この行の残りの部分がスクリプトを解釈できる実行可能ファイルの名前であると解釈します。しかし、Guileはこれらの文字を複数行コメントの開始と解釈し、単独の行にある「!#」文字で終了します。（これはR5RSで説明されている構文の拡張であり、シェルスクリプトをサポートするために追加されました。）
    
* その2文字の直後に、Guileインタープリタへの完全なパス名を指定する必要があります。ほとんどのシステムでは、これは「/usr/local/bin/guile」になります。
* 次にスペースを挟み、Guile に渡すコマンドライン引数を記述する必要があります。これは '\-s' です。このスイッチは、端末からの入力をユーザーに求める代わりに、スクリプトを実行するように Guile に指示します。ここではさらに複雑な操作も可能です。[メタスイッチ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Meta-Switch) を参照してください。
* この後に改行を入れてください。
* スクリプトの2行目には、ファイルの先頭と同じように「!#」という文字のみが含まれている必要がありますが、文字の並び順は逆になります。オペレーティングシステムはここまで読み込まないため、Guileはこの行を、1行目の「#!」文字で始まるコメントの終わりとして扱います。
* このソースコードファイルがASCIIまたはISO-8859-1でエンコードされていない場合は、`coding: utf-8`などのコーディング宣言をファイルの最初の5行以内のコメントに記述する必要があります。詳細は[ソースファイルの文字エンコーディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Encoding-of-Source-Files)を参照してください。
* ファイルの残りの部分はSchemeプログラムである必要があります。

Guileはプログラムを読み込み、式が現れる順序で評価します。ファイルの終わりに達すると、Guileは終了します。

* * *

次へ: [コマンドライン処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command-Line-Handling)、前: [スクリプトファイルの先頭](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Top-of-a-Script-File)、上: [Guile スクリプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.3.2 メタスイッチ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Meta-Switch-1)

Guileのコマンドラインスイッチを使用すると、プログラマはスクリプト内で比較的複雑なアクションを記述できます。しかし、POSIXスクリプト呼び出しメカニズムでは、Guile実行可能ファイルへのパスの後の「#!」行に指定できる引数は1つだけであり、その引数の長さにも任意の制限が課せられています。例えば、次のようなスクリプトを書いたとします。

#!/usr/local/bin/guile -e main -s
!#
(define (main args)
(map (lambda (arg) (display arg) (display " "))
(cdr引数)
（改行）

意図するところは明確です。ファイルを読み込み、コマンドライン引数に対して `main` 関数を呼び出すということです。しかし、システムは Guile パス以降のすべてを単一の引数 (文字列 `"-e main -s"`) として扱ってしまうため、これは望ましくありません。

回避策として、メタスイッチ「\」を使用すると、Guile プログラマーはカーネルにパッチを適用することなく任意の数のオプションを指定できます。Guile への最初の引数が「\」の場合、Guile は「\」に続く名前のスクリプト ファイルを開き、ファイルの 2 行目から引数を解析し (以下に説明するルールに従って)、それらを「\」スイッチに置き換えます。

メタスイッチと連携して、Guileは文字「#!」をコメントの開始として扱い、そのコメントは次の行まで続き、文字「!#」のみで構成されます。この種のコメントはGuileプログラムのどこにでも記述できますが、ファイルの先頭に記述すると最も効果的で、POSIXスクリプト呼び出しメカニズムと見事に連携します。

そこで、/u/jimb/ekko という名前のスクリプトを考えてみましょう。このスクリプトは次のように始まります。

#!/usr/local/bin/guile \\
-e メイン -s
!#
(define (main args)
(map (lambda (arg) (display arg) (display " "))
(cdr引数)
（改行）

ユーザーが次のようにこのスクリプトを実行したとします。

$ /u/jimb/ekko abc

次のようなことが起こります。

* オペレーティングシステムはファイルの先頭にある「#!」トークンを認識し、コマンドラインを次のように書き換えます。
    
/usr/local/bin/guile \\ /u/jimb/ekko abc
    
これはPOSIXで規定されている通常の動作です。
    
* Guile は最初の 2 つの引数 `\ /u/jimb/ekko` を見ると、/u/jimb/ekko を開き、そこから `-e`、`main`、`-s` の 3 つの引数を解析し、それらを `\` スイッチに置き換えます。したがって、Guile のコマンドラインは次のようになります。
    
/usr/local/bin/guile -e main -s /u/jimb/ekko abc
    
* Guile はこれらのスイッチを処理します。/u/jimb/ekko を Scheme コードのファイルとして読み込み (最初の 3 行をコメントとして扱います)、アプリケーション `(main "/u/jimb/ekko" "a" "b" "c")` を実行します。

Guileはメタスイッチ「\」を検出すると、以下のルールに従ってスクリプトファイルからコマンドライン引数を解析します。

* 各スペース文字は引数を終了します。つまり、連続する2つのスペースは引数「""」を導入します。
* 混乱を避けるため、タブ文字は使用できません（下記で説明するようにバックスラッシュ文字で引用する場合を除く）。
改行文字は引数のシーケンスを終了させ、最後の空でない引数も終了します。（ただし、スペースの後に改行文字があっても、最後の空文字列引数は追加されず、引数リストが終了するだけです。）
バックスラッシュ文字はエスケープ文字です。バックスラッシュ、スペース、タブ、改行をエスケープします。`\n` や `\t` などの ANSI C エスケープシーケンスもサポートされています。これらは引数の構成要素を生成します。2 文字の組み合わせ `\n` は、終端の改行のようには動作しません。正確に 3 桁の 8 進数のエスケープシーケンス `\NNN` は、 ASCII コードが NNN の文字として読み取られます。上記と同様に、このようにして生成された文字は引数の構成要素です。バックスラッシュの後に他の文字が続くことは許可されていません。

* * *

次へ: [スクリプトの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scripting-Examples)、前: [メタスイッチ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Meta-Switch)、上: [Guile スクリプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.3.3 コマンドライン処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command-Line-Handling-1)

テキストファイルから情報を抽出したり、既存のコマンドラインアプリケーションと連携したりするなど、特定の課題を解決するためにGuileスクリプトを作成する場合、コマンドライン引数を受け入れて処理する機能は非常に重要です。この章では、Guileがコマンドライン引数をGuileスクリプトで利用できるようにする方法と、コマンドライン引数の処理を支援するためにGuileが提供するユーティリティについて説明します。

Guileスクリプトが呼び出されると、Guileはコマンドライン引数をプロシージャ`command-line`を介してアクセス可能にし、引数を文字列のリストとして返します。

例えば、スクリプトが

#! /usr/local/bin/guile -s
!#
（コマンドラインに書き込む）
（改行）

ファイル cmdline-test.scm に保存され、コマンドライン `./cmdline-test.scm bar.txt -o foo -frumple grob` を使用して呼び出されると、出力は次のようになります。

("./cmdline-test.scm" "bar.txt" "-o" "foo" "-frumple" "grob")

スクリプトの呼び出しに、スクリプトの読み込み後に呼び出すプロシージャを指定する `-e` オプションが含まれている場合、Guile はそのプロシージャを引数 `(command-line)` で呼び出します。したがって、`-e` を使用するスクリプトは、コード内で `command-line` を明示的に参照する必要はありません。たとえば、上記のスクリプトは、次のように記述した場合と全く同じ動作になります。

#! /usr/local/bin/guile \\
-e メイン -s
!#
(define (main args)
（引数を書き込む）
（改行）

（スクリプト呼び出しに複数の Guile オプションを含めることができるように、メタスイッチ `\` を使用していることに注意してください。詳細は [メタスイッチ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Meta-Switch) を参照してください。）

これらのスクリプトは、`#!` POSIX 規約を使用しているため、例のコマンドライン `./cmdline-test.scm bar.txt -o foo -frumple grob` のように、ファイル名を直接使用して実行できます。ただし、次のように、暗黙的に指定される Guile コマンドラインを完全に入力して実行することもできます。

$ guile -s ./cmdline-test.scm bar.txt -o foo -frumple grob

または

$ guile -e main -s ./cmdline-test2.scm bar.txt -o foo -frumple grob

スクリプトがこの長い形式で呼び出された場合でも、スクリプトが受け取る引数は、短い形式で呼び出された場合と同じです。Guile は、`(command-line)` または `-e` 引数がスクリプトの呼び出し方法に依存しないように、Guile 自身が処理する引数を取り除きます。

スクリプトは、コマンドライン引数を自由に解析および処理できます。ただし、可能なオプションと引数のセットが複雑な場合、すべてのオプションを抽出したり、指定された引数の有効性をチェックしたりすることが難しくなる場合があります。このタスクは、Guile に同梱されているモジュール `(ice-9 getopt-long)` を利用することで大幅に簡素化できます。[The (ice-9 getopt-long) Module](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong) を参照してください。

* * *

前へ: [コマンドライン処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command-Line-Handling)、上へ: [Guile スクリプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.3.4 スクリプトの例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scripting-Examples-1)

まず、Guileを直接呼び出す例をいくつか紹介します。

`guile -- abc`

Guileを対話的に実行します。`(command-line)`は以下を返します。
`("/usr/local/bin/guile" "a" "b" "c")`。

`guile -s /u/jimb/ex2 abc`

ファイル /u/jimb/ex2 を読み込みます。`(command-line)` は以下を返します。
`("/u/jimb/ex2" "a" "b" "c")`。

`guile -c '(write %load-path) (newline)'`

変数 `%load-path` の値を書き込み、改行を出力して終了します。

`guile -e main -s /u/jimb/ex4 foo`

ファイル /u/jimb/ex4 を読み込み、次に関数 `main` を呼び出し、リスト `("/u/jimb/ex4" "foo")` を渡します。

`guile -e '(ex4)' -s /u/jimb/ex4.scm foo`

ファイル /u/jimb/ex4.scm を読み込み、モジュール '(ex4)' の関数 `main` を呼び出し、リスト `("/u/jimb/ex4" "foo")` を渡します。

`guile -l first -ds -l last -s script`

ファイル、スクリプト、そして最後に、この順序で読み込みます。`-ds` スイッチは、`-s` スイッチを処理するタイミングを指定します。より具体的な例については、以下のスクリプトを参照してください。

以下は非常にシンプルなGuileスクリプトです。

#!/usr/local/bin/guile -s
!#
（「こんにちは、世界！」と表示）
（改行）

最初の行は、そのファイルがGuileスクリプトであることを示しています。ユーザーがそれを実行すると、システムは/usr/local/bin/guileを実行してスクリプトを解釈し、`-s`オプション、スクリプトのファイル名、およびスクリプトに渡された引数をコマンドライン引数として渡します。Guileは`-s script`オプションを見つけると、スクリプトをロードします。したがって、このプログラムを実行すると、次の出力が得られます。

こんにちは世界！

以下は、引数の階乗を出力するスクリプトです。

#!/usr/local/bin/guile -s
!#
(定義 (事実 n)
(もし (ゼロ? n) 1 の場合)
(* n (事実 (- n 1)))))

(display (fact (string->number (cadr (command-line)))))
（改行）

実際の動作：

$ ./fact 5
120
$

しかし、このファイル内の `fact` の定義を別のスクリプトから使用したいとします。単純にスクリプトファイルを `load` して `fact` の定義を使用することはできません。なぜなら、スクリプトは読み込み時に階乗を計算して表示しようとするからです。この問題を回避するには、スクリプトを次のように記述します。

#!/usr/local/bin/guile \\
-e メイン -s
!#
(定義 (事実 n)
(もし (ゼロ? n) 1 の場合)
(* n (事実 (- n 1)))))

(define (main args)
(display (fact (string->number (cadr args))))
（改行）

このバージョンでは、スクリプトが実行すべきアクションを`main`という関数にまとめています。これにより、余計な計算を行うことなく、定義のみを読み込むことができます。そして、メタスイッチ`\`とエントリポイントスイッチ`-e`を使用して、スクリプトの読み込み後にGuileに`main`を呼び出すように指示しました。

$ ./fact 50
30414093201713378043612608166064768844377641568960512000000000000

ここで、`choose`関数を計算するスクリプトを作成するとします。m個の異なるオブジェクトの集合が与えられたとき、`(choose nm)`は、それぞれn個のオブジェクトを含む異なる部分集合の数です。`fact`が与えられた場合の`choose`は簡単に作成できるので、スクリプトは次のように記述できます。

#!/usr/local/bin/guile \\
-l 事実 -e メイン -s
!#
(定義 (nmを選択)
(/ (事実 m) (\* (事実 (- mn)) (事実 n))))

(define (main args)
(let ((n (string->number (cadr args)))
(m (string->number (caddr args))))
（表示（nmを選択））
(改行)))

ここで指定するコマンドライン引数は、Guileに対し、まずファイルfactを読み込み、次に`main`をエントリポイントとしてスクリプトを実行するように指示します。つまり、`choose`スクリプトは`fact`スクリプトで定義された内容を使用できます。以下に実行例を示します。

$ ./choose 0 4
1
$ ./choose 1 4
4
$ ./choose 2 4
6
$ ./choose 3 4
4
$ ./choose 4 4
1
$ ./choose 50 100
100891344545564193334812497256

特定のモジュールから特定のプロシージャを呼び出すには、特別な形式 `(@ (モジュール) プロシージャ)` を使用できます。

#!/usr/local/bin/guile \\
-l 事実 -e (@ (fac) メイン) -s
!#
(define-module (fac)
#:export (main))

(定義 (nmを選択)
(/ (事実 m) (\* (事実 (- mn)) (事実 n))))

(define (main args)
(let ((n (string->number (cadr args)))
(m (string->number (caddr args))))
（表示（nmを選択））
(改行)))

`@@` を使用して、エクスポートされていないプロシージャを呼び出すことができます。エクスポートされたプロシージャの場合は、`(module)` という省略形を使用して呼び出しを簡略化できます。

#!/usr/local/bin/guile \\
-l 事実 -e (fac) -s
!#
(define-module (fac)
#:export (main))

(定義 (nmを選択)
(/ (事実 m) (\* (事実 (- mn)) (事実 n))))

(define (main args)
(let ((n (string->number (cadr args)))
(m (string->number (caddr args))))
（表示（nmを選択））
(改行)))

最大限の移植性を確保するため、代わりにシェルを使用して、指定したコマンドライン引数で`guile`を実行することもできます。この場合、コマンド引数を正しく引用符で囲むように注意する必要があります。

#!/usr/bin/env sh
exec guile -l fact -e '(@ (fac) main)' -s "$0" "$@"
!#
(define-module (fac)
#:export (main))

(定義 (nmを選択)
(/ (事実 m) (\* (事実 (- mn)) (事実 n))))

(define (main args)
(let ((n (string->number (cadr args)))
(m (string->number (caddr args))))
（表示（nmを選択））
(改行)))

最後に、経験豊富なスクリプト作成者であれば、サブプロセスについて触れていないことに気づくかもしれません。例えば、Bashでは、ほとんどのシェルスクリプトは`sed`などの他のプログラムを実行して実際の処理を行います。

Guile では、多くの場合、Guile 自体ですべての処理を実行できますので、まずはそれを試してみてください。ただし、プログラムを実行して終了を待つだけであれば、`system*` を使用してください。サブルーチンを実行してその出力を取得したり、入力を与えたりする必要がある場合は、`open-pipe` を使用してください。詳細については、[プロセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Processes) および [パイプ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pipes) を参照してください。

* * *

次へ: [Emacs での Guile の使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-in-Emacs)、前: [Guile スクリプト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Scripting)、上: [Scheme でのプログラミング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 4.4 Guile を対話的に使用する [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively-1)

`-c`引数や実行するスクリプト名を指定せずに、単に`guile`と入力してGuileを起動すると、対話型インタープリタが起動し、そこでScheme式を入力できます。Guileはそれらの式を評価し、結果を出力します。以下に簡単な例を示します。

scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) 3 4 5)
$1 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) 12
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "Hello world!\\n")
こんにちは世界！
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([values](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-values) 'a 'b)
$2 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) a
$3 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) b

この使用モードは「REPL」と呼ばれ、「Read-Eval-Print Loop」の略です。これは、Guileインタープリタが最初にユーザーが入力した式を読み込み、次にそれを評価し、最後に結果を出力するためです。

プロンプトには、現在使用している言語とモジュールが表示されます。この場合、現在の言語は`scheme`、現在のモジュールは`(guile-user)`です。Scheme以外の言語に対するGuileのサポートの詳細については、[その他の言語のサポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Other-Languages)を参照してください。

* [初期化ファイル、~/.guile](https://doc.guix.gnu.org/guile/latest/en/guile.html#Init-File)
* [Readline](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline)
* [値履歴](https://doc.guix.gnu.org/guile/latest/en/guile.html#Value-History)
* [REPLコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands)
* [エラー処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Error-Handling)
* [対話型デバッグ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Interactive-Debugging)

* * *

次へ: [Readline](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline)、上へ: [Guile の対話型使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.1 初期化ファイル ~/.guile [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Init-File_002c-_007e_002f_002eguile)

対話モードで実行する場合、Guileは~/.guileからローカルの初期化ファイルを読み込みます。このファイルには、評価対象となるScheme式が含まれている必要があります。

この機能により、ユーザーは対話型のGuile環境をカスタマイズし、追加モジュールを取り込んだり、REPLの実装をパラメータ化したりすることができます。

初期化ファイルを読み込まずにGuileを実行するには、コマンドラインオプション「-q」を使用します。

* * *

次へ: [値の履歴](https://doc.guix.gnu.org/guile/latest/en/guile.html#Value-History)、前: [初期化ファイル、~/.guile](https://doc.guix.gnu.org/guile/latest/en/guile.html#Init-File)、上: [Guile の対話型使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.2 Readline [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline-1)

以前に入力した式を繰り返したり、変更したり、入力中の式を編集したりしやすくするために、Guile は GNU Readline ライブラリを使用できます。ライセンス上の理由からデフォルトでは有効になっていませんが、Readline を有効にするには、次の 2 行を追加するだけで済みます。

scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 [readline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-readline-1)))
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([activate-readline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-activate_002dreadline))

これらの 2 行 (`scheme@(guile-user)>` プロンプトなし) を .guile ファイルに記述することをお勧めします。.guile の詳細については、[初期化ファイル、~/.guile](https://doc.guix.gnu.org/guile/latest/en/guile.html#Init-File) を参照してください。

* * *

次へ: [REPL コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands)、前: [Readline](https://doc.guix.gnu.org/guile/latest/en/guile.html#Readline)、上: [Guile の対話型使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.3 値の履歴 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Value-History-1)

Readlineが以前の入力行を再利用できるのと同様に、_値履歴_を使用すると、以前の評価の_結果_を新しい式で使用できます。値履歴が有効になっている場合、各評価結果は自動的に変数 `$1`、`$2`、… の次の変数に割り当てられます。その後、これらの変数を後続の式で使用できます。

scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([iota](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-iota) 10)
$1 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) (0 1 2 3 4 5 6 7 8 9)
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([apply](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-apply) [\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) ([cdr](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cdr) $1))
$2 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) 362880
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([sqrt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sqrt) $2)
$3 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) 602.3952191045344
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) $2 $1)
$4 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) (362880 0 1 2 3 4 5 6 7 8 9)

GuileのREPLは`(ice-9 history)`モジュールをインポートするため、値履歴はデフォルトで有効になっています。値履歴は、オプションインターフェースを使用してREPL内で有効または無効にすることができます。

scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ,option value-history #f
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) 'foo
フー
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ,option value-history #t
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) 'bar
$5 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) bar

値履歴がオフになっている場合でも、以前に記録された値にはアクセスできることに注意してください。まれに、これらの過去の計算への参照が原因で、Guile がメモリを過剰に使用する場合があります。このような場合は、後述する `clear-value-history!` プロシージャを使用してこれらの値をクリアし、ガベージコレクションを有効にすることができます。

価値履歴へのプログラムによるインターフェースは、以下のモジュールにあります。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (ice-9 の履歴))

スキーム手順: **value-history-enabled?** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-value_002dhistory_002denabled_003f)

値の履歴が有効になっている場合は true を、そうでない場合は false を返します。

Scheme Procedure: **enable-value-history!** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-enable_002dvalue_002dhistory_0021)

値履歴がオフになっている場合は、オンにしてください。

Scheme Procedure: **disable-value-history!** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-disable_002dvalue_002dhistory_0021)

値履歴が有効になっている場合は、無効にしてください。

Scheme Procedure: **clear-value-history!** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-clear_002dvalue_002dhistory_0021)

値の履歴をクリアします。保存された値が他のデータ構造やクロージャによって取得されない場合、ガベージコレクタによって解放される可能性があります。

* * *

次へ: [エラー処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Error-Handling)、前: [値の履歴](https://doc.guix.gnu.org/guile/latest/en/guile.html#Value-History)、上: [Guile の対話型使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.4 REPLコマンド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands-1)

REPLは、式を読み込み、評価し、その結果を出力するために存在します。しかし、REPLに式を別の方法で評価させたり、全く別の処理を実行させたりしたい場合もあります。ユーザーは、_REPLコマンド_を使用してREPLの動作に影響を与えることができます。

前のセクションでは、`,オプション` の形式のコマンドの例を示しました。

scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ,option value-history #t

コマンドは、先頭のカンマ（','）によって式と区別されます。ほとんどのプログラミング言語ではカンマで式を開始できないため、カンマはREPLに対して、続くテキストが式ではなくコマンドであることを示す有効な指標となります。

REPLコマンドは常に利用できるため便利です。現在のモジュールに`pretty-print`のバインディングがなくても、`,pretty-print`と入力すればいつでも実行できます。

以下のセクションでは、機能ごとに分類された各種コマンドについて説明します。多くのコマンドには略語があります。詳細については、オンラインヘルプ（`,help`）を参照してください。

* [ヘルプコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Help-Commands)
* [モジュールコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Module-Commands)
* [言語コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Language-Commands)
* [コンパイルコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compile-Commands)
* [プロファイルコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Profile-Commands)
* [デバッグコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Debug-Commands)
* [コマンドの検査](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inspect-Commands)
* [システムコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#System-Commands)

* * *

次へ: [モジュールコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Module-Commands)、上へ: [REPLコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.4.1 ヘルプコマンド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Help-Commands-1)

Guileが対話モードで起動すると、ユーザーに「,help」と入力することでヘルプが表示されることを通知します。実際、「help」はコマンドであり、他のコマンドを知ることができるため、特に便利なコマンドです。

REPLコマンド: **help** \[`all` | group | `[-c]` command\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-help)

助けてください。

引数が1つの場合、その引数をグループ名として検索し、検索に成功した場合はそのグループに関するヘルプを表示します。それ以外の場合は、引数をコマンドとして検索し、コマンドに関するヘルプを表示します。

コマンド名がグループ名と同じ場合は、「\-c command」形式を使用して、グループ名ではなくコマンドに関するヘルプを表示します。

何も指定しなくても、ヘルプコマンドとコマンドグループの一覧が表示されます。

REPLコマンド: **show** \[topic\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-show)

ガイルに関する情報を提供します。

引数を1つ指定することで、特定の情報を表示しようとします。現在サポートされているトピックは、「保証」（または「w」）、「コピー」（または「c」）、および「バージョン」（または「v」）です。

何の議論もなく、トピックの一覧が表示されます。

REPL コマンド: **apropos** 正規表現 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-apropos)

バインディング/モジュール/パッケージを検索します。

REPLコマンド: **describe** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-describe)

説明／資料を表示します。

* * *

次へ: [言語コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Language-Commands)、前: [ヘルプコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Help-Commands)、上: [REPLコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.4.2 モジュールコマンド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Module-Commands-1)

REPLコマンド: **module** \[module\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-module)

モジュールを変更する／現在のモジュールを表示する。

REPLコマンド: **import**モジュール… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-import)

モジュールをインポートする／インポートされたモジュールの一覧を表示する。

REPLコマンド: **ファイルの読み込み** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-load-1)

現在のモジュールにファイルを読み込みます。

REPLコマンド: **reload** \[module\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reload)

指定されたモジュールを再読み込みします。指定がない場合は、現在のモジュールを再読み込みします。

REPLコマンド: **binding** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-binding)

現在のバインディングを一覧表示します。

REPLコマンド: **in**モジュール式 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-in)

REPLコマンド: **in**モジュールコマンド引数… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-in-1)

式を評価するか、あるいはモジュールのコンテキストで別のメタコマンドを実行します。たとえば、「,in (foo bar) ,binding」と入力すると、モジュール「(foo bar)」内のバインディングが表示されます。

* * *

次へ: [コンパイルコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compile-Commands)、前: [モジュールコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Module-Commands)、上: [REPLコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.4.3 言語コマンド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Language-Commands-1)

REPLコマンド: **language** language [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-language)

言語を変更する。

* * *

次へ: [プロファイルコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Profile-Commands)、前: [言語コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Language-Commands)、上: [REPLコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.4.4 コンパイルコマンド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compile-Commands-1)

REPLコマンド: **compile** exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compile)

コンパイル済みコードを生成します。

REPLコマンド: **compile-file** file [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compile_002dfile)

ファイルをコンパイルします。

REPLコマンド: **expand** exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-expand)

フォーム内のマクロを展開します。

REPLコマンド: **optimize** exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-optimize)

オプティマイザをコードに適用し、結果を出力してください。

REPL コマンド: **逆アセンブル** exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-disassemble)

コンパイル済みのプロシージャを逆アセンブルする。

REPLコマンド: **disassemble-file** file [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-disassemble_002dfile)

ファイルを逆アセンブルする。

* * *

次へ: [デバッグ コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Debug-Commands)、前: [コンパイル コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compile-Commands)、上: [REPL コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.4.5 プロファイルコマンド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Profile-Commands-1)

REPLコマンド: **time** exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-time-3)

時間的実行。

REPLコマンド: **profile** exp \[#:hz hz=100\] \[#:count-calls? count-calls?=#f\] \[#:display-style display-style=list\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-profile)

式の実行をプロファイルします。このコマンドは exp をコンパイルし、statprof プロファイラ内で実行します。すべてのキーワードオプションは `statprof` プロシージャに渡されます。statprof およびこのコマンドで使用できるオプションの詳細については、[Statprof](https://doc.guix.gnu.org/guile/latest/en/guile.html#Statprof) を参照してください。

REPLコマンド: **trace** exp \[#:width w\] \[#:max-indent i\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-trace)

トレース実行。

デフォルトでは、トレースの幅は端末の幅、または指定されている場合はその幅に制限されます。ネストされたプロシージャ呼び出しは右側に表示されますが、インデントの幅が最大インデント幅を超えると、インデントは省略されます。

これらの REPL コマンドは、`(ice-9 time)` モジュールを含めることで、スキーム コード内で通常の関数として呼び出すこともできます。

* * *

次へ: [検査コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inspect-Commands )、前: [プロファイルコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Profile-Commands)、上: [REPLコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.4.6 デバッグコマンド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Debug-Commands-1)

これらのデバッグコマンドは再帰的なREPL内でのみ使用可能であり、トップレベルでは機能しません。

REPLコマンド: **backtrace** \[count\] \[#:width w\] \[#:full? f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-backtrace)

バックトレースを出力する。

スタック内のすべてのフレーム、または最内側のカウントフレームのバックトレースを表示します。カウントが負の値の場合は、最後のカウントフレームが表示されます。

REPLコマンド: **up** \[count\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-up)

呼び出し元のスタックフレームを選択します。

このフレームを呼び出したスタックフレームを選択して出力します。引数には、何フレーム上まで遡るかを指定します。

REPLコマンド: **down** \[count\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-down)

指定されたスタックフレームを選択します。

このフレームによって呼び出されたスタックフレームを選択して出力します。引数には、何フレーム下までスクロールするかを指定します。

REPLコマンド: **frame** \[idx\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-frame)

フレームを表示します。

選択したフレームを表示します。引数を使用して、インデックスでフレームを選択し、表示します。

REPLコマンド: **locals** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-locals)

ローカル変数を表示します。

選択したフレーム内のローカル変数を表示します。

REPLコマンド: **error-message** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error_002dmessage)

REPLコマンド: **エラー** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error)

エラーメッセージを表示します。

現在のデバッグREPLを開始したエラーに関連付けられたメッセージを表示します。

REPLコマンド: **registers** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-registers)

現在のフレームに関連付けられているVMレジスタを表示します。

VMスタックフレームの詳細については、[スタックレイアウト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Stack-Layout)を参照してください。

REPLコマンド: **width** \[cols\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-width)

`,backtrace` および `,locals` の出力における表示列数を cols に設定します。cols が指定されていない場合は、端末の幅が使用されます。

次の3つのコマンドは、どのREPLでも動作します。

REPLコマンド: **break** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-break)

proc にブレークポイントを設定します。

REPLコマンド: **break-at-source** ファイル行 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-break_002dat_002dsource)

指定されたソースコードの場所にブレークポイントを設定します。

REPLコマンド: **tracepoint** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tracepoint)

指定されたプロシージャにトレースポイントを設定します。これにより、プロシージャへのすべての呼び出しでトレースメッセージが出力されます。詳細については、[トレーストラップ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Tracing-Traps)を参照してください。

このサブセクションの残りのコマンドはすべて、スタックが_継続可能_な場合、つまりスタックの元となるプログラムの実行を継続することが理にかなっている場合にのみ適用されます。通常、これはプログラムがトラップまたはブレークポイントによって停止したことを意味します。

REPLコマンド: **step** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-step)

デバッグ対象プログラムに、次のソースコード位置へステップするように指示します。

REPLコマンド: **next** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-next)

デバッグ対象プログラムに、同じフレーム内の次のソース位置へステップするように指示します。（動作の詳細については、[トラップ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Traps)を参照してください。）

REPLコマンド: **finish** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-finish)

デバッグ対象のプログラムに対し、現在のスタックフレームの処理が完了するまで実行を継続させ、完了した時点で結果を出力し、REPLに再入力するように指示します。

* * *

次へ: [システムコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#System-Commands)、前: [デバッグコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Debug-Commands)、上: [REPLコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.4.7 コマンドの検査 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inspect-Commands-1)

REPLコマンド: **inspect** exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-inspect)

実験の評価結果を調べます。

REPLコマンド: **pretty-print** exp [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pretty_002dprint)

exp の評価結果を整形して表示します。

* * *

前へ: [検査コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inspect-Commands)、上へ: [REPLコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.4.8 システムコマンド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#System-Commands-1)

REPLコマンド: **gc** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gc)

ゴミ収集。

REPLコマンド: **statistics** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-statistics)

統計情報を表示します。

REPLコマンド: **オプション** \[名前\] \[式\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option)

引数なしの場合、すべてのオプションを一覧表示します。引数が 1 つの場合、name オプションの現在の値を表示します。引数が 2 つの場合、name オプションを Scheme 式 exp の評価結果に設定します。

REPLコマンド: **quit** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quit)

このセッションを終了する。

現在のREPLオプションは以下のとおりです。

`コンパイルオプション`

REPLで入力した式をコンパイルする際に使用されるオプション。コンパイルオプションの詳細については、[Schemeコードのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compilation)を参照してください。

`interp`

REPLで入力された式を解釈するかコンパイルするか（選択肢がある場合）。デフォルトではオフ（コンパイルを実行する）。

プロンプト

カスタマイズされたREPLプロンプト。デフォルトでは`#f`となっており、デフォルトのプロンプトを示します。

`print`

各式の評価結果を出力するために使用される、2つの引数を持つプロシージャです。引数は、現在のREPLと出力する値です。デフォルトでは、デフォルトのプロシージャを使用するには`#f`を指定します。

`値履歴`

値履歴が有効になっているかどうか。[値履歴](https://doc.guix.gnu.org/guile/latest/en/guile.html#Value-History)を参照してください。

`on-error`

エラー発生時の対処方法。デフォルトでは`debug`が指定されており、これはデバッガーを起動することを意味します。その他の値としては、デバッガーを起動せずにバックトレースを表示する`backtrace`、または簡単なエラーメッセージを表示する`report`があります。

REPLオプションのデフォルト値は、`(system repl common)`の`repl-default-option-set!`を使用して設定できます。

スキームプロシージャ: **repl-default-option-set!** キー値[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-repl_002ddefault_002doption_002dset_0021)

REPLオプションのデフォルト値を設定します。この関数は、ユーザーの初期化ファイルで特に役立ちます。[初期化ファイル、~/.guile](https://doc.guix.gnu.org/guile/latest/en/guile.html#Init-File)を参照してください。

* * *

次へ: [対話型デバッグ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Interactive-Debugging)、前: [REPL コマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Commands)、上: [Guile の対話型使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.5 エラー処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Error-Handling-1)

REPLから評価されているコードでエラーが発生すると、Guileは新しいプロンプトを表示し、エラーのコンテキストを調べることができるようにします。

scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) (map [string-append](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dappend) '("a" "b") '("c" #\\d))
エラー: [プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure) の string-append で:
エラー: 型が間違っています ([string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string) を期待しています): #\\d
新しいプロンプトが表示されます。バックトレースを表示するには「,bt」と入力し、続行するには「,q」と入力してください。
scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e)

新しいプロンプトは、古いプロンプトの中で、エラーの動的なコンテキスト内で実行されます。これは、スタックの具象表現で拡張された再帰的なREPLであり、デバッグの準備が整っています。

`,backtrace`（略して`,bt`）は、エラーが発生した時点でのSchemeのコールスタックを表示します。

scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ,bt
1 (map #<procedure [string-append](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dappend) \_> ("a" "b") ("c" #\\d))
0 ([string-append](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dappend) "b" #\\d)

上記の例では、`map`と`string-append`はどちらもプリミティブであるため、バックトレースにはソース情報があまり含まれていません。しかし、一般的には、バックトレースの左側のスペースは、あるプロシージャが別のプロシージャを呼び出す行と列を示しています。

再帰的な REPL を終了するには、他の REPL と同様に、'(quit)'、',quit' (',q' と略記)、または Cd などのオプションを使用できます。

* * *

前へ: [エラー処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Error-Handling)、上へ: [Guile の対話型使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 4.4.6 対話型デバッグ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Interactive-Debugging-1)

再帰デバッグREPLは、エラー発生時の計算状態を検査する多数のメタコマンドを公開します。これらのコマンドを使用すると、

* エラーが発生した時点のSchemeコールスタックを表示します。
* コールスタックを上下に移動して、各フレームで評価されている式または適用されている手順を詳細に確認します。
* 各フレームの文脈において、変数と式の値を検証する。

個々のコマンドの詳細については、[デバッグコマンド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Debug-Commands)を参照してください。このセクションでは、一般的なデバッグセッションの手順をより詳しく説明します。

まず、適切なエラーメッセージを用意する必要があります。`quasiquote` 形式の外で式 `(unquote foo)` をマクロ展開してみて、マクロエクスパンダーがこのエラーをどのように報告するかを確認してみましょう。

scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) (macroexpand '([unquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote-1) foo))
エラー: [プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure) マクロ展開:
エラー: unquote: 式 [not](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-not) は [quasiquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quasiquote-1) [in](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-in) ([unquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote-1) foo) の外部では有効ではありません
新しいプロンプトが表示されます。バックトレースを表示するには「,bt」と入力し、続行するには「,q」と入力してください。
scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e)

`backtrace`コマンド（`bt`としても呼び出すことができる）は、デバッガが起動した時点のコールスタック（バックトレースとも呼ばれる）を表示します。

scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ,bt
ice-9/psyntax.scm 内:
1130:21 3 (chi-top ([unquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote-1) foo) () ((top)) e ([eval](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eval)) (hygiene #))
1071:30 2 (syntax-type ([unquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote-1) foo) () ((top)) #f #f (# #) #f)
1368:28 1 (chi-macro #<procedure de9360 at ice-9/psyntax.scm...> [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
不明なファイル内:
0 ([scm-error](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_002derror) 構文エラー macroexpand "~a: ~a in ~a" # #f)

コールスタックは、スタックフレームのシーケンスで構成され、各フレームは、別のプロシージャから返された値を使って何らかの処理を行うのを待っているプロシージャを表します。ここでは、スタック上に4つのフレームがあることがわかります。

`macroexpand` はスタック上に存在しないことに注意してください。`chi-top` への末尾呼び出しが行われたに違いありません。実際、`ice-9/psyntax.scm` でその定義を検索すれば、それがわかります。

デバッガーに入ると、最も内側のフレームが選択されます。つまり、「現在の」フレームに関する情報を取得したり、現在のフレームのコンテキストで式を評価したりするコマンドは、デフォルトでは最も内側のフレームを基準として実行されます。これらの操作を別のフレームに適用するには、次のように `up`、`down`、`frame` コマンドを使用します。

scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) 、up
ice-9/psyntax.scm 内:
1368:28 1 (chi-macro #<procedure de9360 at ice-9/psyntax.scm...> [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) 、フレーム 3
ice-9/psyntax.scm 内:
1130:21 3 (chi-top ([unquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote-1) foo) () ((top)) e ([eval](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eval)) (hygiene #))
scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ,down
ice-9/psyntax.scm 内:
1071:30 2 (syntax-type ([unquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote-1) foo) () ((top)) #f #f (# #) #f)

フレーム2で何が起こっているのかに興味があるかもしれないので、そのローカル変数を見てみましょう。

scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ,locals
ローカル変数:
$1 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) e [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) ([unquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote-1) foo)
$2 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) r [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) ()
$3 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) w [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) ((top))
$4 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) s [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) #f
$5 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) リブ [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) #f
$6 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) [mod](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-mod) [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) (hygiene guile-user)
$7 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) for-car? [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) #f
$8 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) [first](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-first) [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) [unquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote-1)
$9 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) ftype [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) macro
$10 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) fval [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) #<procedure de9360 at ice-9/psyntax.scm:2817:2 (x)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e)
$11 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) fe [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) [unquote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unquote-1)
$12 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) fw [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) ((top))
$13 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) fs [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) #f
$14 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) fmod [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) (hygiene guile-user)

すべての値は、値履歴名（`$n`）でアクセスできます。

scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) $10
$15 [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) #<procedure de9360 at ice-9/psyntax.scm:2817:2 (x)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e)

REPLでプロシージャを直接呼び出すこともできます。

scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ($10 'not-going-to-work)
エラー: [プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure) マクロ展開:
エラー: ソース式がパターン [in](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-match) [any](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-any) [match](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-in) not-going-to-work に一致しませんでした
新しいプロンプトが表示されます。バックトレースを表示するには「,bt」と入力し、続行するには「,q」と入力してください。

さて、この時点でエラーの中にさらにエラーが発生してしまいました。トップレベルに戻って終了しましょう。

scheme@(guile-user) \[2\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ,q
scheme@(guile-user) \[1\][\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) ,q
scheme@(guile-user)[\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e)

最後に、賢明な方へのアドバイスとして：ハッカーは REPL プロンプトを Cd で閉じます。

* * *

次へ: [Guile ツールの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Tools)、前: [Guile の対話型使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Interactively)、上: [Scheme でのプログラミング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 4.5 Emacs での Guile の使用 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-in-Emacs-1)

どのテキストエディタでもSchemeを編集できますが、優れたエディタもあればそうでないものもあります。もちろんEmacsが最高ですが、それは単に優れたテキストエディタだからというだけではありません。EmacsはSchemeを標準でしっかりとサポートしており、適切なインデント規則、括弧のマッチング、構文ハイライト、さらには構造編集のためのキーバインディングセットまで備えています。これにより、バランスの取れたS式に対して、ナビゲーション、切り取り＆貼り付け、転置といった操作が可能になります。

とはいえ、EmacsとGuileの使用体験を大幅に向上させる2つの方法があります。

まず一つ目は、テイラー・キャンベルの[Paredit](http://www.emacswiki.org/emacs/ParEdit)です。PareditなしでLispの方言でコーディングするべきではありません。（意見のない文章は退屈だと言われますが、この調子はそのためです。しかし、これは紛れもない事実です。）Pareditは最高です。

2つ目は、José Antonio Ortega Ruiz氏の[Geiser](http://www.nongnu.org/geiser/)です。Geiserは、Emacsの`scheme-mode`を補完し、`comint-mode` REPLバッファを介してGuileプロセスを実行するための緊密な統合を提供します。

もちろん、REPLに切り替えるためのキーバインドや優れたREPL環境はありますが、Geiserはさらに一歩進んで、以下の機能を提供します。

* 現在のファイルのモジュールのコンテキストにおけるフォームの評価。
* マクロ展開。
* ファイル/モジュールの読み込みおよび/またはコンパイル。
* 名前空間を考慮した識別子の補完（ローカルバインディング、現在のモジュールで表示される名前、モジュール名を含む）。
* Autodoc: エコー領域には、ポイント周辺のプロシージャ/マクロのシグネチャに関する情報が自動的に表示されます。
* 指定された位置にある識別子の定義にジャンプします。
* ドキュメントへのアクセス（実装側でドキュメント文字列が提供されている場合は、それらも含む）。
* 特定のモジュールによってエクスポートされた識別子の一覧。
* プロシージャの呼び出し元/呼び出し先のリスト。
* デバッグおよびエラーナビゲーションのための基本的なサポート。
* 複数のREPLを同時にサポートします。

詳細については、ガイザーのウェブページ（[http://www.nongnu.org/geiser/](http://www.nongnu.org/geiser/)）をご覧ください。

* * *

次へ: [サイトパッケージのインストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Installing-Site-Packages)、前: [Emacs での Guile の使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-in-Emacs)、上: [Scheme でのプログラミング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 4.6 Guile Tools の使用 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Tools-1)

Guileには、コンパイラ、逆アセンブラ、モジュールインスペクタなど、コマンドラインユーティリティが多数付属しており、将来的にはインターネットからGuileパッケージをインストールするシステムも提供される予定です。これらのツールは、`guild`プログラムを使用して起動できます。

$ guild compile -o foo.go foo.scm
`foo.go` を書きました

このプログラムは、Guile バージョン 2.0.1 までは `guile-tools` と呼ばれていましたが、後方互換性のために現在でもそのように呼ばれることがあります。しかし、名前を `guild` に変更しました。これは、より短く読みやすいというだけでなく、このツールが CPAN のようなシステムを使ってハッカー同士がコードを共有できるようにすることで、Guile のエキスパート同士を結びつける役割を果たすためです。

`guild compile` の詳細については、[Compiling Scheme Code](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compilation) を参照してください。

ギルドスクリプトの完全なリストは、`guild list` または単に `guild` を実行することで取得できます。

* * *

次へ: [Guile コードの配布](https://doc.guix.gnu.org/guile/latest/en/guile.html#Distributing-Guile-Code)、前: [Guile ツールの使用](https://doc.guix.gnu.org/guile/latest/en/guile.html#Using-Guile-Tools )、上: [Scheme でのプログラミング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 4.7 サイトパッケージのインストール [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Installing-Site-Packages-1)

いずれは、自分のコードを他の人と共有したくなるでしょう。効果的に共有するためには、ユーザーがパッケージを簡単にインストールして使用できるように、共通の規約に従うことが重要です。

まず最初に、Guile が Scheme ファイルを見つけられる場所に Scheme ファイルをインストールする必要があります。Guile は Scheme ファイルを探す際、ロード パスを検索します。最初に Guile 自身のパスを検索し、次にサイト パッケージのパスを検索します。サイト パッケージとは、インストールされた Scheme コードのうち、 Guile 自体の一部ではないものを指します。ロード パスの詳細については、[ロード パス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths) を参照してください。

歴史的な理由から、サイトパスは複数存在しますが、一般的に使用すべきパスは、`%site-dir`プロシージャを呼び出すことで取得できます。[構成、ビルド、インストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Build-Config)を参照してください。Guile 3.0がシステムに`/usr/`にインストールされている場合、`(%site-dir)`は`/usr/share/guile/site/3.0`になります。Schemeファイルはそこにインストールする必要があります。

コンパイル済みの `.go` ファイルをインストールしない場合、Guile はモジュールとプログラムを初めて使用する際にコンパイルし、ユーザーのホーム ディレクトリにキャッシュします。自動コンパイルの詳細については、[Scheme コードのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compilation) を参照してください。ただし、ファイルをインストールする前にコンパイルし、Guile が認識できる場所にコピーする方が望ましいです。

Scheme ファイルと同様に、Guile はコンパイル済みの `.go` ファイルを探すために、`%load-compiled-path` というパスを検索します。デフォルトでは、このパスには Guile のファイル用のパスとサイト パッケージ用のパスの 2 つのエントリがあります。`.go` ファイルは後者のディレクトリにインストールする必要があります。このディレクトリの値は、 `%site-ccache-dir` プロシージャを呼び出すことで取得できます。前の例と同様に、Guile 3.0 がシステムの `/usr/` にインストールされている場合、`(%site-ccache-dir)` のサイト パッケージは `/usr/lib/guile/3.0/site-ccache` になります。

`.go` ファイルは、`.scm` ファイルよりも新しい場合にのみ優先的にロードされることに注意してください。そのため、Scheme ファイルを最初にインストールし、コンパイル済みのファイルを次にインストールする必要があります。ロードプロセスの詳細については、[ロードパス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Load-Paths) を参照してください。

最後に、このセクションでは Scheme についてのみ説明していますが、C 拡張機能もインストールする必要がある場合があります。共有ライブラリは _extensions ディレクトリにインストールする必要があります。この値はビルド構成から取得できます ([構成、ビルド、インストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Build-Config) を参照)。繰り返しになりますが、Guile 3.0 がシステムに `/usr/` にインストールされている場合、extensions ディレクトリは `/usr/lib/guile/3.0/extensions` になります。

* * *

前へ: [サイトパッケージのインストール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Installing-Site-Packages)、上へ: [Scheme でのプログラミング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

### 4.8 Guile コードの配布 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Distributing-Guile-Code-1)

Guileにはバンドルされていませんが、日々のGuileの使用において非常に役立つツールがあります。それは[Hall](https://gitlab.com/a-sassmannshausen/guile-hall)です。

Hallは、シンプルなコマンドラインインターフェースを通して、Guileプロジェクトの作成、管理、パッケージ化を支援します。新しいプロジェクトを開始すると、Hallは新しいプロジェクトの骨組みとなるフォルダを作成します。このフォルダには、テスト、ライブラリ、スクリプト、ドキュメント用のディレクトリが含まれています。つまり、作業中のファイルをどこに配置すればよいかがすぐにわかるということです。

さらに、このスキャフォールドには基本的な「Autotools」の設定が含まれているため、自分で設定する必要はありません（GNU「Autotools」の詳細については、「Autoconf: 自動構成スクリプトの作成」の「GNUビルドシステム」を参照してください）。プロジェクトにAutotoolsが設定されていれば、他の人のコンピュータでコードが動作するかどうかを心配することなく、すぐにプロジェクトの開発に取り掛かることができます。HallはGNU Guixパッケージマネージャ用のパッケージ定義も生成できるため、Guixユーザーは簡単にインストールできます。

* * *

次へ: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference)、前へ: [Scheme でのプログラミング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Programming-in-Scheme)、上へ: [Guile リファレンス マニュアル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Top) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
