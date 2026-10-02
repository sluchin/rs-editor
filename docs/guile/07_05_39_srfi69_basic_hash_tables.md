#### 7.5.39 SRFI-69 - 基本ハッシュテーブル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-_002d-Basic-hash-tables)

これは、Guile の組み込みハッシュテーブルと弱テーブルのサポートをラップしたポータブルなライブラリです。組み込みサポートの詳細については、[ハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-Tables) を参照してください。さらに、このハッシュテーブルインターフェースは、作成時に等価性関数とハッシュ関数をテーブルに関連付けるため、各関数のバリアントは不要です。また、この SRFI では提供されていない、Guile ハッシュテーブルハンドルのほとんどの用途を処理する手順も提供します。

アクセス方法：

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (srfi srfi-69))

* [ハッシュテーブルの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Creating-hash-tables)
* [テーブル項目へのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Accessing-table-items)
* [テーブルプロパティ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Table-properties)
* [ハッシュテーブルアルゴリズム](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Hash-table-algorithms)

* * *

次へ: [テーブル項目へのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Accessing-table-items)、上へ: [SRFI-69 - 基本的なハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.39.1 ハッシュテーブルの作成 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Creating-hash-tables)

Scheme Procedure: **make-hash-table** \[equal-proc hash-proc #:weak weak start-size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dhash_002dtable-1)

等価関数として equal-proc、ハッシュ関数として hash-proc を使用して、新しいハッシュテーブルを作成し、回答を出力してください。

デフォルトでは、equal-proc は `equal?` です。これは任意の 2 つの引数を持つプロシージャで、このテーブルの目的において 2 つのキーが同じかどうかを答える必要があります。

デフォルトでは、hash-proc は `equal-proc` が `equal?` より粗いものではないと想定します。ただし、`string-ci=?` が文字通り指定されている場合は除きます。hash-proc が指定されている場合は、キーと現在のテーブルサイズを受け取り、0 (含む) からサイズ (含まない) までの適切なハッシュ整数を返す 2 つの引数を持つプロシージャである必要があります。

弱さは「#f」またはハッシュテーブルの「弱さ」を示す記号で表す必要があります。

`#f`

通常の非弱ハッシュテーブル。これがデフォルトです。

`キー`

GC時にそのキーに非弱参照がなくなったら、そのエントリを削除します。

`値`

GC時にその値に非弱参照がなくなったら、そのエントリを削除します。

`キーまたは値`

GCにおいて、どちらか一方に非弱参照がなくなったら、関連付けを解除します。

Guileがハッシュテーブルを拡張できなかった時代の名残として、start-sizeはハッシュテーブルのおおよその開始サイズを指定するオプションの整数引数であり、アルゴリズム的に適切な数値に丸められます。

`equal?`よりも_粗い_とは、`(equal-proc xy)`となるすべてのxとyの値に対して、`(equal? xy)`も成り立つことを意味します。equal-procでこれが成り立たない場合は、hash-procを指定する必要があります。

弱いテーブルの場合、上記の _references_ は常に `eq?` による参照を指すことに注意してください。文字列 `"foo"` への参照があるからといって、弱いキーのテーブルでキー `"foo"` との関連付けが収集されないわけではありません。`equal-proc` に関係なく、2 つの `"foo"` が `eq?` である場合にのみ参照としてカウントされます。そのため、弱いテーブルの等価関数とハッシュ関数として `eq?` と `hashq` を使用するのが通常は賢明です。Guile の組み込みの弱いテーブルのサポートの詳細については、[弱い参照](https://doc.guix.gnu.org/guile/latest/en/guile.html#Weak-References) を参照してください。

Scheme Procedure: **alist->hash-table** alist \[equal-proc hash-proc #:weak weakness start-size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-alist_002d_003ehash_002dtable-1)

`make-hash-table`と同様ですが、alist内の関連付けを使用して初期化します。alist内でキーが重複している場合は、左端の関連付けが優先されます。

* * *

次へ: [テーブルプロパティ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Table-properties)、前: [ハッシュテーブルの作成](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Creating-hash-tables)、上: [SRFI-69 - 基本的なハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.39.2 テーブル項目へのアクセス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-table-items)

Scheme Procedure: **hash-table-ref** テーブルキー \[default-thunk\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dref)

スキーム手順: **hash-table-ref/default** テーブルキー default [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dref_002fdefault)

テーブル内のキーに関連付けられた値を入力してください。キーが存在しない場合は、デフォルトでエラーを示すデフォルトサンクの呼び出し結果を入力してください。

`hash-table-ref/default` は、3 番目の引数 default を必要とするバリアントであり、default を呼び出す代わりに default 自体を返します。

Scheme Procedure: **hash-table-set!** table key new-value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- hash_002dtable_002dset_0021)

テーブル内のキーを新しい値に設定します。

Scheme Procedure: **hash-table-delete!** テーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002ddelete_0021)

テーブル内にキーの関連付けが存在する場合は、それを削除します。存在しない場合は、何も行いません。

Scheme Procedure: **hash-table-exists?** テーブルキー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dexists_003f)

キーがテーブル内で関連付けられているかどうかを回答してください。

Scheme 手順: **hash-table-update!** テーブルキー修飾子 \[default-thunk\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dupdate_0021)

スキーム手順: **hash-table-update!/default** テーブルキー修飾子 default [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dupdate_0021_002fdefault)

テーブル内のキーに関連付けられた値を、引数1つ（古い値）を指定して修飾子を呼び出すことで置き換えます。

キーが存在せず、かつdefault-thunkが指定されている場合は、引数なしでそれを呼び出し、上記のように修飾子に渡される「古い値」を取得します。このような場合にdefault-thunkが指定されていない場合は、エラーを通知します。

`hash-table-update!/default` は、4 番目の引数を必要とするバリアントです。この引数は、「古い値」を取得するために呼び出されるサンクとしてではなく、「古い値」として直接使用されます。

* * *

次へ: [ハッシュテーブルアルゴリズム](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Hash-table-algorithms)、前: [テーブル項目へのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Accessing-table-items)、上: [SRFI-69 - 基本ハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引"）\]

#### 7.5.39.3 テーブルプロパティ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Table-properties)

Scheme Procedure: **hash-table-size** テーブル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dsize)

テーブル内の関連付けの数を答えます。非弱テーブルの場合、この処理は定数時間で実行されることが保証されています。

スキーム手順: **ハッシュテーブルキー** テーブル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dkeys)

表内のキーを順不同でリスト形式で回答してください。

Scheme Procedure: **hash-table-values** テーブル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dvalues)

表の値を順不同でリスト形式で答えてください。

Scheme プロシージャ: **hash-table-walk** table proc [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dwalk)

テーブル内の各関連付けに対して、キーと値を引数として渡して、プロシージャを一度呼び出します。

Scheme プロシージャ: **hash-table-fold** table proc init [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dfold)

テーブル内の各キーと値に対して、`(proc key value previous)` を呼び出します。ここで、`previous` は前回の呼び出しの結果であり、`init` を最初の `previous` 値として使用します。最終的な proc の結果を回答してください。

Scheme Procedure: **hash-table->alist** table [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002d_003ealist)

表内の各関連付けが結果内の関連付けとなるような関連付けリストを回答してください。

* * *

前へ: [テーブルプロパティ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69-Table-properties)、上へ: [SRFI-69 - 基本ハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 7.5.39.4 ハッシュテーブルアルゴリズム [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Hash-table-algorithms)

各ハッシュテーブルには、キー検索を実装するために使用される等価関数とハッシュ関数が含まれています。初心者ユーザーは、上記で指定したデフォルトのハッシュプロシージャの一貫性に関するルールに従う必要があります。上級ユーザーは、これらを使用して、特殊な検索セマンティクスに対応する独自の等価関数とハッシュ関数を実装できます。

Scheme Procedure: **hash-table-equivalence-function** hash-table [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dtable_002dequivalence_002dfunction)

Scheme Procedure: **hash-table-hash-function** hash-table [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- hash_002dtable_002dhash_002dfunction)

ハッシュテーブルの等価性とハッシュ関数について、それぞれ回答してください。

Scheme Procedure: **hash** obj \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash-1)

Scheme手順: **string-hash** obj \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dhash-1)

Scheme Procedure: **string-ci-hash** obj \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-string_002dci_002dhash)

スキーム手順: **hash-by-identity** obj \[size\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-hash_002dby_002didentity)

等価述語`equal?`、`string=?`、`string-ci=?`、および`eq?`にそれぞれ適切なハッシュ値を回答してください。

`hash`は、Guileに組み込まれている`hash`の後方互換性のある代替機能です。

* * *

次へ: [SRFI-87 => in case 句](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d87)、前: [SRFI-69 - 基本的なハッシュテーブル](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d69)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
