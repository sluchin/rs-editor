#### 7.5.27 SRFI-39 - パラメータ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d39-_002d-Parameters)

この SRFI は、動的スコープのパラメータのサポートを追加します。SRFI 39 は Guile コアに実装されているため、SRFI-39 自体を取得するためのモジュールは必要ありません。パラメータについては、[Parameters](https://doc.guix.gnu.org/guile/latest/en/guile.html#Parameters) を参照してください。

このモジュールは、`with-parameters*`という追加関数を1つエクスポートします。これは、コアの`with-fluids*`（[流体と動的状態](https://doc.guix.gnu.org/guile/latest/en/guile.html#Fluids-and-Dynamic-States)を参照）と同様に、SRFIへのGuile固有の追加機能です。

関数: **with-parameters\*** param-list value-list thunk [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index- with_002dparameters_002a)

上記の `parameterize` に従って、param-list からパラメータを取得し、value-list から対応する値を取得して、新しい動的スコープを確立します。新しいスコープ内で `(thunk)` が呼び出され、その thunk の結果が `with-parameters*` からの戻り値となります。

* * *

次へ: [SRFI-42 - Eager Comprehensions](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d42)、前: [SRFI-39 - Parameters](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d39)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
