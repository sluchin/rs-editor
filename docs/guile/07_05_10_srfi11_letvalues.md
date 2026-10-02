#### 7.5.10 SRFI-11 - let-values [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d11-_002d-let_002dvalues)

このモジュールは、複数の値に対するバインディング形式である `let-values` と `let*-values` を実装します。これらの形式は `let` と `let*` に似ていますが ([ローカル変数バインディング](https://doc.guix.gnu.org/guile/latest/en/guile.html#Local-Bindings) を参照)、複数値式によって返される値のバインディングをサポートします。

バインディングを有効にするには、`(use-modules (srfi srfi-11))` と記述してください。

(let-values (((xy) ([values](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-values) 1 2))
((zf) ([values](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-values) 3 4)))
([+](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002b) xyzf))
⇒
10

`let-values` はすべてのバインディングを同時に実行します。つまり、バインディング句内のどの式も、同じ句リスト内でバインドされている変数を参照することはできません。一方、`let*-values` は、単一値式の場合の `let*` と同様に、バインディングを順次実行します。

* * *

次へ: [SRFI-14 - 文字セットライブラリ](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d14)、前: [SRFI-11 - let-values](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d11)、上: [SRFI サポートモジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
