### 8.8 イントロスペクション

イントロスペクション、またはリフレクションとは、GOOPSオブジェクトに関する情報を動的に取得できることを意味します。イントロスペクション機能を持たないオブジェクト指向言語、すなわちC++を例に挙げると、この概念が最もよく理解できるでしょう。

C++には、実行中のプログラムが以下の種類の質問に対する答えを得ることを可能にする機能は何もありません。

* このオブジェクトまたはクラスのデータメンバーは何ですか？
このクラスはどのクラスから継承していますか？
* このメソッド呼び出しは仮想呼び出しですか、それとも非仮想呼び出しですか？
* `Employee::adjustHoliday()` を呼び出した場合、適用される `adjustHoliday()` メソッドを含むクラスは何ですか？

C++では、このような疑問に対する答えは、ソースコードにアクセスできれば、それを見なければ分かりません。一方、GOOPSには、これらの疑問に対する答え（あるいはGOOPSにおける同等の疑問）を、実行時に動的に取得できる手続きが用意されています。

* [クラス](#881-クラス)
* [インスタンス](#882-インスタンス)
* [スロット](#883-スロット)
* [汎用関数](#884-汎用関数)
* [スロットへのアクセス](#885-スロットへのアクセス)

* * *

次へ: [インスタンス](#882-インスタンス)、上へ: [イントロスペクション](#88-イントロスペクション) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 8.8.1 クラス

GOOPS クラスは、それ自体が `<class>` クラスのインスタンス、または `<class>` のサブクラスのインスタンスです。`<class>` クラスの定義には、クラスのプロパティを記述するために使用されるスロットがあり、以下が含まれます。

プリミティブプロシージャ: **class-name** クラス

クラスの名前を返します。これは、クラスの `name` スロットの値です。

プリミティブ手続き: **class-direct-supers** クラス

クラスの直接のスーパークラスを含むリストを返します。これは、クラスの`direct-supers`スロットの値です。

プリミティブプロシージャ: **class-direct-slots** クラス

クラスの直接スロットのスロット定義を含むリストを返します。これは、クラスの`direct-slots`スロットの値です。

プリミティブプロシージャ: **class-direct-subclasses** クラス

クラスの直接のサブクラスを含むリストを返します。これは、クラスの`direct-subclasses`スロットの値です。

プリミティブプロシージャ: **class-direct-methods** クラス

クラスを仮引数特殊化子として使用するすべての汎用関数メソッドのリストを返します。これは、クラスの `direct-methods` スロットの値です。

プリミティブ手続き: **クラス優先順位リスト** クラス

クラス class のクラス優先順位リストを返します ([クラス優先順位リスト](08_07_inheritance.md#871-クラス優先順位リスト) を参照)。これは、クラスの `cpl` スロットの値です。

プリミティブプロシージャ: **class-slots** クラス

スーパークラスから継承されたスロットも含め、クラスのすべてのスロットの定義を含むリストを返します。これは、クラスの`slots`スロットの値です。

手順: **class-subclasses** クラス

クラスのすべてのサブクラスのリストを返します。

手順: **class-methods** クラス

クラスまたはクラスのサブクラスを仮引数特殊化子の1つとして使用するすべてのメソッドのリストを返します。

* * *

次へ: [スロット](#883-スロット)、前: [クラス](#881-クラス)、上: [イントロスペクション](#88-イントロスペクション) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 8.8.2 インスタンス

プリミティブ手続き: **class-of** 値

任意のScheme値に対応するGOOPSクラスを返します。

プリミティブプロシージャ: **instance?** オブジェクト

オブジェクトがGOOPSインスタンスであれば`#t`を返し、そうでなければ`#f`を返します。

手順: **is-a?** オブジェクトクラス

オブジェクトがクラスまたはそのサブクラスのインスタンスである場合は、`#t` を返します。

`is-a?`述語を使用すると、特定の値が特定のクラスに属するかどうかを問い合わせることができ、`class-of`を使用すると、特定の値のクラスを調べることができます。GOOPSがロードされると（`(oop goops)`モジュールを使用するコードによって）、`<string>`、`<list>`、`<number>`などの組み込みクラスが自動的に設定され、すべてのGuile Scheme型に対応することに注意してください。

([is-a?](#882-インスタンス) 2.3 <番号>) ⇒ #t
([is-a?](#882-インスタンス) 2.3 <real>) ⇒ #t
([is-a?](#882-インスタンス) 2.3 <string>) ⇒ #f
([is-a?](#882-インスタンス) '("a" "b") <string>) ⇒ #f
([is-a?](#882-インスタンス) '("a" "b") <list>) ⇒ #t
([is-a?](#882-インスタンス) ([car](06_06_08_pairs.md#668-ペア) '("a" "b")) <string>) ⇒ #t
([is-a?](#882-インスタンス) <string> <class>) ⇒ #t
([is-a?](#882-インスタンス) <class> <string>) ⇒ #f

([class-of](#882-インスタンス) 2.3) ⇒ #<<class> <real> 908c708>
([class-of](#882-インスタンス) #(1 2 3)) ⇒ #<<class> <vector> 908cd20>
([class-of](#882-インスタンス) <string>) ⇒ #<<class> <class> 8bd3e10>
([class-of](#882-インスタンス) <class>) ⇒ #<<class> <class> 8bd3e10>

* * *

次へ: [汎用関数](#884-汎用関数)、前: [インスタンス](#882-インスタンス)、上: [イントロスペクション](#88-イントロスペクション) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 8.8.3 スロット

手順: **class-slot-definition** クラス スロット名

クラス class 内の slot-name という名前のスロットのスロット定義を返します。slot-name はシンボルである必要があります。

手順: **slot-definition-name** slot-def

slot-defからスロット名を抽出して返します。

手順: **slot-definition-options** slot-def

slot-defからスロットオプションを抽出して返します。

手順: **スロット定義割り当て** slot-def

slot-def からスロット割り当てオプションを抽出して返します。これは、`#:allocation` キーワードの値（[allocation](08_04_slot_options.md#84-スロットオプション) を参照）か、`#:allocation` キーワードが存在しない場合は `#:instance` です。

プロシージャ: **スロット定義-getter** スロット定義

slot-def からスロット ゲッター オプションを抽出して返します。これは、`#:getter` キーワードの値です ([getter](08_04_slot_options.md#84-スロットオプション) を参照)。`#:getter` キーワードが存在しない場合は `#f` になります。

プロシージャ: **スロット定義セッター** スロット定義

slot-def からスロットセッターオプションを抽出して返します。これは、`#:setter` キーワードの値です ([setter](08_04_slot_options.md#84-スロットオプション) を参照)。`#:setter` キーワードが存在しない場合は `#f` になります。

手順: **slot-definition-accessor** slot-def

slot-def からスロットアクセサオプションを抽出して返します。これは、`#:accessor` キーワードの値です ([accessor](08_04_slot_options.md#84-スロットオプション) を参照)。`#:accessor` キーワードが存在しない場合は `#f` になります。

プロシージャ: **スロット定義-初期値** スロット定義

slot-def から slot init-value オプションを抽出して返します。これは、`#:init-value` キーワードの値（[init-value](08_04_slot_options.md#84-スロットオプション) を参照）か、`#:init-value` キーワードが存在しない場合は未バインドの値です。

手順: **slot-definition-init-form** slot-def

slot-def から slot init-form オプションを抽出して返します。これは、`#:init-form` キーワードの値（[init-form](08_04_slot_options.md#84-スロットオプション) を参照）か、`#:init-form` キーワードが存在しない場合は未定義の値です。

手順: **slot-definition-init-thunk** slot-def

slot-def からスロット init-thunk オプションを抽出して返します。これは、`#:init-thunk` キーワードの値です ([init-thunk](08_04_slot_options.md#84-スロットオプション) を参照)。`#:init-thunk` キーワードが存在しない場合は `#f` になります。

手順: **slot-definition-init-keyword** slot-def

slot-def から slot init-keyword オプションを抽出して返します。これは `#:init-keyword` キーワードの値です ([init-keyword](08_04_slot_options.md#84-スロットオプション) を参照)。`#:init-keyword` キーワードが存在しない場合は `#f` になります。

手順: **slot-init-function** クラス slot-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- slot_002dinit_002dfunction)

クラス class 内の slot-name という名前のスロットの初期化関数を返します。slot-name はシンボルである必要があります。

返される初期化関数には、標準の `#:init-thunk`、`#:init-form`、および `#:init-value` スロットオプションの効果が組み込まれています。これらの初期化は、`#:init-keyword` スロットオプションまたは特殊な `initialize` メソッドによって上書きできるため、一般的に `slot-init-function` によって返される関数は無関係な場合があります。詳細については、[init-value](08_04_slot_options.md#84-スロットオプション) を参照してください。

* * *

次へ: [スロットへのアクセス](#885-スロットへのアクセス)、前: [スロット](#883-スロット)、上: [イントロスペクション](#88-イントロスペクション) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 8.8.4 汎用関数

ジェネリック関数は、`<generic>` クラス、または `<generic>` のサブクラスのインスタンスです。`<generic>` クラスの定義には、ジェネリック関数のプロパティを記述するために使用されるスロットがあります。

プリミティブプロシージャ: **generic-function-name** gf

汎用関数gfの名前を返します。

プリミティブ手続き: **generic-function-methods** gf

汎用関数gfのメソッドの一覧を返します。これはgfの`methods`スロットの値です。

同様に、メソッドは`<method>`クラス、または`<method>`のサブクラスのインスタンスであり、`<method>`クラスの定義には、メソッドのプロパティを記述するために使用されるスロットがあります。

プリミティブプロシージャ: **method-generic-function** メソッド

そのメソッドが属する汎用関数を返します。これは、メソッドの`generic-function`スロットの値です。

プリミティブ手続き: **method-specializers** メソッド

メソッドの仮引数特殊化子のリストを返します。これは、メソッドの `specializers` スロットの値です。

原始プロシージャ: **method-procedure** メソッド

メソッドを実装するプロシージャを返します。これは、メソッドの`procedure`スロットの値です。

汎用: **メソッドソース**

メソッド: **method-source** (m <method>)

メソッドmの定義を示す式を返してください。

(define-generic cube)

(define-method (cube (n <number>))
(* nnn))

(map method-source (generic-function-methods cube))
⇒
((メソッド ((n <数値>)) (\* nnn)))

* * *

前へ: [汎用関数](#884-汎用関数)、上へ: [イントロスペクション](#88-イントロスペクション) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]

#### 8.8.5 スロットへのアクセス

割り当てに関係なく、どのスロットも以下の4つの基本的な手続きを使用して照会、参照、設定できます。

プリミティブプロシージャ: **slot-exists?** obj slot-name

obj に slot-name という名前のスロットがある場合は `#t` を返し、そうでない場合は `#f` を返します。

プリミティブプロシージャ: **スロットバインド?** オブジェクト スロット名

obj 内の slot-name という名前のスロットに値がある場合は `#t` を返し、そうでない場合は `#f` を返します。

`slot-bound?` は、obj に slot-name という名前のスロットがない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](#885-スロットへのアクセス) を参照)。

プリミティブプロシージャ: **slot-ref** obj slot-name

obj 内の slot-name という名前のスロットの値を返します。

`slot-ref` は、obj に slot-name という名前のスロットがない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](#885-スロットへのアクセス) を参照)。

`slot-ref` は、obj 内の名前付きスロットに値がない場合、汎用関数 `slot-unbound` を呼び出します ([slot-unbound](#885-スロットへのアクセス) を参照)。

プリミティブプロシージャ: **slot-set!** obj slot-name value

obj内のslot-nameという名前のスロットの値をvalueに設定します。

`slot-set!` は、obj に slot-name という名前のスロットがない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](#885-スロットへのアクセス) を参照)。

GOOPSはスロットに関する情報をクラスに格納します。内部的には、これらのプロシージャはすべて、クラス`(class-of obj)`内でslot-nameという名前のスロットのスロット定義を検索し、そのスロット定義の「getter」および「setter」クロージャを使用してスロット値を取得および設定することで動作します。

次の4つの手順は、前の手順とは異なり、`(class-of obj)` を前提とするのではなく、クラスを明示的な引数として受け取ります。そのため、あるクラスのスロット定義の「getter」と「setter」クロージャを、別のクラスのインスタンスに適用することができます。

プリミティブプロシージャ: **slot-exists-using-class?** クラス オブジェクト スロット名

クラスに slot-name という名前のスロットのスロット定義がある場合は `#t` を返し、そうでない場合は `#f` を返します。

プリミティブプロシージャ: **slot-bound-using-class?** class obj slot-name

同じ引数に`slot-ref-using-class`を適用した場合に汎用関数`slot-unbound`が呼び出される場合は`#t`を返し、そうでない場合は`#f`を返します。

`slot-bound-using-class?` は、クラスに slot-name という名前のスロットのスロット定義がない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](#885-スロットへのアクセス) を参照)。

プリミティブプロシージャ: **slot-ref-using-class** クラス obj スロット名

クラス内の slot-name という名前のスロットに対する「getter」クロージャを obj に適用し、その結果を返します。

`slot-ref-using-class` は、クラスに slot-name という名前のスロットのスロット定義がない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](#885-スロットへのアクセス) を参照)。

`slot-ref-using-class` は、obj に「getter」クロージャを適用した結果、バインドされていない値が返された場合に、汎用関数 `slot-unbound` を呼び出します ([slot-unbound](#885-スロットへのアクセス) を参照)。

プリミティブプロシージャ: **slot-set-using-class!** class obj slot-name value

クラス内の slot-name という名前のスロットの「セッター」クロージャを obj と value に適用します。

`slot-set-using-class!` は、クラスに slot-name という名前のスロットのスロット定義がない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](#885-スロットへのアクセス) を参照)。

インスタンスごとではなくクラスごとに割り当てられるスロットは、特定のインスタンスを指定することなく参照および設定できます。

手順: **class-slot-ref** クラス スロット名

クラス class 内の slot-name という名前のスロットの値を返します。指定されたスロットには、`#:class` または `#:each-subclass` の割り当てが必要です ([allocation](08_04_slot_options.md#84-スロットオプション) を参照)。

`#:class` または `#:each-subclass` の割り当てを持つスロットが存在しない場合、`class-slot-ref` は引数 class と slot-name を指定して汎用関数 `slot-missing` を呼び出します。それ以外の場合、スロット値が未割り当てであれば、`class-slot-ref` は同じ引数で汎用関数 `slot-unbound` を呼び出します。

手順: **class-slot-set!** クラス スロット名 値

クラス class 内の slot-name という名前のスロットの値を value に設定します。指定されたスロットには、`#:class` または `#:each-subclass` の割り当てが必要です ([allocation](08_04_slot_options.md#84-スロットオプション) を参照)。

`#:class` または `#:each-subclass` の割り当てを持つスロットが存在しない場合、`class-slot-ref` は引数 class と slot-name を指定して汎用関数 `slot-missing` を呼び出します。

`slot-ref` または `slot-set!` 呼び出しで存在しないスロット名が指定された場合、または値がバインドされていないスロットを参照しようとした場合、GOOPS は次の汎用関数のいずれかを呼び出します。

汎用: **スロットが不足しています**

メソッド: **slot-missing** (クラス <class>) slot-name

メソッド: **slot-missing** (クラス <class>) (オブジェクト <object>) slot-name

メソッド: **slot-missing** (クラス <class>) (オブジェクト <object>) slot-name value

アプリケーションがクラスまたはインスタンスのスロットを名前で参照または設定しようとしたときに、指定されたクラスまたはオブジェクトに対してスロット名が無効な場合、GOOPS は汎用関数 `slot-missing` を呼び出します。

デフォルトのメソッドはすべて、適切なメッセージとともに`goops-error`を呼び出します。

汎用: **slot-unbound**

メソッド: **slot-unbound** (オブジェクト <object>)

メソッド: **slot-unbound** (クラス <class>) slot-name

メソッド: **slot-unbound** (class <class>) (object <object>) slot-name

アプリケーションがクラスまたはインスタンスのスロットを参照しようとした際に、そのスロットの値がバインドされていない場合、GOOPS は汎用関数 `slot-unbound` を呼び出します。

デフォルトのメソッドはすべて、適切なメッセージとともに`goops-error`を呼び出します。

* * *

次へ: [GOOPS オブジェクト雑録](08_10_goops_object_miscellany.md#810-goops-オブジェクト雑記)、前: [イントロスペクション](#88-イントロスペクション)、上: [GOOPS](08_00_goops.md#8-goops) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
