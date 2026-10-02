#### 7.5.5 SRFI-4 - 同質な数値ベクトルデータ型

SRFI-4は、要素がすべて単一の数値型である均一数値ベクトルへのインターフェースを提供します。Guileは、符号付きおよび符号なしの8ビット、16ビット、32ビット、64ビット整数、2種類のサイズの浮動小数点値、そしてSRFI-4の拡張として、これら2種類のサイズの複素浮動小数点数に対応する均一数値ベクトルを提供します。

標準のSRFI-4手順とデータタイプは、適切なモジュールをロードすることで組み込むことができます。

(use-modules (srfi srfi-4))

このモジュールは現在、Guileのデフォルト環境に含まれていますが、明示的にインポートすることをお勧めします。今後、SRFI-4モジュールをインポートせずにSRFI-4プロシージャを使用すると、非推奨メッセージが表示されるようになります。（もちろん、C言語の関数はいつでも呼び出すことができます。C言語にもモジュールがあればいいのですが！）

* [SRFI-4 - 概要](#7551-srfi-4---概要)
* [SRFI-4 - API](#7552-srfi-4---api)
* [SRFI-4 - バイトベクトルとの関係](#7553-srfi-4---バイトベクトルとの関係)
* [SRFI-4 - Guile 拡張機能](#7554-srfi-4---guile拡張機能)

* * *

次へ: [SRFI-4 - API](#7552-srfi-4---api)、上へ: [SRFI-4 - 同質な数値ベクトルデータ型](#755-srfi-4---同質な数値ベクトルデータ型) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.5.1 SRFI-4 - 概要

均一な数値ベクトルは、非均一な汎用ベクトルよりもメモリ消費量が少ないため、便利です。また、格納できる型がC言語の型に直接対応しているため、低レベルで効率的に扱うことができます。画像処理を例に考えてみましょう。画像にフィルタを適用したい場合、画像のピクセルを汎用ベクトルに格納して汎用畳み込み関数を作成することもできますが、均一ベクトルを使用するとはるかに効率的です。畳み込み関数は、すべてのピクセルが符号なし8ビット値（例えば）であることを認識しており、非常にタイトな内部ループを使用できます。

Schemeでは、コンパイラがSRFI-4アクセサへの呼び出しを認識し、適切なコンパイル済みコードにインライン化することで、この処理が実現されています。C言語からは生配列にアクセスできます。C言語から均一な数値ベクトルを効率的に扱うための関数は、このセクションの最後に記載されています。

一様数値ベクトルは、一次元一様数値配列の特殊なケースである。

均一な数値ベクトルには12種類の標準的な種類があり、それぞれに独自のコンストラクタ、アクセサなどが備わっています。特定の種類の均一な数値ベクトルを操作するプロシージャには、要素の型を示す「タグ」が名前に含まれています。

`u8`

符号なし8ビット整数

`s8`

符号付き8ビット整数

`u16`

符号なし16ビット整数

`s16`

符号付き16ビット整数

`u32`

符号なし32ビット整数

`s32`

符号付き32ビット整数

`u64`

符号なし64ビット整数

`s64`

符号付き64ビット整数

`f32`

C言語の型「float」

`f64`

C言語の型`double`

さらに、Guileは非標準タグを使用して複素数の均一配列をサポートしています。

`c32`

実部と虚部が浮動小数点数である直交座標形式の複素数

`c64`

実部と虚部が「倍精度浮動小数点数」である、直交座標形式の複素数

これらのベクトルの外部表現（つまり、読み取り構文）は通常の Scheme ベクトルと似ていますが、上記の表からベクトルの型を示す追加のタグが付いています。たとえば、

#u16(1 2 3)
#f64(3.1415 2.71)

ここで、浮動小数点数の読み取り構文が、偽を表す `#f` と競合することに注意してください。Standard Scheme では、3 つの要素を持つリスト `(1 #f 3)` に対して `(1 #f3)` と記述できますが、Guile では `(1 #f3)` は無効です。意図を明確にするためには、ほとんどの場合 `(1 #f 3)` と記述するのが適切であるため、これはめったに問題になりません。

* * *

次へ: [SRFI-4 - バイトベクトルとの関係](#7553-srfi-4---バイトベクトルとの関係)、前へ: [SRFI-4 - 概要](#7551-srfi-4---概要)、上へ: [SRFI-4 - 同種の数値ベクトルデータ型](#755-srfi-4---同質な数値ベクトルデータ型) \[[目次](00_contents.md "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 7.5.5.2 SRFI-4 - API

`c32` および `c64` 関数は `(srfi srfi-4 gnu)` からのみ利用可能であることに注意してください。

Scheme手順: **u8vector?** obj

Scheme手順: **s8vector?** obj

Scheme手順: **u16vector?** obj

スキームプロシージャ: **s16vector?** obj

Scheme手順: **u32vector?** obj

Scheme手順: **s32vector?** obj

Scheme手順: **u64vector?** obj

Scheme手順: **s64vector?** obj

Scheme手順: **f32vector?** obj

Scheme手順: **f64vector?** obj

Scheme手順: **c32vector?** obj

Scheme手順: **c64vector?** obj

C 関数: **scm\_u8vector\_p** (obj)

C 関数: **scm\_s8vector\_p** (obj)

C 関数: **scm\_u16vector\_p** (obj)

C 関数: **scm\_s16vector\_p** (obj)

C 関数: **scm\_u32vector\_p** (obj)

C 関数: **scm\_s32vector\_p** (obj)

C 関数: **scm\_u64vector\_p** (obj)

C 関数: **scm\_s64vector\_p** (obj)

C 関数: **scm\_f32vector\_p** (obj)

C 関数: **scm\_f64vector\_p** (obj)

C 関数: **scm\_c32vector\_p** (obj)

C 関数: **scm\_c64vector\_p** (obj)

objが指定された型の同種数値ベクトルである場合は、`#t`を返します。

Scheme手順: **make-u8vector** n \[value\]

Scheme手順: **make-s8vector** n \[value\]

Scheme手順: **make-u16vector** n \[value\]

スキーム手順: **make-s16vector** n \[value\]

Scheme手順: **make-u32vector** n \[value\]

Scheme手順: **make-s32vector** n \[value\]

Scheme手順: **make-u64vector** n \[value\]

Scheme手順: **make-s64vector** n \[value\]

Scheme手順: **make-f32vector** n \[value\]

Scheme手順: **make-f64vector** n \[value\]

Scheme手順: **make-c32vector** n \[value\]

Scheme手順: **make-c64vector** n \[value\]

C 関数: **scm\_make\_u8vector** (n, value)

C 関数: **scm\_make\_s8vector** (n, value)

C 関数: **scm\_make\_u16vector** (n, value)

C 関数: **scm\_make\_s16vector** (n, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fmake_005fs16vector)

C 関数: **scm\_make\_u32vector** (n, value)

C 関数: **scm\_make\_s32vector** (n, value)

C 関数: **scm\_make\_u64vector** (n, value)

C 関数: **scm\_make\_s64vector** (n, value)

C 関数: **scm\_make\_f32vector** (n, value)

C 関数: **scm\_make\_f64vector** (n, value)

C 関数: **scm\_make\_c32vector** (n, value)

C 関数: **scm\_make\_c64vector** (n, value)

指定された型のn個の要素を保持する、新たに割り当てられた同型数値ベクトルを返します。値が指定されている場合は、その値でベクトルが初期化されます。指定されていない場合は、内容は未指定となります。

Scheme Procedure: **u8vector** value …

Scheme Procedure: **s8vector** value …

Scheme Procedure: **u16vector** value …

Scheme Procedure: **s16vector** value …

Scheme手順: **u32vector**値…

Scheme手順: **s32vector**値…

Scheme Procedure: **u64vector** value …

Scheme Procedure: **s64vector** value …

Scheme手順: **f32vector**値…

Scheme Procedure: **f64vector** value …

Scheme手順: **c32vector**値…

Scheme手順: **c64vector**値…

C 関数: **scm\_u8vector** (値)

C 関数: **scm\_s8vector** (値)

C 関数: **scm\_u16vector** (値)

C 関数: **scm\_s16vector** (値)

C 関数: **scm\_u32vector** (値)

C 関数: **scm\_s32vector** (値)

C 関数: **scm\_u64vector** (値)

C 関数: **scm\_s64vector** (値)

C 関数: **scm\_f32vector** (値)

C 関数: **scm\_f64vector** (値)

C 関数: **scm\_c32vector** (値)

C 関数: **scm\_c64vector** (値)

指定されたパラメータ値を保持する、指定された型の同型数値ベクトルを新たに割り当てて返します。ベクトルの長さは、指定されたパラメータの数です。

Scheme手順: **u8vector-length** vec

スキーム手順: **s8vector-length** vec

Scheme手順: **u16vector-length** vec

Scheme手順: **s16vector-length** vec

Scheme手順: **u32vector-length** vec

Scheme手順: **s32vector-length** vec

Scheme手順: **u64vector-length** vec

Scheme手順: **s64vector-length** vec

Scheme手順: **f32vector-length** vec

Scheme手順: **f64vector-length** vec

Scheme手順: **c32vector-length** vec

Scheme手順: **c64vector-length** vec

C 関数: **scm\_u8vector\_length** (vec)

C 関数: **scm\_s8vector\_length** (vec)

C 関数: **scm\_u16vector\_length** (vec)

C 関数: **scm\_s16vector\_length** (vec)

C 関数: **scm\_u32vector\_length** (vec)

C 関数: **scm\_s32vector\_length** (vec)

C 関数: **scm\_u64vector\_length** (vec)

C 関数: **scm\_s64vector\_length** (vec)

C 関数: **scm\_f32vector\_length** (vec)

C 関数: **scm\_f64vector\_length** (vec)

C 関数: **scm\_c32vector\_length** (vec)

C 関数: **scm\_c64vector\_length** (vec)

vec内の要素数を返します。

Scheme Procedure: **u8vector-ref** vec i

Scheme Procedure: **s8vector-ref** vec i

Scheme Procedure: **u16vector-ref** vec i

Scheme Procedure: **s16vector-ref** vec i

Scheme手順: **u32vector-ref** vec i

Scheme手順: **s32vector-ref** vec i

Scheme Procedure: **u64vector-ref** vec i

Scheme Procedure: **s64vector-ref** vec i

Scheme 手順: **f32vector-ref** vec i

Scheme Procedure: **f64vector-ref** vec i

Scheme手順: **c32vector-ref** vec i

Scheme手順: **c64vector-ref** vec i

C 関数: **scm\_u8vector\_ref** (vec, i)

C 関数: **scm\_s8vector\_ref** (vec, i)

C 関数: **scm\_u16vector\_ref** (vec, i)

C 関数: **scm\_s16vector\_ref** (vec, i)

C 関数: **scm\_u32vector\_ref** (vec, i)

C 関数: **scm\_s32vector\_ref** (vec, i)

C 関数: **scm\_u64vector\_ref** (vec, i)

C 関数: **scm\_s64vector\_ref** (vec, i)

C 関数: **scm\_f32vector\_ref** (vec, i)

C 関数: **scm\_f64vector\_ref** (vec, i)

C 関数: **scm\_c32vector\_ref** (vec, i)

C 関数: **scm\_c64vector\_ref** (vec, i)

vec のインデックス i にある要素を返します。vec の最初の要素のインデックスは 0 です。

Scheme Procedure: **u8vector-set!** vec i value

Scheme 手順: **s8vector-set!** vec i value

Scheme Procedure: **u16vector-set!** vec i value

Scheme Procedure: **s16vector-set!** vec i value

Scheme 手順: **u32vector-set!** vec i value

Scheme 手順: **s32vector-set!** vec i value

Scheme Procedure: **u64vector-set!** vec i value

Scheme 手順: **s64vector-set!** vec i value

Scheme Procedure: **f32vector-set!** vec i value

Scheme 手順: **f64vector-set!** vec i value

Scheme 手順: **c32vector-set!** vec i value

Scheme 手順: **c64vector-set!** vec i value

C 関数: **scm\_u8vector\_set\_x** (vec, i, value)

C 関数: **scm\_s8vector\_set\_x** (vec, i, value)

C 関数: **scm\_u16vector\_set\_x** (vec, i, value)

C 関数: **scm\_s16vector\_set\_x** (vec, i, value)

C 関数: **scm\_u32vector\_set\_x** (vec, i, value)

C 関数: **scm\_s32vector\_set\_x** (vec, i, value)

C 関数: **scm\_u64vector\_set\_x** (vec, i, value)

C 関数: **scm\_s64vector\_set\_x** (vec, i, value)

C 関数: **scm\_f32vector\_set\_x** (vec, i, value)

C 関数: **scm\_f64vector\_set\_x** (vec, i, value)

C 関数: **scm\_c32vector\_set\_x** (vec, i, value)

C 関数: **scm\_c64vector\_set\_x** (vec, i, value)

vec内のインデックスiにある要素に値を設定します。vecの最初の要素のインデックスは0です。戻り値は未指定です。

Scheme手順: **u8vector->list** vec

Scheme手順: **s8vector->list** vec

Scheme手順: **u16vector->list** vec

Scheme手順: **s16vector->list** vec

Scheme手順: **u32vector->list** vec

Scheme手順: **s32vector->list** vec

Scheme手順: **u64vector->list** vec

Scheme手順: **s64vector->list** vec

Scheme手順: **f32vector->list** vec

Scheme手順: **f64vector->list** vec

Scheme手順: **c32vector->list** vec

Scheme手順: **c64vector->list** vec

C 関数: **scm\_u8vector\_to\_list** (vec)

C 関数: **scm\_s8vector\_to\_list** (vec)

C 関数: **scm\_u16vector\_to\_list** (vec)

C 関数: **scm\_s16vector\_to\_list** (vec)

C 関数: **scm\_u32vector\_to\_list** (vec)

C 関数: **scm\_s32vector\_to\_list** (vec)

C 関数: **scm\_u64vector\_to\_list** (vec) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fu64vector_005fto_005flist)

C 関数: **scm\_s64vector\_to\_list** (vec)

C 関数: **scm\_f32vector\_to\_list** (vec)

C 関数: **scm\_f64vector\_to\_list** (vec)

C 関数: **scm\_c32vector\_to\_list** (vec)

C 関数: **scm\_c64vector\_to\_list** (vec)

vec のすべての要素を含む、新しく割り当てられたリストを返します。

Scheme手順: **list->u8vector** lst

Scheme手順: **list->s8vector** lst

Scheme Procedure: **list->u16vector** lst

Scheme手順: **list->s16vector** lst

Scheme手順: **list->u32vector** lst

Scheme手順: **list->s32vector** lst

Scheme手順: **list->u64vector** lst

Scheme手順: **list->s64vector** lst

Scheme手順: **list->f32vector** lst

Scheme手順: **list->f64vector** lst

Scheme手順: **list->c32vector** lst

Scheme手順: **list->c64vector** lst

C 関数: **scm\_list\_to\_u8vector** (lst)

C 関数: **scm\_list\_to\_s8vector** (lst)

C 関数: **scm\_list\_to\_u16vector** (lst)

C 関数: **scm\_list\_to\_s16vector** (lst)

C 関数: **scm\_list\_to\_u32vector** (lst)

C 関数: **scm\_list\_to\_s32vector** (lst)

C 関数: **scm\_list\_to\_u64vector** (lst)

C 関数: **scm\_list\_to\_s64vector** (lst)

C 関数: **scm\_list\_to\_f32vector** (lst)

C 関数: **scm\_list\_to\_f64vector** (lst)

C 関数: **scm\_list\_to\_c32vector** (lst)

C 関数: **scm\_list\_to\_c64vector** (lst)

指定された型の、リスト lst の要素で初期化された、新たに割り当てられた同種の数値ベクトルを返します。

C 関数: `SCM` **scm\_take\_u8vector** `(const scm_t_uint8 *data, size_t len)`

C 関数: `SCM` **scm\_take\_s8vector** `(const scm_t_int8 *data, size_t len)`

C 関数: `SCM` **scm\_take\_u16vector** `(const scm_t_uint16 *data, size_t len)`

C 関数: `SCM` **scm\_take\_s16vector** `(const scm_t_int16 *data, size_t len)`

C 関数: `SCM` **scm\_take\_u32vector** `(const scm_t_uint32 *data, size_t len)`

C 関数: `SCM` **scm\_take\_s32vector** `(const scm_t_int32 *data, size_t len)`

C 関数: `SCM` **scm\_take\_u64vector** `(const scm_t_uint64 *data, size_t len)`

C 関数: `SCM` **scm\_take\_s64vector** `(const scm_t_int64 *data, size_t len)`

C 関数: `SCM` **scm\_take\_f32vector** `(const float *data, size_t len)`

C 関数: `SCM` **scm\_take\_f64vector** `(const double *data, size_t len)`

C 関数: `SCM` **scm\_take\_c32vector** `(const float *data, size_t len)`

C 関数: `SCM` **scm\_take\_c64vector** `(const double *data, size_t len)`

指定された型と長さの新しい一様数値ベクトルを返します。このベクトルは、dataが指すメモリ領域を使用して要素を格納します。このメモリ領域は最終的に`free`によって解放されます。引数lenはdataの要素数を指定するものであり、バイト単位のサイズを指定するものではありません。

`c32` および `c64` バリアントは、`float` または `double` 型の C 配列へのポインタを受け取ります。複素数の実部は配列の偶数インデックスに、対応する虚部は次の奇数インデックスに配置されます。

C 関数: `const scm_t_uint8 *` **scm\_u8vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const scm_t_int8 *` **scm\_s8vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const scm_t_uint16 *` **scm\_u16vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fu16vector_005felements)

C 関数: `const scm_t_int16 *` **scm\_s16vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const scm_t_uint32 *` **scm\_u32vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const scm_t_int32 *` **scm\_s32vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const scm_t_uint64 *` **scm\_u64vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const scm_t_int64 *` **scm\_s64vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const float *` **scm\_f32vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const double *` **scm\_f64vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const float *` **scm\_c32vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `const double *` **scm\_c64vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

`scm_vector_elements`（[C言語からのベクトルアクセス](06_06_10_vectors.md#66104-c言語からのベクトルアクセス)を参照）と同様ですが、指定された種類の均一な数値ベクトルの要素へのポインタを返します。

C 関数: `scm_t_uint8 *` **scm\_u8vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `scm_t_int8 *` **scm\_s8vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `scm_t_uint16 *` **scm\_u16vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `scm_t_int16 *` **scm\_s16vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `scm_t_uint32 *` **scm\_u32vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `scm_t_int32 *` **scm\_s32vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `scm_t_uint64 *` **scm\_u64vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `scm_t_int64 *` **scm\_s64vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `float *` **scm\_f32vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `double *` **scm\_f64vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `float *` **scm\_c32vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

C 関数: `double *` **scm\_c64vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)`

`scm_vector_writable_elements`（[C言語からのベクトルアクセス](06_06_10_vectors.md#66104-c言語からのベクトルアクセス)を参照）と同様ですが、指定された種類の均一な数値ベクトルの要素へのポインタを返します。

* * *

次へ: [SRFI-4 - Guile 拡張機能](#7554-srfi-4---guile拡張機能)、前へ: [SRFI-4 - API](#7552-srfi-4---api)、上へ: [SRFI-4 - 同質な数値ベクトルデータ型](#755-srfi-4---同質な数値ベクトルデータ型) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.5.3 SRFI-4 - バイトベクトルとの関係

Guile は、バイトベクトルを使用して SRFI-4 ベクトルを実装しています ([バイトベクトル](06_06_12_bytevectors.md#6612-バイトベクトル) を参照)。数値ベクトルを扱う場合、多くの場合、そのバイトをどこかに書き込んだり、基となるバイトにアクセスしたり、他の場所からバイトを読み込んだりする必要が生じます。バイトベクトルは、このような処理に非常に適しています。しかし、SRFI-4 API は要素単位でアドレス指定されるため、バイト単位ではなく要素単位でアドレス指定できるので、数値計算を行う際にはより使いやすいです。

そこで妥協案として、Guileではすべてのバイトベクトル関数が数値ベクトルに対して動作するようにしています。これらの関数は、当然のことながら、基となるバイトをネイティブのエンディアンで参照します。

同じ考え方、つまり中身は単なるバイト列であるという考え方に基づき、Guileでは特定の型のユニフォームベクトルを、あたかも任意の型であるかのようにアクセスできます。`u32vector`に値を格納し、`u8vector-ref`でその要素にアクセスできます。バイトベクトルに対して`f64vector-ref`を使用することもできます。Guileにとってはすべて同じです。

このようにして、バイトベクトルを操作する手順を用いて、均一な数値ベクトルを入出力ポートに書き込んだり、入出力ポートから読み取ったりすることができる。

詳細については、[バイトベクトル](06_06_12_bytevectors.md#6612-バイトベクトル)を参照してください。

* * *

前へ: [SRFI-4 - バイトベクトルとの関係](#7553-srfi-4---バイトベクトルとの関係)、上へ: [SRFI-4 - 同種の数値ベクトルデータ型](#755-srfi-4---同質な数値ベクトルデータ型) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.5.4 SRFI-4 - Guile拡張機能

Guileは、デフォルトのGuile環境では利用できないSRFI-4の便利な拡張機能をいくつか定義しています。これらの拡張機能は、extensionsモジュールをロードすることでインポートできます。

(use-modules (srfi srfi-4 gnu))

スキーム手順: **srfi-4-vector-type-size** obj

SRFI-4 ベクトル obj の各要素のサイズをバイト単位で返します。たとえば、`(srfi-4-vector-type-size #u32())` は `4` を返します。

Scheme手順: **any->u8vector** obj

Scheme手順: **any->s8vector** obj

Scheme Procedure: **any->u16vector** obj

Scheme手順: **any->s16vector** obj

Scheme手順: **any->u32vector** obj

Scheme手順: **any->s32vector** obj

Scheme手順: **any->u64vector** obj

Scheme手順: **any->s64vector** obj

Scheme手順: **any->f32vector** obj

Scheme手順: **any->f64vector** obj

Scheme手順: **any->c32vector** obj

Scheme手順: **any->c64vector** obj

C 関数: **scm\_any\_to\_u8vector** (obj)

C 関数: **scm\_any\_to\_s8vector** (obj)

C 関数: **scm\_any\_to\_u16vector** (obj)

C 関数: **scm\_any\_to\_s16vector** (obj)

C 関数: **scm\_any\_to\_u32vector** (obj)

C 関数: **scm\_any\_to\_s32vector** (obj)

C 関数: **scm\_any\_to\_u64vector** (obj)

C 関数: **scm\_any\_to\_s64vector** (obj)

C 関数: **scm\_any\_to\_f32vector** (obj)

C 関数: **scm\_any\_to\_f64vector** (obj)

C 関数: **scm\_any\_to\_c32vector** (obj)

C 関数: **scm\_any\_to\_c64vector** (obj)

指定された型の（場合によっては新規に割り当てられた）均一数値ベクトルを返します。このベクトルは、リスト、ベクトル、または均一ベクトルであるオブジェクト obj の要素で初期化されます。obj が既に適切な均一数値ベクトルである場合は、変更されずに返されます。

Scheme Procedure: **u8vector-copy!** dst at src \[start \[end\]\]

Scheme 手順: **s8vector-copy!** dst at src \[start \[end\]\]

Scheme Procedure: **u16vector-copy!** dst at src \[start \[end\]\]

Scheme Procedure: **s16vector-copy!** dst at src \[start \[end\]\]

Scheme手順: **u32vector-copy!** dst at src \[start \[end\]\]

Scheme 手順: **s32vector-copy!** dst at src \[start \[end\]\]

Scheme Procedure: **u64vector-copy!** dst at src \[start \[end\]\]

Scheme Procedure: **s64vector-copy!** dst at src \[start \[end\]\]

Scheme Procedure: **f32vector-copy!** dst at src \[start \[end\]\]

Scheme Procedure: **f64vector-copy!** dst at src \[start \[end\]\]

Scheme Procedure: **c32vector-copy!** dst at src \[start \[end\]\]

Scheme Procedure: **c64vector-copy!** dst at src \[start \[end\]\]

指定された型のベクトルである要素のブロックをsrcからdstにコピーします。dstではatから、srcではstartからendから開始します。dstの長さがat + (end - start)より短い場合はエラーとなります。atとstartのデフォルト値は0、endのデフォルト値はsrcの長さです。

ソースと宛先が重複している場合、コピーは、ソースがまず一時的なベクターにコピーされ、次に宛先にコピーされるかのように行われます。

[`vector-copy!`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dvector_002dcopy_0021)も参照してください。

Scheme手順: **u8vector-copy** src \[start \[end\]\]

Scheme手順: **s8vector-copy** src \[start \[end\]\]

Scheme手順: **u16vector-copy** src \[start \[end\]\]

Scheme手順: **s16vector-copy** src \[start \[end\]\]

Scheme手順: **u32vector-copy** src \[start \[end\]\]

Scheme手順: **s32vector-copy** src \[start \[end\]\]

Scheme手順: **u64vector-copy** src \[start \[end\]\]

Scheme手順: **s64vector-copy** src \[start \[end\]\]

Scheme手順: **f32vector-copy** src \[start \[end\]\]

Scheme手順: **f64vector-copy** src \[start \[end\]\]

Scheme手順: **c32vector-copy** src \[start \[end\]\]

Scheme手順: **c64vector-copy** src \[start \[end\]\]

指定された型の、新しく割り当てられたベクトルを返します。このベクトルはsrcと同じ型である必要があり、srcのstartとendの間の要素を格納します。startのデフォルト値は0、endのデフォルト値はsrcの長さです。

[`vector-copy`](https://doc.guix.gnu.org/guile/latest/en/guile.html#x_002dvector_002dcopy)も参照してください。

* * *

次へ: [SRFI-8 - 受信](07_05_07_srfi8_receive.md#757-srfi-8---受信)、前: [SRFI-4 - 同種数値ベクトルデータ型](#755-srfi-4---同質な数値ベクトルデータ型)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
