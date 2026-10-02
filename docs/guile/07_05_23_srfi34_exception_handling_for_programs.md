#### 7.5.23 SRFI-34 - プログラムの例外処理

Guile は、独自の組み込みメカニズムの代替として、[SRFI-34 の例外処理メカニズム](http://srfi.schemers.org/srfi-34/srfi-34.html) の実装を提供します ([例外](06_11_controlling_the_flow_of_program_execution.md#6118-例外) を参照)。これは次のようにして利用できます。

([use-modules](06_18_modules.md#6182-guileモジュールの使用) (srfi srfi-34))

`with-exception-handler` および `raise` (コア Guile では `raise-exception` として知られています) の詳細については、[例外の発生と処理](06_11_controlling_the_flow_of_program_execution.md#61182-例外の発生と処理) を参照してください。

SRFI-34の`guard`形式は、`with-exception-handler`に対するシンタックスシュガーです。

構文: **guard** (var 句 …) 本体 …

例外ハンドラを使用して本体を評価し、発生したオブジェクトを変数 var にバインドします。そして、そのバインドの範囲内で、句…を条件式の句であるかのように評価します。その暗黙の条件式は、ガード式の継続と動的な環境を使用して評価されます。

すべての節のテストが偽と評価され、かつ `else` 節がない場合、`raise` は、元の `raise` 呼び出しの動的環境内で、発生したオブジェクトに対して再度呼び出されます。ただし、現在の例外ハンドラは `guard` 式のものです。

* * *

次へ: [SRFI-37 - args-fold](07_05_25_srfi37_argsfold.md#7525-srfi-37---args-fold)、前: [SRFI-34 - プログラムの例外処理](#7523-srfi-34---プログラムの例外処理)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
