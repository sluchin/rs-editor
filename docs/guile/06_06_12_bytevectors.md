#### 6.6.12 バイトベクトル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-1)

_bytevector_ は生のバイト列です。`(rnrs bytevectors)` モジュールは、[Revised^6 Report on the Algorithmic Language Scheme (R6RS)](http://www.r6rs.org/) で規定されたプログラミングインターフェイスを提供します。このモジュールには、バイトベクトルを操作し、その内容をさまざまな方法で解釈するための手順が含まれています。たとえば、さまざまなサイズとエンディアンの符号付きまたは符号なし整数、IEEE-754 浮動小数点数、または文字列として解釈できます。バイナリ データのエンコードとデコードに役立つツールです。[R7RS](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Support) は、独自のバイトベクトル手順セットを提供します ([R7RS のバイトベクトル手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Procedures-in-R7RS) を参照)。

R6RS（セクション4.3.4）では、バイトベクトルの外部表現が規定されており、バイトベクトルに含まれるオクテット（0～255の範囲の整数）は、`#vu8`で始まるリストとして表現されます。

#vu8(1 53 204)

これは、オクテット 1、53、204 を含む 3 バイトのバイトベクトルを表します。文字列リテラルやブール値などと同様に、バイトベクトルは「自己引用符」であり、引用符で囲む必要はありません。

#vu8(1 53 204)
⇒ #vu8(1 53 204)

バイトベクトルは、バイナリ入出力プリミティブで使用できます（[バイナリ入出力](https://doc.guix.gnu.org/guile/latest/en/guile.html#Binary-I_002fO)を参照）。

* [エンディアンネス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Endianness)
* [Bytevector の操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Manipulation)
* [バイトベクトルの内容を整数として解釈する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Integers)
* [バイトベクトルと整数リストの変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-and-Integer-Lists)
* [バイトベクトルの内容を浮動小数点数として解釈する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Floats)
* [バイトベクトルの内容をUnicode文字列として解釈する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Strings)
* [配列APIを使用したバイトベクトルへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Arrays)
* [SRFI-4 API を使用したバイトベクトルへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Uniform-Vectors)
* [R7RS の Bytevector プロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Procedures-in-R7RS)
* [バイトベクトルスライス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Slices)

* * *

次へ: [バイトベクトルの操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Manipulation)、上へ: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.12.1 エンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Endianness)

以下の手順の一部は、エンディアンパラメータを受け取ります。エンディアンは、マルチバイト数値のバイトの順序として定義されます。ビッグエンディアンでエンコードされた数値は最上位バイトが最初に書き込まれ、リトルエンディアンでエンコードされた数値は最下位バイトが最初に書き込まれます[11](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT11)。

リトルエンディアンはIA32アーキテクチャとその派生アーキテクチャのネイティブエンディアンであり、ビッグエンディアンはSPARCやPowerPCなどのネイティブエンディアンです。`native-endianness`プロシージャは、実行中のマシンのネイティブエンディアンを返します。

Scheme 手順: **ネイティブ エンディアン** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-native_002dendianness)

C 関数: **scm\_native\_endianness** () [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fnative_005fendianness)

ホストマシンのネイティブなエンディアンを示す値を返します。

Scheme マクロ: **エンディアン** シンボル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endianness-1)

symbolで指定されたエンディアンを示すオブジェクトを返します。symbolが`big`でも`little`でもない場合は、展開時にエラーが発生します。

C 変数: **scm\_endianness\_big** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fendianness_005fbig)

C 変数: **scm\_endianness\_little** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fendianness_005flittle)

それぞれビッグエンディアンとリトルエンディアンを表すオブジェクト。

* * *

次へ: [バイトベクトルの内容を整数として解釈する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Integers)、前: [エンディアン](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Endianness)、上: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.12.2 バイトベクトルの操作 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Manipulating-Bytevectors)

バイトベクトルは、以下の手順とC言語の関数を用いて作成、コピー、解析することができます。

スキーム手順: **make-bytevector** len \[fill\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dbytevector)

C 関数: **scm\_make\_bytevector** (len, fill) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fbytevector)

C 関数: **scm\_c\_make\_bytevector** (size\_t len) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fmake_005fbytevector)

lenバイトの新しいバイトベクトルを返します。オプションでfillが指定されている場合は、fillで埋めます。fillは[-128,255]の範囲内である必要があります。

スキームプロシージャ: **bytevector?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_003f)

C 関数: **scm\_bytevector\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fp )

objがバイトベクターであればtrueを返します。

C 関数: `int` **scm\_is\_bytevector** `(SCM obj)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005fbytevector)

`scm_is_true (scm_bytevector_p (obj))` と同等です。

Scheme Procedure: **bytevector-length** bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dlength)

C 関数: **scm\_bytevector\_length** (bv) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005flength)

バイトベクトルbvの長さをバイト単位で返します。

C 関数: `size_t` **scm\_c\_bytevector\_length** `(SCM bv)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fbytevector_005flength)

同様に、バイトベクトルbvの長さをバイト単位で返します。

Scheme Procedure: **bytevector=?** bv1 bv2 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_003d_003f)

C 関数: **scm\_bytevector\_eq\_p** (bv1, bv2) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005feq_005fp)

bv1とbv2が等しい場合、つまり長さと内容が同じ場合は、`#t`を返します。

Scheme 手順: **bytevector-fill!** bv fill \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dfill_0021)

C 関数: **scm\_bytevector\_fill\_x** (bv, fill) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005ffill_005fx)

バイトベクトル bv の [start ... end) の位置をバイトで埋めます。start のデフォルト値は 0、end のデフォルト値は bv の長さです。[12](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT12)

Scheme Procedure: **bytevector-copy!** source source-start target target-start len [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dcopy_0021)

C 関数: **scm\_bytevector\_copy\_x** (source, source\_start, target, target\_start, len) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fcopy_005fx)

ソースからlenバイトをターゲットにコピーします。読み取りはソース内のインデックスインデックスであるsource-startから開始し、書き込みはターゲット内のインデックスインデックスから行います。

ソース領域とターゲット領域が重複していても構いません。その場合、コピー処理は、ソース領域がまず一時的なバイトベクトルにコピーされ、次に宛先領域にコピーされるかのように行われます。

スキーム手順: **bytevector-copy** bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dcopy)

C 関数: **scm\_bytevector\_copy** (bv) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fcopy)

新たに割り当てられたbvのコピーを返します。

C 関数: `scm_t_uint8` **scm\_c\_bytevector\_ref** `(SCM bv, size_t index)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fbytevector_005fref)

バイトベクターbv内の指定されたインデックスのバイトを返します。

C 関数: `void` **scm\_c\_bytevector\_set\_x** `(SCM bv, size_t index, scm_t_uint8 value)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fbytevector_005fset_005fx)

bv内のインデックスにあるバイトに値を設定します。

低レベルのC言語マクロが利用可能です。ただし、型チェックは行われないため、使用には注意が必要です。

C マクロ: `size_t` **SCM\_BYTEVECTOR\_LENGTH** `(bv)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fBYTEVECTOR_005fLENGTH)

バイトベクトルbvの長さをバイト単位で返します。

C マクロ: `signed char *` **SCM\_BYTEVECTOR\_CONTENTS** `(bv)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fBYTEVECTOR_005fCONTENTS)

バイトベクターbvの内容へのポインタを返します。

* * *

次へ: [バイトベクトルと整数リストの変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-and-Integer-Lists)、前: [バイトベクトルの操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Manipulation)、上: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.12.3 バイトベクターの内容を整数として解釈する[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Interpreting-Bytevector-Contents-as-Integers)

バイトベクトルの内容は、任意のサイズ、符号、エンディアンを持つ整数のシーケンスとして解釈できます。

(let ((bv ([make-bytevector](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dbytevector) 4)))
([bytevector-u8-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dset_0021) bv 0 #x12)
([bytevector-u8-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dset_0021) bv 1 #x34)
([bytevector-u8-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dset_0021) bv 2 #x56)
([bytevector-u8-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dset_0021) bv 3 #x78)

(map (lambda (number)
([number->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-number_002d_003estring) number 16))
([list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list-1) ([bytevector-u8-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dref) bv 0)
([bytevector-u16-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du16_002dref) bv 0 ([endianness](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endianness-1) big))
([bytevector-u32-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du32_002dref) bv 0 ([endianness](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endianness-1) little)))))

⇒ （"12" "1234" "78563412")

バイトベクトルの内容を整数として解釈するための最も一般的な手順を以下に示します。

スキーム手順: **bytevector-uint-ref** インデックス エンディアン サイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002duint_002dref)

C 関数: **scm\_bytevector\_uint\_ref** (bv、インデックス、エンディアン、サイズ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fuint_005fref)

bv のインデックス index にある size バイト長の符号なし整数を、エンディアンに従ってデコードして返します。

スキーム手順: **bytevector-sint-ref** インデックス エンディアン サイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dsint_002dref)

C 関数: **scm\_bytevector\_sint\_ref** (bv、インデックス、エンディアン、サイズ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fsint_005fref)

bv のインデックス index にある size バイト長の符号付き整数を、エンディアンに従ってデコードして返します。

スキーム手順: **bytevector-uint-set!** bv インデックス値のエンディアンネス サイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002duint_002dset_0021)

C 関数: **scm\_bytevector\_uint\_set\_x** (bv、インデックス、値、エンディアン、サイズ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fuint_005fset_005fx)

指定されたインデックスに、サイズバイト長の符号なし整数を値として設定します。値はエンディアンに従ってエンコードされます。

スキーム手順: **bytevector-sint-set!** bv インデックス値のエンディアンネス サイズ[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dsint_002dset_0021)

C 関数: **scm\_bytevector\_sint\_set\_x** (bv、インデックス、値、エンディアン、サイズ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fsint_005fset_005fx)

指定されたインデックスに、サイズバイト長の符号付き整数を値として設定します。値はエンディアンに従ってエンコードされます。

以下の手順は上記の手順と似ていますが、指定された整数サイズに合わせて特化されています。

スキーム手順: **bytevector-u8-ref** bv インデックス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dref)

スキーム手順: **bytevector-s8-ref** bv インデックス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds8_002dref)

スキーム手順: **bytevector-u16-ref** bv インデックスのエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du16_002dref)

スキーム手順: **bytevector-s16-ref** bv インデックスのエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds16_002dref)

スキーム手順: **bytevector-u32-ref** bv インデックスのエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du32_002dref)

スキーム手順: **bytevector-s32-ref** bv インデックスのエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds32_002dref)

スキーム手順: **bytevector-u64-ref** bv インデックスのエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du64_002dref)

スキーム手順: **bytevector-s64-ref** インデックス エンディアンネスによる [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds64_002dref)

C 関数: **scm\_bytevector\_u8\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu8_005fref)

C 関数: **scm\_bytevector\_s8\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs8_005fref)

C 関数: **scm\_bytevector\_u16\_ref** (bv、インデックス、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu16_005fref)

C 関数: **scm\_bytevector\_s16\_ref** (bv、インデックス、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs16_005fref)

C 関数: **scm\_bytevector\_u32\_ref** (bv、インデックス、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu32_005fref)

C 関数: **scm\_bytevector\_s32\_ref** (bv、インデックス、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs32_005fref)

C 関数: **scm\_bytevector\_u64\_ref** (bv、インデックス、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu64_005fref)

C 関数: **scm\_bytevector\_s64\_ref** (bv、インデックス、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs64_005fref)

bv の指定されたインデックスから、符号なし n ビット (符号付き) 整数 (n は 8、16、32、または 64) をエンディアンに従ってデコードして返します。

スキーム手順: **bytevector-u8-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dset_0021)

スキーム手順: **bytevector-s8-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds8_002dset_0021)

スキーム手順: **bytevector-u16-set!** インデックス値のエンディアンネス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du16_002dset_0021)

スキーム手順: **bytevector-s16-set!** bv インデックス値のエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds16_002dset_0021)

スキーム手順: **bytevector-u32-set!** bv インデックス値のエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du32_002dset_0021)

スキーム手順: **bytevector-s32-set!** bv インデックス値のエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds32_002dset_0021)

スキーム手順: **bytevector-u64-set!** bv インデックス値のエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du64_002dset_0021)

スキーム手順: **bytevector-s64-set!** インデックス値のエンディアンネス[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds64_002dset_0021)

C 関数: **scm\_bytevector\_u8\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu8_005fset_005fx)

C 関数: **scm\_bytevector\_s8\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs8_005fset_005fx)

C 関数: **scm\_bytevector\_u16\_set\_x** (bv、インデックス、値、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu16_005fset_005fx)

C 関数: **scm\_bytevector\_s16\_set\_x** (bv、インデックス、値、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs16_005fset_005fx)

C 関数: **scm\_bytevector\_u32\_set\_x** (bv、インデックス、値、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu32_005fset_005fx)

C 関数: **scm\_bytevector\_s32\_set\_x** (bv、インデックス、値、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs32_005fset_005fx)

C 関数: **scm\_bytevector\_u64\_set\_x** (bv、インデックス、値、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu64_005fset_005fx)

C 関数: **scm\_bytevector\_s64\_set\_x** (bv、インデックス、値、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs64_005fset_005fx)

値をnビット（符号付き）整数（nは8、16、32、または64）として、インデックスbvに格納し、エンディアンに従ってエンコードします。

最後に、これらの各関数には、ホストのエンディアンに特化したバリアントが用意されています（ただし、エンディアンはバイト順序に関するものであり、バイトは1つしかないため、`u8`および`s8`アクセサは除きます）。

スキーム手順: **bytevector-u16-native-ref** bv インデックス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du16_002dnative_002dref)

スキーム手順: **bytevector-s16-native-ref** bv index [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds16_002dnative_002dref)

Scheme Procedure: **bytevector-u32-native-ref** bv index [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du32_002dnative_002dref)

Scheme Procedure: **bytevector-s32-native-ref** bv index [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds32_002dnative_002dref)

Scheme Procedure: **bytevector-u64-native-ref** bv index [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du64_002dnative_002dref)

スキーム手順: **bytevector-s64-native-ref** bv インデックス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds64_002dnative_002dref)

C 関数: **scm\_bytevector\_u16\_native\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu16_005fnative_005fref)

C 関数: **scm\_bytevector\_s16\_native\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fbytevector_005fs16_005fnative_005fref)

C 関数: **scm\_bytevector\_u32\_native\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu32_005fnative_005fref)

C 関数: **scm\_bytevector\_s32\_native\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs32_005fnative_005fref)

C 関数: **scm\_bytevector\_u64\_native\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu64_005fnative_005fref)

C 関数: **scm\_bytevector\_s64\_native\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs64_005fnative_005fref)

bv の指定されたインデックスから、ホストのネイティブエンディアンに従ってデコードされた、符号なし n ビット (符号付き) 整数 (n は 8、16、32、または 64) を返します。

スキーム手順: **bytevector-u16-native-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du16_002dnative_002dset_0021)

スキーム手順: **bytevector-s16-native-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds16_002dnative_002dset_0021)

スキーム手順: **bytevector-u32-native-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du32_002dnative_002dset_0021)

Scheme Procedure: **bytevector-s32-native-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds32_002dnative_002dset_0021)

Scheme Procedure: **bytevector-u64-native-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du64_002dnative_002dset_0021)

スキーム手順: **bytevector-s64-native-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002ds64_002dnative_002dset_0021)

C 関数: **scm\_bytevector\_u16\_native\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu16_005fnative_005fset_005fx)

C 関数: **scm\_bytevector\_s16\_native\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs16_005fnative_005fset_005fx)

C 関数: **scm\_bytevector\_u32\_native\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu32_005fnative_005fset_005fx)

C 関数: **scm\_bytevector\_s32\_native\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs32_005fnative_005fset_005fx)

C 関数: **scm\_bytevector\_u64\_native\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fu64_005fnative_005fset_005fx)

C 関数: **scm\_bytevector\_s64\_native\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fs64_005fnative_005fset_005fx)

値をnビット（符号付き）整数（nは8、16、32、または64）としてbvのインデックスに格納し、ホストのネイティブエンディアンに従ってエンコードします。

* * *

次へ: [バイトベクトルの内容を浮動小数点数として解釈する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Floats)、前: [バイトベクトルの内容を整数として解釈する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Integers)、上: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.12.4 バイトベクトルと整数リストの変換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Converting-Bytevectors-to_002ffrom-Integer-Lists)

バイトベクターの内容は、符号付き整数または符号なし整数のリストとの間で容易に変換できます。

([bytevector->sint-list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002d_003esint_002dlist) ([u8-list->bytevector](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u8_002dlist_002d_003ebytevector) ([make-list](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dlist) 4 255))
([エンディアン](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-endianness-1) 小さい) 2)
⇒ (\-1 \-1)

Scheme Procedure: **bytevector->u8-list** bv [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002d_003eu8_002dlist)

C 関数: **scm\_bytevector\_to\_u8\_list** (bv) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fto_005fu8_005flist)

bvの内容から、新たに割り当てられた符号なし8ビット整数のリストを返します。

Scheme手順: **u8-list->bytevector** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u8_002dlist_002d_003ebytevector)

C 関数: **scm\_u8\_list\_to\_bytevector** (lst) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fu8_005flist_005fto_005fbytevector)

lst にリストされている符号なし 8 ビット整数で構成される、新しく割り当てられたバイトベクトルを返します。

スキーム手順: **bytevector->uint-list** bv エンディアン サイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002d_003euint_002dlist)

C 関数: **scm\_bytevector\_to\_uint\_list** (bv、endianness、size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fto_005fuint_005flist)

bv の内容を表す、サイズが バイトの符号なし整数のリストを、エンディアンに従ってデコードして返します。

スキーム手順: **bytevector->sint-list** エンディアンサイズ別 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002d_003esint_002dlist)

C 関数: **scm\_bytevector\_to\_sint\_list** (bv、endianness、size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fto_005fsint_005flist)

bv の内容を表す、サイズが バイトの符号付き整数のリストを、エンディアンに従ってデコードして返します。

Scheme手順: **uint-list->bytevector** lstエンディアンネスサイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- uint_002dlist_002d_003ebytevector)

C 関数: **scm\_uint\_list\_to\_bytevector** (lst、endianness、size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fuint_005flist_005fto_005fbytevector)

lst にリストされている符号なし整数をエンディアンに従って size バイトにエンコードした新しいバイトベクトルを返します。

Scheme手順: **sint-list->bytevector** lst エンディアン サイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sint_002dlist_002d_003ebytevector)

C 関数: **scm\_sint\_list\_to\_bytevector** (lst、endianness、size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsint_005flist_005fto_005fbytevector)

lst にリストされている符号付き整数をエンディアンに従って size バイトにエンコードした新しいバイトベクトルを返します。

* * *

次へ: [バイトベクトルの内容をUnicode文字列として解釈する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Strings)、前: [バイトベクトルと整数リストの変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-and-Integer-Lists)、上: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 6.6.12.5 バイトベクトルの内容を浮動小数点数として解釈する [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Interpreting-Bytevector-Contents-as-Floating-Point-Numbers)

バイトベクターの内容は、ここで説明する手順を使用することで、IEEE-754規格の単精度または倍精度浮動小数点数（それぞれ32ビットおよび64ビット長）としてアクセスすることもできます。

スキーム手順: **bytevector-ieee-single-ref** インデックス エンディアンネスによる [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dieee_002dsingle_002dref)

スキーム手順: **bytevector-ieee-double-ref** bv インデックス エンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dieee_002ddouble_002dref)

C 関数: **scm\_bytevector\_ieee\_single\_ref** (bv、インデックス、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fieee_005fsingle_005fref)

C 関数: **scm\_bytevector\_ieee\_double\_ref** (bv、インデックス、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fieee_005fdouble_005fref)

bv から指定されたインデックスの IEEE-754 単精度浮動小数点数をエンディアンに従って返します。

スキーム手順: **bytevector-ieee-single-set!** bv インデックス値のエンディアンネス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dieee_002dsingle_002dset_0021)

スキーム手順: **bytevector-ieee-double-set!** bv インデックス値のエンディアン [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dieee_002ddouble_002dset_0021)

C 関数: **scm\_bytevector\_ieee\_single\_set\_x** (bv、インデックス、値、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fieee_005fsingle_005fset_005fx)

C 関数: **scm\_bytevector\_ieee\_double\_set\_x** (bv、インデックス、値、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fbytevector_005fieee_005fdouble_005fset_005fx)

実数値を、エンディアンに従ってインデックスbvに格納します。

専門的な処置も利用可能です。

スキーム手順: **bytevector-ieee-single-native-ref** bv インデックス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dieee_002dsingle_002dnative_002dref)

Scheme Procedure: **bytevector-ieee-double-native-ref** bv index [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dieee_002ddouble_002dnative_002dref)

C 関数: **scm\_bytevector\_ieee\_single\_native\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fieee_005fsingle_005fnative_005fref)

C 関数: **scm\_bytevector\_ieee\_double\_native\_ref** (bv, index) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fieee_005fdouble_005fnative_005fref)

ホストのネイティブエンディアンに従って、bv の指定されたインデックスから IEEE-754 単精度浮動小数点数を返します。

スキーム手順: **bytevector-ieee-single-native-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dieee_002dsingle_002dnative_002dset_0021)

スキーム手順: **bytevector-ieee-double-native-set!** bv インデックス値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dieee_002ddouble_002dnative_002dset_0021)

C 関数: **scm\_bytevector\_ieee\_single\_native\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fieee_005fsingle_005fnative_005fset_005fx)

C 関数: **scm\_bytevector\_ieee\_double\_native\_set\_x** (bv, index, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fieee_005fdouble_005fnative_005fset_005fx)

ホストのネイティブエンディアンに従って、実数値をbvの指定されたインデックスに格納します。

* * *

次へ: [配列 API を使用したバイトベクトルへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Arrays)、前: [バイトベクトルの内容を浮動小数点数として解釈する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Floats)、上: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.12.6 バイトベクターの内容を Unicode 文字列として解釈する [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Interpreting-Bytevector-Contents-as-Unicode-Strings)

バイトベクターの内容は、最も一般的に使用されているエンコード形式のいずれかでエンコードされた Unicode 文字列として解釈することもできます。より汎用的なインターフェースについては、[文字列をバイトとして表現する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Representing-Strings-as-Bytes) を参照してください。

([utf8->string](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utf8_002d_003estring) ([u8-list->bytevector](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u8_002dlist_002d_003ebytevector) '(99 97 102 101)))
⇒ 「カフェ」

([string->utf8](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003eutf8) "café") ;; 鋭アクセント付き小文字ラテン文字E
⇒ #vu8(99 97 102 195 169)

Scheme手順: **string-utf8-length** `str` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dutf8_002dlength)

C 関数: `SCM` **scm\_string\_utf8\_length** `(str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005futf8_005flength)

C 関数: `size_t` **scm\_c\_string\_utf8\_length** `(str)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fstring_005futf8_005flength)

str の UTF-8 表現におけるバイト数を返します。

Scheme手順: **string->utf8** str [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003eutf8)

Scheme手順: **string->utf16** str \[endianness\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003eutf16)

Scheme手順: **string->utf32** str \[endianness\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002d_003eutf32)

C 関数: **scm\_string\_to\_utf8** (str) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fto_005futf8)

C 関数: **scm\_string\_to\_utf16** (str, endianness) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fto_005futf16)

C 関数: **scm\_string\_to\_utf32** (str, endianness) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstring_005fto_005futf32)

str の UTF-8、UTF-16、または UTF-32 (別名 UCS-4) エンコーディングを含む、新しく割り当てられたバイトベクトルを返します。UTF-16 および UTF-32 の場合、エンディアンは `big` または `little` の記号で指定する必要があります。省略した場合、デフォルトでビッグエンディアンになります。

Scheme Procedure: **utf8->string** utf [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utf8_002d_003estring)

Scheme Procedure: **utf16->string** utf \[endianness\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utf16_002d_003estring)

Scheme Procedure: **utf32->string** utf \[endianness\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-utf32_002d_003estring)

C 関数: **scm\_utf8\_to\_string** (utf) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005futf8_005fto_005fstring)

C 関数: **scm\_utf16\_to\_string** (utf、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005futf16_005fto_005fstring)

C 関数: **scm\_utf32\_to\_string** (utf、エンディアン) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005futf32_005fto_005fstring)

バイトベクターutfのUTF-8、UTF-16、またはUTF-32デコードされた内容を含む、新たに割り当てられた文字列を返します。UTF-16およびUTF-32の場合、エンディアンは「big」または「little」の記号で指定する必要があります。省略した場合、デフォルトでビッグエンディアンが使用されます。

* * *

次へ: [SRFI-4 API を使用したバイトベクトルへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Uniform-Vectors)、前: [バイトベクトルの内容を Unicode 文字列として解釈する](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Strings)、上: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 6.6.12.7 配列 API を使用したバイトベクトルへのアクセス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Bytevectors-with-the-Array-API)

R6RSの拡張機能として、Guileでは_array_プロシージャを使用してバイトベクトルを操作できます（[Arrays](https://doc.guix.gnu.org/guile/latest/en/guile.html#Arrays)を参照）。これらのAPIを使用する場合、バイトは8ビット符号なし整数として1バイトずつアクセスされます。

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

次へ: [R7RS のバイトベクトル手続き](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Procedures-in-R7RS)、前: [配列API を使用したバイトベクトルへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Arrays)、上: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.12.8 SRFI-4 API を使用したバイトベクトルへのアクセス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Bytevectors-with-the-SRFI_002d4-API)

バイトベクトルは、SRFI-4 API を使用してアクセスすることもできます。詳細については、[SRFI-4 - バイトベクトルとの関係](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d4-and-Bytevectors) を参照してください。

* * *

次へ: [バイトベクトル スライス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Slices)、前: [SRFI-4 API を使用したバイトベクトルへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors-as-Uniform-Vectors)、上: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.12.9 R7RS の Bytevector プロシージャ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Procedures-in-R7RS-1)

[R7RS](https://doc.guix.gnu.org/guile/latest/en/guile.html#R7RS-Support) (セクション 6.9) では、バイトベクトル操作手順のセットが定義されており、

(use-modules (scheme base))

これらのうち、[`make-bytevector`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dmake_002dbyteve ctor)、[`bytevector?`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dbytevector_003f)、[`bytevector-length`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dbytevector_002dlength)、[`bytevector-u8-ref`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dbytevector_002du8_002dref)および[`bytevector-u8-set!`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dbytevector_002du8_002dset_0021) は R6RS と同じ定義です。以下にリストされているプロシージャは、R7RS と R6RS で定義が異なるか、R6RS では定義されていません。

スキームプロシージャ: **bytevector** arg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector-1)

指定された引数から構成される、新たに割り当てられたバイトベクトルを返します。`list` と同様です。

([bytevector](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector-1) 2 3 4) ⇒ #vu8(2 3 4)

[`u8-list->bytevector`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002du8_002dlist_002d_003ebytevector)も参照してください。

スキーム手順: **bytevector-copy** bv \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dcopy-1)

bv の要素を [start ... end] の範囲で含む、新しく割り当てられたバイトベクトルを返します。start のデフォルト値は 0、end のデフォルト値は bv の長さです。

(define bv #vu8(0 1 2 3 4 5))
([bytevector-copy](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dcopy) bv) ⇒ #vu8(0 1 2 3 4 5)
([bytevector-copy](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dcopy) bv 2) ⇒ #vu8(2 3 4 5)
([bytevector-copy](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dcopy) bv 2 4) ⇒ #vu8(2 3)

[R6RS バージョン](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dr6_003abytevector_002dcopy)も参照してください。

Scheme Procedure: **bytevector-copy!** dst at src \[start \[end\]\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dcopy_0021-1)

バイトベクターsrcから範囲[start ... end)の要素ブロックを、位置atからバイトベクターdstにコピーします。startのデフォルト値は0、endのデフォルト値はsrcの長さです。dstの長さがat + (end - start)より小さい場合はエラーとなります。

[R6RS バージョン](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dr6_003abytevector_002dcopy_0021)も参照してください。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) ((rnrs bytevectors) #:prefix r6:)
((スキームベース) #:プレフィックス r7:))

以下の呼び出しは同等です。

(r6:バイトベクトルコピー! ソース ソース開始 ターゲット ターゲット開始 長さ)
(r7:bytevector-copy! target target-start source source-start ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) source-start len))

スキームプロシージャ: **bytevector-append** arg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dappend)

指定されたバイトベクターargを連結した文字で構成される、新たに割り当てられたバイトベクターを返します。

([bytevector-append](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dappend) #vu8(0 1 2) #vu8(3 4 5))
⇒ #vu8(0 1 2 3 4 5)

* * *

前へ: [R7RS のバイトベクトル手順](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Procedures-in-R7RS)、上へ: [バイトベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.12.10 バイトベクトルスライス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevector-Slices-1)

R6RS仕様の拡張として、`(rnrs bytevectors gnu)`モジュールは、既存のバイトベクターの一部をエイリアスしたバイトベクターを返す`bytevector-slice`プロシージャを提供します。

スキーム手順: **bytevector-slice** bv オフセット \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dslice)

C 関数: **scm\_bytevector\_slice** (bv、オフセット、サイズ) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbytevector_005fslice)

オフセットから始まり、サイズバイト数だけカウントされたbvのスライスを返します。サイズを省略した場合、スライスはオフセットから始まるbv全体をカバーします。返されるスライスはbvとストレージを共有します。スライスへの変更はbvに反映され、その逆も同様です。

bvが実際にSRFI-4均一ベクトルである場合、オフセットとサイズがその要素タイプのサイズに揃っていない限り、その要素タイプは保持されます。

以下に、その使用方法を示す例を示します。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (rnrs バイトベクター)
(rnrs バイトベクトル gnu)

(define bv ([u8-list->bytevector](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-u8_002dlist_002d_003ebytevector) ([iota](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-iota) 10)))
(define slice ([bytevector-slice](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002dslice) bv 2 3))

スライス
⇒ #vu8(2 3 4)

([bytevector-u8-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bytevector_002du8_002dset_0021) スライス 0 77)
スライス
⇒ #vu8(77 3 4)

bv
⇒ #vu8(0 1 77 3 4 5 6 7 8 9)

* * *

次へ: [VLists](https://doc.guix.gnu.org/guile/latest/en/guile.html#VLists)、前: [Bytevectors](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bytevectors)、上: [Data Types](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[Contents](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[Index](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "Index")\]
