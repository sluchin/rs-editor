### 6.3 SCMタイプ [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type-1)

Guile は、すべての Scheme 値を単一の C 型 `SCM` で表現します。このトピックの概要については、[動的型](https://doc.guix.gnu.org/guile/latest/en/guile.html#Dynamic-Types) を参照してください。

Cタイプ: **SCM** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM)

`SCM`は、GuileのSchemeオブジェクトを表すために使用されるユーザーレベルの抽象C型です。Schemeオブジェクトの型に関係なく、すべてのオブジェクトを表すことができます。代入以外のC操作は`SCM`型の変数では動作が保証されていないため、`SCM`値を扱うにはマクロと関数のみを使用してください。Cデータ型と`SCM`型間の値の変換は、ユーティリティ関数とマクロを使用して行います。

Cタイプ: **scm\_t\_bits** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fbits)

`scm_t_bits`は、Schemeオブジェクトを表現するために必要なすべての情報を保持できる十分な大きさであることが保証された符号なし整数型です。このデータ型は主にGuileの内部実装に使用されますが、Guileの特定の拡張機能を作成する際にも必要となります。

C 型: **scm\_t\_signed\_bits** [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-scm_005ft_005fsigned_005fbits)

これは、`scm_t_bits` と同じサイズの符号付き整数型です。

C マクロ: `scm_t_bits` **SCM\_UNPACK** `(SCM x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fUNPACK)

`SCM`値xを整数型として表現します。`SCM_UNPACK`を適用した後でなければ、`SCM`値のビットと内容にアクセスすることはできません。

C マクロ: `SCM` **SCM\_PACK** `(scm_t_bits x)` [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-SCM_005fPACK)

Schemeオブジェクトの有効な整数表現を受け取り、それを`SCM`値としての表現に変換します。

* * *

次へ: [マクロのスナーフィング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Snarfing-Macros)、前: [SCM タイプ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
