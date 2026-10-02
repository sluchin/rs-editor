### 8.4 スロットオプション [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options-1)

スロットを指定する際（`(define-class …)` の形式）、スロット名に加えて様々なオプションを指定できます。各オプションはキーワードで指定します。使用可能なキーワードの一覧は以下のとおりです。

スロットオプション: **#:init-value** init-value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003ainit_002dvalue)

スロットオプション: **#:init-form** init-form [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003ainit_002dform)

スロットオプション: **#:init-thunk** init-thunk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003ainit_002dthunk)

スロットオプション: **#:init-keyword** init-keyword [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003ainit_002dkeyword)

これらのオプションは、インスタンス作成時にスロットの値を初期化する方法を指定するための様々な方法を提供する。

init-valueは、固定の初期スロット値を指定します（クラスのすべての新規インスタンスで共有されます）。

init-thunkは、スロットのデフォルト値を提供するサンクを指定します。このサンクは新しいインスタンスが作成されるときに呼び出され、目的の初期スロット値を返す必要があります。

init-form は、評価されるとスロットの初期値を返す形式を指定します。この形式は、クラスのインスタンスが作成されるたびに、包含する `define-class` 式の字句環境内で評価されます。

init-keywordは、新しいインスタンスを作成する際に`make`に初期スロット値を渡すために使用できるキーワードを指定します。

`init-value` の値はクラスのすべてのインスタンスで共有されるため、初期値が定数などの不変の値である場合にのみ使用してください。スロットを新しい、独立して変更可能な値で初期化する場合は、代わりに `init-thunk` または `init-form` を使用してください。次の例を考えてみましょう。

(define-class <chbouib> ()
(ハッシュテーブル #:init-value (make-hash-table)))

ここではハッシュテーブルが1つだけ作成され、`<chbouib>`のすべてのインスタンスの`hashtab`スロットはそのハッシュテーブルを参照します。`<chbouib>`の各インスタンスが新しいハッシュテーブルを参照するようにするには、代わりに次のように記述する必要があります。

(define-class <chbouib> ()
(ハッシュテーブル #:init-thunk make-hash-table))

または：

(define-class <chbouib> ()
(ハッシュテーブル #:init-form (make-hash-table)))

同じスロットに対してこれらのオプションが複数指定されている場合、優先順位は最も高いものから順に次のようになります。

* `#:init-keyword`、`make`に渡されるオプションにinit-keywordが存在する場合
* `#:init-thunk`、`#:init-form`、または`#:init-value`。

スロット定義に同じ優先順位の初期化オプションが複数含まれている場合、後から指定されたオプションは無視されます。スロットが全く初期化されていない場合、その値は未定義となります。

一般的に、複数のインスタンス間で共有されるスロットは、新しいインスタンスが作成される時点でスロット値が未バインドの場合に限り、その時点で初期化されます。ただし、新しいインスタンスの作成時に共有スロットに対して有効なinitキーワードと値が指定された場合、スロットは以前の値に関係なく再初期化されます。

ただし、GOOPSのメタオブジェクトプロトコルの強力さにより、ここに記述されているすべての内容は特定のクラスに合わせてカスタマイズまたはオーバーライドできることに注意してください。ここで説明するスロットの初期化は、汎用関数`initialize`の最も特殊化されていないメソッドによって実行されます。そのシグネチャは次のとおりです。

(define-method (initialize (object <object>) initargs) ...)

特定のクラスのインスタンスの初期化は、そのクラス専用の `initialize` メソッドを定義することでカスタマイズできます。また、専用メソッドの作成者は、専用コード内の任意の箇所で `next-method` を呼び出すか、あるいは全く呼び出さないかを選択できます。`next-method` を呼び出すと、次に汎用的な `initialize` メソッドが呼び出されます。したがって、一般的に、ここで説明する初期化メカニズムは、より汎用的なコードによって変更または上書きされる場合があり、特定のクラスでは全くサポートされない場合もあります。

スロットオプション: **#:getter** ゲッター [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003agetter)

スロットオプション: **#:setter** セッター [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003asetter)

スロットオプション: **#:accessor** アクセサー [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aaccessor)

`foo` と `bar` という名前のスロットを持つオブジェクト obj が与えられた場合、関連するスロット名を指定して `slot-ref` と `slot-set!` を呼び出すことで、これらのスロットを読み書きすることが常に可能です。例:

(スロット参照オブジェクト 'foo')
(スロット設定! オブジェクト 'bar 25)

`#:getter`、`#:setter`、`#:accessor` オプションが存在する場合、GOOPS は、スロット値をより簡単に取得および設定するために使用できる汎用関数とメソッド定義を作成します。 getter は、GOOPS がスロット値を取得するためのメソッドを追加する汎用関数を指定します。 setter は、GOOPS がスロット値を設定するためのメソッドを追加する汎用関数を指定します。 accessor は、GOOPS がスロット値の取得と設定の両方のメソッドを追加するアクセサーを指定します。

クラスに次のようなスロット定義が含まれている場合：

(c #:getter get-count #:setter set-count #:accessor count)

GOOPSは、ゲッターまたはアクセサーのどちらを使用してもスロット値を参照できる汎用関数メソッドを定義します。

(let ((current-count (get-count obj))) ...)
(let ((current-count (count obj))) ...)

セッターまたはアクセサーのいずれかを使用して設定します。

(set-count obj (+ 1 current-count))
(セット! (count obj) (+ 1 current-count))

ご了承ください

* アクセサを使用する場合、スロット値は汎用的な `set!` 構文を使用して設定されます。
* 実際には、スロットがこれら 3 つのオプションすべてを使用することはまれです。読み取り専用、書き込み専用、読み書き可能なスロットは、通常、それぞれ `#:getter`、`#:setter`、`#:accessor` オプションのみを使用します。

指定された名前のバインドは、`define-class` 式の環境で行われます。名前が既に（その環境で）汎用関数にアップグレードできない値にバインドされている場合、`define-class` 式が評価されるときにそれらの値が上書きされます。詳細については、[ensure-generic](https://doc.guix.gnu.org/guile/latest/en/guile.html#Generic-Function-Internals) を参照してください。

スロットオプション: **#:allocation** 割り当て [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aallocation)

`#:allocation` オプションは、GOOPS にスロットのストレージをどのように割り当てるかを指示します。割り当て可能な値は次のとおりです。

* `#:instance`
    
GOOPSが、このスロット用に、包含クラス（およびそのサブクラス）の新しいインスタンスごとに個別のストレージを作成することを示します。これがデフォルトです。
    
* `#:class`
    
これは、GOOPS がこのスロット用に、包含クラス (およびそのサブクラス) のすべてのインスタンスで共有されるストレージを作成する必要があることを示します。言い換えれば、割り当て `#:class` を持つクラス C のスロットは、`(is-a? instance c)` であるすべてのインスタンスで共有されます。これにより、スロットを定義するクラスの直接的または間接的なインスタンスのみがアクセスできる一種のグローバル変数を定義できます。
    
* `#:each-subclass`
    
これは、GOOPS がこのスロット用に、包含クラスのすべての _direct_ インスタンスで共有されるストレージを作成し、包含クラスのサブクラスが定義されるたびに、GOOPS がそのサブクラスのすべての _direct_ インスタンスで共有される新しいストレージをスロット用に作成する必要があることを示します。言い換えれば、割り当てが `#:each-subclass` のスロットは、同じ `class-of` を持つすべてのインスタンスで共有されます。
    
* `#:virtual`
    
GOOPS がこのスロットにストレージを割り当てないことを示します。スロット定義には、このスロットの値を参照および設定する方法を指定するために、`#:slot-ref` オプションと `#:slot-set!` オプションも含める必要があります。以下の例を参照してください。
    

スロット割り当てオプションは、クラスのメタクラスによって特殊化された汎用関数 `compute-get-n-set` によって新しいクラスを定義する際に処理されます。したがって、新しいタイプのスロット割り当ては、新しいメタクラスと、その新しいメタクラスに特化した `compute-get-n-set` メソッドを定義することによって実装できます。この方法の例については、[クラス定義のカスタマイズ](https://doc.guix.gnu.org/guile/latest/en/guile.html#Customizing-Class-Definition) を参照してください。

スロットオプション: **#:slot-ref** ゲッター [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aslot_002dref-1)

スロットオプション: **#:slot-set!** セッター [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0023_003aslot_002dset_0021-1)

スロット割り当てが `#:virtual` の場合は、`#:slot-ref` オプションと `#:slot-set!` オプションを指定する必要があります。それ以外の場合は、これらのオプションは無視されます。

getterは、単一のインスタンスパラメータを受け取り、現在のスロット値を返すクロージャである必要があります。setterは、インスタンスと新しい値という2つのパラメータを受け取り、スロット値を新しい値に設定するクロージャである必要があります。

* * *

次へ: [メソッドと汎用関数](https://doc.guix.gnu.org/guile/latest/en/guile.html#Methods-and-Generic-Functions )、前: [スロットオプション](https://doc.guix.gnu.org/guile/latest/en/guile.html#Slot-Options)、上: [GOOPS](https://doc.guix.gnu.org/guile/latest/en/guile.html#GOOPS) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
