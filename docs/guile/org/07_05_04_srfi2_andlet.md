#### 7.5.4 SRFI-2 - and-let\* [¶](07_05_04_srfi2_andlet.md#754-srfi-2---and-let)

The following syntax can be obtained with

([use-modules](06_18_modules.md) (srfi srfi-2))

or alternatively

([use-modules](06_18_modules.md) (ice-9 and-let-star))

library syntax: **and-let\*** (clause …) body … [¶](07_05_04_srfi2_andlet.md)

A combination of `and` and `let*`.

Each clause is evaluated in turn, and if `#f` is obtained then evaluation stops and `#f` is returned. If all are non-`#f` then body is evaluated and the last form gives the return value, or if body is empty then the result is `#t`. Each clause should be one of the following,

`(symbol expr)`

Evaluate expr, check for `#f`, and bind it to symbol. Like `let*`, that binding is available to subsequent clauses.

`(expr)`

Evaluate expr and check for `#f`.

`symbol`

Get the value bound to symbol and check for `#f`.

Notice that `(expr)` has an “extra” pair of parentheses, for instance `((eq? x y))`. One way to remember this is to imagine the `symbol` in `(symbol expr)` is omitted.

`and-let*` is good for calculations where a `#f` value means termination, but where a non-`#f` value is going to be needed in subsequent expressions.

The following illustrates this, it returns text between brackets ‘\[...\]’ in a string, or `#f` if there are no such brackets (ie. either `string-index` gives `#f`).

(define (extract-brackets str)
  (and-let\* ((start (string-index str #\\\[))
             (end   (string-index str #\\\] start)))
    (substring str (1+ start) end)))

The following shows plain variables and expressions tested too. `diagnostic-levels` is taken to be an alist associating a diagnostic type with a level. `str` is printed only if the type is known and its level is high enough.

(define (show-diagnostic type str)
  (and-let\* (want-diagnostics
             (level (assq-ref diagnostic-levels type))
             ((>= level current-diagnostic-level)))
    (display str)))

The advantage of `and-let*` is that an extended sequence of expressions and tests doesn’t require lots of nesting as would arise from separate `and` and `let*`, or from `cond` with `=>`.

* * *

Next: [SRFI-6 - Basic String Ports](07_05_06_srfi6_basic_string_ports.md#756-srfi-6---basic-string-ports), Previous: [SRFI-2 - and-let\*](07_05_04_srfi2_andlet.md#754-srfi-2---and-let), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
