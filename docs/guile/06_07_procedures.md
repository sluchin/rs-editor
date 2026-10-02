### 6.7 手順 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-1)

* [ラムダ: 基本的なプロシージャの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lambda)
* [プリミティブプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Primitive-Procedures)
* [コンパイル済みプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiled-Procedures)
* [オプションの引数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Optional-Arguments)
* [Case-lambda](https://doc.guix.gnu.org/guile/latest/en/guile.html#Case_002dlambda)
* [高階関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Higher_002dOrder-Functions)
* [プロシージャのプロパティとメタ情報](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedure-Properties)
* [セッター付きプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-with-Setters)
* [インライン化可能な手続き](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inlinable-Procedures)

* * *

次へ: [プリミティブ手続き](https://doc.guix.gnu.org/guile/latest/en/guile.html#Primitive-Procedures)、上へ: [手続き](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.1 ラムダ: 基本的なプロシージャの作成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lambda_003a-Basic-Procedure-Creation)

`lambda` 式はプロシージャに評価されます。`lambda` 式が評価されるときに有効な環境は、新しく作成されたプロシージャ内に閉じ込められます。これは _クロージャ_ と呼ばれます ([クロージャの概念](https://doc.guix.gnu.org/guile/latest/en/guile.html#About-Closure) を参照)。

`lambda` で作成されたプロシージャが実引数とともに呼び出されると、プロシージャ内の環境は、仮引数リストで指定された変数を新しい場所にバインドし、実引数をこれらの場所に格納することによって拡張されます。その後、`lambda` 式の本体が順次評価されます。プロシージャ本体の最後の式の評価結果が、プロシージャ呼び出しの結果となります。

以下の例では、`lambda` を使用してプロシージャを作成する方法と、これらのプロシージャで何ができるかを示します。

(lambda (x) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) xx)) ⇒ a [procedure](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure)
((lambda (x) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) xx)) 4) ⇒ 8

プロシージャを作成する際に有効な環境がプロシージャ内に含まれるという事実は、次の例で示されています。

(define add4
(let ((x 4))
(lambda (y) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) xy))))
(add4 6) ⇒ 10

構文: **ラムダ** 形式体の本体 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lambda-1)

形式引数は、次の表に示すような形式引数リストである必要があります。

`(変数1 …)`

この手続きは固定数の引数を取ります。手続きが呼び出されると、引数は仮変数用に新しく作成された場所に格納されます。

`変数`

この手続きは任意の数の引数を受け取ります。手続きが呼び出されると、実際の引数のシーケンスがリストに変換され、仮変数用に新しく作成された場所に格納されます。

`(変数1 … 変数n . 変数n+1)`

最後の変数の前にスペースで区切られたピリオドがある場合、この手続きは、ピリオドの前の仮引数の数である n 個以上の変数を受け取ります。ピリオドの前には少なくとも 1 つの引数が必要です。最初の n 個の実引数は、最初の n 個の仮引数用に新しく割り当てられた場所に格納され、残りの実引数のシーケンスはリストに変換されて、最後の仮引数の場所に格納されます。実引数がちょうど n 個の場合、空のリストが最後の仮引数の場所に格納されます。

変数または変数n+1内のリストは常に新しく作成され、必要に応じてプロシージャがそれを変更できます。これは、プロシージャが`apply`を介して呼び出された場合でも同様で、リスト引数の必要な部分がコピーされます（[オンザフライ評価のプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fly-Evaluation)を参照）。

本体は、プロシージャが呼び出されたときに順番に評価される一連のScheme式です。

* * *

次へ: [コンパイル済みプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiled-Procedures)、前: [ラムダ: 基本プロシージャの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lambda)、上: [プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.2 プリミティブプロシージャ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Primitive-Procedures-1)

C言語で記述されたプロシージャは、引数の型が`SCM`のみで、戻り値も`SCM`であれば、Schemeから使用できるように登録できます。登録（`scm_c_make_gsubr`）と定義（`scm_define`）のプロセスを組み合わせた`scm_c_define_gsubr`が最も便利なメカニズムとなるでしょう。

関数: `SCM` **scm\_c\_make\_gsubr** `(const char *name, int req, int opt, int rst, fcn)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmake_005fgsubr)

C言語のプロシージャfcnを「subr」（Schemeから呼び出し可能なプリミティブサブルーチン）として登録します。登録されたサブルーチンは指定された名前に関連付けられますが、環境バインディングは作成されません。引数req、opt、rstは、それぞれ必須引数、オプション引数、および「残りの」引数の数を指定します。これらの引数の合計数は、fcnへの実際の引数の数と一致する必要がありますが、10を超えることはできません。残りの引数の数は0または1である必要があります。`scm_c_make_gsubr`は、プロシージャの「ハンドル」である`SCM`型の値を返します。

関数: `SCM` **scm\_c\_define\_gsubr** `(const char *name, int req, int opt, int rst, fcn)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fdefine_005fgsubr)

上記の`scm_c_make_gsubr`と同様にCプロシージャfcnを登録し、さらに`scm_define`を使用して「現在の環境」にプロシージャのトップレベルSchemeバインディングを作成します。`scm_c_define_gsubr`は`scm_c_make_gsubr`と同様にプロシージャのハンドルを返しますが、通常はそれ以上の処理は必要ありません。

Scheme から C で記述されたプロシージャを呼び出すための別のインターフェースについては、[Foreign Functions](https://doc.guix.gnu.org/guile/latest/en/guile.html#Foreign-Functions) を参照してください。

* * *

次へ: [オプション引数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Optional-Arguments)、前: [プリミティブ手続き](https://doc.guix.gnu.org/guile/latest/en/guile.html#Primitive-Procedures)、上: [手続き](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.3 コンパイル済みプロシージャ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiled-Procedures-1)

[Lambda: 基本プロシージャ作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lambda)で説明されている評価戦略では、プロシージャがどのように解釈されるかが説明されています。解釈は展開された Scheme ソース コードに対して直接動作し、評価器を再帰的に呼び出してネストされた式の値を取得します。

ただし、ほとんどのプロシージャはコンパイルされます。これは、Guileがプロシージャを実行するたびに必要な処理を決定するために、事前に何らかの計算を行っていることを意味します。コンパイルされたプロシージャは、インタプリタ型のプロシージャよりも高速に実行されます。

コンパイル済みプロシージャは通常、ファイルの読み込みによって生成されます。Guile は、ファイルがコンパイルされていない、またはコンパイル済みファイルが古いことを検出した場合、読み込み時にファイルのコンパイルを試み、結果をディスクに保存します。プロシージャは実行時にもコンパイルできます。実行時コンパイルの詳細については、[Scheme コードの読み込みと評価](https://doc.guix.gnu.org/guile/latest/en/guile.html#Read_002fLoad_002fEval_002fCompile) を参照してください。

コンパイル済みプロシージャ（プログラムとも呼ばれます）は、プロシージャを操作するすべてのプロシージャに応答します。`procedure?`、`procedure-name`などにプログラムを渡すことができます（[プロシージャのプロパティとメタ情報](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedure-Properties)を参照）。さらに、プログラムの低レベルな詳細情報を取得するためのアクセサもいくつか用意されています。

ほとんどの人はこのセクションで説明するルーチンを使用する必要はないでしょうが、文書化しておくことは良いことです。ただし、まず適切なモジュールを含める必要があります。

(use-modules (system vm program))

Scheme手順: **program?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_003f)

C 関数: **scm\_program\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprogram_005fp)

objがコンパイル済みプロシージャの場合は`#t`を返し、それ以外の場合は`#f`を返します。

スキーム手順: **program-code** プログラム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002dcode)

C 関数: **scm\_program\_code** (プログラム) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprogram_005fcode)

プログラムのエントリのアドレスを整数として返します。このアドレスは主に`(system vm debug)`内のプロシージャで役立ちます。

スキームプロシージャ: **program-num-free-variable** プログラム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002dnum_002dfree_002dvariable)

C 関数: **scm\_program\_num\_free\_variables** (program) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprogram_005fnum_005ffree_005fvariables)

このプログラムによって捕捉された自由変数の数を返します。

Scheme 手順: **program-free-variable-ref** program n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002dfree_002dvariable_002dref)

C 関数: **scm\_program\_free\_variable-ref** (program, n) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprogram_005ffree_005fvariable_002dref)

Scheme 手順: **program-free-variable-set!** program n val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002dfree_002dvariable_002dset_0021)

C 関数: **scm\_program\_free\_variable\_set\_x** (program, n, val) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprogram_005ffree_005fvariable_005fset_005fx)

プログラムの自由変数へのアクセサ。取得される値の中には、実際には変数「ボックス」に格納されているものもあります。詳細については、「変数とVM」を参照してください。

ユーザーは、自分が本当に賢いと思わない限り、返された値を変更してはならない。

Scheme手順: **program-sources** プログラム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002dsources)

スキーム手順: **source:addr** source [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-source_003aaddr)

スキーム手順: **ソース:行** ソース [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-source_003aline)

スキーム手順: **source:column** source [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-source_003acolumn)

Scheme Procedure: **source:file** source [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-source_003afile)

プログラムとそのアクセサーのソースコード位置を示す注釈。

ソース位置情報はコンパイラを介して伝播し、最終的にプログラムのメタデータにシリアル化されます。この情報は、プログラムのオブジェクトコード内の命令ポインタのオフセットをキーとして使用されます。具体的には、命令の_直後_にある`ip`をキーとして使用されるため、バックトレースによって実行中の呼び出しのソース位置を特定できます。

スキーム手順: **program-arities** プログラム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002darities)

C 関数: **scm\_program\_arities** (プログラム) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprogram_005farities)

スキーム手順: **program-arity** プログラム ip [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002darity)

Scheme Procedure: **arity:start** arity [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-arity_003astart)

スキーム手順: **arity:end** arity [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-arity_003aend)

スキーム手順: **arity:nreq** arity [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-arity_003anreq)

スキーム手順: **arity:nopt** arity [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-arity_003anopt)

スキーム手順: **arity:rest?** arity [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-arity_003arest_003f)

スキーム手順: **arity:kw** arity [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-arity_003akw)

スキーム手順: **arity:allow-other-keys?** arity [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-arity_003aallow_002dother_002dkeys_003f)

プログラムの「アリティ」を表すためのアクセサ。

通常、プロシージャは引数を 1 つ持ちます。たとえば、`(lambda (x) x)` は必須引数を 1 つだけ取ります。必須引数の数は、`(arity:nreq (program-arities (lambda (x) x)))` で確認できます。同様に、`arity:nopt` はオプション引数の数を取得し、`arity:rest?` はプロシージャに残り引数がある場合は true を返します。

`arity:kw` は、プロシージャにキーワード引数がある場合、`(kw . idx)` ペアのリストを返します。idx は idx 番目のローカル変数を指します。詳細については、[変数と VM](https://doc.guix.gnu.org/guile/latest/en/guile.html#Variables-and-the-VM) を参照してください。最後に、`arity:allow-other-keys?` は、他のキーが許可されている場合は true を返します。詳細については、[オプション引数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Optional-Arguments) を参照してください。

では、`arity:start`と`arity:end`はどうでしょうか？これらは、プログラムのバイトコード内で、指定されたアリティが有効なバイト範囲を返します。実際、プロシージャは複数のアリティを持つことができます。「プロシージャのアリティとは何か」という問いは、これらの`arity:start`と`arity:end`の値によって区切られた、プログラムの特定の箇所でのみ意味を持ちます。

Scheme手順: **program-arguments-alist** プログラム \[ip\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002darguments_002dalist)

プログラムが受け入れる引数を記述した連想リストを返します。情報が取得できない場合は `#f` を返します。

現在定義されている alist キーは、「required」、「optional」、「keyword」、「allow-other-keys?」、および「rest」です。例:

(プログラム引数リスト)
(lambda\* (ab #:オプション c #:キー (d 1) #:レスト e)
#t)) ⇒
（（必須。（ab））
（オプション。（c））
(キーワード . ((#:d . 4)))
(allow-other-keys? . #f)
（残りのd）

Scheme Procedure: **program-lambda-list** program \[ip\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-program_002dlambda_002dlist)

プログラムの引数をラムダリストとして返します。情報が利用できない場合は `#f` を返します。

例えば：

(プログラムラムダリスト)
(lambda\* (ab #:オプション c #:キー (d 1) #:レスト e)
#t)) ⇒

* * *

次へ: [Case-lambda](https://doc.guix.gnu.org/guile/latest/en/guile.html#Case_002dlambda)、前: [Compiled Procedures](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiled-Procedures)、上: [Procedures](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.4 オプションの引数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Optional-Arguments-1)

R5RSで定義されているSchemeプロシージャは、固定数の実引数を扱うことも、固定数の実引数の後に任意の数の追加引数を扱うこともできます。可変引数を持つプロシージャを作成することは便利ですが、残念ながら、長さが変化する引数リストを扱うための構文上の手段は少々不便です。固定数の引数には名前を付けることができますが、残りの（オプションの）引数は値のリストとしてのみ参照できます（[Lambda: 基本的なプロシージャの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lambda)を参照）。

このため、Guileは`lambda`の拡張機能である`lambda*`を提供しており、ユーザーはオプション引数とキーワード引数を持つプロシージャを定義できます。さらに、Guileの仮想マシンは、オプション引数とキーワード引数のディスパッチを低レベルでサポートしています。オプション引数とキーワード引数を持つプロシージャへの呼び出しは、レストリストを割り当てることなく、低コストで実行できます。

* [ラムダ式と定義式。](https://doc.guix.gnu.org/guile/latest/en/guile.html#lambda_002a-and-define_002a)
* [(ice-9 optargs)](https://doc.guix.gnu.org/guile/latest/en/guile.html#ice_002d9-optargs)

* * *

次へ: [(ice-9 optargs)](https://doc.guix.gnu.org/guile/latest/en/guile.html#ice_002d9-optargs)、上へ: [Optional Arguments](https://doc.guix.gnu.org/guile/latest/en/guile.html#Optional-Arguments) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.4.1 lambda\* と define\*。[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#lambda_002a-and-define_002a_002e)

`lambda*` は `lambda` に似ていますが、オプション引数とキーワード引数を許可するための拡張機能が追加されています。

ライブラリ構文: **lambda\*** (\[var…\]
[#:オプションの変数定義…]
\[#:key vardef… \[#:allow-other-keys\]\]
\[#:rest var | . var\])
body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lambda_002a)

  

`#:optional` および `#:key` で指定されたオプション引数および/またはキーワード引数を受け取るプロシージャを作成します。例:

([lambda\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lambda_002a) (ab #:optional cd . e) '())

は、固定引数 a と b、オプション引数 c と d、および残りの引数 e を持つ手続きです。呼び出し時にオプション引数が省略された場合、それらの変数は `#f` にバインドされます。

同様に、`define*` は `lambda*` を使用してプロシージャを定義するための構文糖衣です。

`lambda*` はキーワード引数を持つプロシージャを作成することもできます。例えば、次のように定義されたプロシージャです。

(define\* (sir-yes-sir #:key action how-high)
([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) action how-high))

`(sir-yes-sir #:action 'jump)`、`(sir-yes-sir #:how-high 13)`、`(sir-yes-sir #:action 'lay-down #:how-high 0)`、または単に`(sir-yes-sir)`のように呼び出すことができます。キーワードとして指定された引数は値にバインドされます（指定されていない引数は`#f`です）。

オプション引数とキーワード引数には、呼び出し時に引数がない場合に取得するデフォルト値も設定できます。これは、変数名と式の2要素リストを指定することで行います。たとえば、

(define\* (frob foo #:optional (bar 42) #:key (baz 73))
([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) foo bar baz))

fooは固定引数、barはデフォルト値42のオプション引数、bazはデフォルト値73のキーワード引数です。デフォルト値式は、必要になるまで、またプロシージャが呼び出されるまで評価されません。

通常、呼び出しに`#:key`で指定されたキーワード以外のキーワードが含まれている場合はエラーになりますが、定義に（キーワード引数の宣言の後に）`#:allow-other-keys`を追加すると、不明なキーワードは無視されます。

呼び出しでキーワードが2回指定されている場合、最後の値が使用されます。たとえば、

(define\* (flips #:key (heads 0) (tails 0))
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) ([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) 表 裏)))

（コインを投げる回数：表37回、裏42回、表99回）
⊣ (99 42)

`#:rest` は、ドット構文のレスト引数の同義語です。引数リスト `(a . b)` と `(a #:rest b)` は、あらゆる点で同等です。これは、DSSSL、MIT-Scheme、Kawa などとの類似性を高めるため、また他の Lisp 方言からの移行者のために提供されています。

`#:key` がレスト引数と一緒に使用される場合、呼び出しのキーワードパラメータはすべてレストリストに残ります。これは Common Lisp と同じです。たとえば、

(([lambda\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lambda_002a) (#:key (x 0) #:allow-other-keys #:rest r)
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) r))
#:x 123 #:y 456)
⊣ (#:x 123 #:y 456)

`#:optional` と `#:key` は、左から右へ順にバインディングを確立します。これは、デフォルト式が以前のパラメーターを参照できることを意味します。たとえば、

([lambda\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lambda_002a) (start #:optional (end ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) 10 start)))
(do ((i start ([1+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002b-1) i)))
(([\>](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e) i end))
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) i)))

この左から右へのスコープ規則の例外は、レスト引数です。レスト引数がある場合、それはオプション引数の後、キーワード引数の前にバインドされます。

* * *

前へ: [lambda\* と define\*。](https://doc.guix.gnu.org/guile/latest/en/guile.html#lambda_002a-and-define_002a)、上へ: [オプション引数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Optional-Arguments) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.4.2 (ice-9 optargs) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#g_t_0028ice_002d9-optargs_0029)

Guile 2.0 より前は、`lambda*` と `define*` は、レストリスト引数を処理するマクロを使用して実装されていました。これは、オプション引数を持つプロシージャを呼び出す際に、プロシージャ呼び出しごとにレストリストを割り当てる必要があったため、最適な方法ではありませんでした。Guile 2.0 では、オプション引数とキーワード引数を Guile のコアに取り込むことで、この状況が改善されました。

しかし、リストがあり、そこからオプション引数やキーワード引数を解析したい場合もあります。Guile の `(ice-9 optargs)` には、そのような作業を支援するマクロがいくつか用意されています。

構文 `let-optional` と `let-optional*` は、残りの引数リストを分割代入し、リストの各要素に名前を付けるためのものです。`let-optional` はすべての変数を同時にバインドしますが、`let-optional*` は `let` と `let*` と同様に、それらを順次バインドします ([ローカル変数バインディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Bindings) を参照)。

ライブラリ構文: **let-optional** rest-arg (binding …) body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-let_002doptional)

ライブラリ構文: **let-optional\*** rest-arg (binding …) body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-let_002doptional_002a)

これら 2 つのマクロは、非常に Scheme らしいオプション引数インターフェースを提供し、複雑な構文は導入しません。これらは同名の scsh マクロと互換性がありますが、若干拡張されています。それぞれのバインディングは、var または `(var default-value)` のいずれかの形式になります。rest-arg は、これらが使用されるプロシージャの rest-argument である必要があります。rest-arg の項目は、指定された変数名に順次バインドされます。rest-arg がなくなると、残りの var はデフォルト値にバインドされるか、デフォルト値が指定されていない場合は `#f` にバインドされます。rest-arg は、rest-arg に残っていたものにバインドされたままになります。

変数をバインドした後、式 body1 body2 … が順番に評価されます。

同様に、`let-keywords` と `let-keywords*` はキーワード形式の引数リストから値を抽出し、ローカル変数をそれらの値またはデフォルト値にバインドします。

ライブラリ構文: **let-keywords** args allow-other-keys? (binding …) body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-let_002dkeywords)

ライブラリ構文: **let-keywords\*** args allow-other-keys? (binding …) body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-let_002dkeywords_002a)

args は評価され、`(#:keyword1 value1 #:keyword2 value2 …)` の形式のリストが返されます。バインディングは変数とデフォルト式で、変数はキーワード値から (名前で) 設定されます。次に body1 body2 … の形式が評価され、最後のものが結果となります。例を挙げると構文が最も明確になります。

(define args '(#:xyzzy "hello" #:foo "world"))

(let-keywords args #t
((foo "fooのデフォルト値")
(bar (string-append "default" "for" "bar")))
(fooを表示)
（画面 "、 "）
（表示バー）
⊣世界、defaultforbar

`foo` のバインディングは、`args` 内の `#:foo` キーワードから取得されます。しかし、`bar` のバインディングは `let-keywords` のデフォルト値になります。これは、`args` に `#:bar` がないためです。

allow-other-keys? は評価され、引数リストに未知のキーワードを許可するかどうかを制御します。true の場合、他のキーは無視されます (例の `#:xyzzy` など)。`#f` の場合は、未知のものに対してエラーがスローされます。

`(ice-9 optargs)` は、最新の Guile コーディングではあまり役に立たないものの、依然としてサポートされている `define*` の便利な機能もいくつか提供しています。`define*-public` は `define-public` の `lambda*` バージョンです。`defmacro*` と `defmacro*-public` は、引数リストの処理機能が向上したマクロを定義するために存在します。`-public` バージョンは、プロシージャ/マクロを定義するだけでなく、現在のモジュールからエクスポートします。

ライブラリ構文: **define\*-public** 形式定義 body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002a_002dpublic)

`define*`と`define-public`を組み合わせたような感じ。

ライブラリ構文: **defmacro\*** 名前 形式関数 body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-defmacro_002a)

ライブラリ構文: **defmacro\*-public** 名前 形式関数 body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-defmacro_002a_002dpublic)

これらは、`defmacro` および `defmacro-public` とほぼ同じですが、`lambda*` スタイルの拡張パラメータリストを受け取ります。このリストでは、`#:optional`、`#:key`、`#:allow-other-keys`、および `#:rest` が通常のセマンティクスで許可されます。オプション引数を持つマクロの例を以下に示します。

([defmacro\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-defmacro_002a) transmogrify (a #:optional b)
（a 1）

* * *

次へ: [高階関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Higher_002dOrder-Functions)、前: [オプション引数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Optional-Arguments)、上: [プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.5 Case-lambda [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Case_002dlambda-1)

R5RSのレスト引数は確かに便利で汎用性が高いものの、必ずしも目的を達成するための最も適切で効率的な手段とは限りません。例えば、オプション引数の問題に関しては、レスト引数付きの`lambda`よりも`lambda*`の方がはるかに優れた解決策となります。

同様に、`case-lambda` は、1 つのプロシージャに複数の役割 (または 3 つの役割など) を担わせたい場合に、残りのリストを cons するペナルティなしにうまく機能します。

例えば：

(定義 (make-accum n)
([case-lambda](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-case_002dlambda-1)
(() n)
((m) ([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) n ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) nm)) n)))

((make-accum 20)を定義する)
(a) ⇒ 20
(a 10) ⇒ 30
(a) ⇒ 30

`case-lambda`形式が返す値は、実引数の数と各節内の仮引数を順番に照合する手続きです。最初に一致した節が選択され、実引数リストから対応する値が節内の変数名にバインドされ、節の本体が評価されます。一致する節がない場合は、エラーが通知されます。

`case-lambda` 形式の構文は、次の EBNF 文法で定義されています。_Formals_ は、`lambda` と同様に、形式引数リストを意味します ([Lambda: 基本手続きの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lambda) を参照)。

<case-lambda>
--> (case-lambda <case-lambda-clause>\*)
--> (case-lambda <docstring> <case-lambda-clause>\*)
<case-lambda-clause>
--> (<formals> <definition-or-command>\*)
<形式的な表現>
--> (<識別子>\*)
| (<識別子>\* .<識別子>)
| <識別子>

`case-lambda` では、レストリストが役立つ場合があります。

(定義プラス)
([case-lambda](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-case_002dlambda-1)
「すべての引数の合計を返します。」
(() 0)
（（a）a）
((ab) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ab))
((ab . rest) ([apply](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-apply) plus ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ab) rest))))
（プラス 1 2 3）⇒ 6

また、補足として、Guile では `case-lambda*` も定義されています。これは `case-lambda` と似ていますが、`lambda*` 句を使用します。`case-lambda*` 句は、引数が必須引数を満たし、かつオプション引数や残りの引数に対して多すぎない場合に一致します。

`case-lambda*`でもキーワード引数は使用できますが、それらは「マッチング」動作には寄与せず、必須引数、オプション引数、および残りの引数との相互作用は予期せぬものになる可能性があります。

`case-lambda*`（および特殊なケースである`case-lambda`）の目的においては、句は、必要な引数が十分あり、位置引数が多すぎない場合に一致します。必要な引数とは、`#:optional`、`#:key`、および`#:rest`引数より前の引数です。位置引数とは、必要な引数とオプション引数を合わせたものです。

`#:key` または `#:rest` 引数がない場合、位置引数が多すぎる可能性があるのは容易に想像できます。例えば、オプション引数を含めて 4 つの引数しか受け付けない関数に 5 つの引数を渡す場合などです。`#:rest` 引数がある場合は、位置引数が多すぎるということはありません。句に必要な引数が十分に揃っているアプリケーションであれば、`#:key` 引数があってもその句に一致します。

それ以外の場合、`#:key` 引数を持つ句（かつ `#:rest` 引数を持たない句）への適用では、必須引数が十分あり、かつ必須引数とオプション引数をバインドした後の次の引数（存在する場合）がキーワードである場合にのみ、句が一致します。効率上の理由から、Guile は現在、キーワード引数をマッチングアルゴリズムに含めることができません。句は位置引数のみに基づいて一致し、特定のキーワードを関数が持つキーワード引数の利用可能なセットと比較することによって一致するわけではありません。

以下にいくつかの例を示します。

(f を定義する)
(case-lambda\*
((a #:オプション b) '句-1)
((a #:オプション b #:キー c) '句-2)
((a #:キー d) 'clause-3)
((#:キー e #:レスト f) 'clause-4)))

(f) ⇒ 条項4
(f 1) ⇒ 条項-1
(f) ⇒ 条項4
(f #:e 10) 条項-1
(f 1 #:foo) 条項-1
(f 1 #:c 2) 条項-2
(f #:a #:b #:c #:d #:e) 条項-4

;; 条項2は、条項3が一致するものすべてに一致します。
(f 1 #:d 2) ⇒ エラー: 2 節のキーワード引数が不正です

句は順番に照合され、最初に一致した句が採用されることを忘れないでください。これにより、`f #:e 10` の場合のように、キーワードが必須引数にバインドされる可能性があります。

* * *

次へ: [手続きのプロパティとメタ情報](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedure-Properties)、前: [ケースラムダ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Case_002dlambda)、上: [手続き](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.6 高階関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Higher_002dOrder-Functions-1)

関数型プログラミング言語であるSchemeでは、高階関数、すなわち関数を引数として受け取り、かつ／または関数を返す関数を定義できます。他の手続きから手続きを導出するためのユーティリティが提供されており、以下で説明します。

Scheme手順: **const**値[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-const)

任意の数の引数を受け取り、値を返すプロシージャを返します。

([procedure?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_003f) ([const](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-const) 3)) ⇒ #t
(([const](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-const) 'hello)) ⇒ hello
(([const](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-const) 'hello) 'world) ⇒ hello

Scheme プロシージャ: **negate** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-negate)

proc と同じ引数数を持つプロシージャを返し、そのプロシージャの結果の `not` を返します。

([procedure?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_003f) ([negate](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-negate) [number?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f))) ⇒ #t
(([negate](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-negate) [odd?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-odd_003f)) 2) ⇒ #t
(([negate](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-negate) [real?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_003f)) 'dream) ⇒ #t
(([negate](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-negate) [string-prefix?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dprefix_003f)) "GNU" "GNU Guile")
⇒ #f
([filter](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-filter) ([negate](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-negate) [number?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f)) '(a 2 "b"))
⇒（a "b")

Scheme手順: **compose** proc1 proc2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compose)

プロシージャproc1と、最後のproc引数が最初に適用され、proc1が最後に適用されるように、プロシージャproc2…を合成し、結果として得られるプロシージャを返します。指定されたプロシージャは、引数の数が互換性のあるものでなければなりません。

([procedure?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_003f) ([compose](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compose) [1+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002b-1) [1-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002d-1))) ⇒ #t
(([compose](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compose) [sqrt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sqrt) [1+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002b-1) [1+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002b-1)) 2) ⇒ 2.0
(([compose](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compose) [1+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002b-1) [sqrt](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sqrt)) 3) ⇒ 2.73205080756888
([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) ([compose](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compose) [1+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002b-1)) [1+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-1_002b-1)) ⇒ #t

(([compose](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compose) [zip](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-zip) [unzip2](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unzip2)) '((1 2) (ab)))
⇒ ((1 2) (ab))

Scheme Procedure: **identity** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-identity)

Xを返します。

Scheme Procedure: **and=>** value proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-and_003d_003e)

値が`#f`の場合は`#f`を返します。それ以外の場合は`(proc value)`を返します。

* * *

次へ: [セッター付きプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-with-Setters)、前: [高階関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Higher_002dOrder-Functions)、上: [プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.7 プロシージャのプロパティとメタ情報 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedure-Properties-and-Meta_002dinformation)

プロシージャの実行に厳密に必要な情報に加えて、プロシージャにはその他の関連情報が含まれる場合があります。たとえば、プロシージャ名はプロシージャ自体の情報ではなく、プロシージャに関する情報です。このメタ情報は、プロシージャのプロパティインターフェイスからアクセスできます。

スキームプロシージャ: **procedure?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_003f)

C 関数: **scm\_procedure\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprocedure_005fp)

objがプロシージャの場合は`#t`を返します。

Scheme手順: **thunk?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-thunk_003f)

C 関数: **scm\_thunk\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fthunk_005fp)

obj が引数なしで呼び出し可能なプロシージャである場合は、`#t` を返します。プロシージャが受け入れる引数の詳細については、[コンパイル済みプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiled-Procedures) を参照してください。

プロシージャプロパティとは、プロシージャに関連付けられた一般的なプロパティです。これには、プロシージャ名やデバッグヒントなどの関連情報が含まれます。

手続きのプロパティを関連付ける最も一般的な方法は、プログラムによって行うことです。

Scheme プロシージャ: **procedure-property** proc key [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_002dproperty)

C 関数: **scm\_procedure\_property** (proc, key) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprocedure_005fproperty)

名前がキーのprocのプロパティを返します。見つからない場合は`#f`を返します。

Scheme プロシージャ: **set-procedure-property!** proc key value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dprocedure_002dproperty_0021)

C 関数: **scm\_set\_procedure\_property\_x** (proc, key, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fprocedure_005fproperty_005fx)

プロシージャのkeyという名前のプロパティに値を設定します。

しかし、より効率的なインターフェースがあり、定数プロパティをコンパイル済みバイナリに埋め込む際に、プロパティが要求されるまでオーバーヘッドが発生しません。ラムダ式の本体の先頭の非末尾要素で、ペアのリテラルベクトルであるものは、プロシージャプロパティの宣言として解釈されます。これは、次の例で最も分かりやすく説明できます。

(プロシージャを定義する)
(ラムダ引数)
#((a . "hey") (b . "ho")) ;; プロシージャのプロパティ!
42))
(procedure-property proc 'a) ; ⇒ "hey"
(procedure-property proc 'b) ; ⇒ "ho"

`documentation`プロパティを宣言するための省略形があり、これはリテラルベクトルではなくリテラル文字列です。

(プロシージャを定義する)
(ラムダ引数)
「これはドキュメント文字列です。」
42))
(プロシージャプロパティ proc 'ドキュメント)
;; ⇒ "これはドキュメント文字列です。"

キーとして「documentation」を指定して「procedure-property」を呼び出すことは、「procedure-documentation」を呼び出すこととまったく同じです。同様に、「procedure-name」は「name」プロシージャプロパティと同じであり、「procedure-source」は「source」プロパティに対応します。

Scheme プロシージャ: **procedure-name** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_002dname)

C 関数: **scm\_procedure\_name** (proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprocedure_005fname)

スキームプロシージャ: **procedure-source** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_002dsource)

C 関数: **scm\_procedure\_source** (proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprocedure_005fsource)

Scheme 手順: **procedure-documentation** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_002ddocumentation)

C 関数: **scm\_procedure\_documentation** (proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprocedure_005fdocumentation)

proc の `name`、`source`、または `documentation` プロパティの値を返します。プロパティが設定されていない場合は `#f` を返します。

手続きプロパティの全体に対して作業を行うこともできます。

Scheme プロシージャ: **procedure-properties** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_002dproperties)

C 関数: **scm\_procedure\_properties** (proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprocedure_005fproperties)

procに関連付けられたプロパティを、関連付けリストとして返します。

スキームプロシージャ: **set-procedure-properties!** プロシージャリスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dprocedure_002dproperties_0021)

C 関数: **scm\_set\_procedure\_properties\_x** (proc, alist) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fprocedure_005fproperties_005fx)

procのプロパティリストをalistに設定します。

* * *

次へ: [インライン化可能なプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inlinable-Procedures)、前: [プロシージャのプロパティとメタ情報](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedure-Properties)、上: [プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.8 セッター付きプロシージャ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-with-Setters-1)

セッター付きプロシージャは、通常はデータ構造にアクセスするアクセサプロシージャと同様の動作をする特殊なプロシージャです。違いは、この種のプロシージャには、データ構造に何かを格納するためのプロシージャであるセッターが付属している点です。

セッターを持つプロシージャは、プロシージャが特別な形式 `set!` で記述されている場合に特別に扱われます。その動作は、例を見るのが一番分かりやすいでしょう。

`foo-ref` というプロシージャがあるとします。このプロシージャは、型 `foo` の値と整数の 2 つの引数を受け取ります。プロシージャは、`foo` オブジェクト内の指定されたインデックスに格納されている値を返します。`f` を、このような `foo` データ構造を含む変数とします。[13](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT13)

(foo-ref f 0) ⇒ バー
(foo-ref f 1) ⇒ braz

また、`foo-set!` という対応するセッター手続きが存在すると仮定します。

(foo-set! f 0 'bla)
(foo-ref f 0) ⇒ bla

これで、アクセサプロシージャ`foo-ref`とセッタープロシージャ`foo-set!`を指定して`make-procedure-with-setter`を呼び出すことで、セッターを持つプロシージャ`foo`という新しいプロシージャを作成できます。この新しいプロシージャを`foo`と呼びましょう。

(define foo ([make-procedure-with-setter](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dprocedure_002dwith_002dsetter) foo-ref foo-set!))

今後は、`foo` は `f` に格納されているデータ構造から読み込むためにも、その構造に書き込むためにも使用できます。

([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) (foo f 0) 'dum)
(foo f 0) ⇒ dum

スキームプロシージャ: **make-procedure-with-setter** プロシージャセッター [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dprocedure_002dwith_002dsetter)

C 関数: **scm\_make\_procedure\_with\_setter** (procedure, setter) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fprocedure_005fwith_005fsetter)

プロシージャのように動作するが、関連付けられたセッターを持つ新しいプロシージャを作成します。

スキームプロシージャ: **procedure-with-setter?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure_002dwith_002dsetter_003f)

C 関数: **scm\_procedure\_with\_setter\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprocedure_005fwith_005fsetter_005fp)

objが関連付けられたセッタープロシージャを持つプロシージャである場合は、`#t`を返します。

スキームプロシージャ: **procedure** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-procedure)

C 関数: **scm\_procedure** (proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fprocedure)

proc のプロシージャを返します。proc は適用可能な構造体である必要があります。

スキームプロシージャ: **setter** proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-setter-1)

proc のセッターを返します。セッターは、セッターを持つプロシージャ、または演算子構造体のいずれかである必要があります。

* * *

前へ: [セッター付きプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-with-Setters)、上へ: [プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.7.9 インライン化可能な手続き [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Inlinable-Procedures-1)

`define` の代わりに `define-inlinable` を使用することで、インライン化可能なプロシージャを定義できます。インライン化可能なプロシージャは通常のプロシージャと同じように動作しますが、直接呼び出しを行うと、プロシージャ本体が呼び出し元にインライン化されます。

バージョン2.0.3以降、Guileには、必要に応じて内部プロシージャの本体をインライン化できる部分評価機能が搭載されていることに留意してください。

scheme@(guile-user)> 、optimize (define (foo x)
(define (bar) (+ x 3))
(* (バー) 2)
$1 = (define foo
(lambda (#{x 94}#) (\* (+ #{x 94}# 3) 2)))

ただし、部分評価器はトップレベルのバインディングをインライン化しないため、このような状況では`define-inlinable`を使用すると便利かもしれません。

`define-inlinable` で定義されたプロシージャは、すべての直接呼び出し箇所で常にインライン化されます。これにより、コードサイズが増加する代わりに、関数呼び出しのオーバーヘッドが削減されます。さらに、インライン化されたプロシージャが再定義された場合、呼び出し元は新しい定義を透過的に使用しません。インライン化されたプロシージャをトレースしたり、ブレークポイントを設定したりすることはできません（[Traps](https://doc.guix.gnu.org/guile/latest/en/guile.html#Traps) を参照）。これらの理由から、パフォーマンスが決定的に向上することが証明されない限り、プロシージャをインライン化すべきではありません。

一般的に、インライン化を検討すべきなのは小規模な手続きのみである。大規模な手続きをインライン化しようとすると、コードサイズが増加する可能性が高いからである。さらに、大規模な手続きの場合、呼び出しオーバーヘッドの削減はほとんど意味をなさない。

Scheme構文: **define-inlinable** (名前 パラメータ …) body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002dinlinable)

名前を、パラメータ parameters とボディ body1、body2、... を持つプロシージャとして定義します。

* * *

次へ: [汎用ユーティリティ関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Utility-Functions)、前: [プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
