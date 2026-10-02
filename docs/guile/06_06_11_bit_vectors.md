#### 6.6.11 ビットベクトル

ビットベクトルは、ゼロ原点のブール値の1次元配列です。`#*` を接頭辞として `0` と `1` のシーケンスとして表示されます。例:

(make-bitvector 8 #f) ⇒
#\*00000000

ビットベクトルは1次元ビット配列の特殊なケースであり、配列プロシージャで使用できます。[配列](06_06_13_arrays.md#6613-配列)を参照してください。

Scheme手順: **bitvector?** obj

objがビットベクトルの場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme手順: **make-bitvector** len \[fill\]

長さlenの新しいビットベクトルを作成し、必要に応じてすべての要素を初期化して埋めます。

スキーム手順: **bitvector** bit …

引数を要素とする新しいビットベクトルを作成します。

Scheme Procedure: **bitvector-length** vec

ビットベクトルvecの長さを返します。

スキーム手順: **bitvector-bit-set?** vec idx

スキーム手順: **bitvector-bit-clear?** vec idx

ビットベクターvecのインデックスidxのビットがセットされている場合（`bitvector-bit-set?`の場合）、またはクリアされている場合（`bitvector-bit-clear?`の場合）は、`#t`を返します。

Scheme Procedure: **bitvector-set-bit!** vec idx

Scheme Procedure: **bitvector-clear-bit!** vec idx

ビットベクターvecのインデックスidxにあるビットを設定します（`bitvector-set-bit!`の場合）またはクリアします（`bitvector-clear-bit!`の場合）。

Scheme Procedure: **bitvector-set-all-bits!** vec

Scheme Procedure: **bitvector-clear-all-bits!** vec

スキーム手順: **bitvector-flip-all-bits!** vec

vec のすべてのビットを設定、クリア、または反転します。

Scheme手順: **list->bitvector** list

C 関数: **scm\_list\_to\_bitvector** (リスト)

リストの要素で初期化された新しいビットベクトルを返します。

Scheme手順: **bitvector->list** vec

C 関数: **scm\_bitvector\_to\_list** (vec)

ビットベクトルvecの要素で初期化された新しいリストを返します。

Scheme手順: **bitvector-copy** bitvector \[start \[end\]\]

C 関数: **scm\_bitvector\_copy** (ビットベクトル、開始位置、終了位置)

指定された範囲 [start ... end] 内の bitvector の要素を含む、新しく割り当てられた bitvector を返します。start のデフォルト値は 0、end のデフォルト値は bitvector の長さです。

Scheme Procedure: **bitvector-count** bitvector

ビットベクター内のエントリのうち、いくつが設定されているかをカウントして返します。

(ビットベクトルカウント #\*000111000) ⇒ 3

スキーム手順: **bitvector-count-bits** ビットベクトルビット

ビットベクター内のエントリのうち、設定されているエントリの数を返します。ビットベクターのビット数は、考慮するエントリを選択します。ビットベクターの長さは、ビット数以上である必要があります。

例えば、

(ビットベクトル-カウント-ビット #\*01110111 #\*11001101) ⇒ 3

Scheme手順: **bitvector-position** bitvector bool start

C 関数: **scm\_bitvector\_position** (bitvector, bool, start)

ビットベクターの開始位置から、最初の bool の出現位置のインデックスを返します。開始位置からビットベクターの終了位置までの間に bool エントリがない場合は、`#f` を返します。例:

(ビットベクトル位置 #\*000101 #t 0) ⇒ 3
(ビットベクトル位置 #\*0001111 #f 3) ⇒ #f

スキーム手順: **bitvector-set-bits!** bitvector bits

ビットベクトルのエントリを`#t`に設定します。`bits`は設定するビットを選択します。戻り値は未定義です。ビットベクトルは、少なくとも`bits`以上の長さである必要があります。

(define bv (bitvector-copy #\*11000010))
(bitvector-set-bits! bv #\*10010001)
bv
⇒ #\*11010011

スキーム手順: **bitvector-clear-bits!** bitvector bits

ビットベクターのエントリを`#f`に設定します。`bits`はクリアするビットを選択します。戻り値は未指定です。ビットベクターは`bits`以上の長さである必要があります。

(define bv (bitvector-copy #\*11000010))
(bitvector-clear-bits! bv #\*10010001)
bv
⇒ #\*01000010

C 関数: `int` **scm\_is\_bitvector** `(SCM obj)`

C 関数: `SCM` **scm\_c\_make\_bitvector** `(size_t len, SCM fill)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fc_005fmake_005fbitvector)

C 関数: `int` **scm\_bitvector\_bit\_is\_set** `(SCM vec, size_t idx)`

C 関数: `int` **scm\_bitvector\_bit\_is\_clear** `(SCM vec, size_t idx)`

C 関数: `void` **scm\_c\_bitvector\_set\_bit\_x** `(SCM vec, size_t idx)`

C 関数: `void` **scm\_c\_bitvector\_clear\_bit\_x** `(SCM vec, size_t idx)`

C 関数: `void` **scm\_c\_bitvector\_set\_bits\_x** `(SCM vec, SCM bits)`

C 関数: `void` **scm\_c\_bitvector\_clear\_bits\_x** `(SCM vec, SCM bits)`

C 関数: `void` **scm\_c\_bitvector\_set\_all\_bits\_x** `(SCM vec)`

C 関数: `void` **scm\_c\_bitvector\_clear\_all\_bits\_x** `(SCM vec)`

C 関数: `void` **scm\_c\_bitvector\_flip\_all\_bits\_x** `(SCM vec)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fc_005fbitvector_005fflip_005fall_005fbits_005fx)

C 関数: `size_t` **scm\_c\_bitvector\_length** `(SCM ビットベクトル)`

C 関数: `size_t` **scm\_c\_bitvector\_count** `(SCM ビットベクトル)`

C 関数: `size_t` **scm\_c\_bitvector\_count\_bits** `(SCM ビットベクトル、SCM ビット)`

対応するSchemeビットベクトルインターフェース用のC言語API。

C 関数: `const scm_t_uint32 *` **scm\_bitvector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *offp, size_t *lenp, ssize_t *incp)`

`scm_vector_elements`（[C言語からのベクトルアクセス](06_06_10_vectors.md#66104-c言語からのベクトルアクセス)を参照）と同様ですが、ビットベクトル用です。`offp`が指す変数には、`scm_array_handle_bit_elements_offset`が返す値が設定されます。返されたポインタとオフセットの使用方法については、`scm_array_handle_bit_elements`を参照してください。

C 関数: `scm_t_uint32 *` **scm\_bitvector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *offp, size_t *lenp, ssize_t *incp)`

`scm_bitvector_elements`と同様ですが、ポインタは読み書きに適しています。

* * *

次へ: [配列](06_06_13_arrays.md#6613-配列)、前: [ビットベクトル](#6611-ビットベクトル)、上: [データ型](06_06_00_data_types.md#66-データ型) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
