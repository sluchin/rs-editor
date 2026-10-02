#### 7.5.18 SRFI-26 - specializing parameters [¶](07_05_18_srfi26_specializing_parameters.md#7518-srfi-26---specializing-parameters)

This SRFI provides a syntax for conveniently specializing selected parameters of a function. It can be used with,

(use-modules (srfi srfi-26))

library syntax: **cut** slot1 slot2 … [¶](07_05_18_srfi26_specializing_parameters.md)

library syntax: **cute** slot1 slot2 … [¶](07_05_18_srfi26_specializing_parameters.md)

Return a new procedure which will make a call (slot1 slot2 …) but with selected parameters specialized to given expressions.

An example will illustrate the idea. The following is a specialization of `write`, sending output to `my-output-port`,

(cut write <> my-output-port)
⇒
(lambda (obj) (write obj my-output-port))

The special symbol `<>` indicates a slot to be filled by an argument to the new procedure. `my-output-port` on the other hand is an expression to be evaluated and passed, ie. it specializes the behavior of `write`.

`<>`

A slot to be filled by an argument from the created procedure. Arguments are assigned to `<>` slots in the order they appear in the `cut` form, there’s no way to re-arrange arguments.

The first argument to `cut` is usually a procedure (or expression giving a procedure), but `<>` is allowed there too. For example,

(cut <> 1 2 3)
⇒
(lambda (proc) (proc 1 2 3))

`<...>`

A slot to be filled by all remaining arguments from the new procedure. This can only occur at the end of a `cut` form.

For example, a procedure taking a variable number of arguments like `max` but in addition enforcing a lower bound,

(define my-lower-bound 123)

(cut max my-lower-bound <...>)
⇒
(lambda arglist (apply max my-lower-bound arglist))

For `cut` the specializing expressions are evaluated each time the new procedure is called. For `cute` they’re evaluated just once, when the new procedure is created. The name `cute` stands for “`cut` with evaluated arguments”. In all cases the evaluations take place in an unspecified order.

The following illustrates the difference between `cut` and `cute`,

(cut format <> "the time is ~s" (current-time))
⇒
(lambda (port) (format port "the time is ~s" (current-time)))

(cute format <> "the time is ~s" (current-time))
⇒
(let ((val (current-time)))
  (lambda (port) (format port "the time is ~s" val))

(There’s no provision for a mixture of `cut` and `cute` where some expressions would be evaluated every time but others evaluated only once.)

`cut` is really just a shorthand for the sort of `lambda` forms shown in the above examples. But notice `cut` avoids the need to name unspecialized parameters, and is more compact. Use in functional programming style or just with `map`, `for-each` or similar is typical.

(map (cut \* 2 <>) '(1 2 3 4))

(for-each (cut write <> my-port) my-list)

* * *

Next: [SRFI-28 - Basic Format Strings](07_05_20_srfi28_basic_format_strings.md#7520-srfi-28---basic-format-strings), Previous: [SRFI-26 - specializing parameters](07_05_18_srfi26_specializing_parameters.md#7518-srfi-26---specializing-parameters), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
