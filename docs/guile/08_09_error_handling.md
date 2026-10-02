### 8.9 エラー処理

プロシージャ`goops-error`は、以下の汎用関数のデフォルトメソッドによって適切なエラーを発生させるために呼び出されます。

* `slot-missing` ([slot-missing](08_08_introspection.md#885-スロットへのアクセス)を参照)
* `slot-unbound` ([slot-unbound](08_08_introspection.md#885-スロットへのアクセス)を参照)
* `no-method` ([no-method](08_06_methods_and_generic_functions.md#867-呼び出しエラーの処理)を参照)
* `no-applicable-method` ([no-applicable-method](08_06_methods_and_generic_functions.md#867-呼び出しエラーの処理)を参照)
* `no-next-method` ([no-next-method](08_06_methods_and_generic_functions.md#867-呼び出しエラーの処理)を参照)

これらの関数を特定のクラスやメタクラスに合わせてカスタマイズする場合でも、検出したエラー状態を通知するために `goops-error` を使用することをお勧めします。

手順: **goops-error** format-string arg …

キー `goops-error` と format-string および arg ... から構築されたエラーメッセージを使用してエラーを発生させます。エラーメッセージのフォーマットは `scm-error` と同様です。

* * *

次へ: [メタオブジェクトプロトコル](08_11_the_metaobject_protocol.md#811-メタオブジェクトプロトコル)、前: [エラー処理](#89-エラー処理)、上: [GOOPS](08_00_goops.md#8-goops) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
