### 9.2 データ表現 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Representation-1)

Schemeは潜在型言語です。つまり、システムは一般的に、コンパイル時に与えられた式の型を判別できません。型は実行時にのみ明らかになります。変数には固定型がなく、ある時点ではペアを、次の時点では整数を、さらに後には1000要素のベクトルを保持する可能性があります。固定型を持つのは変数ではなく値です。

`pair?`や`string?`といった標準的なScheme関数を実装し、ガベージコレクションを提供するためには、すべての値の表現には、実行時にその型を正確に判別するのに十分な情報が含まれている必要があります。Schemeシステムでは、この情報を使用して、プログラムが不適切な型の値（例えば、文字列の`car`を取るなど）に操作を適用しようとしたかどうかを判断することもよくあります。

変数、ペア、ベクトルは任意の型の値を保持できるため、Schemeの実装では値の統一表現を使用します。これは、完全な値または完全な値へのポインタと必要な型情報を保持できる十分な大きさの単一の型です。

以下のセクションでは、まずシンプルな型システムを紹介し、次にその主な弱点を修正するための改良を加えます。最後に、ガベージコレクションとデータ表現に関してGuileが採用した具体的な選択について考察します。

* [単純な表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Simple-Representation)
* [Faster Integers](https://doc.guix.gnu.org/guile/latest/en/guile.html#Faster-Integers)
* [より安価なペア](https://doc.guix.gnu.org/guile/latest/en/guile.html#Cheaper-Pairs)
* [保守的なゴミ収集](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conservative-GC)
* [GuileにおけるSCMタイプ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in-Guile)

* * *

次へ: [高速整数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Faster-Integers)、上: [データ表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Representation) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.1 単純な表現 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Simple-Representation-1)

C言語でSchemeの値を表現する最も簡単な方法は、各値を型インジケータを含む構造体へのポインタとして表現し、その後に実際の値を保持する共用体を続けることです。`SCM`を汎用型の名前とすると、次のように記述できます。

列挙型 { 整数、ペア、文字列、ベクトル、... };

typedef struct value \*SCM;

構造体値 {
列挙型型;
ユニオン {
int 整数;
struct { SCM car, cdr; } pair;
struct { int length; char \*elts; } string;
struct { int length; SCM \*elts; } vector;
...
} 価値;
};

省略記号の部分は、残りのScheme型のコードに置き換えられています。

この表現は、Schemeのすべてのセマンティクスを実装するのに十分です。xが`SCM`値の場合：

* xが整数かどうかをテストするには、`x->type == integer`と記述します。
* その値を求めるには、`x->value.integer` と記述します。
* xがベクトルかどうかをテストするには、`x->type == vector`と記述します。
* xがベクトルであることがわかっている場合は、`x->value.vector.elts[0]`と記述して、その最初の要素を参照できます。
* xがペアであることがわかっている場合は、`x->value.pair.car`と記述して、そのペアの車を抽出できます。

* * *

次へ: [より安価なペア](https://doc.guix.gnu.org/guile/latest/en/guile.html#Cheaper-Pairs)、前: [シンプルな表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Simple-Representation)、上: [データ表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Representation) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.2 より高速な整数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Faster-Integers-1)

残念ながら、上記の表現方法には重大な欠点があります。整数を返すには、式は`struct value`を割り当て、その整数を表すように初期化し、そのポインタを返す必要があります。さらに、整数の値を取得するにはメモリ参照が必要となり、これはほとんどのプロセッサにおいてレジスタ参照よりもはるかに低速です。整数は非常に頻繁に出現するため、この表現方法は時間と空間の両面でコストが高すぎます。整数は生成と操作が非常に安価であるべきです。

一つの解決策として、多くのアーキテクチャでは、ヒープに割り当てられたデータ（つまり、`malloc` を呼び出したときに取得されるデータ）は8バイト境界にアラインされている必要があるという観察結果が挙げられます。（マシンが実際にそれを要求するかどうかに関わらず、`struct value` オブジェクト用に、これが真であることを保証する独自のアロケータを作成できます。）この場合、構造体のアドレスの下位3ビットはゼロであることがわかっています。

これにより、整数をより適切に表現するための余地が生まれます。そこで、以下のルールを設けます。

* `SCM` 値の下位 3 ビットがゼロの場合、SCM 値は `struct value` へのポインタであり、すべてはこれまでどおりに進みます。
* それ以外の場合、`SCM` 値は整数を表し、その値は上位ビットに表示されます。

この規約を実装したC言語のコードを以下に示します。

列挙型 { ペア、文字列、ベクトル、... };

typedef struct value \*SCM;

構造体値 {
列挙型型;
ユニオン {
struct { SCM car, cdr; } pair;
struct { int length; char \*elts; } string;
struct { int length; SCM \*elts; } vector;
...
} 価値;
};

#define POINTER\_P(x) (((int) (x) & 7) == 0)
#define INTEGER\_P(x) (! POINTER\_P (x))

#define GET_INTEGER(x) ((int) (x) >> 3)
#define MAKE\_INTEGER(x) ((SCM) (((x) << 3) | 1))

`integer` が `enum type` の要素として表示されなくなり、共用体から `integer` メンバーが削除されたことに注意してください。代わりに、`POINTER_P` および `INTEGER_P` マクロを使用して、値を整数と非整数に大まかに分類し、以前と同様に型テストを実行します。

上記の質問に対する回答は以下のとおりです（ここでも、xは`SCM`値であると仮定します）。

* x が整数かどうかをテストするには、`INTEGER_P (x)` と記述できます。
* その値を求めるには、`GET_INTEGER (x)` と記述します。
* xがベクトルかどうかをテストするには、次のように記述できます。
    
POINTER\_P (x) && x->type == vector
    
新しい表現を用いる場合、x を逆参照してその完全な型を決定する前に、x が本当にポインタであることを確認する必要があります。
    
* xがベクトルであることがわかっている場合は、以前と同様に、`x->value.vector.elts[0]`と記述して最初の要素を参照できます。
* xがペアであることがわかっている場合は、以前と同様に`x->value.pair.car`と記述してその車を抽出できます。

この表現方法を用いることで、最初の表現方法よりも効率的に整数を演算できます。例えば、xとyが整数であることが分かっている場合、それらの和は次のように計算できます。

MAKE_INTEGER (GET_INTEGER (x) + GET_INTEGER (y))

さて、整数演算にはメモリ割り当てやメモリ参照は不要です。実際のSchemeシステムのほとんどは、加算などの演算をさらに効率的なアルゴリズムで実装していますが、このエッセイはビット操作について論じるものではありません。（ヒント：多倍長整数へのオーバーフローをどのように判断しますか？アセンブリ言語ではどのように実装しますか？）

* * *

次へ: [保守的なガベージコレクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conservative-GC)、前: [高速整数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Faster-Integers)、上: [データ表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Representation) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.3 より安価なペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Cheaper-Pairs-1)

しかし、解決すべき問題がもう一つあります。ほとんどの Scheme ヒープには、他のどのタイプのオブジェクトよりも多くのペアが含まれています。Jonathan Rees は、自身の Scheme 実装である Scheme 48 では、ペアがヒープの 45% を占めていると述べています。しかし、上記の表現では、ペアごとに 3 つの `SCM` サイズのワードを使用しています。1 つは型、残りの 2 つは CAR と CDR です。ペアを 2 つのワードだけで表現する方法はないでしょうか？

先に確立した慣例をさらに洗練させてみましょう。次のように主張します。

* `SCM` 値の下位 3 ビットが `#b000` の場合、それは以前と同様にポインタです。
* 下位3ビットが「#b001」の場合、上位ビットは整数になります。これは以前よりも少し制限が厳しくなりました。
* 下位 2 ビットが `#b010` の場合、下位 3 ビットをマスクした値はペアのアドレスになります。

新しいC言語のコードは以下のとおりです。

列挙型 { 文字列、ベクトル、... };

typedef struct value \*SCM;

構造体値 {
列挙型型;
ユニオン {
struct { int length; char \*elts; } string;
struct { int length; SCM \*elts; } vector;
...
} 価値;
};

構造体ペア {
SCMカー、司令官。
};

#define POINTER\_P(x) (((int) (x) & 7) == 0)

#define INTEGER\_P(x) (((int) (x) & 7) == 1)
#define GET\_INTEGER(x) ((int) (x) >> 3)
#define MAKE\_INTEGER(x) ((SCM) (((x) << 3) | 1))

#define PAIR\_P(x) (((int) (x) & 7) == 2)
#define GET\_PAIR(x) ((struct pair \*) ((int) (x) & ~7))

`enum type`と`struct value`には、ベクトルと文字列のみが含まれるようになり、整数とペアは特殊なケースになったことに注意してください。また、上記のコードは`int`がポインタを格納するのに十分な大きさであると仮定していますが、これは一般的には当てはまりません。

以下に、事例の一覧を示します。

* x が整数かどうかをテストするには、`INTEGER_P (x)` と記述できます。これは以前と同じです。
* その値を求めるには、以前と同様に `GET_INTEGER (x)` と記述します。
* xがベクトルかどうかをテストするには、次のように記述できます。
    
POINTER\_P (x) && x->type == vector
    
x を逆参照してその型を調べる前に、x が `struct value` へのポインタであることを確認する必要があります。
    
* xがベクトルであることがわかっている場合は、以前と同様に、`x->value.vector.elts[0]`と記述して最初の要素を参照できます。
* x がペアであるかどうかを判断するには `PAIR_P (x)` と記述し、その車を参照するには `GET_PAIR (x)->car` と記述します。

この表現方法の変更により、ヒープサイズが15%削減されます。また、メモリ参照が不要になるため、値がペアであるかどうかを判断するコストも低くなります。`SCM`値の下位2ビットをチェックするだけで済みます。これは、Schemeシステムでよく行われるリストの走査において、特に重要となる可能性があります。

繰り返しになりますが、実際のSchemeシステムのほとんどは、少し異なる実装を使用しています。たとえば、GET_PAIRが`x`の下位ビットをマスクするのではなく減算する場合、オプティマイザは多くの場合、その減算と参照している構造体メンバーのオフセットの加算を組み合わせることができ、変更されたポインタを未変更のポインタと同じくらい高速に使用できます。

* * *

次へ: [Guile の SCM タイプ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in-Guile)、前: [Cheaper Pairs](https://doc.guix.gnu.org/guile/latest/en/guile.html#Cheaper-Pairs)、上: [データ表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Representation) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC _Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.4 保守的なガベージコレクション [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conservative-Garbage-Collection)

潜在型付けを除けば、Scheme実装におけるデータ表現の制約となる主な要因はガベージコレクタである。コレクタはヒープ内のすべての生存オブジェクトを走査し、どのオブジェクトが生存しておらず、したがって収集可能であるかを判断できなければならない。

これを実装する方法は数多くあります。Guileのガベージコレクションは、Boehm-Demers-Weiser保守型ガベージコレクタ（BDW-GC）というライブラリに基づいています。BDW-GCは、ほとんどの場合「そのまま動作する」のですが、こうした仕組みを知ることは興味深いので、ここではBDW-GCの動作について概要を説明します。

ガベージコレクションには、2つの論理フェーズがあります。1つは、生存オブジェクトのセットを列挙する「マーク」フェーズ、もう1つは、マークフェーズで走査されなかったオブジェクトを収集する「スイープ」フェーズです。コレクタが正しく機能するには、生存オブジェクトのセット全体を走査できる必要があります。

マークフェーズでは、コレクタはシステムのグローバル変数とスタック上のローカル変数をスキャンして、Cコードからすぐにアクセスできるオブジェクトを特定します。次に、それらのオブジェクトをスキャンして、それらが指すオブジェクトを見つけ、これを繰り返します。コレクタは、見つけた各オブジェクトに論理的にマークビットを設定するため、各オブジェクトは一度だけ走査されます。

コレクターが、マークされたオブジェクトによって指される未マークのオブジェクトを見つけられない場合、まだマークされていないオブジェクトはプログラムによって決して使用されないと想定し（グローバル変数またはローカル変数からそれらに到達する参照解除のパスがないため）、それらを解放します。

上記の段落では、ガベージコレクタがグローバル変数とローカル変数をどのように見つけるかについては具体的に説明していませんでしたが、通常どおり、さまざまなアプローチが存在します。多くの場合、プログラマは、ヒープを参照するすべてのグローバル変数へのポインタのリストと、コレクタが処理しやすいようにローカル変数のリスト（各関数への入退出時に調整）を保持する必要があります。

グローバル変数は比較的まれであるため、グローバル変数のリストを維持するのは通常それほど難しくありません。しかし、ローカル変数のリストを明示的に維持するのは（筆者の経験上）悪夢のような作業です。そのため、BDW-GCは「保守的なガベージコレクション」と呼ばれる手法を用いて、ローカル変数のリストを不要にしています。

保守的なガベージコレクションのコツは、Cスタックを通常のメモリ領域として扱い、Cスタック上のすべてのワードがヒープへのポインタであると仮定することです。そのため、コレクタは、そのワードがどのように解釈されるべきかを確実に知らなくても、Cスタック内のどこかにアドレスが現れるすべてのオブジェクトをマークします。

BDW-GCはスタックに加えて、静的データセクションもスキャンします。つまり、Schemeのアクティブなオブジェクトを探す際には、グローバル変数もスキャンされるということです。

当然ながら、このようなシステムでは、実際には不要で解放すべきオブジェクトが時折保持されてしまうことがあります。しかし実際には、これは問題になりません。なぜなら、保守的にスキャンされる場所のセットは固定されているからです。SchemeスタックはCスタックとは別に管理され、（保守的ではなく）正確にスキャンされます。また、GC管理ヒープは、ポインタ（ベクトルなど）を格納できる部分と格納できない部分（バイトベクトルなど）に分割されているため、生の整数と有効なオブジェクトへのポインタを混同する可能性が制限されます。

興味のある読者は、保守的なGC全般、特にBDW-GCの実装に関する詳細情報については、[http://www.hboehm.info/gc/](http://www.hboehm.info/gc/)にあるBDW-GCのウェブページを参照してください。

* * *

前へ: [保守的なガベージコレクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Conservative-GC)、上へ: [データ表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Representation) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.5 Guile の SCM タイプ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in-Guile-1)

GuileはSchemeオブジェクトを2種類に分類します。1つは`SCM`内に完全に収まるオブジェクト、もう1つはヒープストレージを必要とするオブジェクトです。

前者のクラスは「即値」と呼ばれます。即値クラスには、小さな整数、文字、ブール値、空のリスト、謎のファイル終端オブジェクト、その他いくつかのオブジェクトが含まれます。

残りの型は、当然ながら「非即時型」と呼ばれます。これには、ペア、プロシージャ、文字列、ベクトル、およびGuileのその他のすべてのデータ型が含まれます。非即時型の場合、`SCM`ワードにはヒープ上のデータへのポインタが含まれており、そのデータには対象オブジェクトに関する詳細情報が格納されています。

このセクションでは、`SCM`型がC言語レベルで実際にどのように表現され、使用されているかを説明します。Guileが型情報をどのように格納しているかについて詳しく知りたい場合は、`libguile/scm.h`を参照してください。

実際、Guileではオブジェクトを表すための基本的なCデータ型が2つあります。`SCM`と`scm_t_bits`です。

* [`SCM` と `scm_t_bits` の関係](https://doc.guix.gnu.org/guile/latest/en/guile.html#Relationship-Between-SCM-and-scm_005ft_005fbits)
* [即時オブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Immediate-Objects)
* [非即時オブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dImmediate-Objects)
* [ヒープオブジェクトの割り当て](https://doc.guix.gnu.org/guile/latest/en/guile.html#Allocating-Heap-Objects)
* [ヒープオブジェクト型情報](https://doc.guix.gnu.org/guile/latest/en/guile.html#Heap-Object-Type-Information)
* [ヒープオブジェクトフィールドへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Heap-Object-Fields)

* * *

次へ: [Immediate Objects](https://doc.guix.gnu.org/guile/latest/en/guile.html#Immediate-Objects)、上へ: [The SCM Type in Guile](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in-Guile) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]

#### 9.2.5.1 `SCM` と `scm_t_bits` の関係 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Relationship-Between-SCM-and-scm_005ft_005fbits-1)

型 `SCM` の変数は、有効な Scheme オブジェクトを保持することが保証されています。一方、型 `scm_t_bits` の変数は、C 整数型として `SCM` 値の表現を保持する場合がありますが、有効な Scheme オブジェクトに対応しない場合でも、任意の C 値を保持する場合があります。

型が `SCM` の変数 x の場合、Scheme オブジェクトの型情報は直接使用できない形式で格納されます。スキーム値の型エンコーディングを操作するには、`SCM_UNPACK` マクロを使用して、`SCM` 変数を対応する `scm_t_bits` 変数 y の表現に変換する必要があります。この変換が完了すると、この章の前半の例で示したように ([Cheaper Pairs](https://doc.guix.gnu.org/guile/latest/en/guile.html#Cheaper-Pairs) を参照)、`scm_t_bits` 値 y のビットの内容からスキーム オブジェクト x の型を導出できます。逆に、`scm_t_bits` 変数としての Scheme 値の有効なビットエンコーディングは、`SCM_PACK` マクロを使用して対応する `SCM` 値に変換できます。

* * *

次へ: [非即時オブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dImmediate- Objects)、前: [`SCM` と `scm_t_bits` の関係](https://doc.guix.gnu.org/guile/latest/en/guile.html#Relationship-Between-SCM-and-scm_005ft_005fbits)、上: [Guile の SCM タイプ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次内容")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.5.2 即時オブジェクト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Immediate-Objects-1)

Schemeオブジェクトは、必要な情報をすべてそれ自体で保持する即値である場合と、名前が示すとおりヒープ上のデータであるヒープオブジェクトへの参照を含む場合があります。一般的に、オブジェクトが即値であるかどうかはユーザーコードには関係ありませんが、Guile自身のコード内では、この区別が重要になる場合があります。そのため、以下の低レベルマクロが提供されています。

マクロ: `int` **SCM\_IMP** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fIMP)

Schemeオブジェクトは、`SCM_IMP`述語を満たす場合は即値であり、そうでない場合はヒープオブジェクトへのエンコードされた参照を保持します。述語の結果はCスタイルのブール値として返されます。ユーザーコードおよびGuileを拡張するコードは、通常、このマクロを使用する必要はありません。

まとめ：

* 不明な型の Scheme オブジェクト x が与えられた場合、まず `SCM_IMP (x)` を使用して、それが即時オブジェクトであるかどうかを確認します。
* その場合、型と値の情報は、`SCM_UNPACK (x)` によって提供される `scm_t_bits` 値からすべて判別できます。

Schemeには多くの特殊値があり、そのほとんどはこのマニュアルの別の箇所に記載されています。ここにそれらを記載するのは適切ではありませんが、とりあえず、これらの値の一部に付けられたC言語名の一覧を以下に示します。

マクロ: `SCM` **SCM\_EOL** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fEOL)

Scheme の空リストオブジェクト、または「リストの末尾」オブジェクトは、通常 Scheme では `'()` と表記されます。

マクロ: `SCM` **SCM\_EOF\_VAL** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fEOF_005fVAL)

Schemeにおけるファイル終端値。当然ながら、標準的な記述方法は存在しない。

マクロ: `SCM` **SCM\_UNSPECIFIED** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fUNSPECIFIED)

Scheme標準では「未指定」の値を返すとされている式の一部（すべてではない）が返す値。

これは少々奇妙なほど文字通りの解釈方法ですが、標準的な読み込み・評価・出力ループでは、式がこの値を返すと何も出力されないため、他に役に立つ方法が思いつかない場合は、これを返すのも悪くない方法です。

マクロ: `SCM` **SCM\_UNDEFINED** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fUNDEFINED)

「未定義」値。その最も重要な特性は、有効なScheme値と等しくないことです。これは、Guileとやり取りするCコードによって、さまざまな内部用途に使用されます。

例えば、Schemeから呼び出し可能で、オプションの引数を取るC関数を記述した場合、インタープリタは受け取らなかった引数に対して`SCM_UNDEFINED`を渡します。

また、これは未定義変数をマークするためにも使用します。

マクロ: `int` **SCM\_UNBNDP** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fUNBNDP)

xが`SCM_UNDEFINED`の場合はtrueを返します。これはxが`SCM_UNBOUND`かどうかをチェックするものではないことに注意してください。歴史は私たちに優しくないでしょう。

* * *

次へ: [ヒープオブジェクトの割り当て](https://doc.guix.gnu.org/guile/latest/en/guile.html#Allocating-Heap-Objects)、前: [即時オブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Immediate-Objects)、上: [Guile の SCM タイプ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.5.3 非即時オブジェクト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dImmediate-Objects-1)

述語 `SCM_IMP` を満たさない `SCM` 型の Scheme オブジェクトは、ヒープ オブジェクトへのエンコードされた参照を保持します。この参照は、`SCM_UNPACK_POINTER` マクロを使用して、ヒープ オブジェクトへの C ポインタにデコードできます。ヒープ オブジェクトへのポインタを `SCM` 値にエンコードするには、`SCM_PACK_POINTER` マクロを使用します。

Guile 2.0以前のGuileは、2ワード単位の「セル」でヒープオブジェクトを割り当てる独自のガベージコレクタを使用していました。Guile 2.0でBDW-GCコレクタに移行したことで、Guileは任意のサイズのヒープオブジェクトを割り当てることができるようになり、セルの概念は廃止されました。しかし、この名前は様々な低レベルインターフェースでまだ使用されているため、ここで言及しておきます。

マクロ: `scm_t_bits *` **SCM\_UNPACK\_POINTER** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fUNPACK_005fPOINTER)

マクロ: `scm_t_cell *` **SCM2PTR** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM2PTR)

非即時 `SCM` オブジェクト x からヒープ オブジェクト ポインタを抽出して返します。`SCM2PTR` という名前は非推奨ですが、まだよく使われています。

マクロ: `SCM_PACK_POINTER` **(scm\_t\_bits** `* x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0028scm_005ft_005fbits)

マクロ: `SCM` **PTR2SCM** `(scm_t_cell * x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-PTR2SCM)

ヒープオブジェクトポインタxへの参照をエンコードした`SCM`値を返します。`PTR2SCM`という名前は非推奨ですが、依然としてよく使われています。

`SCM_UNPACK` を使用して、非即値 `SCM` 値を `scm_t_bits` 変数に変換することも可能です。ただし、`SCM_UNPACK` の結果はヒープ オブジェクトへのポインタとして使用することはできません。`SCM_UNPACK_POINTER` のみが `SCM` オブジェクトを有効なヒープ オブジェクトへのポインタに変換することを保証します。また、有効なヒープ オブジェクトへのポインタではないものに `SCM_PACK_POINTER` を適用することはできません。

まとめ：

* `SCM_IMP` が false の `SCM` 値に対してのみ `SCM_UNPACK_POINTER` を使用してください。
* `(scm_t_cell *) SCM_UNPACK (x)` は使用しないでください。代わりに `SCM_UNPACK_POINTER (x)` を使用してください。
* `SCM_PACK_POINTER` はヒープオブジェクトポインタ以外には使用しないでください！

* * *

次へ: [ヒープオブジェクト型情報](https://doc.guix.gnu.org/guile/latest/en/guile.html#Heap-Object-Type-Information)、前: [非即時オブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Non_002dImmediate-Objects)、上: [Guile の SCM 型](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in -Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.5.4 ヒープオブジェクトの割り当て [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Allocating-Heap-Objects-1)

ヒープオブジェクトは、非即値 `SCM` 値によって指されるヒープに割り当てられたデータです。ヒープオブジェクトの最初のワードには型コードが含まれている必要があります。オブジェクトの長さは任意のワード数でよく、ポインタレス割り当て関数を使用してオブジェクトが割り当てられていない限り、通常はガベージコレクタによって追加のデータがないかスキャンされます。

新しいデータ型を実装する場合や、`<libguile/scm.h>` のコードを完全に理解している場合を除き、通常はこれらの関数は必要ありません。

単にペアを割り当てたい場合は、`scm_cons`を使用してください。

関数: `SCM` **scm\_words** `(scm_t_bits word_0, uint32_t n_words)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fwords)

n_words を含む新しいヒープ オブジェクトを割り当て、最初のスロットを word_0 で初期化し、オブジェクトへのポインタをエンコードした非即値 `SCM` 値を返します。通常、word_0 には型タグが含まれます。

また、2ワードオブジェクトを示すために「cell」という用語を使用する、非推奨ではあるものの一般的な`scm_words`のバリアントも存在します。

関数: `SCM` **scm\_cell** `(scm_t_bits word_0, scm_t_bits word_1)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcell)

新しい2ワードのヒープオブジェクトを割り当て、2つのスロットをword_0とword_1で初期化し、それを返します。これは、`scm_words (word_0, 2)`を呼び出し、2番目のスロットをword_1で初期化するのと同様です。

word_0とword_1は`scm_t_bits`型であることに注意してください。`SCM`オブジェクトを渡す場合は、`SCM_UNPACK`を使用する必要があります。

関数: `SCM` **scm\_double\_cell** `(scm_t_bits word_0, scm_t_bits word_1, scm_t_bits word_2, scm_t_bits word_3)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fdouble_005fcell)

`scm_cell`と同様ですが、4ワードのヒープオブジェクトを割り当てます。

* * *

次へ: [ヒープオブジェクトフィールドへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Heap-Object-Fields)、前: [ヒープオブジェクトの割り当て](https://doc.guix.gnu.org/guile/latest/en/guile.html#Allocating-Heap-Objects)、上: [Guile の SCM タイプ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.5.5 ヒープオブジェクトタイプ情報 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Heap-Object-Type-Information-1)

ヒープオブジェクトには型タグが含まれており、その後にワードサイズの複数のスロットが続きます。オブジェクトの内容の解釈は、オブジェクトの型によって異なります。

マクロ: `scm_t_bits` **SCM\_CELL\_TYPE* `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fTYPE)

xが指すヒープオブジェクトの最初のワードを抽出します。この値にはセルタイプに関する情報が含まれています。

マクロ: `void` **SCM\_SET\_CELL\_TYPE* `(SCM x, scm_t_bits t)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fTYPE)

非即値Schemeオブジェクトxに対して、xが参照するヒープオブジェクトの最初のワードに値tを書き込む。値tは有効なセル型を保持しなければならない。

* * *

前へ: [ヒープオブジェクト型情報](https://doc.guix.gnu.org/guile/latest/en/guile.html#Heap-Object-Type-Information)、上へ: [Guile の SCM 型](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-in-Guile) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 9.2.5.6 ヒープオブジェクトフィールドへのアクセス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Heap-Object-Fields-1)

非直接的な Scheme オブジェクト x の場合、オブジェクトの種類は、前のセクションで説明した `SCM_CELL_TYPE` マクロを使用して判別できます。ヒープ オブジェクトの種類ごとに、タグ付き Scheme オブジェクトを保持するフィールドと、タグなしの生データを保持するフィールドがわかっています。これらの異なるフィールドに適切にアクセスするために、次のマクロが用意されています。

マクロ: `scm_t_bits` **SCM\_CELL\_WORD** `(SCM x, unsigned int n)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fWORD)

マクロ: `scm_t_bits` **SCM\_CELL\_WORD\_0** `(x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fWORD_005f0)

マクロ: `scm_t_bits` **SCM\_CELL\_WORD\_1** `(x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- SCM_005fCELL_005fWORD_005f1)

マクロ: `scm_t_bits` **SCM\_CELL\_WORD\_2** `(x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fWORD_005f2)

マクロ: `scm_t_bits` **SCM\_CELL\_WORD\_3** `(x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fWORD_005f3)

非直接的な Scheme オブジェクト x が参照するヒープ オブジェクトのフィールド n を、タグなしの生データとして出力します。このマクロは、タグなしデータを含むフィールドにのみ使用してください。タグ付きの `SCM` オブジェクトを含むフィールドには使用しないでください。

マクロ: `SCM` **SCM\_CELL\_OBJECT** `(SCM x, unsigned int n)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fOBJECT)

マクロ: `SCM` **SCM\_CELL\_OBJECT\_0** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fOBJECT_005f0)

マクロ: `SCM` **SCM\_CELL\_OBJECT\_1** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fOBJECT_005f1)

マクロ: `SCM` **SCM\_CELL\_OBJECT\_2** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fOBJECT_005f2)

マクロ: `SCM` **SCM\_CELL\_OBJECT\_3** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fCELL_005fOBJECT_005f3)

非直接的な Scheme オブジェクト x が参照するヒープ オブジェクトのフィールド n を Scheme オブジェクトとして返します。このマクロは、タグ付きの `SCM` オブジェクトを含むフィールドにのみ使用してください。タグなしデータを含むフィールドには使用しないでください。

マクロ: `void` **SCM\_SET\_CELL\_WORD** `(SCM x, unsigned int n, scm_t_bits w)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fWORD)

マクロ: `void` **SCM\_SET\_CELL\_WORD\_0** `(x, w)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fWORD_005f0)

マクロ: `void` **SCM\_SET\_CELL\_WORD\_1** `(x, w)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fWORD_005f1)

マクロ: `void` **SCM\_SET\_CELL\_WORD\_2** `(x, w)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fWORD_005f2)

マクロ: `void` **SCM\_SET\_CELL\_WORD\_3** `(x, w)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fWORD_005f3)

生の値 w を、非即値 Scheme 値 x が参照するヒープ オブジェクトのフィールド番号 n に書き込みます。ヒープ オブジェクトに生の値として書き込まれた値は、後で `SCM_CELL_WORD` マクロを使用してのみ読み取ることができます。

マクロ: `void` **SCM\_SET\_CELL\_OBJECT** `(SCM x, unsigned int n, SCM o)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fOBJECT)

マクロ: `void` **SCM\_SET\_CELL\_OBJECT\_0** `(SCM x, SCM o)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fOBJECT_005f0)

マクロ: `void` **SCM\_SET\_CELL\_OBJECT\_1** `(SCM x, SCM o)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- SCM_005fSET_005fCELL_005fOBJECT_005f1)

マクロ: `void` **SCM\_SET\_CELL\_OBJECT\_2** `(SCM x, SCM o)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fOBJECT_005f2)

マクロ: `void` **SCM\_SET\_CELL\_OBJECT\_3** `(SCM x, SCM o)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fSET_005fCELL_005fOBJECT_005f3)

Schemeオブジェクトoを、非即値Scheme値xが参照するヒープオブジェクトのn番目のフィールドに書き込みます。オブジェクトとしてヒープオブジェクトに書き込まれた値は、`SCM_CELL_OBJECT`マクロを使用してのみ読み取ることができます。

まとめ：

* 型が不明な非即時 Scheme オブジェクト x の場合、`SCM_CELL_TYPE (x)` を使用して型情報を取得します。
* 型情報が利用可能になったらすぐに、適切なアクセス方法のみを使用して、さまざまなヒープオブジェクトフィールドへのデータの読み書きを行います。
* フィールド0にはセルタイプの情報が格納されます。一般的に、ヒープオブジェクトに関連付けられたその他のデータはフィールド1から格納されます。

* * *

次へ: [仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine)、前: [データ表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Representation)、上: [Guile の実装](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Implementation) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
