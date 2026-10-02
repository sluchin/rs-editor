### 8.3 インスタンスの作成とスロットへのアクセス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instance-Creation-and-Slot-Access)

定義済みのクラスのインスタンス（またはオブジェクト）は、`make` を使用して作成できます。`make` は、作成するインスタンスのクラスという必須パラメータと、新しいインスタンスのスロットを初期化するために使用されるオプション引数のリストを受け取ります。たとえば、次の形式です。

(define c ([make](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make-1) <my-complex>))

新しい `<my-complex>` オブジェクトを作成し、それを Scheme 変数 `c` にバインドします。

汎用: **make** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make-1)

メソッド: **make** (class <class>) initarg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make-2)

initarg を使用して初期化されたクラスの新しいインスタンスを作成して返します。

理論上、initarg … は、新しく割り当てられたインスタンスに `initialize` 汎用関数が適用されるときに適用されるメソッドによって理解される任意の構造を持つことができます。

実際には、特殊な `initialize` メソッドは通常 `(next-method)` を呼び出し（[Next-method](https://doc.guix.gnu.org/guile/latest/en/guile.html#Next_002dmethod) を参照）、最終的に標準の GOOPS `initialize` メソッドが適用されます。これらのメソッドは、要素数が偶数のリスト initargs を想定しており、偶数番目の要素（0 から数えて）はキーワード、奇数番目の要素は対応する値です。

GOOPS は、定義に `#:init-keyword` オプションが含まれるスロットの初期化引数キーワードを自動的に処理します ([init-keyword](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照)。その他のキーワードと値のペアは、新しいインスタンスのクラスに特化した `initialize` メソッドでのみ処理できます。処理されなかったキーワードと値のペアは無視されます。

汎用: **make-instance** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dinstance)

メソッド: **make-instance** (class <class>) initarg … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dinstance-1)

`make-instance`は`make`のエイリアスです。

新しい複素数のスロットには、`slot-ref`と`slot-set!`を使用してアクセスできます。`slot-set!`はオブジェクトスロットの値を設定し、`slot-ref`はその値を取得します。

([slot-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_0021-1) c 'r 10)
([slot-set!](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_0021-1) c 'i 3)
([slot-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref-1) c 'r) ⇒ 10
([slot-ref](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref-1) c 'i) ⇒ 3

`(oop goops describe)` モジュールは、オブジェクトのすべてのスロットを確認するのに便利な `describe` 関数を提供します。この関数は、スロットとその値を標準出力に出力します。

([describe](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-describe) c)
⊣
#<<my-complex> 401d8638> は [class](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class-1) <my-complex> のインスタンスです
スロットは以下の通りです。
r [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) 10
i [\=](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_003d) 3

* * *

次へ: [スロット記述の例](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Description-Example)、前: [インスタンスの作成とスロットへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instance-Creation)、上: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
