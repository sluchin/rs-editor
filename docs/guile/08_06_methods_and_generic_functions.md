### 8.6 メソッドとジェネリック関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions-1)

GOOPSメソッドはSchemeの手続きに似ていますが、特定の引数クラスのセットに特化しており、呼び出し時の実際の引数がメソッド定義内のクラスと一致する場合にのみ使用されます。

(define-method ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) (x <string>) (y <string>))
([string-append](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dappend) xy))

([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) "abc" "de") ⇒ "abcde"

メソッドは（他の多くのオブジェクト指向言語のように）特定のクラスに正式には関連付けられていません。なぜなら、メソッドは複数のクラスの組み合わせに特化できるからです。Lispy以外の言語でオブジェクト指向を学んだことがあるなら、グラフィック画像をサーフェスに沿って引き伸ばすメソッドは、サーフェスをパラメータとするイメージクラスのメソッドにするべきか、それとも画像をパラメータとするサーフェスクラスのメソッドにするべきかといった議論を覚えているかもしれません。GOOPSでは、単に次のように記述します。

(define-method (stretch (im <image>) (sf <surface>))
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))

そして、そのメソッドがどちらのクラスにより関連しているかという問いに答える必要はない。

同じ名前でも、異なる引数クラスのセットを持つメソッドが複数同時に存在し得る。例えば、次のようになる。

(define-method ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) (x <string>) (y <string)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
(define-method ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) (x <matrix>) (y <matrix>)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
(define-method ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) (f <fish>) (b <bicycle>)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
(define-method ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) (a <foo>) (b <bar>) (c <baz>)) [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))

汎用関数とは、プログラムが使用しようとする一連のメソッドを格納するコンテナのことである。

プログラムのソースコードを見て、どこかに `(+ xy)` という記述を見つけた場合、概念的には、その箇所でプログラムが汎用関数（この場合は識別子 `+` にバインドされた汎用関数）を呼び出していることを意味します。呼び出しが行われると、Guile は汎用関数のどのメソッドが、関数呼び出し時に渡される引数に最も適しているかを判断し、引数を仮引数としてメソッドのコードを評価します。これは汎用関数呼び出しが評価されるたびに発生します。特定のソースコード呼び出しが毎回同じメソッドを呼び出すとは限りません。

識別子を汎用関数として定義するには、`define-generic` マクロを使用します。新しいメソッドを定義するには、`define-method` マクロを使用します。`define-method` は、対象の識別子が既に汎用関数でない場合、自動的に `define-generic` を実行するため、多くの場合、明示的な `define-generic` 呼び出しは不要です。

構文: **define-generic** シンボル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002dgeneric-1)

シンボルという名前の汎用関数を作成し、それを変数シンボルにバインドします。シンボルが以前にSchemeプロシージャ（またはセッター付きプロシージャ）にバインドされていた場合、古いプロシージャ（およびセッター）は、新しい汎用関数のデフォルトプロシージャ（およびセッター）として組み込まれます。既存の汎用関数を含む、その他の以前の値はすべて破棄され、新しい空の汎用関数に置き換えられます。

構文: **define-method** (汎用パラメータ …) 本体 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002dmethod-1)

汎用関数またはアクセサのメソッドを定義します。パラメータ parameters と本体 body ....

generic は汎用関数です。generic がまだ汎用関数オブジェクトにバインドされていない変数である場合、`define-method` の展開には `define-generic` の呼び出しが含まれます。generic が `(setter generic-with-setter)` であり、generic-with-setter がまだ generic-with-setter オブジェクトにバインドされていない変数である場合、展開には `define-accessor` の呼び出しが含まれます。

各パラメータは、シンボルまたは2要素のリスト（シンボルクラス）のいずれかでなければなりません。シンボルは、このメソッドを呼び出す際に呼び出し元から提供されるパラメータにバインドされる、本体フォーム内の変数を参照します。クラスが存在する場合、このメソッドを適用できるパラメータの組み合わせを指定します。

body …はメソッド定義の本体です。

`define-method` 式は、Scheme のプロシージャ定義の形式に少し似ています。

(define (name formals ...) . body)

重要な違いは、可能な「rest」引数を除き、各仮引数にクラス名を修飾できる点です。`formal` は `(formal class)` となります。この修飾の意味は、定義されるメソッドは、対応する引数が `class` (またはそのサブクラス) のインスタンスである場合にのみ、特定の汎用関数呼び出しで適用可能になるということです。複数の仮引数がこのように修飾されている場合、メソッドは、対応する各引数がそれぞれの修飾クラスのインスタンスである場合にのみ適用可能になります。

修飾されていない仮パラメータは、クラス `<top>` によって修飾されているかのように動作することに注意してください。GOOPS では、このクラスは、プリミティブ型と GOOPS クラスの両方を含む、すべての有効な Scheme 型のスーパークラスを意味します。

例えば、汎用関数メソッドがパラメータ `(s1 <square>)` と `(n <number>)` で定義されている場合、そのメソッドは、最初のパラメータが `<square>` クラスのインスタンスであり、2 番目のパラメータが数値であるような、2 つのパラメータを持つ汎用関数の呼び出しにのみ適用されます。

* [アクセサー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessors)
* [プリミティブの拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-Primitives)
* [ジェネリクスのマージ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Merging-Generics)
* [Next-method](https://doc.guix.gnu.org/guile/latest/en/guile.html#Next_002dmethod)
* [method\* および define-method\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#method_002a-and-define_002dmethod_002a)
* [汎用関数とメソッドの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-and-Method-Examples)
* [呼び出しエラーの処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-Invocation-Errors)

* * *

次へ: [プリミティブの拡張](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-Primitives)、上: [メソッドとジェネリック関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.6.1 アクセサー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessors-1)

アクセサは汎用関数であり、汎用化された `set!` 構文でも使用できます ([セッターを使用したプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-with-Setters) を参照)。Guile は次のような呼び出しを処理します。

(set! (アクセサ引数...)値)

`args`と`value`のクラスに一致する`accessor`の最も特殊なメソッドを呼び出すことによって実現されます。`define-accessor`は、識別子をアクセサーにバインドするために使用されます。

構文: **define-accessor** シンボル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002daccessor)

シンボルという名前のアクセサを作成し、それを変数シンボルにバインドします。シンボルが以前にSchemeプロシージャ（またはセッター付きプロシージャ）にバインドされていた場合、古いプロシージャ（およびセッター）は、新しいアクセサのデフォルトプロシージャ（およびセッター）として組み込まれます。既存の汎用関数やアクセサなど、その他の以前の値はすべて破棄され、新しい空のアクセサに置き換えられます。

* * *

次へ: [ジェネリックのマージ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Merging-Generics)、前: [アクセサー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessors)、上: [メソッドとジェネリック関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.6.2 プリミティブの拡張 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-Primitives-1)

Guileの多くの基本プロシージャは、通常のC言語による実装と連携して動作する汎用関数定義を与えることで拡張できます。このようにして基本プロシージャが拡張されると、C言語による実装をデフォルトメソッドとする汎用関数のように動作します。

この拡張機能は、現在の値がプリミティブである変数に対してメソッドが定義されている場合（`define-method`呼び出しによる）、自動的に実行されます。ただし、`enable-primitive-generic!`を呼び出すことで強制的に有効にすることもできます。

プリミティブ手続き: **enable-primitive-generic!** プリミティブ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-enable_002dprimitive_002dgeneric_0021)

プリミティブ型に対して汎用関数定義の作成を強制する。

プリミティブの汎用関数定義が作成されると、`primitive-generic-generic` を使用して取得できます。

プリミティブ手続き: **primitive-generic-generic** プリミティブ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-primitive_002dgeneric_002dgeneric)

プリミティブ型の汎用関数定義を返します。

`primitive-generic-generic` は、primitive がジェネリック機能を持つプリミティブでない場合、エラーを発生させます。

* * *

次へ: [Next-method](https://doc.guix.gnu.org/guile/latest/en/guile.html#Next_002dmethod)、前: [Extending Primitives](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-Primitives)、上: [Methods and Generic Functions](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]

#### 8.6.3 ジェネリクスのマージ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Merging-Generics-1)

GOOPSの汎用関数やアクセサは、多くの場合、短く汎用的な名前を持ちます。たとえば、ベクトルパッケージがベクトルのX座標へのアクセサを提供する場合、そのアクセサは単に`x`と呼ばれることがあります。`vector:x`のように名前を付ける必要はありません。なぜなら、GOOPSは`(x obj)`のようなコードを見たときに、objがベクトルであれば`x`のベクトル固有のメソッドを呼び出すべきだと判断するからです。

しかし、ここで疑問が生じます。異なるパッケージが同じ名前の汎用関数を定義した場合、どうなるのでしょうか。例えば、2Dベクトルと3Dベクトルそれぞれに独立した2つのベクトルパッケージを使用する必要があるグラフィックパッケージを扱っているとします。両方のパッケージが`x`をエクスポートする場合、それらのパッケージを使用するコードは最終的にどうなるのでしょうか。

[重複バインディングハンドラ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Creating-Guile-Modules)では、競合するバインディングが一般的にどのように解決されるかが説明されています。ジェネリクスについては、特別な重複ハンドラ`merge-generics`があり、モジュールシステムに同じ名前のジェネリック関数をマージするように指示します。以下に例を示します。

(define-module (math 2D-vectors)
#:use-module (oop goops)
#:export (xy [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)))
		  
(define-module (math 3D-vectors)
#:use-module (oop goops)
#:export (xyz [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)))

(define-module (my-module)
#:use-module (oop goops)
#:use-module (math 2D-vectors)
#:use-module (math 3D-vectors)
#:重複（マージジェネリクス）

`(my-module)` 内の汎用関数 `x` は、インポートされた両方のモジュールの `x` のすべてのメソッドを取り込むようになります。

正確に言うと、`x` という名前の異なる汎用関数が 3 つ存在することになります。`(math 2D-vectors)` の `x`、`(math 3D-vectors)` の `x`、および `(my-module)` の `x` です。これらの関数は、興味深く動的な方法でメソッドを共有します。

説明のために、インポートされた汎用関数（`(math 2D-vectors)` および `(math 3D-vectors)` 内）を _祖先_、マージされた汎用関数（`(my-module)` 内）を _子孫_ と呼びましょう。一般的なルールとして、任意の汎用関数 G に対して、適用可能なメソッドは、G の子孫関数のメソッド、G 自身のメソッド、および G の祖先関数のメソッドの和集合から選択されます。

このように、祖先関数は子孫関数と効果的にメソッドを共有し、その逆もまた同様です。上記の例では、`(math 2D-vectors)` の `x` は `(my-module)` の `x` のメソッドを共有し、その逆もまた同様です。[33](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT33) 共有は動的であるため、子孫に新しいメソッドを追加すると、その子孫の祖先にも追加されます。

* * *

次へ: [method\* および define-method\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#method_002a-and-define_002dmethod_002a)、前: [Merging Generics](https://doc.guix.gnu.org/guile/latest/en/guile.html#Merging-Generics)、上: [Methods and Generic Functions](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "Table of内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.6.4 次のメソッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Next_002dmethod-1)

汎用関数を特定の引数セットで呼び出すと、GOOPS はそれらの引数に適用可能なすべてのメソッドのリストを作成し、メソッド定義が実際の引数型とどれだけ一致するかに基づいて順序付けします。そして、このリストの先頭にあるメソッドを呼び出します。選択されたメソッドのコードが、このリストの次のメソッドを呼び出す場合は、`next-method` を使用できます。

(define-method (test (a <integer>)) ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) 'integer (next-method)))
(define-method (test (a <number>)) ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) 'number (next-method)))
(define-method (test a) ([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) 'top))

これらの定義により、

（テスト1）⇒（整数の先頭）
(テスト 1.0) ⇒ (番号上部)
（テスト#t）⇒（トップ）

`next-method` は、単に `(next-method)` と記述することもできます。この場合、次のメソッド呼び出しの引数は暗黙的に指定され、元のメソッド呼び出しの引数と同じになります。

同じ名前のメソッドを異なる引数で呼び出したい場合（例えば、C++ のオーバーロードされたメソッドのように）、`next-method` にカスタム引数を渡すことができます。

(define-method (test (a <number>) [min](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-min) [max](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-max))
(if (and ([\>=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003e_003d) a [min](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-min)) ([<=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003c_003d) a [max](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-max)))
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "数値は範囲内です\n"))
(次の方法 a)

（テスト2 1 10）
⊣
数値は[範囲内](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-in)です
⇒
（整数値）

* * *

次へ: [汎用関数とメソッドの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-and-Method-Examples)、前: [次のメソッド](https://doc.guix.gnu.org/guile/latest/en/guile.html#Next_002dmethod)、上: [メソッドと汎用関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.6.5 メソッド*と定義メソッド* [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#method_002a-and-define_002dmethod_002a-1)

`method*` と `define-method*` は、GOOPS における `lambda*` と `define*` のバージョンです。

構文: **メソッド\*** (\[パラメータ…\]
[#:オプションの変数定義…]
\[#:key vardef… \[#:allow-other-keys\]\]
\[#:rest var | . var\])
body1 body2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-method_002a)

  

`#:optional` および `#:key` で指定されたオプション引数および/またはキーワード引数を受け取るメソッドを作成します。[lambda\* および define\* ](https://doc.guix.gnu.org/guile/latest/en/guile.html#lambda_002a-and-define_002a) を参照してください。param… は、`method` および `define-method` と同様の通常のメソッドパラメータ、つまり var または (var typespec) のいずれかです。

`define-method*` は、`method*` を使用してメソッドを定義するための構文糖衣です。たとえば、

(define-method\* (foo (a <整数>) b #:オプション c
#:キー (d 2) e
#:rest f)
([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) abcdef))

は、固定引数 a（型 `<integer>`）と b（型 `<top>`）、オプション引数 c、キーワード引数 d（デフォルト値 2）、e、および残りの引数 f を持つメソッドです。

(foo 1 'x #:d 3 'y)

戻ってきます

(1 x #f 3 #f (#:d 3 y))

cとeの値は、呼び出し時に指定されていないため、`#f`となります。指定されたキーワード引数は、`define*`と同様に、残りの引数に含まれます。

* [メソッドおよび定義メソッドにおける高度な引数処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Advanced-argument-handling-in-method-and-define_002dmethod)
* [高度な引数処理のための型ディスパッチと再定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Type-dispatch-and-redefinition-for-advanced-argument-handling)
* [メソッド内の次のメソッド呼び出し*](https://doc.guix.gnu.org/guile/latest/en/guile.html#next_002dmethod-call-in-method_002a)
* [高度な引数処理設計の選択肢](https://doc.guix.gnu.org/guile/latest/en/guile.html#Advanced-argument-handling-design-choices)

* * *

次へ: [高度な引数処理のための型ディスパッチと再定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Type-dispatch-and-redefinition-for-advanced-argument-handling)、上: [method\* と define-method\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#method_002a-and-define_002dmethod_002a) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.6.5.1 メソッドおよび定義メソッドにおける高度な引数処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Advanced-argument-handling-in-method-and-define_002dmethod-1)

一部のユーザーは、コード内で`define-method`と`define-method*`のどちらかを選択する必要がない方が自然だと感じるかもしれません。

`method*` と `define-method*` は、キーワード仮引数のない通常のメソッドでも問題なく動作することがわかりました。コンパイル時間はわずかに長くなりますが、キーワード仮引数がない場合、生成されるコードは `method` と `define-method` と同じです。

このため、標準の`method`および`define-method`バインディングを対応するキーワード形式に置き換えるモジュール（oop goops keyword-formals）を提供しています。これは次のように使用できます。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (oop goops) (oop goops keyword-formals))

または

(define-module (foo)
#:use-module (oop goops)
#:use-module (oop goops keyword-formals))

* * *

次へ: [メソッド内の next-method 呼び出し *](https://doc.guix.gnu.org/guile/latest/en/guile.html#next_002dmethod-call-in-method_002a)、前: [メソッドおよび define-method における高度な引数処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Advanced-argument-handling-in-method-and-define_002dmethod)、上: [メソッド * および define-method *](https://doc.guix.gnu.org/guile/latest/en/guile.html#method_002a-and-define_002dmethod_002a) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.6.5.2 高度な引数処理のための型ディスパッチと再定義 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Type-dispatch-and-redefinition-for-advanced-argument-handling-1)

CLOSと同様に、GOOPSは必須引数に対してのみ型ディスパッチを行い、これはメソッド*についても同様です。キーワード仮引数を持つ`method*`の場合、特殊化子のリストは、同じ数の必須引数と末尾/レスト引数を持つ対応する`method`の場合と同じになります。たとえば、特殊化子のリストは次のようになります。

(define-method\* (foo (a <整数>) b #:オプション c #:キー d)
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))

になる

(<整数> <top> . <top>)

これは、`(method (a <integer>) b) ...)` が `(method (a <integer>) b #:optional c #:key d) ...)` よりも特殊化されており、適用可能なメソッドのリストで後者よりも前に来ることを意味します。

同じ汎用関数の 2 つのメソッドは同じ特殊化リストを持つことはできません。つまり、

(define-method\* (foo (a <整数>) b #:オプション c)
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))
(define-method\* (foo (a <integer>) b . rest)
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))

2 番目の `define-method*` は再定義を引き起こし、2 番目のメソッドが最初のメソッドに置き換わります。

* * *

次へ: [高度な引数処理の設計上の選択肢](https://doc.guix.gnu.org/guile/latest/en/guile.html#Advanced-argument-handling-design-choices)、前: [高度な引数処理のための型ディスパッチと再定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Type-dispatch-and-redefinition-for-advanced-argument-handling)、上: [method\* と define-method\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#method_002a-and- define_002dmethod_002a) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.6.5.3 メソッド内の next-method 呼び出し* [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#next_002dmethod-call-in-method_002a-1)

`method*` 内の `(next-method)` 呼び出しは、仮引数のリストにある必須引数、オプション引数、および残りの引数をすべて適用可能なメソッドのリストにある次のあまり特殊化されていないメソッドに渡します。また、`method*` の呼び出しで渡された実際のキーワード引数も渡されます。

これはデフォルト値に関して以下の影響を及ぼします。

1. オプション引数Aは、より特殊化されていないメソッドの同じ位置にあるオプション引数のデフォルト値を上書きします。たとえば、<B>が<A>のサブクラスで、bが<B>のインスタンスである場合、
    
(define-method\* (foo (obj <A>) #:optional (c 1)) c)
(define-method\* (foo (obj <B>) #:optional c)) (next-method))
(foo b) ⇒ #f
    
その理由は、c は 2 番目の (最も特殊化された、最初に呼び出される) メソッドの呼び出しで既に値 `#f` を取得し、この値が `(next-method)` 呼び出しによって渡されるためです。
    
2. キーワード引数は、より特殊化されていないメソッド内の同じキーワード引数のデフォルト値を上書きしません。例:
    
(define-method\* (foo (obj <A>) #:key (c 1)) c)
(define-method\* (foo (obj <B>) #:key c)) (next-method))
(foo b) ⇒ 1
    
その理由は、最初の（特殊化されていない、つまり最後の）メソッドには、呼び出しからキーワード引数が渡されないためです。具体的には、`#:c VAL` は渡されないため、デフォルト値が使用されます。
    

ユーザーは、`next-method` に引数を渡すことで動作をカスタマイズできます。特に、現在のメソッドが高度な引数処理を使用しており、仮引数のリストにキーワードが含まれている場合、次の、より特殊化されていないメソッドが通常の `method` である場合は、キーワードを「除外」するために、`next-method` に明示的に引数を渡す必要があります。

* * *

前へ: [メソッド*内のnext-method呼び出し](https://doc.guix.gnu.org/guile/latest/en/guile.html#next_002dmethod-call-in-method_002a)、上へ: [メソッド*とdefine-method*](https://doc.guix.gnu.org/guile/latest/en/guile.html#method_002a-and-define_002dmethod_002a) [[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")] [[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")]

#### 8.6.5.4 高度な引数処理の設計上の選択肢 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Advanced-argument-handling-design-choices-1)

GOOPSメソッドにおける高度な引数処理に関して、説明に値する2つの設計上の選択肢がある。

まず、この機能を`method`と`define-method`本体に追加するか、新しい構文`method*`と`define-method*`を導入するかの選択がありました。

新しい構文を導入しない方が良い理由はいくつかあります。APIが簡素化され、ユーザーの認知負荷が軽減されるだけでなく、実装も（新しい構文も提供する場合と比べて）若干簡素化されます。また、`defmethod`が高度な引数処理をサポートするというCLOSの設計方針にも合致しています。

最終的に、以下の理由から、既存の構文を拡張するのではなく、新しい構文を導入することにしました。

1. `lambda*` および `define*` と整合します。
2. 後方互換性の保護という点では、やや優れている。
3. `method` と `define-method` の概念的なシンプルさを維持します。
4. これにより、他の実装（guile-hootなど）がよりシンプルな機能のみを提供する（`method`および`define-method`を介して）ことを選択しやすくなります。

ただし、`define-method`のみを使用したいユーザーはそうすることも可能です。[メソッドとdefine-methodにおける高度な引数処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Advanced-argument-handling-in-method-and-define_002dmethod)を参照してください。

第二に、オプション引数またはキーワード引数に対して型ディスパッチを行わないことを選択しました。理由は以下のとおりです。

1. 実装の複雑さ（およびユーザーの認知負荷）に関して、適切なバランスが取れている。
2. これはCLOSの実装とも一致しており、CLOSにもこの選択には正当な理由があったと考えられる。
3. 現在、高度な引数処理のための型ディスパッチを規定するルールの概念的枠組みや、その実装方法について明確な考えはありません。

* * *

次へ: [呼び出しエラーの処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-Invocation-Errors)、前: [method\* および define-method\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#method_002a-and-define_002dmethod_002a)、上: [メソッドと汎用関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.6.6 汎用関数とメソッドの例[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-and-Method-Examples-1)

以下の定義を考慮してください。

(define-generic G)
(define-method (G (a <整数>) b) '整数)
(define-method (G (a <real>) b) 'real)
(define-method (G ab) 'top)

`define-generic`呼び出しは、Gを汎用関数として定義します。次の3行は、Gのメソッドを定義します。各メソッドは、指定されたメソッドが適用可能なタイミングを指定する一連のパラメータ特殊化子を使用します。特殊化子を使用すると、パラメータが適用可能となるために（直接的または間接的に）属する必要があるクラスを指定できます。特殊化子が指定されていない場合、システムはデフォルトで`<top>`を使用します。したがって、最初のメソッド定義は以下と同等です。

(define-method (G (a <integer>) (b <top>)) 'integer)

それでは、汎用関数Gへの呼び出し例をいくつか見ていきましょう。

(G 2 3) ⇒ 整数
(G 2 #t) ⇒ 整数
(G 1.2 'a) ⇒ 実在
(G #t #f) ⇒ トップ
(G 1 2 3) ⇒ [エラー](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error) (3 つのパラメータに対して [メソッド](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-method) [exists](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-exists) が存在しないため)

上記のメソッドは、パラメータリストごとに1つの特殊化子のみを使用します。しかし一般に、メソッドのパラメータの全部または一部を特殊化することができます。ここで、次のように定義してみましょう。

(define-method (G (a <整数>) (b <数値>)) '整数-数値)
(define-method (G (a <整数>) (b <実数>)) '整数-実数)
(define-method (G (a <整数>) (b <整数>)) '整数-整数)
(define-method (G a (b <number>)) 'top-number)

これらの定義によれば：

(G 1 2) ⇒ 整数-整数
(G 1 1.0) ⇒ 整数実数
(G 1 #t) ⇒ 整数
(G 'a 1) ⇒ 最上位番号

さらに例として、`<my-complex>` クラスに対する演算の定義を続けましょう。複素数を完全に実装するためにこれを使用したいとします。たとえば、2 つの複素数の加算の定義は次のようになります。

(define-method (new-+ (a <my-complex>) (b <my-complex>))
([make-rectangular](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002drectangular) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) a) ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) b))
([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ([imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart) a) ([imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart) b))))

メソッド `new-+` で使用される `+` が標準的な加算であることを確認するには、次のようにします。

(define-generic new-+)

(let (([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) [+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b)))
(define-method (new-+ (a <my-complex>) (b <my-complex>))
([make-rectangular](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002drectangular) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) a) ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) b))
([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ([imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- imag_002dpart) a) ([imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart) b)))))

`define-generic` は、ここで `new-+` がグローバル環境で定義されることを保証します。これが完了すると、汎用関数 `new-+` に、`+` シンボルのクロージャを作成するメソッドを追加できます。`new-+` メソッドの完全な記述は、[図 8.1](https://doc.guix.gnu.org/guile/latest/en/guile.html#fig_003anewplus) に示されています。

(define-generic new-+)

(let (([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) [+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b)))

(define-method (new-+ (a <real>) (b <real>)) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ab))

(define-method (new-+ (a <real>) (b <my-complex>))
([make-rectangular](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002drectangular) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) a ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) b)) ([imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart) b)))

(define-method (new-+ (a <my-complex>) (b <real>))
([make-rectangular](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002drectangular) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) a) b) ([imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart) a)))

(define-method (new-+ (a <my-complex>) (b <my-complex>))
([make-rectangular](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002drectangular) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) a) ([real-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-real_002dpart) b))
([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) ([imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart) a) ([imag-part](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-imag_002dpart) b))))

(define-method (new-+ (a <number>)) a)
  
(define-method (new-+) 0)

(define-method (new-+ . args)
(new-+ ([car](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-car) args)
([apply](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-apply) new-+ ([cdr](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cdr) args)))))

([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) [+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) new-+)

**図8.1:** 複素数を扱うための `+` の拡張

ここでは、汎用関数は固定数のパラメータを持つ必要がないという事実を利用します。最初の 4 つのメソッドは、2 進加算を実装します。5 番目のメソッドは、単一要素の加算は、その要素自体であることを示します。6 番目のメソッドは、パラメータなしで加算を使用すると常に 0 が返されることを示します (プリミティブ `+` の場合も同様です)。最後のメソッドは、任意の数のパラメータを受け取ります [34](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT34)。このメソッドは、一種の `reduce` として機能します。リストの _car_ と、それを残りの部分に適用した結果に対して 2 進加算を呼び出します。最後に、`set!` を使用すると、`+` 記号を拡張加算に再定義できます。

複素数の実装（統合？）を締めくくるにあたり、標準的なScheme述語を以下のように再定義することができます。

(define-method ([complex?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-complex_003f) c <my-complex>) #t)
(define-method ([complex?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-complex_003f) c) #f)

(define-method ([number?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f) n <number>) #t)
(define-method ([number?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_003f) n) #f)
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)

複素数を含む標準的な基本要素も、同様の方法で再定義できる。

* * *

前へ: [汎用関数とメソッドの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-and-Method-Examples)、上へ: [メソッドと汎用関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.6.7 呼び出しエラーの処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Handling-Invocation-Errors-1)

汎用関数が、適用可能なメソッドが存在しないパラメータの組み合わせで呼び出された場合、GOOPS はエラーを発生させます。

汎用: **no-method** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-no_002dmethod)

メソッド: **no-method** (gf <generic>) args [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-no_002dmethod-1)

アプリケーションが汎用関数を呼び出し、その汎用関数にメソッドが全く定義されていない場合、GOOPS は `no-method` 汎用関数を呼び出します。デフォルトのメソッドは、適切なメッセージとともに `goops-error` を呼び出します。

汎用: **適用可能なメソッドなし** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-no_002dapplicable_002dmethod)

メソッド: **適用できないメソッド** (gf <generic>) 引数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-no_002dapplicable_002dmethod-1)

アプリケーションが引数のセットに汎用関数を適用し、それらの引数型に対応するメソッドが定義されていない場合、GOOPS は汎用関数 `no-applicable-method` を呼び出します。デフォルトのメソッドは、適切なメッセージとともに `goops-error` を呼び出します。

汎用: **no-next-method** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-no_002dnext_002dmethod)

メソッド: **no-next-method** (gf <generic>) 引数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-no_002dnext_002dmethod-1)

汎用関数メソッドが、その汎用関数の次の汎用性の低いメソッドを呼び出すために `(next-method)` を呼び出し、現在の汎用関数引数に対して、より汎用性の低いメソッドが定義されていない場合、GOOPS は `no-next-method` 汎用関数を呼び出します。デフォルトのメソッドは、適切なメッセージとともに `goops-error` を呼び出します。

* * *

次へ: [イントロスペクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Introspection)、前: [メソッドと汎用関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions)、上: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
