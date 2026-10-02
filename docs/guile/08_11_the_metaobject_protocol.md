### 8.11 メタオブジェクトプロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol-1)

ここまでで、メタオブジェクトプロトコルの概念に触れることなくGOOPSについて説明できることはほぼ全てです。クラスの再定義や既存インスタンスのクラス変更など、個別に議論できるトピックは他にもいくつかありますが、実際には、これらのトピックを使用する開発者はメタオブジェクトプロトコルについても理解したいと考えるほど高度なスキルを持っているでしょうし、おそらくプロトコルを使ってこれらのイベント中に何が起こるかを正確にカスタマイズするでしょう。

それでは早速見ていきましょう。GOOPSは、CLOS（Common Lisp Object System）、tiny-clos（CLOSの機能の一部を実装した小さなScheme）、およびSTKlosで使用されているものから派生した「メタオブジェクトプロトコル」（別名「MOP」）に基づいています。

MOPは、アプリケーション定義クラスのインスタンスの初期化をカスタマイズするための`initialize`メソッドの定義など、GOOPSの多くのカスタマイズの基盤となっており、MOPを理解することで、こうしたカスタマイズをより正確に説明できるようになります。さらに深いレベルでは、MOPを理解することは、GOOPSを理解し、GOOPS自体の動作をカスタマイズすることでGOOPSの力を最大限に活用するための重要な要素となります。

* [メタオブジェクトとメタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Metaobjects-and-the-Metaobject-Protocol )
* [メタクラス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Metaclasses)
* [MOP仕様](https://doc.guix.gnu.org/guile/latest/en/guile.html#MOP-Specification)
* [インスタンス作成プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instance-Creation-Protocol)
* [クラス定義プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition-Protocol)
* [クラス定義のカスタマイズ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Customizing-Class-Definition)
* [メソッド定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Method-Definition)
* [メソッド定義内部](https://doc.guix.gnu.org/guile/latest/en/guile.html#Method-Definition-Internals)
* [汎用関数の内部構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-Internals)
* [汎用関数呼び出し](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-Invocation)

* * *

次へ: [メタクラス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Metaclasses)、上へ: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.1 メタオブジェクトとメタオブジェクトプロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Metaobjects-and-the-Metaobject-Protocol-1)

GOOPSの構成要素は、クラス、スロット定義、インスタンス、ジェネリック関数、およびメソッドです。クラスは、継承関係とスロット定義の集合です。インスタンスは、そのクラスのスーパークラスとスロット定義によって示される規則に従ってスロットが割り当てられるオブジェクトです。ジェネリック関数は、メソッドの集合と、ジェネリック関数が呼び出されたときにどのメソッドを適用するかを決定するための規則です。メソッドは、手続きと、その手続きが適用可能な引数の型を指定する特殊化子のセットです。

これらのエンティティのうち、GOOPSではクラス、汎用関数、メソッドを「メタオブジェクト」として表現します。つまり、GOOPSプログラム内でクラス、汎用関数、メソッドを記述する値は、それぞれクラス、汎用関数、メソッドの動作をカプセル化する特別なGOOPSクラスのインスタンス（または「オブジェクト」）なのです。

（残りの2つの要素は、スロット定義とインスタンスです。スロット定義は厳密にはインスタンスではありませんが、すべてのスロット定義は、アクセス可能性とガベージコレクションからの保護に関するスロットの動作を指定するGOOPSクラスに関連付けられています。インスタンスはもちろん通常の意味でのオブジェクトであり、メタオブジェクトとして考えることに利点はありません。）

「メタオブジェクトプロトコル」（または「MOP」）とは、これらのメタオブジェクトの動作を決定する汎用関数と、これらの汎用関数が呼び出される状況を定義する仕様のことである。

これが具体的に何を意味するのかを例にとると、`define-class` を使用して定義されるクラスのスロットセットを GOOPS がどのように計算するかを考えてみましょう。必要なスロットセットは、新しいクラスの直接のスロットと、そのすべてのスーパークラスのスロットの和集合です。しかし、`define-class` 自体はこの計算を実行しません。代わりに、`<class>` 型のインスタンス用に特別化された `initialize` ジェネリック関数のメソッドがあり、このメソッドがスロット計算を実行します。

`initialize`は、GOOPSが新しいインスタンスを作成するたびに、新しいインスタンスのメモリを割り当てた直後に呼び出し、新しいインスタンスのスロットを初期化する汎用関数です。手順は以下のとおりです。

* `define-class` は `make` を使用して `<class>` クラスの新しいインスタンスを作成し、初期化引数として `define-class` フォームで指定されたスーパークラス、スロット定義、およびクラスオプションを渡します。
* `make` は新しいインスタンスのメモリを割り当て、汎用関数 `initialize` を呼び出して新しいインスタンスのスロットを初期化します。
* 汎用関数 `initialize` は、型 `<class>` のインスタンスに特化したメソッドを適用し、このメソッドがスロット計算を実行します。

つまり、`define-class` にハードコーディングされるのではなく、クラス定義のデフォルトの動作は、クラス `<class>` に特化した汎用関数メソッドによってカプセル化されます。

`<class>` を継承する新しいクラス（「メタクラス」と呼ばれる）を作成し、その新しいメタクラスのインスタンス専用の新しい `initialize` メソッドを記述することができます。そして、`define-class` フォームに、値が新しいメタクラスである `#:metaclass` クラスオプションが含まれている場合、`define-class` フォームで定義されるクラスは、デフォルトの `<class>` ではなく、新しいメタクラスのインスタンスとなり、新しい `initialize` メソッドに従って定義されます。このようにして、デフォルトのスロット計算や、新しいクラスとスーパークラスとの関係におけるその他のあらゆる側面を変更または上書きすることができます。

同様に、標準の汎用関数クラス `<generic>` を継承する新しいクラスを作成し、その新しいクラスに特化した適切なメソッドを記述し、その新しいクラスのインスタンスである新しい汎用関数を作成することで、汎用関数の動作を変更またはオーバーライドできます。

メソッドメタオブジェクトについても同様です。そして、同じ基本的な仕組みによって、アプリケーションクラスの作成者は、そのアプリケーションクラスに特化した `initialize` メソッドを作成し、そのクラスのインスタンスを初期化することができます。

これがMOPの力です。`initialize`は、アプリケーションオブジェクトやクラス、そしてGOOPS自体の動作を変更するためにカスタマイズできる多数の汎用関数のうちの1つにすぎないことに注意してください。以降の各セクションでは、GOOPSの機能の特定の領域を取り上げ、その領域のカスタマイズに関連する汎用関数について説明します。

* * *

次へ: [MOP仕様](https://doc.guix.gnu.org/guile/latest/en/guile.html#MOP-Specification)、前: [メタオブジェクトとメタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Metaobjects-and-the-Metaobject-Protocol)、上: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.2 メタクラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Metaclasses-1)

メタクラスとは、GOOPSクラスを表すオブジェクトのクラスのことです。より簡潔に言うと、メタクラスとはクラスのクラスです。

ほとんどのGOOPSクラスはメタクラス`<class>`を持ち、デフォルトでは`define-class`を使用して作成された新しいクラスはすべてメタクラス`<class>`を持ちます。

しかし、これは一体どういう意味なのでしょうか？それを知るために、`define-class`を使って新しいクラスを作成する際に何が起こるのかを詳しく見ていきましょう。

(define-class <my-class> (<object>) . slots)

ガイルはこれを次のように展開します。

(define <my-class> (class (<object>) . slots))

これはさらに以下のように展開されます。

(define <my-class>
(make <class> #:dsupers (list <object>) #:slots slots))

この展開から明らかなように、`<my-class>` の結果として得られる値は、クラス `<class>` のインスタンスであり、スロット値はクラス`<my-class>` のスーパークラスとスロット定義を指定します。（`#:dsupers` と `#:slots` は、`<class>` クラスの `dsupers` スロットと `dslots` スロットの初期化キーワードです。）

ここで、デフォルトの`<class>`以外のメタクラスを持つ新しいクラスを定義したいとします。これは次のように記述することで実現できます。

(define-class <my-class2> (<object>)
スロット...
#:metaclass <my-metaclass>)

そしてガイルはこれを次のように展開します。

(define <my-class2>
(make <my-metaclass> #:dsupers (list <object>) #:slots slots))

この場合、`<my-class2>` の値は、より特殊化されたクラス `<my-metaclass>` のインスタンスです。`<my-metaclass>` 自体は、事前に `<class>` のサブクラスとして定義されている必要があることに注意してください。新しいメタクラスを定義することがいつ、どのように役立つかについての詳細は、[MOP 仕様](https://doc.guix.gnu.org/guile/latest/en/guile.html#MOP-Specification) を参照してください。

それでは、`<my-class2>`のインスタンスを作成しましょう。

(my-object を定義します (make <my-class2> ...))

以下の記述はすべて、`my-object`、`<my-class2>`、`<my-metaclass>`、および`<class>`間の関係を正しく表現したものです。

* `my-object` はクラス `<my-class2>` のインスタンスです。
* `<my-class2>` はクラス `<my-metaclass>` のインスタンスです。
* `<my-metaclass>` はクラス `<class>` のインスタンスです。
* `my-object` のクラスは `<my-class2>` です。
* `<my-class2>` のクラスは `<my-metaclass>` です。
* `<my-metaclass>` のクラスは `<class>` です。

* * *

次へ: [インスタンス作成プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instance-Creation-Protocol)、前: [メタクラス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Metaclasses)、上: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.3 MOP仕様 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#MOP-Specification-1)

本章におけるMOP仕様の目的は、標準GOOPS構文、プロシージャ、メソッドによって実行可能な、カスタマイズ可能な汎用関数呼び出しをすべて規定し、そのような呼び出しをカスタマイズするためのプロトコルを説明することである。

汎用関数呼び出しは、それが適用される引数の型が、呼び出しが現れる語彙的コンテキストによって完全に決定されない場合にカスタマイズ可能です。たとえば、デフォルトの `make-instance` メソッド内の `(initialize instance initargs)` 呼び出しはカスタマイズ可能です。なぜなら、`instance` 引数の型は `make-instance` に渡されたクラスによって決定されるからです。

（一方、反例を挙げると、`define-generic` の `(make <generic> #:name ',name)` という呼び出しは、すべての引数が字句的に型が決定されているため、カスタマイズできません。）

このルールを使用して、特定の汎用関数呼び出しがカスタマイズ可能かどうかを判断する場合、メソッド定義で単一の「残り」リスト引数として処理されることが想定されている引数は無視します。

カスタマイズ可能な汎用関数呼び出しごとに、呼び出しプロトコルが以下のように説明されます。

* 概念的に、適用された方法は何を意図しているのか
呼び出し元が適用したメソッドの副作用についてどのような仮定を置いているか（もしあれば）。
呼び出し元が適用されたメソッドの戻り値として期待するもの。

* * *

次へ: [クラス定義プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition-Protocol)、前: [MOP仕様](https://doc.guix.gnu.org/guile/latest/en/guile.html#MOP-Specification)、上: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.4 インスタンス作成プロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instance-Creation-Protocol-1)

`make <class> . initargs` (メソッド)

* `allocate-instance class initargs` (汎用)
    
適用された `allocate-instance` メソッドは、クラスの新しいインスタンスのストレージを割り当て、初期化されていないインスタンスを返す必要があります。
    
* `インスタンス初期化引数`（汎用）
    
instance は、`allocate-instance` によって返される初期化されていないインスタンスです。適用されたメソッドは、そのクラスに適した方法で新しいインスタンスを初期化する必要があります。メソッドの戻り値は無視されます。
    

`make`自体は汎用関数です。そのため、新しいインスタンスのメタクラスがデフォルトの`<class>`よりも特殊化されている場合は、そのメタクラスに特化した`make`メソッドを定義することで、`make`の呼び出し自体をカスタマイズできます。

しかし通常は、メタクラス`<class>`を持つクラスにはメソッドが適用されます。このメソッドは、次の2つの汎用関数を呼び出します。

* (allocate-instance class . initargs)
* (インスタンスの初期化 . initargs)

`allocate-instance` は、初期化されていない新しいインスタンスのストレージを割り当てて返します。たとえば、他のオブジェクトプログラミングシステムをGOOPSでラップしたい場合などに、`allocate-instance` をカスタマイズできます。

これを行うには、他のシステムからのすべてのクラスとインスタンスのメタクラスとして機能する、専用のメタクラスを作成します。次に、そのメタクラスに特化した`allocate-instance`メソッドを定義します。このメソッドは、GuileのプリミティブC関数（またはFFIコード）を呼び出し、その関数が他のオブジェクトシステムのインターフェースを使用して新しいインスタンスを割り当てます。

この場合、完全なシステムを構築するには、`make`や`initialize`などの汎用関数もカスタマイズする必要があります。そうすることで、GOOPSが他のシステムからクラスを作成したり、インスタンススロットにアクセスしたりする方法を知ることができます。

`initialize`は、`allocate-instance`によって返されるインスタンスを初期化します。標準のGOOPSメソッドは、インスタンスクラスに適した初期化を実行します。

* 最も特殊化されていないレベルでは、型 `<object>` のインスタンスのメソッドは、内部 GOOPS インスタンスの初期化を実行し、スロット定義と initargs に現れるスロット初期化キーワードに従ってインスタンスのスロットを初期化します。
* 型 `<class>` のインスタンスのメソッドは `(next-method)` を呼び出し、その後 [クラス定義プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition-Protocol) で説明されているクラスの初期化を実行します。
* 一般的な関数、メソッド、演算子クラスなどについても同様です。

同様に、アプリケーションで定義されたクラスのインスタンスの初期化をカスタマイズするには、そのクラス専用の `initialize` メソッドを定義します。

インスタンスの作成時にデータベースを照会してインスタンスのスロットを初期化する必要があるクラスを想像してみてください。スロット定義で`#:init-thunk`キーワードとクロージャを組み合わせることでこれを実現できるかもしれませんが、データベースを一度照会して、その結果に基づいて依存するすべてのスロット値を初期化する`initialize`メソッドをクラスに記述する方が簡潔かもしれません。

* * *

次へ: [クラス定義のカスタマイズ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Customizing-Class-Definition)、前: [インスタンス作成プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instance-Creation-Protocol)、上: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.5 クラス定義プロトコル [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition-Protocol-1)

以下は、クラス定義に関わる可能性のある構文、手順、および汎用関数の概要図です。

`define-class`（構文）

* `class`（構文）
* `make-class`（プロシージャ）
* `ensure-metaclass`（手順）
* `make metaclass …` (汎用)
* `allocate-instance` (汎用)
* `初期化`（汎用）
* `compute-cpl` (汎用)
* `compute-std-cpl`（プロシージャ）
* `compute-slots` (汎用)
* `compute-get-n-set` (汎用)
* `compute-getter-method` (汎用)
* `compute-setter-method` (汎用)
* `クラス再定義`（汎用）
* `remove-class-accessors` (汎用)
* `update-direct-method!` (汎用)
* `update-direct-subclass!` (汎用)

上のステップが「汎用」とマークされている場合、それはカスタマイズ可能であり、その下に示されている詳細は、その汎用関数のデフォルトメソッドの動作を説明する限りにおいてのみ「正しい」と言えます。たとえば、あるメタクラスに対して、`next-method` も `compute-cpl` も呼び出さない `initialize` メソッドを作成した場合、そのメタクラスでクラスが定義されたときに `compute-cpl` は呼び出されません。

`(define-class ...)` 形式 ([クラス定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition) を参照) は、次の式に展開されます。

* トップレベルでのみ評価されていることを確認します。
* スロット定義によって暗黙的に示されるアクセサーを定義します
* `class` を使用して新しいクラスを作成します
* 名前の以前のクラス定義をチェックし、見つかった場合は、`class-redefinition` を呼び出して再定義を処理します ([クラスの再定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Redefining-a-Class) を参照)。

構文: **クラス**名 (スーパークラス …) スロット定義 … クラスオプション … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class-1)

スーパークラスを継承し、スロット定義とクラスオプションで直接スロットが定義された、新しく作成されたクラスを返します。スロット定義とクラスオプションの形式については、[define-class](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition)を参照してください。

`class` は、

* クラスとスロットの定義オプションを処理し、それらが整形式であることを確認し、`#:init-form` オプションを `#:init-thunk` オプションに変換し、デフォルトの環境パラメータ (現在のトップレベル環境) を提供し、評価が必要なすべてのビットを評価します。
* 処理および評価されたパラメータを使用してクラスを作成するために `make-class` を呼び出します。

手順: **make-class** supers slots class-option … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dclass)

スーパークラスを継承し、スロットとクラスオプションで直接スロットが定義された、新しく作成されたクラスを返します。スロットとクラスオプションの形式については、[define-class](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition)を参照してください。ただし、`make-class` の場合、スロットはスロット定義の別のリストであることに注意してください。

`make-class`

* supers リストが空の場合、または supers 内のどのクラスもクラス優先順位リストに `<object>` を持っていない場合、`<object>` を supers リストに追加します。
* オプションで指定されていない場合、`#:environment`、`#:name`、`#:metaclass` オプションのデフォルト値をそれぞれ現在のトップレベル環境、未バインド値、および `(ensure-metaclass supers)` に設定します。
* スーパークラス内の重複クラスとスロット内の重複スロット名をチェックし、重複がある場合はエラーを通知します。
* `make` を呼び出し、メタクラスを最初のパラメータとして、その他のすべてのパラメータを値を持つオプションキーワードとして渡します。

手順: **ensure-metaclass** supers env [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ensure_002dmetaclass)

スーパークラスで指定されたクラスを継承するクラスに適したメタクラスを返します。返されるメタクラスは、スーパークラスで指定されたクラスのメタクラスを継承によって結合したものです。

最も単純なケースでは、すべてのスーパークラスがメタクラス `<class>` を持つ単純なクラスである場合、返されるメタクラスは単に `<class>` になります。

より複雑な例として、supers にメタクラス `<operator-class>` を持つクラスとメタクラス `<foreign-object-class>` を持つクラスがそれぞれ 1 つずつ含まれているとします。この場合、返されるメタクラスは `<operator-class>` と `<foreign-object-class>` の両方から継承するクラスになります。

supersが空のリストの場合、`ensure-metaclass`はデフォルトのGOOPSメタクラス`<class>`を返します。

GOOPSは`ensure-metaclass`によって作成されたメタクラスのリストを保持するため、必要なメタクラスのタイプはそれぞれ一度だけ作成すれば済みます。

`env` パラメータは無視されます。

汎用: **make** メタクラス初期化… [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make-3)

metaclass は、定義対象のクラスのメタクラスであり、`#:metaclass` クラスオプションから取得するか、`ensure-metaclass` によって計算されます。適用されたメソッドは、新しいクラス定義に対して完全に初期化されたクラスメタオブジェクトを作成して返す必要があります。

`(make metaclass initarg …)` 呼び出しは、前のセクションで説明したインスタンス作成プロトコルの特殊なケースです。これは、メタクラス metaclass を持つクラスメタオブジェクトを作成します。デフォルトでは、このメタオブジェクトは、型 `<class>` のインスタンス用に特別に設計された `initialize` メソッドによって初期化されます。

クラスの `initialize` メソッド (シグネチャ `(initialize <class> initargs)`) は、以下の汎用関数を呼び出します。

* `compute-cpl クラス` (汎用)
    
適用されたメソッドは、クラスのクラス優先順位リストを計算し、クラスメタオブジェクトのリストとして返します。`compute-cpl`が呼び出されると、次のクラスメタオブジェクトスロットがすべて初期化されます：`name`、`direct-supers`、`direct-slots`、`direct-subclasses`（空）、`direct-methods`。`compute-cpl`によって返された値は、`cpl`スロットに格納されます。
    
* `compute-slots クラス` (汎用)
    
適用されたメソッドは、クラスのスロット（直接スロットと継承スロットの和集合）を計算し、スロット定義のリストとして返します。`compute-slots`が呼び出されると、`compute-cpl`で指定されたすべてのクラスメタオブジェクトスロットに加え、 `cpl`、`redefined`（`#f`）、`environment`が初期化されます。`compute-slots`によって返された値は、`slots`スロットに格納されます。
    
* `compute-get-n-set class slot-def` (汎用)
    
`initialize` は、`compute-slots` によって計算された各スロットに対して `compute-get-n-set` を呼び出します。適用されたメソッドは、それぞれ指定されたスロットの値を取得および設定するクロージャのペアを計算して返す必要があります。get クロージャは引数 1 を持ち、スロット値を取得するインスタンスを引数として 1 つ受け取ります。set クロージャは引数 2 を持ち、2 つの引数を受け取ります。最初の引数はスロット値を設定するインスタンス、2 番目の引数はそのスロットの新しい値です。クロージャは 2 つの要素を持つリスト `(list get set)` として返されます。
    
`compute-get-n-set` によって返されるクロージャは、クラスメタオブジェクトの `getters-n-setters` スロットの値の一部として格納されます。具体的には、このスロットの値は、クラスのスロットの数と同じ数の要素を持つリストであり、各要素は次のような形式になります。
    
(スロット名-シンボル-初期化関数.インデックス)
    
または、
    
(スロット名シンボル初期化関数get設定)
    
get クロージャと set クロージャが index に置き換えられている場合、スロットはインスタンス スロットであり、index は基となる構造におけるスロットのインデックスです。GOOPS はこのようなスロットの値を取得および設定する方法を知っているため、特別に構築された get クロージャと set クロージャは必要ありません。それ以外の場合、get と set は `compute-get-n-set` によって返されるクロージャです。
    
`getters-n-setters`スロット値の構造は、`initialize`呼び出しを行う次のカスタマイズ可能な汎用関数を理解する上で重要です。
    
* `compute-getter-method クラス gns` (汎用)
    
`initialize` は、`compute-slots` で指定されるクラスの各スロット（`#:getter` または `#:accessor` スロットオプションを含む）に対して `compute-getter-method` を呼び出します。gns は、クラスメタオブジェクトの `getters-n-setters` スロットの要素で、上記の`compute-get-n-set` で説明されているように、問題のスロットがどのように参照され、設定されるかを指定します。適用されたメソッドは、型 class のインスタンスに特化し、get クロージャを使用してスロットの値を取得するメソッドを作成して返す必要があります。`initialize` は `add-method!` を使用して、返されたメソッドをスロット定義の `#:getter` または `#:accessor` オプションで指定された汎用関数に追加します。
    
* `compute-setter-method クラス gns` (汎用)
    
`compute-setter-method` は、`#:setter` または `#:accessor` スロットオプションを含むクラスの各スロットに対して、`compute-getter-method` と同じ引数で呼び出されます。適用されるメソッドは、型 class のインスタンスに特化し、セットクロージャを使用してスロットの値を設定するメソッドを作成して返す必要があります。次に、`initialize` は `add-method!` を使用して、返されたメソッドをスロット定義の `#:setter` または `#:accessor` オプションで指定された汎用関数に追加します。
    

* * *

次へ: [メソッド定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Method-Definition)、前: [クラス定義プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition-Protocol)、上: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.6 クラス定義のカスタマイズ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Customizing-Class-Definition-1)

新しいクラスのメタクラスがデフォルトの`<class>`よりも特殊化されている場合、上記の呼び出しにおけるクラスの型は`<class>`よりも特殊化され、その結果、新しいクラスのメタクラスに特化した汎用関数メソッドを定義することが可能になり、`initialize`、`compute-cpl`、または`compute-get-n-set`のデフォルトの動作を変更またはオーバーライドできるようになります。

`compute-cpl` は、新しいクラスのクラス優先順位リスト (「CPL」) を計算し ([クラス優先順位リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Precedence-List) を参照)、クラスオブジェクトのリストとして返します。CPL は、クラスのインスタンスに対して汎用関数が呼び出されたときに、利用可能な汎用関数メソッドのうちどれが最も具体的かを決定するために使用されるスーパークラスの順序を定義するため重要です。したがって、`compute-cpl` は、特別なメタクラスを持つすべてのクラスの CPL 順序付けアルゴリズムを変更するようにカスタマイズできます。

デフォルトのCPLアルゴリズムは、デフォルトの`compute-cpl`メソッドによって呼び出される`compute-std-cpl`プロシージャによってカプセル化されています。

手順: **compute-std-cpl** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-compute_002dstd_002dcpl)

[クラス優先順位リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Precedence-List)で説明されているアルゴリズムに従って、クラスのクラス優先順位リストを計算して返します。

`compute-slots` は、新しいクラスのすべてのスロット定義のリストを計算して返します。デフォルトでは、このリストには `define-class` フォームからの直接のスロット定義と、新しいクラスのスーパークラスから継承されたスロット定義が含まれます。デフォルトの `compute-slots` メソッドは、`compute-cpl` によって計算された CPL を使用して、このスロット定義の和集合を計算します。このとき、スーパークラスから継承されたスロットは、同じ名前の直接のスロットによって隠蔽されるというルールが適用されます。`compute-slots` をカスタマイズする理由の 1 つは、スロット名の競合に対する別の解決戦略を実装することです。

`compute-get-n-set` は、特定のスロットの値を取得および設定するために使用される低レベルのクロージャを計算し、2 つの要素を持つリストとして返します。

返されるクロージャは、そのスロットのストレージがどのように割り当てられるかによって異なります。標準の `compute-get-n-set` メソッドは、型 `<class>` のクラスに特化されており、`#:allocation` スロットオプションの標準 GOOPS 値を処理します ([allocation](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照)。より特化したメタクラス用に新しい `compute-get-n-set` メソッドを定義することで、新しいタイプのスロット割り当てをサポートできます。

あるクラスのインスタンスを多数作成する場合、そのクラスのインスタンスの一部（例えば、10個のインスタンスごとに1つ）でスロットを共有したいとします。次の例では、これを実現するために新しいタイプのスロット割り当てを実装および使用する方法を示します。

(define-class <batched-allocation-metaclass> (<class>))

(let ((batch-allocation-count 0)
(バッチ取得・設定 #f))
(define-method (compute-get-n-set
(クラス <batched-allocation-metaclass>) s)
(case (slot-definition-allocation s)
（（#:バッチ処理済み））
;; 既に同じスロットストレージを10個のインスタンスで使用している場合、
;; 変数をリセットします。
(if (= バッチ割り当て数 10)
（始める
(set! batch-allocation-count 0)
(set! batch-get-n-set #f)))
;; 現在有効な get クロージャと set クロージャのペアがない場合、
;; 1つ作成します。make-closure-variableはクロージャのペアを返します
;; 単一の Scheme 変数を中心にします - 詳細については goops.scm を参照してください。
（またはバッチ取得と設定）
(set! batch-get-n-set (make-closure-variable)))
バッチ割り当て数をインクリメントします。
(set! batch-allocation-count (+ batch-allocation-count 1))
バッチ取得と設定)

;; 標準的な割り当てタイプを処理するには、次のメソッドを呼び出します。
(else (next-method)))))

(define-class <class-using-batched-lot> ()
...
(c #:割り当て #:バッチ処理)
...
#:metaclass <batched-allocation-metaclass>)

`compute-getter-method` と `compute-setter-method` の使用方法については、[クラス定義プロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Definition-Protocol) に記載されています。

`compute-cpl` と `compute-get-n-set` は、メタクラスが `<class>` であるクラスの標準の `initialize` メソッドによって呼び出されます。しかし、`initialize` メソッド自体も、新しいクラスのメタクラスに特化した `initialize` メソッドを定義することで変更できます。このようなメソッドは、`(next-method)` をまったく呼び出さないことで標準の動作を完全にオーバーライドできますが、より一般的には、標準の動作のために `(next-method)` を呼び出す前や後に、追加のクラス初期化手順を実行します。

* * *

次へ: [メソッド定義の内部構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#Method-Definition-Internals)、前: [クラス定義のカスタマイズ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Customizing-Class-Definition)、上: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.7 メソッド定義 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Method-Definition-1)

`define-method`（構文）

* `add-method! 対象メソッド` (汎用)

`define-method` は、新しいメソッドをさまざまな対象に追加する処理を処理する汎用関数 `add-method!` を呼び出します。GOOPS には、対象を処理するメソッドが含まれています。

* 汎用関数（最も一般的なケース）
* 手順
* プリミティブジェネリック（[Extending Primitives](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-Primitives)を参照）

`add-method!` にさらにメソッドを定義することで、理論的には、より多くの種類のターゲットにメソッドを追加する処理を処理できるようになります。

* * *

次へ: [汎用関数の内部構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-Internals)、前: [メソッド定義](https://doc.guix.gnu.org/guile/latest/en/guile.html#Method-Definition)、上: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.8 メソッド定義の内部 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Method-Definition-Internals-1)

`define-method`:

* 最初のパラメータの形式をチェックし、アクセサのセッターが `(setter …)` 形式の場合は、以下の手順をセッターに適用します。
指定された名前の汎用関数がまだ定義されていない場合、`define-generic` または `define-accessor` の呼び出しを補間します。
* パラメータと本体を指定して `method` を呼び出し、新しいメソッドインスタンスを作成します。
* `add-method!` を呼び出して、このメソッドを関連する汎用関数に追加します。

構文: **メソッド** (パラメータ …) 本体 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-method)

パラメータ内のクラスによって特殊化が定義され、パラメータシンボルと本体形式からプロシージャ定義が構築されるメソッドを作成します。

パラメータとボディパラメータは、`define-method` と同様である必要があります（[define-method](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions) を参照）。

`メソッド`:

* パラメータから形式変数と特殊化クラスを抽出し、特殊化されていないパラメータのクラスをデフォルトで `<top>` に設定します。
* 形式と本文形式を使用してクロージャを作成します
* メタクラス `<method>` と、`#:specializers` および `#:procedure` キーワードを使用したスペシャライザとクロージャを使用して `make` を呼び出します。

手順: **make-method** 特殊化手順 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dmethod)

専門家と手順を用いて方法を作成する。

スペシャライザは、このメソッドが適用されるパラメータの組み合わせを指定するクラスのリストである必要があります。

このメソッドが呼び出されたときに、汎用関数のパラメータに適用されるクロージャが、このプロシージャであるべきです。

`make-method` は、メタクラス `<method>` を持つ `make` のシンプルなラッパーです。

汎用: **add-method!** 対象メソッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dmethod_0021)

メソッドメソッドをターゲットに追加するための汎用関数。

メソッド: **add-method!** (generic <generic>) (method <method>) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dmethod_0021-1)

汎用関数 generic にメソッド method を追加します。

メソッド: **add-method!** (proc <プロシージャ>) (method <メソッド>) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dmethod_0021-2)

proc がジェネリック機能を持つプロシージャである場合 ([generic-capability?](https://doc.guix.gnu.org/guile/latest/en/guile.html#Extending-Primitives) を参照)、それをプリミティブジェネリックにアップグレードし、そのジェネリック関数定義にメソッドを追加します。

メソッド: **add-method!** (pg <primitive-generic>) (method <method>) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dmethod_0021-3)

pg の汎用関数定義にメソッド method を追加します。

実装: `(add-method! (primitive-generic-generic pg) method)`。

メソッド: **add-method!** (whatever <top>) (method <method>) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-add_002dmethod_0021-4)

指定された関数が有効な汎用関数ではないことを示すエラーを発生させます。

* * *

次へ: [汎用関数呼び出し](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-Invocation)、前: [メソッド定義の内部構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#Method-Definition-Internals)、上: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.9 汎用関数の内部構造[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-Internals-1)

`define-generic` は、既存のプロシージャ値をアップグレードするために `ensure-generic` を呼び出すか、新しい汎用関数を作成するためにメタクラス `<generic>` を指定して `make` を呼び出します。

`define-accessor` は、既存のプロシージャ値をアップグレードするために `ensure-accessor` を呼び出すか、新しいアクセサーを作成するために `make-accessor` を呼び出します。

手順: **ensure-generic** 旧定義 \[name\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ensure_002dgeneric)

可能であれば、old-definition を使用またはアップグレードして、name という名前の汎用関数を返します。指定されていない場合、name のデフォルト値は `#f` になります。

old-definitionが既に汎用関数である場合は、変更されずに返されます。

old-definitionがSchemeの手続きまたはセッター付き手続きである場合、`ensure-generic`は、デフォルトの手続きとセッターにold-definitionを使用する新しい汎用関数を返します。

それ以外の場合、`ensure-generic`はデフォルト値もメソッドも持たない新しい汎用関数を返します。

手順: **make-generic** \[name\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dgeneric)

`(車名)` という名前の新しい汎用関数を返します。指定しない場合、デフォルトで `#f` になります。

`ensure-generic` は、アップグレードしようとしている変数の以前の値に応じて、メタクラス `<generic>` と `<generic-with-setter>` を使用して `make` を呼び出します。

`make-generic` は、メタクラス `<generic>` を持つ `make` のシンプルなラッパーです。

プロシージャ: **ensure-accessor** proc \[name\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-ensure_002daccessor)

可能であれば、proc を使用またはアップグレードして、name という名前のアクセサーを返します。指定されていない場合、name のデフォルト値は `#f` になります。

procが既にアクセサーである場合、変更されずに返されます。

proc が Scheme プロシージャ、セッター付きプロシージャ、または汎用関数である場合、`ensure-accessor` は proc の再利用可能な要素を再利用するアクセサーを返します。

それ以外の場合、`ensure-accessor` はデフォルト値もメソッドも持たない新しいアクセサーを返します。

手順: **make-accessor** \[name\] [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002daccessor)

`(車名)` という名前の新しいアクセサーを返します。指定しない場合は、デフォルトで `#f` になります。

`ensure-accessor` は、メタクラス `<generic-with-setter>` を指定して `make` を呼び出し、さらに `ensure-generic`、`make-accessor`、および (末尾再帰的に) `ensure-accessor` を呼び出します。

`make-accessor` は `make` を 2 回呼び出します。最初はメタクラス `<generic>` を使用してセッター用の汎用関数を作成し、次にメタクラス `<generic-with-setter>` を使用してアクセサーを作成し、セッターの汎用関数を `#:setter` キーワードの値として渡します。

* * *

前へ: [汎用関数の内部構造](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-Internals)、上へ: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.11.10 汎用関数呼び出し [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-Invocation-1)

汎用関数を呼び出すプロセス、つまり、汎用関数のどのメソッドが現在の引数に適用可能か、そしてどのメソッドを適用するかを決定するプロセスには、詳細かつカスタマイズ可能なプロトコルが関わっています。以下に、関連する汎用関数の概要図を示します。

`apply-generic`（汎用）

* `no-method` (汎用)
* `compute-applicable-methods` (汎用)
* `sort-applicable-methods` (汎用)
* `メソッドをより具体的に?` (汎用)
* `apply-methods` (汎用)
* `apply-method`（汎用）
* `no-next-method` (汎用)
* `適用できないメソッド`

これらに関する完全なドキュメントはまだ用意できていません。詳細はコード（oop/goops.scm）を参照してください。

* * *

次へ: [インスタンスのクラスの変更](https://doc.guix.gnu.org/guile/latest/en/guile.html#Changing-the-Class-of-an-Instance)、前: [メタオブジェクトプロトコル](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-Metaobject-Protocol)、上: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
