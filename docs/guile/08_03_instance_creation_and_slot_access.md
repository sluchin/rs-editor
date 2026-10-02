### 8.3 インスタンスの作成とスロットへのアクセス

定義済みのクラスのインスタンス（またはオブジェクト）は、`make` を使用して作成できます。`make` は、作成するインスタンスのクラスという必須パラメータと、新しいインスタンスのスロットを初期化するために使用されるオプション引数のリストを受け取ります。たとえば、次の形式です。

(define c ([make](#83-インスタンスの作成とスロットへのアクセス) <my-complex>))

新しい `<my-complex>` オブジェクトを作成し、それを Scheme 変数 `c` にバインドします。

汎用: **make**

メソッド: **make** (class <class>) initarg …

initarg を使用して初期化されたクラスの新しいインスタンスを作成して返します。

理論上、initarg … は、新しく割り当てられたインスタンスに `initialize` 汎用関数が適用されるときに適用されるメソッドによって理解される任意の構造を持つことができます。

実際には、特殊な `initialize` メソッドは通常 `(next-method)` を呼び出し（[Next-method](08_06_methods_and_generic_functions.md#864-次のメソッド) を参照）、最終的に標準の GOOPS `initialize` メソッドが適用されます。これらのメソッドは、要素数が偶数のリスト initargs を想定しており、偶数番目の要素（0 から数えて）はキーワード、奇数番目の要素は対応する値です。

GOOPS は、定義に `#:init-keyword` オプションが含まれるスロットの初期化引数キーワードを自動的に処理します ([init-keyword](08_04_slot_options.md#84-スロットオプション) を参照)。その他のキーワードと値のペアは、新しいインスタンスのクラスに特化した `initialize` メソッドでのみ処理できます。処理されなかったキーワードと値のペアは無視されます。

汎用: **make-instance**

メソッド: **make-instance** (class <class>) initarg …

`make-instance`は`make`のエイリアスです。

新しい複素数のスロットには、`slot-ref`と`slot-set!`を使用してアクセスできます。`slot-set!`はオブジェクトスロットの値を設定し、`slot-ref`はその値を取得します。

([slot-set!](08_08_introspection.md#885-スロットへのアクセス) c 'r 10)
([slot-set!](08_08_introspection.md#885-スロットへのアクセス) c 'i 3)
([slot-ref](08_08_introspection.md#885-スロットへのアクセス) c 'r) ⇒ 10
([slot-ref](08_08_introspection.md#885-スロットへのアクセス) c 'i) ⇒ 3

`(oop goops describe)` モジュールは、オブジェクトのすべてのスロットを確認するのに便利な `describe` 関数を提供します。この関数は、スロットとその値を標準出力に出力します。

([describe](04_programming_in_scheme.md#4441-ヘルプコマンド) c)
⊣
#<<my-complex> 401d8638> は [class](08_11_the_metaobject_protocol.md#8115-クラス定義プロトコル) <my-complex> のインスタンスです
スロットは以下の通りです。
r [\=](06_06_02_numerical_data_types.md#6628-比較述語) 10
i [\=](06_06_02_numerical_data_types.md#6628-比較述語) 3

* * *

次へ: [スロット記述の例](08_05_illustrating_slot_description.md#85-スロットの説明の図解)、前: [インスタンスの作成とスロットへのアクセス](#83-インスタンスの作成とスロットへのアクセス)、上: [GOOPS](08_00_goops.md#8-goops) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
