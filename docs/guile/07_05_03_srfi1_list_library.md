#### 7.5.3 SRFI-1 - リストライブラリ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-_002d-List-library)

SRFI-1で定義されているリストライブラリには、リストやペアの構築、検査、分解、操作を行うための多くの便利なリスト処理手順が含まれています。

SRFI-1 では、R5RS に既に含まれている、したがって Guile コアライブラリでサポートされているいくつかの手順も定義されているため、SRFI-1 ドキュメントに記載されているリストおよびペアの手順の一部は、このセクションには記載されていない場合があります。したがって、特定のリスト/ペア処理手順を探す場合は、[リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lists) および [ペア](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pairs) のセクションも参照してください。

* [コンストラクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Constructors)
* [述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Predicates)
* [セレクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Selectors)
* [長さ、追加、連結など](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Length-Append-etc)
* [折りたたみ、展開、マップ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Fold-and-Map)
* [フィルタリングとパーティショニング](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Filtering-and-Partitioning)
* [検索中](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Searching)
* [削除中](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Deleting)
* [関連リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Association-Lists)
* [リストに対する集合演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Set-Operations)

* * *

次へ: [述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Predicates)、上へ: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.3.1 コンストラクタ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Constructors)

以下のいずれかの手続きを呼び出すことで、新しいリストを作成できます。

Scheme Procedure: **xcons** da [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-xcons)

`cons` と同様だが、引数の順序が入れ替わっている。主に高階プロシージャに渡す場合に便利。

Scheme プロシージャ: **list-tabulate** n init-proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002dtabulate)

n個の要素からなるリストを返します。各リスト要素は、対応するリストインデックスにinit-procプロシージャを適用することによって生成されます。init-procがインデックスに適用される順序は指定されません。

Scheme Procedure: **list-copy** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002dcopy-1)

リストlstの要素を含む新しいリストを返します。

この関数は、コア関数である `list-copy` ([リストコンストラクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#List-Constructors) を参照) とは異なり、不適切なリストも受け入れます。また、lst がペアでない場合は、不適切なリストの最後の要素として扱われ、そのまま返されます。

スキーム手順: **circular-list** elt1 elt2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-circular_002dlist)

指定された引数 elt1 elt2 … を含む循環リストを返します。

スキーム手順: **iota** カウント \[開始ステップ\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-iota)

開始値から始めて毎回ステップを追加しながらカウント数を含むリストを返します。デフォルトの開始値は 0、デフォルトのステップ値は 1 です。たとえば、

(イオタ 6) ⇒ (0 1 2 3 4 5)
(イオタ 4 2.5 -2) ⇒ (2.5 0.5 -1.5 -3.5)

この関数は、APL言語における対応する基本要素からその名前を取っています。

* * *

次へ: [セレクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Selectors)、前へ: [コンストラクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Constructors)、上へ: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.3.2 述語 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Predicates)

このセクションの手順では、リストの特定の特性をテストします。

Scheme Procedure: **proper-list?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-proper_002dlist_003f)

objが適切なリストであれば`#t`を返し、そうでなければ`#f`を返します。これはコアの`list?`と同じです（[リスト述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#List-Predicates)を参照）。

適切なリストとは、通常の方法で空リスト `()` で終わるリストのことです。空リスト `()` 自体も適切なリストです。

(proper-list? '(1 2 3)) ⇒ #t
(proper-list? '()) ⇒ #t

スキーム手順: **circular-list?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-circular_002dlist_003f)

objが循環リストの場合は`#t`を返し、そうでない場合は`#f`を返します。

循環リストとは、ある時点で `cdr` がリスト内の前のペア（先頭または後続のポイント）を参照するリストのことです。そのため、`cdr` をたどっていくと、終わりがない円を描くようにリストを巡ることになります。

(define x (list 1 2 3 4))
(set-cdr! (last-pair x) (cddr x))
x ⇒ (1 2 3 4 3 4 3 4 ...)
(循環リスト? x) ⇒ #t

Scheme Procedure: **dotted-list?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-dotted_002dlist_003f)

objがドットリストの場合は`#t`を返し、それ以外の場合は`#f`を返します。

ドット付きリストとは、最後のペアの `cdr` が空リスト `()` ではないリストのことです。ペアでないオブジェクトも、長さがゼロのドット付きリストとみなされます。

(ドットリスト? '(1 2 . 3)) ⇒ #t
(ドットリスト? 99) ⇒ #t

Scheme オブジェクトは、上記の 3 つのテスト `proper-list?`、`circular-list?`、`dotted-list?` のうち、必ず 1 つだけ合格することに注意してください。リストでないものは `dotted-list?` に合格し、有限リストは `proper-list?` または `dotted-list?` に合格し、無限リストは `circular-list?` に合格します。

  

Scheme Procedure: **null-list?** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-null_002dlist_003f)

lst が空のリスト `()` の場合は `#t` を返し、それ以外の場合は `#f` を返します。lst に適切なリストまたは循環リスト以外のものが渡された場合は、エラーが通知されます。この手順は、ドット付きリストが許可されていない状況でリストの末尾を確認する場合に推奨されます。

Scheme Procedure: **not-pair?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-not_002dpair_003f)

obj がペアでない場合は `#t` を返し、そうでない場合は `#f` を返します。これは `(not (pair? obj))` という省略記法で、ドット付きリストが許可されているコンテキストでリストの末尾チェックに使用されることを想定しています。

スキーム手順: **list=** elt= list1 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_003d)

引数リストがすべて等しい場合は `#t` を返し、そうでない場合は `#f` を返します。リストの等価性は、すべてのリストの長さが同じで、対応する要素が等価述語 elt= の意味で等しいかどうかをテストすることによって判定されます。リストが指定されていない場合、またはリストが 1 つだけの場合は、`#t` が返されます。

* * *

次へ: [長さ、追加、連結など](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Length-Append-etc)、前: [述語](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Predicates)、上: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.3.3 セレクタ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Selectors)

スキーム手順: **最初の**ペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-first)

スキーム手順: **2番目の**ペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-second)

スキーム手順: **3番目**のペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-third)

スキーム手順: **4番目**のペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fourth)

スキーム手順: **5番目**のペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fifth)

スキーム手順: **6番目**のペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-sixth)

スキーム手順: **7番目**のペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-seventh)

スキーム手順: **8番目**のペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-eighth)

スキーム手順: **9番目**のペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ninth)

スキーム手順: **10番目**のペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-tenth)

これらは `car`、`cadr`、`caddr`、… の同義語です。

スキーム手順: **car+cdr** ペア [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-car_002bcdr)

ペアのCARとCDRの2つの値を返します。

([car+cdr](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-car_002bcdr) '(0 1 2 3))
⇒
0
（1 2 3）

スキーム手順: **take** lst i [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-take)

Scheme Procedure: **take!** lst i [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-take_0021)

lstの最初のi個の要素を含むリストを返します。

`take!` は、結果を生成するために引数リスト lst の構造を変更する場合があります。

スキーム手順: **drop** lst i [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-drop)

lst の最初の i 個の要素を除くすべての要素を含むリストを返します。

スキーム手順: **take-right** lst i [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-take_002dright)

lst の最後の i 個の要素を含むリストを返します。返されるリストは lst と共通の末尾を持ちます。

スキーム手順: **drop-right** lst i [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-drop_002dright)

スキーム手順: **drop-right!** lst i [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-drop_002dright_0021)

lst の最後の i 個の要素を除くすべての要素を含むリストを返します。

`drop-right` は、i がゼロの場合でも常に新しいリストを返します。`drop-right!` は、結果を生成するために引数リスト lst の構造を変更する場合があります。

スキーム手順: **split-at** lst i [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-split_002dat)

スキーム手順: **split-at!** lst i [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-split_002dat_0021)

リストlstの最初のi個の要素を含むリストと、残りの要素を含むリストの2つの値を返します。

`split-at!` は、結果を生成するために引数リスト lst の構造を変更する場合があります。

Scheme Procedure: **last** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-last)

空でない有限リスト lst の最後の要素を返します。

* * *

次へ: [折りたたみ、展開、マップ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Fold-and-Map)、前: [セレクタ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Selectors)、上: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.3.4 長さ、追加、連結など [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Length_002c-Append_002c-Concatenate_002c-etc_002e)

スキーム手順: **length+** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-length_002b)

引数リスト lst の長さを返します。lst が循環リストの場合は、`#f` が返されます。

Scheme手順: **concatenate** リストのリスト[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-concatenate)

Scheme 手順: **concatenate!** リストのリスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-concatenate_0021)

リストのリストに含まれるすべてのリストを連結して、リストを作成します。

`concatenate!` は、結果を生成するために、指定されたリストの構造を変更する場合があります。

`concatenate` は `(apply append list-of-lists)` と同じです。これは、Scheme の実装によっては関数が受け取る引数の数に制限があり、`apply` がその制限を超える可能性があるためです。Guile にはそのような制限はありません。

Scheme Procedure: **append-reverse** rev-head tail [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-append_002dreverse)

Scheme Procedure: **append-reverse!** rev-head tail [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-append_002dreverse_0021)

rev-headを反転し、それにtailを追加して結果を返します。これは`(append (reverse rev-head) tail)`と同等ですが、実装がより効率的です。

(append-reverse '(1 2 3) '(4 5 6)) ⇒ (3 2 1 4 5 6)

`append-reverse!` は結果を生成するために rev-head を変更する場合があります。

スキーム手順: **zip** lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-zip)

引数として指定されたリストのうち、最も短いものと同じ長さのリストを返します。各要素はリストです。最初のリストには引数リストの最初の要素が含まれ、2番目のリストには2番目の要素が含まれる、といった具合です。

スキーム手順: **unzip1** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unzip1)

Scheme手順: **unzip2** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unzip2)

Scheme手順: **unzip3** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unzip3)

スキーム手順: **unzip4** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unzip4)

Scheme 手順: **unzip5** lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unzip5)

`unzip1`はリストのリストを受け取り、各リストの最初の要素を含むリストを返します。`unzip2`は2つのリストを返し、1つ目は各リストの最初の要素を、2つ目は各リストの2番目の要素を含みます。以下同様です。

スキーム手順: **count** pred lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-count)

指定されたリストの要素に対して pred 関数を呼び出したときに、true を返す回数を返します。

pred 関数は、N 個のパラメータ `(pred elem1 … elemN )` を引数として呼び出されます。各パラメータは、対応するリストから取得されます。最初の呼び出しでは各リストの最初の要素が使用され、2 回目の呼び出しでは各リストの 2 番目の要素が使用される、といった具合です。

最短リストの末尾に達した時点でカウントを停止します。少なくとも1つのリストは循環リストであってはなりません。

* * *

次へ: [フィルタリングとパーティショニング](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Filtering-and-Partitioning)、前: [長さ、追加、連結など](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Length-Append-etc)、上: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 7.5.3.5 折りたたみ、展開、マップ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fold_002c-Unfold-_0026-Map)

Scheme手順: **fold** proc init lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fold)

Scheme手順: **fold-right** proc init lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-fold_002dright)

lst1、lst2…の要素にprocを適用して結果を作成し、その結果を返します。

各proc呼び出しは`(proc elem1 elem2 … previous)`という形式で、elem1はlst1から、elem2はlst2から、といった具合に続きます。previousは前回のproc呼び出しの戻り値、または最初の呼び出しで指定されたinitです。いずれかのリストが空の場合は、initのみが返されます。

`fold` はリストの要素を最初から最後まで順に処理します。以下はリストの反転と、その際に実行される呼び出しを示しています。

(fold cons '() '(1 2 3))

(cons 1 '())
（cons 2 '(1)）
(cons 3 '(2 1)
⇒ (3 2 1)

`fold-right` はリストの要素を最後から最初、つまり右から順に処理します。したがって、たとえば次のコードは最長の文字列を見つけ、同じ長さの文字列の中で最後を見つけます。

(fold-right (lambda (str prev)
(if (> (string-length str) (string-length prev))
str
前へ）
「」
'("x" "abc" "xyz" "jk"))
⇒ 「xyz」

lst1、lst2、…の長さが異なる場合、`fold`は最短の長さの末尾に達した時点で停止し、`fold-right`は最短の長さの最後の要素から開始します。つまり、他のlstでは最短の長さを超える要素は無視されます。少なくとも1つのlstは非循環である必要があります。

処理順序が重要でない場合、またはどちらの順序でも構わない場合は、`fold` の方が `fold-right` よりも若干効率的であるため、`fold` を優先すべきです。

`fold` が反復処理から結果を構築する方法は非常に汎用的で、`map` や `filter` などの他の反復処理よりも多くのことができます。たとえば、次のコードはリストから隣接する重複要素を削除します。

(define (delete-adjacent-duplicates lst)
(fold-right (lambda (elem ret)
(if (equal? elem (first ret))
退役
(cons elem ret)))
(リスト (最後のリスト))
lst))
(delete-adjacent-duplicates '(1 2 3 3 4 4 4 5))
⇒ (1 2 3 4 5)

もちろん、`for-each` と結果を構築するための変数を使えば同様のことはできますが、自己完結型のプロシージャは複数のコンテキストで再利用できます。一方、`for-each` の場合は毎回記述する必要があります。

Scheme Procedure: **pair-fold** proc init lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pair_002dfold)

Scheme 手順: **pair-fold-right** proc init lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pair_002dfold_002dright)

`fold` および `fold-right` と同じですが、リストの要素ではなく、リストのペアに対して proc を適用します。

Scheme プロシージャ: **reduce** proc default lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reduce)

Scheme プロシージャ: **reduce-right** proc default lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-reduce_002dright)

`reduce`は`fold`の派生形であり、procへの最初の呼び出しは、1つの要素と指定された初期値ではなく、lstからの2つの要素に対して行われます。

lstが空の場合、`reduce`はデフォルト値を返します（デフォルト値の唯一の用途です）。lstに要素が1つしかない場合は、それが戻り値になります。それ以外の場合は、lstの要素に対してprocが呼び出されます。

各proc呼び出しは`(proc elem previous)`という形式で行われ、elemはlst（lstの2番目以降の要素）から取得され、previousは前回のproc呼び出しの戻り値です。lstの最初の要素は、最初のproc呼び出しのpreviousです。

例えば、以下のコードは数値のリストを追加するもので、`+` への呼び出しが表示されます。（もちろん、`+` は複数の引数を受け付け、`apply` を使ってリストを直接追加することもできます。）

(reduce + 0 '(5 6 7)) ⇒ 18

(+ 6 5) ⇒ 11
(+ 7 11) ⇒ 18

`reduce` は、初期値が「識別子」、つまり proc で結果が変わらない値である場合に `fold` の代わりに使用できます。この場合、`(+ 5 0)` は単に 5 なので、0 は識別子です。`reduce` は、この不要な呼び出しを回避します。

`reduce-right` は `fold-right` の類似したバリエーションで、lst の末尾 (つまり右端) から処理を行います。lst の最後の要素は、proc を最初に呼び出したときの前の要素であり、elem の値は最後から 2 番目の要素から始まります。

処理順序が重要でない場合、またはどちらの順序でも構わない場合は、`reduce` の方が `reduce-right` より若干効率的なので、`reduce` を優先すべきです。

Scheme 手順: **unfold** pfg seed \[tail-gen\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unfold)

`unfold`は以下のように定義されます。

([unfold](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unfold) pfg seed) [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d)
(もし(pシード)(テールジェンシード)の場合)
([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) (f seed)
([unfold](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unfold) pfg (g seed))))

p

展開を停止するタイミングを決定します。

f

各シード値を対応するリスト要素にマッピングします。

g

各シード値を次のシード値にマッピングします。

シード

展開状態の値。

テールジェン

リストの末尾を作成します。デフォルトは `(lambda (x) '())` です。

gは一連のシード値を生成し、fはそれらをリスト要素にマッピングします。これらの要素は左から右の順にリストに格納され、pは展開を停止するタイミングを示します。

スキーム手順: **unfold-right** pfg seed \[tail\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-unfold_002dright)

以下のループを使用してリストを作成します。

(let lp ((seed seed) (lis tail))
(もし(pシード)リスの場合)
(lp (g シード)
([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) (f seed) lis))))

p

展開を停止するタイミングを決定します。

f

各シード値を対応するリスト要素にマッピングします。

g

各シード値を次のシード値にマッピングします。

シード

展開状態の値。

しっぽ

リストの末尾。デフォルトは「'()」です。

Scheme Procedure: **map** f lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-map-1)

リスト lst1、lst2、… に対してプロシージャをマッピングし、プロシージャの適用結果を含むリストを返します。引数リストの長さが異なる可能性があるため、このプロシージャは R5RS に対して拡張されています。結果リストの長さは、引数リストの中で最も短いものと同じになります。f がリスト要素に適用される順序は指定されていません。

スキーム手順: **for-each** f lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-for_002deach-1)

リストlst1、lst2、…の各対応する要素のペアに、プロシージャfを適用します。戻り値は指定されていません。引数リストの長さが異なる場合があるため、このプロシージャはR5RSに対して拡張されています。fの呼び出し回数は、引数リストの長さによって決まります。fはリスト要素に左から右の順に適用されます。

Scheme Procedure: **append-map** f lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-append_002dmap)

Scheme 手順: **append-map!** f lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-append_002dmap_0021)

同等

([apply](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-apply) [append](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-append) (map f clist1 clist2 [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)))

そして

([apply](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-apply) [append!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-append_0021) (map f clist1 clist2 [...](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002e_002e_002e)))

`map` 関数と同様に、リストの要素に対して `map f` を実行します。ただし、最終的な結果を作成するために、各処理の結果が連結されます。`append-map` は `append` を使用して結果を連結し、`append-map!` は `append!` を使用して連結します。

fの様々な適用が行われる動的な順序は指定されていない。

Scheme Procedure: **map!** f lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-map_0021)

`map` の線形更新バリアント – `map!` は、結果リストを構築するために lst1 の cons セルを変更することが許可されていますが、必須ではありません。

f の様々な適用が行われる動的な順序は指定されていません。n 項の場合、lst2、lst3、…は少なくとも lst1 と同じ数の要素を持つ必要があります。

スキーム手順: **pair-for-each** f lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-pair_002dfor_002deach)

`for-each`と同様ですが、引数リストの要素ではなく、引数リストを構成するペアに対して手続きfを適用します。戻り値は指定されていません。

スキーム手順: **filter-map** f lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-filter_002dmap)

`map`と同様ですが、fを適用した結果のうち、真であるものだけが結果リストに保存されます。

* * *

次へ: [検索](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Searching)、前へ: [折りたたみ、展開、マップ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Fold-and-Map)、上へ: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.3.6 フィルタリングとパーティショニング [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Filtering-and-Partitioning)

フィルタリングとは、特定の条件を満たすリストの要素をすべて収集することです。リストのパーティショニングとは、条件を満たす要素を含むグループと、条件を満たさない要素を含むグループの2つのグループにリストの要素を分けることです。

`filter` および `filter!` 関数は Guile コアに実装されています。[リストの変更](https://doc.guix.gnu.org/guile/latest/en/guile.html#List-Modification) を参照してください。

Scheme Procedure: **partition** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-partition)

Scheme Procedure: **partition!** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-partition_0021)

lst を述語 pred を満たす要素と満たさない要素に分割します。

戻り値は 2 つの値です ([複数の値の返却と受け入れ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Multiple-Values) を参照)。1 つ目は、pred を満たす lst のすべての要素のリスト、2 つ目は、pred を満たさない要素のリストです。

結果リスト内の要素は lst と同じ順序ですが、リスト要素に対して `(pred elem)` の呼び出しが行われる順序は指定されていません。

`partition` は lst を変更しませんが、返されるリストのいずれかが lst と末尾を共有する場合があります。`partition!` は、返されるリストを構築するために lst を変更する場合があります。

Scheme Procedure: **remove** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-remove)

Scheme Procedure: **remove!** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-remove_0021)

述語 pred を満たさない、リスト lst のすべての要素を含むリストを返します。結果リストの要素の順序は lst と同じです。pred がリスト要素に適用される順序は指定されません。

`remove!` は入力リストの構造を変更するために使用できますが、必須ではありません。

* * *

次へ: [削除](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Deleting)、前: [フィルタリングとパーティショニング](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Filtering-and-Partitioning)、上: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.3.7 検索 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Searching)

リスト内の要素を検索する手順は、検索対象となる要素を決定するための述語または比較オブジェクトのいずれかを受け入れる。

Scheme手順: **find** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-find)

述語 pred を満たす lst の最初の要素を返し、そのような要素が見つからない場合は `#f` を返します。

Scheme Procedure: **find-tail** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-find_002dtail)

CARが述語predを満たす最初のlstのペアを返し、そのような要素が見つからない場合は`#f`を返します。

Scheme Procedure: **take-while** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-take_002dwhile)

Scheme Procedure: **take-while!** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-take_002dwhile_0021)

要素がすべて述語 pred を満たす、lst の最長の先頭接頭辞を返します。

`take-while!` は、結果を生成しながら入力リストを変更するために使用できますが、必須ではありません。

Scheme Procedure: **drop-while** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-drop_002dwhile)

要素がすべて述語 pred を満たす lst の最長の先頭接頭辞を削除します。

Scheme Procedure: **span** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-span)

Scheme Procedure: **span!** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-span_0021)

Scheme Procedure: **break** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-break-2)

Scheme Procedure: **break!** pred lst [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-break_0021)

`span` はリスト lst を、要素がすべて述語 pred を満たす最長の先頭接頭辞と、残りの末尾に分割します。`break` は述語の意味を反転させます。

`span!` と `break!` は許可されていますが、結果を生成するために入力リスト lst の構造を変更するために必須ではありません。

`break` という名前は、`while` によって確立された `break` バインディングと競合することに注意してください ([反復メカニズム](https://doc.guix.gnu.org/guile/latest/en/guile.html#while-do) を参照)。`while` ループ内で `break` を使用したいアプリケーションは、別の名前で新しい定義を作成する必要があります。

スキーム手順: **any** pred lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-any)

lst1、lst2、…のいずれかの要素セットがpredを満たすかどうかをテストします。満たす場合は、pred呼び出しが成功したときの戻り値が戻り値となり、満たさない場合は`#f`が戻り値となります。

リスト引数が n 個ある場合、pred は n 個の引数を取る述語でなければなりません。各 pred 呼び出しは、各リストから要素を 1 つずつ取る `(pred elem1 elem2 … )` です。呼び出しは、リストの最初の要素、2 番目の要素などに対して順次行われ、pred が `#f` 以外の値を返すか、最短リストの末尾に達した時点で停止します。

最後の要素セット（つまり、最短リストの末尾に達したとき）に対する pred 呼び出しは、その時点に達した場合、末尾呼び出しになります。

スキーム手順: **every** pred lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-every)

lst1、lst2、…の各要素セットがpredを満たすかどうかをテストします。満たす場合は、最後のpred呼び出しからの戻り値が戻り値となり、満たさない場合は`#f`が戻り値となります。

リスト引数が n 個ある場合、pred は n 個の引数を取る述語でなければなりません。各 pred 呼び出しは、各リストから要素を 1 つずつ取る `(pred elem1 elem2 …)` という形式です。呼び出しは、リストの最初の要素、2 番目の要素などに対して順次行われ、pred が `#f` を返すか、いずれかのリストの末尾に達した時点で停止します。

最後の要素セットに対する pred 呼び出し (つまり、最短リストの末尾に達したとき) は末尾呼び出しです。

lst1、lst2、…のいずれかが空の場合、predへの呼び出しは行われず、戻り値は`#t`になります。

スキーム手順: **list-index** pred lst1 lst2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-list_002dindex)

予測を満たす、lst1、lst2、…の各要素から1つずつ選ばれた最初の要素セットのインデックスを返します。

pred は `(elem1 elem2 …)` として呼び出されます。最短リストの末尾に到達すると検索が停止します。最初の要素セットの場合、戻りインデックスは 0 から始まります。要素セットが通過しない場合は、戻り値は `#f` になります。

(リストインデックスが奇数か？ '(2 4 6 9)) ⇒ 3
(list-index = '(1 2 3) '(3 1 2)) ⇒ #f

スキーム手順: **member** x lst \[=\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-member-1)

lst のサブリストのうち、CAR が x と等しい最初のサブリストを返します。x が lst に含まれていない場合は、`#f` を返します。

等価性は `equal?` または等価述語 \= によって判定されます。 \= は `(= x elem)` と呼ばれ、つまり、最初に与えられた x が付きます。たとえば、5 より大きい最初の要素を見つけるには、

(メンバー 5 '(3 5 1 7 2 9) <) ⇒ (7 2 9)

このバージョンの `member` は、等価述語を受け入れることでコアの `member` ([リスト検索](https://doc.guix.gnu.org/guile/latest/en/guile.html#List-Searching) を参照) を拡張しています。

* * *

次へ: [関連リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Association-Lists)、前へ: [検索](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Searching)、上へ: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.3.8 削除 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Deleting)

Scheme Procedure: **delete** x lst \[=\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-delete-1)

Scheme Procedure: **delete!** x lst \[=\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-delete_0021-1)

lst の要素のうち、x と等しい要素を削除したリストを返します。返される要素は、lst の要素と同じ順序になります。

等価性は、述語「=」によって判定されます。述語が指定されていない場合は、「equal?」によって判定されます。等価性判定は各要素に対して一度だけ行われますが、要素に対する判定の順序は規定されていません。

等価性判定は常に `(= x elem)` で行われます。つまり、指定された x が最初に来ます。これは、例えば 5 より大きい要素は `(delete 5 lst <)` で削除できることを意味します。

`delete` は lst を変更しませんが、戻り値は lst と共通の末尾を持つ可能性があります。`delete!` は、戻り値を構築するために lst の構造を変更する場合があります。

これらの関数は、コア関数である `delete` および `delete!` ([リストの変更](https://doc.guix.gnu.org/guile/latest/en/guile.html#List-Modification) を参照) を拡張し、等価述語を受け入れるようにしています。リストから複数の要素を削除するには、`lset-difference` ([リストに対するセット操作](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Set-Operations) を参照) も参照してください。

スキーム手順: **delete-duplicates** lst \[=\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-delete_002dduplicates)

Scheme Procedure: **delete-duplicates!** lst \[=\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-delete_002dduplicates_0021)

lst の要素を重複なく含んだリストを返します。

要素が同じ場合、リストの最初の要素のみが保持されます。同じ要素はリスト内のどこにあっても構いません。隣接している必要はありません。返されるリストには、保持された要素がリスト内の順序と同じ順序で含まれます。

等価性は、述語 \= によって判定されます。指定されていない場合は `equal?` が用いられます。`(= xy)` という呼び出しは、lst において要素 x が y より前にある場合に行われます。各組み合わせに対して呼び出しは最大 1 回行われますが、要素間の呼び出しの順序は規定されていません。

`delete-duplicates` は lst を変更しませんが、戻り値は lst と共通の末尾を持つ可能性があります。`delete-duplicates!` は、戻り値を構築するために lst の構造を変更する場合があります。

最悪の場合、これはO(N^2)のアルゴリズムになります。なぜなら、各要素をその前のすべての要素と比較する必要があるからです。長いリストの場合は、ソートしてから隣接する要素のみを比較する方が効率的です。

* * *

次へ: [リストに対する集合演算](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Set-Operations)、前へ: [削除](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Deleting)、上へ: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.3.9 関連付けリスト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Association-Lists-2)

関連付けリストについては、[関連付けリスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Association-Lists)のセクションで詳しく説明されています。本セクションでは、SRFI-1で定義されている関連付けリストを扱うための追加手順のみを説明します。

スキーム手順: **assoc** key alist \[=\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-assoc-1)

キーに一致する alist のペアを返します。これは、オプションの \= 比較手順を受け入れることで、コアの `assoc` ([Alist エントリの取得](https://doc.guix.gnu.org/guile/latest/en/guile.html#Retrieving-Alist-Entries) を参照) を拡張します。

デフォルトの比較は `equal?` です。\= パラメータが指定された場合は、`(= key alistcar)` と呼ばれます。つまり、指定されたターゲットキーが最初の引数となり、alist から取得した `car` が 2 番目の引数となります。

例えば、大文字小文字を区別しない文字列検索、

(assoc "yy" '(("XX" . 1) ("YY" . 2)) string-ci=?)
⇒ （"YY" . 2）

スキーム手順: **alist-cons** キー データ alist [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alist_002dcons)

新しい関連付けキーとデータをalistに追加し、結果を返します。これは以下と同等です。

([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) key datum) alist)

Guile コアの `acons` ([Adding or Setting Alist Entries](https://doc.guix.gnu.org/guile/latest/en/guile.html#Adding-or-Setting-Alist-Entries) を参照) も同じことを行います。

Scheme Procedure: **alist-copy** alist [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alist_002dcopy)

alist の新たに割り当てられたコピーを返します。つまり、リストの骨格とペアの両方がコピーされます。

Scheme Procedure: **alist-delete** key alist \[=\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alist_002ddelete)

Scheme Procedure: **alist-delete!** key alist \[=\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alist_002ddelete_0021)

alist の要素のうち、キーが deleted と等しい要素を除いたリストを返します。返される要素は、alist に含まれていた要素と同じ順序になります。

等価性は、述語 \= によって判定されます。指定されていない場合は `equal?` が使用されます。要素のテスト順序は指定されていませんが、各等価性呼び出しは `(= key alistkey)` の形式で行われます。つまり、指定された key パラメータが最初に、alist からの key が次に実行されます。これは、たとえば `(alist-delete 5 alist <)` を使用して、5 より大きいキーを持つすべての関連付けを削除できることを意味します。

`alist-delete`はalistを変更しませんが、戻り値はalistと共通の末尾を持つ可能性があります。`alist-delete!`は、戻り値を構築するためにalistのリスト構造を変更する場合があります。

* * *

前へ: [関連リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1-Association-Lists)、上へ: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.3.10 リストに対する集合演算 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Set-Operations-on-Lists)

リストはオブジェクトの集合を表すために使用できます。このセクションの手順は、そのようなリストを集合として扱います。

リストは大規模なセットを実装する効率的な方法ではないことに注意してください。ここで説明する手順は、m 個と n 個の要素を持つリストを操作する場合、通常 _mxn_ の時間がかかります。ツリー、ビットセット ([ビットベクトル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Bit-Vectors) を参照)、ハッシュテーブル ([ハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Tables) を参照) などの他のデータ構造の方が高速です。

これらの手続きはすべて、最初の引数として等価述語を受け取ります。この述語は、リストセット内のオブジェクトが同一であるかどうかをテストするために使用されます。この述語は、2つのリスト要素が`eq?`である場合、述語の下でもそれらが等しくなければならないという意味で、`eq?`（[Equality](https://doc.guix.gnu.org/guile/latest/en/guile.html#Equality)を参照）と整合していなければなりません。これは、与えられたオブジェクトがそれ自身と等しくなければならないことを意味します。

Scheme手順: **lset<=** \= list … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_003c_003d)

各リストが次のリストのサブセットである場合、`#t` を返します。つまり、list1 は list2 のサブセットであり、list2 は list3 のサブセットである、といった具合に、指定されたリストの数だけサブセットが続きます。リストが 1 つだけの場合、またはリストが指定されていない場合は、戻り値は `#t` になります。

リスト x は、x の各要素が y のいずれかの要素と等しい場合、y の部分集合となります。要素は、指定された \= 手順を使用して比較され、これは `(= xelem yelem)` と呼ばれます。

(lset<= eq?) ⇒ #t
(lset<= eqv? '(1 2 3) '(1)) ⇒ #f
(lset<= eqv? '(1 3 2) '(4 3 1 2)) ⇒ #t

Scheme手順: **lset=** \= list … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_003d)

引数リストがすべて等しい場合は、`#t` を返します。リスト1とリスト2、リスト2とリスト3、といったように、指定されたリストの数だけ比較されます。リストが1つだけの場合、またはリストが指定されていない場合は、戻り値は `#t` になります。

2つのリストxとyは、xの各要素がyのいずれかの要素と等しく、かつyの各要素がxのいずれかの要素と等しい場合に、等しいとみなされます。リスト内の要素の順序は関係ありません。要素の等価性は、指定された\=プロシージャ（`(= xelem yelem)`と呼び出されます）によって判定されますが、具体的にどの呼び出しが行われるかは明記されていません。

(lset= eq?) ⇒ #t
(lset= eqv? '(1 2 3) '(3 2 1)) ⇒ #t
(lset= string-ci=? '("a" "A" "b") '("B" "b" "a")) ⇒ #t

スキームプロシージャ: **lset-adjoin** \= list elem … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002dadjoin)

リストにまだ含まれていない指定された要素をリストに追加します。要素はリストの先頭に`cons`で追加されます（そのため、戻り値はリストと共通の末尾を持ちます）が、要素が追加される順序は指定されていません。

指定された \= 手続きは、`(= listelem elem)` と呼ばれる要素の比較に使用されます。つまり、 2 番目の引数は、指定された elem パラメータのいずれかです。

(lset-adjoin eqv? '(1 2 3) 4 1 5) ⇒ (5 4 1 2 3)

Scheme手順: **lset-union** \= list … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002dunion)

Scheme Procedure: **lset-union!** \= list … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002dunion_0021)

引数として指定されたリストセットの和集合を返します。結果は、リスト1とリスト2の和集合、次にリスト3との和集合、といったように、指定されたリストの数だけ和集合を取ることで構築されます。引数としてリストが1つだけの場合は、そのリスト自体が結果となり、引数としてリストがない場合は、空のリストが結果となります。

2つのリストxとyの和集合は、次のように生成されます。xが空の場合、結果はyになります。そうでない場合は、xを結果として、yの各要素（最初から最後まで）を検討します。結果に既に存在する要素と等しくないyの要素は、結果に`cons`で追加されます。

指定された \= プロシージャは、`(= relem yelem)` と呼ばれる要素の比較に使用されます。最初の引数はこれまでに蓄積された結果から、2番目の引数は結合されるリストから取得されます。ただし、具体的にどのような呼び出しが行われるかは、それ以外は指定されていません。

リスト1（または最初の空でないリスト）内の重複要素は保持されますが、後続のリスト内の重複要素は一度だけ追加されることに注意してください。

(lset-union eqv?) ⇒ ()
(lset-union eqv? '(1 2 3)) ⇒ (1 2 3)
(lset-union eqv? '(1 2 1 3) '(2 4 5) '(5)) ⇒ (5 4 1 2 1 3)

`lset-union` は指定されたリストを変更しませんが、結果は最初の空でないリストと末尾を共有する場合があります。`lset-union!` は指定されたすべてのリストを変更して結果を生成します。

スキーム手順: **lset-intersection** \= list1 list2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002dintersection)

Scheme Procedure: **lset-intersection!** \= list1 list2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002dintersection_0021)

list1と他の引数リストとの共通部分、つまりlist1の要素のうちlist2の要素もすべて含む要素などを返します。引数リストが1つだけの場合は、そのリストのみが返されます。

list1の要素が戻り値に含まれるための条件は、list2などの各要素のいずれかと等しいかどうかです。つまり、list1に2回出現する要素がlist2などに1回しか出現しない場合、戻り値にも2回出現することになります。戻り値の要素は、list1と同じ順序で並びます。

指定された \= プロシージャは、`(= elem1 elemN)` のように要素を比較するために使用されます。最初の引数はリスト1から、2番目の引数は後続のリストのいずれかから取得されます。ただし、どの呼び出しがどのような順序で行われるかは指定されていません。

(lset-intersection eqv? '(xy)) ⇒ (xy)
(lset-intersection eqv? '(1 2 3) '(4 3 2)) ⇒ (2 3)
(lset-intersection eqv? '(1 1 2 2) '(1 2) '(2 1) '(2)) ⇒ (2 2)

`lset-intersection` の戻り値は、list1 と末尾を共有する場合があります。`lset-intersection!` は、結果を生成するために list1 を変更する場合があります。

Scheme Procedure: **lset-difference** \= list1 list2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002ddifference)

Scheme Procedure: **lset-difference!** \= list1 list2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002ddifference_0021)

list1からlist2、list3などの要素をすべて削除（減算）して返します。引数としてリストが1つだけの場合は、そのリストのみが返されます。

指定された \= プロシージャは、`(= elem1 elemN)` のように呼び出される要素の比較に使用されます。最初の引数はリスト1から、2番目の引数は後続のリストのいずれかから取得されます。ただし、どの呼び出しがどのような順序で行われるかは指定されていません。

(lset-difference eqv? '(xy)) ⇒ (xy)
(lset-difference eqv? '(1 2 3) '(3 1)) ⇒ (2)
(lset-difference eqv? '(1 2 3) '(3) '(2)) ⇒ (1)

`lset-difference` の戻り値は、list1 と末尾を共有する場合があります。`lset-difference!` は、結果を生成するために list1 を変更する場合があります。

スキーム手順: **lset-diff+intersection** \= list1 list2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002ddiff_002bintersection)

Scheme Procedure: **lset-diff+intersection!** \= list1 list2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002ddiff_002bintersection_0021)

2 つの値を返します ([複数の値の返却と受け入れ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Multiple-Values) を参照)。これは、上記の `lset-difference` および `lset-intersection` に従って、引数リストの差分と共通部分です。

リスト引数が2つの場合、これはリスト1を、リスト2に含まれる要素と含まれない要素に分割します。（ただし、引数が3つ以上の場合、リスト1の要素は差分にも共通部分にも含まれない可能性があります。）

`lset-diff+intersection` の戻り値のうちの 1 つは、list1 と末尾を共有する可能性があります。`lset-diff+intersection!` は、結果を生成するために list1 を変更する可能性があります。

Scheme Procedure: **lset-xor** \= list … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002dxor)

Scheme Procedure: **lset-xor!** \= list … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-lset_002dxor_0021)

引数として渡されたリストのXOR演算結果を返します。2つのリストの場合、これはどちらか一方のリストにのみ含まれる要素を意味します。3つ以上のリストの場合、これは奇数個のリストに含まれる要素を意味します。

正確には、2つのリストxとyのXORは、xの要素のうちyのどの要素とも等しくないものと、yの要素のうちxのどの要素とも等しくないものを足し合わせることによって生成されます。等価性は、指定された\=プロシージャ（`(= e1 e2)`と表記）によって判定されます。引数の1つはxから、もう1つはyから取得されますが、どちらが先かは指定されていません。どの関数が呼び出されるか、また結果の要素の順序も指定されていません。

(lset-xor eqv? '(xy)) ⇒ (xy)
(lset-xor eqv? '(1 2 3) '(4 3 2)) ⇒ (4 1)

`lset-xor` の戻り値は、リスト引数のいずれかと末尾を共有する場合があります。`lset-xor!` は、結果を生成するためにリスト1を変更する場合があります。

* * *

次へ: [SRFI-4 - 同質な数値ベクトルデータ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d4)、前: [SRFI-1 - リストライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d1)、上: [SRFI サポートモジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
