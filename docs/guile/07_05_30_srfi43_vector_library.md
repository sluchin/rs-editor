#### 7.5.30 SRFI-43 - ベクトルライブラリ

このサブセクションは、Taylor Campbell による [SRFI-43 の仕様](http://srfi.schemers.org/srfi-43/srfi-43.html) に基づいています。

SRFI-43は、包括的なベクトル演算ライブラリを実装しています。以下の方法で利用可能です。

(use-modules (srfi srfi-43))

* [SRFI-43 コンストラクター](#75301-srfi-43-コンストラクター)
* [SRFI-43 述語](#75302-srfi-43-述語)
* [SRFI-43 セレクター](#75303-srfi-43-セレクター)
* [SRFI-43 イテレーション](#75304-srfi-43-イテレーション)
* [SRFI-43検索](#75305-srfi-43-検索)
* [SRFI-43 ミューテーター](#75306-srfi-43-ミューテーター)
* [SRFI-43 変換](#75307-srfi-43-変換)

* * *

次へ: [SRFI-43 述語](#75302-srfi-43-述語)、上へ: [SRFI-43 - ベクトルライブラリ](https://doc.guix.gnu.org/guile/latest/en/tmguile. \[[目次](00_contents.md "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-xIndex\")

#### 7.5.30.1 SRFI-43 コンストラクター

Scheme手順: **make-vector** size \[fill\]

サイズが size のベクトルを作成し、必要に応じて fill で埋めて返します。fill のデフォルト値は指定されていません。

(make-vector 5 3) ⇒ #(3 3 3 3 3)

Scheme Procedure: **vector** x …

要素が x ... であるベクトルを作成して返します。

(ベクトル 0 1 2 3 4) ⇒ #(0 1 2 3 4)

スキーム手順: **vector-unfold** f length initial-seed …

基本的なベクトルコンストラクタ。長さが length のベクトルを作成し、インデックス k を 0 から length - 1 まで順に処理します。各イテレーションで、現在のインデックスと現在のシードに f を順番に適用し、n + 1 個の値を取得します。これらの値は、新しいベクトルの k 番目のスロットに格納する要素と、次のイテレーションで使用する n 個の新しいシードです。イテレーションごとにシードの数が異なるとエラーになります。

(vector-unfold (lambda (ix) (values x (- x 1)))
10 0)
⇒ #(0 -1 -2 -3 -4 -5 -6 -7 -8 -9)

（ベクトル展開値 10）
⇒ #(0 1 2 3 4 5 6 7 8 9)

スキーム手順: **vector-unfold-right** f length initial-seed …

`vector-unfold`に似ていますが、f関数を使って左から右ではなく、右から左に要素を生成します。

(vector-unfold-right (lambda (ix) (values x (+ x 1)))
10 0)
⇒ #(9 8 7 6 5 4 3 2 1 0)

Scheme手順: **vector-copy** vec \[start \[end \[fill\]\]\]

長さが end - start の新しいベクトルを割り当て、vec から要素を取得します。要素は、インデックス start から始まり、インデックス end で終了します。start のデフォルト値は 0、end のデフォルト値は `(vector-length vec)` です。end が vec の長さを超える場合、vec の要素では埋められない新しいベクトルのスロットは fill で埋められます。fill のデフォルト値は指定されていません。

(vector-copy '#(abcdefghi))
⇒ #(abcdefghi)

(vector-copy '#(abcdefghi) 6)
⇒ #(ghi)

(vector-copy '#(abcdefghi) 3 6)
⇒ #(def)

(vector-copy '#(abcdefghi) 6 12 'x)
⇒ #(ghixxx)

Scheme手順: **vector-reverse-copy** vec \[start \[end\]\]

`vector-copy` と同様ですが、vec から逆の順序で要素をコピーします。

(vector-reverse-copy '#(5 4 3 2 1 0) 1 5)
⇒ #(1 2 3 4)

スキーム手順: **vector-append** vec …

vec ... の次の位置からすべての要素を順番に含む、新しく割り当てられたベクトルを返します。

(vector-append '#(a) '#(bcd))
⇒ #(abcd)

Scheme手順: **vector-concatenate** ベクトルのリスト

各ベクトルをリスト・オブ・ベクターズに追加します。`(apply vector-append list-of-vectors)` と同等です。

(vector-concatenate '(#(ab) #(cd)))
⇒ #(abcd)

* * *

次へ: [SRFI-43 セレクタ](#75303-srfi-43-セレクター)、前へ: [SRFI-43 コンストラクタ](#75301-srfi-43-コンストラクター)、上へ: [SRFI-43 - ベクトルライブラリ](#7530-srfi-43---ベクトルライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.30.2 SRFI-43 述語

Scheme手順: **vector?** obj

objがベクトルであればtrueを返し、そうでなければfalseを返します。

スキーム手順: **vector-empty?** vec

vecが空（つまり長さが0）の場合はtrueを返し、そうでない場合はfalseを返します。

Scheme Procedure: **vector=** elt=? vec …

ベクトル vec … の長さと要素が elt=? に従って等しい場合、true を返します。elt=? は常に 2 つの引数に適用されます。要素の比較は、次の意味で `eq?` と整合していなければなりません。`(eq? ab)` が true を返す場合、`(elt=? ab)` も true を返さなければなりません。比較を実行する順序は指定されていません。

* * *

次へ: [SRFI-43 イテレーション](#75304-srfi-43-イテレーション)、前: [SRFI-43 述語](#75302-srfi-43-述語)、上: [SRFI-43 - ベクトルライブラリ](#7530-srfi-43---ベクトルライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.30.3 SRFI-43 セレクター

Scheme 手順: **vector-ref** vec i

vec内のインデックスiにある要素を返します。インデックスは0から始まります。

Scheme手順: **vector-length** vec

vec の長さを返します。

* * *

次へ: [SRFI-43 検索](#75305-srfi-43-検索)、前へ: [SRFI-43 セレクタ](#75303-srfi-43-セレクター)、上へ: [SRFI-43 - ベクトルライブラリ](#7530-srfi-43---ベクトルライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.30.4 SRFI-43 イテレーション

Scheme Procedure: **vector-fold** kons knil vec1 vec2 …

基本的なベクトルイテレータ。kons はすべてのベクトルの各インデックスに対して反復され、最短の末尾で停止します。kons は次のように適用されます。

(kons i state ([vector-ref](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更) vec1 i) ([vector-ref](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更) vec2 i) [...](06_08_macros.md#6821-パターン))

ここで、state は現在の状態値、i は現在のインデックスです。現在の状態値は knil から始まり、各イテレーションで kons が返す値になります。イテレーションは厳密に左から右に行われます。

Scheme Procedure: **vector-fold-right** kons knil vec1 vec2 …

`vector-fold`に似ていますが、左から右ではなく右から左に処理を行います。

Scheme Procedure: **vector-map** f vec1 vec2 …

ベクトル引数の中で最短サイズの新しいベクトルを返します。新しいベクトルのインデックス i の各要素は、古いベクトルから次のようにマッピングされます。

(fi ([vector-ref](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更) vec1 i) ([vector-ref](06_06_10_vectors.md#66103-ベクトルコンテンツへのアクセスと変更) vec2 i) [...](06_08_macros.md#6821-パターン))

fの適用順序は動的に規定されていない。

Scheme手順: **vector-map!** f vec1 vec2 …

`vector-map`に似ていますが、新しい要素を新しいベクトルにマッピングするのではなく、マッピングされた新しい要素がvec1に破壊的に挿入されます。fの適用順序は動的に指定されていません。

Scheme Procedure: **vector-for-each** f vec1 vec2 …

渡された最短ベクトルの長さよりも小さいインデックス i ごとに、`(fi (vector-ref vec1 i) (vector-ref vec2 i) ...)` を呼び出します。反復処理は厳密に左から右に行われます。

Scheme Procedure: **vector-count** pred? vec1 vec2 …

最小ベクトルの長さより小さい各インデックス i に対して、i およびそのインデックスにあるベクトルの各並列要素に順番に適用され、pred? を満たすベクトル内の並列要素の数を数えます。

(vector-count (lambda (i elt) (even? elt))
'#(3 1 4 1 5 9 2 5 6))
⇒ 3
(vector-count (lambda (ixy) (< xy))
'#(1 3 6 9) '#(2 4 6 8 10 12))
⇒ 2

* * *

次へ: [SRFI-43 ミューテーター](#75306-srfi-43-ミューテーター)、前へ: [SRFI-43 イテレーション](#75304-srfi-43-イテレーション)、上へ: [SRFI-43 - ベクトルライブラリ](#7530-srfi-43---ベクトルライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.30.5 SRFI-43 検索

Scheme Procedure: **vector-index** pred? vec1 vec2 …

vec1、vec2、…の中でpred?を満たす最初の要素のインデックスを見つけて返します。最短ベクトルの末尾までに一致する要素が見つからない場合は、`#f`を返します。

(ベクトルインデックスが偶数か? '#(3 1 4 1 5 9))
⇒ 2
(vector-index < '#(3 1 4 1 5 9 2 5 6) '#(2 7 1 8 2))
⇒ 1
(vector-index = '#(3 1 4 1 5 9 2 5 6) '#(2 7 1 8 2))
⇒ #f

Scheme Procedure: **vector-index-right** pred? vec1 vec2 …

`vector-index` と同様ですが、左から右ではなく右から左に検索します。SRFI 43 仕様ではすべてのベクトルの長さが同じでなければならないと規定されていますが、SRFI 43 のリファレンス実装と Guile の実装はどちらも長さの異なるベクトルを許容し、最短のベクトルの最後のインデックスから検索を開始します。

Scheme Procedure: **vector-skip** pred? vec1 vec2 …

vec1、vec2、…の中で、pred?を満たさない最初の要素のインデックスを見つけて返します。最短ベクトルの末尾までに一致する要素が見つからない場合は、`#f`を返します。`vector-index`と同等ですが、述語が反転しています。

(ベクトルスキップ番号? '#(1 2 ab 3 4 cd)) ⇒ 2

Scheme Procedure: **vector-skip-right** pred? vec1 vec2 …

`vector-skip` と同様ですが、一致しない要素を左から右ではなく右から左に検索します。SRFI 43 仕様ではすべてのベクトルの長さが同じでなければならないと規定されていますが、SRFI 43 のリファレンス実装と Guile の実装はどちらも長さの異なるベクトルを許可し、最短のベクトルの最後のインデックスから検索を開始します。

Scheme Procedure: **vector-binary-search** vec value cmp \[start \[end\]\]

バイナリサーチを使用して、開始位置と終了位置の間で、値が vec の値と一致するインデックスを検索して返します。一致する要素が見つからない場合は、`#f` を返します。デフォルトの開始位置は 0、デフォルトの終了位置は vec の長さです。

cmpは2つの引数を取る手続きでなければならず、`(cmp ab)`は_a < b_の場合は負の整数を、_a > b_の場合は正の整数を、_a = b_の場合はゼロを返す必要があります。vecの要素はcmpに従って非減少順にソートされていなければなりません。

SRFI 43では開始引数と終了引数については文書化されていませんが、参照実装とGuileの実装の両方でこれらの引数がサポートされています。

(define (char-cmp c1 c2)
(条件 ((char<? c1 c2) -1)
((char>? c1 c2) 1)
（それ以外の場合は0））

(vector-binary-search '#(#\\a #\\b #\\c #\\d #\\e #\\f #\\g #\\h)
#\\g
文字比較）
⇒ 6

Scheme Procedure: **vector-any** pred? vec1 vec2 …

vec1、vec2…から、pred?が真値を返す最初の並列要素セットを見つけます。そのような並列要素セットが存在する場合、`vector-any`はその要素セットに対してpred?が返した値を返します。反復処理は厳密に左から右に行われます。

Scheme Procedure: **vector-every** pred? vec1 vec2 …

0 から最短ベクトル引数の長さまでのすべてのインデックス i について、要素の集合 `(vector-ref vec1 i)` `(vector-ref vec2 i)` … が pred? を満たす場合、`vector-every` は、最短ベクトルの最後のインデックスで pred? が返した最後の要素の集合の値を返します。そうでない場合は `#f` を返します。反復処理は厳密に左から右に行われます。

* * *

次へ: [SRFI-43 変換](#75307-srfi-43-変換)、前へ: [SRFI-43 検索](#75305-srfi-43-検索)、上へ: [SRFI-43 - ベクトルライブラリ](#7530-srfi-43---ベクトルライブラリ) \[[目次](00_contents.md "テーブルコンテンツ")\]\[[インデックス](index_r5rs.md "インデックス")\]

#### 7.5.30.6 SRFI-43 ミューテーター

Scheme Procedure: **vector-set!** vec i value

vec の i の位置にある内容を value に代入します。

スキーム手順: **vector-swap!** vec ij

vec内のiとjの位置の値を入れ替えます。

Scheme 手順: **vector-fill!** vec fill \[start \[end\]\]

開始位置と終了位置の間のvec内のすべての場所に値を割り当てて埋めます。開始位置のデフォルト値は0、終了位置のデフォルト値はvecの長さです。

Scheme手順: **vector-reverse!** vec \[start \[end\]\]

vec の内容を、開始位置と終了位置の間で破壊的に反転します。開始位置はデフォルトで 0、終了位置はデフォルトで vec の長さになります。

Scheme Procedure: **vector-copy!** target tstart source \[sstart \[send\]\]

ソースからターゲットへ、ベクトルである要素のブロックをコピーします。ターゲットではtstart、ソースではsstartから開始し、(send - sstart)個の要素がコピーされた時点で終了します。ターゲットの長さが(tstart + send - sstart)より短い場合はエラーとなります。sstartのデフォルト値は0、sendのデフォルト値はソースの長さです。

Scheme Procedure: **vector-reverse-copy!** target tstart source \[sstart \[send\]\]

`vector-copy!` と同様ですが、要素を逆順にコピーします。ターゲットとソースが同一のベクトルで、ターゲットとソースの範囲が重複している場合はエラーになります。ただし、tstart = sstart の場合、`vector-reverse-copy!` は `(vector-reverse! target tstart send)` と同じように動作します。

* * *

前へ: [SRFI-43 ミューテーター](#75306-srfi-43-ミューテーター)、上へ: [SRFI-43 - ベクトルライブラリ](#7530-srfi-43---ベクトルライブラリ) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 7.5.30.7 SRFI-43 変換

Scheme手順: **vector->list** vec \[start \[end\]\]

vec 内の start と end の間の要素を含む、新しく割り当てられたリストを返します。start のデフォルト値は 0、end のデフォルト値は vec の長さです。

Scheme手順: **reverse-vector->list** vec \[start \[end\]\]

`vector->list`と同様ですが、結果として得られるリストには、vecの指定された範囲の要素が逆順で含まれます。

Scheme手順: **list->vector** proper-list \[start \[end\]\]

開始値と終了値の間のインデックスを持つ、proper-list の要素からなる、新たに割り当てられたベクトルを返します。開始値のデフォルト値は 0、終了値のデフォルト値は proper-list の長さです。なお、SRFI 43 では開始値と終了値の引数については説明されていませんが、参照実装と Guile の実装の両方でサポートされています。

Scheme手順: **reverse-list->vector** proper-list \[start \[end\]\]

`list->vector`と同様ですが、結果として得られるベクトルには、proper-listの指定された範囲の要素が逆順で格納されます。なお、SRFI 43ではstart引数とend引数については説明されていませんが、参照実装とGuileの実装の両方でサポートされています。

* * *

次へ: [SRFI-46 基本構文規則拡張](07_05_32_srfi46_basic_syntaxrules_extensions.md#7532-srfi-46-基本構文規則の拡張)、前: [SRFI-43 - ベクトルライブラリ](#7530-srfi-43---ベクトルライブラリ)、上: [SRFIサポートモジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
