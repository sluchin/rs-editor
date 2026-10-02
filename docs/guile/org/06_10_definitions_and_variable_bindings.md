### 6.10 Definitions and Variable Bindings [¶](06_10_definitions_and_variable_bindings.md#610-definitions-and-variable-bindings)

Scheme supports the definition of variables in different contexts. Variables can be defined at the top level, so that they are visible in the entire program, and variables can be defined locally to procedures and expressions. This is important for modularity and data abstraction.

*   [Top Level Variable Definitions](06_10_definitions_and_variable_bindings.md#6101-top-level-variable-definitions)
*   [Local Variable Bindings](06_10_definitions_and_variable_bindings.md#6102-local-variable-bindings)
*   [Internal definitions](06_10_definitions_and_variable_bindings.md#6103-internal-definitions)
*   [Querying variable bindings](06_10_definitions_and_variable_bindings.md#6104-querying-variable-bindings)
*   [Binding multiple return values](06_10_definitions_and_variable_bindings.md#6105-binding-multiple-return-values)

* * *

Next: [Local Variable Bindings](06_10_definitions_and_variable_bindings.md#6102-local-variable-bindings), Up: [Definitions and Variable Bindings](06_10_definitions_and_variable_bindings.md#610-definitions-and-variable-bindings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.10.1 Top Level Variable Definitions [¶](06_10_definitions_and_variable_bindings.md#6101-top-level-variable-definitions)

At the top level of a program (i.e., not nested within any other expression), a definition of the form

(define a value)

defines a variable called `a` and sets it to the value value.

If the variable already exists in the current module, because it has already been created by a previous `define` expression with the same name, its value is simply changed to the new value. In this case, then, the above form is completely equivalent to

([set!](07_06_r6rs_support.md) a value)

This equivalence means that `define` can be used interchangeably with `set!` to change the value of variables at the top level of the REPL or a Scheme source file. It is useful during interactive development when reloading a Scheme file that you have modified, because it allows the `define` expressions in that file to work as expected both the first time that the file is loaded and on subsequent occasions.

Note, though, that `define` and `set!` are not always equivalent. For example, a `set!` is not allowed if the named variable does not already exist, and the two expressions can behave differently in the case where there are imported variables visible from another module.

Scheme Syntax: **define** name value [¶](06_10_definitions_and_variable_bindings.md)

Create a top level variable named name with value value. If the named variable already exists, just change its value. The return value of a `define` expression is unspecified.

The C API equivalents of `define` are `scm_define` and `scm_c_define`, which differ from each other in whether the variable name is specified as a `SCM` symbol or as a null-terminated C string.

C Function: **scm\_define** (sym, value) [¶](06_10_definitions_and_variable_bindings.md)

C Function: **scm\_c\_define** (const char \*name, value) [¶](06_10_definitions_and_variable_bindings.md)

C equivalents of `define`, with variable name specified either by sym, a symbol, or by name, a null-terminated C string. Both variants return the new or preexisting variable object.

`define` (when it occurs at top level), `scm_define` and `scm_c_define` all create or set the value of a variable in the top level environment of the current module. If there was not already a variable with the specified name belonging to the current module, but a similarly named variable from another module was visible through having been imported, the newly created variable in the current module will shadow the imported variable, such that the imported variable is no longer visible.

Attention: Scheme definitions inside local binding constructs (see [Local Variable Bindings](06_10_definitions_and_variable_bindings.md#6102-local-variable-bindings)) act differently (see [Internal definitions](06_10_definitions_and_variable_bindings.md#6103-internal-definitions)).

Many people end up in a development style of adding and changing definitions at runtime, building out their program without restarting it. (You can do this using `reload-module`, the `reload` REPL command, the `load` procedure, or even just pasting code into a REPL.) If you are one of these people, you will find that sometimes there are some variables that you _don’t_ want to redefine all the time. For these, use `define-once`.

Scheme Syntax: **define-once** name value [¶](06_10_definitions_and_variable_bindings.md)

Create a top level variable named name with value value, but only if name is not already bound in the current module.

Old Lispers probably know `define-once` under its Lisp name, `defvar`.

* * *

Next: [Internal definitions](06_10_definitions_and_variable_bindings.md#6103-internal-definitions), Previous: [Top Level Variable Definitions](06_10_definitions_and_variable_bindings.md#6101-top-level-variable-definitions), Up: [Definitions and Variable Bindings](06_10_definitions_and_variable_bindings.md#610-definitions-and-variable-bindings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.10.2 Local Variable Bindings [¶](06_10_definitions_and_variable_bindings.md#6102-local-variable-bindings)

As opposed to definitions at the top level, which creates bindings that are visible to all code in a module, it is also possible to define variables which are only visible in a well-defined part of the program. Normally, this part of a program will be a procedure or a subexpression of a procedure.

With the constructs for local binding (`let`, `let*`, `letrec`, and `letrec*`), the Scheme language has a block structure like most other programming languages since the days of ALGOL 60. Readers familiar to languages like C or Java should already be used to this concept, but the family of `let` expressions has a few properties which are well worth knowing.

The most basic local binding construct is `let`.

syntax: **let** bindings body [¶](06_10_definitions_and_variable_bindings.md)

bindings has the form

((variable1 init1) [...](06_08_macros.md))

that is zero or more two-element lists of a variable and an arbitrary expression each. All variable names must be distinct.

body is a sequence of expressions and definitions, ending in an expression.

A `let` expression is evaluated as follows.

*   All init expressions are evaluated.
*   New storage is allocated for the variables.
*   The values of the init expressions are stored into the variables.
*   The expressions and definitions in body are evaluated in order (see [Internal definitions](06_10_definitions_and_variable_bindings.md#6103-internal-definitions)), and the values of the last expression are returned as the result of the `let` expression.

The init expressions are not allowed to refer to any of the variables.

The other binding constructs are variations on the same theme: making new values, binding them to variables, and executing a body in that new, extended lexical context.

syntax: **let\*** bindings body [¶](06_10_definitions_and_variable_bindings.md)

Similar to `let`, but the variable bindings are performed sequentially, that means that all init expression are allowed to use the variables defined on their left in the binding list.

A `let*` expression can always be expressed with nested `let` expressions.

(let\* ((a 1) (b a))
   b)
≍
(let ((a 1))
  (let ((b a))
    b))

syntax: **letrec** bindings body [¶](06_10_definitions_and_variable_bindings.md)

Similar to `let`, but it is possible to refer to the variable from lambda expression created in any of the inits. That is, procedures created in the init expression can recursively refer to the defined variables.

(letrec (([even?](06_06_02_numerical_data_types.md) (lambda (n)
                  (if ([zero?](06_06_02_numerical_data_types.md) n)
                      #t
                      ([odd?](06_06_02_numerical_data_types.md) ([\-](06_06_02_numerical_data_types.md) n 1)))))
         ([odd?](06_06_02_numerical_data_types.md) (lambda (n)
                  (if ([zero?](06_06_02_numerical_data_types.md) n)
                      #f
                      ([even?](06_06_02_numerical_data_types.md) ([\-](06_06_02_numerical_data_types.md) n 1))))))
  ([even?](06_06_02_numerical_data_types.md) 88))
⇒
#t

Note that while the init expressions may refer to the new variables, they may not access their values. For example, making the `even?` function above creates a closure (see [The Concept of Closure](03_hello_scheme.md#34-the-concept-of-closure)) referencing the `odd?` variable. But `odd?` can’t be called until after execution has entered the body.

syntax: **letrec\*** bindings body [¶](06_10_definitions_and_variable_bindings.md)

Similar to `letrec`, except the init expressions are bound to their variables in order.

`letrec*` thus relaxes the letrec restriction, in that later init expressions may refer to the values of previously bound variables.

(letrec ((a 42)
         (b ([+](06_06_02_numerical_data_types.md) a 10)))  ;; Illegal access
  ([\*](06_06_02_numerical_data_types.md) a b))
;; The behavior of the expression above is unspecified
([letrec\*](06_10_definitions_and_variable_bindings.md) ((a 42)
          (b ([+](06_06_02_numerical_data_types.md) a 10)))
  ([\*](06_06_02_numerical_data_types.md) a b))
⇒ 2184

There is also an alternative form of the `let` form, which is used for expressing iteration. Because of the use as a looping construct, this form (the _named let_) is documented in the section about iteration (see [Iteration](06_11_controlling_the_flow_of_program_execution.md#6114-iteration-mechanisms))

* * *

Next: [Querying variable bindings](06_10_definitions_and_variable_bindings.md#6104-querying-variable-bindings), Previous: [Local Variable Bindings](06_10_definitions_and_variable_bindings.md#6102-local-variable-bindings), Up: [Definitions and Variable Bindings](06_10_definitions_and_variable_bindings.md#610-definitions-and-variable-bindings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.10.3 Internal definitions [¶](06_10_definitions_and_variable_bindings.md#6103-internal-definitions)

A `define` form which appears inside the body of a `lambda`, `let`, `let*`, `letrec`, `letrec*` or equivalent expression is called an _internal definition_. An internal definition differs from a top level definition (see [Top Level Variable Definitions](06_10_definitions_and_variable_bindings.md#6101-top-level-variable-definitions)), because the definition is only visible inside the complete body of the enclosing form. Let us examine the following example.

(let ((frumble "froz"))
  (define banana (lambda () (apple 'peach)))
  (define apple (lambda (x) x))
  (banana))
⇒
peach

Here the enclosing form is a `let`, so the `define`s in the `let`\-body are internal definitions. Because the scope of the internal definitions is the **complete** body of the `let`\-expression, the `lambda`\-expression which gets bound to the variable `banana` may refer to the variable `apple`, even though its definition appears lexically _after_ the definition of `banana`. This is because a sequence of internal definition acts as if it were a `letrec*` expression.

(let ()
  (define a 1)
  (define b 2)
  ([+](06_06_02_numerical_data_types.md) a b))

is equivalent to

(let ()
  ([letrec\*](06_10_definitions_and_variable_bindings.md) ((a 1) (b 2))
    ([+](06_06_02_numerical_data_types.md) a b)))

Internal definitions may be mixed with non-definition expressions. If an expression precedes a definition, it is treated as if it were a definition of an unreferenced variable. So this:

(let ()
  (define a 1)
  (foo)
  (define b 2)
  ([+](06_06_02_numerical_data_types.md) a b))

is equivalent to

(let ()
  ([letrec\*](06_10_definitions_and_variable_bindings.md) ((a 1) ([\_](06_08_macros.md) (begin (foo) #f)) (b 2))
    ([+](06_06_02_numerical_data_types.md) a b)))

Another noteworthy difference to top level definitions is that within one group of internal definitions all variable names must be distinct. Whereas on the top level a second define for a given variable acts like a `set!`, for internal definitions, duplicate bound identifiers signals an error.

As a historical note, it used to be that internal bindings were expanded in terms of `letrec`, not `letrec*`. This was the situation for the R5RS report and before. However with the R6RS, it was recognized that sequential definition was a more intuitive expansion, as in the following case:

(let ()
  (define a 1)
  (define b ([+](06_06_02_numerical_data_types.md) a a))
  ([+](06_06_02_numerical_data_types.md) a b))

Guile decided to follow the R6RS in this regard, and now expands internal definitions using `letrec*`. Relatedly, it used to be that internal definitions had to precede all expressions in the body; this restriction was relaxed in Guile 3.0.

* * *

Next: [Binding multiple return values](06_10_definitions_and_variable_bindings.md#6105-binding-multiple-return-values), Previous: [Internal definitions](06_10_definitions_and_variable_bindings.md#6103-internal-definitions), Up: [Definitions and Variable Bindings](06_10_definitions_and_variable_bindings.md#610-definitions-and-variable-bindings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.10.4 Querying variable bindings [¶](06_10_definitions_and_variable_bindings.md#6104-querying-variable-bindings)

Guile provides a procedure for checking whether a symbol is bound in the top level environment.

Scheme Procedure: **defined?** sym \[module\] [¶](06_10_definitions_and_variable_bindings.md)

C Function: **scm\_defined\_p** (sym, module) [¶](06_10_definitions_and_variable_bindings.md)

Return `#t` if sym is defined in the module module or the current module when module is not specified; otherwise return `#f`.

* * *

Previous: [Querying variable bindings](06_10_definitions_and_variable_bindings.md#6104-querying-variable-bindings), Up: [Definitions and Variable Bindings](06_10_definitions_and_variable_bindings.md#610-definitions-and-variable-bindings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.10.5 Binding multiple return values [¶](06_10_definitions_and_variable_bindings.md#6105-binding-multiple-return-values)

Syntax: **define-values** formals expression [¶](06_10_definitions_and_variable_bindings.md)

The expression is evaluated, and the formals are bound to the return values in the same way that the formals in a `lambda` expression are matched to the arguments in a procedure call.

(define-values (q r) (floor/ 10 3))
(list q r) ⇒ (3 1)

(define-values (x . y) (values 1 2 3))
x ⇒ 1
y ⇒ (2 3)

(define-values x (values 1 2 3))
x ⇒ (1 2 3)

* * *

Next: [Input and Output](06_12_input_and_output.md#612-input-and-output), Previous: [Definitions and Variable Bindings](06_10_definitions_and_variable_bindings.md#610-definitions-and-variable-bindings), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
