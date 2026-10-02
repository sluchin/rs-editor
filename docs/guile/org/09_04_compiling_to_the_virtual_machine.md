### 9.4 Compiling to the Virtual Machine [¶](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine)

Compilers! The word itself inspires excitement and awe, even among experienced practitioners. But a compiler is just a program: an eminently hackable thing. This section aims to describe Guile’s compiler in such a way that interested Scheme hackers can feel comfortable reading and extending it.

See [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code), if you’re lost and you just wanted to know how to compile your `.scm` file.

*   [Compiler Tower](09_04_compiling_to_the_virtual_machine.md#941-compiler-tower)
*   [The Scheme Compiler](09_04_compiling_to_the_virtual_machine.md#942-the-scheme-compiler)
*   [Tree-IL](09_04_compiling_to_the_virtual_machine.md#943-tree-il)
*   [Continuation-Passing Style](09_04_compiling_to_the_virtual_machine.md#944-continuation-passing-style)
*   [Bytecode](09_04_compiling_to_the_virtual_machine.md#945-bytecode)
*   [Writing New High-Level Languages](09_04_compiling_to_the_virtual_machine.md#946-writing-new-high-level-languages)
*   [Extending the Compiler](09_04_compiling_to_the_virtual_machine.md#947-extending-the-compiler)

* * *

Next: [The Scheme Compiler](09_04_compiling_to_the_virtual_machine.md#942-the-scheme-compiler), Up: [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.1 Compiler Tower [¶](09_04_compiling_to_the_virtual_machine.md#941-compiler-tower)

Guile’s compiler is quite simple – its _compilers_, to put it more accurately. Guile defines a tower of languages, starting at Scheme and progressively simplifying down to languages that resemble the VM instruction set (see [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set)).

Each language knows how to compile to the next, so each step is simple and understandable. Furthermore, this set of languages is not hardcoded into Guile, so it is possible for the user to add new high-level languages, new passes, or even different compilation targets.

Languages are registered in the module, `(system base language)`:

(use-modules (system base language))

They are registered with the `define-language` form.

Scheme Syntax: **define-language** \[#:name\] \[#:title\] \[#:reader\] \[#:printer\] \[#:parser=#f\] \[#:compilers=’()\] \[#:decompilers=’()\] \[#:evaluator=#f\] \[#:joiner=#f\] \[#:for-humans?=#t\] \[#:make-default-environment=make-fresh-user-module\] \[#:lowerer=#f\] \[#:analyzer=#f\] \[#:compiler-chooser=#f\] [¶](09_04_compiling_to_the_virtual_machine.md)

Define a language.

This syntax defines a `<language>` object, bound to name in the current environment. In addition, the language will be added to the global language set. For example, this is the language definition for Scheme:

(define-language scheme
  #:title	"Scheme"
  #:reader      (lambda (port env) ...)
  #:compilers   \`((tree-il . ,compile-tree-il))
  #:decompilers \`((tree-il . ,decompile-tree-il))
  #:evaluator	(lambda (x module) (primitive-eval x))
  #:printer	write
  #:make-default-environment (lambda () ...))

The interesting thing about having languages defined this way is that they present a uniform interface to the read-eval-print loop. This allows the user to change the current language of the REPL:

scheme@(guile-user)> ,language tree-il
Happy hacking with Tree Intermediate Language!  To switch back, type \`,L scheme'.
tree-il@(guile-user)> ,L scheme
Happy hacking with Scheme!  To switch back, type \`,L tree-il'.
scheme@(guile-user)> 

Languages can be looked up by name, as they were above.

Scheme Procedure: **lookup-language** name [¶](09_04_compiling_to_the_virtual_machine.md)

Looks up a language named name, autoloading it if necessary.

Languages are autoloaded by looking for a variable named name in a module named `(language name spec)`.

The language object will be returned, or `#f` if there does not exist a language with that name.

When Guile goes to compile Scheme to bytecode, it will ask the Scheme language to choose a compiler from Scheme to the next language on the path from Scheme to bytecode. Performing this computation recursively builds transformations from a flexible chain of compilers. The next link will be obtained by invoking the language’s compiler chooser, or if not present, from the language’s compilers field.

A language can specify an analyzer, which is run before a term of that language is lowered and compiled. This is where compiler warnings are issued.

If a language specifies a lowerer, that procedure is called on expressions before compilation. This is where optimizations and canonicalizations go.

Finally a language’s compiler translates a lowered term from one language to the next one in the chain.

There is a notion of a “current language”, which is maintained in the `current-language` parameter, defined in the core `(guile)` module. This language is normally Scheme, and may be rebound by the user. The run-time compilation interfaces (see [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)) also allow you to choose other source and target languages.

The normal tower of languages when compiling Scheme goes like this:

*   Scheme
*   Tree Intermediate Language (Tree-IL)
*   Continuation-Passing Style (CPS)
*   Bytecode

As discussed before (see [Object File Format](09_03_a_virtual_machine_for_guile.md#936-object-file-format)), bytecode is in ELF format, ready to be serialized to disk. But when compiling Scheme at run time, you want a Scheme value: for example, a compiled procedure. For this reason, so as not to break the abstraction, Guile defines a fake language at the bottom of the tower:

*   Value

Compiling to `value` loads the bytecode into a procedure, turning cold bytes into warm code.

Perhaps this strangeness can be explained by example: `compile-file` defaults to compiling to bytecode, because it produces object code that has to live in the barren world outside the Guile runtime; but `compile` defaults to compiling to `value`, as its product re-enters the Guile world.

Indeed, the process of compilation can circulate through these different worlds indefinitely, as shown by the following quine:

((lambda (x) ((compile x) x)) '(lambda (x) ((compile x) x)))

* * *

Next: [Tree-IL](09_04_compiling_to_the_virtual_machine.md#943-tree-il), Previous: [Compiler Tower](09_04_compiling_to_the_virtual_machine.md#941-compiler-tower), Up: [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.2 The Scheme Compiler [¶](09_04_compiling_to_the_virtual_machine.md#942-the-scheme-compiler)

The job of the Scheme compiler is to expand all macros and all of Scheme to its most primitive expressions. The definition of “primitive expression” is given by the inventory of constructs provided by Tree-IL, the target language of the Scheme compiler: procedure calls, conditionals, lexical references, and so on. This is described more fully in the next section.

The tricky and amusing thing about the Scheme-to-Tree-IL compiler is that it is completely implemented by the macro expander. Since the macro expander has to run over all of the source code already in order to expand macros, it might as well do the analysis at the same time, producing Tree-IL expressions directly.

Because this compiler is actually the macro expander, it is extensible. Any macro which the user writes becomes part of the compiler.

The Scheme-to-Tree-IL expander may be invoked using the generic `compile` procedure:

([compile](04_programming_in_scheme.md) '([+](06_06_02_numerical_data_types.md) 1 2) #:from 'scheme #:to 'tree-il)
⇒
#<tree-il ([call](09_03_a_virtual_machine_for_guile.md) (toplevel [+](06_06_02_numerical_data_types.md)) ([const](06_07_procedures.md) 1) ([const](06_07_procedures.md) 2))[\>](06_06_02_numerical_data_types.md)

`(compile foo #:from 'scheme #:to 'tree-il)` is entirely equivalent to calling the macro expander as `(macroexpand foo 'c '(compile load eval))`. See [Macro Expansion](06_08_macros.md#689-macro-expansion). `compile-tree-il`, the procedure dispatched by `compile` to `'tree-il`, is a small wrapper around `macroexpand`, to make it conform to the general form of compiler procedures in Guile’s language tower.

Compiler procedures take three arguments: an expression, an environment, and a keyword list of options. They return three values: the compiled expression, the corresponding environment for the target language, and a “continuation environment”. The compiled expression and environment will serve as input to the next language’s compiler. The “continuation environment” can be used to compile another expression from the same source language within the same module.

For example, you might compile the expression, `(define-module (foo))`. This will result in a Tree-IL expression and environment. But if you compiled a second expression, you would want to take into account the compile-time effect of compiling the previous expression, which puts the user in the `(foo)` module. That is the purpose of the “continuation environment”; you would pass it as the environment when compiling the subsequent expression.

For Scheme, an environment is a module. By default, the `compile` and `compile-file` procedures compile in a fresh module, such that bindings and macros introduced by the expression being compiled are isolated:

(eq? (current-module) (compile '(current-module)))
⇒ #f

(compile '(define hello 'world))
(defined? 'hello)
⇒ #f

(define / \*)
(eq? (compile '/) /)
⇒ #f

Similarly, changes to the `current-reader` fluid (see [`current-reader`](06_16_reading_and_evaluating_scheme_code.md#6167-loading-scheme-code-from-file)) are isolated:

(compile '(fluid-set! current-reader (lambda args 'fail)))
(fluid-ref current-reader)
⇒ #f

Nevertheless, having the compiler and _compilee_ share the same name space can be achieved by explicitly passing `(current-module)` as the compilation environment:

(define hello 'world)
(compile 'hello #:env (current-module))
⇒ world

* * *

Next: [Continuation-Passing Style](09_04_compiling_to_the_virtual_machine.md#944-continuation-passing-style), Previous: [The Scheme Compiler](09_04_compiling_to_the_virtual_machine.md#942-the-scheme-compiler), Up: [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.3 Tree-IL [¶](09_04_compiling_to_the_virtual_machine.md#943-tree-il)

Tree Intermediate Language (Tree-IL) is a structured intermediate language that is close in expressive power to Scheme. It is an expanded, pre-analyzed Scheme.

Tree-IL is “structured” in the sense that its representation is based on records, not S-expressions. This gives a rigidity to the language that ensures that compiling to a lower-level language only requires a limited set of transformations. For example, the Tree-IL type `<const>` is a record type with two fields, `src` and `exp`. Instances of this type are created via `make-const`. Fields of this type are accessed via the `const-src` and `const-exp` procedures. There is also a predicate, `const?`. See [Records](06_06_17_records.md#6617-records), for more information on records.

All Tree-IL types have a `src` slot, which holds source location information for the expression. This information, if present, will be residualized into the compiled object code, allowing backtraces to show source information. The format of `src` is the same as that returned by Guile’s `source-properties` function. See [Source Properties](06_26_debugging_infrastructure.md#6263-source-properties), for more information.

Although Tree-IL objects are represented internally using records, there is also an equivalent S-expression external representation for each kind of Tree-IL. For example, the S-expression representation of `#<const src: #f exp: 3>` expression would be:

(const 3)

Users may program with this format directly at the REPL:

scheme@(guile-user)> ,language tree-il
Happy hacking with Tree Intermediate Language!  To switch back, type \`,L scheme'.
tree-il@(guile-user)> (call (primitive +) (const 32) (const 10))
⇒ 42

The `src` fields are left out of the external representation.

One may create Tree-IL objects from their external representations via calling `parse-tree-il`, the reader for Tree-IL. If any source information is attached to the input S-expression, it will be propagated to the resulting Tree-IL expressions. This is probably the easiest way to compile to Tree-IL: just make the appropriate external representations in S-expression format, and let `parse-tree-il` take care of the rest.

Scheme Variable: **<void>** src [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(void)** [¶](09_04_compiling_to_the_virtual_machine.md)

An empty expression. In practice, equivalent to Scheme’s `(if #f #f)`.

Scheme Variable: **<const>** src exp [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(const** exp) [¶](09_04_compiling_to_the_virtual_machine.md)

A constant.

Scheme Variable: **<primitive-ref>** src name [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(primitive** name) [¶](09_04_compiling_to_the_virtual_machine.md)

A reference to a “primitive”. A primitive is a procedure that, when compiled, may be open-coded. For example, `cons` is usually recognized as a primitive, so that it compiles down to a single instruction.

Compilation of Tree-IL usually begins with a pass that resolves some `<module-ref>` and `<toplevel-ref>` expressions to `<primitive-ref>` expressions. The actual compilation pass has special cases for calls to certain primitives, like `apply` or `cons`.

Scheme Variable: **<lexical-ref>** src name gensym [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(lexical** name gensym) [¶](09_04_compiling_to_the_virtual_machine.md)

A reference to a lexically-bound variable. The name is the original name of the variable in the source program. gensym is a unique identifier for this variable.

Scheme Variable: **<lexical-set>** src name gensym exp [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(set!** (lexical name gensym) exp) [¶](09_04_compiling_to_the_virtual_machine.md)

Sets a lexically-bound variable.

Scheme Variable: **<module-ref>** src mod name public? [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(@** mod name) [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(@@** mod name) [¶](09_04_compiling_to_the_virtual_machine.md)

A reference to a variable in a specific module. mod should be the name of the module, e.g. `(guile-user)`.

If public? is true, the variable named name will be looked up in mod’s public interface, and serialized with `@`; otherwise it will be looked up among the module’s private bindings, and is serialized with `@@`.

Scheme Variable: **<module-set>** src mod name public? exp [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(set!** (@ mod name) exp) [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(set!** (@@ mod name) exp) [¶](09_04_compiling_to_the_virtual_machine.md)

Sets a variable in a specific module.

Scheme Variable: **<toplevel-ref>** src name [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(toplevel** name) [¶](09_04_compiling_to_the_virtual_machine.md)

References a variable from the current procedure’s module.

Scheme Variable: **<toplevel-set>** src name exp [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(set!** (toplevel name) exp) [¶](09_04_compiling_to_the_virtual_machine.md)

Sets a variable in the current procedure’s module.

Scheme Variable: **<toplevel-define>** src name exp [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(define** name exp) [¶](09_04_compiling_to_the_virtual_machine.md)

Defines a new top-level variable in the current procedure’s module.

Scheme Variable: **<conditional>** src test then else [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(if** test then else) [¶](09_04_compiling_to_the_virtual_machine.md)

A conditional. Note that else is not optional.

Scheme Variable: **<call>** src proc args [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(call** proc . args) [¶](09_04_compiling_to_the_virtual_machine.md)

A procedure call.

Scheme Variable: **<primcall>** src name args [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(primcall** name . args) [¶](09_04_compiling_to_the_virtual_machine.md)

A call to a primitive. Equivalent to `(call (primitive name) . args)`. This construct is often more convenient to generate and analyze than `<call>`.

As part of the compilation process, instances of `(call (primitive name) . args)` are transformed into primcalls.

Scheme Variable: **<seq>** src head tail [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(seq** head tail) [¶](09_04_compiling_to_the_virtual_machine.md)

A sequence. The semantics is that head is evaluated first, and any resulting values are ignored. Then tail is evaluated, in tail position.

Scheme Variable: **<lambda>** src meta body [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(lambda** meta body) [¶](09_04_compiling_to_the_virtual_machine.md)

A closure. meta is an association list of properties for the procedure. body is a single Tree-IL expression of type `<lambda-case>`. As the `<lambda-case>` clause can chain to an alternate clause, this makes Tree-IL’s `<lambda>` have the expressiveness of Scheme’s `case-lambda`.

Scheme Variable: **<lambda-case>** req opt rest kw inits gensyms body alternate [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(lambda-case** ((req opt rest kw inits gensyms) body) \[alternate\]) [¶](09_04_compiling_to_the_virtual_machine.md)

One clause of a `case-lambda`. A `lambda` expression in Scheme is treated as a `case-lambda` with one clause.

req is a list of the procedure’s required arguments, as symbols. opt is a list of the optional arguments, or `#f` if there are no optional arguments. rest is the name of the rest argument, or `#f`.

kw is a list of the form, `(allow-other-keys? (keyword name var) ...)`, where keyword is the keyword corresponding to the argument named name, and whose corresponding gensym is var, or `#f` if there are no keyword arguments. inits are tree-il expressions corresponding to all of the optional and keyword arguments, evaluated to bind variables whose value is not supplied by the procedure caller. Each init expression is evaluated in the lexical context of previously bound variables, from left to right.

gensyms is a list of gensyms corresponding to all arguments: first all of the required arguments, then the optional arguments if any, then the rest argument if any, then all of the keyword arguments.

body is the body of the clause. If the procedure is called with an appropriate number of arguments, body is evaluated in tail position. Otherwise, if there is an alternate, it should be a `<lambda-case>` expression, representing the next clause to try. If there is no alternate, a wrong-number-of-arguments error is signaled.

Scheme Variable: **<let>** src names gensyms vals exp [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(let** names gensyms vals exp) [¶](09_04_compiling_to_the_virtual_machine.md)

Lexical binding, like Scheme’s `let`. names are the original binding names, gensyms are gensyms corresponding to the names, and vals are Tree-IL expressions for the values. exp is a single Tree-IL expression.

Scheme Variable: **<letrec>** in-order? src names gensyms vals exp [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(letrec** names gensyms vals exp) [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(letrec\*** names gensyms vals exp) [¶](09_04_compiling_to_the_virtual_machine.md)

A version of `<let>` that creates recursive bindings, like Scheme’s `letrec`, or `letrec*` if in-order? is true.

Scheme Variable: **<prompt>** escape-only? tag body handler [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(prompt** escape-only? tag body handler) [¶](09_04_compiling_to_the_virtual_machine.md)

A dynamic prompt. Instates a prompt named tag, an expression, during the dynamic extent of the execution of body, also an expression. If an abort occurs to this prompt, control will be passed to handler, also an expression, which should be a procedure. The first argument to the handler procedure will be the captured continuation, followed by all of the values passed to the abort. If escape-only? is true, the handler should be a `<lambda>` with a single `<lambda-case>` body expression with no optional or keyword arguments, and no alternate, and whose first argument is unreferenced. See [Prompts](06_11_controlling_the_flow_of_program_execution.md#6115-prompts), for more information.

Scheme Variable: **<abort>** tag args tail [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(abort** tag args tail) [¶](09_04_compiling_to_the_virtual_machine.md)

An abort to the nearest prompt with the name tag, an expression. args should be a list of expressions to pass to the prompt’s handler, and tail should be an expression that will evaluate to a list of additional arguments. An abort will save the partial continuation, which may later be reinstated, resulting in the `<abort>` expression evaluating to some number of values.

There are two Tree-IL constructs that are not normally produced by higher-level compilers, but instead are generated during the source-to-source optimization and analysis passes that the Tree-IL compiler does. Users should not generate these expressions directly, unless they feel very clever, as the default analysis pass will generate them as necessary.

Scheme Variable: **<let-values>** src names gensyms exp body [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(let-values** names gensyms exp body) [¶](09_04_compiling_to_the_virtual_machine.md)

Like Scheme’s `receive` – binds the values returned by evaluating `exp` to the `lambda`\-like bindings described by gensyms. That is to say, gensyms may be an improper list.

`<let-values>` is an optimization of a `<call>` to the primitive, `call-with-values`.

Scheme Variable: **<fix>** src names gensyms vals body [¶](09_04_compiling_to_the_virtual_machine.md)

External Representation: **(fix** names gensyms vals body) [¶](09_04_compiling_to_the_virtual_machine.md)

Like `<letrec>`, but only for vals that are unset `lambda` expressions.

`fix` is an optimization of `letrec` (and `let`).

Tree-IL is a convenient compilation target from source languages. It can be convenient as a medium for optimization, though CPS is usually better. The strength of Tree-IL is that it does not fix order of evaluation, so it makes some code motion a bit easier.

Optimization passes performed on Tree-IL currently include:

*   Open-coding (turning toplevel-refs into primitive-refs, and calls to primitives to primcalls)
*   Partial evaluation (comprising inlining, copy propagation, and constant folding)

* * *

Next: [Bytecode](09_04_compiling_to_the_virtual_machine.md#945-bytecode), Previous: [Tree-IL](09_04_compiling_to_the_virtual_machine.md#943-tree-il), Up: [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.4 Continuation-Passing Style [¶](09_04_compiling_to_the_virtual_machine.md#944-continuation-passing-style)

Continuation-passing style (CPS) is Guile’s principal intermediate language, bridging the gap between languages for people and languages for machines. CPS gives a name to every part of a program: every control point, and every intermediate value. This makes it an excellent medium for reasoning about programs, which is the principal job of a compiler.

*   [An Introduction to CPS](09_04_compiling_to_the_virtual_machine.md#9441-an-introduction-to-cps)
*   [CPS in Guile](09_04_compiling_to_the_virtual_machine.md#9442-cps-in-guile)
*   [Building CPS](09_04_compiling_to_the_virtual_machine.md#9443-building-cps)
*   [CPS Soup](09_04_compiling_to_the_virtual_machine.md#9444-cps-soup)
*   [Compiling CPS](09_04_compiling_to_the_virtual_machine.md#9445-compiling-cps)

* * *

Next: [CPS in Guile](09_04_compiling_to_the_virtual_machine.md#9442-cps-in-guile), Up: [Continuation-Passing Style](09_04_compiling_to_the_virtual_machine.md#944-continuation-passing-style)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.4.1 An Introduction to CPS [¶](09_04_compiling_to_the_virtual_machine.md#9441-an-introduction-to-cps)

Consider the following Scheme expression:

(begin
  ([display](06_16_reading_and_evaluating_scheme_code.md) "The sum of 32 and 10 is: ")
  ([display](06_16_reading_and_evaluating_scheme_code.md) 42)
  ([newline](06_12_input_and_output.md)))

Let us identify all of the sub-expressions in this expression, annotating them with unique labels:

(begin
  ([display](06_16_reading_and_evaluating_scheme_code.md) "The sum of 32 and 10 is: ")
  |k1      k2
  k0
  ([display](06_16_reading_and_evaluating_scheme_code.md) 42)
  |k4      k5
  k3
  ([newline](06_12_input_and_output.md)))
  |k7
  k6

Each of these labels identifies a point in a program. One label may be the continuation of another label. For example, the continuation of `k7` is `k6`. This is because after evaluating the value of `newline`, performed by the expression labelled `k7`, we continue to apply it in `k6`.

Which expression has `k0` as its continuation? It is either the expression labelled `k1` or the expression labelled `k2`. Scheme does not have a fixed order of evaluation of arguments, though it does guarantee that they are evaluated in some order. Unlike general Scheme, continuation-passing style makes evaluation order explicit. In Guile, this choice is made by the higher-level language compilers.

Let us assume a left-to-right evaluation order. In that case the continuation of `k1` is `k2`, and the continuation of `k2` is `k0`.

With this example established, we are ready to give an example of CPS in Scheme:

(lambda (ktail)
  (let ((k1 (lambda ()
              (let ((k2 (lambda (proc)
                          (let ((k0 (lambda (arg0)
                                      (proc k4 arg0))))
                            (k0 "The sum of 32 and 10 is: ")))))
                (k2 [display](06_16_reading_and_evaluating_scheme_code.md)))))
        (k4 (lambda [\_](06_08_macros.md)
              (let ((k5 (lambda (proc)
                          (let ((k3 (lambda (arg0)
                                      (proc k7 arg0))))
                            (k3 42)))))
                (k5 [display](06_16_reading_and_evaluating_scheme_code.md)))))
        (k7 (lambda [\_](06_08_macros.md)
              (let ((k6 (lambda (proc)
                          (proc ktail))))
                (k6 [newline](06_12_input_and_output.md))))))
    (k1))

Holy code explosion, Batman! What’s with all the lambdas? Indeed, CPS is by nature much more verbose than “direct-style” intermediate languages like Tree-IL. At the same time, CPS is simpler than full Scheme, because it makes things more explicit.

In the original program, the expression labelled `k0` is in effect context. Any values it returns are ignored. In Scheme, this fact is implicit. In CPS, we can see it explicitly by noting that its continuation, `k4`, takes any number of values and ignores them. Compare this to `k2`, which takes a single value; in this way we can say that `k1` is in a “value” context. Likewise `k6` is in tail context with respect to the expression as a whole, because its continuation is the tail continuation, `ktail`. CPS makes these details manifest, and gives them names.

* * *

Next: [Building CPS](09_04_compiling_to_the_virtual_machine.md#9443-building-cps), Previous: [An Introduction to CPS](09_04_compiling_to_the_virtual_machine.md#9441-an-introduction-to-cps), Up: [Continuation-Passing Style](09_04_compiling_to_the_virtual_machine.md#944-continuation-passing-style)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.4.2 CPS in Guile [¶](09_04_compiling_to_the_virtual_machine.md#9442-cps-in-guile)

Guile’s CPS language is composed of _continuations_. A continuation is a labelled program point. If you are used to traditional compilers, think of a continuation as a trivial basic block. A program is a “soup” of continuations, represented as a map from labels to continuations.

Like basic blocks, each continuation belongs to only one function. Some continuations are special, like the continuation corresponding to a function’s entry point, or the continuation that represents the tail of a function. Others contain a _term_. A term contains an _expression_, which evaluates to zero or more values. The term also records the continuation to which it will pass its values. Some terms, like conditional branches, may continue to one of a number of continuations.

Continuation labels are small integers. This makes it easy to sort them and to group them into sets. Whenever a term refers to a continuation, it does so by name, simply recording the label of the continuation. Continuation labels are unique among the set of labels in a program.

Variables are also named by small integers. Variable names are unique among the set of variables in a program.

For example, a simple continuation that receives two values and adds them together can be matched like this, using the `match` form from `(ice-9 match)`:

(match cont
  (($ $kargs (x-name y-name) (x-var y-var)
      ($ $continue k src ($ $primcall '+ #f (x-var y-var))))
   (format #t "Add ~a and ~a and pass the result to label ~a"
           x-var y-var k)))

Here we see the most common kind of continuation, `$kargs`, which binds some number of values to variables and then evaluates a term.

CPS Continuation: **$kargs** names vars term [¶](09_04_compiling_to_the_virtual_machine.md)

Bind the incoming values to the variables vars, with original names names, and then evaluate term.

The names of a `$kargs` are just for debugging, and will end up residualized in the object file for use by the debugger.

The term in a `$kargs` is always a `$continue`, which evaluates an expression and continues to a continuation.

CPS Term: **$continue** k src exp [¶](09_04_compiling_to_the_virtual_machine.md)

Evaluate the expression exp and pass the resulting values (if any) to the continuation labelled k. The source information associated with the expression may be found in src, which is either an alist as in `source-properties` or is `#f` if there is no associated source.

There are a number of expression kinds. Above you see an example of `$primcall`.

CPS Expression: **$primcall** name param args [¶](09_04_compiling_to_the_virtual_machine.md)

Perform the primitive operation identified by `name`, a well-known symbol, passing it the arguments args, and pass all resulting values to the continuation.

param is a constant parameter whose interpretation is up to the primcall in question. Usually it’s `#f` but for a primcall that might need some compile-time constant information – such as `add/immediate`, which adds a constant number to a value – the parameter holds this information.

The set of available primitives includes many primitives known to Tree-IL and then some more; see the source code for details. Note that some Tree-IL primcalls need to be converted to a sequence of lower-level CPS primcalls. Again, see `(language tree-il compile-cps)` for full details.

The variables that are used by `$primcall`, or indeed by any expression, must be defined before the expression is evaluated. An equivalent way of saying this is that predecessor `$kargs` continuation(s) that bind the variables(s) used by the expression must _dominate_ the continuation that uses the expression: definitions dominate uses. This condition is trivially satisfied in our example above, but in general to determine the set of variables that are in “scope” for a given term, you need to do a flow analysis to see what continuations dominate a term. The variables that are in scope are those variables defined by the continuations that dominate a term.

Here is an inventory of the kinds of expressions in Guile’s CPS language, besides `$primcall` which has already been described. Recall that all expressions are wrapped in a `$continue` term which specifies their continuation.

CPS Expression: **$const** val [¶](09_04_compiling_to_the_virtual_machine.md)

Continue with the constant value val.

CPS Expression: **$prim** name [¶](09_04_compiling_to_the_virtual_machine.md)

Continue with the procedure that implements the primitive operation named by name.

CPS Expression: **$call** proc args [¶](09_04_compiling_to_the_virtual_machine.md)

Call proc with the arguments args, and pass all values to the continuation. proc and the elements of the args list should all be variable names. The continuation identified by the term’s k should be a `$kreceive` or a `$ktail` instance.

CPS Expression: **$values** args [¶](09_04_compiling_to_the_virtual_machine.md)

Pass the values named by the list args to the continuation.

CPS Expression: **$prompt** escape? tag handler [¶](09_04_compiling_to_the_virtual_machine.md)

There are two sub-languages of CPS, _higher-order CPS_ and _first-order CPS_. The difference is that in higher-order CPS, there are `$fun` and `$rec` expressions that bind functions or mutually-recursive functions in the implicit scope of their use sites. Guile transforms higher-order CPS into first-order CPS by _closure conversion_, which chooses representations for all closures and which arranges to access free variables through the implicit closure parameter that is passed to every function call.

CPS Expression: **$fun** body [¶](09_04_compiling_to_the_virtual_machine.md)

Continue with a procedure. body names the entry point of the function, which should be a `$kfun`. This expression kind is only valid in higher-order CPS, which is the CPS language before closure conversion.

CPS Expression: **$rec** names vars funs [¶](09_04_compiling_to_the_virtual_machine.md)

Continue with a set of mutually recursive procedures denoted by names, vars, and funs. names is a list of symbols, vars is a list of variable names (unique integers), and funs is a list of `$fun` values. Note that the `$kargs` continuation should also define names/vars bindings.

The contification pass will attempt to transform the functions declared in a `$rec` into local continuations. Any remaining `$fun` instances are later removed by the closure conversion pass. If the function has no free variables, it gets allocated as a constant.

CPS Expression: **$const-fun** label [¶](09_04_compiling_to_the_virtual_machine.md)

A constant which is a function whose entry point is label. As a constant, instances of `$const-fun` with the same label will not allocate; the space for the function is allocated as part of the compilation unit.

In practice, `$const-fun` expressions are reified by CPS-conversion for functions whose call sites are not all visible within the compilation unit and which have no free variables. This expression kind is part of first-order CPS.

Otherwise, if the closure has free variables, it will be allocated at its definition site via an `allocate-words` primcall and its free variables initialized there. The code pointer in the closure is initialized from a `$code` expression.

CPS Expression: **$code** label [¶](09_04_compiling_to_the_virtual_machine.md)

Continue with the value of label, which should denote some `$kfun` continuation in the program. Used when initializing the code pointer of closure objects.

However, If the closure can be proven to never escape its scope then other lighter-weight representations can be chosen. Additionally, if all call sites are known, closure conversion will hard-wire the calls by lowering `$call` to `$callk`.

CPS Expression: **$callk** label proc args [¶](09_04_compiling_to_the_virtual_machine.md)

Like `$call`, but for the case where the call target is known to be in the same compilation unit. label should denote some `$kfun` continuation in the program. In this case the proc is simply an additional argument, since it is not used to determine the call target at run-time.

To summarize: a `$continue` is a CPS term that continues to a single label. But there are other kinds of CPS terms that can continue to a different number of labels: `$branch`, `$switch`, `$throw`, and `$prompt`.

CPS Term: **$branch** kf kt src op param args [¶](09_04_compiling_to_the_virtual_machine.md)

Evaluate the branching primcall op, with arguments args and constant parameter param, and continue to kt with zero values if the test is true. Otherwise continue to kf.

The `$branch` term is like a `$continue` term with a `$primcall` expression, except that instead of binding a value and continuing to a single label, the result of the test is not bound but instead used to choose the continuation label.

The set of operations (corresponding to op values) that are valid in a $branch is limited. In the general case, bind the result of a test expression to a variable, and then make a `$branch` on a `true?` op referencing that variable. The optimizer should inline the branch if possible.

CPS Term: **$switch** kf kt\* src arg [¶](09_04_compiling_to_the_virtual_machine.md)

Continue to a label in the list k\* according to the index argument arg, or to the default continuation kf if arg is greater than or equal to the length k\*. The index variable arg is an unboxed, unsigned 64-bit value.

The `$switch` term is like C’s `switch` statement. The compiler to CPS can generate a `$switch` term directly, if the source language has such a concept, or it can rely on the CPS optimizer to turn appropriate chains of `$branch` statements to `$switch` instances, which is what the Scheme compiler does.

CPS Term: **$throw** src op param args [¶](09_04_compiling_to_the_virtual_machine.md)

Throw a non-resumable exception. Throw terms do not continue at all. The usual value of op is `throw`, with two arguments key and args. There are also some specific primcalls that compile to the VM `throw/value` and `throw/value+data` instructions; see the code for full details.

The advantage of having `$throw` as a term is that, because it does not continue, this allows the optimizer to gather more information from type predicates. For example, if the predicate is `char?` and the kf continues to a throw, the set of labels dominated by kt is larger than if the throw notationally continued to some label that would never be reached by the throw.

CPS Term: **$prompt** k kh src escape? tag [¶](09_04_compiling_to_the_virtual_machine.md)

Push a prompt on the stack identified by the variable name tag, which may be escape-only if escape? is true, and continue to kh with zero values. If the body aborts to this prompt, control will proceed at the continuation labelled kh, which should be a `$kreceive` continuation. Prompts are later popped by `pop-prompt` primcalls.

At this point we have described terms, expressions, and the most common kind of continuation, `$kargs`. `$kargs` is used when the predecessors of the continuation can be instructed to pass the values where the continuation wants them. For example, if a `$kargs` continuation k binds a variable v, and the compiler decides to allocate v to slot 6, all predecessors of k should put the value for v in slot 6 before jumping to k. One situation in which this isn’t possible is receiving values from function calls. Guile has a calling convention for functions which currently places return values on the stack. A continuation of a call must check that the number of values returned from a function matches the expected number of values, and then must shuffle or collect those values to named variables. `$kreceive` denotes this kind of continuation.

CPS Continuation: **$kreceive** arity k [¶](09_04_compiling_to_the_virtual_machine.md)

Receive values on the stack. Parse them according to arity, and then proceed with the parsed values to the `$kargs` continuation labelled k. As a limitation specific to `$kreceive`, arity may only contain required and rest arguments.

`$arity` is a helper data structure used by `$kreceive` and also by `$kclause`, described below.

CPS Data: **$arity** req opt rest kw allow-other-keys? [¶](09_04_compiling_to_the_virtual_machine.md)

A data type declaring an arity. req and opt are lists of source names of required and optional arguments, respectively. rest is either the source name of the rest variable, or `#f` if this arity does not accept additional values. kw is a list of the form `((keyword name var) ...)`, describing the keyword arguments. allow-other-keys? is true if other keyword arguments are allowed and false otherwise.

Note that all of these names with the exception of the vars in the kw list are source names, not unique variable names.

Additionally, there are three specific kinds of continuations that are only used in function entries.

CPS Continuation: **$kfun** src meta self tail clause [¶](09_04_compiling_to_the_virtual_machine.md)

Declare a function entry. src is the source information for the procedure declaration, and meta is the metadata alist as described above in Tree-IL’s `<lambda>`. self is a variable bound to the procedure being called, and which may be used for self-references. tail is the label of the `$ktail` for this function, corresponding to the function’s tail continuation. clause is the label of the first `$kclause` for the first `case-lambda` clause in the function, or otherwise `#f`.

CPS Continuation: **$ktail** [¶](09_04_compiling_to_the_virtual_machine.md)

A tail continuation.

CPS Continuation: **$kclause** arity cont alternate [¶](09_04_compiling_to_the_virtual_machine.md)

A clause of a function with a given arity. Applications of a function with a compatible set of actual arguments will continue to the continuation labelled cont, a `$kargs` instance representing the clause body. If the arguments are incompatible, control proceeds to alternate, which is a `$kclause` for the next clause, or `#f` if there is no next clause.

* * *

Next: [CPS Soup](09_04_compiling_to_the_virtual_machine.md#9444-cps-soup), Previous: [CPS in Guile](09_04_compiling_to_the_virtual_machine.md#9442-cps-in-guile), Up: [Continuation-Passing Style](09_04_compiling_to_the_virtual_machine.md#944-continuation-passing-style)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.4.3 Building CPS [¶](09_04_compiling_to_the_virtual_machine.md#9443-building-cps)

Unlike Tree-IL, the CPS language is built to be constructed and deconstructed with abstract macros instead of via procedural constructors or accessors, or instead of S-expression matching.

Deconstruction and matching is handled adequately by the `match` form from `(ice-9 match)`. See [Pattern Matching](07_08_pattern_matching.md#78-pattern-matching). Construction is handled by a set of mutually builder macros: `build-term`, `build-cont`, and `build-exp`.

In the following interface definitions, consider `term` and `exp` to be built by `build-term` or `build-exp`, respectively. Consider any other name to be evaluated as a Scheme expression. Many of these forms recognize `unquote` in some contexts, to splice in a previously-built value; see the specifications below for full details.

Scheme Syntax: **build-term** ,val [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-term** ($continue k src exp) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ,val [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($const val) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($prim name) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($fun kentry) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($const-fun kentry) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($code kentry) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($rec names syms funs) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($call proc (arg ...)) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($call proc args) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($callk k proc (arg ...)) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($callk k proc args) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($primcall name param (arg ...)) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($primcall name param args) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($values (arg ...)) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($values args) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-exp** ($prompt escape? tag handler) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-term** ($branch kf kt src op param (arg ...)) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-term** ($branch kf kt src op param args) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-term** ($switch kf kt\* src arg) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-term** ($throw src op param (arg ...)) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-term** ($throw src op param args) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-term** ($prompt k kh src escape? tag) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-cont** ,val [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-cont** ($kargs (name ...) (sym ...) term) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-cont** ($kargs names syms term) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-cont** ($kreceive req rest kargs) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-cont** ($kfun src meta self ktail kclause) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-cont** ($kclause ,arity kbody kalt) [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **build-cont** ($kclause (req opt rest kw aok?) kbody) [¶](09_04_compiling_to_the_virtual_machine.md)

Construct a CPS term, expression, or continuation.

There are a few more miscellaneous interfaces as well.

Scheme Procedure: **make-arity** req opt rest kw allow-other-keywords? [¶](09_04_compiling_to_the_virtual_machine.md)

A procedural constructor for `$arity` objects.

Scheme Syntax: **rewrite-term** val (pat term) ... [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **rewrite-exp** val (pat exp) ... [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Syntax: **rewrite-cont** val (pat cont) ... [¶](09_04_compiling_to_the_virtual_machine.md)

Match val against the series of patterns pat..., using `match`. The body of the matching clause should be a template in the syntax of `build-term`, `build-exp`, or `build-cont`, respectively.

* * *

Next: [Compiling CPS](09_04_compiling_to_the_virtual_machine.md#9445-compiling-cps), Previous: [Building CPS](09_04_compiling_to_the_virtual_machine.md#9443-building-cps), Up: [Continuation-Passing Style](09_04_compiling_to_the_virtual_machine.md#944-continuation-passing-style)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.4.4 CPS Soup [¶](09_04_compiling_to_the_virtual_machine.md#9444-cps-soup)

We describe programs in Guile’s CPS language as being a kind of “soup” because all continuations in the program are mixed into the same “pot”, so to speak, without explicit markers as to what function or scope a continuation is in. A program in CPS is a map from continuation labels to continuation values. As discussed in the introduction, a continuation label is an integer. No label may be negative.

As a matter of convention, label 0 should map to the `$kfun` continuation of the entry to the program, which should be a function of no arguments. The body of a function consists of the labelled continuations that are reachable from the function entry. A program can refer to other functions, either via `$fun` and `$rec` in higher-order CPS, or via `$const-fun`, `$callk`, and allocated closures in first-order CPS. The program logically contains all continuations of all functions reachable from the entry function. A compiler pass may leave unreachable continuations in a program; subsequent compiler passes should ensure that their transformations and analyses only take reachable continuations into account. It’s OK though if transformation runs over all continuations if including the unreachable continuations has no effect on the transformations on the live continuations.

The “soup” itself is implemented as an _intmap_, a functional array-mapped trie specialized for integer keys. Intmaps associate integers with values of any kind. Currently intmaps are a private data structure only used by the CPS phase of the compiler. To work with intmaps, load the `(language cps intmap)` module:

(use-modules (language cps intmap))

Intmaps are functional data structures, so there is no constructor as such: one can simply start with the empty intmap and add entries to it.

(intmap? empty-intmap) ⇒ #t
(define x (intmap-add empty-intmap 42 "hi"))
(intmap? x) ⇒ #t
(intmap-ref x 42) ⇒ "hi"
(intmap-ref x 43) ⇒ error: 43 not present
(intmap-ref x 43 (lambda (k) "yo!")) ⇒ "yo"
(intmap-add x 42 "hej") ⇒ error: 42 already present

`intmap-ref` and `intmap-add` are the core of the intmap interface. There is also `intmap-replace`, which replaces the value associated with a given key, requiring that the key was present already, and `intmap-remove`, which removes a key from an intmap.

Intmaps have a tree-like structure that is well-suited to set operations such as union and intersection, so there are also the binary `intmap-union` and `intmap-intersect` procedures. If the result is equivalent to either argument, that argument is returned as-is; in that way, one can detect whether the set operation produced a new result simply by checking with `eq?`. This makes intmaps useful when computing fixed points.

If a key is present in both intmaps and the associated values are not the same in the sense of `eq?`, the resulting value is determined by a “meet” procedure, which is the optional last argument to `intmap-union`, `intmap-intersect`, and also to `intmap-add`, `intmap-replace`, and similar functions. The meet procedure will be called with the two values and should return the intersected or unioned value in some domain-specific way. If no meet procedure is given, the default meet procedure will raise an error.

To traverse over the set of values in an intmap, there are the `intmap-next` and `intmap-prev` procedures. For example, if intmap x has one entry mapping 42 to some value, we would have:

(intmap-next x) ⇒ 42
(intmap-next x 0) ⇒ 42
(intmap-next x 42) ⇒ 42
(intmap-next x 43) ⇒ #f
(intmap-prev x) ⇒ 42
(intmap-prev x 42) ⇒ 42
(intmap-prev x 41) ⇒ #f

There is also the `intmap-fold` procedure, which folds over keys and values in the intmap from lowest to highest value, and `intmap-fold-right` which does so in the opposite direction. These procedures may take up to 3 seed values. The number of values that the fold procedure returns is the number of seed values.

(define q (intmap-add (intmap-add empty-intmap 1 2) 3 4))
(intmap-fold acons q '()) ⇒ ((3 . 4) (1 . 2))
(intmap-fold-right acons q '()) ⇒ ((1 . 2) (3 . 4))

When an entry in an intmap is updated (removed, added, or changed), a new intmap is created that shares structure with the original intmap. This operation ensures that the result of existing computations is not affected by future computations: no mutation is ever visible to user code. This is a great property in a compiler data structure, as it lets us hold a copy of a program before a transformation and use it while we build a post-transformation program. Updating an intmap is O(log n) in the size of the intmap.

However, the O(log n) allocation costs are sometimes too much, especially in cases when we know that we can just update the intmap in place. As an example, say we have an intmap mapping the integers 1 to 100 to the integers 42 to 141. Let’s say that we want to transform this map by adding 1 to each value. There is already an efficient `intmap-map` procedure in the `(language cps utils)` module, but if we didn’t know about that we might do:

(define (intmap-increment map)
  (let lp ((k 0) (map map))
    (let ((k (intmap-next map k)))
      (if k
          (let ((v (intmap-ref map k)))
            (lp (1+ k) (intmap-replace map k (1+ v))))
          map))))

Observe that the intermediate values created by `intmap-replace` are completely invisible to the program – only the last result of `intmap-replace` value is needed. The rest might as well share state with the last one, and we could update in place. Guile allows this kind of interface via _transient intmaps_, inspired by Clojure’s transient interface ([http://clojure.org/transients](http://clojure.org/transients)).

The in-place `intmap-add!` and `intmap-replace!` procedures return transient intmaps. If one of these in-place procedures is called on a normal persistent intmap, a new transient intmap is created. This is an O(1) operation. In all other respects the interface is like their persistent counterparts, `intmap-add` and `intmap-replace`. If an in-place procedure is called on a transient intmap, the intmap is mutated in-place and the same value is returned.

If a persistent operation like `intmap-add` is called on a transient intmap, the transient’s mutable substructure is then marked as persistent, and `intmap-add` then runs on a new persistent intmap sharing structure but not state with the original transient. Mutating a transient will cause enough copying to ensure that it can make its change, but if part of its substructure is already “owned” by it, no more copying is needed.

We can use transients to make `intmap-increment` more efficient. The two changed elements have been marked **like this**.

(define (intmap-increment map)
  (let lp ((k 0) (map map))
    (let ((k (intmap-next map k)))
      (if k
          (let ((v (intmap-ref map k)))
            (lp (1+ k) (intmap-replace! map k (1+ v))))
          (persistent-intmap map)))))

Be sure to tag the result as persistent using the `persistent-intmap` procedure to prevent the mutability from leaking to other parts of the program. For added paranoia, you could call `persistent-intmap` on the incoming map, to ensure that if it were already transient, that the mutations in the body of `intmap-increment` wouldn’t affect the incoming value.

In summary, programs in CPS are intmaps whose values are continuations. See the source code of `(language cps utils)` for a number of useful facilities for working with CPS values.

* * *

Previous: [CPS Soup](09_04_compiling_to_the_virtual_machine.md#9444-cps-soup), Up: [Continuation-Passing Style](09_04_compiling_to_the_virtual_machine.md#944-continuation-passing-style)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.4.5 Compiling CPS [¶](09_04_compiling_to_the_virtual_machine.md#9445-compiling-cps)

Compiling CPS in Guile has three phases: conversion, optimization, and code generation.

CPS conversion is the process of taking a higher-level language and compiling it to CPS. Source languages can do this directly, or they can convert to Tree-IL (which is probably easier) and let Tree-IL convert to CPS later. Going through Tree-IL has the advantage of running Tree-IL optimization passes, like partial evaluation. Also, the compiler from Tree-IL to CPS handles assignment conversion, in which assigned local variables (in Tree-IL, locals that are `<lexical-set>`) are converted to being boxed values on the heap. See [Variables and the VM](09_03_a_virtual_machine_for_guile.md#934-variables-and-the-vm).

After CPS conversion, Guile runs some optimization passes over the CPS. Most optimization in Guile is done on the CPS language. The one major exception is partial evaluation, which for historic reasons is done on Tree-IL.

The major optimization performed on CPS is contification, in which functions that are always called with the same continuation are incorporated directly into a function’s body. This opens up space for more optimizations, and turns procedure calls into `goto`. It can also make loops out of recursive function nests. Guile also does dead code elimination, common subexpression elimination, loop peeling and invariant code motion, and range and type inference.

The rest of the optimization passes are really cleanups and canonicalizations. CPS spans the gap between high-level languages and low-level bytecodes, which allows much of the compilation process to be expressed as source-to-source transformations. Such is the case for closure conversion, in which references to variables that are free in a function are converted to closure references, and in which functions are converted to closures. There are a few more passes to ensure that the only primcalls left in the term are those that have a corresponding instruction in the virtual machine, and that their continuations expect the right number of values.

Finally, the backend of the CPS compiler emits bytecode for each function, one by one. To do so, it determines the set of live variables at all points in the function. Using this liveness information, it allocates stack slots to each variable, such that a variable can live in one slot for the duration of its lifetime, without shuffling. (Of course, variables with disjoint lifetimes can share a slot.) Finally the backend emits code, typically just one VM instruction, for each continuation in the function.

* * *

Next: [Writing New High-Level Languages](09_04_compiling_to_the_virtual_machine.md#946-writing-new-high-level-languages), Previous: [Continuation-Passing Style](09_04_compiling_to_the_virtual_machine.md#944-continuation-passing-style), Up: [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.5 Bytecode [¶](09_04_compiling_to_the_virtual_machine.md#945-bytecode)

As mentioned before, Guile compiles all code to bytecode, and that bytecode is contained in ELF images. See [Object File Format](09_03_a_virtual_machine_for_guile.md#936-object-file-format), for more on Guile’s use of ELF.

To produce a bytecode image, Guile provides an assembler and a linker.

The assembler, defined in the `(system vm assembler)` module, has a relatively straightforward imperative interface. It provides a `make-assembler` function to instantiate an assembler and a set of `emit-inst` procedures to emit instructions of each kind.

The `emit-inst` procedures are actually generated at compile-time from a machine-readable description of the VM. With a few exceptions for certain operand types, each operand of an emit procedure corresponds to an operand of the corresponding instruction.

Consider `allocate-words`, from see [Memory Access Instructions](09_03_a_virtual_machine_for_guile.md#9379-memory-access-instructions). It is documented as:

Instruction: **allocate-words** `s12:dst s12:nwords` [¶](09_04_compiling_to_the_virtual_machine.md)

Therefore the emit procedure has the form:

Scheme Procedure: **emit-allocate-words** asm dst nwords [¶](09_04_compiling_to_the_virtual_machine.md)

All emit procedure take the assembler as their first argument, and return no useful values.

The argument types depend on the operand types. See [Instruction Set](09_03_a_virtual_machine_for_guile.md#937-instruction-set). Most are integers within a restricted range, though labels are generally expressed as opaque symbols. Besides the emitters that correspond to instructions, there are a few additional helpers defined in the assembler module.

Scheme Procedure: **emit-label** asm label [¶](09_04_compiling_to_the_virtual_machine.md)

Define a label at the current program point.

Scheme Procedure: **emit-source** asm source [¶](09_04_compiling_to_the_virtual_machine.md)

Associate source with the current program point.

Scheme Procedure: **emit-cache-ref** asm dst key [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Procedure: **emit-cache-set!** asm key val [¶](09_04_compiling_to_the_virtual_machine.md)

Macro-instructions to implement compilation-unit caches. A single cache cell corresponding to key will be allocated for the compilation unit.

Scheme Procedure: **emit-load-constant** asm dst constant [¶](09_04_compiling_to_the_virtual_machine.md)

Load the Scheme datum constant into dst.

Scheme Procedure: **emit-begin-program** asm label properties [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Procedure: **emit-end-program** asm [¶](09_04_compiling_to_the_virtual_machine.md)

Delimit the bounds of a procedure, with the given label and the metadata properties.

Scheme Procedure: **emit-load-static-procedure** asm dst label [¶](09_04_compiling_to_the_virtual_machine.md)

Load a procedure with the given label into local dst. This macro-instruction should only be used with procedures without free variables – procedures that are not closures.

Scheme Procedure: **emit-begin-standard-arity** asm req nlocals alternate [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Procedure: **emit-begin-opt-arity** asm req opt rest nlocals alternate [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Procedure: **emit-begin-kw-arity** asm req opt rest kw-indices allow-other-keys? nlocals alternate [¶](09_04_compiling_to_the_virtual_machine.md)

Scheme Procedure: **emit-end-arity** asm [¶](09_04_compiling_to_the_virtual_machine.md)

Delimit a clause of a procedure.

The linker is a complicated beast. Hackers interested in how it works would do well do read Ian Lance Taylor’s series of articles on linkers. Searching the internet should find them easily. From the user’s perspective, there is only one knob to control: whether the resulting image will be written out to a file or not. If the user passes `#:to-file? #t` as part of the compiler options (see [The Scheme Compiler](09_04_compiling_to_the_virtual_machine.md#942-the-scheme-compiler)), the linker will align the resulting segments on page boundaries, and otherwise not.

Scheme Procedure: **link-assembly** asm #:page-aligned?=#t [¶](09_04_compiling_to_the_virtual_machine.md)

Link an ELF image, and return the bytevector. If page-aligned? is true, Guile will align the segments with different permissions on page-sized boundaries, in order to maximize code sharing between different processes. Otherwise, padding is minimized, to minimize address space consumption.

To write an image to disk, just use `put-bytevector` from `(ice-9 binary-ports)`.

Compiling object code to the fake language, `value`, is performed via loading objcode into a program, then executing that thunk with respect to the compilation environment. Normally the environment propagates through the compiler transparently, but users may specify the compilation environment manually as well, as a module. Procedures to load images can be found in the `(system vm loader)` module:

([use-modules](06_18_modules.md) ([system](07_02_07_processes.md) vm loader))

Scheme Variable: **load-thunk-from-file** file [¶](09_04_compiling_to_the_virtual_machine.md)

C Function: **scm\_load\_thunk\_from\_file** (file) [¶](09_04_compiling_to_the_virtual_machine.md)

Load object code from a file named file. The file will be mapped into memory via `mmap`, so this is a very fast operation.

Scheme Variable: **load-thunk-from-memory** bv [¶](09_04_compiling_to_the_virtual_machine.md)

C Function: **scm\_load\_thunk\_from\_memory** (bv) [¶](09_04_compiling_to_the_virtual_machine.md)

Load object code from a bytevector. The data will be copied out of the bytevector in order to ensure proper alignment of embedded Scheme values.

Additionally there are procedures to find the ELF image for a given pointer, or to list all mapped ELF images:

Scheme Variable: **find-mapped-elf-image** ptr [¶](09_04_compiling_to_the_virtual_machine.md)

Given the integer value ptr, find and return the ELF image that contains that pointer, as a bytevector. If no image is found, return `#f`. This routine is mostly used by debuggers and other introspective tools.

Scheme Variable: **all-mapped-elf-images** [¶](09_04_compiling_to_the_virtual_machine.md)

Return all mapped ELF images, as a list of bytevectors.

* * *

Next: [Extending the Compiler](09_04_compiling_to_the_virtual_machine.md#947-extending-the-compiler), Previous: [Bytecode](09_04_compiling_to_the_virtual_machine.md#945-bytecode), Up: [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.6 Writing New High-Level Languages [¶](09_04_compiling_to_the_virtual_machine.md#946-writing-new-high-level-languages)

In order to integrate a new language lang into Guile’s compiler system, one has to create the module `(language lang spec)` containing the language definition and referencing the parser, compiler and other routines processing it. The module hierarchy in `(language brainfuck)` defines a very basic Brainfuck implementation meant to serve as easy-to-understand example on how to do this. See for instance [http://en.wikipedia.org/wiki/Brainfuck](http://en.wikipedia.org/wiki/Brainfuck) for more information about the Brainfuck language itself.

* * *

Previous: [Writing New High-Level Languages](09_04_compiling_to_the_virtual_machine.md#946-writing-new-high-level-languages), Up: [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.4.7 Extending the Compiler [¶](09_04_compiling_to_the_virtual_machine.md#947-extending-the-compiler)

At this point we take a detour from the impersonal tone of the rest of the manual. Admit it: if you’ve read this far into the compiler internals manual, you are a junkie. Perhaps a course at your university left you unsated, or perhaps you’ve always harbored a desire to hack the holy of computer science holies: a compiler. Well you’re in good company, and in a good position. Guile’s compiler needs your help.

There are many possible avenues for improving Guile’s compiler. Probably the most important improvement, speed-wise, will be some form of optimized ahead-of-time native compilation with global register allocation. A first pass could simply extend the compiler to also emit machine code in addition to bytecode, pre-filling the corresponding JIT data structures referenced by the `instrument-entry` bytecodes. See [Instrumentation Instructions](09_03_a_virtual_machine_for_guile.md#9376-instrumentation-instructions).

The compiler also needs help at the top end, adding new high-level compilers. We have JavaScript and Emacs Lisp mostly complete, but they could use some love; Lua would be nice as well, but whatever language it is that strikes your fancy would be welcome too.

Compilers are for hacking, not for admiring or for complaining about. Get to it!

* * *

Next: [Concept Index](index_concept.md#concept-index), Previous: [Guile Implementation](09_00_guile_implementation.md#9-guile-implementation), Up: [The Guile Reference Manual](00_contents.md)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
