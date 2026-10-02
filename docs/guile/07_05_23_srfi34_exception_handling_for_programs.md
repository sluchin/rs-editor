#### 7.5.23 SRFI-34 - プログラムの例外処理 [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d34-_002d-Exception-handling-for-programs)

Guile は、独自の組み込みメカニズムの代替として、[SRFI-34 の例外処理メカニズム](http://srfi.schemers.org/srfi-34/srfi-34.html) の実装を提供します ([例外](https://doc.guix.gnu.org/guile/latest/en/guile.html#Exceptions) を参照)。これは次のようにして利用できます。

([use-modules](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-use_002dmodules) (srfi srfi-34))

`with-exception-handler` および `raise` (コア Guile では `raise-exception` として知られています) の詳細については、[例外の発生と処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#Raising-and-Handling-Exceptions) を参照してください。

SRFI-34の`guard`形式は、`with-exception-handler`に対するシンタックスシュガーです。

構文: **guard** (var 句 …) 本体 … [¶](https://doc.guix.gnu.org/guile/latest/en/guile.html#index-guard)

例外ハンドラを使用して本体を評価し、発生したオブジェクトを変数 var にバインドします。そして、そのバインドの範囲内で、句…を条件式の句であるかのように評価します。その暗黙の条件式は、ガード式の継続と動的な環境を使用して評価されます。

すべての節のテストが偽と評価され、かつ `else` 節がない場合、`raise` は、元の `raise` 呼び出しの動的環境内で、発生したオブジェクトに対して再度呼び出されます。ただし、現在の例外ハンドラは `guard` 式のものです。

* * *

次へ: [SRFI-37 - args-fold](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d37)、前: [SRFI-34 - プログラムの例外処理](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI_002d34)、上: [SRFI サポート モジュール](https://doc.guix.gnu.org/guile/latest/en/guile.html#SRFI-Support) \[[目次](https://doc.guix.gnu.org/guile/latest/en/guile.html#SEC_Contents "目次")\]\[[索引](https://doc.guix.gnu.org/guile/latest/en/guile.html#R5RS-Index "索引")\]
