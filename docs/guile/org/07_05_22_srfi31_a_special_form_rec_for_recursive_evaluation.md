#### 7.5.22 SRFI-31 - A special form ‘rec’ for recursive evaluation [¶](07_05_22_srfi31_a_special_form_rec_for_recursive_evaluation.md#7522-srfi-31---a-special-form-rec-for-recursive-evaluation)

SRFI-31 defines a special form that can be used to create self-referential expressions more conveniently. The syntax is as follows:

<rec expression> --> (rec <variable> <expression>)
<rec expression> --> (rec (<variable>+) <body>)

The first syntax can be used to create self-referential expressions, for example:

  guile> (define tmp (rec ones ([cons](06_06_08_pairs.md) 1 (delay ones))))

The second syntax can be used to create anonymous recursive functions:

  guile> (define tmp (rec (display-n item n)
                       ([when](06_11_controlling_the_flow_of_program_execution.md) ([positive?](06_06_02_numerical_data_types.md) n)
                         ([display](06_16_reading_and_evaluating_scheme_code.md) item) ([newline](06_12_input_and_output.md))
                         (display-n item ([\-](06_06_02_numerical_data_types.md) n 1)))))
  guile> (tmp 42 3)
  42
  42
  42
  guile>

* * *

Next: [SRFI-35 - Conditions](07_05_24_srfi35_conditions.md#7524-srfi-35---conditions), Previous: [SRFI-31 - A special form ‘rec’ for recursive evaluation](07_05_22_srfi31_a_special_form_rec_for_recursive_evaluation.md#7522-srfi-31---a-special-form-rec-for-recursive-evaluation), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
