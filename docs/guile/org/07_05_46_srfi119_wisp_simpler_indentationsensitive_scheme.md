#### 7.5.46 SRFI-119 Wisp: simpler indentation-sensitive Scheme. [¶](07_05_46_srfi119_wisp_simpler_indentationsensitive_scheme.md#7546-srfi-119-wisp-simpler-indentation-sensitive-scheme)

The languages shipped in Guile include SRFI-119, also referred to as _Wisp_ (for “Whitespace to Lisp”), an encoding of Scheme that allows replacing parentheses with equivalent indentation and inline colons. See [the specification of SRFI-119](http://srfi.schemers.org/srfi-119/srfi-119.html). Some examples:

display "Hello World!"         ⇒  (display "Hello World!")

define : factorial n           ⇒  (define (factorial n)
    if : zero? n               ⇒      (if (zero? n)
       . 1                     ⇒          1
       \* n : factorial {n - 1} ⇒    (\* n (factorial {n - 1}))))

To execute a file with Wisp code, select the language and filename extension `.w` vie `guile --language=wisp -x .w`.

In files using Wisp, See [SRFI-105 Curly-infix expressions.](07_05_44_srfi105_curlyinfix_expressions.md#7544-srfi-105-curly-infix-expressions) (Curly Infix) is always activated.

* * *

Next: [SRFI-197: Pipeline Operators](07_05_48_srfi197_pipeline_operators.md#7548-srfi-197-pipeline-operators), Previous: [SRFI-119 Wisp: simpler indentation-sensitive Scheme.](07_05_46_srfi119_wisp_simpler_indentationsensitive_scheme.md#7546-srfi-119-wisp-simpler-indentation-sensitive-scheme), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
