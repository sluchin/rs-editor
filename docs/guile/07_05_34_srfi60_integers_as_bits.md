#### 7.5.34 SRFI-60 - ビットとしての整数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d60-_002d-Integers-as-Bits)

この SRFI は、整数をビットとして扱うためのさまざまな関数と、ビット単位の操作のための関数を提供します。これらの関数は、以下を使用して取得できます。

(use-modules (srfi srfi-60))

整数は、コア論理関数と同様に、無限精度の2の補数として扱われます（[ビット演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bitwise-Operations)を参照）。同様に、ビットインデックスは最下位ビットから0で始まります。このSRFIの以下の関数は、すでにGuileコアに含まれています。

> `logand`、`logior`、`logxor`、`lognot`、`logtest`、`logcount`、`integer-length`、`logbit?`、`ash`

  

関数: **ビットごとのAND** n1 ... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bitwise_002dand)

関数: **bitwise-ior** n1 ... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bitwise_002dior)

関数: **ビットごとのXOR** n1 ... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bitwise_002dxor)

関数: **ビットごとの否定** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bitwise_002dnot)

機能: **any-bits-set?** jk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-any_002dbits_002dset_003f)

機能: **bit-set?** インデックス n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bit_002dset_003f)

関数: **arithmetic-shift** n カウント [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-arithmetic_002dshift)

機能: **bit-field** n 開始 終了 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bit_002dfield)

関数: **bit-count** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bit_002dcount)

それぞれ `logand`、`logior`、`logxor`、`lognot`、`logtest`、`logbit?`、`ash`、`bit-extract`、`logcount` のエイリアスです。

`bit-count` という名前は、コア内の `bit-count` と競合することに注意してください ([ビット ベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bit-Vectors) を参照)。

関数: **ビットごとのif** マスク n1 n0 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bitwise_002dif)

機能: **ビット単位マージ** マスク n1 n0 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-bitwise_002dmerge)

マスクに従ってn1とn0から選択されたビットを持つ整数を返します。マスクに1が含まれるビットはn1から、マスクに0が含まれるビットはn0から取得されます。

(ビットごとのif 3 #b0101 #b1010) ⇒ 9

関数: **log2-binary-factors** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-log2_002dbinary_002dfactors)

機能: **first-set-bit** n [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-first_002dset_002dbit)

nの中に2の約数がいくつあるかを返します。これはnの最下位1ビットのビットインデックスでもあります。nが0の場合は、-1が返されます。

(log2-バイナリ因子6) ⇒ 1
(log2-バイナリ因子 -8) ⇒ 3

関数: **copy-bit** index n newbit [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-copy_002dbit)

インデックスのビットをnewbitに従って設定したnを返します。newbitは、ビットを1に設定する場合は`#t`、0に設定する場合は`#f`を指定します。インデックス以外のビットは、戻り値では変更されません。

(コピービット 1 #b0101 #t) ⇒ 7

機能: **copy-bit-field** n newbits start end [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-copy_002dbit_002dfield)

n の先頭 (含む) から末尾 (含まない) までのビットを newbits の値に変更して返します。

newbits の最下位ビットは start に、次のビットは _start+1_ に割り当てられ、以下同様に割り当てられます。指定された end を超えた newbits の内容はすべて無視されます。

(copy-bit-field #b10000 #b11 1 3) ⇒ #b10110

機能: **rotate-bit-field** n count start end [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-rotate_002dbit_002dfield)

n を返します。n は、開始 (含む) から終了 (含まない) までのビットフィールドを、count ビットだけ上に回転させたものです。

カウントは正の値でも負の値でも構いませんし、フィールド幅を超える値でも構いません（その場合は幅を法として減算されます）。

(rotate-bit-field #b0110 2 1 4) ⇒ #b1010

機能: **reverse-bit-field** n 開始 終了 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reverse_002dbit_002dfield)

n の先頭（含む）から末尾（含まない）までのビットを反転させた値を返します。

(逆ビットフィールド #b101001 2 4) ⇒ #b100101

関数: **integer->list** n \[len\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-integer_002d_003elist)

n からビットを、1 の場合は `#t`、0 の場合は `#f` のリストの形式で返します。最下位 len ビットが返され、リストの最初の要素はそれらのビットの最上位ビットです。len が指定されていない場合、デフォルトは `(整数長 n)` です ([ビット演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bitwise-Operations) を参照)。

(整数→リスト 6) ⇒ (#t #t #f)
(整数→リスト 1 4) ⇒ (#f #f #f #t)

関数: **リスト→整数** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002d_003einteger)

関数: **booleans->integer** bool… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-booleans_002d_003einteger)

指定されたブール値のリストからビット単位で生成された整数を返すか、`booleans->integer` の場合はブール引数から整数を返します。

各ブール値は、1の場合は`#t`、0の場合は`#f`となります。最初の要素が戻り値の最上位ビットになります。

(list->integer '(#t #f #t #f)) ⇒ 10

* * *

次へ: [SRFI-62 - S式に関するコメント](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d62)、前: [SRFI-60 - ビットとしての整数](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d60)、上: [SRFIサポートモジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
