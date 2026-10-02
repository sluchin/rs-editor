### 6.1 Overview of the Guile API [¶](06_01_overview_of_the_guile_api.md#61-overview-of-the-guile-api)

Guile’s application programming interface (_API_) makes functionality available that an application developer can use in either C or Scheme programming. The interface consists of _elements_ that may be macros, functions or variables in C, and procedures, variables, syntax or other types of object in Scheme.

Many elements are available to both Scheme and C, in a form that is appropriate. For example, the `assq` Scheme procedure is also available as `scm_assq` to C code. These elements are documented only once, addressing both the Scheme and C aspects of them.

The Scheme name of an element is related to its C name in a regular way. Also, a C function takes its parameters in a systematic way.

Normally, the name of a C function can be derived given its Scheme name, using some simple textual transformations:

*   Replace `-` (hyphen) with `_` (underscore).
*   Replace `?` (question mark) with `_p`.
*   Replace `!` (exclamation point) with `_x`.
*   Replace internal `->` with `_to_`.
*   Replace `<=` (less than or equal) with `_leq`.
*   Replace `>=` (greater than or equal) with `_geq`.
*   Replace `<` (less than) with `_less`.
*   Replace `>` (greater than) with `_gr`.
*   Prefix with `scm_`.

A C function always takes a fixed number of arguments of type `SCM`, even when the corresponding Scheme function takes a variable number.

For some Scheme functions, some last arguments are optional; the corresponding C function must always be invoked with all optional arguments specified. To get the effect as if an argument has not been specified, pass `SCM_UNDEFINED` as its value. You can not do this for an argument in the middle; when one argument is `SCM_UNDEFINED` all the ones following it must be `SCM_UNDEFINED` as well.

Some Scheme functions take an arbitrary number of _rest_ arguments; the corresponding C function must be invoked with a list of all these arguments. This list is always the last argument of the C function.

These two variants can also be combined.

The type of the return value of a C function that corresponds to a Scheme function is always `SCM`. In the descriptions below, types are therefore often omitted but for the return value and for the arguments.

* * *

Next: [The SCM Type](06_03_the_scm_type.md#63-the-scm-type), Previous: [Overview of the Guile API](06_01_overview_of_the_guile_api.md#61-overview-of-the-guile-api), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
