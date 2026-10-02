#### 7.5.24 SRFI-35 - 条件 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d35-_002d-Conditions)

[SRFI-35](http://srfi.schemers.org/srfi-35/srfi-35.html)では、プログラムの各部分間の例外的な状況に関する情報を伝えるために設計されたレコードに似たデータ構造である_conditions_が定義されています。これは通常、SRFI-34の`raise`と組み合わせて使用されます。

([raise](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-raise) ([condition](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition) ([&message](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026message)
(メッセージ「エラーが発生しました」))))

ユーザーは、任意の情報を含む条件型を定義できます。条件型は互いに継承できます。これにより、条件を処理（または「捕捉」）するプログラム部分は、発生した例外的な条件に関する正確な情報を取得できます。

SRFI-35の条件は、以下の方法で入手できます。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (srfi srfi-35))

条件タイプを操作するために利用できる手順は以下のとおりです。

スキーム手順: **make-condition-type** id parent field-names [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- make_002dcondition_002dtype)

親クラスを継承し、field-namesにリストされたフィールド名を持つ、idという名前の新しい条件型を返します。field-namesはシンボルのリストである必要があり、親クラスまたはそのスーパークラスですでに使用されている名前を含めてはなりません。

Scheme Procedure: **condition-type?** obj [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dtype_003f)

objが条件型であればtrueを返します。

条件の作成とアクセスは、以下の手順で行えます。

Scheme Procedure: **make-condition** type . field+value [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcondition)

フィールドが field+value で指定されたように初期化され、フィールド名 (シンボル) と値のシーケンスを含む、型 type の新しい条件を返します。例を以下に示します。

(let ((&ct ([make-condition-type](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcondition_002dtype) 'foo [&condition](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026condition) '(abc))))
([make-condition](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcondition) &ct 'a 1 'b 2 'c 3))

型とその上位型のすべてのフィールドを指定する必要があることに注意してください。

スキーム手順: **make-compound-condition** condition1 condition2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcompound_002dcondition)

condition1 condition2 ... で構成される新しい複合条件を返します。返される条件は、condition1 condition2 … の各条件の型を持ちます (`condition-has-type?` による)。

Scheme プロシージャ: **condition-has-type?** c type [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dhas_002dtype_003f)

条件 c の型が type であれば true を返します。

スキーム手順: **条件参照** c フィールド名 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dref)

条件 c から、field-name という名前のフィールドの値を返します。

c が複合条件であり、複数の基となる条件タイプに field-name という名前のフィールドが含まれている場合、条件が `make-compound-condition` に渡された順序を使用して、そのような最初のフィールドの値が返されます。

Scheme 手順: **extract-condition** c 型 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-extract_002dcondition)

cで指定されたフィールド値を持つ条件タイプの条件を返します。

c が複合条件である場合、その条件を作成した `make-compound-condition` の呼び出しで最初に現れたタイプのサブ条件からフィールド値を抽出します。

条件タイプや条件を作成するための便利なマクロも利用できます。

ライブラリ構文: **define-condition-type** type supertype predicate field-spec... [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-define_002dcondition_002dtype)

スーパータイプを継承する新しい条件タイプ type を定義します。さらに、型 type またはそのサブタイプの条件が渡されたときに true を返す型述語に述語をバインドします。フィールド仕様は `(フィールドアクセサー)` の形式である必要があります。ここで、field は type のフィールド名、accessor は型 type の条件でフィールド field にアクセスするためのプロシージャ名です。

以下の例では、`&condition` を継承し、フィールド `a`、`b`、`c` を持つ条件型 `&foo` を定義しています。

(define-condition-type &foo [&condition](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026condition)
フード条件？
（フーア）
(b foo-b)
(c foo-c))

ライブラリ構文: **条件** 型フィールドバインディング1 型フィールドバインディング2 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition)

type-field-binding1、type-field-binding2、...に従って初期化された新しい条件または複合条件を返します。各 type-field-binding は `(type field-specs...)` の形式である必要があります。ここで type は条件タイプにバインドされた変数の名前です。各 field-spec は `(field-name value)` の形式である必要があります。ここで field-name は value に初期化されるフィールドを示すシンボルです。`make-condition` と同様に、すべてのフィールドを指定する必要があります。

次の例は、単純な条件を返します。

([condition](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition) ([&message](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026message) (message "エラーが発生しました")))

以下のものは複合条件を返します。

([condition](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition) ([&message](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026message) (message "エラーが発生しました"))
([&serious](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026serious)))

最後に、SRFI-35はいくつかの標準的な状態タイプを定義している。

変数: **&condition** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026condition-1)

この条件タイプは、すべての条件タイプのルートです。フィールドは存在しません。

変数: **&message** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026message-2)

人間に対して病状の性質を説明するメッセージを伝える病状タイプ。

Scheme Procedure: **message-condition?** c [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-message_002dcondition_003f)

cが型`&message`またはそのサブタイプのいずれかの型である場合はtrueを返します。

スキーム手順: **condition-message** c [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-condition_002dmessage)

メッセージ条件cに関連付けられたメッセージを返します。

変数: **&serious** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026serious-1)

この型は、無視するにはあまりにも深刻な状態を表します。フィールドはありません。

スキーム手順: **serious-condition?** c [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-serious_002dcondition_003f)

cが型`&serious`またはそのサブタイプの1つである場合はtrueを返します。

変数: **&error** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_0026error-2)

この状態はエラーを表しており、通常はプログラムと外部環境またはユーザーとのやり取りにおいて何らかの問題が発生したことが原因です。

Scheme Procedure: **error?** c [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-error_003f-1)

cが型`&error`またはそのサブタイプのいずれかの型である場合はtrueを返します。

実装上の注意として、Guile の条件オブジェクトは「例外オブジェクト」と同じです。[例外オブジェクト](https://doc.guix.gnu.org/guile/latest/en/guile.html#Exception-Objects) を参照してください。`&condition`、`&serious`、`&error` の条件タイプは、コア Guile ではそれぞれ `&exception`、`&error`、`&external-error` として知られています。

* * *

次へ: [SRFI-38 - 共有構造を持つデータの外部表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d38)、前: [SRFI-35 - 条件](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d35)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
