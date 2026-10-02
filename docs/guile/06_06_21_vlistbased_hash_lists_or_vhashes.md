#### 6.6.21 VList ベースのハッシュ リストまたは「VHash」

`(ice-9 vlist)` モジュールは、_VList ベースのハッシュリスト_ の実装を提供します ([VLists](06_06_14_vlists.md#6614-vlists) を参照)。VList ベースのハッシュリスト、または _vhashes_ は、 _キー_ を _値_ にマッピングする、連想リストに似た不変の辞書型です。ただし、連想リストとは異なり、キーを指定して値にアクセスする操作は通常定数時間で完了します。

`(ice-9 vlist)` の VHash プログラミング インターフェイスは、SRFI-1 にある連想リストのものとほぼ同じですが、プロシージャ名に `alist-` の代わりに `vhash-` が接頭辞として付いています ([連想リスト](07_05_03_srfi1_list_library.md#7539-関連付けリスト) を参照)。

さらに、vhashはVList操作を使用して操作できます。

(vlist-head (vhash-consq 'a 1 vlist-null))
⇒ （a.1）

(vh1 を定義します (vhash-consq 'b 2 (vhash-consq 'a 1 vlist-null)))
(define vh2 (vhash-consq 'c 3 (vlist-tail vh1)))

(vhash-assq 'a vh2)
⇒ （a.1）
(vhash-assq 'b vh2)
⇒ #f
(vhash-assq 'c vh2)
⇒ （c.3）
(vlist->list vh2)
⇒ ((c . 3) (a . 1))

ただし、新しい VList を構築する手順 (`vlist-map`、`vlist-filter` など) は、vhash ではなく生の VList を返すことに注意してください。

(define vh (alist->vhash '((a . 1) (b . 2) (c . 3)) hashq))
(vhash-assq 'a vh)
⇒ （a.1）

(vl を定義)
;; これにより、生の vlist が作成されます。
(vlist-filter (lambda (key+value) (odd? (cdr key+value))) vh))
(vhash-assq 'a vl)
⇒エラー: 位置 2 の引数の型が間違っています

(vlist->list vl)
⇒ ((a . 1) (c . 3))

Scheme Procedure: **vhash?** obj

objがvhashであればtrueを返します。

スキーム手順: **vhash-cons** キー値 vhash \[hash-proc\]

スキーム手順: **vhash-consq** キー値 vhash

スキーム手順: **vhash-consv** キー値 vhash

キーと値が関連付けられたvhashに基づく新しいハッシュリストを返します。キーのハッシュはhash-procを使用して計算されます。vhashは`vlist-null`または`vhash-cons`の以前の呼び出しで返されたvhashのいずれかである必要があります。hash-procのデフォルトは`hash`です（[`hash`プロシージャ](06_06_22_hash_tables.md#66222-ハッシュテーブルリファレンス)を参照）。`vhash-consq`では`hashq`ハッシュ関数が使用され、`vhash-consv`では`hashv`ハッシュ関数が使用されます。

vhashを構築するために行われるすべての`vhash-cons`呼び出しは、同じハッシュプロシージャを使用する必要があります。そうでない場合、結果は未定義となります。

スキーム手順: **vhash-assoc** key vhash \[equal? \[hash-proc\]\]

スキーム手順: **vhash-assq** キー vhash

スキーム手順: **vhash-assv** キー vhash

`equal?` 等価述語 (デフォルトは `equal?`) に従ってキーがキーと等しい最初のキー/値ペアを vhash から返します。また、`hash-proc` (デフォルトは `hash`) を使用してキーのハッシュを計算します。2 番目の形式では等価述語として `eq?`、ハッシュ関数として `hashq` を使用し、最後の形式では `eqv?` と `hashv` を使用します。

ハッシュ処理に使用するハッシュ関数は、`vhash-cons`に渡されたものと常に同じものを使用することが重要です。そうしないと、結果が予測不能になります。

Scheme Procedure: **vhash-delete** key vhash \[equal? \[hash-proc\]\]

スキーム手順: **vhash-delq** キー vhash

スキーム手順: **vhash-delv** キー vhash

キーと vhash のすべての関連付けを削除し、キーを equal? (デフォルトは `equal?`) で比較し、hash-proc (デフォルトは `hash`) を使用してキーのハッシュを計算します。2 番目の形式では、等価述語として `eq?`、ハッシュ関数として `hashq` を使用します。最後の形式では、`eqv?` と `hashv` を使用します。

ここでも、hash-procの選択は、以前の`vhash-cons`呼び出しと一貫性がある必要があります。

Scheme手順: **vhash-fold** proc init vhash

Scheme 手順: **vhash-fold-right** proc init vhash

vhash のキー/値要素を指定された方向に折り返し、proc の各呼び出しは `(proc key value result)` の形式になります。ここで result は proc の前の呼び出しの結果であり、proc の最初の呼び出しでは result の値を初期化します。

Scheme Procedure: **vhash-fold\*** proc init key vhash \[equal? \[hash\]\]

Scheme Procedure: **vhash-foldq\*** proc init key vhash

スキーム手順: **vhash-foldv\*** proc init key vhash

vhash 内の key に関連付けられたすべての値を折り返し、proc の各呼び出しは `(proc value result)` の形式になります。ここで result は proc の前の呼び出しの結果であり、proc の最初の呼び出しのために result の値を初期化します。

vhash のキーは hash を使用してハッシュ化され、equal? を使用して比較されます。2 番目の形式では、等価述語として `eq?`、ハッシュ関数として `hashq` を使用します。3 番目の形式では、`eqv?` と `hashv` を使用します。

例：

(vh を定義)
(alist->vhash '((a . 1) (a . 2) (z . 0) (a . 3))))

(vhash-fold\* cons '() 'a vh)
⇒ (3 2 1)

(vhash-fold\* cons '() 'z vh)
⇒ （0）

スキーム手順: **alist->vhash** alist \[hash-proc\]

連想リスト alist に対応する vhash を返します。キーハッシュの計算には hash-proc を使用します。省略した場合、hash-proc はデフォルトで `hash` を使用します。

* * *

次へ: [その他の型](06_06_23_other_types.md#6623-その他の型)、前: [VList ベースのハッシュ リストまたは「VHash」](#6621-vlist-ベースのハッシュ-リストまたはvhash)、上: [データ型](06_06_00_data_types.md#66-データ型) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
