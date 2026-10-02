#### 6.6.15 レコードの概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Record-Overview-1)

レコード（構造体とも呼ばれる）は、Scheme において新しい非連結型を定義するための主要なメカニズムです。レコード型は、その型のインスタンスを構成するフィールドのリストを定義します。これは C 言語の `struct` に相当します。

歴史的に、Guileはレコード型を定義し、レコードを作成するためのさまざまな方法を提供してきました。それぞれ異なる機能とトレードオフを備えています。長年にわたり、それぞれの「標準」には独自の新しいレコードインターフェースが付属しており、レコードAPIの複雑な迷路のような状態になっています。

最上位レベルは、ほとんどの Scheme 実装で実装されている高レベルレコードインターフェースである SRFI-9 です ([SRFI-9 Records](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d9-Records) を参照)。これは、レコード型とそれに関連付けられた型述語、フィールド、フィールドアクセサのシンプルで効率的な構文的抽象化を定義します。SRFI-9 はほとんどの用途に適しており、Guile でレコード型を作成する推奨方法です。同様の高レベルレコード API には、SRFI-35 ([SRFI-35 - Conditions](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d35) を参照) と R6RS レコード ([rnrs records syntactic](https://doc.guix.gnu.org/guile/latest/en/guile.html#rnrs-records-syntactic) を参照) があります。

次に、Guile の従来型の「レコード」API が登場します ([レコード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Records) を参照)。このように定義されたレコード型は、第一級オブジェクトです。イントロスペクション機能が利用可能で、ユーザーは型を事前に知らなくても、実行時にフィールドのリストや特定のフィールドの値を照会できます。

最後に、これらのインターフェースに共通する要素は、Guile の _structure_ API です ([Structures](https://doc.guix.gnu.org/guile/latest/en/guile.html#Structures) を参照)。Guile の構造体は、他のすべてのレコード API の低レベルな構成要素です。アプリケーション開発者は通常、これを使用する必要はありません。

これらのAPIを使用して作成されたレコードはすべて、Guileの標準パターンマッチング機能を使用してパターンマッチングできます（[パターンマッチング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Pattern-Matching)を参照）。

* * *

次へ: [レコード](https://doc.guix.gnu.org/guile/latest/en/guile.html#Records)、前: [レコードの概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#Record-Overview)、上: [データ型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Types) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
