### 7.4 (ice-9 getopt-long) モジュール [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-_0028ice_002d9-getopt_002dlong_0029-Module)

`(ice-9 getopt-long)` 機能は、コマンドラインで Guile プログラムに渡される引数を解析するのに役立つように設計されており、C ライブラリの同名の機能をモデルにしています (GNU C ライブラリ リファレンス マニュアルの [Getopt](https://doc.guix.gnu.org/libc/latest/en/libc.html#Getopt) を参照)。コマンドライン引数解析のより低レベルなインターフェースについては、[SRFI-37 - args-fold](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d37) を参照してください。

`(ice-9 getopt-long)` モジュールは、`getopt-long` と `option-ref` という 2 つのプロシージャをエクスポートします。

* `getopt-long` は、文字列のリスト（コマンドライン引数）、_オプション指定_、およびいくつかのオプションのキーワードパラメータを受け取ります。オプション指定とキーワードパラメータに従ってコマンドライン引数を解析し、解析結果をカプセル化したデータ構造を返します。
* `option-ref` は、解析されたデータ構造と特定のオプション名を受け取り、そのオプションに関する情報を返します。

これらの手順を Guile スクリプトで使用できるようにするには、`getopt-long` または `option-ref` の最初の使用の前に、スクリプトの先頭付近に `(use-modules (ice-9 getopt-long))` という式を含めます。

* [getopt-long の簡単な例](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong-Example)
* [オプション仕様の書き方](https://doc.guix.gnu.org/guile/latest/en/guile.html#Option-Specification)
* [想定されるコマンドライン形式](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command-Line-Format)
* [`getopt-long` のリファレンスドキュメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong-Reference)
* [`option-ref` のリファレンスドキュメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#option_002dref-Reference)

* * *

次へ: [オプション仕様の書き方](https://doc.guix.gnu.org/guile/latest/en/guile.html#Option-Specification)、上へ: [(ice-9 getopt-long) モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.4.1 getopt-long の短い例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Short-getopt_002dlong-Example)

このセクションでは、簡単な例を提示して分析することで、`getopt-long` の使い方を説明します。まず必要なのは、`getopt-long` がコマンドラインをどのように解析するかを指示する _オプション仕様_ です。この仕様は、長いオプション名をキーとする連想リストです。仕様は次のようになります。

(オプション仕様を定義)
'(([version](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-version) (single-char #\\v) (value #f))
([help](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-help) (単一文字 #\\h) (値 #f))))

この alist は、`getopt-long` に対して、_version_ と _help_ という 2 つの長いオプションを受け入れること、そしてこれらのオプションはそれぞれ _v_ と _h_ という 1 文字の略語でも選択できることを指示します。`(value #f)` 句は、どちらのオプションも値を受け付けないことを示します。

この仕様により、`getopt-long`を使用して指定されたコマンドラインを解析できます。

(オプションを定義します ([getopt-long](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getopt_002dlong) ([command-line](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-command_002dline)) オプション仕様))

この呼び出しの後、`options`には解析されたコマンドラインが格納され、`option-ref`による検査の準備が整います。`option-ref`は次のように呼び出されます。

([option-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option_002dref) options 'help #f)

`option-ref` は、解析されたコマンドライン、検査対象のオプションを示すシンボル、およびデフォルト値を受け取ります。コマンドラインにオプションが存在しない場合、またはオプションは存在するが値が指定されていない場合は、デフォルト値が返されます。それ以外の場合は、コマンドラインの値が返されます。通常、`option-ref` はスクリプトがサポートするオプションごとに一度呼び出されます。

以下の例は、これらすべてを組み合わせてコマンドラインを解析し、ユーザーが何を求めているのかを判断するメインプログラムを示しています。

(define (main args)
(let\* ((option-spec '(([version](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-version) (single-char #\\v) (value #f))
([help](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-help) (単一文字 #\\h) (値 #f))))
(オプション ([getopt-long](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getopt_002dlong) 引数 オプション仕様))
(help-wanted ([option-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option_002dref) options 'help #f))
(version-wanted ([option-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option_002dref) options 'version #f)))
(もし (またはバージョン希望、ヘルプ希望)
（始める
（バージョン指定の場合）
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "getopt-long-example バージョン 0.3\n"))
（求人情報）
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "\\
getopt-long-example [オプション]
-v、--version バージョンを表示する
-h、--help このヘルプを表示します
")))
（始める
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "Hello, World!") ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))))))

* * *

次へ: [想定されるコマンドライン形式](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command-Line-Format)、前: [短い getopt-long の例](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong-Example)、上: [(ice-9 getopt-long) モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.4.2 オプション仕様の書き方 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#How-to-Write-an-Option-Specification)

オプション仕様は、サポートされているオプションごとに1つのリスト要素を持つ連想リストです（[連想リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Association-Lists)を参照）。各リスト要素のキーはオプションの名前を表すシンボルであり、値はオプションのプロパティのリストです。

OPTION-SPEC ::= '( (OPT-NAME1 (PROP-NAME PROP-VALUE) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
(OPT-NAME2 (PROP-NAME PROP-VALUE) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
(OPT-NAME3 (PROP-NAME PROP-VALUE) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)
）

各 opt-name は、そのオプションの長いオプション名を指定します。たとえば、opt-name が `background` のリスト要素は、コマンドラインで長いオプション `--background` を使用して指定できるオプションを指定します。オプションに関する詳細情報（値を取るかどうか、コマンドラインで必須かどうかなど）は、オプションのプロパティで指定されます。

前のセクションで説明した例では、長いオプション名には同等の短いオプション文字が存在することをすでに確認しました。同等の短いオプション文字は、オプションのプロパティリストで `single-char` プロパティを指定することで設定できます。たとえば、`'(output (single-char #\o) …)` のようなリスト要素は、長い名前 `--output` を持つオプションを指定しますが、これは同等の短い名前 `-o` でも指定できます。

`value` プロパティは、オプションが値を必要とするか、値を受け入れるかを指定します。`value` プロパティが `#t` に設定されている場合、オプションは値を必要とします。対応する値がないオプション名が存在する場合、`getopt-long` はエラーを通知します。`#f` に設定されている場合、オプションは値を受け取りません。この場合、コマンドラインでオプション名の後に続くオプション以外の単語は、オプション以外の引数として扱われます。シンボル `optional` に設定されている場合、オプションは値を受け入れますが、値を必要としません。コマンドラインでオプション名の後に続くオプション以外の単語は、そのオプションの値として解釈されます。`'(value optional)` を持つオプションのオプション名の直後に _another_ オプション名がコマンドラインで続く場合、最初のオプションの値は暗黙的に `#t` になります。

`required?` プロパティは、コマンドラインでオプションを指定する必要があるかどうかを示します。`required?` プロパティが `#t` に設定されている場合、オプションが指定されていないと `getopt-long` はエラーを通知します。

最後に、`predicate` プロパティを使用して、オプションの可能な値を制限できます。使用する場合は、`predicate` プロパティを、提案されたオプション値を文字列として引数に取り、提案された値が許容されるか否かに応じて `#t` または `#f` を返すプロシージャに設定する必要があります。predicate プロシージャが `#f` を返すと、`getopt-long` はエラーを通知します。

デフォルトでは、オプションには1文字の対応語がなく、必須ではなく、値も取りません。オプションのリスト要素に`value`プロパティは含まれているが`predicate`プロパティが含まれていない場合、オプションの値は制約されません。

* * *

次へ: [`getopt-long` のリファレンス ドキュメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong-Reference)、前: [オプション仕様の書き方](https://doc.guix.gnu.org/guile/latest/en/guile.html#Option-Specification)、上: [(ice-9 getopt-long) モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.4.3 想定されるコマンドライン形式 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Expected-Command-Line-Format)

`getopt-long`がコマンドラインを正しく解析するためには、そのコマンドラインが、コマンドラインオプションの指定方法に関する標準的な規則に準拠している必要があります。このセクションでは、それらの規則について説明します。

`getopt-long` は、指定されたコマンドラインを複数の部分に分割します。引数リストのすべての要素は、オプションまたは通常の引数に分類されます。オプションは、2 つのハイフンとオプション名（いわゆる _long_ オプション）または 1 つのハイフンの後に 1 つの文字が続く形式（_short_ オプション）で構成されます。

オプションは、値を指定せずに指定するとスイッチのように動作し、値を指定してプログラムに渡すために使用できます。オプションの値は、等号を使用して指定することも、コマンドラインの次の単語をそのまま指定することもできます。したがって、次の 2 つの呼び出しは同等です。

$ ./foo.scm --output=bar.txt
$ ./foo.scm --output bar.txt

短いオプションは、長いオプションの代わりに使用でき、単一のハイフンで区切ってグループ化できます。たとえば、以下のコマンドは同等です。

$ ./foo.scm --version --help
$ ./foo.scm -v --help
$ ./foo.scm -vh

オプションに値が必要な場合、そのオプションはグループ内の最後のオプションである場合にのみ、他の短いオプションとグループ化できます。値は次の引数になります。たとえば、次のオプション指定では、

((りんご (1文字 #\\a))
(飛行船 (1文字 #\\b) (値 #t))
(catalexis (単一文字 #\\c) (値 #t)))

—以下のコマンドラインはすべて許容されます。

$ ./foo.scm -a -b bang -c couth
$ ./foo.scm -ab bang -c couth
$ ./foo.scm -ac couth -b bang

しかし、次のコマンドラインはエラーです。なぜなら、`-b` はその組み合わせの最後のオプションではなく、短いオプションのグループには、両方とも値を必要とするオプションを 2 つ含めることはできないからです。

$ ./foo.scm -abc couth bang

オプションの値がオプションの場合、`getopt-long` は引数リストのその後に続く要素を見て、そのオプションに値があるかどうかを判断します。次の要素が文字列であり、それがオプション自体ではない場合、その文字列がオプションの値となります。

引数リストにオプション `--` が含まれている場合、引数の解析はそこで停止し、後続の引数はオプションに似ていても通常の引数として返されます。したがって、コマンドラインでは

$ ./foo.scm --apples "Granny Smith" -- --blimp Goodyear

`getopt-long` は `--apples` オプションを「Granny Smith」という値を持つものとして認識しますが、`--blimp` はオプションとして扱いません。文字列 `--blimp` と `Goodyear` は通常の引数文字列として返されます。

* * *

次へ: [`option-ref` のリファレンス ドキュメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#option_002dref-Reference)、前: [想定されるコマンドライン フォーマット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Command-Line-Format)、上: [(ice-9 getopt-long) モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.4.4 `getopt-long` のリファレンスドキュメント [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reference-Documentation-for-getopt_002dlong)

Scheme 手順: **getopt-long** 引数文法 \[#:stop-at-first-non-option #f\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-getopt_002dlong)

引数args（文字列のリストである必要があります）で指定されたコマンドラインを、オプション指定の文法に従って解析します。

文法引数は、以下の形式のリストである必要があります。

`((オプション(プロパティ値) …) …)`

ここで、各オプションは長いオプションを表す記号ですが、先頭の2つのハイフンは含まれません（たとえば、オプション名が`--version`の場合は`version`）。

各オプションには、任意の数のプロパティ/値のペアのリストが存在する可能性があります。ペアの順序は重要ではありませんが、各プロパティはプロパティリストに一度しか出現できません。以下の表に、使用可能なプロパティを示します。

（1文字の文字）

`-char` は、`--option` と同等の1文字のオプションとして受け入れられます。これは、従来のUnixスタイルのフラグを指定する方法です。

（必須？ブール値）

bool が true の場合、このオプションは必須です。`getopt-long` は、引数にこのオプションが見つからない場合、エラーを発生させます。

`(値 bool)`

bool型が`#t`の場合、オプションは値を受け入れます。`#f`の場合は受け入れません。また、シンボル`optional`の場合は、オプションは値の有無にかかわらず引数に出現する可能性があります。

`(述語関数)`

オプションが値を受け入れる場合（つまり、このオプションに `(value #t)` を指定した場合）、`getopt-long` はその値に func を適用し、`#f` が返された場合は例外をスローします。func は文字列を受け取り、ブール値を返すプロシージャである必要があります。文法に合わせるために、準引用符を使用する必要があるかもしれません。

`#:stop-at-first-non-option` キーワードに真の値を指定すると、`getopt-long` はコマンドラインで最初の非オプションに到達した時点で停止します。つまり、オプション自体でもオプションの値でもない最初の単語に到達した時点で停止します。その単語以降のコマンドラインの内容はすべて、非オプション引数として返されます。

`getopt-long`のargsパラメータは、`command-line`が返すような文字列のリストであることが想定されており、最初の要素はコマンド名です。そのため、`getopt-long`はargsの最初の要素を無視し、2番目の要素から引数の解釈を開始します。

`getopt-long` は、以下のいずれかの条件が満たされた場合にエラーを通知します。

* オプション文法の構文が無効です。
引数リスト内のオプションのいずれかが、文法で指定されていませんでした。
* 必須オプションが省略されています。
引数を必要とするオプションに引数が指定されませんでした。
* 引数を受け付けないオプションでも引数を受け取ることができます（これは長いオプション構文 `--opt=value` を使用した場合にのみ可能です）。
* オプション述語が失敗しました。

`#:stop-at-first-non-option` は、`guild [--help | --version] [script [script-options]]` や `cvs [general-options] command [command-options]` のようなコマンドライン呼び出しで役立ちます。これらのコマンドでは、オプションが 2 つのレベルで存在します。1 つは汎用的で外側のコマンドによって理解されるオプション、もう 1 つは呼び出される特定のスクリプトまたはコマンドに固有のオプションです。このような場合に `getopt-long` を使用するには、2 回呼び出します。まず `#:stop-at-first-non-option #t` を指定して、汎用オプションを解析し、目的のスクリプトまたはサブコマンドを識別します。次に、最初の汎用コマンドの単語を切り取った後、スクリプトまたはサブコマンド固有のオプション文法を指定して、それらの固有のオプションを処理します。

* * *

前へ: [`getopt-long` のリファレンス ドキュメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong-Reference)、上へ: [(ice-9 getopt-long) モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.4.5 `option-ref` のリファレンスドキュメント [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Reference-Documentation-for-option_002dref)

スキーム手順: **option-ref** オプションキー default [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-option_002dref)

指定されたキーという名前のコマンドラインオプションを検索し、見つかった場合はその値を返します。オプションに値が指定されていない場合は、`#t` を返します。オプションが指定されていない場合は、デフォルト値を返します。オプションは、`getopt-long` の呼び出し結果である必要があります。

`option-ref` は常に成功し、コマンドラインから要求されたオプション値を返すか、デフォルト値を返します。

特殊キー「'()」を使用すると、オプション以外のすべての引数のリストを取得できます。

* * *

次へ: [R6RS サポート](https://doc.guix.gnu.org/guile/latest/en/guile.html#R6RS-Support)、前: [ (ice-9 getopt-long) モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#getopt_002dlong)、上: [Guile モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Modules) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
