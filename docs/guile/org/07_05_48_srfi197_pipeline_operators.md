#### 7.5.48 SRFI-197: Pipeline Operators [¶](07_05_48_srfi197_pipeline_operators.md#7548-srfi-197-pipeline-operators)

[SRFI-197](http://srfi.schemers.org/srfi-197/srfi-197.html) provides functional pipeline (threading) operators like Clojure’s `->` or OCaml’s `|>`. Pipelines are a simple, terse, and readable way to write deeply-nested expressions. This SRFI defines a family of chain and nest pipeline operators, which can rewrite nested expressions like `(a b (c d (e f g)))` as a sequence of operations: `(chain g (e f _) (c d _) (a b _))`.

Note that Like `let*`, `chain` guarantees evaluation order. In fact, `(chain a (b _) (c _))` expands to something like `(let* ((x (b a)) (x (c x))) x)`, not `(c (b a))`, and so `chain` is not suitable for pipelines containing syntax like `if` or `let`.

For pipelines containing complex syntax, the `nest` and `nest-reverse` operators look like `chain` but are guaranteed to expand to nested forms, not `let*` forms. `nest` nests in the opposite direction of `chain`, so `(nest (a _) (b _) c)` expands to `(a (b c))`.

Scheme Syntax: **chain** initial-value \[placeholder \[ellipsis\]\] step … [¶](07_05_48_srfi197_pipeline_operators.md)

Converts earch part into a sequence of small integers and returns a bytevector of the corresponding bytes as follows:

initial-value is an expression.

placeholder and ellipsis are literal symbols; these are the placeholder symbol and ellipsis symbol. If placeholder or ellipsis are not present, they default to `_` and `...`, respectively.

The syntax of step is (datum …), where each datum is either the placeholder symbol, the ellipsis symbol, or an expression. A step must contain at least one datum. The ellipsis symbol is only allowed at the end of a step, and it must immediately follow a placeholder symbol.

Semantics: `chain` evaluates each step in order from left to right, passing the result of each step to the next.

Each step is evaluated as an application, and the return value(s) of that application are passed to the next step as its pipeline values. initial-value is the pipeline value of the first step. The return value(s) of `chain` are the return value(s) of the last step.

The placeholder symbols in each step are replaced with that step’s pipeline values, in the order they appear. It is an error if the number of placeholders for a step does not equal the number of pipeline values for that step, unless the step contains no placeholders, in which case it will ignore its pipeline values.

([chain](07_05_48_srfi197_pipeline_operators.md) x (a b [\_](06_08_macros.md)))
  ⇒ (a b x)
([chain](07_05_48_srfi197_pipeline_operators.md) (a b) (c [\_](06_08_macros.md) d) (e f [\_](06_08_macros.md)))
  ⇒ (let\* ((x (a b)) (x (c x d))) (e f x))
([chain](07_05_48_srfi197_pipeline_operators.md) (a) (b [\_](06_08_macros.md) [\_](06_08_macros.md)) (c [\_](06_08_macros.md)))
  ⇒ (let\*-values (((x1 x2) (a)) ((x) (b x1 x2))) (c x))

If a step ends with a placeholder symbol followed by an ellipsis symbol, that placeholder sequence is replaced with all remaining pipeline values that do not have a matching placeholder.

([chain](07_05_48_srfi197_pipeline_operators.md) (a) (b [\_](06_08_macros.md) c [\_](06_08_macros.md) [...](06_08_macros.md)) (d [\_](06_08_macros.md)))
  ⇒ (let\*-values (((x1 . x2) (a)) ((x) ([apply](06_16_reading_and_evaluating_scheme_code.md) b x1 c x2))) (d x))

`chain` and all other SRFI 197 macros support custom placeholder symbols, which can help to preserve hygiene when used in the body of a syntax definition that may insert a `_` or `....`

([chain](07_05_48_srfi197_pipeline_operators.md) (a b) <> (c <> d) (e f <>))
  ⇒ (let\* ((x (a b)) (x (c x d))) (e f x))
([chain](07_05_48_srfi197_pipeline_operators.md) (a) [\-](06_06_02_numerical_data_types.md) \--- (b [\-](06_06_02_numerical_data_types.md) c [\-](06_06_02_numerical_data_types.md) \---) (d [\-](06_06_02_numerical_data_types.md)))
  ⇒ (let\*-values (((x1 . x2) (a)) ((x) ([apply](06_16_reading_and_evaluating_scheme_code.md) b x1 c x2))) (d x))

Scheme Syntax: **chain-and** initial-value \[placeholder\] step … [¶](07_05_48_srfi197_pipeline_operators.md)

initial-value is an expression. placeholder is a literal symbol; this is the placeholder symbol. If placeholder is not given, the placeholder symbol is `_`. The syntax of step is (datum … \[placeholder datum …\]).

Semantics: A variant of `chain` that short-circuits and returns `#f` if any step returns `#f`. `chain-and` is to `chain` as SRFI 2 `and-let*` is to `let*`.

Each step is evaluated as an application. If the step evaluates to `#f`, the remaining steps are not evaluated, and `chain-and` returns #f. Otherwise, the return value of the step is passed to the next step as its pipeline value. initial-value is the pipeline value of the first step. If no step evaluates to `#f`, the return value of `chain-and` is the return value of the last step.

The placeholder placeholder in each step is replaced with that step’s pipeline value. If a step does not contain placeholder, it will ignore its pipeline value, but `chain-and` will still check whether that pipeline value is `#f`.

Because `chain-and` checks the return value of each step, it does not support steps with multiple return values. It is an error if a step returns more than one value.

Scheme Syntax: **chain-when** initial-value \[placeholder\] (\[guard\] step) … [¶](07_05_48_srfi197_pipeline_operators.md)

initial-value and guard are expressions. placeholder is a literal symbol; this is the placeholder symbol. If placeholder is not present, the placeholder symbol is `_`. The syntax of step is (datum … \[placeholder datum …\]).

Semantics: A variant of `chain` in which each step has a guard expression and will be skipped if the guard expression evaluates to #f.

(define (describe-number n)
  ([chain-when](07_05_48_srfi197_pipeline_operators.md) '()
    (([odd?](06_06_02_numerical_data_types.md) n) ([cons](06_06_08_pairs.md) "odd" [\_](06_08_macros.md)))
    (([even?](06_06_02_numerical_data_types.md) n) ([cons](06_06_08_pairs.md) "even" [\_](06_08_macros.md)))
    (([zero?](06_06_02_numerical_data_types.md) n) ([cons](06_06_08_pairs.md) "zero" [\_](06_08_macros.md)))
    (([positive?](06_06_02_numerical_data_types.md) n) ([cons](06_06_08_pairs.md) "positive" [\_](06_08_macros.md)))))

(describe-number 3) ; => '("positive" "odd")
(describe-number 4) ; => '("positive" "even")

Each step is evaluated as an application. The return value of the step is passed to the next step as its pipeline value. initial-value is the pipeline value of the first step.

The placeholder placeholder in each step is replaced with that step’s pipeline value. If a step does not contain placeholder, it will ignore its pipeline value.

If a step’s guard is present and evaluates to `#f`, that step will be skipped, and its pipeline value will be reused as the pipeline value of the next step. The return value of `chain-when` is the return value of the last non-skipped step, or initial-value if all steps are skipped.

Because `chain-when` may skip steps, it does not support steps with multiple return values. It is an error if a step returns more than one value.

Scheme Syntax: **chain-lambda** \[placeholder \[ellipsis\]\] step … [¶](07_05_48_srfi197_pipeline_operators.md)

placeholder and ellipsis are literal symbols; these are the placeholder symbol and ellipsis symbol. If placeholder or ellipsis are not present, they default to `_` and `...`, respectively.

The syntax of step is (datum …), where each datum is either the placeholder symbol, the ellipsis symbol, or an expression. A step must contain at least one datum. The ellipsis symbol is only allowed at the end of a step, and it must immediately follow a placeholder symbol.

Semantics: Creates a procedure from a sequence of `chain` steps. When called, a `chain-lambda` procedure evaluates each step in order from left to right, passing the result of each step to the next.

([chain-lambda](07_05_48_srfi197_pipeline_operators.md) (a [\_](06_08_macros.md)) (b [\_](06_08_macros.md)))
  ⇒ (lambda (x) (let\* ((x (a x))) (b x)))
([chain-lambda](07_05_48_srfi197_pipeline_operators.md) (a [\_](06_08_macros.md) [\_](06_08_macros.md)) (b c [\_](06_08_macros.md)))
  ⇒ (lambda (x1 x2) (let\* ((x (a x1 x2))) (b c x)))

Each step is evaluated as an application, and the return value(s) of that application are passed to the next step as its pipeline values. The procedure’s arguments are the pipeline values of the first step. The return value(s) of the procedure are the return value(s) of the last step.

The placeholder symbols in each step are replaced with that step’s pipeline values, in the order they appear. It is an error if the number of placeholders for a step does not equal the number of pipeline values for that step, unless the step contains no placeholders, in which case it will ignore its pipeline values.

If a step ends with a placeholder symbol followed by an ellipsis symbol, that placeholder sequence is replaced with all remaining pipeline values that do not have a matching placeholder.

The number of placeholders in the first step determines the arity of the procedure. If the first step ends with an ellipsis symbol, the procedure is variadic.

Scheme Syntax: **nest** \[placeholder\] <step> … initial-value [¶](07_05_48_srfi197_pipeline_operators.md)

placeholder is a literal symbol; this is the placeholder symbol. If placeholder is not present, the placeholder symbol is `_`. The syntax of step is (datum … placeholder datum …). initial-value is an expression.

Semantics: `nest` is similar to `chain`, but sequences its steps in the opposite order. Unlike `chain`, `nest` literally nests expressions; as a result, it does not provide the same strict evaluation order guarantees as `chain`.

([nest](07_05_48_srfi197_pipeline_operators.md) (a b [\_](06_08_macros.md)) (c d [\_](06_08_macros.md)) e) ; => (a b (c d e))

A `nest` expression is evaluated by lexically replacing the placeholder in the last step with initial-value, then replacing the placeholder in the next-to-last step with that replacement, and so on until the placeholder in the first step has been replaced. It is an error if the resulting final replacement is not an expression, which is then evaluated and its values are returned.

Because it produces an actual nested form, `nest` can build expressions that `chain` cannot. For example, `nest` can build a quoted data structure:

([nest](07_05_48_srfi197_pipeline_operators.md) '\_ (1 2 [\_](06_08_macros.md)) (3 [\_](06_08_macros.md) 5) ([\_](06_08_macros.md)) 4) ; => '(1 2 (3 (4) 5))

`nest` can also safely include special forms like if, let, lambda, or parameterize in a pipeline.

A custom placeholder can be used to safely nest `nest` expressions.

([nest](07_05_48_srfi197_pipeline_operators.md) ([nest](07_05_48_srfi197_pipeline_operators.md) \_2 '\_2 (1 2 3 \_2) [\_](06_08_macros.md) 6)
      ([\_](06_08_macros.md) 5 \_2)
      4)
  ⇒ '(1 2 3 (4 5 6))

Scheme Syntax: **nest-reverse** initial-value \[placeholder\] step … [¶](07_05_48_srfi197_pipeline_operators.md)

Syntax: initial-value is an expression. placeholder is a literal symbol; this is the placeholder symbol. If placeholder is not present, the placeholder symbol is `_`. The syntax of step is (datum … placeholder datum …).

Semantics: `nest-reverse` is variant of `nest` that nests in reverse order, which is the same order as `chain`.

([nest-reverse](07_05_48_srfi197_pipeline_operators.md) e (c d [\_](06_08_macros.md)) (a b [\_](06_08_macros.md))) ; => (a b (c d e))

A `nest-reverse` expression is evaluated by lexically replacing the placeholder in the first step with initial-value, then replacing the placeholder in the second step with that replacement, and so on until the placeholder in the last step has been replaced. It is an error if the resulting final replacement is not an expression, which is then evaluated and its values are returned.

*   [Acknowledgements](07_05_48_srfi197_pipeline_operators.md#75481-acknowledgements)

#### 7.5.48.1 Acknowledgements [¶](07_05_48_srfi197_pipeline_operators.md#75481-acknowledgements)

Thanks to the participants in the SRFI 197 mailing list who helped Adam Nelson refine this SRFI, including Marc Nieper-Wißkirchen, Linus Björnstam, Shiro Kawai, Lassi Kortela, and John Cowan.

Marc provided a paragraph that has been included (with only minor changes) in the Semantics section of the nest and nest-reverse macros.

Thanks to Rich Hickey for [Clojure](https://clojure.org/) and the original implementation of Clojure threading macros, and to Paulus Esterhazy for the (EPL licensed) threading macros documentation page, which was a source of inspiration and some of the examples in this document.

* * *

Next: [SRFI-244 - Multiple-value Definitions](07_05_50_srfi244_multiplevalue_definitions.md#7550-srfi-244---multiple-value-definitions), Previous: [SRFI-197: Pipeline Operators](07_05_48_srfi197_pipeline_operators.md#7548-srfi-197-pipeline-operators), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
