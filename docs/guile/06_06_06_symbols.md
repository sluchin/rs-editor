#### 6.6.6 シンボル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols-1)

Schemeにおけるシンボルは、離散データの項目として、アリリストやハッシュテーブルのルックアップキーとして、そして変数参照を表すために、主に3つの方法で広く使用されています。

シンボルは文字列と同様に、文字の並びによって定義されます。この文字の並びはシンボルの名前と呼ばれます。通常の場合、つまりシンボルの名前にScheme構文の他の要素と混同される可能性のある文字が含まれていない場合、シンボルは、引用符やその他の特殊な構文を使用せずに、名前を構成する文字の並びを記述することでSchemeプログラムに書き込まれます。たとえば、「multiply-by-2」という名前のシンボルは、次のように記述されます。

2倍する

これは、「multiply-by-2」という内容の文字列とはどのように異なるかに注目してください。文字列は、次のように二重引用符で囲まれて記述されます。

「2倍する」

記号は、その書き方以外にも、文字列とは2つの重要な点で異なっている。

まず重要な違いは、一意性です。プログラム内で同じ文字列を2つの異なる場所から読み込んだ場合、結果として得られるのは、内容がたまたま同じであるものの、_異なる_文字列オブジェクトです。一方、プログラム内で同じシンボルを2つの異なる場所から読み込んだ場合、結果として得られるのはどちらの場合も同じシンボルオブジェクトです。

2つの読み取ったシンボルが与えられた場合、`eq?` を使用してそれらが同じかどうか（つまり、同じ名前を持っているかどうか）をテストできます。`eq?` は Scheme で最も効率的な比較演算子であり、このように 2 つのシンボルを比較するのは、たとえば 2 つの数値を比較するのと同じくらい高速です。一方、2 つの文字列が与えられた場合、文字列の内容が同じかどうかを判断するには、`equal?` または `string=?` を使用する必要があります。これらははるかに低速な比較演算子です。

(define sym1 ([quote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quote-1) hello))
(define sym2 ([quote](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-quote-1) hello))
([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) sym1 sym2) ⇒ #t

(define str1 "hello")
(define str2 "hello")
([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) str1 str2) ⇒ #f
([equal?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-equal_003f) str1 str2) ⇒ #t

2つ目の重要な違いは、シンボルは文字列とは異なり、自己評価されないということです。そのため、上記の例では `(quote …)` が必要になります。`(quote hello)` は「hello」という名前のシンボル自体に評価されますが、引用符のない `hello` は「hello」という名前のシンボルとして _読み取られ_、変数参照として評価されます。これについては後述します ([変数を表すシンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol- Variables) を参照)。

* [離散データとしてのシンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Data)
* [シンボルをルックアップキーとして使用する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Keys)
* [変数を表す記号](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Variables)
* [シンボルに関連する操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Primitives)
* [シンボルの拡張読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Read-Syntax)
* [非インターニングシンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Uninterned)

* * *

次へ: [記号をルックアップキーとして使用する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Keys)、上: [記号](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.6.1 離散データとしてのシンボル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols-as-Discrete-Data)

数字と記号は、どちらも「eq?」という比較方法に適しているという点で似ている。しかし、記号は数字よりも説明力に優れている。なぜなら、記号の名前は、その記号が表す概念を直接説明するために使用できるからである。

例えば、コンピュータプログラムで色を表現する必要があると想像してみてください。数値を使う場合、数値と色の間の対応関係を任意に選択し、その対応関係を常に一貫して使用するように注意しなければなりません。

1=赤、2=緑、3=紫
(if ([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) (color-of vehicle) 1)
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))

定数を定義することで、マッピングをより明確にし、コードの可読性を高めることができます。

(赤1を定義する)
（グリーン2を定義する）
（紫3を定義する）

(if ([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) (color-of vehicle) red)
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))

しかし、最もシンプルで分かりやすい方法は、数字を一切使わず、色を指定する記号を使うことです。

(if ([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) (color-of vehicle) 'red)
[...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e))

記号が数値よりも記述上の利点を持つのは、記述したい概念の集合が大きくなるにつれて増える。例えば、車というオブジェクトには、以下のような他の特性もあるとしよう。

* オートマチックまたはマニュアルトランスミッション
* 有鉛または無鉛燃料
* パワーステアリング（またはパワーステアリングなし）。

すると、車の複合的な特性セットは、記号のリストとして自然に表現および操作できるようになる。

（車両1の特性）
⇒
（赤色、マニュアル、無鉛ガソリン、パワーステアリング）

(if ([memq](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-memq) 'power-steering (properties-of vehicle1))
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "運転に適さない人でもこの車両を運転できます。\n")
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "この乗り物を運転するには、強い腕力が必要です！\n"))
⊣
運転に適さない人でもこの車両を運転できます。

ここで私たちが依拠しているシンボルの基本的な特性は、プログラムのある部分における「red」というシンボルは、プログラムの別の部分における「red」というシンボルと区別できないということです。つまり、シンボルは「eq?」を使って効果的に比較できます。同時に、シンボルには自然に分かりやすい名前が付けられます。この効率性と説明力の組み合わせにより、シンボルは離散データとして使用するのに理想的です。

* * *

次へ: [変数を表す記号](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Variables)、前: [離散データとしての記号](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Data)、上: [記号](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.6.2 ルックアップキーとしてのシンボル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols-as-Lookup-Keys)

その効率性と記述力から、記号を連想リストやハッシュテーブルのキーとして使用するのは自然なことである。

これを説明するために、前の小節で挙げた車のプロパティの例を、より構造化された形で表現してみましょう。すべてのプロパティをフラットなリストにまとめて記述するのではなく、次のような連想リストを使用できます。

(define car1-properties '((color . red)
（トランスミッション：マニュアル）
（燃料：無鉛ガソリン）
（パワーステアリング）

この構造は、フラットなリストよりも明確で拡張性が高いことに注目してください。例えば、「manual」は車の窓やロックではなく、トランスミッションを指していることが明確に分かります。また、プロパティ間で同じ記号を値として使用しても、曖昧さが生じません。

(define car1-properties '((color . red)
（トランスミッション：マニュアル）
（燃料：無鉛ガソリン）
（パワーステアリング）
（シートカラー：赤）
（ロック・手動）

このような表現を用いることで、効率的な`assq-XXX`ファミリーの手続き（[連想リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Association-Lists)を参照）を使用して、個々の情報を抽出または変更することが容易になります。

([assq-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-assq_002dref) car1-properties 'fuel) ⇒ 無鉛ガソリン
([assq-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-assq_002dref) car1-properties 'transmission) ⇒ manual

([assq-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-assq_002dset_0021) car1-properties 'seat-color 'black)
⇒
((色.赤)
（トランスミッション：マニュアル）
（燃料：無鉛ガソリン）
（パワーステアリング）
（シートカラー：黒）
（ロック・手動）

ハッシュテーブルにもキーがあり、ハッシュテーブルにおけるシンボルの使用には、連想リストの場合とまったく同じ引数が適用されます。Guile がシンボルをキーとするエントリをハッシュテーブルに追加する場所を決定するために使用するハッシュ値は、`symbol-hash` プロシージャを呼び出すことで取得できます。

スキーム手順: **symbol-hash** シンボル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002dhash)

C 関数: **scm\_symbol\_hash** (symbol) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsymbol_005fhash)

シンボルのハッシュ値を返します。

ハッシュテーブル全般に関する情報、および連想リストではなくハッシュテーブルを使用する理由については、[ハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Tables)を参照してください。

* * *

次へ: [シンボルに関連する操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Primitives)、前: [ルックアップキーとしてのシンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Keys)、上: [シンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.6.3 変数を表す記号 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols-as-Denoting-Variables)

Schemeプログラムにおいて引用符で囲まれていないシンボルが評価される場合、それは変数参照として解釈され、評価結果は該当する変数の値となります。

例えば、式 `(string-length "abcd")` が読み込まれて評価される場合、文字シーケンス `string-length` は、名前が "string-length" のシンボルとして読み込まれます。このシンボルは、文字列の長さを計算するプロシージャの値を持つ変数に関連付けられています。したがって、`string-length` シンボルの評価結果は、そのプロシージャになります。

引用符で囲まれていないシンボルと、それが参照する変数との間の関係の詳細については、別の箇所で説明されています。シンボルと変数の関連付けがどのように作成されるかについては、[定義と変数バインディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binding-Constructs) を、Guile のモジュール システムによってこれらの関連付けがどのように影響を受けるかについては、[モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#Modules) を参照してください。

* * *

次へ: [シンボルの拡張読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Read-Syntax)、前: [変数を表すシンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Variables)、上: [シンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.6.4 シンボルに関連する操作 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Operations-Related-to-Symbols)

Schemeの値が与えられた場合、`symbol?`プリミティブを使用して、それがシンボルであるかどうかを判定できます。

Scheme Procedure: **symbol?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_003f)

C 関数: **scm\_symbol\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsymbol_005fp)

objがシンボルの場合は`#t`を返し、そうでない場合は`#f`を返します。

C 関数: `int` **scm\_is\_symbol** `(SCM 値)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fsymbol)

`scm_is_true (scm_symbol_p (val))` と同等です。

シンボルが特定できたら、`symbol->string` を呼び出すことで、そのシンボル名を文字列として取得できます。なお、Guile は、`symbol->string` の大文字小文字の区別に関して、デフォルトでは R5RS と異なる点にご注意ください。

Scheme手順: **symbol->string** s [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring)

C 関数: **scm\_symbol\_to\_string** (s) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsymbol_005fto_005fstring)

シンボル s の名前を文字列として返します。デフォルトでは、Guile はシンボルを大文字と小文字を区別して読み込むため、返される文字列は、s が作成された原因となった文字シーケンスと同じ大文字小文字のバリエーションを持ちます。

Guile が (R5RS で指定されているように) シンボルを大文字小文字を区別せずに読み込むように設定されている場合、s がリテラル式の一部として (「The Revised^5 Report on Scheme」の [Literal expressions](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Literal-expressions) または `read` または `string-ci->symbol` プロシージャの呼び出しによって生成される場合、Guile はシンボル オブジェクトを作成する前にシンボル名に含まれるアルファベット文字を小文字に変換するため、ここで返される文字列は小文字になります。

s が `string->symbol` によって作成された場合、返される文字列の文字の大文字/小文字は、s が作成された時点での Guile の大文字/小文字の区別設定に関係なく、`string->symbol` に渡された文字列の文字の大文字/小文字と同じになります。

このプロシージャによって返される文字列に対して、`string-set!`のような変更プロシージャを適用することはエラーです。

ほとんどのシンボルは、コードに文字通り記述することによって作成されます。しかし、以下の手順を使用してプログラム的にシンボルを作成することも可能です。

Scheme手順: **symbol** char… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol)

指定された文字引数から作成された、新たに割り当てられたシンボルを返します。

(記号 #\\x #\\y #\\z) ⇒ xyz

Scheme手順: **list->symbol** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002d_003esymbol)

文字リストから作成された、新たに割り当てられたシンボルを返します。

(リスト->シンボル '(#\\a #\\b #\\c)) ⇒ abc

スキームプロシージャ: **symbol-append** arg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002dappend)

指定されたシンボル、arg の連結で構成される文字を持つ、新しく割り当てられたシンボルを返します。

(let ((h 'hello))
(symbol-append h 'world))
⇒ハローワールド

Scheme手順: **string->symbol** string [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol)

C 関数: **scm\_string\_to\_symbol** (string) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fto_005fsymbol)

名前が文字列であるシンボルを返します。この手続きでは、特殊文字や非標準の文字を含む名前のシンボルを作成できますが、Scheme の実装によっては、そのようなシンボルをそのまま読み取ることができない場合があるため、通常はそのようなシンボルを作成することはお勧めできません。

Scheme手順: **string-ci->symbol** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_002d_003esymbol)

C 関数: **scm\_string\_ci\_to\_symbol** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fci_005fto_005fsymbol)

名前がstrであるシンボルを返します。Guileが現在シンボルを大文字小文字を区別せずに読み込んでいる場合、返されるシンボルが検索または作成される前にstrは小文字に変換されます。

以下の例は、Guileにおける記号の大文字小文字の区別に関する詳細な動作を示しています。

([read-enable](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002denable) 'case-insensitive) ; R5RS準拠の動作
([symbol->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring) 'flying-fish) ⇒ "flying-fish"
([symbol->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring) 'Martin) ⇒ "martin"
([symbol->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring)
([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "Malvina")) ⇒ "Malvina"

([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) 'mISSISSIppi 'mississippi) ⇒ #t
([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "mISSISSIppi") ⇒ mISSISSIppi
([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) 'bitBlt ([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "bitBlt")) ⇒ #f
([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) 'LolliPop
([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) ([symbol- >string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring) 'LolliPop))) ⇒ #t
([string=?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003d_003f) "K. Harper, MD"
([symbol->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring)
([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "K. Harper, MD"))) ⇒ #t

([read-disable](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-read_002ddisable) 'case-insensitive) ; Guile のデフォルトの動作
([symbol->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring) 'flying-fish) ⇒ "flying-fish"
([symbol->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring) 'Martin) ⇒ "Martin"
([symbol->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring)
([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "Malvina")) ⇒ "Malvina"

([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) 'mISSISSIppi 'mississippi) ⇒ #f
([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "mISSISSIppi") ⇒ mISSISSIppi
([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) 'bitBlt ([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "bitBlt")) ⇒ #t
([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) 'LolliPop
([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) ([symbol->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring) 'LolliPop))) ⇒ #t
([string=?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003d_003f) "K. Harper, MD"
([symbol->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002d_003estring)
([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "K. Harper, MD"))) ⇒ #t

C言語には、現在のロケールエンコーディングのC文字列からSchemeシンボルを構築する低レベル関数が存在する。

C言語でより多くのことを行いたい場合は、`scm_symbol_to_string`と`scm_string_to_symbol`を使用してシンボルと文字列を相互に変換し、文字列を操作する必要があります。

C 関数: `SCM` **scm\_from\_latin1\_symbol** `(const char *name)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flatin1_005fsymbol)

C 関数: `SCM` **scm\_from\_utf8\_symbol** `(const char *name)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005futf8_005fsymbol)

ヌル終端されたC文字列名で指定された名前を持つSchemeシンボルを構築して返します。これは、C文字列がソースコードにハードコーディングされている場合に適しています。

C 関数: `SCM` **scm\_from\_locale\_symbol** `(const char *name)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flocale_005fsymbol)

C 関数: `SCM` **scm\_from\_locale\_symboln** `(const char *name, size_t len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ffrom_005flocale_005fsymboln)

指定された名前を持つ Scheme シンボルを構築して返します。`scm_from_locale_symbol` の場合、name はヌル終端されている必要があります。`scm_from_locale_symboln` の場合、name の長さは len で明示的に指定されます。

これらの関数は、name が C 文字列定数である場合は使用しないでください。現在のロケールが、文字列定数や文字定数に使用される実行文字セットと一致する保証がないためです。最新の C コンパイラのほとんどはデフォルトで UTF-8 を使用するため、そのような場合は `scm_from_utf8_symbol` の使用をお勧めします。

C 関数: `SCM` **scm\_take\_locale\_symbol** `(char *str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftake_005flocale_005fsymbol)

C 関数: `SCM` **scm\_take\_locale\_symboln** `(char *str, size_t len)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ftake_005flocale_005fsymboln)

`scm_from_locale_symbol` や `scm_from_locale_symboln` と同様ですが、最終的には `free` で str を解放します。したがって、Scheme 文字列を作成した直後に str を解放する場合に、この関数を使用できます。場合によっては、Guile は str を内部表現として直接使用できます。

シンボルのサイズはC言語からも取得できます。

C 関数: `size_t` **scm\_c\_symbol\_length** `(SCM sym)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fsymbol_005flength)

sym の文字数を返します。

最後に、特に動的に新しいSchemeコードを生成するアプリケーションでは、生成されたコードで使用するシンボルを生成する必要があります。`gensym`プリミティブはこのニーズを満たします。

Scheme手順: **gensym** \[prefix\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-gensym)

C 関数: **scm\_gensym** (プレフィックス) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fgensym)

接頭辞とカウンタ値から構成される名前を持つ新しいシンボルを作成します。文字列接頭辞はオプションの引数として指定できます。デフォルトの接頭辞は「g」です。カウンタは呼び出しごとに1ずつ増加します。カウンタをリセットする機能はありません。

`gensym` で生成されるシンボルは、名前がスペースで始まるため、一意である可能性が高いです。プログラマーが意図的にそうしない限り、このようなシンボルを生成することはできません。一意性を保証するには、代わりにインターン化されていないシンボルを使用します ([インターン化されていないシンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Uninterned) を参照)。ただし、インターン化されていないシンボルは、出力して読み戻すのに適していません。

* * *

次へ: [非インターニングシンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Uninterned)、前: [シンボルに関連する操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Primitives)、上: [シンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.6.5 シンボルの拡張読み取り構文 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extended-Read-Syntax-for-Symbols)

記号の読み取り構文は、数字、数字、および拡張アルファベット文字のシーケンスであり、数字の先頭には使用できない文字で始まります。さらに、数字が「+」、「-」、「.」で始まる場合でも、「+」、「-」、「...」などの特殊なケースは記号として読み取られます。

拡張アルファベット文字は、識別子内で文字と同様に使用できます。拡張アルファベット文字のセットは次のとおりです。

! $ % & \* + - . / : < = > ? @ ^ \_ ~

上記で定義した標準の読み取り構文（R5RS（『The Revised^5 Report on Scheme』の[Formal syntax](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Formal-syntax)を参照）から取得）に加えて、Guileは、スペース文字、改行、括弧などの特殊な文字を含めることができる拡張シンボル読み取り構文を提供します。何らかの理由で、上記に記載されていない文字を含むシンボルを記述する必要がある場合は、次のように記述できます。

* 記号を文字 `#{` で開始します。
* 記号の文字を書き、
* 記号を文字 `}#` で終了します。

以下に、この形式の読み取り構文の例をいくつか示します。最初の記号はスペース文字が含まれているため拡張構文を使用する必要があり、2番目の記号は改行文字が含まれているため、そして最後の記号は数字のように見えるため拡張構文を使用する必要があります。

#{foo bar}#

＃{何
これまで}＃

#{4242}#

Guileはシンボルに対してこのような拡張された読み取り構文を提供していますが、移植性が低く、読みやすさもあまり良くないため、広く使用することは推奨されません。

あるいは、`r7rs-symbols` 読み取りオプションを有効にすると（[スキームコードの読み取り](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Read)を参照）、二重引用符の代わりに縦棒で区切られる点を除いて、文字列に使用されるのと同じ表記法を使用して任意のシンボルを記述できます。

|フーバー|
|\\x3BB; はギリシャ文字のラムダです|
|\\| は縦棒です|

`r7rs-symbols` という出力オプションもあることに注意してください（[スキーム値の書き込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Scheme-Write)を参照）。この表記法を使用するには、次の式のいずれか、または両方を評価します。

(読み取り有効化 'r7rs-symbols')
(print-enable 'r7rs-symbols)

* * *

前へ: [シンボルの拡張読み取り構文](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Read-Syntax)、上へ: [シンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.6.6 非インターニングシンボル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Uninterned-Symbols)

シンボルが便利なのは、自動的に一意性が保たれるからです。異なるオブジェクトでありながら同じ名前を持つシンボルは存在しません。もちろん、例外のない規則はありません。これまで説明してきた通常のシンボルに加えて、少し異なる動作をする特別な_非インターニング_シンボルを作成することもできます。

それらが何が違うのか、そしてなぜ有用なのかを理解するために、通常の記号がどのようにして独自性を保っているのかを見ていきましょう。

Guileは、例えば`read`時や`string->symbol`の実行時など、特定の名前を持つシンボルを検索したい場合、まず既存のシンボル一覧表を調べて、指定された名前のシンボルが既に存在するかどうかを確認します。既に存在する場合は、そのシンボルを返します。存在しない場合は、指定された名前の新しいシンボルが作成され、後で検索できるように一覧表に追加されます。

場合によっては、以前には存在しなかった、つまり「新規」のシンボルを作成したい場合があります。また、将来、他の誰かが意図せずそのシンボルに遭遇しないようにしたい場合もあるでしょう。このようなシンボルの特性は、マクロ展開時にコードを生成する際によく必要になります。新しい一時変数を導入する際には、他の人のコード内の変数と競合しないようにする必要があります。

これを実現する最も簡単な方法は、新しいシンボルを作成するものの、それをすべてのシンボルのグローバルテーブルに登録しないことです。そうすれば、誰も偶然にあなたのシンボルにアクセスすることはできません。テーブルに登録されていないシンボルは「未登録」と呼ばれ、もちろん、テーブルに登録されているシンボルは「登録済み」と呼ばれます。

`make-symbol`関数を使用すると、インターン化されていない新しいシンボルを作成できます。`symbol-interned?`を使用すると、シンボルがインターン化されているかどうかをテストできます。

非インターンシンボルは、シンボル名がシンボルオブジェクトを一意に識別するというルールに違反します。そのため、インターンシンボルのように書き出して読み込むことはできません。現在、Guileは非インターンシンボルの読み込みをサポートしていません。このため、関数`gensym`は非インターンシンボルを返さないことに注意してください。

Scheme 手順: **make-symbol** name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dsymbol)

C 関数: **scm\_make\_symbol** (名前) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fsymbol)

名前が name の新しい非インターン化シンボルを返します。返されるシンボルは一意であることが保証されており、以降の `string->symbol` の呼び出しでは同じシンボルは返されません。

Scheme 手順: **symbol-interned?** シンボル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002dinterned_003f)

C 関数: **scm\_symbol\_interned\_p** (シンボル) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsymbol_005finterned_005fp)

シンボルがインターンされている場合は`#t`を返し、そうでない場合は`#f`を返します。

例えば：

(define foo-1 ([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "foo"))
(define foo-2 ([string->symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003esymbol) "foo"))
(define foo-3 ([make-symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- make_002dsymbol) "foo"))
(define foo-4 ([make-symbol](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dsymbol) "foo"))

([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) foo-1 foo-2)
⇒ #t
同じ名前の2つのインターンシンボルは同じオブジェクトです。
([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) foo-1 foo-3)
⇒ #f
; しかし、同じ名前でmake-symbolを呼び出すと、
; 別個のオブジェクト。
([eq?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f) foo-3 foo-4)
⇒ #f
; make-symbol の呼び出しは、たとえ
同じ名前。
フー3
⇒ #<uninterned-symbol foo 8085290>
; 非収容シンボルは収容シンボルとは異なる印刷をします。
([symbol?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_003f) foo-3)
⇒ #t
しかし、それらは依然として象徴であり、
([symbol-interned?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-symbol_002dinterned_003f) foo-3)
⇒ #f
ただ、収容所には入れられなかっただけだ。

* * *

次へ: [ペア](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pairs)、前: [シンボル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbols)、上: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
