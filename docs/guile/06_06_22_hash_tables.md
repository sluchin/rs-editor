#### 6.6.22 ハッシュテーブル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Tables-1)

ハッシュテーブルは、連想リストと同様の機能を提供する辞書です。キーから値へのマッピングを提供します。違いは、連想リストはエントリを検索する際に要素のサイズに比例した時間を要するのに対し、ハッシュテーブルは通常定数時間で検索できる点です。欠点は、ハッシュテーブルはより多くのメモリを必要とすることと、通常のリスト操作手順（[Lists](https://doc.guix.gnu.org/guile/latest/en/guile.html#Lists)を参照）を使用できないことです。

* [ハッシュテーブルの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Table-Examples)
* [ハッシュテーブルリファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Table-Reference)

* * *

次へ: [ハッシュテーブルリファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Table-Reference)、上へ: [ハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Tables) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.22.1 ハッシュテーブルの例 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Table-Examples-1)

このセクションでは、説明のために、ハッシュテーブルの手順の使用例をいくつか示し、それぞれの手順が何をするのかを説明します。

まず、31個のスロットを持つ新しいハッシュテーブルを作成し、そこに2つのキーと値のペアを格納します。

(define h ([make-hash-table](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dhash_002dtable) 31))

;; これは不透明なオブジェクトです
h
⇒
#<ハッシュテーブル 0/31>

;; ハッシュテーブルへの挿入は hashq-set で行うことができます!
([hashq-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dset_0021) h 'foo "bar")
⇒
"バー"

([hashq-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dset_0021) h 'braz "zonk")
⇒
「ゾンク」

;; または hash-create-handle を使用！
([hashq-create-handle!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dcreate_002dhandle_0021) h 'frob #f)
⇒
(frob . #f)

`hashq-ref` プロシージャを使用すると、指定されたキーに対応する値を取得できますが、このプロシージャの問題点は、キーがテーブルに存在するかどうかを確実に判断できないことです。その理由は、キーがテーブルに存在しない場合はプロシージャが `#f` を返しますが、キーがテーブルに存在していてたまたま値が `#f` である場合も同じ値を返すためです。以下の例で確認できます。

([hashq-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dref) h 'foo)
⇒
"バー"

([hashq-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dref) h 'frob)
⇒
#f

([hashq-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dref) h 'not-there)
⇒
#f

多くの場合、2つのケースを区別するプロシージャ`hashq-get-handle`を使用する方が良いでしょう。`assq`と同様に、このプロシージャは成功した場合はキーと値のペアを返し、キーが見つからない場合は`#f`を返します。

([hashq-get-handle](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dget_002dhandle) h 'foo)
⇒
(foo . "bar")

([hashq-get-handle](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dget_002dhandle) h 'not-there)
⇒
#f

`hash-fold` を使用して各要素を処理することで、興味深い結果が得られます。この例では、要素の総数をカウントします。

([hash-fold](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dfold) (lambda (key value seed) ([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- _002b) 1 seed)) 0 h)
⇒
3

同様のことは、特定の述語に一致する要素の数をカウントできるプロシージャ`hash-count`でも実行できます。たとえば、文字列値を持つ要素の数をカウントするには、次のようにします。

([hash-count](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dcount) (lambda (key value) ([string?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_003f) value)) h)
⇒
2

`const` を使用すれば、すべての要素を数えるのは簡単な作業です。

([hash-count](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dcount) ([const](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-const) #t) h)
⇒
3

* * *

前へ: [ハッシュテーブルの例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Table-Examples)、上へ: [ハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Tables) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 6.6.22.2 ハッシュテーブルリファレンス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Table-Reference-1)

連想リスト関数と同様に、ハッシュテーブル関数もキーの等価性判定方法によっていくつかの種類があります。通常の`hash-`関数は`equal?`を使用し、 `hashq-`関数は`eq?`を使用し、`hashv-`関数は`eqv?`を使用し、`hashx-`関数はアプリケーションが提供する判定方法を使用します。

`make-hash-table` コマンドを一度実行するだけで、任意の関数セットで使用できるハッシュテーブルを作成できますが、その後は必ず 1 つの関数セットのみを一貫して使用することが不可欠です。そうしないと、結果が予測不能になる可能性があります。

ハッシュテーブルは、キーから生成されたハッシュ値をインデックスとするベクトルとして実装され、異なるキーが同じハッシュ値になる場合に備えて、各バケットごとにキーと値のペアの関連付けリストが作成されます。これらのリスト内のペアへの直接アクセスは、`-handle-`関数によって提供されます。

ハッシュテーブルのエントリ数がしきい値を超えると、バケットリストが長くなりすぎてアクセス速度が低下するのを防ぐため、ベクトルが拡張され、エントリが再ハッシュされます。エントリ数がしきい値を下回ると、スペースを節約するためにベクトルが縮小されます。

`hashx-` の「拡張」ルーチンでは、アプリケーションは、以下の `hashq` などのような整数インデックスを生成するハッシュ関数と、`assq` などのような連想リスト検索関数を提供します ([Alist エントリの取得](https://doc.guix.gnu.org/guile/latest/en/guile.html#Retrieving-Alist-Entries) を参照)。以下は、文字列キーの大文字小文字を区別しないハッシュを実装する関数の例です。

(use-modules (srfi srfi-1)
(srfi srfi-13))

(define (my-hash str size)
(残りの (文字列ハッシュ ci str) サイズ))
(define (my-assoc str alist)
(find (lambda (pair) (string-ci=? str (car pair))) alist))

(define my-table (make-hash-table))
(hashx-set! my-hash my-assoc my-table "foo" 123)

(hashx-ref my-hash my-assoc my-table "FOO")
⇒ 123

`hashx-`ハッシュ関数では、キーをベクトル全体に分散させることで、バケットリストが長くなりすぎないようにすることが目的です。ただし、実際の値は0から_size\-1_の範囲内であれば任意です。ハッシュ値を生成するのに役立つ関数として、以下の `hashq` などに加えて、`symbol-hash` ([Symbols as Lookup Keys](https://doc.guix.gnu.org/guile/latest/en/guile.html#Symbol-Keys) を参照)、`string-hash` および `string-hash-ci` ([String Comparison](https://doc.guix.gnu.org/guile/latest/en/guile.html#String-Comparison) を参照)、`char-set-hash` ([Character Set Predicates/Comparison](https://doc.guix.gnu.org/guile/latest/en/guile.html#Character-Set-Predicates_002fComparison) を参照) があります。

  

Scheme手順: **make-hash-table** \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dhash_002dtable)

オプションで最小ベクトルサイズを指定して、新しいハッシュテーブルオブジェクトを作成します。

サイズが指定されている場合、テーブルベクトルは上記のように自動的に拡大縮小しますが、サイズは最小値となります。アプリケーションがテーブルに格納されるエントリ数を概ね把握している場合は、サイズを使用して初期エントリの追加時に再ハッシュを回避することができます。

スキーム手順: **alist->hash-table** alist [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alist_002d_003ehash_002dtable)

スキーム手順: **alist->hashq-table** alist [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alist_002d_003ehashq_002dtable)

スキーム手順: **alist->hashv-table** alist [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alist_002d_003ehashv_002dtable)

スキーム手順: **alist->hashx-table** ハッシュ関連付け alist [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- alist_002d_003ehashx_002dtable)

alistをハッシュテーブルに変換します。alist内でキーが重複している場合、最も左にある関連付けが優先されます。

(use-modules (ice-9 hash-table))
(alist->hash-table '((foo . 1) (bar . 2)))

拡張ハッシュテーブルに変換する場合、カスタムハッシュおよび関連付け手順を提供する必要があります。

(alist->hashx-table hash assoc '((foo . 1) (bar . 2)))

Scheme Procedure: **hash-table?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_003f)

C 関数: **scm\_hash\_table\_p** (obj) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005ftable_005fp)

objが抽象ハッシュテーブルオブジェクトである場合は、`#t`を返します。

Scheme Procedure: **hash-clear!** table [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dclear_0021)

C 関数: **scm\_hash\_clear\_x** (表) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005fclear_005fx)

テーブルからすべての項目を削除します（サイズ変更は発生させません）。

スキーム手順: **hash-ref** テーブルキー \[dflt\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dref)

スキーム手順: **hashq-ref** テーブルキー \[dflt\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dref)

スキーム手順: **hashv-ref** テーブルキー \[dflt\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashv_002dref)

スキーム手順: **hashx-ref** ハッシュ関連付けテーブルキー \[dflt\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashx_002dref)

C 関数: **scm\_hash\_ref** (table, key, dflt) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005fref)

C 関数: **scm\_hashq\_ref** (table, key, dflt) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashq_005fref)

C 関数: **scm\_hashv\_ref** (table, key, dflt) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashv_005fref)

C 関数: **scm\_hashx\_ref** (hash、assoc、table、key、dflt) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashx_005fref)

指定されたハッシュテーブルでキーを検索し、対応する値を返します。キーが見つからない場合はdfltを返し、dfltが指定されていない場合は`#f`を返します。

Scheme Procedure: **hash-set!** table key val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dset_0021)

Scheme Procedure: **hashq-set!** table key val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dset_0021)

Scheme Procedure: **hashv-set!** table key val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashv_002dset_0021)

Scheme Procedure: **hashx-set!** hash assoc table key val [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashx_002dset_0021)

C 関数: **scm\_hash\_set\_x** (table, key, val) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fhash_005fset_005fx)

C 関数: **scm\_hashq\_set\_x** (テーブル、キー、値) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashq_005fset_005fx)

C 関数: **scm\_hashv\_set\_x** (テーブル、キー、値) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashv_005fset_005fx)

C 関数: **scm\_hashx\_set\_x** (hash, assoc, table, key, val) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashx_005fset_005fx)

指定されたハッシュテーブル内で、値とキーを関連付けます。キーが既に存在する場合は、関連付けられた値が変更されます。存在しない場合は、新しいエントリが作成されます。

Scheme Procedure: **hash-remove!** テーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dremove_0021)

Scheme Procedure: **hashq-remove!** テーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dremove_0021)

Scheme Procedure: **hashv-remove!** テーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashv_002dremove_0021)

スキーム手順: **hashx-remove!** ハッシュ関連付けテーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashx_002dremove_0021)

C 関数: **scm\_hash\_remove\_x** (テーブル、キー) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005fremove_005fx)

C 関数: **scm\_hashq\_remove\_x** (テーブル、キー) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashq_005fremove_005fx)

C 関数: **scm\_hashv\_remove\_x** (テーブル、キー) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashv_005fremove_005fx)

C 関数: **scm\_hashx\_remove\_x** (hash、assoc、table、key) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashx_005fremove_005fx)

指定されたハッシュテーブル内のキーに対応するすべての関連付けを削除します。キーがテーブルに存在しない場合は、何も行いません。

スキーム手順: **ハッシュ**キーサイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash)

スキーム手順: **hashq** キーサイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq)

スキーム手順: **hashv** キーサイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashv)

C 関数: **scm\_hash** (key, size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash)

C 関数: **scm\_hashq** (key, size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashq)

C 関数: **scm\_hashv** (key, size) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashv)

キーに対応するハッシュ値を返します。これは、指定されたサイズのハッシュテーブルで使用するのに適した、_0_ から _size\-1_ の範囲の数値です。

`hashq` と `hashv` はオブジェクトの内部アドレスを使用する可能性があるため、オブジェクトがガベージコレクションされて再作成されると、2 つのハッシュ値が概念的に `eq?` であっても、ハッシュ値が異なる可能性があります。たとえば、シンボルの場合、

(hashq '何か 123) ⇒ 19
(gc)
(hashq '何か 123) ⇒ 62

通常の使用においては、ハッシュテーブルに格納されたオブジェクトは削除されるまでガベージコレクションされないため、これは問題になりません。ハッシュ計算が通常の参照から何らかの形で分離されている場合にのみ、そのオブジェクトのライフサイクルを考慮する必要があります。

スキーム手順: **hash-get-handle** テーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dget_002dhandle)

Scheme Procedure: **hashq-get-handle** テーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dget_002dhandle)

Scheme Procedure: **hashv-get-handle** テーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashv_002dget_002dhandle)

スキーム手順: **hashx-get-handle** ハッシュ関連付けテーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashx_002dget_002dhandle)

C 関数: **scm\_hash\_get\_handle** (テーブル、キー) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005fget_005fhandle)

C 関数: **scm\_hashq\_get\_handle** (テーブル、キー) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashq_005fget_005fhandle)

C 関数: **scm\_hashv\_get\_handle** (テーブル、キー) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashv_005fget_005fhandle)

C 関数: **scm\_hashx\_get\_handle** (hash、assoc、table、key) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashx_005fget_005fhandle)

指定されたハッシュテーブル内のキーに対応する `(キー . 値)` のペアを返します。キーがテーブルにない場合は `#f` を返します。

Scheme 手順: **hash-create-handle!** テーブルキー初期化 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dcreate_002dhandle_0021)

Scheme 手順: **hashq-create-handle!** テーブル キー初期化 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashq_002dcreate_002dhandle_0021)

Scheme 手順: **hashv-create-handle!** テーブルキー初期化 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashv_002dcreate_002dhandle_0021)

Scheme Procedure: **hashx-create-handle!** hash assoc table key init [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hashx_002dcreate_002dhandle_0021)

C 関数: **scm\_hash\_create\_handle\_x** (テーブル、キー、初期化) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005fcreate_005fhandle_005fx)

C 関数: **scm\_hashq\_create\_handle\_x** (テーブル、キー、初期化) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashq_005fcreate_005fhandle_005fx)

C 関数: **scm\_hashv\_create\_handle\_x** (テーブル、キー、初期化) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhashv_005fcreate_005fhandle_005fx)

C 関数: **scm\_hashx\_create\_handle\_x** (hash、assoc、table、key、init) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- scm_005fhashx_005fcreate_005fhandle_005fx)

指定されたハッシュテーブル内のキーに対応する `(キー . 値)` のペアを返します。キーがテーブルに存在しない場合は、値として init を持つエントリを作成し、そのペアを返します。

Scheme プロシージャ: **hash-map->list** proc table [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dmap_002d_003elist)

Scheme Procedure: **hash-for-each** proc table [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dfor_002deach)

C 関数: **scm\_hash\_map\_to\_list** (proc, table) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005fmap_005fto_005flist)

C 関数: **scm\_hash\_for\_each** (proc, table) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005ffor_005feach)

指定されたハッシュテーブルのエントリにprocを適用します。各呼び出しは`(procキー値)`です。`hash-map->list`はこれらの呼び出しの結果のリストを返し、`hash-for-each`は結果を破棄して未指定の値を返します。

テーブルエントリに対する呼び出しは、指定されていない順序で行われます。また、`hash-map->list` の場合、返されるリストの値の順序は指定されていません。反復処理中にテーブルが変更された場合、結果は予測不能になります。

例えば、以下は`mytable`のすべてのエントリを順不同で含む新しいalistを返します。

(ハッシュマップ->リスト cons mytable)

Scheme Procedure: **hash-for-each-handle** proc table [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dfor_002deach_002dhandle)

C 関数: **scm\_hash\_for\_each\_handle** (proc, table) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005ffor_005feach_005fhandle)

指定されたハッシュテーブルのエントリにprocを適用します。各呼び出しは`(proc handle)`で、handleは`(key . value)`のペアです。戻り値は未指定です。

`hash-for-each-handle` は、proc の引数リストのみが `hash-for-each` と異なります。

Scheme手順: **hash-fold** proc init table [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dfold)

C 関数: **scm\_hash\_fold** (proc、init、table) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005ffold)

指定されたハッシュテーブルの要素にprocを適用して結果を蓄積します。各呼び出しは`(proc key value prior-result)`の形式で行われ、keyとvalueはテーブルから取得され、prior-resultは前回のproc呼び出しの戻り値です。最初の呼び出しでは、prior-resultは指定された初期値です。

テーブルエントリに対する呼び出しは、指定されていない順序で行われます。`hash-fold`の実行中にテーブルが変更された場合、結果は予測不可能になります。

例えば、以下は`mytable`内のキーのうち文字列であるキーの数を返します。

(hash-fold (lambda (key value prior)
(if (string? key) (1+ prior) prior))
0 mytable)

Scheme Procedure: **hash-count** pred table [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dcount)

C 関数: **scm\_hash\_count** (pred, table) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005fhash_005fcount)

指定されたハッシュテーブル内で、`(pred key value)` が true を返す要素の数を返します。要素の総数を素早く判断するには、pred に `(const #t)` を使用します。

* * *

前へ: [ハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Tables)、上へ: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
