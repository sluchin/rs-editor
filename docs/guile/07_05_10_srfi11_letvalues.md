#### 7.5.10 SRFI-11 - let-values

このモジュールは、複数の値に対するバインディング形式である `let-values` と `let*-values` を実装します。これらの形式は `let` と `let*` に似ていますが ([ローカル変数バインディング](06_10_definitions_and_variable_bindings.md#6102-ローカル変数バインディング) を参照)、複数値式によって返される値のバインディングをサポートします。

バインディングを有効にするには、`(use-modules (srfi srfi-11))` と記述してください。

(let-values (((xy) ([values](06_11_controlling_the_flow_of_program_execution.md#6117-複数の値の返却と受け入れ) 1 2))
((zf) ([values](06_11_controlling_the_flow_of_program_execution.md#6117-複数の値の返却と受け入れ) 3 4)))
([+](06_06_02_numerical_data_types.md#66211-算術関数) xyzf))
⇒
10

`let-values` はすべてのバインディングを同時に実行します。つまり、バインディング句内のどの式も、同じ句リスト内でバインドされている変数を参照することはできません。一方、`let*-values` は、単一値式の場合の `let*` と同様に、バインディングを順次実行します。

* * *

次へ: [SRFI-14 - 文字セットライブラリ](07_05_12_srfi14_characterset_library.md#7512-srfi-14---文字セットライブラリ)、前: [SRFI-11 - let-values](#7510-srfi-11---let-values)、上: [SRFI サポートモジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
