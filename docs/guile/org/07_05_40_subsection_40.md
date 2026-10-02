#### 7.5.40 SRFI-71 - Extended let-syntax for multiple values [¶](07_05_40_subsection_40.md#7540-srfi-71---extended-let-syntax-for-multiple-values)

This SRFI shadows the forms for `let`, `let*`, and `letrec` so that they may accept multiple values. For example:

(use-modules (srfi srfi-71))

(let\* ((x y (values 1 2))
       (z (+ x y)))
  (\* z 2))
⇒ 6

See [the specification of SRFI-71](http://srfi.schemers.org/srfi-71/srfi-71.html).

* * *

Next: [SRFI-88 Keyword Objects](07_05_42_srfi88_keyword_objects.md#7542-srfi-88-keyword-objects), Previous: [SRFI-71 - Extended let-syntax for multiple values](07_05_40_subsection_40.md#7540-srfi-71---extended-let-syntax-for-multiple-values), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
