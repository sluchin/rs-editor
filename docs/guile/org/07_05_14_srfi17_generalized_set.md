#### 7.5.14 SRFI-17 - Generalized set! [¶](07_05_14_srfi17_generalized_set.md#7514-srfi-17---generalized-set)

This SRFI implements a generalized `set!`, allowing some “referencing” functions to be used as the target location of a `set!`. This feature is available from

(use-modules (srfi srfi-17))

For example `vector-ref` is extended so that

(set! (vector-ref vec idx) new-value)

is equivalent to

(vector-set! vec idx new-value)

The idea is that a `vector-ref` expression identifies a location, which may be either fetched or stored. The same form is used for the location in both cases, encouraging visual clarity. This is similar to the idea of an “lvalue” in C.

The mechanism for this kind of `set!` is in the Guile core (see [Procedures with Setters](06_07_procedures.md#678-procedures-with-setters)). This module adds definitions of the following functions as procedures with setters, allowing them to be targets of a `set!`,

> `car`, `cdr`, `caar`, `cadr`, `cdar`, `cddr`, `caaar`, `caadr`, `cadar`, `caddr`, `cdaar`, `cdadr`, `cddar`, `cdddr`, `caaaar`, `caaadr`, `caadar`, `caaddr`, `cadaar`, `cadadr`, `caddar`, `cadddr`, `cdaaar`, `cdaadr`, `cdadar`, `cdaddr`, `cddaar`, `cddadr`, `cdddar`, `cddddr`
> 
> `string-ref`, `vector-ref`

The SRFI specifies `setter` (see [Procedures with Setters](06_07_procedures.md#678-procedures-with-setters)) as a procedure with setter, allowing the setter for a procedure to be changed, eg. `(set! (setter foo) my-new-setter-handler)`. Currently Guile does not implement this, a setter can only be specified on creation (`getter-with-setter` below).

Function: **getter-with-setter** [¶](07_05_14_srfi17_generalized_set.md)

The same as the Guile core `make-procedure-with-setter` (see [Procedures with Setters](06_07_procedures.md#678-procedures-with-setters)).

* * *

Next: [SRFI-19 - Time/Date Library](07_05_16_srfi19_timedate_library.md#7516-srfi-19---timedate-library), Previous: [SRFI-17 - Generalized set!](07_05_14_srfi17_generalized_set.md#7514-srfi-17---generalized-set), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
