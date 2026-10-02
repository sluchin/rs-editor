### 6.1 Guile API の概要 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Overview-of-the-Guile-API)

Guileのアプリケーションプログラミングインターフェース（API）は、アプリケーション開発者がC言語またはScheme言語のどちらでも使用できる機能を提供します。このインターフェースは、C言語ではマクロ、関数、変数、Scheme言語ではプロシージャ、変数、構文、その他のオブジェクトといった要素で構成されています。

多くの要素は、SchemeとCの両方で適切な形式で利用可能です。例えば、Schemeのプロシージャ`assq`は、Cコードでは`scm_assq`として利用できます。これらの要素は、SchemeとCの両方の側面を網羅した形で一度だけ文書化されています。

Schemeにおける要素名は、C言語における要素名と規則的な関係にある。また、C言語の関数は、体系的な方法で引数を受け取る。

通常、C言語の関数名は、Scheme名から簡単なテキスト変換によって導き出すことができます。

* `-` (ハイフン) を `_` (アンダースコア) に置き換えます。
* `?` (疑問符) を `_p` に置き換えます。
* `!` (感嘆符) を `_x` に置き換えます。
* 内部の `->` を `_to_` に置き換えます。
* `<=` (以下) を `_leq` に置き換えます。
* `>=` (以上) を `_geq` に置き換えます。
* `<` (小なり記号) を `_less` に置き換えます。
* `>` (より大きい) を `_gr` に置き換えます。
* `scm_` を接頭辞として付けます。

AC関数は、対応するScheme関数が可変個の引数を取る場合でも、常に固定数の型`SCM`の引数を取ります。

Scheme 関数の中には、最後の引数が省略可能なものがあります。対応する C 関数は、常にすべての省略可能な引数を指定して呼び出す必要があります。引数が指定されていないかのように動作させるには、その引数の値として `SCM_UNDEFINED` を渡します。途中の引数に対しては、この操作はできません。1 つの引数が `SCM_UNDEFINED` の場合、それに続くすべての引数も `SCM_UNDEFINED` にする必要があります。

Schemeの一部の関数は任意の数の_rest_引数を受け取ります。対応するC関数は、これらの引数すべてをリストとして指定して呼び出す必要があります。このリストは常にC関数の最後の引数となります。

これら2つのバリエーションは組み合わせることもできます。

Scheme関数に対応するC関数の戻り値の型は常に`SCM`です。そのため、以下の説明では、戻り値と引数を除いて、型は省略されることがよくあります。

* * *

次へ: [SCM タイプ](https://doc.guix.gnu.org/guile/latest/en/guile.html#The-SCM-Type)、前: [Guile API の概要](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Overview)、上: [API リファレンス](https://doc.guix.gnu.org/guile/latest/en/guile.html#API-Reference) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
