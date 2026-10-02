#### 7.5.2 SRFI-0 - cond-expand [¶](07_05_02_subsection_2.md#752-srfi-0---cond-expand)

This SRFI lets a portable Scheme program test for the presence of certain features, and adapt itself by using different blocks of code, or fail if the necessary features are not available. There’s no module to load, this is in the Guile core.

A program designed only for Guile will generally not need this mechanism, such a program can of course directly use the various documented parts of Guile.

syntax: **cond-expand** (feature body…) … [¶](07_05_02_subsection_2.md)

Expand to the body of the first clause whose feature specification is satisfied. It is an error if no feature is satisfied.

Features are symbols such as `srfi-1`, and a feature specification can use `and`, `or` and `not` forms to test combinations. The last clause can be an `else`, to be used if no other passes.

For example, define a private version of `alist-cons` if SRFI-1 is not available.

(cond-expand (srfi-1
              )
             (else
              (define (alist-cons key val alist)
                (cons (cons key val) alist))))

Or demand a certain set of SRFIs (list operations, string ports, `receive` and string operations), failing if they’re not available.

(cond-expand ((and srfi-1 srfi-6 srfi-8 srfi-13)
              ))

The Guile core has the following features,

guile
guile-2   ;; starting from Guile 2.x
guile-2.2 ;; starting from Guile 2.2
guile-3   ;; starting from Guile 3.x
guile-3.0 ;; starting from Guile 3.0
r5rs
r6rs
r7rs
exact-closed ieee-float full-unicode ratios ;; R7RS features
srfi-0
srfi-4
srfi-6
srfi-13
srfi-14
srfi-16
srfi-23
srfi-30
srfi-39
srfi-46
srfi-55
srfi-61
srfi-62
srfi-87
srfi-105

Other SRFI feature symbols are defined once their code has been loaded with `use-modules`, since only then are their bindings available.

The ‘\--use-srfi’ command line option (see [Invoking Guile](04_programming_in_scheme.md#42-invoking-guile)) is a good way to load SRFIs to satisfy `cond-expand` when running a portable program.

Testing the `guile` feature allows a program to adapt itself to the Guile module system, but still run on other Scheme systems. For example the following demands SRFI-8 (`receive`), but also knows how to load it with the Guile mechanism.

(cond-expand (srfi-8
              )
             (guile
              (use-modules (srfi srfi-8))))

Likewise, testing the `guile-2` feature allows code to be portable between Guile 2.x and previous versions of Guile. For instance, it makes it possible to write code that accounts for Guile 2.x’s compiler, yet be correctly interpreted on 1.8 and earlier versions:

(cond-expand (guile-2 (eval-when (compile)
                        ;; This must be evaluated at compile time.
                        (fluid-set! current-reader my-reader)))
             (guile
                      ;; Earlier versions of Guile do not have a
                      ;; separate compilation phase.
                      (fluid-set! current-reader my-reader)))

It should be noted that `cond-expand` is separate from the `*features*` mechanism (see [Feature Tracking](06_23_configuration_features_and_runtime_options.md#6232-feature-tracking)), feature symbols in one are unrelated to those in the other.

* * *

Next: [SRFI-2 - and-let\*](07_05_04_srfi2_andlet.md#754-srfi-2---and-let), Previous: [SRFI-0 - cond-expand](07_05_02_subsection_2.md#752-srfi-0---cond-expand), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
