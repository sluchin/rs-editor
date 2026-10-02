### 6.9 一般的なユーティリティ関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#General-Utility-Functions)

この章では、特定のデータ型に明確に結び付けられていない手続きに関する情報を取り上げています。これらの手続きは用途が広いため、「ユーティリティ」の章にまとめられています。

* [Equality](https://doc.guix.gnu.org/guile/latest/en/guile.html#Equality)
* [オブジェクトプロパティ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-Properties)
* [ソート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Sorting)
* [ディープ構造のコピー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Copying)
* [一般的な文字列変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#General-Conversion)
* [フック](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks)

* * *

次へ: [オブジェクトプロパティ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-Properties)、上: [汎用ユーティリティ関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Utility-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.1 平等 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Equality-1)

Schemeには、以下に説明する3種類の基本的な等価述語があります。同様の比較は、`memq`などの他の関数でも発生します（[リスト検索](https://doc.guix.gnu.org/guile/latest/en/guile.html#List-Searching)を参照）。

これら3つのテストすべてにおいて、異なる型のオブジェクトは決して等しくありません。例えば、リストとベクトルは、内容が同じであっても「等しい」とはみなされません。正確な数値と不正確な数値も異なる型とみなされるため、値が同じであっても等しくはありません。

`eq?` は、同じオブジェクトであるかどうかのみをテストします（基本的にはポインタの比較です）。これは高速で、特定のオブジェクトを検索する場合や、シンボルやキーワード（常に一意のオブジェクト）を扱う場合に使用できます。

`eqv?` は `eq?` を拡張し、数値と文字の値を参照できるようにします。たとえば、`=` のように使用できます ([比較述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison) )が、オペランドの 1 つが数値でない場合でもエラーは発生しません。

`equal?` はさらに一歩進んで、リストやベクトルなどの内容を（再帰的に）調べます。これは、例えば、さまざまな場所で読み込まれたり計算されたりして、同じ要素のペアで構成されているわけではないものの、内容は同じリストの場合に有効です。このようなリストは（印刷すると）同じように見えるため、`equal?` はそれらを同じものとみなします。

  

スキーム手順: **eq?** … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eq_003f)

C 関数: **scm\_eq\_p** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feq_005fp)

Scheme の手続きは、数値と文字を除くすべての引数が同じオブジェクトである場合、`#t` を返します。C の関数も同様ですが、引数は正確に 2 つ取ります。たとえば、

(define x (vector 1 2 3))
(define y (vector 1 2 3))

(eq? xx) ⇒ #t
(eq? xy) ⇒ #f

数値や文字は他のオブジェクトと等しくはありませんが、問題は、それらが必ずしも自分自身とも等しくないということです。これは、数値が変数から直接取得された場合でも同様です。

(let ((n (+ 2 3)))
(eq? nn)) ⇒ \*未指定\*

一般的に、数値や文字を比較する場合は、以下の `eqv?` を使用してください。`=` ([比較述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#Comparison) を参照) または `char=?` ([文字](https://doc.guix.gnu.org/guile/latest/en/guile.html#Characters) を参照) も使用できます。

リストの末尾の `()`、`#t`、`#f`、特定の名前のシンボル、特定の名前のキーワードは、それぞれ固有のオブジェクトであることに注意が必要です。それぞれが 1 つずつしか存在しないため、たとえばプログラム内で `()` がどのように出現しても、それは同じオブジェクトであり、`eq?` と比較できます。

(define x (cdr '(123)))
(define y (cdr '(456)))
(eq? xy) ⇒ #t

(define x (string->symbol "foo"))
(eq? x 'foo) ⇒ #t

C 関数: `int` **scm\_is\_eq** `(SCM x, SCM y)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fis_005feq)

xとyが`eq?`の意味で等しい場合は`1`を返し、そうでない場合は`0`を返します。

`==`演算子は`SCM`値には使用しないでください。`SCM`はC型であり、必ずしも`==`を使用して比較できるとは限りません（[SCM型](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type)を参照）。

  

Scheme Procedure: **eqv?** … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eqv_003f)

C 関数: **scm\_eqv\_p** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005feqv_005fp)

Schemeの手続きは、引数がすべて同じオブジェクトの場合、または文字と数値が同じ値の場合、`#t`を返します。C言語の関数も同様ですが、引数は正確に2つ取ります。

文字と数値以外のオブジェクトの場合、`eqv?` は上記の `eq?` と同じです。`(eqv? xy)` は、x と y が同じオブジェクトである場合に真となります。

xとyが数値または文字の場合、`eqv?`はそれらの型と値を比較します。正確な数値は、不正確な数値とは`eqv?`しません（たとえ値が同じであっても）。

(eqv? 3 (+ 1 2)) ⇒ #t
(eqv? 1 1.0) ⇒ #f

  

Scheme Procedure: **equal?** … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-equal_003f)

C 関数: **scm\_equal\_p** (x, y) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fequal_005fp)

Schemeの手続きは、引数がすべて同じ型で、かつその内容または値が等しい場合に`#t`を返します。C言語の関数も同様ですが、引数は正確に2つ取ります。

ペア、文字列、ベクトル、配列、構造体の場合、`equal?` は内容を比較し、同じ `equal?` を再帰的に使用して比較するため、深い構造を走査できます。

(等しい? (リスト 1 2 3) (リスト 1 2 3)) ⇒ #t
(等しい? (リスト 1 2 3) (ベクトル 1 2 3)) ⇒ #f

その他のオブジェクトについては、`equal?` は上記の `eqv?` と同様に比較します。つまり、文字と数値は型と値によって比較されます（`eqv?` と同様に、正確な数値と不正確な数値は、値が同じであっても `equal?` とはみなされません）。

(等しい? 3 (+ 1 2)) ⇒ #t
(等しい? 1 1.0) ⇒ #f

ハッシュテーブルは現在、`eq?` に従ってのみ比較されるため、内容が同じであっても、2つの異なるテーブルは `equal?` とはみなされません。

`equal?` は循環データ構造をサポートしていないため、2 つの循環リストなどを比較するように指示された場合、無限ループに陥る可能性があります。

GOOPS オブジェクト型 ([GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) を参照) および外部オブジェクト型 ([新しい外部オブジェクト型の定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Defining-New-Foreign-Object-Types) を参照) は、同じ型の 2 つの値に特化した `equal?` 実装を持つことができます。同じ型の 2 つの GOOPS オブジェクトに対して `equal?` が呼び出されると、`equal?` は汎用関数にディスパッチします。これにより、アプリケーションは、その型の 2 つのオブジェクトの内容を走査したり、何が `equal?` とみなされるかを制御したりできます。そのようなハンドラがない場合、デフォルトでは `eq?` に従って比較されます。

* * *

次へ: [ソート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Sorting)、前: [等価性](https://doc.guix.gnu.org/guile/latest/en/guile.html#Equality)、上: [汎用ユーティリティ関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Utility-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.2 オブジェクトプロパティ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-Properties-1)

Schemeオブジェクトに、追加情報を格納するための専用スロットがない場合でも、そのオブジェクトに追加情報を関連付けることが便利な場合がよくあります。オブジェクトプロパティを使用すると、まさにそれが可能になります。

Guile におけるオブジェクト プロパティの表現は、セッター付きプロシージャ ([セッター付きプロシージャ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Procedures-with-Setters) を参照) であり、`set!` の一般化された形式と組み合わせて、任意の Scheme オブジェクトのプロパティを設定および取得するために使用できます。したがって、プロパティを設定するには、次のように記述します。

([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) (my-property obj1) value-for-obj1)
([set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_0021) (my-property obj2) value-for-obj2)

同じプロパティの値を取得するには、次のようになります。

(私のプロパティ obj1)
⇒
value-for-obj1

(my-property obj2)
⇒
value-for-obj2

オブジェクトプロパティを最初に作成するには、`make-object-property` プロシージャを使用します。

(define my-property ([make-object-property](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dobject_002dproperty)))

Scheme手順: **make-object-property** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dobject_002dproperty)

オブジェクトプロパティを作成して返します。オブジェクトプロパティは、セッターを持つプロシージャであり、2つの方法で呼び出すことができます。`(set! (property obj) val)` は、obj のプロパティを val に設定します。`(property obj)` は、obj のプロパティの現在の設定を返します。

`make-object-property` によって作成された単一のオブジェクトプロパティは、`eq?` で区別可能なすべての Scheme 値に、異なるプロパティ値を関連付けることができます (数値は除外されます)。

内部的には、オブジェクトのプロパティは弱いキーのハッシュテーブルを使用して実装されています。つまり、プロパティ値を持つ Scheme 値がガベージコレクションから保護されている限り、そのプロパティ値も保護されます。Scheme 値がガベージコレクションされると、プロパティテーブル内のエントリが削除されるため、（以前の）プロパティ値はテーブルによって保護されなくなります。

Guileは、プロパティに関してより伝統的なLispyインターフェースも実装しています。このインターフェースでは、各オブジェクトに関連付けられたキーと値のペアのリストを持ちます。そのリスト内のプロパティはシンボルをキーとして指定されます。これは旧来のインターフェースであるため、代わりに弱いハッシュテーブルまたはオブジェクトプロパティを使用することをお勧めします。

Scheme手順: **object-properties** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-object_002dproperties)

C 関数: **scm\_object\_properties** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fobject_005fproperties)

objのプロパティリストを返します。

Scheme 手順: **set-object-properties!** オブジェクトリスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dobject_002dproperties_0021)

C 関数: **scm\_set\_object\_properties\_x** (obj, alist) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fobject_005fproperties_005fx)

objのプロパティリストをalistに設定します。

Scheme Procedure: **object-property** obj key [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-object_002dproperty)

C 関数: **scm\_object\_property** (obj, key) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fobject_005fproperty)

名前がキーとなっているobjのプロパティを返します。

Scheme 手順: **set-object-property!** obj key value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-set_002dobject_002dproperty_0021)

C 関数: **scm\_set\_object\_property\_x** (obj, key, value) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fset_005fobject_005fproperty_005fx)

objのプロパティリストで、keyという名前のプロパティに値を設定します。

* * *

次へ: [ディープ構造のコピー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Copying)、前: [オブジェクトプロパティ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Object-Properties)、上: [汎用ユーティリティ関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Utility-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.3 ソート [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Sorting-1)

コンピュータプログラムにおいて、ソートは非常に重要です。そのため、Guileにはいくつかのソート手順が組み込まれています。いつものように、名前の末尾が「!」で終わる手順は副作用を持ちます。つまり、結果を生成するためにパラメータを変更する可能性があります。以下の手順に2番目または3番目の引数として渡される述語 less は、マージまたはソートされる要素に対して厳密な弱い順序を定義するものとみなされます。

最初の手順群は、2つのリスト（それぞれ既にソートされている必要がある）をマージし、入力リストのすべての要素を含むソート済みリストを生成するために使用できます。

Scheme Procedure: **merge** alist blist less [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-merge)

C 関数: **scm\_merge** (alist、blist、less) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmerge)

既にソート済みの 2 つのリストを 1 つにマージします。 `(sorted? alist less?)` および `(sorted? blist less?)` を満たす 2 つのリスト alist と blist が与えられた場合、`(sorted? (merge alist blist less?) less?)` となるように alist と blist の要素が安定的にインターリーブされた新しいリストを返します。 注: これはベクトルを受け付けません。

Scheme 手順: **merge!** alist blist less [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-merge_0021)

C 関数: **scm\_merge\_x** (alist、blist、less) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmerge_005fx)

`(sorted? alist less?)` および `(sorted? blist less?)` を満たす 2 つのリスト alist と blist を受け取り、`(sorted? (merge alist blist less?) less?)` を満たすように alist と blist の要素が安定的にインターリーブされた新しいリストを返します。これは `merge` の破壊的なバリアントです。注: これはベクトルを受け付けません。

以下の手順は、ベクトルまたはリストのいずれかであるシーケンスに対して操作を実行できます。指定された引数に応じて、それぞれソート済みのベクトルまたはリストを返します。以下の手順のうち、最初の手順はシーケンスが既にソートされているかどうかを判定し、もう1つの手順は指定されたシーケンスをソートします。`stable-` で始まる名前のバリアントは、入力シーケンスの特別なプロパティを維持するという点で特殊です。比較述語に従って 2 つ以上の要素が同じである場合、それらは入力に現れた順序のまま保持されます。

スキーム手順: **ソート済み?** 項目数を減らす [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sorted_003f)

C 関数: **scm\_sorted\_p** (items, less) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsorted_005fp)

itemsがリストまたはベクトルであり、各要素xと次の要素yに対して、(less yx)が#fを返す場合、`#t`を返します。そうでない場合は、`#f`を返します。

Scheme Procedure: **sort** items less [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sort)

C 関数: **scm\_sort** (items, less) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsort)

リストまたはベクトルであるシーケンス項目をソートします。less関数はシーケンス要素の比較に使用されます。これは安定ソートではありません。

Scheme Procedure: **sort!** items less [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sort_0021)

C 関数: **scm\_sort\_x** (items, less) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsort_005fx)

リストまたはベクトルであるシーケンス項目をソートします。シーケンス要素の比較にはless関数を使用します。このソートは破壊的であり、ソート結果を生成するために入力シーケンスが変更されます。これは安定ソートではありません。

Scheme Procedure: **stable-sort** items less [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stable_002dsort)

C 関数: **scm\_stable\_sort** (items, less) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstable_005fsort)

リストまたはベクトルであるシーケンス項目をソートします。less関数はシーケンス要素の比較に使用されます。これは安定ソートです。

Scheme 手順: **stable-sort!** 項目の数が少ない [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-stable_002dsort_0021)

C 関数: **scm\_stable\_sort\_x** (items, less) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fstable_005fsort_005fx)

リストまたはベクトルであるシーケンス項目をソートします。シーケンス要素の比較にはless関数を使用します。ソートは破壊的であり、ソート結果を生成するために入力シーケンスが変更されます。これは安定ソートです。

最後のグループの手続きは、その名前が示すとおり、入力としてリストまたはベクトルのみを受け入れます。

スキーム手順: **sort-list** 項目を [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sort_002dlist) より少なくする

C 関数: **scm\_sort\_list** (items, less) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsort_005flist)

リスト項目を、lessを使って比較することでソートします。これは安定ソートです。

スキーム手順: **sort-list!** 項目を [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sort_002dlist_0021) より少なくする

C 関数: **scm\_sort\_list\_x** (items, less) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fsort_005flist_005fx)

リスト項目を、lessを使って比較することでソートします。このソートは破壊的であり、ソート結果を生成するために入力リストが変更されます。これは安定ソートです。

Scheme 手順: **restricted-vector-sort!** vec less startpos endpos [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-restricted_002dvector_002dsort_0021)

C 関数: **scm\_restricted\_vector\_sort\_x** (vec, less, startpos, endpos) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frestricted_005fvector_005fsort_005fx)

ベクターvecを、lessを使用してベクター要素を比較しながらソートします。startpos（含む）とendpos（含まない）は、ソート対象となるベクターの範囲を指定します。戻り値は指定されていません。

* * *

次へ: [一般的な文字列変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#General-Conversion)、前へ: [ソート](https://doc.guix.gnu.org/guile/latest/en/guile.html#Sorting)、上へ: [一般的なユーティリティ関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Utility-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.4 ディープ構造のコピー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Copying-Deep-Structures)

リストをコピーする手順（[Lists](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lists)を参照）では、入力リストのフラットコピーしか生成されず、現在のGuileにはベクトルをコピーする手順すら含まれていません。`(ice-9 copy-tree)`モジュールには、この目的に使用できる`copy-tree`関数が含まれています。この関数は、リストのスパインをコピーするだけでなく、入力リストのカーにあるペアもコピーします。

(use-modules (ice-9 copy-tree))

Scheme手順: **copy-tree** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-copy_002dtree)

C 関数: **scm\_copy\_tree** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fcopy_005ftree)

obj にバインドされているデータツリーを再帰的にコピーし、新しいデータ構造を返します。`copy-tree` は、ペアとベクトルの両方の内容を再帰的に辿り（cons セルとベクトルセルはどちらも任意のオブジェクトを指す可能性があるため）、他のオブジェクトに到達すると再帰を停止します。

* * *

次へ: [フック](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks)、前: [ディープ構造のコピー](https://doc.guix.gnu.org/guile/latest/en/guile.html#Copying)、上: [汎用ユーティリティ関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Utility-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.5 一般的な文字列変換 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#General-String-Conversion)

Schemeプログラムのデバッグ時だけでなく、人間にとって分かりやすいインターフェースを提供するためにも、Schemeオブジェクトを文字列形式に変換する手順は非常に便利です。もちろん、変換対象オブジェクトのデータ型が分かっている場合は、専用の手順を使って文字列への変換も可能ですが、この手順の方が多くの場合、より快適です。

`object->string` は、プリント処理を使用して文字列ポートに書き込み、結果として得られる文字列を返すことでオブジェクトを文字列に変換します。オブジェクトを文字列から文字列に戻すことができるのは、オブジェクト型に読み取り構文があり、かつその読み取り構文がプリント処理によって保持される場合に限られます。

Scheme手順: **object->string** obj \[printer\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-object_002d_003estring)

C 関数: **scm\_object\_to\_string** (obj, printer) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fobject_005fto_005fstring)

obj を出力して得られた Scheme 文字列を返します。出力関数は、オプションの 2 番目の引数 printer で指定できます (デフォルト: `write`)。

* * *

前へ: [一般的な文字列変換](https://doc.guix.gnu.org/guile/latest/en/guile.html#General-Conversion)、上へ: [一般的なユーティリティ関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Utility-Functions) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.6 フック [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks-1)

フックとは、特定の時点で呼び出されるプロシージャのリストです。通常、アプリケーションはフックhを提供し、アプリケーション処理の特定の時点でhに含まれるすべてのプロシージャを呼び出すことをユーザーに約束します。アプリケーションユーザーは、hに独自のプロシージャを追加することで、アプリケーションの処理状況にアクセスしたり、処理の進行に影響を与えたりすることができます。

Guile自体には、デバッグやカスタマイズの目的で使用できるフックがいくつか用意されています。これらは以下のサブセクションで説明します。

アプリケーションがフックを初めて作成する際、フックの実行時にフックのプロシージャに渡される引数の数を把握しておく必要があります。引数の数（0個の場合もあります）はフックの作成時に宣言され、そのフックに追加されるすべてのプロシージャは、その引数の数を受け入れることができなければなりません。

フックは `make-hook` を使用して作成します。プロシージャは `add-hook!` または `remove-hook!` を使用してフックに追加または削除でき、フックのすべてのプロシージャは`reset-hook!` を使用してまとめて削除できます。アプリケーションがフックを実行したい場合は、`run-hook` を使用して実行します。

* [フックの使用例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hook-Example)
* [フックリファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hook-Reference)
* [Cコード用のフック](https://doc.guix.gnu.org/guile/latest/en/guile.html#C-Hooks)
* [ガベージコレクション用のフック](https://doc.guix.gnu.org/guile/latest/en/guile.html#GC-Hooks)
* [Guile REPLへのフック](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Hooks)

* * *

次へ: [フックリファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hook-Reference)、上へ: [フック](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.6.1 フックの使用例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hook-Usage-by-Example)

このセクションでは、フックの使用方法をいくつかの例で示します。まず、引数の数を2つ持つフックを定義します。つまり、フックに格納されるプロシージャは2つの引数を受け取る必要があります。

(define hook ([make-hook](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dhook) 2))
フック
⇒ #<hook 2 40286c90>

これで、`add-hook!` コマンドを使って、新しく作成したフックにいくつかのプロシージャを追加する準備が整いました。次の例では、異なるメッセージを出力し、引数に対して異なる処理を行う 2 つのプロシージャを追加しています。

([add-hook!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dhook_0021) hook (lambda (xy)
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "Foo: ")
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) xy))
([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))))
([add-hook!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dhook_0021) hook (lambda (xy)
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "バー: ")
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) xy))
([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))))

手順を追加したら、`run-hook`を使用してフックを呼び出すことができます。

([run-hook](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-run_002dhook) フック 3 4)
⊣バー: 12
⊣ Foo: 7

プロシージャは追加された順序とは逆の順序で呼び出されることに注意してください。これは、`add-hook!` のデフォルトの動作が、そのプロシージャをフックのプロシージャリストの先頭に追加するためです。`add-hook!` の 2 回目の呼び出しで 3 番目の引数 `#t` を指定することで、プロシージャをリストの末尾に追加するように強制できます。

([add-hook!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dhook_0021) hook (lambda (xy)
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "Foo: ")
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) xy))
([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))))
([add-hook!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dhook_0021) hook (lambda (xy)
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) "バー: ")
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) ([\*](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002a) xy))
([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline)))
#t) ; ←ここを変更してください！
([run-hook](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-run_002dhook) フック 3 4)
⊣ Foo: 7
⊣バー: 12

* * *

次へ: [C コードのフック](https://doc.guix.gnu.org/guile/latest/en/guile.html#C-Hooks)、前: [フックの使用例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hook-Example)、上: [フック](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.6.2 フックリファレンス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hook-Reference-1)

`make-hook` を使用してフックを作成する場合、フックに追加できるプロシージャの引数の数を指定する必要があります。引数の数が `make-hook` の引数として明示的に指定されていない場合、デフォルト値はゼロになります。特定のフックのすべてのプロシージャは同じ引数数を持つ必要があり、 `run-hook` を使用してプロシージャを呼び出す際には、渡される引数の数がフック作成時に指定された引数の数と一致している必要があります。

プロシージャをフックに追加する順序は重要です。`add-hook!` の 3 番目のパラメータが省略されているか、`#f` と等しい場合、プロシージャは既にそのフックに存在する可能性のあるプロシージャの前に追加されます。それ以外の場合は、プロシージャは最後に追加されます。プロシージャは、`run-hook` を介して呼び出される際に、常にリストの先頭から末尾の順に呼び出されます。

`hook->list` によって返されるプロシージャのリストの順序は、`run-hook` を使用してフックが実行された場合にそれらのプロシージャが呼び出される順序と一致します。

以下の項目にある C 関数は、C 言語で Scheme レベルのフックを処理するためのものであることに注意してください。独自のインターフェースを持つ C レベルのフックも存在します ([C コード用のフック](https://doc.guix.gnu.org/guile/latest/en/guile.html#C-Hooks) を参照)。

Scheme手順: **make-hook** \[n\_args\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dhook)

C 関数: **scm\_make\_hook** (n\_args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fmake_005fhook)

n_args個の引数を持つ格納プロシージャ用のフックを作成します。n_argsのデフォルト値は0です。返される値は、他のフックプロシージャで使用するためのフックオブジェクトです。

Scheme Procedure: **hook?** x [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hook_003f)

C 関数: **scm\_hook\_p** (x) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhook_005fp)

xがフックの場合は`#t`を返し、そうでない場合は`#f`を返します。

Scheme 手順: **hook-empty?** フック [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hook_002dempty_003f)

C 関数: **scm\_hook\_empty\_p** (フック) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhook_005fempty_005fp)

フックが空のフックの場合は `#t` を返し、それ以外の場合は `#f` を返します。

Scheme プロシージャ: **add-hook!** hook proc \[append\_p\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dhook_0021)

C 関数: **scm\_add\_hook\_x** (hook、proc、append\_p) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fadd_005fhook_005fx)

プロシージャ proc をフック hook に追加します。append\_p が true の場合、プロシージャは末尾に追加され、そうでない場合は先頭に追加されます。このプロシージャの戻り値は指定されていません。

Scheme プロシージャ: **remove-hook!** hook proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-remove_002dhook_0021)

C 関数: **scm\_remove\_hook\_x** (hook, proc) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fremove_005fhook_005fx)

プロシージャ proc をフック hook から削除してください。このプロシージャの戻り値は指定されていません。

Scheme Procedure: **reset-hook!** hook [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reset_002dhook_0021)

C 関数: **scm\_reset\_hook\_x** (フック) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005freset_005fhook_005fx)

フックフックからすべてのプロシージャを削除します。このプロシージャの戻り値は指定されていません。

Scheme 手順: **hook->list** hook [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hook_002d_003elist)

C 関数: **scm\_hook\_to\_list** (フック) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhook_005fto_005flist)

フックの手順リストをリストに変換します。

Scheme Procedure: **run-hook** hook arg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-run_002dhook)

C 関数: **scm\_run\_hook** (hook, args) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005frun_005fhook)

フック hook から引数 arg ... までのすべてのプロシージャを適用します。プロシージャの適用順序は、最初から最後です。このプロシージャの戻り値は指定されていません。

C コードにおいて、フック オブジェクトと、そのフックに対する適切な形式の引数リストが確実に存在する場合は、`scm_c_run_hook` を使用することもできます。これは `scm_run_hook` と同一ですが、型チェックは行いません。

C 関数: `void` **scm\_c\_run\_hook** `(SCM フック、SCM 引数)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005frun_005fhook)

`scm_run_hook` と同じですが、hook が実際にフック オブジェクトであること、および args がフックの引数の数に一致する整形式のリストであることを確認するための型チェックは行われません。

C言語の場合、`SCM_HOOKP`は`scm_hook_p`よりも高速な代替手段です。

C マクロ: `int` **SCM\_HOOKP** `(x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fHOOKP)

xがSchemeレベルのフックであれば1を返し、そうでなければ0を返します。

* * *

次へ: [ガベージコレクションのフック](https://doc.guix.gnu.org/guile/latest/en/guile.html#GC-Hooks)、前: [フックリファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hook-Reference)、上: [フック](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.6.3 Cコード用のフック。 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks-For-C-Code_002e)

既に説明したフックは、Schemeレベルのプロシージャによって設定されることを想定しています。これに加えて、Guileライブラリは、C言語で実装された関数によって設定されるフックの作成と操作のための独立したインターフェースセットを提供します。

ここでの本来の目的は、ガベージコレクションのさまざまな段階で安全に呼び出せるフックを提供することでした。Scheme レベルのフックは、実行時にメモリ割り当てが必要になる場合があり、その結果ガベージコレクションが再帰的に呼び出されてしまうため、この目的には適していません。しかし、C 関数のみを登録したい場合、これらのフックは Scheme レベルのフックよりも扱いやすいという利点もあります。したがって、コードが主にそのような処理を必要とする場合は、このインターフェースを使用することをお勧めします。

Cフックを作成するには、`scm_t_c_hook`型の構造体のストレージを割り当て、次に`scm_c_hook_init`を使用してそれを初期化する必要があります。

C 型: **scm\_t\_c\_hook** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fc_005fhook)

C言語のフック用のデータ型。この型の内部構造は不透明として扱う必要があります。

C 列挙型: **scm\_t\_c\_hook\_type** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fc_005fhook_005ftype )

考えられるフックの種類を列挙します。

`SCM_C_HOOK_NORMAL` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fC_005fHOOK_005fNORMAL)

登録されているすべての関数が常に呼び出されるフックの種類。

`SCM_C_HOOK_OR` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fC_005fHOOK_005fOR)

登録された関数のシーケンスが、そのうちの1つがC true（NULL以外のポインタ）を返すまでのみ呼び出されるフックの種類。

`SCM_C_HOOK_AND` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fC_005fHOOK_005fAND)

登録された関数のシーケンスが、そのうちの1つがC false（NULLポインタ）を返すまでのみ呼び出されるフックの種類。

C 関数: `void` **scm\_c\_hook\_init** `(scm_t_c_hook *hook, void *hook_data, scm_t_c_hook_type type)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fhook_005finit)

hook が指すメモリで C フックを初期化します。type は `scm_t_c_hook_type` 列挙値のいずれかである必要があり、フック関数の呼び出し方法を制御します。hook\_data はクロージャパラメータであり、登録されているすべてのフック関数が呼び出されたときに渡されます。

CフックにC関数を追加または削除するには、`scm_c_hook_add`または`scm_c_hook_remove`を使用します。フック関数は、それぞれ次の3つの`void *`パラメータを受け取る必要があります。

フック_データ

`scm_c_hook_init`によってフックが初期化された際に指定されたフックのクロージャデータ。

関数_データ

`scm_c_hook_add`によってその関数がフックに登録された時点で指定された関数クロージャデータ。

データ

フックを実行する`scm_c_hook_run`呼び出しによって指定された呼び出し終了データ。

C 型: **scm\_t\_c\_hook\_function** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fc_005fhook_005ffunction)

C言語のフック関数の関数型：3つの`void *`パラメータを受け取り、`void *`の結果を返します。

C 関数: `void` **scm\_c\_hook\_add** `(scm_t_c_hook *hook, scm_t_c_hook_function func, void *func_data, int appendp)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fhook_005fadd)

関数クロージャデータ func\_data を持つ関数 func を C フック hook に追加します。appendp がゼロ以外の場合は、新しい関数はフックの関数リストに追加され、そうでない場合は先頭に追加されます。

C 関数: `void` **scm\_c\_hook\_remove** `(scm_t_c_hook *hook, scm_t_c_hook_function func, void *func_data)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fhook_005fremove)

Cフックフックから、関数クロージャデータfunc\_dataを持つ関数funcを削除します。`scm_c_hook_remove`は、funcとfunc\_dataの両方をチェックし、同じ関数が異なるクロージャデータで複数回登録されることを可能にします。

最後に、Cフックを呼び出すには、フックと今回の実行の呼び出しクロージャデータを指定して`scm_c_hook_run`関数を呼び出します。

C 関数: `void *` **scm\_c\_hook\_run** `(scm_t_c_hook *hook, void *data)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fc_005fhook_005frun)

Cフックを実行すると、フックはクロージャデータデータを呼び出します。フックタイプ`SCM_C_HOOK_OR`と`SCM_C_HOOK_AND`のバリエーションを除き、`scm_c_hook_run`はフックに登録された関数を順番に呼び出し、フックのクロージャデータ、各関数のクロージャデータ、および呼び出しクロージャデータを渡します。

`scm_c_hook_run` の戻り値は、最後に呼び出された関数の戻り値です。

* * *

次へ: [Guile REPL へのフック](https://doc.guix.gnu.org/guile/latest/en/guile.html#REPL-Hooks)、前: [C コード用フック](https://doc.guix.gnu.org/guile/latest/en/guile.html#C-Hooks)、上: [フック](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.6.4 ガベージコレクションのフック [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks-for-Garbage-Collection)

Guileがガベージコレクションを実行するたびに、以下のフックが示された順序で呼び出されます。

C フック: **scm\_before\_gc\_c\_hook** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbefore_005fgc_005fc_005fhook)

ガベージコレクションの開始時、`scm_gc_running_p`を1に設定した後、GCクリティカルセクションに入る前に呼び出されるCフック。

`scm_block_gc`がゼロ以外の値であるためにガベージコレクションがブロックされた場合、このフックが呼び出された直後にGCは早期に終了し、それ以降のフックは呼び出されません。

C フック: **scm\_before\_mark\_c\_hook** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbefore_005fmark_005fc_005fhook)

GCスレッドがクリティカルセクションに入った後、ガベージコレクションのマークフェーズを開始する前に呼び出されるCフック。

C フック: **scm\_before\_sweep\_c\_hook** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fbefore_005fsweep_005fc_005fhook)

Cフックは、ガベージコレクションのスイープフェーズを開始する前に呼び出されます。これは、マークフェーズの終了時と同じです。マークとスイープの間には他に何も起こらないためです。

C フック: **scm\_after\_sweep\_c\_hook** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fafter_005fsweep_005fc_005fhook)

Cフックは、ガベージコレクションのスイープフェーズの終了後に呼び出されますが、GCスレッドはまだクリティカルセクション内にあります。

C フック: **scm\_after\_gc\_c\_hook** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fafter_005fgc_005fc_005fhook)

C フックは、GC スレッドがクリティカル セクションを抜けた後、ガベージ コレクションの最終段階で呼び出されます。

Scheme フック: **after-gc-hook** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-after_002dgc_002dhook)

引数0のSchemeフック。このフックは、GCが完了し、ガベージコレクション中に延期されたその他のイベントが処理された直後に非同期で実行されます（[非同期割り込み](https://doc.guix.gnu.org/guile/latest/en/guile.html#Asyncs)を参照）。(C言語からも`scm_after_gc_hook`という名前でアクセスできます。)

ここにリストされているすべての C フックはタイプ `SCM_C_HOOK_NORMAL` であり、フッククロージャデータ NULL で初期化され、呼び出しクロージャデータ NULL で `scm_c_hook_run` によって呼び出されます。

Scheme フック `after-gc-hook` は、ガーディアン ([ガーディアン](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guardians) を参照) と組み合わせて使用すると特に便利です。通常、ガーディアンを使用している場合は、ガベージ コレクション後にガーディアンを呼び出して、ガーディアンに追加されたオブジェクトが回収されたかどうかを確認する必要があります。この呼び出しを実行するサンクを `after-gc-hook` に追加することで、ガーディアンがすべてのガベージ コレクション サイクル後にテストされることを保証できます。

* * *

前へ: [ガベージコレクションのフック](https://doc.guix.gnu.org/guile/latest/en/guile.html#GC-Hooks)、上へ: [フック](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.9.6.5 Guile REPL へのフック [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hooks-into-the-Guile-REPL)

* * *

次へ: [プログラム実行フローの制御](https://doc.guix.gnu.org/guile/latest/en/guile.html#Control-Mechanisms)、前: [汎用ユーティリティ関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Utility-Functions)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
