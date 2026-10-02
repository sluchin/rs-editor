#### 7.5.44 SRFI-105 Curly-infix expressions. [¶](07_05_44_srfi105_curlyinfix_expressions.md#7544-srfi-105-curly-infix-expressions)

Guile’s built-in reader includes support for SRFI-105 curly-infix expressions. See [the specification of SRFI-105](http://srfi.schemers.org/srfi-105/srfi-105.html). Some examples:

{n <= 5}                ⇒  (<= n 5)
{a + b + c}             ⇒  (+ a b c)
{a \* {b + c}}           ⇒  (\* a (+ b c))
{(- a) / b}             ⇒  (/ (- a) b)
{-(a) / b}              ⇒  (/ (- a) b) as well
{(f a b) + (g h)}       ⇒  (+ (f a b) (g h))
{f(a b) + g(h)}         ⇒  (+ (f a b) (g h)) as well
{f\[a b\] + g(h)}         ⇒  (+ ($bracket-apply$ f a b) (g h))
'{a + f(b) + x}         ⇒  '(+ a (f b) x)
{length(x) >= 6}        ⇒  (>= (length x) 6)
{n-1 + n-2}             ⇒  (+ n-1 n-2)
{n \* factorial{n - 1}}  ⇒  (\* n (factorial (- n 1)))
{{a > 0} and {b >= 1}}  ⇒  (and (> a 0) (>= b 1))
{f{n - 1}(x)}           ⇒  ((f (- n 1)) x)
{a . z}                 ⇒  ($nfx$ a . z)
{a + b - c}             ⇒  ($nfx$ a + b - c)

To enable curly-infix expressions within a file, place the reader directive `#!curly-infix` before the first use of curly-infix notation. To globally enable curly-infix expressions in Guile’s reader, set the `curly-infix` read option.

Guile also implements the following non-standard extension to SRFI-105: if `curly-infix` is enabled and there is no other meaning assigned to square brackets (i.e. the `square-brackets` read option is turned off), then lists within square brackets are read as normal lists but with the special symbol `$bracket-list$` added to the front. To enable this combination of read options within a file, use the reader directive `#!curly-infix-and-bracket-lists`. For example:

\[a b\]    ⇒  ($bracket-list$ a b)
\[a . b\]  ⇒  ($bracket-list$ a . b)

For more information on reader options, See [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code).

* * *

Next: [SRFI-119 Wisp: simpler indentation-sensitive Scheme.](07_05_46_srfi119_wisp_simpler_indentationsensitive_scheme.md#7546-srfi-119-wisp-simpler-indentation-sensitive-scheme), Previous: [SRFI-105 Curly-infix expressions.](07_05_44_srfi105_curlyinfix_expressions.md#7544-srfi-105-curly-infix-expressions), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
