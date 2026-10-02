#### 7.5.10 SRFI-11 - let-values [¶](07_05_10_srfi11_letvalues.md#7510-srfi-11---let-values)

This module implements the binding forms for multiple values `let-values` and `let*-values`. These forms are similar to `let` and `let*` (see [Local Variable Bindings](06_10_definitions_and_variable_bindings.md#6102-local-variable-bindings)), but they support binding of the values returned by multiple-valued expressions.

Write `(use-modules (srfi srfi-11))` to make the bindings available.

(let-values (((x y) ([values](06_11_controlling_the_flow_of_program_execution.md) 1 2))
             ((z f) ([values](06_11_controlling_the_flow_of_program_execution.md) 3 4)))
   ([+](06_06_02_numerical_data_types.md) x y z f))
⇒
10

`let-values` performs all bindings simultaneously, which means that no expression in the binding clauses may refer to variables bound in the same clause list. `let*-values`, on the other hand, performs the bindings sequentially, just like `let*` does for single-valued expressions.

* * *

Next: [SRFI-14 - Character-set Library](07_05_12_srfi14_characterset_library.md#7512-srfi-14---character-set-library), Previous: [SRFI-11 - let-values](07_05_10_srfi11_letvalues.md#7510-srfi-11---let-values), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
