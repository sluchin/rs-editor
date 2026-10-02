### 6.7 Procedures [¶](06_07_procedures.md#67-procedures)

*   [Lambda: Basic Procedure Creation](06_07_procedures.md#671-lambda-basic-procedure-creation)
*   [Primitive Procedures](06_07_procedures.md#672-primitive-procedures)
*   [Compiled Procedures](06_07_procedures.md#673-compiled-procedures)
*   [Optional Arguments](06_07_procedures.md#674-optional-arguments)
*   [Case-lambda](06_07_procedures.md#675-case-lambda)
*   [Higher-Order Functions](06_07_procedures.md#676-higher-order-functions)
*   [Procedure Properties and Meta-information](06_07_procedures.md#677-procedure-properties-and-meta-information)
*   [Procedures with Setters](06_07_procedures.md#678-procedures-with-setters)
*   [Inlinable Procedures](06_07_procedures.md#679-inlinable-procedures)

* * *

Next: [Primitive Procedures](06_07_procedures.md#672-primitive-procedures), Up: [Procedures](06_07_procedures.md#67-procedures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.1 Lambda: Basic Procedure Creation [¶](06_07_procedures.md#671-lambda-basic-procedure-creation)

A `lambda` expression evaluates to a procedure. The environment which is in effect when a `lambda` expression is evaluated is enclosed in the newly created procedure, this is referred to as a _closure_ (see [The Concept of Closure](03_hello_scheme.md#34-the-concept-of-closure)).

When a procedure created by `lambda` is called with some actual arguments, the environment enclosed in the procedure is extended by binding the variables named in the formal argument list to new locations and storing the actual arguments into these locations. Then the body of the `lambda` expression is evaluated sequentially. The result of the last expression in the procedure body is then the result of the procedure invocation.

The following examples will show how procedures can be created using `lambda`, and what you can do with these procedures.

(lambda (x) ([+](06_06_02_numerical_data_types.md) x x))       ⇒ a [procedure](06_07_procedures.md)
((lambda (x) ([+](06_06_02_numerical_data_types.md) x x)) 4)   ⇒ 8

The fact that the environment in effect when creating a procedure is enclosed in the procedure is shown with this example:

(define add4
  (let ((x 4))
    (lambda (y) ([+](06_06_02_numerical_data_types.md) x y))))
(add4 6)                   ⇒ 10

syntax: **lambda** formals body [¶](06_07_procedures.md)

formals should be a formal argument list as described in the following table.

`(variable1 …)`

The procedure takes a fixed number of arguments; when the procedure is called, the arguments will be stored into the newly created location for the formal variables.

`variable`

The procedure takes any number of arguments; when the procedure is called, the sequence of actual arguments will be converted into a list and stored into the newly created location for the formal variable.

`(variable1 … variablen . variablen+1)`

If a space-delimited period precedes the last variable, then the procedure takes n or more variables where n is the number of formal arguments before the period. There must be at least one argument before the period. The first n actual arguments will be stored into the newly allocated locations for the first n formal arguments and the sequence of the remaining actual arguments is converted into a list and the stored into the location for the last formal argument. If there are exactly n actual arguments, the empty list is stored into the location of the last formal argument.

The list in variable or variablen+1 is always newly created and the procedure can modify it if desired. This is the case even when the procedure is invoked via `apply`, the required part of the list argument there will be copied (see [Procedures for On the Fly Evaluation](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation)).

body is a sequence of Scheme expressions which are evaluated in order when the procedure is invoked.

* * *

Next: [Compiled Procedures](06_07_procedures.md#673-compiled-procedures), Previous: [Lambda: Basic Procedure Creation](06_07_procedures.md#671-lambda-basic-procedure-creation), Up: [Procedures](06_07_procedures.md#67-procedures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.2 Primitive Procedures [¶](06_07_procedures.md#672-primitive-procedures)

Procedures written in C can be registered for use from Scheme, provided they take only arguments of type `SCM` and return `SCM` values. `scm_c_define_gsubr` is likely to be the most useful mechanism, combining the process of registration (`scm_c_make_gsubr`) and definition (`scm_define`).

Function: `SCM` **scm\_c\_make\_gsubr** `(const char *name, int req, int opt, int rst, fcn)` [¶](06_07_procedures.md)

Register a C procedure fcn as a “subr” — a primitive subroutine that can be called from Scheme. It will be associated with the given name but no environment binding will be created. The arguments req, opt and rst specify the number of required, optional and “rest” arguments respectively. The total number of these arguments should match the actual number of arguments to fcn, but may not exceed 10. The number of rest arguments should be 0 or 1. `scm_c_make_gsubr` returns a value of type `SCM` which is a “handle” for the procedure.

Function: `SCM` **scm\_c\_define\_gsubr** `(const char *name, int req, int opt, int rst, fcn)` [¶](06_07_procedures.md)

Register a C procedure fcn, as for `scm_c_make_gsubr` above, and additionally create a top-level Scheme binding for the procedure in the “current environment” using `scm_define`. `scm_c_define_gsubr` returns a handle for the procedure in the same way as `scm_c_make_gsubr`, which is usually not further required.

See [Foreign Functions](06_19_foreign_function_interface.md#6195-foreign-functions), for another interface to call procedures written in C from Scheme.

* * *

Next: [Optional Arguments](06_07_procedures.md#674-optional-arguments), Previous: [Primitive Procedures](06_07_procedures.md#672-primitive-procedures), Up: [Procedures](06_07_procedures.md#67-procedures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.3 Compiled Procedures [¶](06_07_procedures.md#673-compiled-procedures)

The evaluation strategy given in [Lambda: Basic Procedure Creation](06_07_procedures.md#671-lambda-basic-procedure-creation) describes how procedures are _interpreted_. Interpretation operates directly on expanded Scheme source code, recursively calling the evaluator to obtain the value of nested expressions.

Most procedures are compiled, however. This means that Guile has done some pre-computation on the procedure, to determine what it will need to do each time the procedure runs. Compiled procedures run faster than interpreted procedures.

Loading files is the normal way that compiled procedures come to being. If Guile sees that a file is uncompiled, or that its compiled file is out of date, it will attempt to compile the file when it is loaded, and save the result to disk. Procedures can be compiled at runtime as well. See [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code), for more information on runtime compilation.

Compiled procedures, also known as _programs_, respond to all procedures that operate on procedures: you can pass a program to `procedure?`, `procedure-name`, and so on (see [Procedure Properties and Meta-information](06_07_procedures.md#677-procedure-properties-and-meta-information)). In addition, there are a few more accessors for low-level details on programs.

Most people won’t need to use the routines described in this section, but it’s good to have them documented. You’ll have to include the appropriate module first, though:

(use-modules (system vm program))

Scheme Procedure: **program?** obj [¶](06_07_procedures.md)

C Function: **scm\_program\_p** (obj) [¶](06_07_procedures.md)

Returns `#t` if obj is a compiled procedure, or `#f` otherwise.

Scheme Procedure: **program-code** program [¶](06_07_procedures.md)

C Function: **scm\_program\_code** (program) [¶](06_07_procedures.md)

Returns the address of the program’s entry, as an integer. This address is mostly useful to procedures in `(system vm debug)`.

Scheme Procedure: **program-num-free-variable** program [¶](06_07_procedures.md)

C Function: **scm\_program\_num\_free\_variables** (program) [¶](06_07_procedures.md)

Return the number of free variables captured by this program.

Scheme Procedure: **program-free-variable-ref** program n [¶](06_07_procedures.md)

C Function: **scm\_program\_free\_variable-ref** (program, n) [¶](06_07_procedures.md)

Scheme Procedure: **program-free-variable-set!** program n val [¶](06_07_procedures.md)

C Function: **scm\_program\_free\_variable\_set\_x** (program, n, val) [¶](06_07_procedures.md)

Accessors for a program’s free variables. Some of the values captured are actually in variable “boxes”. See [Variables and the VM](09_03_a_virtual_machine_for_guile.md#934-variables-and-the-vm), for more information.

Users must not modify the returned value unless they think they’re really clever.

Scheme Procedure: **program-sources** program [¶](06_07_procedures.md)

Scheme Procedure: **source:addr** source [¶](06_07_procedures.md)

Scheme Procedure: **source:line** source [¶](06_07_procedures.md)

Scheme Procedure: **source:column** source [¶](06_07_procedures.md)

Scheme Procedure: **source:file** source [¶](06_07_procedures.md)

Source location annotations for programs, along with their accessors.

Source location information propagates through the compiler and ends up being serialized to the program’s metadata. This information is keyed by the offset of the instruction pointer within the object code of the program. Specifically, it is keyed on the `ip` _just following_ an instruction, so that backtraces can find the source location of a call that is in progress.

Scheme Procedure: **program-arities** program [¶](06_07_procedures.md)

C Function: **scm\_program\_arities** (program) [¶](06_07_procedures.md)

Scheme Procedure: **program-arity** program ip [¶](06_07_procedures.md)

Scheme Procedure: **arity:start** arity [¶](06_07_procedures.md)

Scheme Procedure: **arity:end** arity [¶](06_07_procedures.md)

Scheme Procedure: **arity:nreq** arity [¶](06_07_procedures.md)

Scheme Procedure: **arity:nopt** arity [¶](06_07_procedures.md)

Scheme Procedure: **arity:rest?** arity [¶](06_07_procedures.md)

Scheme Procedure: **arity:kw** arity [¶](06_07_procedures.md)

Scheme Procedure: **arity:allow-other-keys?** arity [¶](06_07_procedures.md)

Accessors for a representation of the “arity” of a program.

The normal case is that a procedure has one arity. For example, `(lambda (x) x)`, takes one required argument, and that’s it. One could access that number of required arguments via `(arity:nreq (program-arities (lambda (x) x)))`. Similarly, `arity:nopt` gets the number of optional arguments, and `arity:rest?` returns a true value if the procedure has a rest arg.

`arity:kw` returns a list of `(kw . idx)` pairs, if the procedure has keyword arguments. The idx refers to the idxth local variable; See [Variables and the VM](09_03_a_virtual_machine_for_guile.md#934-variables-and-the-vm), for more information. Finally `arity:allow-other-keys?` returns a true value if other keys are allowed. See [Optional Arguments](06_07_procedures.md#674-optional-arguments), for more information.

So what about `arity:start` and `arity:end`, then? They return the range of bytes in the program’s bytecode for which a given arity is valid. You see, a procedure can actually have more than one arity. The question, “what is a procedure’s arity” only really makes sense at certain points in the program, delimited by these `arity:start` and `arity:end` values.

Scheme Procedure: **program-arguments-alist** program \[ip\] [¶](06_07_procedures.md)

Return an association list describing the arguments that program accepts, or `#f` if the information cannot be obtained.

The alist keys that are currently defined are ‘required’, ‘optional’, ‘keyword’, ‘allow-other-keys?’, and ‘rest’. For example:

(program-arguments-alist
 (lambda\* (a b #:optional c #:key (d 1) #:rest e)
   #t)) ⇒
((required . (a b))
 (optional . (c))
 (keyword . ((#:d . 4)))
 (allow-other-keys? . #f)
 (rest . d))

Scheme Procedure: **program-lambda-list** program \[ip\] [¶](06_07_procedures.md)

Return a representation of the arguments of program as a lambda list, or `#f` if this information is not available.

For example:

(program-lambda-list
 (lambda\* (a b #:optional c #:key (d 1) #:rest e)
   #t)) ⇒

* * *

Next: [Case-lambda](06_07_procedures.md#675-case-lambda), Previous: [Compiled Procedures](06_07_procedures.md#673-compiled-procedures), Up: [Procedures](06_07_procedures.md#67-procedures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.4 Optional Arguments [¶](06_07_procedures.md#674-optional-arguments)

Scheme procedures, as defined in R5RS, can either handle a fixed number of actual arguments, or a fixed number of actual arguments followed by arbitrarily many additional arguments. Writing procedures of variable arity can be useful, but unfortunately, the syntactic means for handling argument lists of varying length is a bit inconvenient. It is possible to give names to the fixed number of arguments, but the remaining (optional) arguments can be only referenced as a list of values (see [Lambda: Basic Procedure Creation](06_07_procedures.md#671-lambda-basic-procedure-creation)).

For this reason, Guile provides an extension to `lambda`, `lambda*`, which allows the user to define procedures with optional and keyword arguments. In addition, Guile’s virtual machine has low-level support for optional and keyword argument dispatch. Calls to procedures with optional and keyword arguments can be made cheaply, without allocating a rest list.

*   [lambda\* and define\*.](06_07_procedures.md#6741-lambda-and-define)
*   [(ice-9 optargs)](06_07_procedures.md#6742-ice-9-optargs)

* * *

Next: [(ice-9 optargs)](06_07_procedures.md#6742-ice-9-optargs), Up: [Optional Arguments](06_07_procedures.md#674-optional-arguments)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.4.1 lambda\* and define\*. [¶](06_07_procedures.md#6741-lambda-and-define)

`lambda*` is like `lambda`, except with some extensions to allow optional and keyword arguments.

library syntax: **lambda\*** (\[var…\]  
\[#:optional vardef…\]  
\[#:key vardef… \[#:allow-other-keys\]\]  
\[#:rest var | . var\])  
body1 body2 … [¶](06_07_procedures.md)

  

Create a procedure which takes optional and/or keyword arguments specified with `#:optional` and `#:key`. For example,

([lambda\*](06_07_procedures.md) (a b #:optional c d . e) '())

is a procedure with fixed arguments a and b, optional arguments c and d, and rest argument e. If the optional arguments are omitted in a call, the variables for them are bound to `#f`.

Likewise, `define*` is syntactic sugar for defining procedures using `lambda*`.

`lambda*` can also make procedures with keyword arguments. For example, a procedure defined like this:

(define\* (sir-yes-sir #:key action how-high)
  ([list](06_06_09_lists.md) action how-high))

can be called as `(sir-yes-sir #:action 'jump)`, `(sir-yes-sir #:how-high 13)`, `(sir-yes-sir #:action 'lay-down #:how-high 0)`, or just `(sir-yes-sir)`. Whichever arguments are given as keywords are bound to values (and those not given are `#f`).

Optional and keyword arguments can also have default values to take when not present in a call, by giving a two-element list of variable name and expression. For example in

(define\* (frob foo #:optional (bar 42) #:key (baz 73))
  ([list](06_06_09_lists.md) foo bar baz))

foo is a fixed argument, bar is an optional argument with default value 42, and baz is a keyword argument with default value 73. Default value expressions are not evaluated unless they are needed, and until the procedure is called.

Normally it’s an error if a call has keywords other than those specified by `#:key`, but adding `#:allow-other-keys` to the definition (after the keyword argument declarations) will ignore unknown keywords.

If a call has a keyword given twice, the last value is used. For example,

(define\* (flips #:key (heads 0) (tails 0))
  ([display](06_16_reading_and_evaluating_scheme_code.md) ([list](06_06_09_lists.md) heads tails)))

(flips #:heads 37 #:tails 42 #:heads 99)
⊣ (99 42)

`#:rest` is a synonym for the dotted syntax rest argument. The argument lists `(a . b)` and `(a #:rest b)` are equivalent in all respects. This is provided for more similarity to DSSSL, MIT-Scheme and Kawa among others, as well as for refugees from other Lisp dialects.

When `#:key` is used together with a rest argument, the keyword parameters in a call all remain in the rest list. This is the same as Common Lisp. For example,

(([lambda\*](06_07_procedures.md) (#:key (x 0) #:allow-other-keys #:rest r)
   ([display](06_16_reading_and_evaluating_scheme_code.md) r))
 #:x 123 #:y 456)
⊣ (#:x 123 #:y 456)

`#:optional` and `#:key` establish their bindings successively, from left to right. This means default expressions can refer back to prior parameters, for example

([lambda\*](06_07_procedures.md) (start #:optional (end ([+](06_06_02_numerical_data_types.md) 10 start)))
  (do ((i start ([1+](06_06_02_numerical_data_types.md) i)))
      (([\>](06_06_02_numerical_data_types.md) i end))
    ([display](06_16_reading_and_evaluating_scheme_code.md) i)))

The exception to this left-to-right scoping rule is the rest argument. If there is a rest argument, it is bound after the optional arguments, but before the keyword arguments.

* * *

Previous: [lambda\* and define\*.](06_07_procedures.md#6741-lambda-and-define), Up: [Optional Arguments](06_07_procedures.md#674-optional-arguments)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.4.2 (ice-9 optargs) [¶](06_07_procedures.md#6742-ice-9-optargs)

Before Guile 2.0, `lambda*` and `define*` were implemented using macros that processed rest list arguments. This was not optimal, as calling procedures with optional arguments had to allocate rest lists at every procedure invocation. Guile 2.0 improved this situation by bringing optional and keyword arguments into Guile’s core.

However there are occasions in which you have a list and want to parse it for optional or keyword arguments. Guile’s `(ice-9 optargs)` provides some macros to help with that task.

The syntax `let-optional` and `let-optional*` are for destructuring rest argument lists and giving names to the various list elements. `let-optional` binds all variables simultaneously, while `let-optional*` binds them sequentially, consistent with `let` and `let*` (see [Local Variable Bindings](06_10_definitions_and_variable_bindings.md#6102-local-variable-bindings)).

library syntax: **let-optional** rest-arg (binding …) body1 body2 … [¶](06_07_procedures.md)

library syntax: **let-optional\*** rest-arg (binding …) body1 body2 … [¶](06_07_procedures.md)

These two macros give you an optional argument interface that is very _Schemey_ and introduces no fancy syntax. They are compatible with the scsh macros of the same name, but are slightly extended. Each of binding may be of one of the forms var or `(var default-value)`. rest-arg should be the rest-argument of the procedures these are used from. The items in rest-arg are sequentially bound to the variable names are given. When rest-arg runs out, the remaining vars are bound either to the default values or `#f` if no default value was specified. rest-arg remains bound to whatever may have been left of rest-arg.

After binding the variables, the expressions body1 body2 … are evaluated in order.

Similarly, `let-keywords` and `let-keywords*` extract values from keyword style argument lists, binding local variables to those values or to defaults.

library syntax: **let-keywords** args allow-other-keys? (binding …) body1 body2 … [¶](06_07_procedures.md)

library syntax: **let-keywords\*** args allow-other-keys? (binding …) body1 body2 … [¶](06_07_procedures.md)

args is evaluated and should give a list of the form `(#:keyword1 value1 #:keyword2 value2 …)`. The bindings are variables and default expressions, with the variables to be set (by name) from the keyword values. The body1 body2 … forms are then evaluated and the last is the result. An example will make the syntax clearest,

(define args '(#:xyzzy "hello" #:foo "world"))

(let-keywords args #t
      ((foo  "default for foo")
       (bar  (string-append "default" "for" "bar")))
  (display foo)
  (display ", ")
  (display bar))
⊣ world, defaultforbar

The binding for `foo` comes from the `#:foo` keyword in `args`. But the binding for `bar` is the default in the `let-keywords`, since there’s no `#:bar` in the args.

allow-other-keys? is evaluated and controls whether unknown keywords are allowed in the args list. When true other keys are ignored (such as `#:xyzzy` in the example), when `#f` an error is thrown for anything unknown.

`(ice-9 optargs)` also provides some more `define*` sugar, which is not so useful with modern Guile coding, but still supported: `define*-public` is the `lambda*` version of `define-public`; `defmacro*` and `defmacro*-public` exist for defining macros with the improved argument list handling possibilities. The `-public` versions not only define the procedures/macros, but also export them from the current module.

library syntax: **define\*-public** formals body1 body2 … [¶](06_07_procedures.md)

Like a mix of `define*` and `define-public`.

library syntax: **defmacro\*** name formals body1 body2 … [¶](06_07_procedures.md)

library syntax: **defmacro\*-public** name formals body1 body2 … [¶](06_07_procedures.md)

These are just like `defmacro` and `defmacro-public` except that they take `lambda*`\-style extended parameter lists, where `#:optional`, `#:key`, `#:allow-other-keys` and `#:rest` are allowed with the usual semantics. Here is an example of a macro with an optional argument:

([defmacro\*](06_07_procedures.md) transmogrify (a #:optional b)
  (a 1))

* * *

Next: [Higher-Order Functions](06_07_procedures.md#676-higher-order-functions), Previous: [Optional Arguments](06_07_procedures.md#674-optional-arguments), Up: [Procedures](06_07_procedures.md#67-procedures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.5 Case-lambda [¶](06_07_procedures.md#675-case-lambda)

R5RS’s rest arguments are indeed useful and very general, but they often aren’t the most appropriate or efficient means to get the job done. For example, `lambda*` is a much better solution to the optional argument problem than `lambda` with rest arguments.

Likewise, `case-lambda` works well for when you want one procedure to do double duty (or triple, or ...), without the penalty of consing a rest list.

For example:

(define (make-accum n)
  ([case-lambda](07_06_r6rs_support.md)
    (() n)
    ((m) ([set!](07_06_r6rs_support.md) n ([+](06_06_02_numerical_data_types.md) n m)) n)))

(define a (make-accum 20))
(a) ⇒ 20
(a 10) ⇒ 30
(a) ⇒ 30

The value returned by a `case-lambda` form is a procedure which matches the number of actual arguments against the formals in the various clauses, in order. The first matching clause is selected, the corresponding values from the actual parameter list are bound to the variable names in the clauses and the body of the clause is evaluated. If no clause matches, an error is signaled.

The syntax of the `case-lambda` form is defined in the following EBNF grammar. _Formals_ means a formal argument list just like with `lambda` (see [Lambda: Basic Procedure Creation](06_07_procedures.md#671-lambda-basic-procedure-creation)).

<case-lambda>
   --> (case-lambda <case-lambda-clause>\*)
   --> (case-lambda <docstring> <case-lambda-clause>\*)
<case-lambda-clause>
   --> (<formals> <definition-or-command>\*)
<formals>
   --> (<identifier>\*)
     | (<identifier>\* . <identifier>)
     | <identifier>

Rest lists can be useful with `case-lambda`:

(define plus
  ([case-lambda](07_06_r6rs_support.md)
    "Return the sum of all arguments."
    (() 0)
    ((a) a)
    ((a b) ([+](06_06_02_numerical_data_types.md) a b))
    ((a b . rest) ([apply](06_16_reading_and_evaluating_scheme_code.md) plus ([+](06_06_02_numerical_data_types.md) a b) rest))))
(plus 1 2 3) ⇒ 6

Also, for completeness. Guile defines `case-lambda*` as well, which is like `case-lambda`, except with `lambda*` clauses. A `case-lambda*` clause matches if the arguments fill the required arguments, but are not too many for the optional and/or rest arguments.

Keyword arguments are possible with `case-lambda*` as well, but they do not contribute to the “matching” behavior, and their interactions with required, optional, and rest arguments can be surprising.

For the purposes of `case-lambda*` (and of `case-lambda`, as a special case), a clause _matches_ if it has enough required arguments, and not too many positional arguments. The required arguments are any arguments before the `#:optional`, `#:key`, and `#:rest` arguments. _Positional_ arguments are the required arguments, together with the optional arguments.

In the absence of `#:key` or `#:rest` arguments, it’s easy to see how there could be too many positional arguments: you pass 5 arguments to a function that only takes 4 arguments, including optional arguments. If there is a `#:rest` argument, there can never be too many positional arguments: any application with enough required arguments for a clause will match that clause, even if there are also `#:key` arguments.

Otherwise, for applications to a clause with `#:key` arguments (and without a `#:rest` argument), a clause will match there only if there are enough required arguments and if the next argument after binding required and optional arguments, if any, is a keyword. For efficiency reasons, Guile is currently unable to include keyword arguments in the matching algorithm. Clauses match on positional arguments only, not by comparing a given keyword to the available set of keyword arguments that a function has.

Some examples follow.

(define f
  (case-lambda\*
    ((a #:optional b) 'clause-1)
    ((a #:optional b #:key c) 'clause-2)
    ((a #:key d) 'clause-3)
    ((#:key e #:rest f) 'clause-4)))

(f) ⇒ clause-4
(f 1) ⇒ clause-1
(f) ⇒ clause-4
(f #:e 10) clause-1
(f 1 #:foo) clause-1
(f 1 #:c 2) clause-2
(f #:a #:b #:c #:d #:e) clause-4

;; clause-2 will match anything that clause-3 would match.
(f 1 #:d 2) ⇒ error: bad keyword args in clause 2

Don’t forget that the clauses are matched in order, and the first matching clause will be taken. This can result in a keyword being bound to a required argument, as in the case of `f #:e 10`.

* * *

Next: [Procedure Properties and Meta-information](06_07_procedures.md#677-procedure-properties-and-meta-information), Previous: [Case-lambda](06_07_procedures.md#675-case-lambda), Up: [Procedures](06_07_procedures.md#67-procedures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.6 Higher-Order Functions [¶](06_07_procedures.md#676-higher-order-functions)

As a functional programming language, Scheme allows the definition of _higher-order functions_, i.e., functions that take functions as arguments and/or return functions. Utilities to derive procedures from other procedures are provided and described below.

Scheme Procedure: **const** value [¶](06_07_procedures.md)

Return a procedure that accepts any number of arguments and returns value.

([procedure?](06_07_procedures.md) ([const](06_07_procedures.md) 3))        ⇒ #t
(([const](06_07_procedures.md) 'hello))              ⇒ hello
(([const](06_07_procedures.md) 'hello) 'world)       ⇒ hello

Scheme Procedure: **negate** proc [¶](06_07_procedures.md)

Return a procedure with the same arity as proc that returns the `not` of proc’s result.

([procedure?](06_07_procedures.md) ([negate](06_07_procedures.md) [number?](06_06_02_numerical_data_types.md))) ⇒ #t
(([negate](06_07_procedures.md) [odd?](06_06_02_numerical_data_types.md)) 2)             ⇒ #t
(([negate](06_07_procedures.md) [real?](06_06_02_numerical_data_types.md)) 'dream)       ⇒ #t
(([negate](06_07_procedures.md) [string-prefix?](06_06_05_strings.md)) "GNU" "GNU Guile")
                              ⇒ #f
([filter](06_06_09_lists.md) ([negate](06_07_procedures.md) [number?](06_06_02_numerical_data_types.md)) '(a 2 "b"))
                              ⇒ (a "b")

Scheme Procedure: **compose** proc1 proc2 … [¶](06_07_procedures.md)

Compose proc1 with the procedures proc2 … such that the last proc argument is applied first and proc1 last, and return the resulting procedure. The given procedures must have compatible arity.

([procedure?](06_07_procedures.md) ([compose](06_07_procedures.md) [1+](06_06_02_numerical_data_types.md) [1-](06_06_02_numerical_data_types.md))) ⇒ #t
(([compose](06_07_procedures.md) [sqrt](06_06_02_numerical_data_types.md) [1+](06_06_02_numerical_data_types.md) [1+](06_06_02_numerical_data_types.md)) 2)     ⇒ 2.0
(([compose](06_07_procedures.md) [1+](06_06_02_numerical_data_types.md) [sqrt](06_06_02_numerical_data_types.md)) 3)        ⇒ 2.73205080756888
([eq?](06_09_general_utility_functions.md) ([compose](06_07_procedures.md) [1+](06_06_02_numerical_data_types.md)) [1+](06_06_02_numerical_data_types.md))        ⇒ #t

(([compose](06_07_procedures.md) [zip](07_05_03_srfi1_list_library.md) [unzip2](07_05_03_srfi1_list_library.md)) '((1 2) (a b)))
                             ⇒ ((1 2) (a b))

Scheme Procedure: **identity** x [¶](06_07_procedures.md)

Return X.

Scheme Procedure: **and=>** value proc [¶](06_07_procedures.md)

When value is `#f`, return `#f`. Otherwise, return `(proc value)`.

* * *

Next: [Procedures with Setters](06_07_procedures.md#678-procedures-with-setters), Previous: [Higher-Order Functions](06_07_procedures.md#676-higher-order-functions), Up: [Procedures](06_07_procedures.md#67-procedures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.7 Procedure Properties and Meta-information [¶](06_07_procedures.md#677-procedure-properties-and-meta-information)

In addition to the information that is strictly necessary to run, procedures may have other associated information. For example, the name of a procedure is information not for the procedure, but about the procedure. This meta-information can be accessed via the procedure properties interface.

Scheme Procedure: **procedure?** obj [¶](06_07_procedures.md)

C Function: **scm\_procedure\_p** (obj) [¶](06_07_procedures.md)

Return `#t` if obj is a procedure.

Scheme Procedure: **thunk?** obj [¶](06_07_procedures.md)

C Function: **scm\_thunk\_p** (obj) [¶](06_07_procedures.md)

Return `#t` if obj is a procedure that can be called with zero arguments. See [Compiled Procedures](06_07_procedures.md#673-compiled-procedures), to get more information on what arguments a procedure will accept.

Procedure properties are general properties associated with procedures. These can be the name of a procedure or other relevant information, such as debug hints.

The most general way to associate a property of a procedure is programmatically:

Scheme Procedure: **procedure-property** proc key [¶](06_07_procedures.md)

C Function: **scm\_procedure\_property** (proc, key) [¶](06_07_procedures.md)

Return the property of proc with name key, or `#f` if not found.

Scheme Procedure: **set-procedure-property!** proc key value [¶](06_07_procedures.md)

C Function: **scm\_set\_procedure\_property\_x** (proc, key, value) [¶](06_07_procedures.md)

Set proc’s property named key to value.

However, there is a more efficient interface that allows constant properties to be embedded into compiled binaries in a way that does not incur any overhead until someone asks for the property: initial non-tail elements of the body of a lambda expression that are literal vectors of pairs are interpreted as declaring procedure properties. This is easiest to see with an example:

(define proc
  (lambda args
    #((a . "hey") (b . "ho")) ;; procedure properties!
    42))
(procedure-property proc 'a) ; ⇒ "hey"
(procedure-property proc 'b) ; ⇒ "ho"

There is a shorthand for declaring the `documentation` property, which is a literal string instead of a literal vector:

(define proc
  (lambda args
    "This is a docstring."
    42))
(procedure-property proc 'documentation)
;; ⇒ "This is a docstring."

Calling `procedure-property` with a key of `documentation` is exactly the same as calling `procedure-documentation`. Similarly, `procedure-name` is the same as the `name` procedure property, and `procedure-source` is for the `source` property.

Scheme Procedure: **procedure-name** proc [¶](06_07_procedures.md)

C Function: **scm\_procedure\_name** (proc) [¶](06_07_procedures.md)

Scheme Procedure: **procedure-source** proc [¶](06_07_procedures.md)

C Function: **scm\_procedure\_source** (proc) [¶](06_07_procedures.md)

Scheme Procedure: **procedure-documentation** proc [¶](06_07_procedures.md)

C Function: **scm\_procedure\_documentation** (proc) [¶](06_07_procedures.md)

Return the value of the `name`, `source`, or `documentation` property for proc, or `#f` if no property is set.

One can also work on the entire set of procedure properties.

Scheme Procedure: **procedure-properties** proc [¶](06_07_procedures.md)

C Function: **scm\_procedure\_properties** (proc) [¶](06_07_procedures.md)

Return the properties associated with proc, as an association list.

Scheme Procedure: **set-procedure-properties!** proc alist [¶](06_07_procedures.md)

C Function: **scm\_set\_procedure\_properties\_x** (proc, alist) [¶](06_07_procedures.md)

Set proc’s property list to alist.

* * *

Next: [Inlinable Procedures](06_07_procedures.md#679-inlinable-procedures), Previous: [Procedure Properties and Meta-information](06_07_procedures.md#677-procedure-properties-and-meta-information), Up: [Procedures](06_07_procedures.md#67-procedures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.8 Procedures with Setters [¶](06_07_procedures.md#678-procedures-with-setters)

A _procedure with setter_ is a special kind of procedure which normally behaves like any accessor procedure, that is a procedure which accesses a data structure. The difference is that this kind of procedure has a so-called _setter_ attached, which is a procedure for storing something into a data structure.

Procedures with setters are treated specially when the procedure appears in the special form `set!`. How it works is best shown by example.

Suppose we have a procedure called `foo-ref`, which accepts two arguments, a value of type `foo` and an integer. The procedure returns the value stored at the given index in the `foo` object. Let `f` be a variable containing such a `foo` data structure.[13](99_footnotes.md)

(foo-ref f 0)       ⇒ bar
(foo-ref f 1)       ⇒ braz

Also suppose that a corresponding setter procedure called `foo-set!` does exist.

(foo-set! f 0 'bla)
(foo-ref f 0)       ⇒ bla

Now we could create a new procedure called `foo`, which is a procedure with setter, by calling `make-procedure-with-setter` with the accessor and setter procedures `foo-ref` and `foo-set!`. Let us call this new procedure `foo`.

(define foo ([make-procedure-with-setter](06_07_procedures.md) foo-ref foo-set!))

`foo` can from now on be used to either read from the data structure stored in `f`, or to write into the structure.

([set!](07_06_r6rs_support.md) (foo f 0) 'dum)
(foo f 0)          ⇒ dum

Scheme Procedure: **make-procedure-with-setter** procedure setter [¶](06_07_procedures.md)

C Function: **scm\_make\_procedure\_with\_setter** (procedure, setter) [¶](06_07_procedures.md)

Create a new procedure which behaves like procedure, but with the associated setter setter.

Scheme Procedure: **procedure-with-setter?** obj [¶](06_07_procedures.md)

C Function: **scm\_procedure\_with\_setter\_p** (obj) [¶](06_07_procedures.md)

Return `#t` if obj is a procedure with an associated setter procedure.

Scheme Procedure: **procedure** proc [¶](06_07_procedures.md)

C Function: **scm\_procedure** (proc) [¶](06_07_procedures.md)

Return the procedure of proc, which must be an applicable struct.

Scheme Procedure: **setter** proc [¶](06_07_procedures.md)

Return the setter of proc, which must be either a procedure with setter or an operator struct.

* * *

Previous: [Procedures with Setters](06_07_procedures.md#678-procedures-with-setters), Up: [Procedures](06_07_procedures.md#67-procedures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.7.9 Inlinable Procedures [¶](06_07_procedures.md#679-inlinable-procedures)

You can define an _inlinable procedure_ by using `define-inlinable` instead of `define`. An inlinable procedure behaves the same as a regular procedure, but direct calls will result in the procedure body being inlined into the caller.

Bear in mind that starting from version 2.0.3, Guile has a partial evaluator that can inline the body of inner procedures when deemed appropriate:

scheme@(guile-user)> ,optimize (define (foo x)
                                 (define (bar) (+ x 3))
                                 (\* (bar) 2))
$1 = (define foo
       (lambda (#{x 94}#) (\* (+ #{x 94}# 3) 2)))

The partial evaluator does not inline top-level bindings, though, so this is a situation where you may find it interesting to use `define-inlinable`.

Procedures defined with `define-inlinable` are _always_ inlined, at all direct call sites. This eliminates function call overhead at the expense of an increase in code size. Additionally, the caller will not transparently use the new definition if the inline procedure is redefined. It is not possible to trace an inlined procedures or install a breakpoint in it (see [Traps](06_26_debugging_infrastructure.md#6265-traps)). For these reasons, you should not make a procedure inlinable unless it demonstrably improves performance in a crucial way.

In general, only small procedures should be considered for inlining, as making large procedures inlinable will probably result in an increase in code size. Additionally, the elimination of the call overhead rarely matters for large procedures.

Scheme Syntax: **define-inlinable** (name parameter …) body1 body2 … [¶](06_07_procedures.md)

Define name as a procedure with parameters parameters and bodies body1, body2, ....

* * *

Next: [General Utility Functions](06_09_general_utility_functions.md#69-general-utility-functions), Previous: [Procedures](06_07_procedures.md#67-procedures), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
