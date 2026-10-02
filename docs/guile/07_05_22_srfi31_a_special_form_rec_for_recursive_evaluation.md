#### 7.5.22 SRFI-31 - 再帰評価のための特殊形式「rec」[¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d31-_002d-A-special-form-_0060rec_0027-for-recursive-evaluation)

SRFI-31では、自己参照式をより簡単に作成できる特別な形式が定義されています。構文は以下のとおりです。

<rec 式> --> (rec <変数> <式>)
<rec 式> --> (rec (<変数>+) <本体>)

最初の構文は、自己参照式を作成するために使用できます。例：

guile> (define tmp (rec ones ([cons](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-cons) 1 (delay ones))))

2番目の構文は、匿名再帰関数を作成するために使用できます。

guile> (define tmp (rec (display-n item n)
([when](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-when-1) ([positive?](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-positive_003f) n)
([display](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-display) item) ([newline](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-newline))
(display-n item ([\-](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-_002d) n 1)))))
guile> (tmp 42 3)
42
42
42
策略>

* * *

次へ: [SRFI-35 - 条件](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d35)、前: [SRFI-31 - 再帰評価のための特殊形式 'rec'](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d31)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
