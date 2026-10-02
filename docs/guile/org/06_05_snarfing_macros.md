### 6.5 Snarfing Macros [¶](06_05_snarfing_macros.md#65-snarfing-macros)

The following macros do two different things: when compiled normally, they expand in one way; when processed during snarfing, they cause the `guile-snarf` program to pick up some initialization code, See [Function Snarfing](05_programming_in_c.md#56-function-snarfing).

The descriptions below use the term ‘normally’ to refer to the case when the code is compiled normally, and ‘while snarfing’ when the code is processed by `guile-snarf`.

C Macro: **SCM\_SNARF\_INIT** (code) [¶](06_05_snarfing_macros.md)

Normally, `SCM_SNARF_INIT` expands to nothing; while snarfing, it causes code to be included in the initialization action file, followed by a semicolon.

This is the fundamental macro for snarfing initialization actions. The more specialized macros below use it internally.

C Macro: **SCM\_DEFINE** (c\_name, scheme\_name, req, opt, var, arglist, docstring) [¶](06_05_snarfing_macros.md)

Normally, this macro expands into

static const char s\_c\_name\[\] = scheme\_name;
SCM
c\_name arglist

While snarfing, it causes

scm\_c\_define\_gsubr (s\_c\_name, req, opt, var,
                    c\_name);

to be added to the initialization actions. Thus, you can use it to declare a C function named c\_name that will be made available to Scheme with the name scheme\_name.

Note that the arglist argument must have parentheses around it.

C Macro: **SCM\_SYMBOL** (c\_name, scheme\_name) [¶](06_05_snarfing_macros.md)

C Macro: **SCM\_GLOBAL\_SYMBOL** (c\_name, scheme\_name) [¶](06_05_snarfing_macros.md)

Normally, these macros expand into

static SCM c\_name

or

SCM c\_name

respectively. While snarfing, they both expand into the initialization code

c\_name = scm\_permanent\_object (scm\_from\_locale\_symbol (scheme\_name));

Thus, you can use them declare a static or global variable of type `SCM` that will be initialized to the symbol named scheme\_name.

C Macro: **SCM\_KEYWORD** (c\_name, scheme\_name) [¶](06_05_snarfing_macros.md)

C Macro: **SCM\_GLOBAL\_KEYWORD** (c\_name, scheme\_name) [¶](06_05_snarfing_macros.md)

Normally, these macros expand into

static SCM c\_name

or

SCM c\_name

respectively. While snarfing, they both expand into the initialization code

c\_name = scm\_permanent\_object (scm\_c\_make\_keyword (scheme\_name));

Thus, you can use them declare a static or global variable of type `SCM` that will be initialized to the keyword named scheme\_name.

C Macro: **SCM\_VARIABLE** (c\_name, scheme\_name) [¶](06_05_snarfing_macros.md)

C Macro: **SCM\_GLOBAL\_VARIABLE** (c\_name, scheme\_name) [¶](06_05_snarfing_macros.md)

These macros are equivalent to `SCM_VARIABLE_INIT` and `SCM_GLOBAL_VARIABLE_INIT`, respectively, with a value of `SCM_BOOL_F`.

C Macro: **SCM\_VARIABLE\_INIT** (c\_name, scheme\_name, value) [¶](06_05_snarfing_macros.md)

C Macro: **SCM\_GLOBAL\_VARIABLE\_INIT** (c\_name, scheme\_name, value) [¶](06_05_snarfing_macros.md)

Normally, these macros expand into

static SCM c\_name

or

SCM c\_name

respectively. While snarfing, they both expand into the initialization code

c\_name = scm\_permanent\_object (scm\_c\_define (scheme\_name, value));

Thus, you can use them declare a static or global C variable of type `SCM` that will be initialized to the object representing the Scheme variable named scheme\_name in the current module. The variable will be defined when it doesn’t already exist. It is always set to value.

* * *

Next: [Procedures](06_07_procedures.md#67-procedures), Previous: [Snarfing Macros](06_05_snarfing_macros.md#65-snarfing-macros), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
