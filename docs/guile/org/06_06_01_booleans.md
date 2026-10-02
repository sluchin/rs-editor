#### 6.6.1 Booleans [¶](06_06_01_booleans.md#661-booleans)

The two boolean values are `#t` for true and `#f` for false. They can also be written as `#true` and `#false`, as per R7RS.

Boolean values are returned by predicate procedures, such as the general equality predicates `eq?`, `eqv?` and `equal?` (see [Equality](06_09_general_utility_functions.md#691-equality)) and numerical and string comparison operators like `string=?` (see [String Comparison](06_06_05_strings.md#6657-string-comparison)) and `<=` (see [Comparison Predicates](06_06_02_numerical_data_types.md#6628-comparison-predicates)).

([<=](06_06_02_numerical_data_types.md) 3 8)
⇒ #t

([<=](06_06_02_numerical_data_types.md) 3 \-3)
⇒ #f

([equal?](06_09_general_utility_functions.md) "house" "houses")
⇒ #f

([eq?](06_09_general_utility_functions.md) #f #f)
⇒
#t

In test condition contexts like `if` and `cond` (see [Simple Conditional Evaluation](06_11_controlling_the_flow_of_program_execution.md#6112-simple-conditional-evaluation)), where a group of subexpressions will be evaluated only if a condition expression evaluates to “true”, “true” means any value at all except `#f`.

(if #t "yes" "no")
⇒ "yes"

(if 0 "yes" "no")
⇒ "yes"

(if #f "yes" "no")
⇒ "no"

A result of this asymmetry is that typical Scheme source code more often uses `#f` explicitly than `#t`: `#f` is necessary to represent an `if` or `cond` false value, whereas `#t` is not necessary to represent an `if` or `cond` true value.

It is important to note that `#f` is **not** equivalent to any other Scheme value. In particular, `#f` is not the same as the number 0 (like in C and C++), and not the same as the “empty list” (like in some Lisp dialects).

In C, the two Scheme boolean values are available as the two constants `SCM_BOOL_T` for `#t` and `SCM_BOOL_F` for `#f`. Care must be taken with the false value `SCM_BOOL_F`: it is not false when used in C conditionals. In order to test for it, use `scm_is_false` or `scm_is_true`.

Scheme Procedure: **not** x [¶](06_06_01_booleans.md)

C Function: **scm\_not** (x) [¶](06_06_01_booleans.md)

Return `#t` if x is `#f`, else return `#f`.

Scheme Procedure: **boolean?** obj [¶](06_06_01_booleans.md)

C Function: **scm\_boolean\_p** (obj) [¶](06_06_01_booleans.md)

Return `#t` if obj is either `#t` or `#f`, else return `#f`.

C Macro: `SCM` **SCM\_BOOL\_T** [¶](06_06_01_booleans.md)

The `SCM` representation of the Scheme object `#t`.

C Macro: `SCM` **SCM\_BOOL\_F** [¶](06_06_01_booleans.md)

The `SCM` representation of the Scheme object `#f`.

C Function: `int` **scm\_is\_true** `(SCM obj)` [¶](06_06_01_booleans.md)

Return `0` if obj is `#f`, else return `1`.

C Function: `int` **scm\_is\_false** `(SCM obj)` [¶](06_06_01_booleans.md)

Return `1` if obj is `#f`, else return `0`.

C Function: `int` **scm\_is\_bool** `(SCM obj)` [¶](06_06_01_booleans.md)

Return `1` if obj is either `#t` or `#f`, else return `0`.

C Function: `SCM` **scm\_from\_bool** `(int val)` [¶](06_06_01_booleans.md)

Return `#f` if val is `0`, else return `#t`.

C Function: `int` **scm\_to\_bool** `(SCM val)` [¶](06_06_01_booleans.md)

Return `1` if val is `SCM_BOOL_T`, return `0` when val is `SCM_BOOL_F`, else signal a ‘wrong type’ error.

You should probably use `scm_is_true` instead of this function when you just want to test a `SCM` value for trueness.

* * *

Next: [Characters](06_06_03_characters.md#663-characters), Previous: [Booleans](06_06_01_booleans.md#661-booleans), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
