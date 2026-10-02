#### 7.5.22 SRFI-31 - 再帰評価のための特殊形式「rec」

SRFI-31では、自己参照式をより簡単に作成できる特別な形式が定義されています。構文は以下のとおりです。

<rec 式> --> (rec <変数> <式>)
<rec 式> --> (rec (<変数>+) <本体>)

最初の構文は、自己参照式を作成するために使用できます。例：

guile> (define tmp (rec ones ([cons](06_06_08_pairs.md#668-ペア) 1 (delay ones))))

2番目の構文は、匿名再帰関数を作成するために使用できます。

guile> (define tmp (rec (display-n item n)
([when](06_11_controlling_the_flow_of_program_execution.md#6112-単純な条件評価) ([positive?](06_06_02_numerical_data_types.md#6628-比較述語) n)
([display](06_16_reading_and_evaluating_scheme_code.md#6164-scheme値の記述) item) ([newline](06_12_input_and_output.md#61211-ヴェネラブルポートインターフェース))
(display-n item ([\-](06_06_02_numerical_data_types.md#66211-算術関数) n 1)))))
guile> (tmp 42 3)
42
42
42
策略>

* * *

次へ: [SRFI-35 - 条件](07_05_24_srfi35_conditions.md#7524-srfi-35---条件)、前: [SRFI-31 - 再帰評価のための特殊形式 'rec'](#7522-srfi-31---再帰評価のための特殊形式rec)、上: [SRFI サポート モジュール](07_05_00_srfi_support_modules.md#75-srfi-サポート-モジュール) \[[目次](00_contents.md "目次")\]\[[索引](index_r5rs.md "索引")\]
