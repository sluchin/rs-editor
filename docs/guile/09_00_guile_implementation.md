9 Guile の実装 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Implementation-1)
------------------------------------------------------------------------------------------------------

Scheme でしばらくプログラミングをしていると、ある時点で Scheme の別のレベル、つまり実装が見えてきます。Scheme がどのように実装できるかを知ることは、熟練したハッカーになるために必要であることがわかります。Peter Norvig が PAIP[36](https://doc.guix.gnu.org/guile/latest/en/guile.html#FOOT36) に関する回顧録で述べているように、「熟練した Lisp プログラマーは最終的に優れた『効率モデル』を開発します」。

ノーヴィグがここで言いたいのは、Lispハッカーは時間をかけて、自分のコードが空間と時間の面でどれだけの「コスト」を負担するのかを最終的に理解するようになるということだ。

この章では、Schemeの実装であるGuileについて、その歴史、データの表現方法と評価方法、そしてコンパイラについて解説します。この知識は、単にSchemeに精通しているだけのユーザーから、真のハッカーへとステップアップするのに役立つでしょう。

* [Guileの簡単な歴史](https://doc.guix.gnu.org/guile/latest/en/guile.html#History)
* [データ表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Representation)
* [Guile 用仮想マシン](https://doc.guix.gnu.org/guile/latest/en/guile.html#A-Virtual-Machine-for-Guile)
* [仮想マシンへのコンパイル](https://doc.guix.gnu.org/guile/latest/en/guile.html#Compiling-to-the-Virtual-Machine)

* * *

次へ: [データ表現](https://doc.guix.gnu.org/guile/latest/en/guile.html#Data-Representation)、上: [Guile実装](https://doc.guix.gnu.org/guile/latest/en/guile.html#Guile-Implementation) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
