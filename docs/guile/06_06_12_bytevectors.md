#### 6.6.12 バイトベクトル

_bytevector_ は生のバイト列です。`(rnrs bytevectors)` モジュールは、[Revised^6 Report on the Algorithmic Language Scheme (R6RS)](http://www.r6rs.org/) で規定されたプログラミングインターフェイスを提供します。このモジュールには、バイトベクトルを操作し、その内容をさまざまな方法で解釈するための手順が含まれています。たとえば、さまざまなサイズとエンディアンの符号付きまたは符号なし整数、IEEE-754 浮動小数点数、または文字列として解釈できます。バイナリ データのエンコードとデコードに役立つツールです。[R7RS](07_07_r7rs_support.md#77-r7rs-サポート) は、独自のバイトベクトル手順セットを提供します ([R7RS のバイトベクトル手順](#66129-r7rs-の-bytevector-プロシージャ) を参照)。

R6RS（セクション4.3.4）では、バイトベクトルの外部表現が規定されており、バイトベクトルに含まれるオクテット（0～255の範囲の整数）は、`#vu8`で始まるリストとして表現されます。

#vu8(1 53 204)

これは、オクテット 1、53、204 を含む 3 バイトのバイトベクトルを表します。文字列リテラルやブール値などと同様に、バイトベクトルは「自己引用符」であり、引用符で囲む必要はありません。

#vu8(1 53 204)
⇒ #vu8(1 53 204)

バイトベクトルは、バイナリ入出力プリミティブで使用できます（[バイナリ入出力](06_12_input_and_output.md#6122-バイナリ入出力)を参照）。

* [エンディアンネス](#66121-エンディアン)
* [Bytevector の操作](#66122-バイトベクトルの操作)
* [バイトベクトルの内容を整数として解釈する](#66123-バイトベクターの内容を整数として解釈する)
* [バイトベクトルと整数リストの変換](#66124-バイトベクトルと整数リストの変換)
* [バイトベクトルの内容を浮動小数点数として解釈する](#66125-バイトベクトルの内容を浮動小数点数として解釈する)
* [バイトベクトルの内容をUnicode文字列として解釈する](#66126-バイトベクターの内容を-unicode-文字列として解釈する)
* [配列APIを使用したバイトベクトルへのアクセス](#66127-配列-api-を使用したバイトベクトルへのアクセス)
* [SRFI-4 API を使用したバイトベクトルへのアクセス](#66128-srfi-4-api-を使用したバイトベクトルへのアクセス)
* [R7RS の Bytevector プロシージャ](#66129-r7rs-の-bytevector-プロシージャ)
* [バイトベクトルスライス](#661210-バイトベクトルスライス)

* * *

次へ: [バイトベクトルの操作](#66122-バイトベクトルの操作)、上へ: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.12.1 エンディアン

以下の手順の一部は、エンディアンパラメータを受け取ります。エンディアンは、マルチバイト数値のバイトの順序として定義されます。ビッグエンディアンでエンコードされた数値は最上位バイトが最初に書き込まれ、リトルエンディアンでエンコードされた数値は最下位バイトが最初に書き込まれます[11](99_footnotes.md#11)。

リトルエンディアンはIA32アーキテクチャとその派生アーキテクチャのネイティブエンディアンであり、ビッグエンディアンはSPARCやPowerPCなどのネイティブエンディアンです。`native-endianness`プロシージャは、実行中のマシンのネイティブエンディアンを返します。

Scheme 手順: **ネイティブ エンディアン**

C 関数: **scm\_native\_endianness** ()

ホストマシンのネイティブなエンディアンを示す値を返します。

Scheme マクロ: **エンディアン** シンボル

symbolで指定されたエンディアンを示すオブジェクトを返します。symbolが`big`でも`little`でもない場合は、展開時にエラーが発生します。

C 変数: **scm\_endianness\_big**

C 変数: **scm\_endianness\_little**

それぞれビッグエンディアンとリトルエンディアンを表すオブジェクト。

* * *

次へ: [バイトベクトルの内容を整数として解釈する](#66123-バイトベクターの内容を整数として解釈する)、前: [エンディアン](#66121-エンディアン)、上: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.12.2 バイトベクトルの操作

バイトベクトルは、以下の手順とC言語の関数を用いて作成、コピー、解析することができます。

スキーム手順: **make-bytevector** len \[fill\]

C 関数: **scm\_make\_bytevector** (len, fill)

C 関数: **scm\_c\_make\_bytevector** (size\_t len)

lenバイトの新しいバイトベクトルを返します。オプションでfillが指定されている場合は、fillで埋めます。fillは[-128,255]の範囲内である必要があります。

スキームプロシージャ: **bytevector?** obj

C 関数: **scm\_bytevector\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fp )

objがバイトベクターであればtrueを返します。

C 関数: `int` **scm\_is\_bytevector** `(SCM obj)`

`scm_is_true (scm_bytevector_p (obj))` と同等です。

Scheme Procedure: **bytevector-length** bv

C 関数: **scm\_bytevector\_length** (bv)

バイトベクトルbvの長さをバイト単位で返します。

C 関数: `size_t` **scm\_c\_bytevector\_length** `(SCM bv)`

同様に、バイトベクトルbvの長さをバイト単位で返します。

Scheme Procedure: **bytevector=?** bv1 bv2

C 関数: **scm\_bytevector\_eq\_p** (bv1, bv2)

bv1とbv2が等しい場合、つまり長さと内容が同じ場合は、`#t`を返します。

Scheme 手順: **bytevector-fill!** bv fill \[start \[end\]\]

C 関数: **scm\_bytevector\_fill\_x** (bv, fill)

バイトベクトル bv の [start ... end) の位置をバイトで埋めます。start のデフォルト値は 0、end のデフォルト値は bv の長さです。[12](99_footnotes.md#12)

Scheme Procedure: **bytevector-copy!** source source-start target target-start len

C 関数: **scm\_bytevector\_copy\_x** (source, source\_start, target, target\_start, len)

ソースからlenバイトをターゲットにコピーします。読み取りはソース内のインデックスインデックスであるsource-startから開始し、書き込みはターゲット内のインデックスインデックスから行います。

ソース領域とターゲット領域が重複していても構いません。その場合、コピー処理は、ソース領域がまず一時的なバイトベクトルにコピーされ、次に宛先領域にコピーされるかのように行われます。

スキーム手順: **bytevector-copy** bv

C 関数: **scm\_bytevector\_copy** (bv)

新たに割り当てられたbvのコピーを返します。

C 関数: `scm_t_uint8` **scm\_c\_bytevector\_ref** `(SCM bv, size_t index)`

バイトベクターbv内の指定されたインデックスのバイトを返します。

C 関数: `void` **scm\_c\_bytevector\_set\_x** `(SCM bv, size_t index, scm_t_uint8 value)`

bv内のインデックスにあるバイトに値を設定します。

低レベルのC言語マクロが利用可能です。ただし、型チェックは行われないため、使用には注意が必要です。

C マクロ: `size_t` **SCM\_BYTEVECTOR\_LENGTH** `(bv)`

バイトベクトルbvの長さをバイト単位で返します。

C マクロ: `signed char *` **SCM\_BYTEVECTOR\_CONTENTS** `(bv)`

バイトベクターbvの内容へのポインタを返します。

* * *

次へ: [バイトベクトルと整数リストの変換](#66124-バイトベクトルと整数リストの変換)、前: [バイトベクトルの操作](#66122-バイトベクトルの操作)、上: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.12.3 バイトベクターの内容を整数として解釈する

バイトベクトルの内容は、任意のサイズ、符号、エンディアンを持つ整数のシーケンスとして解釈できます。

(let ((bv ([make-bytevector](#66122-バイトベクトルの操作) 4)))
([bytevector-u8-set!](#66123-バイトベクターの内容を整数として解釈する) bv 0 #x12)
([bytevector-u8-set!](#66123-バイトベクターの内容を整数として解釈する) bv 1 #x34)
([bytevector-u8-set!](#66123-バイトベクターの内容を整数として解釈する) bv 2 #x56)
([bytevector-u8-set!](#66123-バイトベクターの内容を整数として解釈する) bv 3 #x78)

(map (lambda (number)
([number->string](06_06_02_numerical_data_types.md#6629-数値と文字列の変換) number 16))
([list](06_06_09_lists.md#6693-リストコンストラクタ) ([bytevector-u8-ref](#66123-バイトベクターの内容を整数として解釈する) bv 0)
([bytevector-u16-ref](#66123-バイトベクターの内容を整数として解釈する) bv 0 ([endianness](#66121-エンディアン) big))
([bytevector-u32-ref](#66123-バイトベクターの内容を整数として解釈する) bv 0 ([endianness](#66121-エンディアン) little)))))

⇒ （"12" "1234" "78563412")

バイトベクトルの内容を整数として解釈するための最も一般的な手順を以下に示します。

スキーム手順: **bytevector-uint-ref** インデックス エンディアン サイズ

C 関数: **scm\_bytevector\_uint\_ref** (bv、インデックス、エンディアン、サイズ)

bv のインデックス index にある size バイト長の符号なし整数を、エンディアンに従ってデコードして返します。

スキーム手順: **bytevector-sint-ref** インデックス エンディアン サイズ

C 関数: **scm\_bytevector\_sint\_ref** (bv、インデックス、エンディアン、サイズ)

bv のインデックス index にある size バイト長の符号付き整数を、エンディアンに従ってデコードして返します。

スキーム手順: **bytevector-uint-set!** bv インデックス値のエンディアンネス サイズ

C 関数: **scm\_bytevector\_uint\_set\_x** (bv、インデックス、値、エンディアン、サイズ)

指定されたインデックスに、サイズバイト長の符号なし整数を値として設定します。値はエンディアンに従ってエンコードされます。

スキーム手順: **bytevector-sint-set!** bv インデックス値のエンディアンネス サイズ

C 関数: **scm\_bytevector\_sint\_set\_x** (bv、インデックス、値、エンディアン、サイズ)

指定されたインデックスに、サイズバイト長の符号付き整数を値として設定します。値はエンディアンに従ってエンコードされます。

以下の手順は上記の手順と似ていますが、指定された整数サイズに合わせて特化されています。

スキーム手順: **bytevector-u8-ref** bv インデックス

スキーム手順: **bytevector-s8-ref** bv インデックス

スキーム手順: **bytevector-u16-ref** bv インデックスのエンディアン

スキーム手順: **bytevector-s16-ref** bv インデックスのエンディアン

スキーム手順: **bytevector-u32-ref** bv インデックスのエンディアン

スキーム手順: **bytevector-s32-ref** bv インデックスのエンディアン

スキーム手順: **bytevector-u64-ref** bv インデックスのエンディアン

スキーム手順: **bytevector-s64-ref** インデックス エンディアンネスによる

C 関数: **scm\_bytevector\_u8\_ref** (bv, index)

C 関数: **scm\_bytevector\_s8\_ref** (bv, index)

C 関数: **scm\_bytevector\_u16\_ref** (bv、インデックス、エンディアン)

C 関数: **scm\_bytevector\_s16\_ref** (bv、インデックス、エンディアン)

C 関数: **scm\_bytevector\_u32\_ref** (bv、インデックス、エンディアン)

C 関数: **scm\_bytevector\_s32\_ref** (bv、インデックス、エンディアン)

C 関数: **scm\_bytevector\_u64\_ref** (bv、インデックス、エンディアン)

C 関数: **scm\_bytevector\_s64\_ref** (bv、インデックス、エンディアン)

bv の指定されたインデックスから、符号なし n ビット (符号付き) 整数 (n は 8、16、32、または 64) をエンディアンに従ってデコードして返します。

スキーム手順: **bytevector-u8-set!** bv インデックス値

スキーム手順: **bytevector-s8-set!** bv インデックス値

スキーム手順: **bytevector-u16-set!** インデックス値のエンディアンネス

スキーム手順: **bytevector-s16-set!** bv インデックス値のエンディアン

スキーム手順: **bytevector-u32-set!** bv インデックス値のエンディアン

スキーム手順: **bytevector-s32-set!** bv インデックス値のエンディアン

スキーム手順: **bytevector-u64-set!** bv インデックス値のエンディアン

スキーム手順: **bytevector-s64-set!** インデックス値のエンディアンネス

C 関数: **scm\_bytevector\_u8\_set\_x** (bv, index, value)

C 関数: **scm\_bytevector\_s8\_set\_x** (bv, index, value)

C 関数: **scm\_bytevector\_u16\_set\_x** (bv、インデックス、値、エンディアン)

C 関数: **scm\_bytevector\_s16\_set\_x** (bv、インデックス、値、エンディアン)

C 関数: **scm\_bytevector\_u32\_set\_x** (bv、インデックス、値、エンディアン)

C 関数: **scm\_bytevector\_s32\_set\_x** (bv、インデックス、値、エンディアン)

C 関数: **scm\_bytevector\_u64\_set\_x** (bv、インデックス、値、エンディアン)

C 関数: **scm\_bytevector\_s64\_set\_x** (bv、インデックス、値、エンディアン)

値をnビット（符号付き）整数（nは8、16、32、または64）として、インデックスbvに格納し、エンディアンに従ってエンコードします。

最後に、これらの各関数には、ホストのエンディアンに特化したバリアントが用意されています（ただし、エンディアンはバイト順序に関するものであり、バイトは1つしかないため、`u8`および`s8`アクセサは除きます）。

スキーム手順: **bytevector-u16-native-ref** bv インデックス

スキーム手順: **bytevector-s16-native-ref** bv index

Scheme Procedure: **bytevector-u32-native-ref** bv index

Scheme Procedure: **bytevector-s32-native-ref** bv index

Scheme Procedure: **bytevector-u64-native-ref** bv index

スキーム手順: **bytevector-s64-native-ref** bv インデックス

C 関数: **scm\_bytevector\_u16\_native\_ref** (bv, index)

C 関数: **scm\_bytevector\_s16\_native\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fbytevector_005fs16_005fnative_005fref)

C 関数: **scm\_bytevector\_u32\_native\_ref** (bv, index)

C 関数: **scm\_bytevector\_s32\_native\_ref** (bv, index)

C 関数: **scm\_bytevector\_u64\_native\_ref** (bv, index)

C 関数: **scm\_bytevector\_s64\_native\_ref** (bv, index)

bv の指定されたインデックスから、ホストのネイティブエンディアンに従ってデコードされた、符号なし n ビット (符号付き) 整数 (n は 8、16、32、または 64) を返します。

スキーム手順: **bytevector-u16-native-set!** bv インデックス値

スキーム手順: **bytevector-s16-native-set!** bv インデックス値

スキーム手順: **bytevector-u32-native-set!** bv インデックス値

Scheme Procedure: **bytevector-s32-native-set!** bv インデックス値

Scheme Procedure: **bytevector-u64-native-set!** bv インデックス値

スキーム手順: **bytevector-s64-native-set!** bv インデックス値

C 関数: **scm\_bytevector\_u16\_native\_set\_x** (bv, index, value)

C 関数: **scm\_bytevector\_s16\_native\_set\_x** (bv, index, value)

C 関数: **scm\_bytevector\_u32\_native\_set\_x** (bv, index, value)

C 関数: **scm\_bytevector\_s32\_native\_set\_x** (bv, index, value)

C 関数: **scm\_bytevector\_u64\_native\_set\_x** (bv, index, value)

C 関数: **scm\_bytevector\_s64\_native\_set\_x** (bv, index, value)

値をnビット（符号付き）整数（nは8、16、32、または64）としてbvのインデックスに格納し、ホストのネイティブエンディアンに従ってエンコードします。

* * *

次へ: [バイトベクトルの内容を浮動小数点数として解釈する](#66125-バイトベクトルの内容を浮動小数点数として解釈する)、前: [バイトベクトルの内容を整数として解釈する](#66123-バイトベクターの内容を整数として解釈する)、上: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.12.4 バイトベクトルと整数リストの変換

バイトベクターの内容は、符号付き整数または符号なし整数のリストとの間で容易に変換できます。

([bytevector->sint-list](#66124-バイトベクトルと整数リストの変換) ([u8-list->bytevector](#66124-バイトベクトルと整数リストの変換) ([make-list](06_06_09_lists.md#6693-リストコンストラクタ) 4 255))
([エンディアン](#66121-エンディアン) 小さい) 2)
⇒ (\-1 \-1)

Scheme Procedure: **bytevector->u8-list** bv

C 関数: **scm\_bytevector\_to\_u8\_list** (bv)

bvの内容から、新たに割り当てられた符号なし8ビット整数のリストを返します。

Scheme手順: **u8-list->bytevector** lst

C 関数: **scm\_u8\_list\_to\_bytevector** (lst)

lst にリストされている符号なし 8 ビット整数で構成される、新しく割り当てられたバイトベクトルを返します。

スキーム手順: **bytevector->uint-list** bv エンディアン サイズ

C 関数: **scm\_bytevector\_to\_uint\_list** (bv、endianness、size)

bv の内容を表す、サイズが バイトの符号なし整数のリストを、エンディアンに従ってデコードして返します。

スキーム手順: **bytevector->sint-list** エンディアンサイズ別

C 関数: **scm\_bytevector\_to\_sint\_list** (bv、endianness、size)

bv の内容を表す、サイズが バイトの符号付き整数のリストを、エンディアンに従ってデコードして返します。

Scheme手順: **uint-list->bytevector** lstエンディアンネスサイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- uint_002dlist_002d_003ebytevector)

C 関数: **scm\_uint\_list\_to\_bytevector** (lst、endianness、size)

lst にリストされている符号なし整数をエンディアンに従って size バイトにエンコードした新しいバイトベクトルを返します。

Scheme手順: **sint-list->bytevector** lst エンディアン サイズ

C 関数: **scm\_sint\_list\_to\_bytevector** (lst、endianness、size)

lst にリストされている符号付き整数をエンディアンに従って size バイトにエンコードした新しいバイトベクトルを返します。

* * *

次へ: [バイトベクトルの内容をUnicode文字列として解釈する](#66126-バイトベクターの内容を-unicode-文字列として解釈する)、前: [バイトベクトルと整数リストの変換](#66124-バイトベクトルと整数リストの変換)、上: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 6.6.12.5 バイトベクトルの内容を浮動小数点数として解釈する

バイトベクターの内容は、ここで説明する手順を使用することで、IEEE-754規格の単精度または倍精度浮動小数点数（それぞれ32ビットおよび64ビット長）としてアクセスすることもできます。

スキーム手順: **bytevector-ieee-single-ref** インデックス エンディアンネスによる

スキーム手順: **bytevector-ieee-double-ref** bv インデックス エンディアン

C 関数: **scm\_bytevector\_ieee\_single\_ref** (bv、インデックス、エンディアン)

C 関数: **scm\_bytevector\_ieee\_double\_ref** (bv、インデックス、エンディアン)

bv から指定されたインデックスの IEEE-754 単精度浮動小数点数をエンディアンに従って返します。

スキーム手順: **bytevector-ieee-single-set!** bv インデックス値のエンディアンネス

スキーム手順: **bytevector-ieee-double-set!** bv インデックス値のエンディアン

C 関数: **scm\_bytevector\_ieee\_single\_set\_x** (bv、インデックス、値、エンディアン)

C 関数: **scm\_bytevector\_ieee\_double\_set\_x** (bv、インデックス、値、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fbytevector_005fieee_005fdouble_005fset_005fx)

実数値を、エンディアンに従ってインデックスbvに格納します。

専門的な処置も利用可能です。

スキーム手順: **bytevector-ieee-single-native-ref** bv インデックス

Scheme Procedure: **bytevector-ieee-double-native-ref** bv index

C 関数: **scm\_bytevector\_ieee\_single\_native\_ref** (bv, index)

C 関数: **scm\_bytevector\_ieee\_double\_native\_ref** (bv, index)

ホストのネイティブエンディアンに従って、bv の指定されたインデックスから IEEE-754 単精度浮動小数点数を返します。

スキーム手順: **bytevector-ieee-single-native-set!** bv インデックス値

スキーム手順: **bytevector-ieee-double-native-set!** bv インデックス値

C 関数: **scm\_bytevector\_ieee\_single\_native\_set\_x** (bv, index, value)

C 関数: **scm\_bytevector\_ieee\_double\_native\_set\_x** (bv, index, value)

ホストのネイティブエンディアンに従って、実数値をbvの指定されたインデックスに格納します。

* * *

次へ: [配列 API を使用したバイトベクトルへのアクセス](#66127-配列-api-を使用したバイトベクトルへのアクセス)、前: [バイトベクトルの内容を浮動小数点数として解釈する](#66125-バイトベクトルの内容を浮動小数点数として解釈する)、上: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.12.6 バイトベクターの内容を Unicode 文字列として解釈する

バイトベクターの内容は、最も一般的に使用されているエンコード形式のいずれかでエンコードされた Unicode 文字列として解釈することもできます。より汎用的なインターフェースについては、[文字列をバイトとして表現する](06_06_05_strings.md#66513-文字列をバイトとして表現する) を参照してください。

([utf8->string](#66126-バイトベクターの内容を-unicode-文字列として解釈する) ([u8-list->bytevector](#66124-バイトベクトルと整数リストの変換) '(99 97 102 101)))
⇒ 「カフェ」

([string->utf8](#66126-バイトベクターの内容を-unicode-文字列として解釈する) "café") ;; 鋭アクセント付き小文字ラテン文字E
⇒ #vu8(99 97 102 195 169)

Scheme手順: **string-utf8-length** `str`

C 関数: `SCM` **scm\_string\_utf8\_length** `(str)`

C 関数: `size_t` **scm\_c\_string\_utf8\_length** `(str)`

str の UTF-8 表現におけるバイト数を返します。

Scheme手順: **string->utf8** str

Scheme手順: **string->utf16** str \[endianness\]

Scheme手順: **string->utf32** str \[endianness\]

C 関数: **scm\_string\_to\_utf8** (str)

C 関数: **scm\_string\_to\_utf16** (str, endianness)

C 関数: **scm\_string\_to\_utf32** (str, endianness)

str の UTF-8、UTF-16、または UTF-32 (別名 UCS-4) エンコーディングを含む、新しく割り当てられたバイトベクトルを返します。UTF-16 および UTF-32 の場合、エンディアンは `big` または `little` の記号で指定する必要があります。省略した場合、デフォルトでビッグエンディアンになります。

Scheme Procedure: **utf8->string** utf

Scheme Procedure: **utf16->string** utf \[endianness\]

Scheme Procedure: **utf32->string** utf \[endianness\]

C 関数: **scm\_utf8\_to\_string** (utf)

C 関数: **scm\_utf16\_to\_string** (utf、エンディアン)

C 関数: **scm\_utf32\_to\_string** (utf、エンディアン)

バイトベクターutfのUTF-8、UTF-16、またはUTF-32デコードされた内容を含む、新たに割り当てられた文字列を返します。UTF-16およびUTF-32の場合、エンディアンは「big」または「little」の記号で指定する必要があります。省略した場合、デフォルトでビッグエンディアンが使用されます。

* * *

次へ: [SRFI-4 API を使用したバイトベクトルへのアクセス](#66128-srfi-4-api-を使用したバイトベクトルへのアクセス)、前: [バイトベクトルの内容を Unicode 文字列として解釈する](#66126-バイトベクターの内容を-unicode-文字列として解釈する)、上: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 6.6.12.7 配列 API を使用したバイトベクトルへのアクセス

R6RSの拡張機能として、Guileでは_array_プロシージャを使用してバイトベクトルを操作できます（[Arrays](06_06_13_arrays.md#6613-配列)を参照）。これらのAPIを使用する場合、バイトは8ビット符号なし整数として1バイトずつアクセスされます。

(define bv #vu8(0 1 2 3))

(配列? bv)
⇒ #t

(配列ランク bv)
⇒ 1

(配列参照 bv 2)
⇒ 2

;; 配列セットの引数の順序が異なることに注意してください!。
(配列セット! bv 77 2)
(配列参照 bv 2)
⇒ 77

(配列型 bv)
⇒ vu8

* * *

次へ: [R7RS のバイトベクトル手続き](#66129-r7rs-の-bytevector-プロシージャ)、前: [配列API を使用したバイトベクトルへのアクセス](#66127-配列-api-を使用したバイトベクトルへのアクセス)、上: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.12.8 SRFI-4 API を使用したバイトベクトルへのアクセス

バイトベクトルは、SRFI-4 API を使用してアクセスすることもできます。詳細については、[SRFI-4 - バイトベクトルとの関係](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7553-srfi-4---バイトベクトルとの関係) を参照してください。

* * *

次へ: [バイトベクトル スライス](#661210-バイトベクトルスライス)、前: [SRFI-4 API を使用したバイトベクトルへのアクセス](#66128-srfi-4-api-を使用したバイトベクトルへのアクセス)、上: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.12.9 R7RS の Bytevector プロシージャ

[R7RS](07_07_r7rs_support.md#77-r7rs-サポート) (セクション 6.9) では、バイトベクトル操作手順のセットが定義されており、

(use-modules (scheme base))

これらのうち、[`make-bytevector`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dmake_002dbyteve ctor)、[`bytevector?`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dbytevector_003f)、[`bytevector-length`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dbytevector_002dlength)、[`bytevector-u8-ref`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dbytevector_002du8_002dref)および[`bytevector-u8-set!`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dbytevector_002du8_002dset_0021) は R6RS と同じ定義です。以下にリストされているプロシージャは、R7RS と R6RS で定義が異なるか、R6RS では定義されていません。

スキームプロシージャ: **bytevector** arg …

指定された引数から構成される、新たに割り当てられたバイトベクトルを返します。`list` と同様です。

([bytevector](#66129-r7rs-の-bytevector-プロシージャ) 2 3 4) ⇒ #vu8(2 3 4)

[`u8-list->bytevector`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002du8_002dlist_002d_003ebytevector)も参照してください。

スキーム手順: **bytevector-copy** bv \[start \[end\]\]

bv の要素を [start ... end] の範囲で含む、新しく割り当てられたバイトベクトルを返します。start のデフォルト値は 0、end のデフォルト値は bv の長さです。

(define bv #vu8(0 1 2 3 4 5))
([bytevector-copy](#66122-バイトベクトルの操作) bv) ⇒ #vu8(0 1 2 3 4 5)
([bytevector-copy](#66122-バイトベクトルの操作) bv 2) ⇒ #vu8(2 3 4 5)
([bytevector-copy](#66122-バイトベクトルの操作) bv 2 4) ⇒ #vu8(2 3)

[R6RS バージョン](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dr6_003abytevector_002dcopy)も参照してください。

Scheme Procedure: **bytevector-copy!** dst at src \[start \[end\]\]

バイトベクターsrcから範囲[start ... end)の要素ブロックを、位置atからバイトベクターdstにコピーします。startのデフォルト値は0、endのデフォルト値はsrcの長さです。dstの長さがat + (end - start)より小さい場合はエラーとなります。

[R6RS バージョン](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dr6_003abytevector_002dcopy_0021)も参照してください。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) ((rnrs bytevectors) #:prefix r6:)
((スキームベース) #:プレフィックス r7:))

以下の呼び出しは同等です。

(r6:バイトベクトルコピー! ソース ソース開始 ターゲット ターゲット開始 長さ)
(r7:bytevector-copy! target target-start source source-start ([+](06_06_02_numerical_data_types.md#66211-算術関数) source-start len))

スキームプロシージャ: **bytevector-append** arg …

指定されたバイトベクターargを連結した文字で構成される、新たに割り当てられたバイトベクターを返します。

([bytevector-append](#66129-r7rs-の-bytevector-プロシージャ) #vu8(0 1 2) #vu8(3 4 5))
⇒ #vu8(0 1 2 3 4 5)

* * *

前へ: [R7RS のバイトベクトル手順](#66129-r7rs-の-bytevector-プロシージャ)、上へ: [バイトベクトル](#6612-バイトベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.12.10 バイトベクトルスライス

R6RS仕様の拡張として、`(rnrs bytevectors gnu)`モジュールは、既存のバイトベクターの一部をエイリアスしたバイトベクターを返す`bytevector-slice`プロシージャを提供します。

スキーム手順: **bytevector-slice** bv オフセット \[size\]

C 関数: **scm\_bytevector\_slice** (bv、オフセット、サイズ)

オフセットから始まり、サイズバイト数だけカウントされたbvのスライスを返します。サイズを省略した場合、スライスはオフセットから始まるbv全体をカバーします。返されるスライスはbvとストレージを共有します。スライスへの変更はbvに反映され、その逆も同様です。

bvが実際にSRFI-4均一ベクトルである場合、オフセットとサイズがその要素タイプのサイズに揃っていない限り、その要素タイプは保持されます。

以下に、その使用方法を示す例を示します。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (rnrs バイトベクター)
(rnrs バイトベクトル gnu)

(define bv ([u8-list->bytevector](#66124-バイトベクトルと整数リストの変換) ([iota](07_05_03_srfi1_list_library.md#7531-コンストラクタ) 10)))
(define slice ([bytevector-slice](#661210-バイトベクトルスライス) bv 2 3))

スライス
⇒ #vu8(2 3 4)

([bytevector-u8-set!](#66123-バイトベクターの内容を整数として解釈する) スライス 0 77)
スライス
⇒ #vu8(77 3 4)

bv
⇒ #vu8(0 1 77 3 4 5 6 7 8 9)

* * *

次へ: [VLists](06_06_14_vlists.md#6614-vlists)、前: [Bytevectors](#6612-バイトベクトル)、上: [Data Types](06_06_00_data_types.md#66-データ型) \[[Contents](00_contents.md "目次")\]\[[Index](index_r5rs.md "Index")\]
