#### 7.5.27 SRFI-39 - パラメータ

この SRFI は、動的スコープのパラメータのサポートを追加します。SRFI 39 は Guile コアに実装されているため、SRFI-39 自体を取得するためのモジュールは必要ありません。パラメータについては、[Parameters](06_11_controlling_the_flow_of_program_execution.md#61112-パラメータ) を参照してください。

このモジュールは、`with-parameters*`という追加関数を1つエクスポートします。これは、コアの`with-fluids*`（[流体と動的状態](06_11_controlling_the_flow_of_program_execution.md#61111-流体と動的状態)を参照）と同様に、SRFIへのGuile固有の追加機能です。

関数: **with-parameters\*** param-list value-list thunk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- with_002dparameters_002a)

上記の `parameterize` に従って、param-list からパラメータを取得し、value-list から対応する値を取得して、新しい動的スコープを確立します。新しいスコープ内で `(thunk)` が呼び出され、その thunk の結果が `with-parameters*` からの戻り値となります。

* * *

次へ: [SRFI-42 - Eager Comprehensions](07_05_29_srfi42_eager_comprehensions.md#7529-srfi-42---積極的な理解)、前: [SRFI-39 - Parameters](#7527-srfi-39---パラメータ)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
