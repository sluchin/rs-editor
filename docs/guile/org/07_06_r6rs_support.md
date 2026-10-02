### 7.6 R6RS Support [¶](07_06_r6rs_support.md#76-r6rs-support)

See [R6RS Libraries](06_18_modules.md#6186-r6rs-libraries), for more information on how to define R6RS libraries, and their integration with Guile modules.

*   [Incompatibilities with the R6RS](07_06_r6rs_support.md#761-incompatibilities-with-the-r6rs)
*   [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)

* * *

Next: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries), Up: [R6RS Support](07_06_r6rs_support.md#76-r6rs-support)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.1 Incompatibilities with the R6RS [¶](07_06_r6rs_support.md#761-incompatibilities-with-the-r6rs)

There are some incompatibilities between Guile and the R6RS. Some of them are intentional, some of them are bugs, and some are simply unimplemented features. Please let the Guile developers know if you find one that is not on this list.

*   The R6RS specifies many situations in which a conforming implementation must signal a specific error. Guile doesn’t really care about that too much—if a correct R6RS program would not hit that error, we don’t bother checking for it.
*   Multiple `library` forms in one file are not yet supported. This is because the expansion of `library` sets the current module, but does not restore it. This is a bug.
*   R6RS unicode escapes within strings are disabled by default, because they conflict with Guile’s already-existing escapes. The same is the case for R6RS treatment of escaped newlines in strings.
    
    R6RS behavior can be turned on via a reader option. See [String Read Syntax](06_06_05_strings.md#6651-string-read-syntax), for more information.
    
*   Guile does not yet support Unicode escapes in symbols, such as `H\x65;llo` (the same as `Hello`), or `\x3BB;` (the same as `λ`).
*   A `set!` to a variable transformer may only expand to an expression, not a definition—even if the original `set!` expression was in definition context.
*   Instead of using the algorithm detailed in chapter 10 of the R6RS, expansion of toplevel forms happens sequentially.
    
    For example, while the expansion of the following set of toplevel definitions does the correct thing:
    
    (begin
     (define even?
       (lambda (x)
         (or (= x 0) (odd? (- x 1)))))
     (define-syntax odd?
       (syntax-rules ()
         ((odd? x) (not (even? x)))))
     (even? 10))
    ⇒ #t
    
    The same definitions outside of the `begin` wrapper do not:
    
    (define even?
      (lambda (x)
        (or (= x 0) (odd? (- x 1)))))
    (define-syntax odd?
      (syntax-rules ()
        ((odd? x) (not (even? x)))))
    (even? 10)
    <unnamed port>:4:18: In procedure even?:
    <unnamed port>:4:18: Wrong type to apply: #<syntax-transformer odd?>
    
    This is because when expanding the right-hand-side of `even?`, the reference to `odd?` is not yet marked as a syntax transformer, so it is assumed to be a function.
    
    This bug will only affect top-level programs, not code in `library` forms. Fixing it for toplevel forms seems doable, but tricky to implement in a backward-compatible way. Suggestions and/or patches would be appreciated.
    
*   The `(rnrs io ports)` module is incomplete. Work is ongoing to fix this.
*   Guile does not prevent use of textual I/O procedures on binary ports, or vice versa. All ports in Guile support both binary and textual I/O. See [Encoding](06_12_input_and_output.md#6123-encoding), for full details.
*   Guile’s implementation of `equal?` may fail to terminate when applied to arguments containing cycles.

Guile exposes a procedure in the root module to choose R6RS defaults over Guile’s historical defaults.

Scheme Procedure: **install-r6rs!** [¶](07_06_r6rs_support.md)

Alter Guile’s default settings to better conform to the R6RS.

While Guile’s defaults may evolve over time, the current changes that this procedure imposes are to add `.sls` and `.guile.sls` to the set of supported `%load-extensions`, to better support R6RS conventions. See [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths). Also, enable R6RS unicode escapes in strings; see the discussion above.

Finally, note that the `--r6rs` command-line argument will call `install-r6rs!` before calling user code. R6RS users probably want to pass this argument to their Guile.

* * *

Previous: [Incompatibilities with the R6RS](07_06_r6rs_support.md#761-incompatibilities-with-the-r6rs), Up: [R6RS Support](07_06_r6rs_support.md#76-r6rs-support)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2 R6RS Standard Libraries [¶](07_06_r6rs_support.md#762-r6rs-standard-libraries)

In contrast with earlier versions of the Revised Report, the R6RS organizes the procedures and syntactic forms required of conforming implementations into a set of “standard libraries” which can be imported as necessary by user programs and libraries. Here we briefly list the libraries that have been implemented for Guile.

We do not attempt to document these libraries fully here, as most of their functionality is already available in Guile itself. The expectation is that most Guile users will use the well-known and well-documented Guile modules. These R6RS libraries are mostly useful to users who want to port their code to other R6RS systems.

The documentation in the following sections reproduces some of the content of the library section of the Report, but is mostly intended to provide supplementary information about Guile’s implementation of the R6RS standard libraries. For complete documentation, design rationales and further examples, we advise you to consult the “Standard Libraries” section of the Report (see [R6RS Standard Libraries](https://doc.guix.gnu.org/guile/latest/en/r6rs.html#Standard-Libraries) in The Revised^6 Report on the Algorithmic Language Scheme).

*   [Library Usage](07_06_r6rs_support.md#7621-library-usage)
*   [rnrs base](07_06_r6rs_support.md#7622-rnrs-base)
*   [rnrs unicode](07_06_r6rs_support.md#7623-rnrs-unicode)
*   [rnrs bytevectors](07_06_r6rs_support.md#7624-rnrs-bytevectors)
*   [rnrs lists](07_06_r6rs_support.md#7625-rnrs-lists)
*   [rnrs sorting](07_06_r6rs_support.md#7626-rnrs-sorting)
*   [rnrs control](07_06_r6rs_support.md#7627-rnrs-control)
*   [R6RS Records](07_06_r6rs_support.md#7628-r6rs-records)
*   [rnrs records syntactic](07_06_r6rs_support.md#7629-rnrs-records-syntactic)
*   [rnrs records procedural](07_06_r6rs_support.md#76210-rnrs-records-procedural)
*   [rnrs records inspection](07_06_r6rs_support.md#76211-rnrs-records-inspection)
*   [rnrs exceptions](07_06_r6rs_support.md#76212-rnrs-exceptions)
*   [rnrs conditions](07_06_r6rs_support.md#76213-rnrs-conditions)
*   [I/O Conditions](07_06_r6rs_support.md#76214-io-conditions)
*   [Transcoders](07_06_r6rs_support.md#76215-transcoders)
*   [rnrs io ports](07_06_r6rs_support.md#76216-rnrs-io-ports)
*   [R6RS File Ports](07_06_r6rs_support.md#76217-r6rs-file-ports)
*   [rnrs io simple](07_06_r6rs_support.md#76218-rnrs-io-simple)
*   [rnrs files](07_06_r6rs_support.md#76219-rnrs-files)
*   [rnrs programs](07_06_r6rs_support.md#76220-rnrs-programs)
*   [rnrs arithmetic fixnums](07_06_r6rs_support.md#76221-rnrs-arithmetic-fixnums)
*   [rnrs arithmetic flonums](07_06_r6rs_support.md#76222-rnrs-arithmetic-flonums)
*   [rnrs arithmetic bitwise](07_06_r6rs_support.md#76223-rnrs-arithmetic-bitwise)
*   [rnrs syntax-case](07_06_r6rs_support.md#76224-rnrs-syntax-case)
*   [rnrs hashtables](07_06_r6rs_support.md#76225-rnrs-hashtables)
*   [rnrs enums](07_06_r6rs_support.md#76226-rnrs-enums)
*   [rnrs](07_06_r6rs_support.md#76227-rnrs)
*   [rnrs eval](07_06_r6rs_support.md#76228-rnrs-eval)
*   [rnrs mutable-pairs](07_06_r6rs_support.md#76229-rnrs-mutable-pairs)
*   [rnrs mutable-strings](07_06_r6rs_support.md#76230-rnrs-mutable-strings)
*   [rnrs r5rs](07_06_r6rs_support.md#76231-rnrs-r5rs)

* * *

Next: [rnrs base](07_06_r6rs_support.md#7622-rnrs-base), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.1 Library Usage [¶](07_06_r6rs_support.md#7621-library-usage)

Guile implements the R6RS ‘library’ form as a transformation to a native Guile module definition. As a consequence of this, all of the libraries described in the following subsections, in addition to being available for use by R6RS libraries and top-level programs, can also be imported as if they were normal Guile modules—via a `use-modules` form, say. For example, the R6RS “composite” library can be imported by:

  (import (rnrs (6)))

  ([use-modules](06_18_modules.md) ((rnrs) :version (6)))

For more information on Guile’s library implementation, see (see [R6RS Libraries](06_18_modules.md#6186-r6rs-libraries)).

* * *

Next: [rnrs unicode](07_06_r6rs_support.md#7623-rnrs-unicode), Previous: [Library Usage](07_06_r6rs_support.md#7621-library-usage), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.2 rnrs base [¶](07_06_r6rs_support.md#7622-rnrs-base)

The `(rnrs base (6))` library exports the procedures and syntactic forms described in the main section of the Report (see [R6RS Base library](https://doc.guix.gnu.org/guile/latest/en/r6rs.html#Base-library) in The Revised^6 Report on the Algorithmic Language Scheme). They are grouped below by the existing manual sections to which they correspond.

Scheme Procedure: **boolean?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **not** x [¶](07_06_r6rs_support.md)

See [Booleans](06_06_01_booleans.md#661-booleans), for documentation.

Scheme Procedure: **symbol?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **symbol->string** sym [¶](07_06_r6rs_support.md)

Scheme Procedure: **string->symbol** str [¶](07_06_r6rs_support.md)

See [Operations Related to Symbols](06_06_06_symbols.md#6664-operations-related-to-symbols), for documentation.

Scheme Procedure: **char?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **char=?** [¶](07_06_r6rs_support.md)

Scheme Procedure: **char<?** [¶](07_06_r6rs_support.md)

Scheme Procedure: **char>?** [¶](07_06_r6rs_support.md)

Scheme Procedure: **char<=?** [¶](07_06_r6rs_support.md)

Scheme Procedure: **char>=?** [¶](07_06_r6rs_support.md)

Scheme Procedure: **integer->char** n [¶](07_06_r6rs_support.md)

Scheme Procedure: **char->integer** chr [¶](07_06_r6rs_support.md)

See [Characters](06_06_03_characters.md#663-characters), for documentation.

Scheme Procedure: **list?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **null?** x [¶](07_06_r6rs_support.md)

See [List Predicates](06_06_09_lists.md#6692-list-predicates), for documentation.

Scheme Procedure: **pair?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **cons** x y [¶](07_06_r6rs_support.md)

Scheme Procedure: **car** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **caar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cadr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cddr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **caaar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **caadr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cadar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdaar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **caddr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdadr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cddar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdddr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **caaaar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **caaadr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **caadar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cadaar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdaaar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cddaar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdadar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdaadr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cadadr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **caaddr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **caddar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cadddr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdaddr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cddadr** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cdddar** pair [¶](07_06_r6rs_support.md)

Scheme Procedure: **cddddr** pair [¶](07_06_r6rs_support.md)

See [Pairs](06_06_08_pairs.md#668-pairs), for documentation.

Scheme Procedure: **number?** obj [¶](07_06_r6rs_support.md)

See [Scheme’s Numerical “Tower”](06_06_02_numerical_data_types.md#6621-schemes-numerical-tower), for documentation.

Scheme Procedure: **string?** obj [¶](07_06_r6rs_support.md)

See [String Predicates](06_06_05_strings.md#6652-string-predicates), for documentation.

Scheme Procedure: **procedure?** obj [¶](07_06_r6rs_support.md)

See [Procedure Properties and Meta-information](06_07_procedures.md#677-procedure-properties-and-meta-information), for documentation.

Scheme Syntax: **define** name value [¶](07_06_r6rs_support.md)

Scheme Syntax: **set!** variable-name value [¶](07_06_r6rs_support.md)

See [Defining and Setting Variables](03_hello_scheme.md#313-defining-and-setting-variables), for documentation.

Scheme Syntax: **define-syntax** keyword expression [¶](07_06_r6rs_support.md)

Scheme Syntax: **let-syntax** ((keyword transformer) …) exp1 exp2 … [¶](07_06_r6rs_support.md)

Scheme Syntax: **letrec-syntax** ((keyword transformer) …) exp1 exp2 … [¶](07_06_r6rs_support.md)

See [Defining Macros](06_08_macros.md#681-defining-macros), for documentation.

Scheme Syntax: **identifier-syntax** exp [¶](07_06_r6rs_support.md)

See [Identifier Macros](06_08_macros.md#686-identifier-macros), for documentation.

Scheme Syntax: **syntax-rules** literals (pattern template) ... [¶](07_06_r6rs_support.md)

See [Syntax-rules Macros](06_08_macros.md#682-syntax-rules-macros), for documentation.

Scheme Syntax: **lambda** formals body [¶](07_06_r6rs_support.md)

See [Lambda: Basic Procedure Creation](06_07_procedures.md#671-lambda-basic-procedure-creation), for documentation.

Scheme Syntax: **let** bindings body [¶](07_06_r6rs_support.md)

Scheme Syntax: **let\*** bindings body [¶](07_06_r6rs_support.md)

Scheme Syntax: **letrec** bindings body [¶](07_06_r6rs_support.md)

Scheme Syntax: **letrec\*** bindings body [¶](07_06_r6rs_support.md)

See [Local Variable Bindings](06_10_definitions_and_variable_bindings.md#6102-local-variable-bindings), for documentation.

Scheme Syntax: **let-values** bindings body [¶](07_06_r6rs_support.md)

Scheme Syntax: **let\*-values** bindings body [¶](07_06_r6rs_support.md)

See [SRFI-11 - let-values](07_05_10_srfi11_letvalues.md#7510-srfi-11---let-values), for documentation.

Scheme Syntax: **begin** expr1 expr2 ... [¶](07_06_r6rs_support.md)

See [Sequencing and Splicing](06_11_controlling_the_flow_of_program_execution.md#6111-sequencing-and-splicing), for documentation.

Scheme Syntax: **quote** expr [¶](07_06_r6rs_support.md)

Scheme Syntax: **quasiquote** expr [¶](07_06_r6rs_support.md)

Scheme Syntax: **unquote** expr [¶](07_06_r6rs_support.md)

Scheme Syntax: **unquote-splicing** expr [¶](07_06_r6rs_support.md)

See [Expression Syntax](06_16_reading_and_evaluating_scheme_code.md#61611-expression-syntax), for documentation.

Scheme Syntax: **if** test consequence \[alternate\] [¶](07_06_r6rs_support.md)

Scheme Syntax: **cond** clause1 clause2 ... [¶](07_06_r6rs_support.md)

Scheme Syntax: **case** key clause1 clause2 ... [¶](07_06_r6rs_support.md)

See [Simple Conditional Evaluation](06_11_controlling_the_flow_of_program_execution.md#6112-simple-conditional-evaluation), for documentation.

Scheme Syntax: **and** expr ... [¶](07_06_r6rs_support.md)

Scheme Syntax: **or** expr ... [¶](07_06_r6rs_support.md)

See [Conditional Evaluation of a Sequence of Expressions](06_11_controlling_the_flow_of_program_execution.md#6113-conditional-evaluation-of-a-sequence-of-expressions), for documentation.

Scheme Procedure: **eq?** x y [¶](07_06_r6rs_support.md)

Scheme Procedure: **eqv?** x y [¶](07_06_r6rs_support.md)

Scheme Procedure: **equal?** x y [¶](07_06_r6rs_support.md)

Scheme Procedure: **symbol=?** symbol1 symbol2 ... [¶](07_06_r6rs_support.md)

See [Equality](06_09_general_utility_functions.md#691-equality), for documentation.

`symbol=?` is identical to `eq?`.

Scheme Procedure: **complex?** z [¶](07_06_r6rs_support.md)

See [Complex Numbers](06_06_02_numerical_data_types.md#6624-complex-numbers), for documentation.

Scheme Procedure: **real-part** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **imag-part** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-rectangular** real\_part imaginary\_part [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-polar** x y [¶](07_06_r6rs_support.md)

Scheme Procedure: **magnitude** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **angle** z [¶](07_06_r6rs_support.md)

See [Complex Number Operations](06_06_02_numerical_data_types.md#66210-complex-number-operations), for documentation.

Scheme Procedure: **sqrt** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **exp** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **expt** z1 z2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **log** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **sin** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **cos** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **tan** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **asin** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **acos** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **atan** z [¶](07_06_r6rs_support.md)

See [Scientific Functions](06_06_02_numerical_data_types.md#66212-scientific-functions), for documentation.

Scheme Procedure: **real?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **rational?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **numerator** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **denominator** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **rationalize** x eps [¶](07_06_r6rs_support.md)

See [Real and Rational Numbers](06_06_02_numerical_data_types.md#6623-real-and-rational-numbers), for documentation.

Scheme Procedure: **exact?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **inexact?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **exact** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **inexact** z [¶](07_06_r6rs_support.md)

See [Exact and Inexact Numbers](06_06_02_numerical_data_types.md#6625-exact-and-inexact-numbers), for documentation. The `exact` and `inexact` procedures are identical to the `inexact->exact` and `exact->inexact` procedures provided by Guile’s code library.

Scheme Procedure: **integer?** x [¶](07_06_r6rs_support.md)

See [Integers](06_06_02_numerical_data_types.md#6622-integers), for documentation.

Scheme Procedure: **odd?** n [¶](07_06_r6rs_support.md)

Scheme Procedure: **even?** n [¶](07_06_r6rs_support.md)

Scheme Procedure: **gcd** x ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **lcm** x ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **exact-integer-sqrt** k [¶](07_06_r6rs_support.md)

See [Operations on Integer Values](06_06_02_numerical_data_types.md#6627-operations-on-integer-values), for documentation.

Scheme Procedure: **\=** [¶](07_06_r6rs_support.md)

Scheme Procedure: **<** [¶](07_06_r6rs_support.md)

Scheme Procedure: **\>** [¶](07_06_r6rs_support.md)

Scheme Procedure: **<=** [¶](07_06_r6rs_support.md)

Scheme Procedure: **\>=** [¶](07_06_r6rs_support.md)

Scheme Procedure: **zero?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **positive?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **negative?** x [¶](07_06_r6rs_support.md)

See [Comparison Predicates](06_06_02_numerical_data_types.md#6628-comparison-predicates), for documentation.

Scheme Procedure: **for-each** f lst1 lst2 ... [¶](07_06_r6rs_support.md)

See [Fold, Unfold & Map](07_05_03_srfi1_list_library.md#7535-fold-unfold--map), for documentation.

Scheme Procedure: **list** elem … [¶](07_06_r6rs_support.md)

See [List Constructors](06_06_09_lists.md#6693-list-constructors), for documentation.

Scheme Procedure: **length** lst [¶](07_06_r6rs_support.md)

Scheme Procedure: **list-ref** lst k [¶](07_06_r6rs_support.md)

Scheme Procedure: **list-tail** lst k [¶](07_06_r6rs_support.md)

See [List Selection](06_06_09_lists.md#6694-list-selection), for documentation.

Scheme Procedure: **append** lst … obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **append** [¶](07_06_r6rs_support.md)

Scheme Procedure: **reverse** lst [¶](07_06_r6rs_support.md)

See [Append and Reverse](06_06_09_lists.md#6695-append-and-reverse), for documentation.

Scheme Procedure: **number->string** n \[radix\] [¶](07_06_r6rs_support.md)

Scheme Procedure: **string->number** str \[radix\] [¶](07_06_r6rs_support.md)

See [Converting Numbers To and From Strings](06_06_02_numerical_data_types.md#6629-converting-numbers-to-and-from-strings), for documentation.

Scheme Procedure: **string** char ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-string** k \[chr\] [¶](07_06_r6rs_support.md)

Scheme Procedure: **list->string** lst [¶](07_06_r6rs_support.md)

See [String Constructors](06_06_05_strings.md#6653-string-constructors), for documentation.

Scheme Procedure: **string->list** str \[start \[end\]\] [¶](07_06_r6rs_support.md)

See [List/String conversion](06_06_05_strings.md#6654-liststring-conversion), for documentation.

Scheme Procedure: **string-length** str [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-ref** str k [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-copy** str \[start \[end\]\] [¶](07_06_r6rs_support.md)

Scheme Procedure: **substring** str start \[end\] [¶](07_06_r6rs_support.md)

See [String Selection](06_06_05_strings.md#6655-string-selection), for documentation.

Scheme Procedure: **string=?** s1 s2 s3 … [¶](07_06_r6rs_support.md)

Scheme Procedure: **string<?** s1 s2 s3 … [¶](07_06_r6rs_support.md)

Scheme Procedure: **string>?** s1 s2 s3 … [¶](07_06_r6rs_support.md)

Scheme Procedure: **string<=?** s1 s2 s3 … [¶](07_06_r6rs_support.md)

Scheme Procedure: **string>=?** s1 s2 s3 … [¶](07_06_r6rs_support.md)

See [String Comparison](06_06_05_strings.md#6657-string-comparison), for documentation.

Scheme Procedure: **string-append** arg … [¶](07_06_r6rs_support.md)

See [Reversing and Appending Strings](06_06_05_strings.md#66510-reversing-and-appending-strings), for documentation.

Scheme Procedure: **string-for-each** proc s \[start \[end\]\] [¶](07_06_r6rs_support.md)

See [Mapping, Folding, and Unfolding](06_06_05_strings.md#66511-mapping-folding-and-unfolding), for documentation.

Scheme Procedure: **+** z1 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **\-** z1 z2 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **\*** z1 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **/** z1 z2 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **max** x1 x2 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **min** x1 x2 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **abs** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **truncate** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **floor** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **ceiling** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **round** x [¶](07_06_r6rs_support.md)

See [Arithmetic Functions](06_06_02_numerical_data_types.md#66211-arithmetic-functions), for documentation.

Scheme Procedure: **div** x y [¶](07_06_r6rs_support.md)

Scheme Procedure: **mod** x y [¶](07_06_r6rs_support.md)

Scheme Procedure: **div-and-mod** x y [¶](07_06_r6rs_support.md)

These procedures accept two real numbers x and y, where the divisor y must be non-zero. `div` returns the integer q and `mod` returns the real number r such that _x = q\*y + r_ and _0 <= r < abs(y)_. `div-and-mod` returns both q and r, and is more efficient than computing each separately. Note that when _y > 0_, `div` returns _floor(x/y)_, otherwise it returns _ceiling(x/y)_.

([div](07_06_r6rs_support.md) 123 10) ⇒ 12
([mod](07_06_r6rs_support.md) 123 10) ⇒ 3
([div-and-mod](07_06_r6rs_support.md) 123 10) ⇒ 12 and 3
([div-and-mod](07_06_r6rs_support.md) 123 \-10) ⇒ \-12 and 3
([div-and-mod](07_06_r6rs_support.md) \-123 10) ⇒ \-13 and 7
([div-and-mod](07_06_r6rs_support.md) \-123 \-10) ⇒ 13 and 7
([div-and-mod](07_06_r6rs_support.md) \-123.2 \-63.5) ⇒ 2.0 and 3.8
([div-and-mod](07_06_r6rs_support.md) 16/3 \-10/7) ⇒ \-3 and 22/21

Scheme Procedure: **div0** x y [¶](07_06_r6rs_support.md)

Scheme Procedure: **mod0** x y [¶](07_06_r6rs_support.md)

Scheme Procedure: **div0-and-mod0** x y [¶](07_06_r6rs_support.md)

These procedures accept two real numbers x and y, where the divisor y must be non-zero. `div0` returns the integer q and `mod0` returns the real number r such that _x = q\*y + r_ and _\-abs(y/2) <= r < abs(y/2)_. `div0-and-mod0` returns both q and r, and is more efficient than computing each separately.

Note that `div0` returns _x/y_ rounded to the nearest integer. When _x/y_ lies exactly half-way between two integers, the tie is broken according to the sign of y. If _y > 0_, ties are rounded toward positive infinity, otherwise they are rounded toward negative infinity. This is a consequence of the requirement that _\-abs(y/2) <= r < abs(y/2)_.

([div0](07_06_r6rs_support.md) 123 10) ⇒ 12
([mod0](07_06_r6rs_support.md) 123 10) ⇒ 3
([div0-and-mod0](07_06_r6rs_support.md) 123 10) ⇒ 12 and 3
([div0-and-mod0](07_06_r6rs_support.md) 123 \-10) ⇒ \-12 and 3
([div0-and-mod0](07_06_r6rs_support.md) \-123 10) ⇒ \-12 and \-3
([div0-and-mod0](07_06_r6rs_support.md) \-123 \-10) ⇒ 12 and \-3
([div0-and-mod0](07_06_r6rs_support.md) \-123.2 \-63.5) ⇒ 2.0 and 3.8
([div0-and-mod0](07_06_r6rs_support.md) 16/3 \-10/7) ⇒ \-4 and \-8/21

Scheme Procedure: **real-valued?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **rational-valued?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **integer-valued?** obj [¶](07_06_r6rs_support.md)

These procedures return `#t` if and only if their arguments can, respectively, be coerced to a real, rational, or integer value without a loss of numerical precision.

`real-valued?` will return `#t` for complex numbers whose imaginary parts are zero.

Scheme Procedure: **nan?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **infinite?** x [¶](07_06_r6rs_support.md)

Scheme Procedure: **finite?** x [¶](07_06_r6rs_support.md)

`nan?` returns `#t` if x is a NaN value, `#f` otherwise. `infinite?` returns `#t` if x is an infinite value, `#f` otherwise. `finite?` returns `#t` if x is neither infinite nor a NaN value, otherwise it returns `#f`. Every real number satisfies exactly one of these predicates. An exception is raised if x is not real.

Scheme Syntax: **assert** expr [¶](07_06_r6rs_support.md)

Raises an `&assertion` condition if expr evaluates to `#f`; otherwise evaluates to the value of expr.

Scheme Procedure: **error** who message irritant1 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **assertion-violation** who message irritant1 ... [¶](07_06_r6rs_support.md)

These procedures raise compound conditions based on their arguments: If who is not `#f`, the condition will include a `&who` condition whose `who` field is set to who; a `&message` condition will be included with a `message` field equal to message; an `&irritants` condition will be included with its `irritants` list given by `irritant1 ...`.

`error` produces a compound condition with the simple conditions described above, as well as an `&error` condition; `assertion-violation` produces one that includes an `&assertion` condition.

Scheme Procedure: **vector-map** proc v [¶](07_06_r6rs_support.md)

Scheme Procedure: **vector-for-each** proc v [¶](07_06_r6rs_support.md)

These procedures implement the `map` and `for-each` contracts over vectors.

Scheme Procedure: **vector** arg … [¶](07_06_r6rs_support.md)

Scheme Procedure: **vector?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-vector** len [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-vector** len fill [¶](07_06_r6rs_support.md)

Scheme Procedure: **list->vector** l [¶](07_06_r6rs_support.md)

Scheme Procedure: **vector->list** v [¶](07_06_r6rs_support.md)

See [Dynamic Vector Creation and Validation](06_06_10_vectors.md#66102-dynamic-vector-creation-and-validation), for documentation.

Scheme Procedure: **vector-length** vector [¶](07_06_r6rs_support.md)

Scheme Procedure: **vector-ref** vector k [¶](07_06_r6rs_support.md)

Scheme Procedure: **vector-set!** vector k obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **vector-fill!** v fill [¶](07_06_r6rs_support.md)

See [Accessing and Modifying Vector Contents](06_06_10_vectors.md#66103-accessing-and-modifying-vector-contents), for documentation.

Scheme Procedure: **call-with-current-continuation** proc [¶](07_06_r6rs_support.md)

Scheme Procedure: **call/cc** proc [¶](07_06_r6rs_support.md)

See [Continuations](06_11_controlling_the_flow_of_program_execution.md#6116-continuations), for documentation.

Scheme Procedure: **values** arg … [¶](07_06_r6rs_support.md)

Scheme Procedure: **call-with-values** producer consumer [¶](07_06_r6rs_support.md)

See [Returning and Accepting Multiple Values](06_11_controlling_the_flow_of_program_execution.md#6117-returning-and-accepting-multiple-values), for documentation.

Scheme Procedure: **dynamic-wind** in\_guard thunk out\_guard [¶](07_06_r6rs_support.md)

See [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind), for documentation.

Scheme Procedure: **apply** proc arg … arglst [¶](07_06_r6rs_support.md)

See [Procedures for On the Fly Evaluation](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation), for documentation.

* * *

Next: [rnrs bytevectors](07_06_r6rs_support.md#7624-rnrs-bytevectors), Previous: [rnrs base](07_06_r6rs_support.md#7622-rnrs-base), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.3 rnrs unicode [¶](07_06_r6rs_support.md#7623-rnrs-unicode)

The `(rnrs unicode (6))` library provides procedures for manipulating Unicode characters and strings.

Scheme Procedure: **char-upcase** char [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-downcase** char [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-titlecase** char [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-foldcase** char [¶](07_06_r6rs_support.md)

These procedures translate their arguments from one Unicode character set to another. `char-upcase`, `char-downcase`, and `char-titlecase` are identical to their counterparts in the Guile core library; See [Characters](06_06_03_characters.md#663-characters), for documentation.

`char-foldcase` returns the result of applying `char-upcase` to its argument, followed by `char-downcase`—except in the case of the Turkic characters `U+0130` and `U+0131`, for which the procedure acts as the identity function.

Scheme Procedure: **char-ci=?** char1 char2 char3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-ci<?** char1 char2 char3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-ci>?** char1 char2 char3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-ci<=?** char1 char2 char3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-ci>=?** char1 char2 char3 ... [¶](07_06_r6rs_support.md)

These procedures facilitate case-insensitive comparison of Unicode characters. They are identical to the procedures provided by Guile’s core library. See [Characters](06_06_03_characters.md#663-characters), for documentation.

Scheme Procedure: **char-alphabetic?** char [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-numeric?** char [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-whitespace?** char [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-upper-case?** char [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-lower-case?** char [¶](07_06_r6rs_support.md)

Scheme Procedure: **char-title-case?** char [¶](07_06_r6rs_support.md)

These procedures implement various Unicode character set predicates. They are identical to the procedures provided by Guile’s core library. See [Characters](06_06_03_characters.md#663-characters), for documentation.

Scheme Procedure: **char-general-category** char [¶](07_06_r6rs_support.md)

See [Characters](06_06_03_characters.md#663-characters), for documentation.

Scheme Procedure: **string-upcase** string [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-downcase** string [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-titlecase** string [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-foldcase** string [¶](07_06_r6rs_support.md)

These procedures perform Unicode case folding operations on their input. See [Alphabetic Case Mapping](06_06_05_strings.md#6659-alphabetic-case-mapping), for documentation.

Scheme Procedure: **string-ci=?** string1 string2 string3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-ci<?** string1 string2 string3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-ci>?** string1 string2 string3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-ci<=?** string1 string2 string3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-ci>=?** string1 string2 string3 ... [¶](07_06_r6rs_support.md)

These procedures perform case-insensitive comparison on their input. See [String Comparison](06_06_05_strings.md#6657-string-comparison), for documentation.

Scheme Procedure: **string-normalize-nfd** string [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-normalize-nfkd** string [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-normalize-nfc** string [¶](07_06_r6rs_support.md)

Scheme Procedure: **string-normalize-nfkc** string [¶](07_06_r6rs_support.md)

These procedures perform Unicode string normalization operations on their input. See [String Comparison](06_06_05_strings.md#6657-string-comparison), for documentation.

* * *

Next: [rnrs lists](07_06_r6rs_support.md#7625-rnrs-lists), Previous: [rnrs unicode](07_06_r6rs_support.md#7623-rnrs-unicode), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.4 rnrs bytevectors [¶](07_06_r6rs_support.md#7624-rnrs-bytevectors)

The `(rnrs bytevectors (6))` library provides procedures for working with blocks of binary data. This functionality is documented in its own section of the manual; See [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors).

* * *

Next: [rnrs sorting](07_06_r6rs_support.md#7626-rnrs-sorting), Previous: [rnrs bytevectors](07_06_r6rs_support.md#7624-rnrs-bytevectors), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.5 rnrs lists [¶](07_06_r6rs_support.md#7625-rnrs-lists)

The `(rnrs lists (6))` library provides procedures additional procedures for working with lists.

Scheme Procedure: **find** proc list [¶](07_06_r6rs_support.md)

This procedure is identical to the one defined in Guile’s SRFI-1 implementation. See [Searching](07_05_03_srfi1_list_library.md#7537-searching), for documentation.

Scheme Procedure: **for-all** proc list1 list2 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **exists** proc list1 list2 ... [¶](07_06_r6rs_support.md)

The `for-all` procedure is identical to the `every` procedure defined by SRFI-1; the `exists` procedure is identical to SRFI-1’s `any`. See [Searching](07_05_03_srfi1_list_library.md#7537-searching), for documentation.

Scheme Procedure: **filter** proc list [¶](07_06_r6rs_support.md)

Scheme Procedure: **partition** proc list [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by SRFI-1. See [List Modification](06_06_09_lists.md#6696-list-modification), for a description of `filter`; See [Filtering and Partitioning](07_05_03_srfi1_list_library.md#7536-filtering-and-partitioning), for `partition`.

Scheme Procedure: **fold-right** combine nil list1 list2 … [¶](07_06_r6rs_support.md)

This procedure is identical the `fold-right` procedure provided by SRFI-1. See [Fold, Unfold & Map](07_05_03_srfi1_list_library.md#7535-fold-unfold--map), for documentation.

Scheme Procedure: **fold-left** combine nil list1 list2 … [¶](07_06_r6rs_support.md)

This procedure is like `fold` from SRFI-1, but combine is called with the seed as the first argument. See [Fold, Unfold & Map](07_05_03_srfi1_list_library.md#7535-fold-unfold--map), for documentation.

Scheme Procedure: **remp** proc list [¶](07_06_r6rs_support.md)

Scheme Procedure: **remove** obj list [¶](07_06_r6rs_support.md)

Scheme Procedure: **remv** obj list [¶](07_06_r6rs_support.md)

Scheme Procedure: **remq** obj list [¶](07_06_r6rs_support.md)

`remove`, `remv`, and `remq` are identical to the `delete`, `delv`, and `delq` procedures provided by Guile’s core library, (see [List Modification](06_06_09_lists.md#6696-list-modification)). `remp` is identical to the alternate `remove` procedure provided by SRFI-1; See [Deleting](07_05_03_srfi1_list_library.md#7538-deleting).

Scheme Procedure: **memp** proc list [¶](07_06_r6rs_support.md)

Scheme Procedure: **member** obj list [¶](07_06_r6rs_support.md)

Scheme Procedure: **memv** obj list [¶](07_06_r6rs_support.md)

Scheme Procedure: **memq** obj list [¶](07_06_r6rs_support.md)

`member`, `memv`, and `memq` are identical to the procedures provided by Guile’s core library; See [List Searching](06_06_09_lists.md#6697-list-searching), for their documentation. `memp` uses the specified predicate function `proc` to test elements of the list list—it behaves similarly to `find`, except that it returns the first sublist of list whose `car` satisfies proc.

Scheme Procedure: **assp** proc alist [¶](07_06_r6rs_support.md)

Scheme Procedure: **assoc** obj alist [¶](07_06_r6rs_support.md)

Scheme Procedure: **assv** obj alist [¶](07_06_r6rs_support.md)

Scheme Procedure: **assq** obj alist [¶](07_06_r6rs_support.md)

`assoc`, `assv`, and `assq` are identical to the procedures provided by Guile’s core library; See [Alist Key Equality](06_06_20_association_lists.md#66201-alist-key-equality), for their documentation. `assp` uses the specified predicate function `proc` to test keys in the association list alist.

Scheme Procedure: **cons\*** obj1 ... obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **cons\*** obj [¶](07_06_r6rs_support.md)

This procedure is identical to the one exported by Guile’s core library. See [List Constructors](06_06_09_lists.md#6693-list-constructors), for documentation.

* * *

Next: [rnrs control](07_06_r6rs_support.md#7627-rnrs-control), Previous: [rnrs lists](07_06_r6rs_support.md#7625-rnrs-lists), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.6 rnrs sorting [¶](07_06_r6rs_support.md#7626-rnrs-sorting)

The `(rnrs sorting (6))` library provides procedures for sorting lists and vectors.

Scheme Procedure: **list-sort** proc list [¶](07_06_r6rs_support.md)

Scheme Procedure: **vector-sort** proc vector [¶](07_06_r6rs_support.md)

These procedures return their input sorted in ascending order, without modifying the original data. proc must be a procedure that takes two elements from the input list or vector as arguments, and returns a true value if the first is “less” than the second, `#f` otherwise. `list-sort` returns a list; `vector-sort` returns a vector.

Both `list-sort` and `vector-sort` are implemented in terms of the `stable-sort` procedure from Guile’s core library. See [Sorting](06_09_general_utility_functions.md#693-sorting), for a discussion of the behavior of that procedure.

Scheme Procedure: **vector-sort!** proc vector [¶](07_06_r6rs_support.md)

Performs a destructive, “in-place” sort of vector, using proc as described above to determine an ascending ordering of elements. `vector-sort!` returns an unspecified value.

This procedure is implemented in terms of the `sort!` procedure from Guile’s core library. See [Sorting](06_09_general_utility_functions.md#693-sorting), for more information.

* * *

Next: [R6RS Records](07_06_r6rs_support.md#7628-r6rs-records), Previous: [rnrs sorting](07_06_r6rs_support.md#7626-rnrs-sorting), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.7 rnrs control [¶](07_06_r6rs_support.md#7627-rnrs-control)

The `(rnrs control (6))` library provides syntactic forms useful for constructing conditional expressions and controlling the flow of execution.

Scheme Syntax: **when** test expression1 expression2 ... [¶](07_06_r6rs_support.md)

Scheme Syntax: **unless** test expression1 expression2 ... [¶](07_06_r6rs_support.md)

The `when` form is evaluated by evaluating the specified test expression; if the result is a true value, the expressions that follow it are evaluated in order, and the value of the final expression becomes the value of the entire `when` expression.

The `unless` form behaves similarly, with the exception that the specified expressions are only evaluated if the value of test is false.

Scheme Syntax: **do** ((variable init step) ...) (test expression ...) command ... [¶](07_06_r6rs_support.md)

This form is identical to the one provided by Guile’s core library. See [Iteration mechanisms](06_11_controlling_the_flow_of_program_execution.md#6114-iteration-mechanisms), for documentation.

Scheme Syntax: **case-lambda** clause ... [¶](07_06_r6rs_support.md)

This form is identical to the one provided by Guile’s core library. See [Case-lambda](06_07_procedures.md#675-case-lambda), for documentation.

* * *

Next: [rnrs records syntactic](07_06_r6rs_support.md#7629-rnrs-records-syntactic), Previous: [rnrs control](07_06_r6rs_support.md#7627-rnrs-control), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.8 R6RS Records [¶](07_06_r6rs_support.md#7628-r6rs-records)

The manual sections below describe Guile’s implementation of R6RS records, which provide support for user-defined data types. The R6RS records API provides a superset of the features provided by Guile’s “native” records, as well as those of the SRFI-9 records API; See [Records](06_06_17_records.md#6617-records), and [SRFI-9 Records](06_06_16_srfi9_records.md#6616-srfi-9-records), for a description of those interfaces.

As with SRFI-9 and Guile’s native records, R6RS records are constructed using a record-type descriptor that specifies attributes like the record’s name, its fields, and the mutability of those fields.

R6RS records extend this framework to support single inheritance via the specification of a “parent” type for a record type at definition time. Accessors and mutator procedures for the fields of a parent type may be applied to records of a subtype of this parent. A record type may be _sealed_, in which case it cannot be used as the parent of another record type.

The inheritance mechanism for record types also informs the process of initializing the fields of a record and its parents. Constructor procedures that generate new instances of a record type are obtained from a record constructor descriptor, which encapsulates the record-type descriptor of the record to be constructed along with a _protocol_ procedure that defines how constructors for record subtypes delegate to the constructors of their parent types.

A protocol is a procedure used by the record system at construction time to bind arguments to the fields of the record being constructed. The protocol procedure is passed a procedure n that accepts the arguments required to construct the record’s parent type; this procedure, when invoked, will return a procedure p that accepts the arguments required to construct a new instance of the record type itself and returns a new instance of the record type.

The protocol should in turn return a procedure that uses n and p to initialize the fields of the record type and its parent type(s). This procedure will be the constructor returned by

As a trivial example, consider the hypothetical record type `pixel`, which encapsulates an x-y location on a screen, and `voxel`, which has `pixel` as its parent type and stores an additional coordinate. The following protocol produces a constructor procedure that accepts all three coordinates, uses the first two to initialize the fields of `pixel`, and binds the third to the single field of `voxel`.

  (lambda (n)
    (lambda (x y z)
      (let ((p (n x y)))
        (p z))))

It may be helpful to think of protocols as “constructor factories” that produce chains of delegating constructors glued together by the helper procedure n.

An R6RS record type may be declared to be _nongenerative_ via the use of a unique generated or user-supplied symbol—or _uid_—such that subsequent record type declarations with the same uid and attributes will return the previously-declared record-type descriptor.

R6RS record types may also be declared to be _opaque_, in which case the various predicates and introspection procedures defined in `(rnrs records introspection)` will behave as if records of this type are not records at all.

Note that while the R6RS records API shares much of its namespace with both the SRFI-9 and native Guile records APIs, it is not currently compatible with either.

* * *

Next: [rnrs records procedural](07_06_r6rs_support.md#76210-rnrs-records-procedural), Previous: [R6RS Records](07_06_r6rs_support.md#7628-r6rs-records), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.9 rnrs records syntactic [¶](07_06_r6rs_support.md#7629-rnrs-records-syntactic)

The `(rnrs records syntactic (6))` library exports the syntactic API for working with R6RS records.

Scheme Syntax: **define-record-type** name-spec record-clause … [¶](07_06_r6rs_support.md)

Defines a new record type, introducing bindings for a record-type descriptor, a record constructor descriptor, a constructor procedure, a record predicate, and accessor and mutator procedures for the new record type’s fields.

name-spec must either be an identifier or must take the form `(record-name constructor-name predicate-name)`, where record-name, constructor-name, and predicate-name are all identifiers and specify the names to which, respectively, the record-type descriptor, constructor, and predicate procedures will be bound. If name-spec is only an identifier, it specifies the name to which the generated record-type descriptor will be bound.

Each record-clause must be one of the following:

*   `(fields field-spec*)`, where each field-spec specifies a field of the new record type and takes one of the following forms:
    *   `(immutable field-name accessor-name)`, which specifies an immutable field with the name field-name and binds an accessor procedure for it to the name given by accessor-name
    *   `(mutable field-name accessor-name mutator-name)`, which specifies a mutable field with the name field-name and binds accessor and mutator procedures to accessor-name and mutator-name, respectively
    *   `(immutable field-name)`, which specifies an immutable field with the name field-name; an accessor procedure for it will be created and named by appending record name and field-name with a hyphen separator
    *   `(mutable field-name`), which specifies a mutable field with the name field-name; an accessor procedure for it will be created and named as described above; a mutator procedure will also be created and named by appending `-set!` to the accessor name
    *   `field-name`, which specifies an immutable field with the name field-name; an access procedure for it will be created and named as described above
*   `(parent parent-name)`, where parent-name is a symbol giving the name of the record type to be used as the parent of the new record type
*   `(protocol expression)`, where expression evaluates to a protocol procedure which behaves as described above, and is used to create a record constructor descriptor for the new record type
*   `(sealed sealed?)`, where sealed? is a boolean value that specifies whether or not the new record type is sealed
*   `(opaque opaque?)`, where opaque? is a boolean value that specifies whether or not the new record type is opaque
*   `(nongenerative [uid])`, which specifies that the record type is nongenerative via the optional uid uid. If uid is not specified, a unique uid will be generated at expansion time
*   `(parent-rtd parent-rtd parent-cd)`, a more explicit form of the `parent` form above; parent-rtd and parent-cd should evaluate to a record-type descriptor and a record constructor descriptor, respectively

Scheme Syntax: **record-type-descriptor** record-name [¶](07_06_r6rs_support.md)

Evaluates to the record-type descriptor associated with the type specified by record-name.

Scheme Syntax: **record-constructor-descriptor** record-name [¶](07_06_r6rs_support.md)

Evaluates to the record-constructor descriptor associated with the type specified by record-name.

* * *

Next: [rnrs records inspection](07_06_r6rs_support.md#76211-rnrs-records-inspection), Previous: [rnrs records syntactic](07_06_r6rs_support.md#7629-rnrs-records-syntactic), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.10 rnrs records procedural [¶](07_06_r6rs_support.md#76210-rnrs-records-procedural)

The `(rnrs records procedural (6))` library exports the procedural API for working with R6RS records.

Scheme Procedure: **make-record-type-descriptor** name parent uid sealed? opaque? fields [¶](07_06_r6rs_support.md)

Returns a new record-type descriptor with the specified characteristics: name must be a symbol giving the name of the new record type; parent must be either `#f` or a non-sealed record-type descriptor for the returned record type to extend; uid must be either `#f`, indicating that the record type is generative, or a symbol giving the type’s nongenerative uid; sealed? and opaque? must be boolean values that specify the sealedness and opaqueness of the record type; fields must be a vector of zero or more field specifiers of the form `(mutable name)` or `(immutable name)`, where name is a symbol giving a name for the field.

If uid is not `#f`, it must be a symbol

Scheme Procedure: **record-type-descriptor?** obj [¶](07_06_r6rs_support.md)

Returns `#t` if obj is a record-type descriptor, `#f` otherwise.

Scheme Procedure: **make-record-constructor-descriptor** rtd parent-constructor-descriptor protocol [¶](07_06_r6rs_support.md)

Returns a new record constructor descriptor that can be used to produce constructors for the record type specified by the record-type descriptor rtd and whose delegation and binding behavior are specified by the protocol procedure protocol.

parent-constructor-descriptor specifies a record constructor descriptor for the parent type of rtd, if one exists. If rtd represents a base type, then parent-constructor-descriptor must be `#f`. If rtd is an extension of another type, parent-constructor-descriptor may still be `#f`, but protocol must also be `#f` in this case.

Scheme Procedure: **record-constructor** rcd [¶](07_06_r6rs_support.md)

Returns a record constructor procedure by invoking the protocol defined by the record-constructor descriptor rcd.

Scheme Procedure: **record-predicate** rtd [¶](07_06_r6rs_support.md)

Returns the record predicate procedure for the record-type descriptor rtd.

Scheme Procedure: **record-accessor** rtd k [¶](07_06_r6rs_support.md)

Returns the record field accessor procedure for the kth field of the record-type descriptor rtd.

Scheme Procedure: **record-mutator** rtd k [¶](07_06_r6rs_support.md)

Returns the record field mutator procedure for the kth field of the record-type descriptor rtd. An `&assertion` condition will be raised if this field is not mutable.

* * *

Next: [rnrs exceptions](07_06_r6rs_support.md#76212-rnrs-exceptions), Previous: [rnrs records procedural](07_06_r6rs_support.md#76210-rnrs-records-procedural), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.11 rnrs records inspection [¶](07_06_r6rs_support.md#76211-rnrs-records-inspection)

The `(rnrs records inspection (6))` library provides procedures useful for accessing metadata about R6RS records.

Scheme Procedure: **record?** obj [¶](07_06_r6rs_support.md)

Return `#t` if the specified object is a non-opaque R6RS record, `#f` otherwise.

Scheme Procedure: **record-rtd** record [¶](07_06_r6rs_support.md)

Returns the record-type descriptor for record. An `&assertion` is raised if record is opaque.

Scheme Procedure: **record-type-name** rtd [¶](07_06_r6rs_support.md)

Returns the name of the record-type descriptor rtd.

Scheme Procedure: **record-type-parent** rtd [¶](07_06_r6rs_support.md)

Returns the parent of the record-type descriptor rtd, or `#f` if it has none.

Scheme Procedure: **record-type-uid** rtd [¶](07_06_r6rs_support.md)

Returns the uid of the record-type descriptor rtd, or `#f` if it has none.

Scheme Procedure: **record-type-generative?** rtd [¶](07_06_r6rs_support.md)

Returns `#t` if the record-type descriptor rtd is generative, `#f` otherwise.

Scheme Procedure: **record-type-sealed?** rtd [¶](07_06_r6rs_support.md)

Returns `#t` if the record-type descriptor rtd is sealed, `#f` otherwise.

Scheme Procedure: **record-type-opaque?** rtd [¶](07_06_r6rs_support.md)

Returns `#t` if the record-type descriptor rtd is opaque, `#f` otherwise.

Scheme Procedure: **record-type-field-names** rtd [¶](07_06_r6rs_support.md)

Returns a vector of symbols giving the names of the fields defined by the record-type descriptor rtd (and not any of its sub- or supertypes).

Scheme Procedure: **record-field-mutable?** rtd k [¶](07_06_r6rs_support.md)

Returns `#t` if the field at index k of the record-type descriptor rtd (and not any of its sub- or supertypes) is mutable.

* * *

Next: [rnrs conditions](07_06_r6rs_support.md#76213-rnrs-conditions), Previous: [rnrs records inspection](07_06_r6rs_support.md#76211-rnrs-records-inspection), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.12 rnrs exceptions [¶](07_06_r6rs_support.md#76212-rnrs-exceptions)

The `(rnrs exceptions (6))` library provides functionality related to signaling and handling exceptional situations. This functionality re-exports Guile’s core exception-handling primitives. See [Exceptions](06_11_controlling_the_flow_of_program_execution.md#6118-exceptions), for a full discussion. See [SRFI-34 - Exception handling for programs](07_05_23_srfi34_exception_handling_for_programs.md#7523-srfi-34---exception-handling-for-programs), for a similar pre-R6RS facility. In Guile, SRFI-34, SRFI-35, and R6RS exception handling are all built on the same core facilities, and so are interoperable.

Scheme Procedure: **with-exception-handler** handler thunk [¶](07_06_r6rs_support.md)

See [Raising and Handling Exceptions](06_11_controlling_the_flow_of_program_execution.md#61182-raising-and-handling-exceptions), for more information on `with-exception-handler`.

Scheme Syntax: **guard** (variable clause1 clause2 ...) body [¶](07_06_r6rs_support.md)

Evaluates the expression given by body, first creating an ad hoc exception handler that binds a raised exception to variable and then evaluates the specified clauses as if they were part of a `cond` expression, with the value of the first matching clause becoming the value of the `guard` expression (see [Simple Conditional Evaluation](06_11_controlling_the_flow_of_program_execution.md#6112-simple-conditional-evaluation)). If none of the clause’s test expressions evaluates to `#t`, the exception is re-raised, with the exception handler that was current before the evaluation of the `guard` form.

For example, the expression

([guard](07_05_23_srfi34_exception_handling_for_programs.md) (ex (([eq?](06_09_general_utility_functions.md) ex 'foo) 'bar) (([eq?](06_09_general_utility_functions.md) ex 'bar) 'baz)) 
  ([raise](07_02_08_signals.md) 'bar))

evaluates to `baz`.

Scheme Procedure: **raise** obj [¶](07_06_r6rs_support.md)

Equivalent to core Guile `(raise-exception obj)`. See [Raising and Handling Exceptions](06_11_controlling_the_flow_of_program_execution.md#61182-raising-and-handling-exceptions). (Unfortunately, `raise` is already bound to a different function in core Guile. See [Signals](07_02_08_signals.md#728-signals).)

Scheme Procedure: **raise-continuable** obj [¶](07_06_r6rs_support.md)

Equivalent to core Guile `(raise-exception obj #:continuable? #t)`. See [Raising and Handling Exceptions](06_11_controlling_the_flow_of_program_execution.md#61182-raising-and-handling-exceptions).

* * *

Next: [I/O Conditions](07_06_r6rs_support.md#76214-io-conditions), Previous: [rnrs exceptions](07_06_r6rs_support.md#76212-rnrs-exceptions), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.13 rnrs conditions [¶](07_06_r6rs_support.md#76213-rnrs-conditions)

The `(rnrs condition (6))` library provides forms and procedures for constructing new condition types, as well as a library of pre-defined condition types that represent a variety of common exceptional situations. Conditions are records of a subtype of the `&condition` record type, which is neither sealed nor opaque. See [R6RS Records](07_06_r6rs_support.md#7628-r6rs-records).

Conditions may be manipulated singly, as _simple conditions_, or when composed with other conditions to form _compound conditions_. Compound conditions do not “nest”—constructing a new compound condition out of existing compound conditions will “flatten” them into their component simple conditions. For example, making a new condition out of a `&message` condition and a compound condition that contains an `&assertion` condition and another `&message` condition will produce a compound condition that contains two `&message` conditions and one `&assertion` condition.

The record type predicates and field accessors described below can operate on either simple or compound conditions. In the latter case, the predicate returns `#t` if the compound condition contains a component simple condition of the appropriate type; the field accessors return the requisite fields from the first component simple condition found to be of the appropriate type.

Guile’s R6RS layer uses core exception types from the `(ice-9 exceptions)` module as the basis for its R6RS condition system. Guile prefers to use the term “exception object” and “exception type” rather than “condition” or “condition type”, but that’s just a naming difference. Guile also has different names for the types in the condition hierarchy. See [Exception Objects](06_11_controlling_the_flow_of_program_execution.md#61181-exception-objects), for full details.

This library is quite similar to the SRFI-35 conditions module (see [SRFI-35 - Conditions](07_05_24_srfi35_conditions.md#7524-srfi-35---conditions)). Among other minor differences, the `(rnrs conditions)` library features slightly different semantics around condition field accessors, and comes with a larger number of pre-defined condition types. The two APIs are compatible; the `condition?` predicate from one API will return `#t` when applied to a condition object created in the other. of the condition types are the same, also.

Condition Type: **&condition** [¶](07_06_r6rs_support.md)

Scheme Procedure: **condition?** obj [¶](07_06_r6rs_support.md)

The base record type for conditions. Known as `&exception` in core Guile.

Scheme Procedure: **condition** condition1 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **simple-conditions** condition [¶](07_06_r6rs_support.md)

The `condition` procedure creates a new compound condition out of its condition arguments, flattening any specified compound conditions into their component simple conditions as described above.

`simple-conditions` returns a list of the component simple conditions of the compound condition `condition`, in the order in which they were specified at construction time.

Scheme Procedure: **condition-predicate** rtd [¶](07_06_r6rs_support.md)

Scheme Procedure: **condition-accessor** rtd proc [¶](07_06_r6rs_support.md)

These procedures return condition predicate and accessor procedures for the specified condition record type rtd.

Scheme Syntax: **define-condition-type** condition-type supertype constructor predicate field-spec ... [¶](07_06_r6rs_support.md)

Evaluates to a new record type definition for a condition type with the name condition-type that has the condition type supertype as its parent. A default constructor, which binds its arguments to the fields of this type and its parent types, will be bound to the identifier constructor; a condition predicate will be bound to predicate. The fields of the new type, which are immutable, are specified by the field-specs, each of which must be of the form:

(field accessor)

where field gives the name of the field and accessor gives the name for a binding to an accessor procedure created for this field.

Condition Type: **&message** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-message-condition** message [¶](07_06_r6rs_support.md)

Scheme Procedure: **message-condition?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **condition-message** condition [¶](07_06_r6rs_support.md)

A type that includes a message describing the condition that occurred.

Condition Type: **&warning** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-warning** [¶](07_06_r6rs_support.md)

Scheme Procedure: **warning?** obj [¶](07_06_r6rs_support.md)

A base type for representing non-fatal conditions during execution.

Condition Type: **&serious** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-serious-condition** [¶](07_06_r6rs_support.md)

Scheme Procedure: **serious-condition?** obj [¶](07_06_r6rs_support.md)

A base type for conditions representing errors serious enough that cannot be ignored. Known as `&error` in core Guile.

Condition Type: **&error** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-error** [¶](07_06_r6rs_support.md)

Scheme Procedure: **error?** obj [¶](07_06_r6rs_support.md)

A base type for conditions representing errors. Known as `&external-error` in core Guile.

Condition Type: **&violation** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-violation** [¶](07_06_r6rs_support.md)

Scheme Procedure: **violation?** [¶](07_06_r6rs_support.md)

A subtype of `&serious` that can be used to represent violations of a language or library standard. Known as `&programming-error` in core Guile.

Condition Type: **&assertion** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-assertion-violation** [¶](07_06_r6rs_support.md)

Scheme Procedure: **assertion-violation?** obj [¶](07_06_r6rs_support.md)

A subtype of `&violation` that indicates an invalid call to a procedure. Known as `&assertion-failure` in core Guile.

Condition Type: **&irritants** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-irritants-condition** irritants [¶](07_06_r6rs_support.md)

Scheme Procedure: **irritants-condition?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **condition-irritants** condition [¶](07_06_r6rs_support.md)

A base type used for storing information about the causes of another condition in a compound condition.

Condition Type: **&who** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-who-condition** who [¶](07_06_r6rs_support.md)

Scheme Procedure: **who-condition?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **condition-who** condition [¶](07_06_r6rs_support.md)

A base type used for storing the identity, a string or symbol, of the entity responsible for another condition in a compound condition.

Condition Type: **&non-continuable** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-non-continuable-violation** [¶](07_06_r6rs_support.md)

Scheme Procedure: **non-continuable-violation?** obj [¶](07_06_r6rs_support.md)

A subtype of `&violation` used to indicate that an exception handler invoked by `raise` has returned locally.

Condition Type: **&implementation-restriction** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-implementation-restriction-violation** [¶](07_06_r6rs_support.md)

Scheme Procedure: **implementation-restriction-violation?** obj [¶](07_06_r6rs_support.md)

A subtype of `&violation` used to indicate a violation of an implementation restriction.

Condition Type: **&lexical** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-lexical-violation** [¶](07_06_r6rs_support.md)

Scheme Procedure: **lexical-violation?** obj [¶](07_06_r6rs_support.md)

A subtype of `&violation` used to indicate a syntax violation at the level of the datum syntax.

Condition Type: **&syntax** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-syntax-violation** form subform [¶](07_06_r6rs_support.md)

Scheme Procedure: **syntax-violation?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **syntax-violation-form** condition [¶](07_06_r6rs_support.md)

Scheme Procedure: **syntax-violation-subform** condition [¶](07_06_r6rs_support.md)

A subtype of `&violation` that indicates a syntax violation. The form and subform fields, which must be datum values, indicate the syntactic form responsible for the condition.

Condition Type: **&undefined** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-undefined-violation** [¶](07_06_r6rs_support.md)

Scheme Procedure: **undefined-violation?** obj [¶](07_06_r6rs_support.md)

A subtype of `&violation` that indicates a reference to an unbound identifier. Known as `&undefined-variable` in core Guile.

* * *

Next: [Transcoders](07_06_r6rs_support.md#76215-transcoders), Previous: [rnrs conditions](07_06_r6rs_support.md#76213-rnrs-conditions), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.14 I/O Conditions [¶](07_06_r6rs_support.md#76214-io-conditions)

These condition types are exported by both the `(rnrs io ports (6))` and `(rnrs io simple (6))` libraries.

Condition Type: **&i/o** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-error** [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-error?** obj [¶](07_06_r6rs_support.md)

A condition supertype for more specific I/O errors.

Condition Type: **&i/o-read** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-read-error** [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-read-error?** obj [¶](07_06_r6rs_support.md)

A subtype of `&i/o`; represents read-related I/O errors.

Condition Type: **&i/o-write** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-write-error** [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-write-error?** obj [¶](07_06_r6rs_support.md)

A subtype of `&i/o`; represents write-related I/O errors.

Condition Type: **&i/o-invalid-position** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-invalid-position-error** position [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-invalid-position-error?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-error-position** condition [¶](07_06_r6rs_support.md)

A subtype of `&i/o`; represents an error related to an attempt to set the file position to an invalid position.

Condition Type: **&i/o-filename** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-io-filename-error** filename [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-filename-error?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-error-filename** condition [¶](07_06_r6rs_support.md)

A subtype of `&i/o`; represents an error related to an operation on a named file.

Condition Type: **&i/o-file-protection** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-file-protection-error** filename [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-file-protection-error?** obj [¶](07_06_r6rs_support.md)

A subtype of `&i/o-filename`; represents an error resulting from an attempt to access a named file for which the caller had insufficient permissions.

Condition Type: **&i/o-file-is-read-only** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-file-is-read-only-error** filename [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-file-is-read-only-error?** obj [¶](07_06_r6rs_support.md)

A subtype of `&i/o-file-protection`; represents an error related to an attempt to write to a read-only file.

Condition Type: **&i/o-file-already-exists** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-file-already-exists-error** filename [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-file-already-exists-error?** obj [¶](07_06_r6rs_support.md)

A subtype of `&i/o-filename`; represents an error related to an operation on an existing file that was assumed not to exist.

Condition Type: **&i/o-file-does-not-exist** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-file-does-not-exist-error** [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-file-does-not-exist-error?** obj [¶](07_06_r6rs_support.md)

A subtype of `&i/o-filename`; represents an error related to an operation on a non-existent file that was assumed to exist.

Condition Type: **&i/o-port** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-port-error** port [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-port-error?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-error-port** condition [¶](07_06_r6rs_support.md)

A subtype of `&i/o`; represents an error related to an operation on the port port.

* * *

Next: [rnrs io ports](07_06_r6rs_support.md#76216-rnrs-io-ports), Previous: [I/O Conditions](07_06_r6rs_support.md#76214-io-conditions), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.15 Transcoders [¶](07_06_r6rs_support.md#76215-transcoders)

The transcoder facilities are exported by `(rnrs io ports)`.

Several different Unicode encoding schemes describe standard ways to encode characters and strings as byte sequences and to decode those sequences. Within this document, a _codec_ is an immutable Scheme object that represents a Unicode or similar encoding scheme.

An _end-of-line style_ is a symbol that, if it is not `none`, describes how a textual port transcodes representations of line endings.

A _transcoder_ is an immutable Scheme object that combines a codec with an end-of-line style and a method for handling decoding errors. Each transcoder represents some specific bidirectional (but not necessarily lossless), possibly stateful translation between byte sequences and Unicode characters and strings. Every transcoder can operate in the input direction (bytes to characters) or in the output direction (characters to bytes). A transcoder parameter name means that the corresponding argument must be a transcoder.

A _binary port_ is a port that supports binary I/O, does not have an associated transcoder and does not support textual I/O. A _textual port_ is a port that supports textual I/O, and does not support binary I/O. A textual port may or may not have an associated transcoder.

Scheme Procedure: **latin-1-codec** [¶](07_06_r6rs_support.md)

Scheme Procedure: **utf-8-codec** [¶](07_06_r6rs_support.md)

Scheme Procedure: **utf-16-codec** [¶](07_06_r6rs_support.md)

These are predefined codecs for the ISO 8859-1, UTF-8, and UTF-16 encoding schemes.

A call to any of these procedures returns a value that is equal in the sense of `eqv?` to the result of any other call to the same procedure.

Scheme Syntax: **eol-style** eol-style-symbol [¶](07_06_r6rs_support.md)

eol-style-symbol should be a symbol whose name is one of `lf`, `cr`, `crlf`, `nel`, `crnel`, `ls`, and `none`.

The form evaluates to the corresponding symbol. If the name of eol-style-symbol is not one of these symbols, the effect and result are implementation-dependent; in particular, the result may be an eol-style symbol acceptable as an eol-style argument to `make-transcoder`. Otherwise, an exception is raised.

All eol-style symbols except `none` describe a specific line-ending encoding:

`lf`

linefeed

`cr`

carriage return

`crlf`

carriage return, linefeed

`nel`

next line

`crnel`

carriage return, next line

`ls`

line separator

For a textual port with a transcoder, and whose transcoder has an eol-style symbol `none`, no conversion occurs. For a textual input port, any eol-style symbol other than `none` means that all of the above line-ending encodings are recognized and are translated into a single linefeed. For a textual output port, `none` and `lf` are equivalent. Linefeed characters are encoded according to the specified eol-style symbol, and all other characters that participate in possible line endings are encoded as is.

> **Note:** Only the name of eol-style-symbol is significant.

Scheme Procedure: **native-eol-style** [¶](07_06_r6rs_support.md)

Returns the default end-of-line style of the underlying platform, e.g., `lf` on Unix and `crlf` on Windows.

Condition Type: **&i/o-decoding** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-decoding-error** port [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-decoding-error?** obj [¶](07_06_r6rs_support.md)

This condition type could be defined by

(define-condition-type [&i/o-decoding](07_06_r6rs_support.md) [&i/o-port](07_06_r6rs_support.md)
  [make-i/o-decoding-error](07_06_r6rs_support.md) [i/o-decoding-error?](07_06_r6rs_support.md))

An exception with this type is raised when one of the operations for textual input from a port encounters a sequence of bytes that cannot be translated into a character or string by the input direction of the port’s transcoder.

When such an exception is raised, the port’s position is past the invalid encoding.

Condition Type: **&i/o-encoding** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-i/o-encoding-error** port char [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-encoding-error?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **i/o-encoding-error-char** condition [¶](07_06_r6rs_support.md)

This condition type could be defined by

(define-condition-type [&i/o-encoding](07_06_r6rs_support.md) [&i/o-port](07_06_r6rs_support.md)
  [make-i/o-encoding-error](07_06_r6rs_support.md) [i/o-encoding-error?](07_06_r6rs_support.md)
  (char [i/o-encoding-error-char](07_06_r6rs_support.md)))

An exception with this type is raised when one of the operations for textual output to a port encounters a character that cannot be translated into bytes by the output direction of the port’s transcoder. char is the character that could not be encoded.

Scheme Syntax: **error-handling-mode** error-handling-mode-symbol [¶](07_06_r6rs_support.md)

error-handling-mode-symbol should be a symbol whose name is one of `ignore`, `raise`, and `replace`. The form evaluates to the corresponding symbol. If error-handling-mode-symbol is not one of these identifiers, effect and result are implementation-dependent: The result may be an error-handling-mode symbol acceptable as a handling-mode argument to `make-transcoder`. If it is not acceptable as a handling-mode argument to `make-transcoder`, an exception is raised.

> **Note:** Only the name of error-handling-mode-symbol is significant.

The error-handling mode of a transcoder specifies the behavior of textual I/O operations in the presence of encoding or decoding errors.

If a textual input operation encounters an invalid or incomplete character encoding, and the error-handling mode is `ignore`, an appropriate number of bytes of the invalid encoding are ignored and decoding continues with the following bytes.

If the error-handling mode is `replace`, the replacement character U+FFFD is injected into the data stream, an appropriate number of bytes are ignored, and decoding continues with the following bytes.

If the error-handling mode is `raise`, an exception with condition type `&i/o-decoding` is raised.

If a textual output operation encounters a character it cannot encode, and the error-handling mode is `ignore`, the character is ignored and encoding continues with the next character. If the error-handling mode is `replace`, a codec-specific replacement character is emitted by the transcoder, and encoding continues with the next character. The replacement character is U+FFFD for transcoders whose codec is one of the Unicode encodings, but is the `?` character for the Latin-1 encoding. If the error-handling mode is `raise`, an exception with condition type `&i/o-encoding` is raised.

Scheme Procedure: **make-transcoder** codec [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-transcoder** codec eol-style [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-transcoder** codec eol-style handling-mode [¶](07_06_r6rs_support.md)

codec must be a codec; eol-style, if present, an eol-style symbol; and handling-mode, if present, an error-handling-mode symbol.

eol-style may be omitted, in which case it defaults to the native end-of-line style of the underlying platform. handling-mode may be omitted, in which case it defaults to `replace`. The result is a transcoder with the behavior specified by its arguments.

Scheme procedure: **native-transcoder** [¶](07_06_r6rs_support.md)

Returns an implementation-dependent transcoder that represents a possibly locale-dependent “native” transcoding.

Scheme Procedure: **transcoder-codec** transcoder [¶](07_06_r6rs_support.md)

Scheme Procedure: **transcoder-eol-style** transcoder [¶](07_06_r6rs_support.md)

Scheme Procedure: **transcoder-error-handling-mode** transcoder [¶](07_06_r6rs_support.md)

These are accessors for transcoder objects; when applied to a transcoder returned by `make-transcoder`, they return the codec, eol-style, and handling-mode arguments, respectively.

Scheme Procedure: **bytevector->string** bytevector transcoder [¶](07_06_r6rs_support.md)

Returns the string that results from transcoding the bytevector according to the input direction of the transcoder.

Scheme Procedure: **string->bytevector** string transcoder [¶](07_06_r6rs_support.md)

Returns the bytevector that results from transcoding the string according to the output direction of the transcoder.

* * *

Next: [R6RS File Ports](07_06_r6rs_support.md#76217-r6rs-file-ports), Previous: [Transcoders](07_06_r6rs_support.md#76215-transcoders), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.16 rnrs io ports [¶](07_06_r6rs_support.md#76216-rnrs-io-ports)

Guile’s binary and textual port interface was heavily inspired by R6RS, so many R6RS port interfaces are documented elsewhere. Note that R6RS ports are not disjoint from Guile’s native ports, so Guile-specific procedures will work on ports created using the R6RS API, and vice versa. Also note that in Guile, all ports are both textual and binary. See [Input and Output](06_12_input_and_output.md#612-input-and-output), for more on Guile’s core port API. The R6RS ports module wraps Guile’s I/O routines in a helper that will translate native Guile exceptions to R6RS conditions; See [I/O Conditions](07_06_r6rs_support.md#76214-io-conditions), for more. See [R6RS File Ports](07_06_r6rs_support.md#76217-r6rs-file-ports), for documentation on the R6RS file port interface.

_Note_: The implementation of this R6RS API is not complete yet.

Scheme Procedure: **eof-object?** obj [¶](07_06_r6rs_support.md)

See [Binary I/O](06_12_input_and_output.md#6122-binary-io), for documentation.

Scheme Procedure: **eof-object** [¶](07_06_r6rs_support.md)

Return the end-of-file (EOF) object.

([eof-object?](06_12_input_and_output.md) ([eof-object](07_06_r6rs_support.md)))
⇒ #t

Scheme Procedure: **port?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **input-port?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **output-port?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **call-with-port** port proc [¶](07_06_r6rs_support.md)

See [Ports](06_12_input_and_output.md#6121-ports), for documentation.

Scheme Procedure: **port-transcoder** port [¶](07_06_r6rs_support.md)

Return a transcoder associated with the encoding of port. See [Encoding](06_12_input_and_output.md#6123-encoding), and See [Transcoders](07_06_r6rs_support.md#76215-transcoders).

Scheme Procedure: **binary-port?** port [¶](07_06_r6rs_support.md)

Return `#t` if port appears to be a binary port, else return `#f`. Note that Guile does not currently distinguish between binary and textual ports, so this predicate is not a reliable indicator of whether the port was created as a binary port. Currently, it returns `#t` if and only if the port encoding is “ISO-8859-1”, because Guile uses this encoding when creating a binary port. See [Encoding](06_12_input_and_output.md#6123-encoding), for more details.

Scheme Procedure: **textual-port?** port [¶](07_06_r6rs_support.md)

Return `#t` if port appears to be a textual port, else return `#f`. Note that Guile does not currently distinguish between binary and textual ports, so this predicate is not a reliable indicator of whether the port was created as a textual port. Currently, it always returns `#t`, because all ports can be used for textual I/O in Guile. See [Encoding](06_12_input_and_output.md#6123-encoding), for more details.

Scheme Procedure: **transcoded-port** binary-port transcoder [¶](07_06_r6rs_support.md)

The `transcoded-port` procedure returns a new textual port with the specified transcoder. Otherwise the new textual port’s state is largely the same as that of binary-port. If binary-port is an input port, the new textual port will be an input port and will transcode the bytes that have not yet been read from binary-port. If binary-port is an output port, the new textual port will be an output port and will transcode output characters into bytes that are written to the byte sink represented by binary-port.

As a side effect, however, `transcoded-port` closes binary-port in a special way that allows the new textual port to continue to use the byte source or sink represented by binary-port, even though binary-port itself is closed and cannot be used by the input and output operations described in this chapter.

Scheme Procedure: **port-position** port [¶](07_06_r6rs_support.md)

Equivalent to `(seek port 0 SEEK_CUR)`. See [Random Access](06_12_input_and_output.md#6127-random-access).

Scheme Procedure: **port-has-port-position?** port [¶](07_06_r6rs_support.md)

Return `#t` is port supports `port-position`.

Scheme Procedure: **set-port-position!** port offset [¶](07_06_r6rs_support.md)

Equivalent to `(seek port offset SEEK_SET)`. See [Random Access](06_12_input_and_output.md#6127-random-access).

Scheme Procedure: **port-has-set-port-position!?** port [¶](07_06_r6rs_support.md)

Return `#t` is port supports `set-port-position!`.

Scheme Procedure: **port-eof?** input-port [¶](07_06_r6rs_support.md)

Equivalent to `(eof-object? (lookahead-u8 input-port))`.

Scheme Procedure: **standard-input-port** [¶](07_06_r6rs_support.md)

Scheme Procedure: **standard-output-port** [¶](07_06_r6rs_support.md)

Scheme Procedure: **standard-error-port** [¶](07_06_r6rs_support.md)

Returns a fresh binary input port connected to standard input, or a binary output port connected to the standard output or standard error, respectively. Whether the port supports the `port-position` and `set-port-position!` operations is implementation-dependent.

Scheme Procedure: **current-input-port** [¶](07_06_r6rs_support.md)

Scheme Procedure: **current-output-port** [¶](07_06_r6rs_support.md)

Scheme Procedure: **current-error-port** [¶](07_06_r6rs_support.md)

See [Default Ports for Input, Output and Errors](06_12_input_and_output.md#6129-default-ports-for-input-output-and-errors).

Scheme Procedure: **open-bytevector-input-port** bv \[transcoder\] [¶](07_06_r6rs_support.md)

Scheme Procedure: **open-bytevector-output-port** \[transcoder\] [¶](07_06_r6rs_support.md)

See [Bytevector Ports](06_12_input_and_output.md#612102-bytevector-ports).

Scheme Procedure: **make-custom-binary-input-port** id read! get-position set-position! close [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-custom-binary-output-port** id write! get-position set-position! close [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-custom-binary-input/output-port** id read! write! get-position set-position! close [¶](07_06_r6rs_support.md)

See [Custom Ports](06_12_input_and_output.md#612104-custom-ports).

Scheme Procedure: **make-custom-textual-input-port** id read! get-position set-position! close [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-custom-textual-output-port** id write! get-position set-position! close [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-custom-textual-input/output-port** id read! write! get-position set-position! close [¶](07_06_r6rs_support.md)

See [Custom Ports](06_12_input_and_output.md#612104-custom-ports).

Scheme Procedure: **get-u8** port [¶](07_06_r6rs_support.md)

Scheme Procedure: **lookahead-u8** port [¶](07_06_r6rs_support.md)

Scheme Procedure: **get-bytevector-n** port count [¶](07_06_r6rs_support.md)

Scheme Procedure: **get-bytevector-n!** port bv start count [¶](07_06_r6rs_support.md)

Scheme Procedure: **get-bytevector-some** port [¶](07_06_r6rs_support.md)

Scheme Procedure: **get-bytevector-all** port [¶](07_06_r6rs_support.md)

Scheme Procedure: **put-u8** port octet [¶](07_06_r6rs_support.md)

Scheme Procedure: **put-bytevector** port bv \[start \[count\]\] [¶](07_06_r6rs_support.md)

See [Binary I/O](06_12_input_and_output.md#6122-binary-io).

Scheme Procedure: **get-char** textual-input-port [¶](07_06_r6rs_support.md)

Scheme Procedure: **lookahead-char** textual-input-port [¶](07_06_r6rs_support.md)

Scheme Procedure: **get-string-n** textual-input-port count [¶](07_06_r6rs_support.md)

Scheme Procedure: **get-string-n!** textual-input-port string start count [¶](07_06_r6rs_support.md)

Scheme Procedure: **get-string-all** textual-input-port [¶](07_06_r6rs_support.md)

Scheme Procedure: **get-line** textual-input-port [¶](07_06_r6rs_support.md)

Scheme Procedure: **put-char** port char [¶](07_06_r6rs_support.md)

Scheme Procedure: **put-string** port string \[start \[count\]\] [¶](07_06_r6rs_support.md)

See [Textual I/O](06_12_input_and_output.md#6124-textual-io).

Scheme Procedure: **get-datum** textual-input-port count [¶](07_06_r6rs_support.md)

Reads an external representation from textual-input-port and returns the datum it represents. The `get-datum` procedure returns the next datum that can be parsed from the given textual-input-port, updating textual-input-port to point exactly past the end of the external representation of the object.

Any _interlexeme space_ (comment or whitespace, see [Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions)) in the input is first skipped. If an end of file occurs after the interlexeme space, the end-of-file object is returned.

If a character inconsistent with an external representation is encountered in the input, an exception with condition types `&lexical` and `&i/o-read` is raised. Also, if the end of file is encountered after the beginning of an external representation, but the external representation is incomplete and therefore cannot be parsed, an exception with condition types `&lexical` and `&i/o-read` is raised.

Scheme Procedure: **put-datum** textual-output-port datum [¶](07_06_r6rs_support.md)

datum should be a datum value. The `put-datum` procedure writes an external representation of datum to textual-output-port. The specific external representation is implementation-dependent. However, whenever possible, an implementation should produce a representation for which `get-datum`, when reading the representation, will return an object equal (in the sense of `equal?`) to datum.

> **Note:** Not all datums may allow producing an external representation for which `get-datum` will produce an object that is equal to the original. Specifically, NaNs contained in datum may make this impossible.

> **Note:** The `put-datum` procedure merely writes the external representation, but no trailing delimiter. If `put-datum` is used to write several subsequent external representations to an output port, care should be taken to delimit them properly so they can be read back in by subsequent calls to `get-datum`.

Scheme Procedure: **flush-output-port** port [¶](07_06_r6rs_support.md)

See [Buffering](06_12_input_and_output.md#6126-buffering), for documentation on `force-output`.

* * *

Next: [rnrs io simple](07_06_r6rs_support.md#76218-rnrs-io-simple), Previous: [rnrs io ports](07_06_r6rs_support.md#76216-rnrs-io-ports), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.17 R6RS File Ports [¶](07_06_r6rs_support.md#76217-r6rs-file-ports)

The facilities described in this section are exported by the `(rnrs io ports)` module.

Scheme Syntax: **buffer-mode** buffer-mode-symbol [¶](07_06_r6rs_support.md)

buffer-mode-symbol must be a symbol whose name is one of `none`, `line`, and `block`. The result is the corresponding symbol, and specifies the associated buffer mode. See [Buffering](06_12_input_and_output.md#6126-buffering), for a discussion of these different buffer modes. To control the amount of buffering, use `setvbuf` instead. Note that only the name of buffer-mode-symbol is significant.

See [Buffering](06_12_input_and_output.md#6126-buffering), for a discussion of port buffering.

Scheme Procedure: **buffer-mode?** obj [¶](07_06_r6rs_support.md)

Returns `#t` if the argument is a valid buffer-mode symbol, and returns `#f` otherwise.

When opening a file, the various procedures accept a `file-options` object that encapsulates flags to specify how the file is to be opened. A `file-options` object is an enum-set (see [rnrs enums](07_06_r6rs_support.md#76226-rnrs-enums)) over the symbols constituting valid file options.

A file-options parameter name means that the corresponding argument must be a file-options object.

Scheme Syntax: **file-options** file-options-symbol ... [¶](07_06_r6rs_support.md)

Each file-options-symbol must be a symbol.

The `file-options` syntax returns a file-options object that encapsulates the specified options.

When supplied to an operation that opens a file for output, the file-options object returned by `(file-options)` specifies that the file is created if it does not exist and an exception with condition type `&i/o-file-already-exists` is raised if it does exist. The following standard options can be included to modify the default behavior.

`no-create`

If the file does not already exist, it is not created; instead, an exception with condition type `&i/o-file-does-not-exist` is raised. If the file already exists, the exception with condition type `&i/o-file-already-exists` is not raised and the file is truncated to zero length.

`no-fail`

If the file already exists, the exception with condition type `&i/o-file-already-exists` is not raised, even if `no-create` is not included, and the file is truncated to zero length.

`no-truncate`

If the file already exists and the exception with condition type `&i/o-file-already-exists` has been inhibited by inclusion of `no-create` or `no-fail`, the file is not truncated, but the port’s current position is still set to the beginning of the file.

These options have no effect when a file is opened only for input. Symbols other than those listed above may be used as file-options-symbols; they have implementation-specific meaning, if any.

> **Note:** Only the name of file-options-symbol is significant.

Scheme Procedure: **open-file-input-port** filename [¶](07_06_r6rs_support.md)

Scheme Procedure: **open-file-input-port** filename file-options [¶](07_06_r6rs_support.md)

Scheme Procedure: **open-file-input-port** filename file-options buffer-mode [¶](07_06_r6rs_support.md)

Scheme Procedure: **open-file-input-port** filename file-options buffer-mode maybe-transcoder [¶](07_06_r6rs_support.md)

maybe-transcoder must be either a transcoder or `#f`.

The `open-file-input-port` procedure returns an input port for the named file. The file-options and maybe-transcoder arguments are optional.

The file-options argument, which may determine various aspects of the returned port, defaults to the value of `(file-options)`.

The buffer-mode argument, if supplied, must be one of the symbols that name a buffer mode. The buffer-mode argument defaults to `block`.

If maybe-transcoder is a transcoder, it becomes the transcoder associated with the returned port.

If maybe-transcoder is `#f` or absent, the port will be a binary port and will support the `port-position` and `set-port-position!` operations. Otherwise the port will be a textual port, and whether it supports the `port-position` and `set-port-position!` operations is implementation-dependent (and possibly transcoder-dependent).

Scheme Procedure: **open-file-output-port** filename [¶](07_06_r6rs_support.md)

Scheme Procedure: **open-file-output-port** filename file-options [¶](07_06_r6rs_support.md)

Scheme Procedure: **open-file-output-port** filename file-options buffer-mode [¶](07_06_r6rs_support.md)

Scheme Procedure: **open-file-output-port** filename file-options buffer-mode maybe-transcoder [¶](07_06_r6rs_support.md)

maybe-transcoder must be either a transcoder or `#f`.

The `open-file-output-port` procedure returns an output port for the named file.

The file-options argument, which may determine various aspects of the returned port, defaults to the value of `(file-options)`.

The buffer-mode argument, if supplied, must be one of the symbols that name a buffer mode. The buffer-mode argument defaults to `block`.

If maybe-transcoder is a transcoder, it becomes the transcoder associated with the port.

If maybe-transcoder is `#f` or absent, the port will be a binary port and will support the `port-position` and `set-port-position!` operations. Otherwise the port will be a textual port, and whether it supports the `port-position` and `set-port-position!` operations is implementation-dependent (and possibly transcoder-dependent).

* * *

Next: [rnrs files](07_06_r6rs_support.md#76219-rnrs-files), Previous: [R6RS File Ports](07_06_r6rs_support.md#76217-r6rs-file-ports), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.18 rnrs io simple [¶](07_06_r6rs_support.md#76218-rnrs-io-simple)

The `(rnrs io simple (6))` library provides convenience functions for performing textual I/O on ports. This library also exports all of the condition types and associated procedures described in (see [I/O Conditions](07_06_r6rs_support.md#76214-io-conditions)). In the context of this section, when stating that a procedure behaves “identically” to the corresponding procedure in Guile’s core library, this is modulo the behavior wrt. conditions: such procedures raise the appropriate R6RS conditions in case of error, but otherwise behave identically.

> **Note:** There are still known issues regarding condition-correctness; some errors may still be thrown as native Guile exceptions instead of the appropriate R6RS conditions.

Scheme Procedure: **eof-object** [¶](07_06_r6rs_support.md)

Scheme Procedure: **eof-object?** obj [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by the `(rnrs io ports (6))` library. See [rnrs io ports](07_06_r6rs_support.md#76216-rnrs-io-ports), for documentation.

Scheme Procedure: **input-port?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **output-port?** obj [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by Guile’s core library. See [Ports](06_12_input_and_output.md#6121-ports), for documentation.

Scheme Procedure: **call-with-input-file** filename proc [¶](07_06_r6rs_support.md)

Scheme Procedure: **call-with-output-file** filename proc [¶](07_06_r6rs_support.md)

Scheme Procedure: **open-input-file** filename [¶](07_06_r6rs_support.md)

Scheme Procedure: **open-output-file** filename [¶](07_06_r6rs_support.md)

Scheme Procedure: **with-input-from-file** filename thunk [¶](07_06_r6rs_support.md)

Scheme Procedure: **with-output-to-file** filename thunk [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by Guile’s core library. See [File Ports](06_12_input_and_output.md#612101-file-ports), for documentation.

Scheme Procedure: **close-input-port** input-port [¶](07_06_r6rs_support.md)

Scheme Procedure: **close-output-port** output-port [¶](07_06_r6rs_support.md)

Closes the given input-port or output-port. These are legacy interfaces; just use `close-port`.

Scheme Procedure: **peek-char** [¶](07_06_r6rs_support.md)

Scheme Procedure: **peek-char** textual-input-port [¶](07_06_r6rs_support.md)

Scheme Procedure: **read-char** [¶](07_06_r6rs_support.md)

Scheme Procedure: **read-char** textual-input-port [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by Guile’s core library. See [Venerable Port Interfaces](06_12_input_and_output.md#61211-venerable-port-interfaces), for documentation.

Scheme Procedure: **read** [¶](07_06_r6rs_support.md)

Scheme Procedure: **read** textual-input-port [¶](07_06_r6rs_support.md)

This procedure is identical to the one provided by Guile’s core library. See [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code), for documentation.

Scheme Procedure: **display** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **display** obj textual-output-port [¶](07_06_r6rs_support.md)

Scheme Procedure: **newline** [¶](07_06_r6rs_support.md)

Scheme Procedure: **newline** textual-output-port [¶](07_06_r6rs_support.md)

Scheme Procedure: **write** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **write** obj textual-output-port [¶](07_06_r6rs_support.md)

Scheme Procedure: **write-char** char [¶](07_06_r6rs_support.md)

Scheme Procedure: **write-char** char textual-output-port [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by Guile’s core library. See [Venerable Port Interfaces](06_12_input_and_output.md#61211-venerable-port-interfaces), and See [Writing Scheme Values](06_16_reading_and_evaluating_scheme_code.md#6164-writing-scheme-values), for documentation.

* * *

Next: [rnrs programs](07_06_r6rs_support.md#76220-rnrs-programs), Previous: [rnrs io simple](07_06_r6rs_support.md#76218-rnrs-io-simple), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.19 rnrs files [¶](07_06_r6rs_support.md#76219-rnrs-files)

The `(rnrs files (6))` library provides the `file-exists?` and `delete-file` procedures, which test for the existence of a file and allow the deletion of files from the file system, respectively.

These procedures are identical to the ones provided by Guile’s core library. See [File System](07_02_03_file_system.md#723-file-system), for documentation.

* * *

Next: [rnrs arithmetic fixnums](07_06_r6rs_support.md#76221-rnrs-arithmetic-fixnums), Previous: [rnrs files](07_06_r6rs_support.md#76219-rnrs-files), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.20 rnrs programs [¶](07_06_r6rs_support.md#76220-rnrs-programs)

The `(rnrs programs (6))` library provides procedures for process management and introspection.

Scheme Procedure: **command-line** [¶](07_06_r6rs_support.md)

This procedure is identical to the one provided by Guile’s core library. See [Runtime Environment](07_02_06_runtime_environment.md#726-runtime-environment), for documentation.

Scheme Procedure: **exit** \[status\] [¶](07_06_r6rs_support.md)

This procedure is identical to the one provided by Guile’s core library. See [Processes](07_02_07_processes.md#727-processes), for documentation.

* * *

Next: [rnrs arithmetic flonums](07_06_r6rs_support.md#76222-rnrs-arithmetic-flonums), Previous: [rnrs programs](07_06_r6rs_support.md#76220-rnrs-programs), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.21 rnrs arithmetic fixnums [¶](07_06_r6rs_support.md#76221-rnrs-arithmetic-fixnums)

The `(rnrs arithmetic fixnums (6))` library provides procedures for performing arithmetic operations on an implementation-dependent range of exact integer values, which R6RS refers to as _fixnums_. In Guile, the size of a fixnum is determined by the size of the `SCM` type; a single SCM struct is guaranteed to be able to hold an entire fixnum, making fixnum computations particularly efficient—(see [The SCM Type](06_03_the_scm_type.md#63-the-scm-type)). On 32-bit systems, the most negative and most positive fixnum values are, respectively, -536870912 and 536870911.

Unless otherwise specified, all of the procedures below take fixnums as arguments, and will raise an `&assertion` condition if passed a non-fixnum argument or an `&implementation-restriction` condition if their result is not itself a fixnum.

Scheme Procedure: **fixnum?** obj [¶](07_06_r6rs_support.md)

Returns `#t` if obj is a fixnum, `#f` otherwise.

Scheme Procedure: **fixnum-width** [¶](07_06_r6rs_support.md)

Scheme Procedure: **least-fixnum** [¶](07_06_r6rs_support.md)

Scheme Procedure: **greatest-fixnum** [¶](07_06_r6rs_support.md)

These procedures return, respectively, the maximum number of bits necessary to represent a fixnum value in Guile, the minimum fixnum value, and the maximum fixnum value.

Scheme Procedure: **fx=?** fx1 fx2 fx3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fx>?** fx1 fx2 fx3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fx<?** fx1 fx2 fx3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fx>=?** fx1 fx2 fx3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fx<=?** fx1 fx2 fx3 ... [¶](07_06_r6rs_support.md)

These procedures return `#t` if their fixnum arguments are (respectively): equal, monotonically increasing, monotonically decreasing, monotonically nondecreasing, or monotonically nonincreasing; `#f` otherwise.

Scheme Procedure: **fxzero?** fx [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxpositive?** fx [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxnegative?** fx [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxodd?** fx [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxeven?** fx [¶](07_06_r6rs_support.md)

These numerical predicates return `#t` if fx is, respectively, zero, greater than zero, less than zero, odd, or even; `#f` otherwise.

Scheme Procedure: **fxmax** fx1 fx2 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxmin** fx1 fx2 ... [¶](07_06_r6rs_support.md)

These procedures return the maximum or minimum of their arguments.

Scheme Procedure: **fx+** fx1 fx2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fx\*** fx1 fx2 [¶](07_06_r6rs_support.md)

These procedures return the sum or product of their arguments.

Scheme Procedure: **fx-** fx1 fx2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fx-** fx [¶](07_06_r6rs_support.md)

Returns the difference of fx1 and fx2, or the negation of fx, if called with a single argument.

An `&assertion` condition is raised if the result is not itself a fixnum.

Scheme Procedure: **fxdiv-and-mod** fx1 fx2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxdiv** fx1 fx2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxmod** fx1 fx2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxdiv0-and-mod0** fx1 fx2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxdiv0** fx1 fx2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxmod0** fx1 fx2 [¶](07_06_r6rs_support.md)

These procedures implement number-theoretic division on fixnums; See [(rnrs base)](https://doc.guix.gnu.org/guile/latest/en/rnrs%20base.html#Top), for a description of their semantics.

Scheme Procedure: **fx+/carry** fx1 fx2 fx3 [¶](07_06_r6rs_support.md)

Returns the two fixnum results of the following computation:

(let\* ((s ([+](06_06_02_numerical_data_types.md) fx1 fx2 fx3))
       (s0 ([mod0](07_06_r6rs_support.md) s ([expt](06_06_02_numerical_data_types.md) 2 ([fixnum-width](07_06_r6rs_support.md)))))
       (s1 ([div0](07_06_r6rs_support.md) s ([expt](06_06_02_numerical_data_types.md) 2 ([fixnum-width](07_06_r6rs_support.md))))))
  ([values](06_11_controlling_the_flow_of_program_execution.md) s0 s1))

Scheme Procedure: **fx-/carry** fx1 fx2 fx3 [¶](07_06_r6rs_support.md)

Returns the two fixnum results of the following computation:

(let\* ((d ([\-](06_06_02_numerical_data_types.md) fx1 fx2 fx3))
       (d0 ([mod0](07_06_r6rs_support.md) d ([expt](06_06_02_numerical_data_types.md) 2 ([fixnum-width](07_06_r6rs_support.md)))))
       (d1 ([div0](07_06_r6rs_support.md) d ([expt](06_06_02_numerical_data_types.md) 2 ([fixnum-width](07_06_r6rs_support.md))))))
  ([values](06_11_controlling_the_flow_of_program_execution.md) d0 d1))

Scheme Procedure: **fx\*/carry** fx1 fx2 fx3 [¶](07_06_r6rs_support.md)

Returns the two fixnum results of the following computation:
(let\* ((s ([+](06_06_02_numerical_data_types.md) ([\*](06_06_02_numerical_data_types.md) fx1 fx2) fx3))
       (s0 ([mod0](07_06_r6rs_support.md) s ([expt](06_06_02_numerical_data_types.md) 2 ([fixnum-width](07_06_r6rs_support.md)))))
       (s1 ([div0](07_06_r6rs_support.md) s ([expt](06_06_02_numerical_data_types.md) 2 ([fixnum-width](07_06_r6rs_support.md))))))
  ([values](06_11_controlling_the_flow_of_program_execution.md) s0 s1))

Scheme Procedure: **fxnot** fx [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxand** fx1 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxior** fx1 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxxor** fx1 ... [¶](07_06_r6rs_support.md)

These procedures are identical to the `lognot`, `logand`, `logior`, and `logxor` procedures provided by Guile’s core library. See [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations), for documentation.

Scheme Procedure: **fxif** fx1 fx2 fx3 [¶](07_06_r6rs_support.md)

Returns the bitwise “if” of its fixnum arguments. The bit at position `i` in the return value will be the `i`th bit from fx2 if the `i`th bit of fx1 is 1, the `i`th bit from fx3.

Scheme Procedure: **fxbit-count** fx [¶](07_06_r6rs_support.md)

Returns the number of 1 bits in the two’s complement representation of fx.

Scheme Procedure: **fxlength** fx [¶](07_06_r6rs_support.md)

Returns the number of bits necessary to represent fx.

Scheme Procedure: **fxfirst-bit-set** fx [¶](07_06_r6rs_support.md)

Returns the index of the least significant 1 bit in the two’s complement representation of fx.

Scheme Procedure: **fxbit-set?** fx1 fx2 [¶](07_06_r6rs_support.md)

Returns `#t` if the fx2th bit in the two’s complement representation of fx1 is 1, `#f` otherwise.

Scheme Procedure: **fxcopy-bit** fx1 fx2 fx3 [¶](07_06_r6rs_support.md)

Returns the result of setting the fx2th bit of fx1 to the fx2th bit of fx3.

Scheme Procedure: **fxbit-field** fx1 fx2 fx3 [¶](07_06_r6rs_support.md)

Returns the integer representation of the contiguous sequence of bits in fx1 that starts at position fx2 (inclusive) and ends at position fx3 (exclusive).

Scheme Procedure: **fxcopy-bit-field** fx1 fx2 fx3 fx4 [¶](07_06_r6rs_support.md)

Returns the result of replacing the bit field in fx1 with start and end positions fx2 and fx3 with the corresponding bit field from fx4.

Scheme Procedure: **fxarithmetic-shift** fx1 fx2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxarithmetic-shift-left** fx1 fx2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fxarithmetic-shift-right** fx1 fx2 [¶](07_06_r6rs_support.md)

Returns the result of shifting the bits of fx1 right or left by the fx2 positions. `fxarithmetic-shift` is identical to `fxarithmetic-shift-left`.

Scheme Procedure: **fxrotate-bit-field** fx1 fx2 fx3 fx4 [¶](07_06_r6rs_support.md)

Returns the result of cyclically permuting the bit field in fx1 with start and end positions fx2 and fx3 by fx4 bits in the direction of more significant bits.

Scheme Procedure: **fxreverse-bit-field** fx1 fx2 fx3 [¶](07_06_r6rs_support.md)

Returns the result of reversing the order of the bits of fx1 between position fx2 (inclusive) and position fx3 (exclusive).

* * *

Next: [rnrs arithmetic bitwise](07_06_r6rs_support.md#76223-rnrs-arithmetic-bitwise), Previous: [rnrs arithmetic fixnums](07_06_r6rs_support.md#76221-rnrs-arithmetic-fixnums), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.22 rnrs arithmetic flonums [¶](07_06_r6rs_support.md#76222-rnrs-arithmetic-flonums)

The `(rnrs arithmetic flonums (6))` library provides procedures for performing arithmetic operations on inexact representations of real numbers, which R6RS refers to as _flonums_.

Unless otherwise specified, all of the procedures below take flonums as arguments, and will raise an `&assertion` condition if passed a non-flonum argument.

Scheme Procedure: **flonum?** obj [¶](07_06_r6rs_support.md)

Returns `#t` if obj is a flonum, `#f` otherwise.

Scheme Procedure: **real->flonum** x [¶](07_06_r6rs_support.md)

Returns the flonum that is numerically closest to the real number x.

Scheme Procedure: **fl=?** fl1 fl2 fl3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fl<?** fl1 fl2 fl3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fl<=?** fl1 fl2 fl3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fl>?** fl1 fl2 fl3 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fl>=?** fl1 fl2 fl3 ... [¶](07_06_r6rs_support.md)

These procedures return `#t` if their flonum arguments are (respectively): equal, monotonically increasing, monotonically decreasing, monotonically nondecreasing, or monotonically nonincreasing; `#f` otherwise.

Scheme Procedure: **flinteger?** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flzero?** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flpositive?** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flnegative?** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flodd?** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **fleven?** fl [¶](07_06_r6rs_support.md)

These numerical predicates return `#t` if fl is, respectively, an integer, zero, greater than zero, less than zero, odd, even, `#f` otherwise. In the case of `flodd?` and `fleven?`, fl must be an integer-valued flonum.

Scheme Procedure: **flfinite?** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flinfinite?** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flnan?** fl [¶](07_06_r6rs_support.md)

These numerical predicates return `#t` if fl is, respectively, not infinite, infinite, or a `NaN` value.

Scheme Procedure: **flmax** fl1 fl2 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **flmin** fl1 fl2 ... [¶](07_06_r6rs_support.md)

These procedures return the maximum or minimum of their arguments.

Scheme Procedure: **fl+** fl1 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fl\*** fl ... [¶](07_06_r6rs_support.md)

These procedures return the sum or product of their arguments.

Scheme Procedure: **fl-** fl1 fl2 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fl-** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **fl/** fl1 fl2 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **fl/** fl [¶](07_06_r6rs_support.md)

These procedures return, respectively, the difference or quotient of their arguments when called with two arguments; when called with a single argument, they return the additive or multiplicative inverse of fl.

Scheme Procedure: **flabs** fl [¶](07_06_r6rs_support.md)

Returns the absolute value of fl.

Scheme Procedure: **fldiv-and-mod** fl1 fl2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fldiv** fl1 fl2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fldmod** fl1 fl2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fldiv0-and-mod0** fl1 fl2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **fldiv0** fl1 fl2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **flmod0** fl1 fl2 [¶](07_06_r6rs_support.md)

These procedures implement number-theoretic division on flonums; See [(rnrs base)](https://doc.guix.gnu.org/guile/latest/en/rnrs%20base.html#Top), for a description for their semantics.

Scheme Procedure: **flnumerator** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **fldenominator** fl [¶](07_06_r6rs_support.md)

These procedures return the numerator or denominator of fl as a flonum.

Scheme Procedure: **flfloor** fl1 [¶](07_06_r6rs_support.md)

Scheme Procedure: **flceiling** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **fltruncate** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flround** fl [¶](07_06_r6rs_support.md)

These procedures are identical to the `floor`, `ceiling`, `truncate`, and `round` procedures provided by Guile’s core library. See [Arithmetic Functions](06_06_02_numerical_data_types.md#66211-arithmetic-functions), for documentation.

Scheme Procedure: **flexp** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **fllog** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **fllog** fl1 fl2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **flsin** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flcos** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **fltan** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flasin** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flacos** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flatan** fl [¶](07_06_r6rs_support.md)

Scheme Procedure: **flatan** fl1 fl2 [¶](07_06_r6rs_support.md)

These procedures, which compute the usual transcendental functions, are the flonum variants of the procedures provided by the R6RS base library (see [(rnrs base)](https://doc.guix.gnu.org/guile/latest/en/rnrs%20base.html#Top)).

Scheme Procedure: **flsqrt** fl [¶](07_06_r6rs_support.md)

Returns the square root of fl. If fl is `-0.0`, \-0.0 is returned; for other negative values, a `NaN` value is returned.

Scheme Procedure: **flexpt** fl1 fl2 [¶](07_06_r6rs_support.md)

Returns the value of fl1 raised to the power of fl2.

The following condition types are provided to allow Scheme implementations that do not support infinities or `NaN` values to indicate that a computation resulted in such a value. Guile supports both of these, so these conditions will never be raised by Guile’s standard libraries implementation.

Condition Type: **&no-infinities** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-no-infinities-violation** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **no-infinities-violation?** [¶](07_06_r6rs_support.md)

A condition type indicating that a computation resulted in an infinite value on a Scheme implementation incapable of representing infinities.

Condition Type: **&no-nans** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-no-nans-violation** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **no-nans-violation?** obj [¶](07_06_r6rs_support.md)

A condition type indicating that a computation resulted in a `NaN` value on a Scheme implementation incapable of representing `NaN`s.

Scheme Procedure: **fixnum->flonum** fx [¶](07_06_r6rs_support.md)

Returns the flonum that is numerically closest to the fixnum fx.

* * *

Next: [rnrs syntax-case](07_06_r6rs_support.md#76224-rnrs-syntax-case), Previous: [rnrs arithmetic flonums](07_06_r6rs_support.md#76222-rnrs-arithmetic-flonums), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.23 rnrs arithmetic bitwise [¶](07_06_r6rs_support.md#76223-rnrs-arithmetic-bitwise)

The `(rnrs arithmetic bitwise (6))` library provides procedures for performing bitwise arithmetic operations on the two’s complement representations of fixnums.

This library and the procedures it exports share functionality with SRFI-60, which provides support for bitwise manipulation of integers (see [SRFI-60 - Integers as Bits](07_05_34_srfi60_integers_as_bits.md#7534-srfi-60---integers-as-bits)).

Scheme Procedure: **bitwise-not** ei [¶](07_06_r6rs_support.md)

Scheme Procedure: **bitwise-and** ei1 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **bitwise-ior** ei1 ... [¶](07_06_r6rs_support.md)

Scheme Procedure: **bitwise-xor** ei1 ... [¶](07_06_r6rs_support.md)

These procedures are identical to the `lognot`, `logand`, `logior`, and `logxor` procedures provided by Guile’s core library. See [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations), for documentation.

Scheme Procedure: **bitwise-if** ei1 ei2 ei3 [¶](07_06_r6rs_support.md)

Returns the bitwise “if” of its arguments. The bit at position `i` in the return value will be the `i`th bit from ei2 if the `i`th bit of ei1 is 1, the `i`th bit from ei3.

Scheme Procedure: **bitwise-bit-count** ei [¶](07_06_r6rs_support.md)

Returns the number of 1 bits in the two’s complement representation of ei.

Scheme Procedure: **bitwise-length** ei [¶](07_06_r6rs_support.md)

Returns the number of bits necessary to represent ei.

Scheme Procedure: **bitwise-first-bit-set** ei [¶](07_06_r6rs_support.md)

Returns the index of the least significant 1 bit in the two’s complement representation of ei.

Scheme Procedure: **bitwise-bit-set?** ei1 ei2 [¶](07_06_r6rs_support.md)

Returns `#t` if the ei2th bit in the two’s complement representation of ei1 is 1, `#f` otherwise.

Scheme Procedure: **bitwise-copy-bit** ei1 ei2 ei3 [¶](07_06_r6rs_support.md)

Returns the result of setting the ei2th bit of ei1 to the ei2th bit of ei3.

Scheme Procedure: **bitwise-bit-field** ei1 ei2 ei3 [¶](07_06_r6rs_support.md)

Returns the integer representation of the contiguous sequence of bits in ei1 that starts at position ei2 (inclusive) and ends at position ei3 (exclusive).

Scheme Procedure: **bitwise-copy-bit-field** ei1 ei2 ei3 ei4 [¶](07_06_r6rs_support.md)

Returns the result of replacing the bit field in ei1 with start and end positions ei2 and ei3 with the corresponding bit field from ei4.

Scheme Procedure: **bitwise-arithmetic-shift** ei1 ei2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **bitwise-arithmetic-shift-left** ei1 ei2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **bitwise-arithmetic-shift-right** ei1 ei2 [¶](07_06_r6rs_support.md)

Returns the result of shifting the bits of ei1 right or left by the ei2 positions. `bitwise-arithmetic-shift` is identical to `bitwise-arithmetic-shift-left`.

Scheme Procedure: **bitwise-rotate-bit-field** ei1 ei2 ei3 ei4 [¶](07_06_r6rs_support.md)

Returns the result of cyclically permuting the bit field in ei1 with start and end positions ei2 and ei3 by ei4 bits in the direction of more significant bits.

Scheme Procedure: **bitwise-reverse-bit-field** ei1 ei2 ei3 [¶](07_06_r6rs_support.md)

Returns the result of reversing the order of the bits of ei1 between position ei2 (inclusive) and position ei3 (exclusive).

* * *

Next: [rnrs hashtables](07_06_r6rs_support.md#76225-rnrs-hashtables), Previous: [rnrs arithmetic bitwise](07_06_r6rs_support.md#76223-rnrs-arithmetic-bitwise), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.24 rnrs syntax-case [¶](07_06_r6rs_support.md#76224-rnrs-syntax-case)

The `(rnrs syntax-case (6))` library provides access to the `syntax-case` system for writing hygienic macros. With one exception, all of the forms and procedures exported by this library are “re-exports” of Guile’s native support for `syntax-case`; See [Support for the `syntax-case` System](06_08_macros.md#683-support-for-the-syntax-case-system), for documentation, examples, and rationale.

Scheme Procedure: **make-variable-transformer** proc [¶](07_06_r6rs_support.md)

Creates a new variable transformer out of proc, a procedure that takes a syntax object as input and returns a syntax object. If an identifier to which the result of this procedure is bound appears on the left-hand side of a `set!` expression, proc will be called with a syntax object representing the entire `set!` expression, and its return value will replace that `set!` expression.

Scheme Syntax: **syntax-case** expression (literal ...) clause ... [¶](07_06_r6rs_support.md)

The `syntax-case` pattern matching form.

Scheme Syntax: **syntax** template [¶](07_06_r6rs_support.md)

Scheme Syntax: **quasisyntax** template [¶](07_06_r6rs_support.md)

Scheme Syntax: **unsyntax** template [¶](07_06_r6rs_support.md)

Scheme Syntax: **unsyntax-splicing** template [¶](07_06_r6rs_support.md)

These forms allow references to be made in the body of a syntax-case output expression subform to datum and non-datum values. They are identical to the forms provided by Guile’s core library; See [Support for the `syntax-case` System](06_08_macros.md#683-support-for-the-syntax-case-system), for documentation.

Scheme Procedure: **identifier?** obj [¶](07_06_r6rs_support.md)

Scheme Procedure: **bound-identifier=?** id1 id2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **free-identifier=?** id1 id2 [¶](07_06_r6rs_support.md)

These predicate procedures operate on syntax objects representing Scheme identifiers. `identifier?` returns `#t` if obj represents an identifier, `#f` otherwise. `bound-identifier=?` returns `#t` if and only if a binding for id1 would capture a reference to id2 in the transformer’s output, or vice-versa. `free-identifier=?` returns `#t` if and only id1 and id2 would refer to the same binding in the output of the transformer, independent of any bindings introduced by the transformer.

Scheme Procedure: **generate-temporaries** l [¶](07_06_r6rs_support.md)

Returns a list, of the same length as l, which must be a list or a syntax object representing a list, of globally unique symbols.

Scheme Procedure: **syntax->datum** syntax-object [¶](07_06_r6rs_support.md)

Scheme Procedure: **datum->syntax** template-id datum [¶](07_06_r6rs_support.md)

These procedures convert wrapped syntax objects to and from Scheme datum values. The syntax object returned by `datum->syntax` shares contextual information with the syntax object template-id.

Scheme Procedure: **syntax-violation** whom message form [¶](07_06_r6rs_support.md)

Scheme Procedure: **syntax-violation** whom message form subform [¶](07_06_r6rs_support.md)

Constructs a new compound condition that includes the following simple conditions:

*   If whom is not `#f`, a `&who` condition with the whom as its field
*   A `&message` condition with the specified message
*   A `&syntax` condition with the specified form and optional subform fields

* * *

Next: [rnrs enums](07_06_r6rs_support.md#76226-rnrs-enums), Previous: [rnrs syntax-case](07_06_r6rs_support.md#76224-rnrs-syntax-case), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.25 rnrs hashtables [¶](07_06_r6rs_support.md#76225-rnrs-hashtables)

The `(rnrs hashtables (6))` library provides structures and procedures for creating and accessing hash tables. The hash tables API defined by R6RS is substantially similar to both Guile’s native hash tables implementation as well as the one provided by SRFI-69; See [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables), and [SRFI-69 - Basic hash tables](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---basic-hash-tables), respectively. Note that you can write portable R6RS library code that manipulates SRFI-69 hash tables (by importing the `(srfi :69)` library); however, hash tables created by one API cannot be used by another.

Like SRFI-69 hash tables—and unlike Guile’s native ones—R6RS hash tables associate hash and equality functions with a hash table at the time of its creation. Additionally, R6RS allows for the creation (via `hashtable-copy`; see below) of immutable hash tables.

Scheme Procedure: **make-eq-hashtable** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-eq-hashtable** k [¶](07_06_r6rs_support.md)

Returns a new hash table that uses `eq?` to compare keys and Guile’s `hashq` procedure as a hash function. If k is given, it specifies the initial capacity of the hash table.

Scheme Procedure: **make-eqv-hashtable** [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-eqv-hashtable** k [¶](07_06_r6rs_support.md)

Returns a new hash table that uses `eqv?` to compare keys and Guile’s `hashv` procedure as a hash function. If k is given, it specifies the initial capacity of the hash table.

Scheme Procedure: **make-hashtable** hash-function equiv [¶](07_06_r6rs_support.md)

Scheme Procedure: **make-hashtable** hash-function equiv k [¶](07_06_r6rs_support.md)

Returns a new hash table that uses equiv to compare keys and hash-function as a hash function. equiv must be a procedure that accepts two arguments and returns a true value if they are equivalent, `#f` otherwise; hash-function must be a procedure that accepts one argument and returns a non-negative integer.

If k is given, it specifies the initial capacity of the hash table.

Scheme Procedure: **hashtable?** obj [¶](07_06_r6rs_support.md)

Returns `#t` if obj is an R6RS hash table, `#f` otherwise.

Scheme Procedure: **hashtable-size** hashtable [¶](07_06_r6rs_support.md)

Returns the number of keys currently in the hash table hashtable.

Scheme Procedure: **hashtable-ref** hashtable key default [¶](07_06_r6rs_support.md)

Returns the value associated with key in the hash table hashtable, or default if none is found.

Scheme Procedure: **hashtable-set!** hashtable key obj [¶](07_06_r6rs_support.md)

Associates the key key with the value obj in the hash table hashtable, and returns an unspecified value. An `&assertion` condition is raised if hashtable is immutable.

Scheme Procedure: **hashtable-delete!** hashtable key [¶](07_06_r6rs_support.md)

Removes any association found for the key key in the hash table hashtable, and returns an unspecified value. An `&assertion` condition is raised if hashtable is immutable.

Scheme Procedure: **hashtable-contains?** hashtable key [¶](07_06_r6rs_support.md)

Returns `#t` if the hash table hashtable contains an association for the key key, `#f` otherwise.

Scheme Procedure: **hashtable-update!** hashtable key proc default [¶](07_06_r6rs_support.md)

Associates with key in the hash table hashtable the result of calling proc, which must be a procedure that takes one argument, on the value currently associated key in hashtable—or on default if no such association exists. An `&assertion` condition is raised if hashtable is immutable.

Scheme Procedure: **hashtable-copy** hashtable [¶](07_06_r6rs_support.md)

Scheme Procedure: **hashtable-copy** hashtable mutable [¶](07_06_r6rs_support.md)

Returns a copy of the hash table hashtable. If the optional argument mutable is provided and is a true value, the new hash table will be mutable.

Scheme Procedure: **hashtable-clear!** hashtable [¶](07_06_r6rs_support.md)

Scheme Procedure: **hashtable-clear!** hashtable k [¶](07_06_r6rs_support.md)

Removes all of the associations from the hash table hashtable. The optional argument k, which specifies a new capacity for the hash table, is accepted by Guile’s `(rnrs hashtables)` implementation, but is ignored.

Scheme Procedure: **hashtable-keys** hashtable [¶](07_06_r6rs_support.md)

Returns a vector of the keys with associations in the hash table hashtable, in an unspecified order.

Scheme Procedure: **hashtable-entries** hashtable [¶](07_06_r6rs_support.md)

Return two values—a vector of the keys with associations in the hash table hashtable, and a vector of the values to which these keys are mapped, in corresponding but unspecified order.

Scheme Procedure: **hashtable-equivalence-function** hashtable [¶](07_06_r6rs_support.md)

Returns the equivalence predicated use by hashtable. This procedure returns `eq?` and `eqv?`, respectively, for hash tables created by `make-eq-hashtable` and `make-eqv-hashtable`.

Scheme Procedure: **hashtable-hash-function** hashtable [¶](07_06_r6rs_support.md)

Returns the hash function used by hashtable. For hash tables created by `make-eq-hashtable` or `make-eqv-hashtable`, `#f` is returned.

Scheme Procedure: **hashtable-mutable?** hashtable [¶](07_06_r6rs_support.md)

Returns `#t` if hashtable is mutable, `#f` otherwise.

A number of hash functions are provided for convenience:

Scheme Procedure: **equal-hash** obj [¶](07_06_r6rs_support.md)

Returns an integer hash value for obj, based on its structure and current contents. This hash function is suitable for use with `equal?` as an equivalence function.

Scheme Procedure: **string-hash** string [¶](07_06_r6rs_support.md)

Scheme Procedure: **symbol-hash** symbol [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by Guile’s core library. See [Hash Table Reference](06_06_22_hash_tables.md#66222-hash-table-reference), for documentation.

Scheme Procedure: **string-ci-hash** string [¶](07_06_r6rs_support.md)

Returns an integer hash value for string based on its contents, ignoring case. This hash function is suitable for use with `string-ci=?` as an equivalence function.

* * *

Next: [rnrs](07_06_r6rs_support.md#76227-rnrs), Previous: [rnrs hashtables](07_06_r6rs_support.md#76225-rnrs-hashtables), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.26 rnrs enums [¶](07_06_r6rs_support.md#76226-rnrs-enums)

The `(rnrs enums (6))` library provides structures and procedures for working with enumerable sets of symbols. Guile’s implementation defines an _enum-set_ record type that encapsulates a finite set of distinct symbols, the _universe_, and a subset of these symbols, which define the enumeration set.

The SRFI-1 list library provides a number of procedures for performing set operations on lists; Guile’s `(rnrs enums)` implementation makes use of several of them. See [Set Operations on Lists](07_05_03_srfi1_list_library.md#75310-set-operations-on-lists), for more information.

Scheme Procedure: **make-enumeration** symbol-list [¶](07_06_r6rs_support.md)

Returns a new enum-set whose universe and enumeration set are both equal to symbol-list, a list of symbols.

Scheme Procedure: **enum-set-universe** enum-set [¶](07_06_r6rs_support.md)

Returns an enum-set representing the universe of enum-set, an enum-set.

Scheme Procedure: **enum-set-indexer** enum-set [¶](07_06_r6rs_support.md)

Returns a procedure that takes a single argument and returns the zero-indexed position of that argument in the universe of enum-set, or `#f` if its argument is not a member of that universe.

Scheme Procedure: **enum-set-constructor** enum-set [¶](07_06_r6rs_support.md)

Returns a procedure that takes a single argument, a list of symbols from the universe of enum-set, an enum-set, and returns a new enum-set with the same universe that represents a subset containing the specified symbols.

Scheme Procedure: **enum-set->list** enum-set [¶](07_06_r6rs_support.md)

Returns a list containing the symbols of the set represented by enum-set, an enum-set, in the order that they appear in the universe of enum-set.

Scheme Procedure: **enum-set-member?** symbol enum-set [¶](07_06_r6rs_support.md)

Scheme Procedure: **enum-set-subset?** enum-set1 enum-set2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **enum-set=?** enum-set1 enum-set2 [¶](07_06_r6rs_support.md)

These procedures test for membership of symbols and enum-sets in other enum-sets. `enum-set-member?` returns `#t` if and only if symbol is a member of the subset specified by enum-set. `enum-set-subset?` returns `#t` if and only if the universe of enum-set1 is a subset of the universe of enum-set2 and every symbol in enum-set1 is present in enum-set2. `enum-set=?` returns `#t` if and only if enum-set1 is a subset, as per `enum-set-subset?` of enum-set2 and vice versa.

Scheme Procedure: **enum-set-union** enum-set1 enum-set2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **enum-set-intersection** enum-set1 enum-set2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **enum-set-difference** enum-set1 enum-set2 [¶](07_06_r6rs_support.md)

These procedures return, respectively, the union, intersection, and difference of their enum-set arguments.

Scheme Procedure: **enum-set-complement** enum-set [¶](07_06_r6rs_support.md)

Returns enum-set’s complement (an enum-set), with regard to its universe.

Scheme Procedure: **enum-set-projection** enum-set1 enum-set2 [¶](07_06_r6rs_support.md)

Returns the projection of the enum-set enum-set1 onto the universe of the enum-set enum-set2.

Scheme Syntax: **define-enumeration** type-name (symbol ...) constructor-syntax [¶](07_06_r6rs_support.md)

Evaluates to two new definitions: A constructor bound to constructor-syntax that behaves similarly to constructors created by `enum-set-constructor`, above, and creates new enum-sets in the universe specified by `(symbol ...)`; and a “predicate macro” bound to type-name, which has the following form:

(type-name sym)

If sym is a member of the universe specified by the symbols above, this form evaluates to sym. Otherwise, a `&syntax` condition is raised.

* * *

Next: [rnrs eval](07_06_r6rs_support.md#76228-rnrs-eval), Previous: [rnrs enums](07_06_r6rs_support.md#76226-rnrs-enums), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.27 rnrs [¶](07_06_r6rs_support.md#76227-rnrs)

The `(rnrs (6))` library is a composite of all of the other R6RS standard libraries—it imports and re-exports all of their exported procedures and syntactic forms—with the exception of the following libraries:

*   `(rnrs eval (6))`
*   `(rnrs mutable-pairs (6))`
*   `(rnrs mutable-strings (6))`
*   `(rnrs r5rs (6))`

* * *

Next: [rnrs mutable-pairs](07_06_r6rs_support.md#76229-rnrs-mutable-pairs), Previous: [rnrs](07_06_r6rs_support.md#76227-rnrs), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.28 rnrs eval [¶](07_06_r6rs_support.md#76228-rnrs-eval)

The `(rnrs eval (6)` library provides procedures for performing “on-the-fly” evaluation of expressions.

Scheme Procedure: **eval** expression environment [¶](07_06_r6rs_support.md)

Evaluates expression, which must be a datum representation of a valid Scheme expression, in the environment specified by environment. This procedure is identical to the one provided by Guile’s code library; See [Procedures for On the Fly Evaluation](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation), for documentation.

Scheme Procedure: **environment** import-spec ... [¶](07_06_r6rs_support.md)

Constructs and returns a new environment based on the specified import-specs, which must be datum representations of the import specifications used with the `import` form. See [R6RS Libraries](06_18_modules.md#6186-r6rs-libraries), for documentation.

* * *

Next: [rnrs mutable-strings](07_06_r6rs_support.md#76230-rnrs-mutable-strings), Previous: [rnrs eval](07_06_r6rs_support.md#76228-rnrs-eval), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.29 rnrs mutable-pairs [¶](07_06_r6rs_support.md#76229-rnrs-mutable-pairs)

The `(rnrs mutable-pairs (6))` library provides the `set-car!` and `set-cdr!` procedures, which allow the `car` and `cdr` fields of a pair to be modified.

These procedures are identical to the ones provide by Guile’s core library. See [Pairs](06_06_08_pairs.md#668-pairs), for documentation. All pairs in Guile are mutable; consequently, these procedures will never throw the `&assertion` condition described in the R6RS libraries specification.

* * *

Next: [rnrs r5rs](07_06_r6rs_support.md#76231-rnrs-r5rs), Previous: [rnrs mutable-pairs](07_06_r6rs_support.md#76229-rnrs-mutable-pairs), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.30 rnrs mutable-strings [¶](07_06_r6rs_support.md#76230-rnrs-mutable-strings)

The `(rnrs mutable-strings (6))` library provides the `string-set!` and `string-fill!` procedures, which allow the content of strings to be modified “in-place.”

These procedures are identical to the ones provided by Guile’s core library. See [String Modification](06_06_05_strings.md#6656-string-modification), for documentation. All strings in Guile are mutable; consequently, these procedures will never throw the `&assertion` condition described in the R6RS libraries specification.

* * *

Previous: [rnrs mutable-strings](07_06_r6rs_support.md#76230-rnrs-mutable-strings), Up: [R6RS Standard Libraries](07_06_r6rs_support.md#762-r6rs-standard-libraries)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.6.2.31 rnrs r5rs [¶](07_06_r6rs_support.md#76231-rnrs-r5rs)

The `(rnrs r5rs (6))` library exports bindings for some procedures present in R5RS but omitted from the R6RS base library specification.

Scheme Procedure: **exact->inexact** z [¶](07_06_r6rs_support.md)

Scheme Procedure: **inexact->exact** z [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by Guile’s core library. See [Exact and Inexact Numbers](06_06_02_numerical_data_types.md#6625-exact-and-inexact-numbers), for documentation.

Scheme Procedure: **quotient** n1 n2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **remainder** n1 n2 [¶](07_06_r6rs_support.md)

Scheme Procedure: **modulo** n1 n2 [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by Guile’s core library. See [Operations on Integer Values](06_06_02_numerical_data_types.md#6627-operations-on-integer-values), for documentation.

Scheme Syntax: **delay** expr [¶](07_06_r6rs_support.md)

Scheme Procedure: **force** promise [¶](07_06_r6rs_support.md)

The `delay` form and the `force` procedure are identical to their counterparts in Guile’s core library. See [Delayed Evaluation](06_16_reading_and_evaluating_scheme_code.md#61610-delayed-evaluation), for documentation.

Scheme Procedure: **null-environment** n [¶](07_06_r6rs_support.md)

Scheme Procedure: **scheme-report-environment** n [¶](07_06_r6rs_support.md)

These procedures are identical to the ones provided by the `(ice-9 r5rs)` Guile module. See [Environments](06_18_modules.md#61812-environments), for documentation.

* * *

Next: [Pattern Matching](07_08_pattern_matching.md#78-pattern-matching), Previous: [R6RS Support](07_06_r6rs_support.md#76-r6rs-support), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
