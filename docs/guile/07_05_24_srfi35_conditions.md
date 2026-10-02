#### 7.5.24 SRFI-35 - 条件

[SRFI-35](http://srfi.schemers.org/srfi-35/srfi-35.html)では、プログラムの各部分間の例外的な状況に関する情報を伝えるために設計されたレコードに似たデータ構造である_conditions_が定義されています。これは通常、SRFI-34の`raise`と組み合わせて使用されます。

([raise](07_02_08_signals.md#728-シグナル) ([condition](#7524-srfi-35---条件) ([&message](07_06_r6rs_support.md#76213-rnrs-条件)
(メッセージ「エラーが発生しました」))))

ユーザーは、任意の情報を含む条件型を定義できます。条件型は互いに継承できます。これにより、条件を処理（または「捕捉」）するプログラム部分は、発生した例外的な条件に関する正確な情報を取得できます。

SRFI-35の条件は、以下の方法で入手できます。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (srfi srfi-35))

条件タイプを操作するために利用できる手順は以下のとおりです。

スキーム手順: **make-condition-type** id parent field-names [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- make_002dcondition_002dtype)

親クラスを継承し、field-namesにリストされたフィールド名を持つ、idという名前の新しい条件型を返します。field-namesはシンボルのリストである必要があり、親クラスまたはそのスーパークラスですでに使用されている名前を含めてはなりません。

Scheme Procedure: **condition-type?** obj

objが条件型であればtrueを返します。

条件の作成とアクセスは、以下の手順で行えます。

Scheme Procedure: **make-condition** type . field+value

フィールドが field+value で指定されたように初期化され、フィールド名 (シンボル) と値のシーケンスを含む、型 type の新しい条件を返します。例を以下に示します。

(let ((&ct ([make-condition-type](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-make_002dcondition_002dtype) 'foo [&condition](07_06_r6rs_support.md#76213-rnrs-条件) '(abc))))
([make-condition](#7524-srfi-35---条件) &ct 'a 1 'b 2 'c 3))

型とその上位型のすべてのフィールドを指定する必要があることに注意してください。

スキーム手順: **make-compound-condition** condition1 condition2 …

condition1 condition2 ... で構成される新しい複合条件を返します。返される条件は、condition1 condition2 … の各条件の型を持ちます (`condition-has-type?` による)。

Scheme プロシージャ: **condition-has-type?** c type

条件 c の型が type であれば true を返します。

スキーム手順: **条件参照** c フィールド名

条件 c から、field-name という名前のフィールドの値を返します。

c が複合条件であり、複数の基となる条件タイプに field-name という名前のフィールドが含まれている場合、条件が `make-compound-condition` に渡された順序を使用して、そのような最初のフィールドの値が返されます。

Scheme 手順: **extract-condition** c 型

cで指定されたフィールド値を持つ条件タイプの条件を返します。

c が複合条件である場合、その条件を作成した `make-compound-condition` の呼び出しで最初に現れたタイプのサブ条件からフィールド値を抽出します。

条件タイプや条件を作成するための便利なマクロも利用できます。

ライブラリ構文: **define-condition-type** type supertype predicate field-spec...

スーパータイプを継承する新しい条件タイプ type を定義します。さらに、型 type またはそのサブタイプの条件が渡されたときに true を返す型述語に述語をバインドします。フィールド仕様は `(フィールドアクセサー)` の形式である必要があります。ここで、field は type のフィールド名、accessor は型 type の条件でフィールド field にアクセスするためのプロシージャ名です。

以下の例では、`&condition` を継承し、フィールド `a`、`b`、`c` を持つ条件型 `&foo` を定義しています。

(define-condition-type &foo [&condition](07_06_r6rs_support.md#76213-rnrs-条件)
フード条件？
（フーア）
(b foo-b)
(c foo-c))

ライブラリ構文: **条件** 型フィールドバインディング1 型フィールドバインディング2 …

type-field-binding1、type-field-binding2、...に従って初期化された新しい条件または複合条件を返します。各 type-field-binding は `(type field-specs...)` の形式である必要があります。ここで type は条件タイプにバインドされた変数の名前です。各 field-spec は `(field-name value)` の形式である必要があります。ここで field-name は value に初期化されるフィールドを示すシンボルです。`make-condition` と同様に、すべてのフィールドを指定する必要があります。

次の例は、単純な条件を返します。

([condition](#7524-srfi-35---条件) ([&message](07_06_r6rs_support.md#76213-rnrs-条件) (message "エラーが発生しました")))

以下のものは複合条件を返します。

([condition](#7524-srfi-35---条件) ([&message](07_06_r6rs_support.md#76213-rnrs-条件) (message "エラーが発生しました"))
([&serious](07_06_r6rs_support.md#76213-rnrs-条件)))

最後に、SRFI-35はいくつかの標準的な状態タイプを定義している。

変数: **&condition**

この条件タイプは、すべての条件タイプのルートです。フィールドは存在しません。

変数: **&message**

人間に対して病状の性質を説明するメッセージを伝える病状タイプ。

Scheme Procedure: **message-condition?** c

cが型`&message`またはそのサブタイプのいずれかの型である場合はtrueを返します。

スキーム手順: **condition-message** c

メッセージ条件cに関連付けられたメッセージを返します。

変数: **&serious**

この型は、無視するにはあまりにも深刻な状態を表します。フィールドはありません。

スキーム手順: **serious-condition?** c

cが型`&serious`またはそのサブタイプの1つである場合はtrueを返します。

変数: **&error**

この状態はエラーを表しており、通常はプログラムと外部環境またはユーザーとのやり取りにおいて何らかの問題が発生したことが原因です。

Scheme Procedure: **error?** c

cが型`&error`またはそのサブタイプのいずれかの型である場合はtrueを返します。

実装上の注意として、Guile の条件オブジェクトは「例外オブジェクト」と同じです。[例外オブジェクト](06_11_controlling_the_flow_of_program_execution.md#61181-例外オブジェクト) を参照してください。`&condition`、`&serious`、`&error` の条件タイプは、コア Guile ではそれぞれ `&exception`、`&error`、`&external-error` として知られています。

* * *

次へ: [SRFI-38 - 共有構造を持つデータの外部表現](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md#7526-srfi-38---共有構造を持つデータの外部表現)、前: [SRFI-35 - 条件](#7524-srfi-35---条件)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
