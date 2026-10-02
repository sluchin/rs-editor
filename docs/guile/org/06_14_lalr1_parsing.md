### 6.14 LALR(1) Parsing [¶](06_14_lalr1_parsing.md#614-lalr1-parsing)

The `(system base lalr)` module provides the [`lalr-scm` LALR(1) parser generator by Dominique Boucher](https://github.com/schemeway/lalr-scm/). `lalr-scm` uses the same algorithm as GNU Bison (see [Introduction to Bison](https://www.gnu.org/software/bison/manual/bison.html#Introduction) in Bison, The Yacc-compatible Parser Generator). Parsers are defined using the `lalr-parser` macro.

Scheme Syntax: **lalr-parser** \[options\] tokens rules... [¶](06_14_lalr1_parsing.md)

Generate an LALR(1) syntax analyzer. tokens is a list of symbols representing the terminal symbols of the grammar. rules are the grammar production rules.

Each rule has the form `(non-terminal (rhs ...) : action ...)`, where non-terminal is the name of the rule, rhs are the right-hand sides, i.e., the production rule, and action is a semantic action associated with the rule.

The generated parser is a two-argument procedure that takes a _tokenizer_ and a _syntax error procedure_. The tokenizer should be a thunk that returns lexical tokens as produced by `make-lexical-token`. The syntax error procedure may be called with at least an error message (a string), and optionally the lexical token that caused the error.

Please refer to the `lalr-scm` documentation for details.

* * *

Next: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code), Previous: [LALR(1) Parsing](06_14_lalr1_parsing.md#614-lalr1-parsing), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
