8 GOOPS
------------------------------------------------------------------------

GOOPSは「Guileオブジェクト指向プログラミングシステム」の略です。その実装は、Erick Gallesio氏によるSTk-3.99.3と、Gregor Kiczales氏によるTiny-Closのバージョン1.3を基にしています。Common LispオブジェクトシステムであるCLOSと精神的に非常に近いものですが、Scheme言語向けに改良されています。

GOOPSは、クラス、オブジェクト、多重継承、マルチメソッドディスパッチを備えた汎用関数など、完全なオブジェクト指向システムです。さらに、その実装はメタオブジェクトプロトコルに依存しており、GOOPSの中核となる操作自体が関連クラスのメソッドとして定義され、それらのメソッドをオーバーライドまたは再定義することでカスタマイズできます。

GOOPSを使い始めるには、まず`(oop goops)`モジュールをインポートする必要があります。これはGuile REPLで以下のように評価することで行えます。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (oop goops))

* [著作権表示](08_01_copyright_notice.md#81-著作権表示)
* [クラス定義](08_02_class_definition.md#82-クラス定義)
* [インスタンスの作成とスロットへのアクセス](08_03_instance_creation_and_slot_access.md#83-インスタンスの作成とスロットへのアクセス)
* [スロットオプション](08_04_slot_options.md#84-スロットオプション)
* [スロットの説明例](08_05_illustrating_slot_description.md#85-スロットの説明の図解)
* [メソッドと汎用関数](08_06_methods_and_generic_functions.md#86-メソッドとジェネリック関数)
* [継承](08_07_inheritance.md#87-継承)
* [イントロスペクション](08_08_introspection.md#88-イントロスペクション)
* [エラー処理](08_09_error_handling.md#89-エラー処理)
* [GOOPS オブジェクト雑録](08_10_goops_object_miscellany.md#810-goops-オブジェクト雑記)
* [メタオブジェクトプロトコル](08_11_the_metaobject_protocol.md#811-メタオブジェクトプロトコル)
* [クラスの再定義](08_12_redefining_a_class.md#812-クラスの再定義)
* [インスタンスのクラスの変更](https://doc.guix.gnu.org/guile/latest/en/guile.html#Changing-the-Class-of-an-Instance )

* * *

次へ: [クラス定義](08_02_class_definition.md#82-クラス定義)、上へ: [GOOPS](#8-goops) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
