#### 6.6.10 ベクトル

ベクトルは、Schemeオブジェクトのシーケンスです。リストとは異なり、ベクトルは一度作成されると長さを変更できません。リストに対するベクトルの利点は、ベクトルの要素の位置（インデックスと同義）がゼロから始まる数値である場合、その要素にアクセスするのに必要な時間が一定であるのに対し、リストではアクセス時間がリスト内のアクセスする要素の位置に比例することです。

ベクトルにはあらゆる種類の Scheme オブジェクトを含めることができます。同じベクトル内に異なる型のオブジェクトを含めることも可能です。ベクトルの中にベクトルを含める場合は、[配列](06_06_13_arrays.md#6613-配列) を使用することをお勧めします。また、ベクトルは一次元非均一配列の特殊なケースであり、配列の手続きはベクトルに対しても問題なく動作することに注意してください。

より包括的なベクターライブラリについては、[SRFI-43 - ベクトルライブラリ](07_05_30_srfi43_vector_library.md#7530-srfi-43---ベクトルライブラリ)、[R6RS サポート](07_06_r6rs_support.md#76-r6rs-サポート)、または [R7RS サポート](07_07_r7rs_support.md#77-r7rs-サポート) も参照してください。

* [ベクトルの構文を読む](#66101-ベクトルの構文の読み取り)
* [動的ベクトルの作成と検証](#66102-動的ベクトルの作成と検証)
* [ベクターコンテンツへのアクセスと変更](#66103-ベクトルコンテンツへのアクセスと変更)
* [C言語からのベクトルアクセス](#66104-c言語からのベクトルアクセス)
* [一様数値ベクトル](#66105-一様数値ベクトル)

* * *

次へ: [動的ベクトルの作成と検証](#66102-動的ベクトルの作成と検証)、上: [ベクトル](#6610-ベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.10.1 ベクトルの構文の読み取り

ベクトルは、文字列や文字、その他のデータ型と同様に、ソースコードに直接入力できます。ベクトルの読み取り構文は次のとおりです。シャープ記号（#）、開き括弧、各要素の読み取り構文、そして閉じ括弧の順です。文字列と同様に、ベクトルは引用符で囲む必要はありません。

以下はベクトルの読み取り構文の例です。最初のベクトルには数値のみが含まれており、2番目のベクトルには文字列、シンボル、16進数表記の数値という3種類のオブジェクトが含まれています。

#(1 2 3)
#("こんにちは" foo #xdeadbeef)

* * *

次へ: [ベクトルの内容へのアクセスと変更](#66103-ベクトルコンテンツへのアクセスと変更)、前: [ベクトルの構文を読む](#66101-ベクトルの構文の読み取り)、上: [ベクトル](#6610-ベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.10.2 動的ベクトルの作成と検証

先ほど説明した読み取り構文を使用して暗黙的にベクトルを作成する代わりに、ベクトルに格納したいScheme値のリストを引数として`vector`または`list->vector`プリミティブを呼び出すことで、ベクトルを動的に作成できます。このようにして作成されるベクトルのサイズは、指定された引数の数によって暗黙的に決定されます。

Scheme手順: **vector** arg …

Scheme手順: **list->vector** l

C 関数: **scm\_vector** (l)

指定された引数から構成される、新たに割り当てられたベクトルを返します。`list` と同様です。

([vector](#66102-動的ベクトルの作成と検証) 'a 'b 'c) ⇒ #(abc)

逆の操作は `vector->list` です。

Scheme手順: **vector->list** v

C 関数: **scm\_vector\_to\_list** (v)

v の要素で構成される、新たに割り当てられたリストを返します。

([vector->list](#66102-動的ベクトルの作成と検証) #(ダダダダダ)) ⇒ (ダダダダダ)
([list->vector](#66102-動的ベクトルの作成と検証) '(dididit dah)) ⇒ #(dididit dah)

明示的にサイズを指定してベクトルを割り当てるには、`make-vector`を使用します。このプリミティブでは、ベクトルの要素の初期値（すべての要素に同じ値）を指定することもできます。

Scheme手順: **make-vector** len \[fill\]

C 関数: **scm\_make\_vector** (len, fill)

len個の要素からなる、新たに割り当てられたベクトルを返します。2番目の引数が指定された場合、各位置は初期値で埋められます。指定がない場合は、各位置の初期値は未指定です。

C 関数: `SCM` **scm\_c\_make\_vector** `(size_t k, SCM fill)`

`scm_make_vector`と同様ですが、長さは`size_t`で指定されます。

任意のScheme値がベクトルであるかどうかを確認するには、`vector?`プリミティブを使用します。

Scheme手順: **vector?** obj

C 関数: **scm\_vector\_p** (obj)

objがベクトルの場合は`#t`を返し、そうでない場合は`#f`を返します。

C 関数: `int` **scm\_is\_vector** `(SCM obj)`

objがベクトルの場合はゼロ以外の値を返し、それ以外の場合はゼロを返します。

* * *

次へ: [C からのベクトルアクセス](#66104-c言語からのベクトルアクセス)、前: [動的ベクトルの作成と検証](#66102-動的ベクトルの作成と検証)、上: [ベクトル](#6610-ベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.10.3 ベクトルコンテンツへのアクセスと変更

`vector-length`と`vector-ref`は、それぞれ指定されたベクトルのサイズと、そのベクトルに含まれる要素に関する情報を返します。

Scheme手順: **vector-length** ベクトル

C 関数: **scm\_vector\_length** (vector)

ベクトル内の要素数を正確な整数で返します。

C 関数: `size_t` **scm\_c\_vector\_length** `(SCM vec)`

vec内の要素数を`size_t`型で返します。

Scheme Procedure: **vector-ref** vec k

C 関数: **scm\_vector\_ref** (vec, k)

vec の k 番目の位置の内容を返します。k は vec の有効なインデックスである必要があります。

([vector-ref](#66103-ベクトルコンテンツへのアクセスと変更) #(1 1 2 3 5 8 13 21) 5) ⇒ 8
([vector-ref](#66103-ベクトルコンテンツへのアクセスと変更) #(1 1 2 3 5 8 13 21)
(let ((i ([round](06_06_02_numerical_data_types.md#66211-算術関数) ([\*](06_06_02_numerical_data_types.md#66211-算術関数) 2 ([acos](06_06_02_numerical_data_types.md#66212-科学関数) \-1)))))
(if ([inexact?](06_06_02_numerical_data_types.md#6625-正確な数と不正確な数) i)
([inexact->exact](06_06_02_numerical_data_types.md#6625-正確な数と不正確な数) i)
i))) ⇒ 13

C 関数: `SCM` **scm\_c\_vector\_ref** `(SCM vec, size_t k)`

vec の位置 k (サイズ t) の内容を返します。

動的ベクトルコンストラクタ手順のいずれかによって作成されたベクトル（[動的ベクトルの作成と検証](#66102-動的ベクトルの作成と検証)を参照）は、以下の手順を使用して変更できます。

_注記:_ R5RSによると、これらの手順をリテラルに読み込んだベクトルに適用することはエラーです。なぜなら、そのようなベクトルは定数として扱われるべきだからです。しかしながら、現在のところ、Guileはこのエラーを検出していません。

Scheme 手順: **vector-set!** vec k obj

C 関数: **scm\_vector\_set\_x** (vec, k, obj)

objをvecのk番目の位置に格納します。kはvecの有効なインデックスである必要があります。「vector-set!」によって返される値は未定義です。

(let ((vec ([vector](#66102-動的ベクトルの作成と検証) 0 '(2 2 2 2) "Anna")))
([vector-set!](#66103-ベクトルコンテンツへのアクセスと変更) vec 1 '("Sue" "Sue"))
vec) ⇒ #(0 ("スー" "スー") "アンナ")

C 関数: `void` **scm\_c\_vector\_set\_x** `(SCM vec, size_t k, SCM obj)`

obj を vec の位置 k (サイズ t) に格納します。

Scheme 手順: **vector-fill!** vec fill \[start \[end\]\]

C 関数: **scm\_vector\_fill\_x** (vec, fill)

vec の各位置を [start ... end] の範囲内に格納します。start のデフォルト値は 0、end のデフォルト値は vec の長さです。

`vector-fill!` が返す値は未定義です。

Scheme手順: **vector-copy** vec \[start \[end\]\]

C 関数: **scm\_vector\_copy** (vec)

vec の要素を範囲 [start ... end] 内に含む、新しく割り当てられたベクトルを返します。start のデフォルト値は 0、end のデフォルト値は vec の長さです。

Scheme Procedure: **vector-copy!** dst at src \[start \[end\]\]

ベクトル src から範囲 [start ... end) の要素ブロックを、位置 at から開始してベクトル dst にコピーします。at と start はデフォルトで 0 に設定され、end はデフォルトで src の長さになります。

dst の長さが at + (end - start) より短い場合はエラーです。

要素がコピーされる順序は規定されていませんが、ソースと宛先が重複する場合は、ソースが最初に一時的なベクトルにコピーされ、次に宛先にコピーされるかのようにコピーが行われます。

`vector-copy!` が返す値は未定義です。

Scheme Procedure: **vector-move-left!** vec1 start1 end1 vec2 start2

C 関数: **scm\_vector\_move\_left\_x** (vec1, start1, end1, vec2, start2)

vec1のstart1からend1までの位置にある要素を、vec2のstart2の位置からコピーします。start1とstart2は包含インデックスであり、end1は包含インデックスではありません。

`vector-move-left!` は、要素を左端の順にコピーします。したがって、vec1 と vec2 が同じベクトルを参照している場合、`vector-move-left!` は通常、start1 が start2 より大きい場合に適切です。

`vector-move-left!` が返す値は未定義です。

Scheme Procedure: **vector-move-right!** vec1 start1 end1 vec2 start2

C 関数: **scm\_vector\_move\_right\_x** (vec1, start1, end1, vec2, start2)

vec1のstart1からend1までの位置にある要素を、vec2のstart2の位置からコピーします。start1とstart2は包含インデックスであり、end1は包含インデックスではありません。

`vector-move-right!` は要素を右端の順にコピーします。したがって、vec1 と vec2 が同じベクトルを参照している場合、`vector-move-right!` は通常、start1 が start2 より小さい場合に適切です。

`vector-move-right!` が返す値は未定義です。

* * *

次へ: [Uniform Numeric Vectors](#66105-一様数値ベクトル)、前: [Accessing and Modifying Vector Contents](#66103-ベクトルコンテンツへのアクセスと変更)、上: [Vectors](#6610-ベクトル) \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md "Index")\]

#### 6.6.10.4 C言語からのベクトルアクセス

ベクトルは、関数 [`scm_c_vector_ref`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dscm_005fc_005fvector_005fref) および [`scm_c_vector_set_x`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dscm_005fc_005fvector_005fset_005fx) を使用して C から読み取ったり変更したりできます。これらの関数に加えて、特定の状況ではより効率的な C からベクトルにアクセスする方法が 2 つあります。安全でない _vector マクロ_ を使用できます。または、あらゆる種類の配列にアクセスするための汎用フレームワークを使用することもできます（[C からの配列へのアクセス](06_06_13_arrays.md#66135-c言語から配列にアクセスする) を参照）。これは冗長ですが、あらゆる種類のベクトル（および配列）を効率的に処理できます。バッキング ストアがベクトルであるランク 1 の配列の場合は、ショートカットとして `scm_vector_elements` 関数と `scm_vector_writable_elements` 関数を使用できます。

C マクロ: `size_t` **SCM\_SIMPLE\_VECTOR\_LENGTH** `(SCM vec)`

ベクトルvecの長さに評価されます。型チェックは行われません。

C マクロ: `SCM` **SCM\_SIMPLE\_VECTOR\_REF** `(SCM vec, size_t idx)`

ベクトルvec内の位置idxにある要素を評価します。型や範囲のチェックは行われません。

C マクロ: `void` **SCM\_SIMPLE\_VECTOR\_SET** `(SCM vec, size_t idx, SCM val)`

ベクトルvec内の位置idxにある要素をvalに設定します。型や範囲のチェックは行われません。

C 関数: `const SCM *` **scm\_vector\_elements** `(SCM 配列、scm_t_array_handle *handle、size_t *lenp、ssize_t *incp)`

配列の [ハンドル](06_06_13_arrays.md#66135-c言語から配列にアクセスする) を取得し、その要素への読み取り専用ポインタを返します。配列は、ベクトルであるか、またはバッキングストアがベクトルであるランク 1 の配列である必要があります。そうでない場合は、エラーが通知されます。ハンドルは最終的に [`scm_array_handle_release`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dscm_005farray_005fhandle_005frelease) を使用して解放する必要があります。

lenp と incp が指す変数には、それぞれ配列の要素数と連続する要素間の増分 (要素数) が格納されます。配列の連続する要素は、ここで返される基となる「ルートベクトル」内で連続している必要はありません。したがって、増分は必ずしも 1 に等しくなく、負の値になる場合もあります ([共有配列](06_06_13_arrays.md#66133-共有配列) を参照)。

以下の例は、この関数の典型的な使用方法を示しています。配列のすべての要素を（逆順で）リストとして作成します。

scm_t_array_handle ハンドル;
size_t i, len;
ssize_t 増加;
const SCM \*elt;
SCMリスト;

elt = scm\_vector\_elements (array, &handle, &len, &inc);
リスト = SCM\_EOL;
for (i = 0; i < len; i++, elt += inc)
list = scm\_cons (\*elt, list);
scm_array_handle_release (&handle);

C 関数: `SCM *` **scm\_vector\_writable\_elements** `(SCM 配列、scm_t_array_handle *handle、size_t *lenp、ssize_t *incp)`

`scm_vector_elements`と同様ですが、ポインタを使用して配列を変更できます。

次の例は、この関数の典型的な使用方法を示しています。配列に`#t`を格納します。

scm_t_array_handle ハンドル;
size_t i, len;
ssize_t 増加;
SCM \*elt;

elt = scm\_vector\_writable\_elements (array, &handle, &len, &inc);
for (i = 0; i < len; i++, elt += inc)
*elt = SCM_BOOL_T;
scm_array_handle_release (&handle);

* * *

前へ: [C言語からのベクトルアクセス](#66104-c言語からのベクトルアクセス)、上へ: [ベクトル](#6610-ベクトル) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 6.6.10.5 一様数値ベクトル

均一数値ベクトルとは、要素がすべて単一の数値型であるベクトルです。Guile では、符号付きおよび符号なしの 8 ビット、16 ビット、32 ビット、64 ビット整数、2 種類の浮動小数点値、およびこれらの 2 種類の複素浮動小数点数に対して均一数値ベクトルを提供しています。詳細については、[SRFI-4 - 均一数値ベクトルデータ型](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#755-srfi-4---同質な数値ベクトルデータ型) を参照してください。

多くの場合、バイトベクトルはユニフォームベクトルと同等の機能を発揮し、バイナリ入出力との統合性に優れているという利点があります。バイトベクトルの詳細については、[バイトベクトル](06_06_12_bytevectors.md#6612-バイトベクトル)を参照してください。

* * *

次へ: [バイトベクトル](06_06_12_bytevectors.md#6612-バイトベクトル)、前: [ベクトル](#6610-ベクトル)、上: [データ型](06_06_00_data_types.md#66-データ型) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
