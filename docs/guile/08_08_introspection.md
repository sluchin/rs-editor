### 8.8 イントロスペクション [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Introspection-1)

イントロスペクション、またはリフレクションとは、GOOPSオブジェクトに関する情報を動的に取得できることを意味します。イントロスペクション機能を持たないオブジェクト指向言語、すなわちC++を例に挙げると、この概念が最もよく理解できるでしょう。

C++には、実行中のプログラムが以下の種類の質問に対する答えを得ることを可能にする機能は何もありません。

* このオブジェクトまたはクラスのデータメンバーは何ですか？
このクラスはどのクラスから継承していますか？
* このメソッド呼び出しは仮想呼び出しですか、それとも非仮想呼び出しですか？
* `Employee::adjustHoliday()` を呼び出した場合、適用される `adjustHoliday()` メソッドを含むクラスは何ですか？

C++では、このような疑問に対する答えは、ソースコードにアクセスできれば、それを見なければ分かりません。一方、GOOPSには、これらの疑問に対する答え（あるいはGOOPSにおける同等の疑問）を、実行時に動的に取得できる手続きが用意されています。

* [クラス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Classes)
* [インスタンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instances)
* [スロット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slots)
* [汎用関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Functions)
* [スロットへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots)

* * *

次へ: [インスタンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instances)、上へ: [イントロスペクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Introspection) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.8.1 クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Classes-1)

GOOPS クラスは、それ自体が `<class>` クラスのインスタンス、または `<class>` のサブクラスのインスタンスです。`<class>` クラスの定義には、クラスのプロパティを記述するために使用されるスロットがあり、以下が含まれます。

プリミティブプロシージャ: **class-name** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dname)

クラスの名前を返します。これは、クラスの `name` スロットの値です。

プリミティブ手続き: **class-direct-supers** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002ddirect_002dsupers)

クラスの直接のスーパークラスを含むリストを返します。これは、クラスの`direct-supers`スロットの値です。

プリミティブプロシージャ: **class-direct-slots** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002ddirect_002dslots)

クラスの直接スロットのスロット定義を含むリストを返します。これは、クラスの`direct-slots`スロットの値です。

プリミティブプロシージャ: **class-direct-subclasses** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002ddirect_002dsubclasses)

クラスの直接のサブクラスを含むリストを返します。これは、クラスの`direct-subclasses`スロットの値です。

プリミティブプロシージャ: **class-direct-methods** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002ddirect_002dmethods)

クラスを仮引数特殊化子として使用するすべての汎用関数メソッドのリストを返します。これは、クラスの `direct-methods` スロットの値です。

プリミティブ手続き: **クラス優先順位リスト** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dprecedence_002dlist)

クラス class のクラス優先順位リストを返します ([クラス優先順位リスト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Class-Precedence-List) を参照)。これは、クラスの `cpl` スロットの値です。

プリミティブプロシージャ: **class-slots** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dslots)

スーパークラスから継承されたスロットも含め、クラスのすべてのスロットの定義を含むリストを返します。これは、クラスの`slots`スロットの値です。

手順: **class-subclasses** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dsubclasses)

クラスのすべてのサブクラスのリストを返します。

手順: **class-methods** クラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dmethods)

クラスまたはクラスのサブクラスを仮引数特殊化子の1つとして使用するすべてのメソッドのリストを返します。

* * *

次へ: [スロット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slots)、前: [クラス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Classes)、上: [イントロスペクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Introspection) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.8.2 インスタンス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instances-1)

プリミティブ手続き: **class-of** 値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dof)

任意のScheme値に対応するGOOPSクラスを返します。

プリミティブプロシージャ: **instance?** オブジェクト [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-instance_003f)

オブジェクトがGOOPSインスタンスであれば`#t`を返し、そうでなければ`#f`を返します。

手順: **is-a?** オブジェクトクラス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-is_002da_003f)

オブジェクトがクラスまたはそのサブクラスのインスタンスである場合は、`#t` を返します。

`is-a?`述語を使用すると、特定の値が特定のクラスに属するかどうかを問い合わせることができ、`class-of`を使用すると、特定の値のクラスを調べることができます。GOOPSがロードされると（`(oop goops)`モジュールを使用するコードによって）、`<string>`、`<list>`、`<number>`などの組み込みクラスが自動的に設定され、すべてのGuile Scheme型に対応することに注意してください。

([is-a?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-is_002da_003f) 2.3 <番号>) ⇒ #t
([is-a?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-is_002da_003f) 2.3 <real>) ⇒ #t
([is-a?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-is_002da_003f) 2.3 <string>) ⇒ #f
([is-a?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-is_002da_003f) '("a" "b") <string>) ⇒ #f
([is-a?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-is_002da_003f) '("a" "b") <list>) ⇒ #t
([is-a?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-is_002da_003f) ([car](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-car) '("a" "b")) <string>) ⇒ #t
([is-a?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-is_002da_003f) <string> <class>) ⇒ #t
([is-a?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-is_002da_003f) <class> <string>) ⇒ #f

([class-of](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dof) 2.3) ⇒ #<<class> <real> 908c708>
([class-of](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dof) #(1 2 3)) ⇒ #<<class> <vector> 908cd20>
([class-of](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dof) <string>) ⇒ #<<class> <class> 8bd3e10>
([class-of](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dof) <class>) ⇒ #<<class> <class> 8bd3e10>

* * *

次へ: [汎用関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Functions)、前: [インスタンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Instances)、上: [イントロスペクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Introspection) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.8.3 スロット [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slots-1)

手順: **class-slot-definition** クラス スロット名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dslot_002ddefinition)

クラス class 内の slot-name という名前のスロットのスロット定義を返します。slot-name はシンボルである必要があります。

手順: **slot-definition-name** slot-def [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002dname)

slot-defからスロット名を抽出して返します。

手順: **slot-definition-options** slot-def [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002doptions)

slot-defからスロットオプションを抽出して返します。

手順: **スロット定義割り当て** slot-def [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002dallocation)

slot-def からスロット割り当てオプションを抽出して返します。これは、`#:allocation` キーワードの値（[allocation](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照）か、`#:allocation` キーワードが存在しない場合は `#:instance` です。

プロシージャ: **スロット定義-getter** スロット定義 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002dgetter)

slot-def からスロット ゲッター オプションを抽出して返します。これは、`#:getter` キーワードの値です ([getter](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照)。`#:getter` キーワードが存在しない場合は `#f` になります。

プロシージャ: **スロット定義セッター** スロット定義 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002dsetter)

slot-def からスロットセッターオプションを抽出して返します。これは、`#:setter` キーワードの値です ([setter](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照)。`#:setter` キーワードが存在しない場合は `#f` になります。

手順: **slot-definition-accessor** slot-def [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002daccessor)

slot-def からスロットアクセサオプションを抽出して返します。これは、`#:accessor` キーワードの値です ([accessor](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照)。`#:accessor` キーワードが存在しない場合は `#f` になります。

プロシージャ: **スロット定義-初期値** スロット定義 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002dinit_002dvalue)

slot-def から slot init-value オプションを抽出して返します。これは、`#:init-value` キーワードの値（[init-value](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照）か、`#:init-value` キーワードが存在しない場合は未バインドの値です。

手順: **slot-definition-init-form** slot-def [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002dinit_002dform)

slot-def から slot init-form オプションを抽出して返します。これは、`#:init-form` キーワードの値（[init-form](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照）か、`#:init-form` キーワードが存在しない場合は未定義の値です。

手順: **slot-definition-init-thunk** slot-def [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002dinit_002dthunk)

slot-def からスロット init-thunk オプションを抽出して返します。これは、`#:init-thunk` キーワードの値です ([init-thunk](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照)。`#:init-thunk` キーワードが存在しない場合は `#f` になります。

手順: **slot-definition-init-keyword** slot-def [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002ddefinition_002dinit_002dkeyword)

slot-def から slot init-keyword オプションを抽出して返します。これは `#:init-keyword` キーワードの値です ([init-keyword](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照)。`#:init-keyword` キーワードが存在しない場合は `#f` になります。

手順: **slot-init-function** クラス slot-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- slot_002dinit_002dfunction)

クラス class 内の slot-name という名前のスロットの初期化関数を返します。slot-name はシンボルである必要があります。

返される初期化関数には、標準の `#:init-thunk`、`#:init-form`、および `#:init-value` スロットオプションの効果が組み込まれています。これらの初期化は、`#:init-keyword` スロットオプションまたは特殊な `initialize` メソッドによって上書きできるため、一般的に `slot-init-function` によって返される関数は無関係な場合があります。詳細については、[init-value](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照してください。

* * *

次へ: [スロットへのアクセス](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots)、前: [スロット](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slots)、上: [イントロスペクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Introspection) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.8.4 汎用関数 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Functions-1)

ジェネリック関数は、`<generic>` クラス、または `<generic>` のサブクラスのインスタンスです。`<generic>` クラスの定義には、ジェネリック関数のプロパティを記述するために使用されるスロットがあります。

プリミティブプロシージャ: **generic-function-name** gf [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-generic_002dfunction_002dname)

汎用関数gfの名前を返します。

プリミティブ手続き: **generic-function-methods** gf [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-generic_002dfunction_002dmethods)

汎用関数gfのメソッドの一覧を返します。これはgfの`methods`スロットの値です。

同様に、メソッドは`<method>`クラス、または`<method>`のサブクラスのインスタンスであり、`<method>`クラスの定義には、メソッドのプロパティを記述するために使用されるスロットがあります。

プリミティブプロシージャ: **method-generic-function** メソッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-method_002dgeneric_002dfunction)

そのメソッドが属する汎用関数を返します。これは、メソッドの`generic-function`スロットの値です。

プリミティブ手続き: **method-specializers** メソッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-method_002dspecializers)

メソッドの仮引数特殊化子のリストを返します。これは、メソッドの `specializers` スロットの値です。

原始プロシージャ: **method-procedure** メソッド [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-method_002dprocedure)

メソッドを実装するプロシージャを返します。これは、メソッドの`procedure`スロットの値です。

汎用: **メソッドソース** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-method_002dsource)

メソッド: **method-source** (m <method>) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-method_002dsource-1)

メソッドmの定義を示す式を返してください。

(define-generic cube)

(define-method (cube (n <number>))
(* nnn))

(map method-source (generic-function-methods cube))
⇒
((メソッド ((n <数値>)) (\* nnn)))

* * *

前へ: [汎用関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Functions)、上へ: [イントロスペクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Introspection) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]

#### 8.8.5 スロットへのアクセス [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots-1)

割り当てに関係なく、どのスロットも以下の4つの基本的な手続きを使用して照会、参照、設定できます。

プリミティブプロシージャ: **slot-exists?** obj slot-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dexists_003f)

obj に slot-name という名前のスロットがある場合は `#t` を返し、そうでない場合は `#f` を返します。

プリミティブプロシージャ: **スロットバインド?** オブジェクト スロット名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dbound_003f)

obj 内の slot-name という名前のスロットに値がある場合は `#t` を返し、そうでない場合は `#f` を返します。

`slot-bound?` は、obj に slot-name という名前のスロットがない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots) を参照)。

プリミティブプロシージャ: **slot-ref** obj slot-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref-1)

obj 内の slot-name という名前のスロットの値を返します。

`slot-ref` は、obj に slot-name という名前のスロットがない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots) を参照)。

`slot-ref` は、obj 内の名前付きスロットに値がない場合、汎用関数 `slot-unbound` を呼び出します ([slot-unbound](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots) を参照)。

プリミティブプロシージャ: **slot-set!** obj slot-name value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_0021-1)

obj内のslot-nameという名前のスロットの値をvalueに設定します。

`slot-set!` は、obj に slot-name という名前のスロットがない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots) を参照)。

GOOPSはスロットに関する情報をクラスに格納します。内部的には、これらのプロシージャはすべて、クラス`(class-of obj)`内でslot-nameという名前のスロットのスロット定義を検索し、そのスロット定義の「getter」および「setter」クロージャを使用してスロット値を取得および設定することで動作します。

次の4つの手順は、前の手順とは異なり、`(class-of obj)` を前提とするのではなく、クラスを明示的な引数として受け取ります。そのため、あるクラスのスロット定義の「getter」と「setter」クロージャを、別のクラスのインスタンスに適用することができます。

プリミティブプロシージャ: **slot-exists-using-class?** クラス オブジェクト スロット名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dexists_002dusing_002dclass_003f)

クラスに slot-name という名前のスロットのスロット定義がある場合は `#t` を返し、そうでない場合は `#f` を返します。

プリミティブプロシージャ: **slot-bound-using-class?** class obj slot-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dbound_002dusing_002dclass_003f)

同じ引数に`slot-ref-using-class`を適用した場合に汎用関数`slot-unbound`が呼び出される場合は`#t`を返し、そうでない場合は`#f`を返します。

`slot-bound-using-class?` は、クラスに slot-name という名前のスロットのスロット定義がない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots) を参照)。

プリミティブプロシージャ: **slot-ref-using-class** クラス obj スロット名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dref_002dusing_002dclass)

クラス内の slot-name という名前のスロットに対する「getter」クロージャを obj に適用し、その結果を返します。

`slot-ref-using-class` は、クラスに slot-name という名前のスロットのスロット定義がない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots) を参照)。

`slot-ref-using-class` は、obj に「getter」クロージャを適用した結果、バインドされていない値が返された場合に、汎用関数 `slot-unbound` を呼び出します ([slot-unbound](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots) を参照)。

プリミティブプロシージャ: **slot-set-using-class!** class obj slot-name value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dset_002dusing_002dclass_0021)

クラス内の slot-name という名前のスロットの「セッター」クロージャを obj と value に適用します。

`slot-set-using-class!` は、クラスに slot-name という名前のスロットのスロット定義がない場合、汎用関数 `slot-missing` を呼び出します ([slot-missing](https://doc.guix.gnu.org/guile/latest/en/guile.html#Accessing-Slots) を参照)。

インスタンスごとではなくクラスごとに割り当てられるスロットは、特定のインスタンスを指定することなく参照および設定できます。

手順: **class-slot-ref** クラス スロット名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dslot_002dref)

クラス class 内の slot-name という名前のスロットの値を返します。指定されたスロットには、`#:class` または `#:each-subclass` の割り当てが必要です ([allocation](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照)。

`#:class` または `#:each-subclass` の割り当てを持つスロットが存在しない場合、`class-slot-ref` は引数 class と slot-name を指定して汎用関数 `slot-missing` を呼び出します。それ以外の場合、スロット値が未割り当てであれば、`class-slot-ref` は同じ引数で汎用関数 `slot-unbound` を呼び出します。

手順: **class-slot-set!** クラス スロット名 値 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-class_002dslot_002dset_0021)

クラス class 内の slot-name という名前のスロットの値を value に設定します。指定されたスロットには、`#:class` または `#:each-subclass` の割り当てが必要です ([allocation](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options) を参照)。

`#:class` または `#:each-subclass` の割り当てを持つスロットが存在しない場合、`class-slot-ref` は引数 class と slot-name を指定して汎用関数 `slot-missing` を呼び出します。

`slot-ref` または `slot-set!` 呼び出しで存在しないスロット名が指定された場合、または値がバインドされていないスロットを参照しようとした場合、GOOPS は次の汎用関数のいずれかを呼び出します。

汎用: **スロットが不足しています** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dmissing)

メソッド: **slot-missing** (クラス <class>) slot-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dmissing-1)

メソッド: **slot-missing** (クラス <class>) (オブジェクト <object>) slot-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dmissing-2)

メソッド: **slot-missing** (クラス <class>) (オブジェクト <object>) slot-name value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dmissing-3)

アプリケーションがクラスまたはインスタンスのスロットを名前で参照または設定しようとしたときに、指定されたクラスまたはオブジェクトに対してスロット名が無効な場合、GOOPS は汎用関数 `slot-missing` を呼び出します。

デフォルトのメソッドはすべて、適切なメッセージとともに`goops-error`を呼び出します。

汎用: **slot-unbound** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dunbound)

メソッド: **slot-unbound** (オブジェクト <object>) [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dunbound-1)

メソッド: **slot-unbound** (クラス <class>) slot-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dunbound-2)

メソッド: **slot-unbound** (class <class>) (object <object>) slot-name [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-slot_002dunbound-3)

アプリケーションがクラスまたはインスタンスのスロットを参照しようとした際に、そのスロットの値がバインドされていない場合、GOOPS は汎用関数 `slot-unbound` を呼び出します。

デフォルトのメソッドはすべて、適切なメッセージとともに`goops-error`を呼び出します。

* * *

次へ: [GOOPS オブジェクト雑録](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS-Object-Miscellany)、前: [イントロスペクション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Introspection)、上: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
