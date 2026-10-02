### 6.26 Debugging Infrastructure [¶](06_26_debugging_infrastructure.md#626-debugging-infrastructure)

Guile provides facilities for simple print-based debugging as well as more advanced debugging features. In order to understand Guile’s advanced debugging facilities, one first must understand a little about how Guile represents the Scheme control stack. With that in place, we can explain the low level trap calls that the virtual machine can be configured to make, and the trap and breakpoint infrastructure that builds on top of those calls.

*   [Simple Debugging](06_26_debugging_infrastructure.md#6261-simple-debugging)
*   [Evaluation and the Scheme Stack](06_26_debugging_infrastructure.md#6262-evaluation-and-the-scheme-stack)
*   [Source Properties](06_26_debugging_infrastructure.md#6263-source-properties)
*   [Programmatic Error Handling](06_26_debugging_infrastructure.md#6264-programmatic-error-handling)
*   [Traps](06_26_debugging_infrastructure.md#6265-traps)
*   [GDB Support](06_26_debugging_infrastructure.md#6266-gdb-support)

* * *

Next: [Evaluation and the Scheme Stack](06_26_debugging_infrastructure.md#6262-evaluation-and-the-scheme-stack), Up: [Debugging Infrastructure](06_26_debugging_infrastructure.md#626-debugging-infrastructure)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.1 Simple Debugging [¶](06_26_debugging_infrastructure.md#6261-simple-debugging)

Guile offers powerful tools for introspection and debugging at the REPL, covered in the rest of this section and elsewhere in this manual (see [Interactive Debugging](04_programming_in_scheme.md#446-interactive-debugging)). Here we deal with a more primitive approach, commonly called “print debugging,” which is a quick way to diagnose simple errors by printing values during a program’s execution. Guile provides the `peek` procedure, more commonly known as `pk` (pronounced by naming the letters), as a convenient and powerful tool for this kind of debugging.

Scheme Procedure: **peek** stuff … [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **pk** stuff … [¶](06_26_debugging_infrastructure.md)

Print stuff to the current output port using `write`. Return the last argument.

`pk` improves on using `write` directly because it enables inspection of the state of code as it runs without breaking the normal code flow. It is also sometimes more practical than a full debugger because it does not require the program to be stopped for inspection. Here is a basic example:

(define fire 'burns)

([pk](06_26_debugging_infrastructure.md) fire)
⇒

;;; (burns)
burns

Here is an example of inspecting a value in the midst of code flow:

(map (lambda (v)
       (if ([number?](06_06_02_numerical_data_types.md) v)
           ([pk](06_26_debugging_infrastructure.md) 'number->string ([number->string](06_06_02_numerical_data_types.md) v))
           v))
     '(1 "2" "3" 4))
⇒

;;; ("1")
;;; ("4")
("1" "2" "3" "4")

A common technique when using `pk` is to label values with symbols to keep track of where they’re coming from. There’s no reason these labels need to be symbols; symbols are just convenient. Here’s a slightly more complex example demonstrating that pattern:

(define (pk-identity x)
  ([pk](06_26_debugging_infrastructure.md) 'arg-to-identity x))

(pk-identity 42)
⇒

;;; (arg-to-identity 42)
42

`pk` has one small quirk of note. Currently, it only returns the first value returned from any multi-value returns. So for example:

([pk](06_26_debugging_infrastructure.md) 'vals ([values](06_11_controlling_the_flow_of_program_execution.md) 1 2 3))
⇒

;;; (vals 1)
1

The way to get around this limitation is to bind such multi-value returns then inspect the results. Still, `pk` can only return a single value:

([use-modules](06_18_modules.md) (srfi srfi-11))

(let-values (((x y z)
              ([values](06_11_controlling_the_flow_of_program_execution.md) 1 2 3)))
  ([pk](06_26_debugging_infrastructure.md) 'vals x y z))
⇒

;;; (vals 1 2 3)
3

* * *

Next: [Source Properties](06_26_debugging_infrastructure.md#6263-source-properties), Previous: [Simple Debugging](06_26_debugging_infrastructure.md#6261-simple-debugging), Up: [Debugging Infrastructure](06_26_debugging_infrastructure.md#626-debugging-infrastructure)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.2 Evaluation and the Scheme Stack [¶](06_26_debugging_infrastructure.md#6262-evaluation-and-the-scheme-stack)

The idea of the Scheme stack is central to a lot of debugging. The Scheme stack is a reified representation of the pending function returns in an expression’s continuation. As Guile implements function calls using a stack, this reification takes the form of a number of nested stack frames, each of which corresponds to the application of a procedure to a set of arguments.

A Scheme stack always exists implicitly, and can be summoned into concrete existence as a first-class Scheme value by the `make-stack` call, so that an introspective Scheme program – such as a debugger – can present it in some way and allow the user to query its details. The first thing to understand, therefore, is how Guile’s function call convention creates the stack.

Broadly speaking, Guile represents all control flow on a stack. Calling a function involves pushing an empty frame on the stack, then evaluating the procedure and its arguments, then fixing up the new frame so that it points to the old one. Frames on the stack are thus linked together. A tail call is the same, except it reuses the existing frame instead of pushing on a new one.

In this way, the only frames that are on the stack are “active” frames, frames which need to do some work before the computation is complete. On the other hand, a function that has tail-called another function will not be on the stack, as it has no work left to do.

Therefore, when an error occurs in a running program, or the program hits a breakpoint, or in fact at any point that the programmer chooses, its state at that point can be represented by a _stack_ of all the procedure applications that are logically in progress at that time, each of which is known as a _frame_. The programmer can learn more about the program’s state at that point by inspecting the stack and its frames.

*   [Stack Capture](06_26_debugging_infrastructure.md#62621-stack-capture)
*   [Stacks](06_26_debugging_infrastructure.md#62622-stacks)
*   [Frames](06_26_debugging_infrastructure.md#62623-frames)

* * *

Next: [Stacks](06_26_debugging_infrastructure.md#62622-stacks), Up: [Evaluation and the Scheme Stack](06_26_debugging_infrastructure.md#6262-evaluation-and-the-scheme-stack)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.2.1 Stack Capture [¶](06_26_debugging_infrastructure.md#62621-stack-capture)

A Scheme program can use the `make-stack` primitive anywhere in its code, with first arg `#t`, to construct a Scheme value that describes the Scheme stack at that point.

([make-stack](06_26_debugging_infrastructure.md) #t)
⇒
#<stack 25205a0>

Use `start-stack` to limit the stack extent captured by future `make-stack` calls.

Scheme Procedure: **make-stack** obj arg … [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_make\_stack** (obj, args) [¶](06_26_debugging_infrastructure.md)

Create a new stack. If obj is `#t`, the current evaluation stack is used for creating the stack frames, otherwise the frames are taken from obj (which must be a continuation or a frame object).

arg … can be any combination of integer, procedure, address range, and prompt tag values.

These values specify various ways of cutting away uninteresting stack frames from the top and bottom of the stack that `make-stack` returns. They come in pairs like this: `(inner_cut_1 outer_cut_1 inner_cut_2 outer_cut_2 …)`.

Each inner\_cut\_i can be an integer, a procedure, an address range, or a prompt tag. An integer means to cut away exactly that number of frames. A procedure means to cut away all frames up to but excluding the frame whose procedure matches the specified one. An address range is a pair of integers indicating the low and high addresses of a procedure’s code, and is the same as cutting away to a procedure (though with less work). Anything else is interpreted as a prompt tag which cuts away all frames that are inside a prompt with the given tag.

Each outer\_cut\_i can likewise be an integer, a procedure, an address range, or a prompt tag. An integer means to cut away that number of frames. A procedure means to cut away frames down to but excluding the frame whose procedure matches the specified one. An address range is the same, but with the procedure’s code specified as an address range. Anything else is taken to be a prompt tag, which cuts away all frames that are outside a prompt with the given tag.

If the outer\_cut\_i of the last pair is missing, it is taken as 0.

Scheme Syntax: **start-stack** id exp [¶](06_26_debugging_infrastructure.md)

Evaluate exp on a new calling stack with identity id. If exp is interrupted during evaluation, backtraces will not display frames farther back than exp’s top-level form. This macro is a way of artificially limiting backtraces and stack procedures, largely as a convenience to the user.

* * *

Next: [Frames](06_26_debugging_infrastructure.md#62623-frames), Previous: [Stack Capture](06_26_debugging_infrastructure.md#62621-stack-capture), Up: [Evaluation and the Scheme Stack](06_26_debugging_infrastructure.md#6262-evaluation-and-the-scheme-stack)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.2.2 Stacks [¶](06_26_debugging_infrastructure.md#62622-stacks)

Scheme Procedure: **stack?** obj [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_stack\_p** (obj) [¶](06_26_debugging_infrastructure.md)

Return `#t` if obj is a calling stack.

Scheme Procedure: **stack-id** stack [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_stack\_id** (stack) [¶](06_26_debugging_infrastructure.md)

Return the identifier given to stack by `start-stack`.

Scheme Procedure: **stack-length** stack [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_stack\_length** (stack) [¶](06_26_debugging_infrastructure.md)

Return the length of stack.

Scheme Procedure: **stack-ref** stack index [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_stack\_ref** (stack, index) [¶](06_26_debugging_infrastructure.md)

Return the index’th frame from stack.

Scheme Procedure: **display-backtrace** stack port \[first \[depth \[highlights\]\]\] [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_display\_backtrace\_with\_highlights** (stack, port, first, depth, highlights) [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_display\_backtrace** (stack, port, first, depth) [¶](06_26_debugging_infrastructure.md)

Display a backtrace to the output port port. stack is the stack to take the backtrace from, first specifies where in the stack to start and depth how many frames to display. first and depth can be `#f`, which means that default values will be used. If highlights is given it should be a list; the elements of this list will be highlighted wherever they appear in the backtrace.

* * *

Previous: [Stacks](06_26_debugging_infrastructure.md#62622-stacks), Up: [Evaluation and the Scheme Stack](06_26_debugging_infrastructure.md#6262-evaluation-and-the-scheme-stack)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.2.3 Frames [¶](06_26_debugging_infrastructure.md#62623-frames)

Scheme Procedure: **frame?** obj [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_frame\_p** (obj) [¶](06_26_debugging_infrastructure.md)

Return `#t` if obj is a stack frame.

Scheme Procedure: **frame-previous** frame [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_frame\_previous** (frame) [¶](06_26_debugging_infrastructure.md)

Return the previous frame of frame, or `#f` if frame is the first frame in its stack.

Scheme Procedure: **frame-procedure-name** frame [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_frame\_procedure\_name** (frame) [¶](06_26_debugging_infrastructure.md)

Return the name of the procedure being applied in frame, as a symbol, or `#f` if the procedure has no name.

Scheme Procedure: **frame-arguments** frame [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_frame\_arguments** (frame) [¶](06_26_debugging_infrastructure.md)

Return the arguments of frame.

Scheme Procedure: **frame-address** frame [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **frame-instruction-pointer** frame [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **frame-stack-pointer** frame [¶](06_26_debugging_infrastructure.md)

Accessors for the three VM registers associated with this frame: the frame pointer (fp), instruction pointer (ip), and stack pointer (sp), respectively. See [VM Concepts](09_03_a_virtual_machine_for_guile.md#932-vm-concepts), for more information.

Scheme Procedure: **frame-dynamic-link** frame [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **frame-return-address** frame [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **frame-mv-return-address** frame [¶](06_26_debugging_infrastructure.md)

Accessors for the three saved VM registers in a frame: the previous frame pointer, the single-value return address, and the multiple-value return address. See [Stack Layout](09_03_a_virtual_machine_for_guile.md#933-stack-layout), for more information.

Scheme Procedure: **frame-bindings** frame [¶](06_26_debugging_infrastructure.md)

Return a list of binding records indicating the local variables that are live in a frame.

Scheme Procedure: **frame-lookup-binding** frame var [¶](06_26_debugging_infrastructure.md)

Fetch the bindings in frame, and return the first one whose name is var, or `#f` otherwise.

Scheme Procedure: **binding-index** binding [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **binding-name** binding [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **binding-slot** binding [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **binding-representation** binding [¶](06_26_debugging_infrastructure.md)

Accessors for the various fields in a binding. The implicit “callee” argument is index 0, the first argument is index 1, and so on to the end of the arguments. After that are temporary variables. Note that if a variable is dead, it might not be available.

Scheme Procedure: **binding-ref** binding [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **binding-set!** binding val [¶](06_26_debugging_infrastructure.md)

Accessors for the values of local variables in a frame.

Scheme Procedure: **display-application** frame \[port \[indent\]\] [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_display\_application** (frame, port, indent) [¶](06_26_debugging_infrastructure.md)

Display a procedure application frame to the output port port. indent specifies the indentation of the output.

Additionally, the `(system vm frame)` module defines a number of higher-level introspective procedures, for example to retrieve the names of local variables, and the source location to correspond to a frame. See its source code for more details.

* * *

Next: [Programmatic Error Handling](06_26_debugging_infrastructure.md#6264-programmatic-error-handling), Previous: [Evaluation and the Scheme Stack](06_26_debugging_infrastructure.md#6262-evaluation-and-the-scheme-stack), Up: [Debugging Infrastructure](06_26_debugging_infrastructure.md#626-debugging-infrastructure)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.3 Source Properties [¶](06_26_debugging_infrastructure.md#6263-source-properties)

How best to associate source locations with datums parsed from a port? The right way to do this is to annotate all components of each parsed datum. See [Reading Scheme Code, For the Compiler](06_16_reading_and_evaluating_scheme_code.md#6163-reading-scheme-code-for-the-compiler), for more on `read-syntax`.

Guile only switched to use `read-syntax` in 2021, however. For the previous thirty years, it used a mechanism known as _source properties_.

As Guile reads in Scheme code from file or from standard input, it can record the file name, line number and column number where each expression begins in a side table.

The way that this side table associates datums with source properties has a limitation, however: Guile can only associate source properties with freshly allocated objects. This notably excludes individual symbols, keywords, characters, booleans, or small integers. This limitation finally motivated the switch to `read-syntax`.

Scheme Procedure: **supports-source-properties?** obj [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_supports\_source\_properties\_p** (obj) [¶](06_26_debugging_infrastructure.md)

Return #t if source properties can be associated with obj, otherwise return #f.

The recording of source properties is controlled by the read option named “positions” (see [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code)). This option is switched _on_ by default. Now that `read-syntax` is available, however, Guile may change the default for this flag to off in the future.

The following procedures can be used to access and set the source properties of read expressions.

Scheme Procedure: **set-source-properties!** obj alist [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_set\_source\_properties\_x** (obj, alist) [¶](06_26_debugging_infrastructure.md)

Install the association list alist as the source property list for obj.

Scheme Procedure: **set-source-property!** obj key datum [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_set\_source\_property\_x** (obj, key, datum) [¶](06_26_debugging_infrastructure.md)

Set the source property of object obj, which is specified by key to datum. Normally, the key will be a symbol.

Scheme Procedure: **source-properties** obj [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_source\_properties** (obj) [¶](06_26_debugging_infrastructure.md)

Return the source property association list of obj.

Scheme Procedure: **source-property** obj key [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_source\_property** (obj, key) [¶](06_26_debugging_infrastructure.md)

Return the property specified by key from obj’s source properties.

If the `positions` reader option is enabled, supported expressions will have values set for the `filename`, `line` and `column` properties.

Source properties are also associated with syntax objects. Procedural macros can get at the source location of their input using the `syntax-source` accessor. See [Syntax Transformer Helpers](06_08_macros.md#684-syntax-transformer-helpers), for more.

Guile also defines a couple of convenience macros built on `syntax-source`:

Scheme Syntax: **current-source-location** [¶](06_26_debugging_infrastructure.md)

Expands to the source properties corresponding to the location of the `(current-source-location)` form.

Scheme Syntax: **current-filename** [¶](06_26_debugging_infrastructure.md)

Expands to the current filename: the filename that the `(current-filename)` form appears in. Expands to `#f` if this information is unavailable.

If you’re stuck with defmacros (see [Lisp-style Macro Definitions](06_08_macros.md#685-lisp-style-macro-definitions)), and want to preserve source information, the following helper function might be useful to you:

Scheme Procedure: **cons-source** xorig x y [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_cons\_source** (xorig, x, y) [¶](06_26_debugging_infrastructure.md)

Create and return a new pair whose car and cdr are x and y. Any source properties associated with xorig are also associated with the new pair.

* * *

Next: [Traps](06_26_debugging_infrastructure.md#6265-traps), Previous: [Source Properties](06_26_debugging_infrastructure.md#6263-source-properties), Up: [Debugging Infrastructure](06_26_debugging_infrastructure.md#626-debugging-infrastructure)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.4 Programmatic Error Handling [¶](06_26_debugging_infrastructure.md#6264-programmatic-error-handling)

For better or for worse, all programs have bugs, and dealing with bugs is part of programming. This section deals with that class of bugs that causes an exception to be raised – from your own code, from within a library, or from Guile itself.

*   [Catching Exceptions](06_26_debugging_infrastructure.md#62641-catching-exceptions)
*   [Pre-Unwind Debugging](06_26_debugging_infrastructure.md#62642-pre-unwind-debugging)
*   [call-with-error-handling](06_26_debugging_infrastructure.md#62643-call-with-error-handling)
*   [Stack Overflow](06_26_debugging_infrastructure.md#62644-stack-overflow)
*   [Debug options](06_26_debugging_infrastructure.md#62645-debug-options)

* * *

Next: [Pre-Unwind Debugging](06_26_debugging_infrastructure.md#62642-pre-unwind-debugging), Up: [Programmatic Error Handling](06_26_debugging_infrastructure.md#6264-programmatic-error-handling)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.4.1 Catching Exceptions [¶](06_26_debugging_infrastructure.md#62641-catching-exceptions)

A common requirement is to be able to show as much useful context as possible when a Scheme program hits an error. The most immediate information about an error is the kind of error that it is – such as “division by zero” – and any parameters that the code which signaled the error chose explicitly to provide. This information originates with the `error` or `raise-exception` call (or their C code equivalents, if the error is detected by C code) that signals the error, and is passed automatically to the handler procedure of the innermost applicable exception handler.

Therefore, to catch errors that occur within a chunk of Scheme code, and to intercept basic information about those errors, you need to execute that code inside the dynamic context of a `with-exception-handler`, or the equivalent in C.

For example, to print out a message and return #f when an error occurs, you might use:

(define (catch-all thunk)
  ([with-exception-handler](06_11_controlling_the_flow_of_program_execution.md)
    (lambda (exn)
      ([format](07_05_20_srfi28_basic_format_strings.md) ([current-error-port](06_12_input_and_output.md))
              "Uncaught exception: ~s\\n" exn)
      #f)
    thunk
    #:unwind? #t))

(catch-all
 (lambda () ([error](04_programming_in_scheme.md) "Not a vegetable: tomato")))
⊣ Uncaught exception: #<&exception-with-kind-and-args ...>
⇒ #f

See [Exceptions](06_11_controlling_the_flow_of_program_execution.md#6118-exceptions), for full details.

* * *

Next: [call-with-error-handling](06_26_debugging_infrastructure.md#62643-call-with-error-handling), Previous: [Catching Exceptions](06_26_debugging_infrastructure.md#62641-catching-exceptions), Up: [Programmatic Error Handling](06_26_debugging_infrastructure.md#6264-programmatic-error-handling)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.4.2 Pre-Unwind Debugging [¶](06_26_debugging_infrastructure.md#62642-pre-unwind-debugging)

Sometimes when something goes wrong, what you want is not just a representation of the exceptional situation, but the context that brought about that situation. The example in the previous section passed `#:unwind #t` to `with-exception-handler`, indicating that `raise-exception` should unwind the stack before invoking the exception handler. However if you don’t take this approach and instead let the exception handler be invoked in the context of the `raise-exception`, you can print a backtrace, launch a recursive debugger, or take other “pre-unwind” actions.

The most basic idea would be to simply print a backtrace:

(define (call-with-backtrace thunk)
  (with-exception-handler
    (lambda (exn)
      (backtrace)
      (raise-exception exn))
    thunk))

Here we use the built-in `backtrace` procedure to print the backtrace.

Scheme Procedure: **backtrace** \[highlights\] [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_backtrace\_with\_highlights** (highlights) [¶](06_26_debugging_infrastructure.md)

C Function: **scm\_backtrace** () [¶](06_26_debugging_infrastructure.md)

Display a backtrace of the current stack to the current output port. If highlights is given it should be a list; the elements of this list will be highlighted wherever they appear in the backtrace.

By re-raising the exception, `call-with-backtrace` doesn’t actually handle the error. We could define a version that instead aborts the computation:

(use-modules (ice-9 control))
(define (call-with-backtrace thunk)
  (let/ec cancel
    (with-exception-handler
      (lambda (exn)
        (backtrace)
        (cancel #f))
      thunk)))

In this second example, we use an escape continuation to abort the computation after printing the backtrace, returning `#f` instead.

It could be that you want to only print a limited backtrace. In that case, use `start-stack`:

(use-modules (ice-9 control))
(define (call-with-backtrace thunk)
  (let/ec cancel
    (start-stack 'stack-with-backtrace
      (with-exception-handler
        (lambda (exn)
          (backtrace)
          (cancel #f))
        thunk))))

There are also more powerful, programmatic ways to walk the stack using `make-stack` and friends; see the API described in [Stacks](06_26_debugging_infrastructure.md#62622-stacks) and [Frames](06_26_debugging_infrastructure.md#62623-frames).

* * *

Next: [Stack Overflow](06_26_debugging_infrastructure.md#62644-stack-overflow), Previous: [Pre-Unwind Debugging](06_26_debugging_infrastructure.md#62642-pre-unwind-debugging), Up: [Programmatic Error Handling](06_26_debugging_infrastructure.md#6264-programmatic-error-handling)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.4.3 call-with-error-handling [¶](06_26_debugging_infrastructure.md#62643-call-with-error-handling)

The Guile REPL code (in system/repl/repl.scm and related files) uses a `catch` with a pre-unwind handler to capture the stack when an error occurs in an expression that was typed into the REPL, and debug that stack interactively in the context of the error.

These procedures are available for use by user programs, in the `(system repl error-handling)` module.

([use-modules](06_18_modules.md) ([system](07_02_07_processes.md) repl error-handling))

Scheme Procedure: **call-with-error-handling** thunk \[#:on-error on-error=’debug\] \[#:post-error post-error=’catch\] \[#:pass-keys pass-keys=’(quit)\] \[#:report-keys report-keys=’(stack-overflow)\] \[#:trap-handler trap-handler=’debug\] [¶](06_26_debugging_infrastructure.md)

Call a thunk in a context in which errors are handled.

Note that this function was written when `throw`/`catch` were the fundamental exception handling primitives in Guile, and so exposes some aspects of that interface (notably in the form of the procedural handlers). Guile will probably replace this function with a `call-with-standard-exception-handling` in the future.

There are five keyword arguments:

on-error

Specifies what to do before the stack is unwound.

Valid options are `debug` (the default), which will enter a debugger; `pass`, in which case nothing is done, and the exception is rethrown; or a procedure, which will be the pre-unwind handler.

post-error

Specifies what to do after the stack is unwound.

Valid options are `catch` (the default), which will silently catch errors, returning the unspecified value; `report`, which prints out a description of the error (via `display-error`), and then returns the unspecified value; or a procedure, which will be the catch handler.

trap-handler

Specifies a trap handler: what to do when a breakpoint is hit.

Valid options are `debug`, which will enter the debugger; `pass`, which does nothing; or `disabled`, which disables traps entirely. See [Traps](06_26_debugging_infrastructure.md#6265-traps), for more information.

pass-keys

A set of keys to ignore, as a list.

report-keys

A set of keys to always report even if the post-error handler is `catch`, as a list.

* * *

Next: [Debug options](06_26_debugging_infrastructure.md#62645-debug-options), Previous: [call-with-error-handling](06_26_debugging_infrastructure.md#62643-call-with-error-handling), Up: [Programmatic Error Handling](06_26_debugging_infrastructure.md#6264-programmatic-error-handling)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.4.4 Stack Overflow [¶](06_26_debugging_infrastructure.md#62644-stack-overflow)

Every time a Scheme program makes a call that is not in tail position, it pushes a new frame onto the stack. Returning a value from a function pops the top frame off the stack. Stack frames take up memory, and as nobody has an infinite amount of memory, deep recursion could cause Guile to run out of memory. Running out of stack memory is called _stack overflow_.

#### Stack Limits [¶](06_26_debugging_infrastructure.md#stack-limits)

Most languages have a terrible stack overflow story. For example, in C, if you use too much stack, your program will exhibit “undefined behavior”, which if you are lucky means that it will crash. It’s especially bad in C, as you neither know ahead of time how much stack your functions use, nor the stack limit imposed by the user’s system, and the stack limit is often quite small relative to the total memory size.

Managed languages like Python have a better error story, as they are defined to raise an exception on stack overflow – but like C, Python and most dynamic languages still have a fixed stack size limit that is usually much smaller than the heap.

Arbitrary stack limits would have an unfortunate effect on Guile programs. For example, the following implementation of the inner loop of `map` is clean and elegant:

(define (map f l)
  (if (pair? l)
      (cons (f (car l))
            (map f (cdr l)))
      '()))

However, if there were a stack limit, that would limit the size of lists that can be processed with this `map`. Eventually, you would have to rewrite it to use iteration with an accumulator:

(define (map f l)
  (let lp ((l l) (out '()))
    (if (pair? l)
        (lp (cdr l) (cons (f (car l)) out))
        (reverse out))))

This second version is sadly not as clear, and it also allocates more heap memory (once to build the list in reverse, and then again to reverse the list). You would be tempted to use the destructive `reverse!` to save memory and time, but then your code would not be continuation-safe – if f returned again after the map had finished, it would see an out list that had already been reversed. The recursive `map` has none of these problems.

Guile has no stack limit for Scheme code. When a thread makes its first Guile call, a small stack is allocated – just one page of memory. Whenever that memory limit would be reached, Guile arranges to grow the stack by a factor of two. When garbage collection happens, Guile arranges to return the unused part of the stack to the operating system, but without causing the stack to shrink. In this way, the stack can grow to consume up to all memory available to the Guile process, and when the recursive computation eventually finishes, that stack memory is returned to the system.

#### Exceptional Situations [¶](06_26_debugging_infrastructure.md#exceptional-situations)

Of course, it’s still possible to run out of stack memory. The most common cause of this is program bugs that cause unbounded recursion, as in:

(define (faulty-map f l)
  (if (pair? l)
      (cons (f (car l)) (faulty-map f l))
      '()))

Did you spot the bug? The recursive call to `faulty-map` recursed on l, not `(cdr l)`. Running this program would cause Guile to use up all memory in your system, and eventually Guile would fail to grow the stack. At that point you have a problem: Guile needs to raise an exception to unwind the stack and return memory to the system, but the user might have exception handlers in place (see [Raising and Handling Exceptions](06_11_controlling_the_flow_of_program_execution.md#61182-raising-and-handling-exceptions)) that want to run before the stack is unwound, and we don’t have any stack in which to run them.

Therefore in this case, Guile raises an unwind-only exception that does not run pre-unwind handlers. Because this is such an odd case, Guile prints out a message on the console, in case the user was expecting to be able to get a backtrace from any pre-unwind handler.

#### Runaway Recursion [¶](06_26_debugging_infrastructure.md#runaway-recursion)

Still, this failure mode is not so nice. If you are running an environment in which you are interactively building a program while it is running, such as at a REPL, you might want to impose an artificial stack limit on the part of your program that you are building to detect accidental runaway recursion. For that purpose, there is `call-with-stack-overflow-handler`, from `(system vm vm)`.

(use-module (system vm vm))

Scheme Procedure: **call-with-stack-overflow-handler** limit thunk handler [¶](06_26_debugging_infrastructure.md)

Call thunk in an environment in which the stack limit has been reduced to limit additional words. If the limit is reached, handler (a thunk) will be invoked in the dynamic environment of the error. For the extent of the call to handler, the stack limit and handler are restored to the values that were in place when `call-with-stack-overflow-handler` was called.

Usually, handler should raise an exception or abort to an outer prompt. However if handler does return, it should return a number of additional words of stack space to allow to the inner environment.

A stack overflow handler may only ever “credit” the inner thunk with stack space that was available when the handler was instated. When Guile first starts, there is no stack limit in place, so the outer handler may allow the inner thunk an arbitrary amount of space, but any nested stack overflow handler will not be able to consume more than its limit.

Unlike the unwind-only exception that is thrown if Guile is unable to grow its stack, any exception thrown by a stack overflow handler might invoke pre-unwind handlers. Indeed, the stack overflow handler is itself a pre-unwind handler of sorts. If the code imposing the stack limit wants to protect itself against malicious pre-unwind handlers from the inner thunk, it should abort to a prompt of its own making instead of throwing an exception that might be caught by the inner thunk.

#### C Stack Usage [¶](06_26_debugging_infrastructure.md#c-stack-usage)

It is also possible for Guile to run out of space on the C stack. If you call a primitive procedure which then calls a Scheme procedure in a loop, you will consume C stack space. Guile tries to detect excessive consumption of C stack space, throwing an error when you have hit 80% of the process’ available stack (as allocated by the operating system), or 160 kilowords in the absence of a strict limit.

For example, looping through `call-with-vm`, a primitive that calls a thunk, gives us the following:

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([use-modules](06_18_modules.md) ([system](07_02_07_processes.md) vm vm))
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) (let lp () (call-with-vm lp))
ERROR: Stack overflow

Unfortunately, that’s all the information we get. Overrunning the C stack will throw an unwind-only exception, because it’s not safe to do very much when you are close to the C stack limit.

If you get an error like this, you can either try rewriting your code to use less stack space, or increase the maximum stack size. To increase the maximum stack size, use `debug-set!`, for example:

([debug-set!](06_26_debugging_infrastructure.md) stack 200000)

The next section describes `debug-set!` more thoroughly. Of course the best thing is to have your code operate without so much resource consumption by avoiding loops through C trampolines.

* * *

Previous: [Stack Overflow](06_26_debugging_infrastructure.md#62644-stack-overflow), Up: [Programmatic Error Handling](06_26_debugging_infrastructure.md#6264-programmatic-error-handling)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.4.5 Debug options [¶](06_26_debugging_infrastructure.md#62645-debug-options)

The behavior of the `backtrace` procedure and of the default error handler can be parameterized via the debug options.

Scheme Procedure: **debug-options** \[setting\] [¶](06_26_debugging_infrastructure.md)

Display the current settings of the debug options. If setting is omitted, only a short form of the current read options is printed. Otherwise if setting is the symbol `help`, a complete options description is displayed.

The set of available options, and their default values, may be had by invoking `debug-options` at the prompt.

scheme@(guile-user)>
backwards       no      Display backtrace in anti-chronological order.
width           79      Maximal width of backtrace.
depth           20      Maximal length of printed backtrace.
backtrace       yes     Show backtrace on error.
stack           1048576 Stack size limit (measured in words;
                        0 = no check). 
show-file-name  #t      Show file names and line numbers in backtraces
                        when not \`#f'.  A value of \`base' displays only
                        base names, while \`#t' displays full names. 
warn-deprecated no      Warn when deprecated features are used.

The boolean options may be toggled with `debug-enable` and `debug-disable`. The non-boolean options must be set using `debug-set!`.

Scheme Procedure: **debug-enable** option-name [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **debug-disable** option-name [¶](06_26_debugging_infrastructure.md)

Scheme Syntax: **debug-set!** option-name value [¶](06_26_debugging_infrastructure.md)

Modify the debug options. `debug-enable` should be used with boolean options and switches them on, `debug-disable` switches them off.

`debug-set!` can be used to set an option to a specific value. Due to historical oddities, it is a macro that expects an unquoted option name.

* * *

Next: [GDB Support](06_26_debugging_infrastructure.md#6266-gdb-support), Previous: [Programmatic Error Handling](06_26_debugging_infrastructure.md#6264-programmatic-error-handling), Up: [Debugging Infrastructure](06_26_debugging_infrastructure.md#626-debugging-infrastructure)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.5 Traps [¶](06_26_debugging_infrastructure.md#6265-traps)

Guile’s virtual machine can be configured to call out at key points to arbitrary user-specified procedures.

In principle, these _hooks_ allow Scheme code to implement any model it chooses for examining the evaluation stack as program execution proceeds, and for suspending execution to be resumed later.

VM hooks are very low-level, though, and so Guile also has a library of higher-level _traps_ on top of the VM hooks. A trap is an execution condition that, when fulfilled, will fire a handler. For example, Guile defines a trap that fires when control reaches a certain source location.

Finally, Guile also defines a third level of abstractions: per-thread _trap states_. A trap state exists to give names to traps, and to hold on to the set of traps so that they can be enabled, disabled, or removed. The trap state infrastructure defines the most useful abstractions for most cases. For example, Guile’s REPL uses trap state functions to set breakpoints and tracepoints.

The following subsections describe all this in detail, for both the user wanting to use traps, and the developer interested in understanding how the interface hangs together.

*   [VM Hooks](06_26_debugging_infrastructure.md#62651-vm-hooks)
*   [Trap Interface](06_26_debugging_infrastructure.md#62652-trap-interface)
*   [Low-Level Traps](06_26_debugging_infrastructure.md#62653-low-level-traps)
*   [Tracing Traps](06_26_debugging_infrastructure.md#62654-tracing-traps)
*   [Trap States](06_26_debugging_infrastructure.md#62655-trap-states)
*   [High-Level Traps](06_26_debugging_infrastructure.md#62656-high-level-traps)

* * *

Next: [Trap Interface](06_26_debugging_infrastructure.md#62652-trap-interface), Up: [Traps](06_26_debugging_infrastructure.md#6265-traps)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.5.1 VM Hooks [¶](06_26_debugging_infrastructure.md#62651-vm-hooks)

Everything that runs in Guile runs on its virtual machine, a C program that defines a number of operations that Scheme programs can perform.

Note that there are multiple VM “engines” for Guile. Only some of them have support for hooks compiled in. Normally the deal is that you get hooks if you are running interactively, and otherwise they are disabled, as they do have some overhead (about 10 or 20 percent).

To ensure that you are running with hooks, pass `--debug` to Guile when running your program, or otherwise use the `call-with-vm` and `set-vm-engine!` procedures to ensure that you are running in a VM with the `debug` engine.

To digress, Guile’s VM has 4 different hooks that can be fired at different times. For implementation reasons, these hooks are not actually implemented with first-class Scheme hooks (see [Hooks](06_09_general_utility_functions.md#696-hooks)); they are managed using an ad-hoc interface.

VM hooks are called with one argument: the current frame. See [Frames](06_26_debugging_infrastructure.md#62623-frames). Since these hooks may be fired very frequently, Guile does a terrible thing: it allocates the frames on the C stack instead of the garbage-collected heap.

The upshot here is that the frames are only valid within the dynamic extent of the call to the hook. If a hook procedure keeps a reference to the frame outside the extent of the hook, bad things will happen.

The interface to hooks is provided by the `(system vm vm)` module:

(use-modules (system vm vm))

All of these functions implicitly act on the VM for the current thread only.

Scheme Procedure: **vm-add-next-hook!** f [¶](06_26_debugging_infrastructure.md)

Arrange to call f when before an instruction is retired (and executed).

Scheme Procedure: **vm-add-apply-hook!** f [¶](06_26_debugging_infrastructure.md)

Arrange to call f whenever a procedure is applied. The frame locals will be the callee, followed by the arguments to the call.

Note that procedure application is somewhat orthogonal to continuation pushes and pops. To know whether a call is a tail call or not, with respect to the frame previously in place, check the value of the frame pointer compared the previous frame pointer.

Scheme Procedure: **vm-add-return-hook!** f [¶](06_26_debugging_infrastructure.md)

Arrange to call f before returning from a frame. The values in the frame will be the frame’s return values.

Note that it’s possible to return from an “inner” frame: one that was not immediately proceeded by a call with that frame pointer. In that case, it corresponds to a non-local control flow jump, either because of applying a composable continuation or because of restoring a saved undelimited continuation.

Scheme Procedure: **vm-add-abort-hook!** [¶](06_26_debugging_infrastructure.md)

Arrange to call f after aborting to a prompt. See [Prompts](06_11_controlling_the_flow_of_program_execution.md#6115-prompts).

Unfortunately, the values passed to the prompt handler are not easily available to f.

Scheme Procedure: **vm-remove-next-hook!** f [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **vm-remove-apply-hook!** f [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **vm-remove-return-hook!** f [¶](06_26_debugging_infrastructure.md)

Scheme Procedure: **vm-remove-abort-hook!** f [¶](06_26_debugging_infrastructure.md)

Remove f from the corresponding VM hook for the current thread.

These hooks do impose a performance penalty, if they are on. Obviously, the `vm-next-hook` has quite an impact, performance-wise. Therefore Guile exposes a single, heavy-handed knob to turn hooks on or off, the _VM trace level_. If the trace level is positive, hooks run; otherwise they don’t.

For convenience, when the VM fires a hook, it does so with the trap level temporarily set to 0. That way the hooks don’t fire while you’re handling a hook. The trace level is restored to whatever it was once the hook procedure finishes.

Scheme Procedure: **vm-trace-level** [¶](06_26_debugging_infrastructure.md)

Retrieve the “trace level” of the VM. If positive, the trace hooks associated with vm will be run. The initial trace level is 0.

Scheme Procedure: **set-vm-trace-level!** level [¶](06_26_debugging_infrastructure.md)

Set the “trace level” of the VM.

See [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile), for more information on Guile’s virtual machine.

* * *

Next: [Low-Level Traps](06_26_debugging_infrastructure.md#62653-low-level-traps), Previous: [VM Hooks](06_26_debugging_infrastructure.md#62651-vm-hooks), Up: [Traps](06_26_debugging_infrastructure.md#6265-traps)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.5.2 Trap Interface [¶](06_26_debugging_infrastructure.md#62652-trap-interface)

The capabilities provided by hooks are great, but hooks alone rarely correspond to what users want to do.

For example, if a user wants to break when and if control reaches a certain source location, how do you do it? If you install a “next” hook, you get unacceptable overhead for the execution of the entire program. It would be possible to install an “apply” hook, then if the procedure encompasses those source locations, install a “next” hook, but already you’re talking about one concept that might be implemented by a varying number of lower-level concepts.

It’s best to be clear about things and define one abstraction for all such conditions: the _trap_.

Considering the myriad capabilities offered by the hooks though, there is only a minimum of functionality shared by all traps. Guile’s current take is to reduce this to the absolute minimum, and have the only standard interface of a trap be “turn yourself on” or “turn yourself off”.

This interface sounds a bit strange, but it is useful to procedurally compose higher-level traps from lower-level building blocks. For example, Guile defines a trap that calls one handler when control enters a procedure, and another when control leaves the procedure. Given that trap, one can define a trap that adds to the next-hook only when within a given procedure. Building further, one can define a trap that fires when control reaches particular instructions within a procedure.

Or of course you can stop at any of these intermediate levels. For example, one might only be interested in calls to a given procedure. But the point is that a simple enable/disable interface is all the commonality that exists between the various kinds of traps, and furthermore that such an interface serves to allow “higher-level” traps to be composed from more primitive ones.

Specifically, a trap, in Guile, is a procedure. When a trap is created, by convention the trap is enabled; therefore, the procedure that is the trap will, when called, disable the trap, and return a procedure that will enable the trap, and so on.

Trap procedures take one optional argument: the current frame. (A trap may want to add to different sets of hooks depending on the frame that is current at enable-time.)

If this all sounds very complicated, it’s because it is. Some of it is essential, but probably most of it is not. The advantage of using this minimal interface is that composability is more lexically apparent than when, for example, using a stateful interface based on GOOPS. But perhaps this reflects the cognitive limitations of the programmer who made the current interface more than anything else.

* * *

Next: [Tracing Traps](06_26_debugging_infrastructure.md#62654-tracing-traps), Previous: [Trap Interface](06_26_debugging_infrastructure.md#62652-trap-interface), Up: [Traps](06_26_debugging_infrastructure.md#6265-traps)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.5.3 Low-Level Traps [¶](06_26_debugging_infrastructure.md#62653-low-level-traps)

To summarize the last sections, traps are enabled or disabled, and when they are enabled, they add to various VM hooks.

Note, however, that _traps do not increase the VM trace level_. So if you create a trap, it will be enabled, but unless something else increases the VM’s trace level (see [VM Hooks](06_26_debugging_infrastructure.md#62651-vm-hooks)), the trap will not fire. It turns out that getting the VM trace level right is tricky without a global view of what traps are enabled. See [Trap States](06_26_debugging_infrastructure.md#62655-trap-states), for Guile’s answer to this problem.

Traps are created by calling procedures. Most of these procedures share a set of common keyword arguments, so rather than document them separately, we discuss them all together here:

`#:vm`

The VM to instrument. Defaults to the current thread’s VM.

`#:current-frame`

For traps that enable more hooks depending on their dynamic context, this argument gives the current frame that the trap is running in. Defaults to `#f`.

To have access to these procedures, you’ll need to have imported the `(system vm traps)` module:

([use-modules](06_18_modules.md) ([system](07_02_07_processes.md) vm traps))

Scheme Procedure: **trap-at-procedure-call** proc handler \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that calls handler when proc is applied.

Scheme Procedure: **trap-in-procedure** proc enter-handler exit-handler \[#:current-frame\] \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that calls enter-handler when control enters proc, and exit-handler when control leaves proc.

Control can enter a procedure via:

*   A procedure call.
*   A return to a procedure’s frame on the stack.
*   A continuation returning directly to an application of this procedure.

Control can leave a procedure via:

*   A normal return from the procedure.
*   An application of another procedure.
*   An invocation of a continuation.
*   An abort.

Scheme Procedure: **trap-instructions-in-procedure** proc next-handler exit-handler \[#:current-frame\] \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that calls next-handler for every instruction executed in proc, and exit-handler when execution leaves proc.

Scheme Procedure: **trap-at-procedure-ip-in-range** proc range handler \[#:current-frame\] \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that calls handler when execution enters a range of instructions in proc. range is a simple of pairs, `((start . end) ...)`. The start addresses are inclusive, and end addresses are exclusive.

Scheme Procedure: **trap-at-source-location** file user-line handler \[#:current-frame\] \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that fires when control reaches a given source location. The user-line parameter is one-indexed, as a user counts lines, instead of zero-indexed, as Guile counts lines.

Scheme Procedure: **trap-frame-finish** frame return-handler abort-handler \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that fires when control leaves the given frame. frame should be a live frame in the current continuation. return-handler will be called on a normal return, and abort-handler on a nonlocal exit.

Scheme Procedure: **trap-in-dynamic-extent** proc enter-handler return-handler abort-handler \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A more traditional dynamic-wind trap, which fires enter-handler when control enters proc, return-handler on a normal return, and abort-handler on a nonlocal exit.

Note that rewinds are not handled, so there is no rewind handler.

Scheme Procedure: **trap-calls-in-dynamic-extent** proc apply-handler return-handler \[#:current-frame\] \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that calls apply-handler every time a procedure is applied, and return-handler for returns, but only during the dynamic extent of an application of proc.

Scheme Procedure: **trap-instructions-in-dynamic-extent** proc next-handler \[#:current-frame\] \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that calls next-handler for all retired instructions within the dynamic extent of a call to proc.

Scheme Procedure: **trap-calls-to-procedure** proc apply-handler return-handler \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that calls apply-handler whenever proc is applied, and return-handler when it returns, but with an additional argument, the call depth.

That is to say, the handlers will get two arguments: the frame in question, and the call depth (a non-negative integer).

Scheme Procedure: **trap-matching-instructions** frame-pred handler \[#:vm\] [¶](06_26_debugging_infrastructure.md)

A trap that calls frame-pred at every instruction, and if frame-pred returns a true value, calls handler on the frame.

* * *

Next: [Trap States](06_26_debugging_infrastructure.md#62655-trap-states), Previous: [Low-Level Traps](06_26_debugging_infrastructure.md#62653-low-level-traps), Up: [Traps](06_26_debugging_infrastructure.md#6265-traps)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.5.4 Tracing Traps [¶](06_26_debugging_infrastructure.md#62654-tracing-traps)

The `(system vm trace)` module defines a number of traps for tracing of procedure applications. When a procedure is _traced_, it means that every call to that procedure is reported to the user during a program run. The idea is that you can mark a collection of procedures for tracing, and Guile will subsequently print out a line of the form

|  |  ([procedure](06_07_procedures.md) args [...](06_08_macros.md))

whenever a marked procedure is about to be applied to its arguments. This can help a programmer determine whether a function is being called at the wrong time or with the wrong set of arguments.

In addition, the indentation of the output is useful for demonstrating how the traced applications are or are not tail recursive with respect to each other. Thus, a trace of a non-tail recursive factorial implementation looks like this:

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) (define (fact1 n) 
                       (if ([zero?](06_06_02_numerical_data_types.md) n) 1
                           ([\*](06_06_02_numerical_data_types.md) n (fact1 ([1-](06_06_02_numerical_data_types.md) n)))))
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ,trace (fact1 4)
trace: (fact1 4)
trace: |  (fact1 3)
trace: |  |  (fact1 2)
trace: |  |  |  (fact1 1)
trace: |  |  |  |  (fact1 0)
trace: |  |  |  |  1
trace: |  |  |  1
trace: |  |  2
trace: |  6
trace: 24

While a typical tail recursive implementation would look more like this:

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) (define (facti acc n)
                       (if ([zero?](06_06_02_numerical_data_types.md) n) acc
                           (facti ([\*](06_06_02_numerical_data_types.md) n acc) ([1-](06_06_02_numerical_data_types.md) n))))
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) (define (fact2 n) (facti 1 n))
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ,trace (fact2 4)
trace: (fact2 4)
trace: (facti 1 4)
trace: (facti 4 3)
trace: (facti 12 2)
trace: (facti 24 1)
trace: (facti 24 0)
trace: 24

The low-level traps below (see [Low-Level Traps](06_26_debugging_infrastructure.md#62653-low-level-traps)) share some common options:

`#:width`

The maximum width of trace output. Trace printouts will try not to exceed this column, but for highly nested procedure calls, it may be unavoidable. Defaults to 80.

`#:vm`

The VM on which to add the traps. Defaults to the current thread’s VM.

`#:prefix`

A string to print out before each trace line. As seen above in the examples, defaults to `"trace: "`.

To have access to these procedures, you’ll need to have imported the `(system vm trace)` module:

([use-modules](06_18_modules.md) ([system](07_02_07_processes.md) vm [trace](04_programming_in_scheme.md)))

Scheme Procedure: **trace-calls-to-procedure** proc \[#:width\] \[#:vm\] \[#:prefix\] [¶](06_26_debugging_infrastructure.md)

Print a trace at applications of and returns from proc.

Scheme Procedure: **trace-calls-in-procedure** proc \[#:width\] \[#:vm\] \[#:prefix\] [¶](06_26_debugging_infrastructure.md)

Print a trace at all applications and returns within the dynamic extent of calls to proc.

Scheme Procedure: **trace-instructions-in-procedure** proc \[#:width\] \[#:vm\] [¶](06_26_debugging_infrastructure.md)

Print a trace at all instructions executed in the dynamic extent of calls to proc.

In addition, Guile defines a procedure to call a thunk, tracing all procedure calls and returns within the thunk.

Scheme Procedure: **call-with-trace** thunk \[#:calls?=#t\] \[#:instructions?=#f\] \[#:width=80\] [¶](06_26_debugging_infrastructure.md)

Call thunk, tracing all execution within its dynamic extent.

If calls? is true, Guile will print a brief report at each procedure call and return, as given above.

If instructions? is true, Guile will also print a message each time an instruction is executed. This is a lot of output, but it is sometimes useful when doing low-level optimization.

Note that because this procedure manipulates the VM trace level directly, it doesn’t compose well with traps at the REPL.

See [Profile Commands](04_programming_in_scheme.md#4445-profile-commands), for more information on tracing at the REPL.

* * *

Next: [High-Level Traps](06_26_debugging_infrastructure.md#62656-high-level-traps), Previous: [Tracing Traps](06_26_debugging_infrastructure.md#62654-tracing-traps), Up: [Traps](06_26_debugging_infrastructure.md#6265-traps)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.5.5 Trap States [¶](06_26_debugging_infrastructure.md#62655-trap-states)

When multiple traps are present in a system, we begin to have a bookkeeping problem. How are they named? How does one disable, enable, or delete them?

Guile’s answer to this is to keep an implicit per-thread _trap state_. The trap state object is not exposed to the user; rather, API that works on trap states fetches the current trap state from the dynamic environment.

Traps are identified by integers. A trap can be enabled, disabled, or removed, and can have an associated user-visible name.

These procedures have their own module:

([use-modules](06_18_modules.md) ([system](07_02_07_processes.md) vm trap-state))

Scheme Procedure: **add-trap!** trap name [¶](06_26_debugging_infrastructure.md)

Add a trap to the current trap state, associating the given name with it. Returns a fresh trap identifier (an integer).

Note that usually the more specific functions detailed in [High-Level Traps](06_26_debugging_infrastructure.md#62656-high-level-traps) are used in preference to this one.

Scheme Procedure: **list-traps** [¶](06_26_debugging_infrastructure.md)

List the current set of traps, both enabled and disabled. Returns a list of integers.

Scheme Procedure: **trap-name** idx [¶](06_26_debugging_infrastructure.md)

Returns the name associated with trap idx, or `#f` if there is no such trap.

Scheme Procedure: **trap-enabled?** idx [¶](06_26_debugging_infrastructure.md)

Returns `#t` if trap idx is present and enabled, or `#f` otherwise.

Scheme Procedure: **enable-trap!** idx [¶](06_26_debugging_infrastructure.md)

Enables trap idx.

Scheme Procedure: **disable-trap!** idx [¶](06_26_debugging_infrastructure.md)

Disables trap idx.

Scheme Procedure: **delete-trap!** idx [¶](06_26_debugging_infrastructure.md)

Removes trap idx, disabling it first, if necessary.

* * *

Previous: [Trap States](06_26_debugging_infrastructure.md#62655-trap-states), Up: [Traps](06_26_debugging_infrastructure.md#6265-traps)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.5.6 High-Level Traps [¶](06_26_debugging_infrastructure.md#62656-high-level-traps)

The low-level trap API allows one to make traps that call procedures, and the trap state API allows one to keep track of what traps are there. But neither of these APIs directly helps you when you want to set a breakpoint, because it’s unclear what to do when the trap fires. Do you enter a debugger, or mail a summary of the situation to your great-aunt, or what?

So for the common case in which you just want to install breakpoints, and then have them all result in calls to one parameterizable procedure, we have the high-level trap interface.

Perhaps we should have started this section with this interface, as it’s clearly the one most people should use. But as its capabilities and limitations proceed from the lower layers, we felt that the character-building exercise of building a mental model might be helpful.

These procedures share a module with trap states:

([use-modules](06_18_modules.md) ([system](07_02_07_processes.md) vm trap-state))

Scheme Procedure: **with-default-trap-handler** handler thunk [¶](06_26_debugging_infrastructure.md)

Call thunk in a dynamic context in which handler is the current trap handler.

Additionally, during the execution of thunk, the VM trace level (see [VM Hooks](06_26_debugging_infrastructure.md#62651-vm-hooks)) is set to the number of enabled traps. This ensures that traps will in fact fire.

handler may be `#f`, in which case VM hooks are not enabled as they otherwise would be, as there is nothing to handle the traps.

The trace-level-setting behavior of `with-default-trap-handler` is one of its more useful aspects, but if you are willing to forgo that, and just want to install a global trap handler, there’s a function for that too:

Scheme Procedure: **install-trap-handler!** handler [¶](06_26_debugging_infrastructure.md)

Set the current thread’s trap handler to handler.

Trap handlers are called when traps installed by procedures from this module fire. The current “consumer” of this API is Guile’s REPL, but one might easily imagine other trap handlers being used to integrate with other debugging tools.

Scheme Procedure: **add-trap-at-procedure-call!** proc [¶](06_26_debugging_infrastructure.md)

Install a trap that will fire when proc is called.

This is a breakpoint.

Scheme Procedure: **add-trace-at-procedure-call!** proc [¶](06_26_debugging_infrastructure.md)

Install a trap that will print a tracing message when proc is called. See [Tracing Traps](06_26_debugging_infrastructure.md#62654-tracing-traps), for more information.

This is a tracepoint.

Scheme Procedure: **add-trap-at-source-location!** file user-line [¶](06_26_debugging_infrastructure.md)

Install a trap that will fire when control reaches the given source location. user-line is one-indexed, as users count lines, instead of zero-indexed, as Guile counts lines.

This is a source breakpoint.

Scheme Procedure: **add-ephemeral-trap-at-frame-finish!** frame handler [¶](06_26_debugging_infrastructure.md)

Install a trap that will call handler when frame finishes executing. The trap will be removed from the trap state after firing, or on nonlocal exit.

This is a finish trap, used to implement the “finish” REPL command.

Scheme Procedure: **add-ephemeral-stepping-trap!** frame handler \[#:into?\] \[#:instruction?\] [¶](06_26_debugging_infrastructure.md)

Install a trap that will call handler after stepping to a different source line or instruction. The trap will be removed from the trap state after firing, or on nonlocal exit.

If instruction? is false (the default), the trap will fire when control reaches a new source line. Otherwise it will fire when control reaches a new instruction.

Additionally, if into? is false (not the default), the trap will only fire for frames at or prior to the given frame. If into? is true (the default), the trap may step into nested procedure invocations.

This is a stepping trap, used to implement the “step”, “next”, “step-instruction”, and “next-instruction” REPL commands.

* * *

Previous: [Traps](06_26_debugging_infrastructure.md#6265-traps), Up: [Debugging Infrastructure](06_26_debugging_infrastructure.md#626-debugging-infrastructure)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.26.6 GDB Support [¶](06_26_debugging_infrastructure.md#6266-gdb-support)

Sometimes, you may find it necessary to debug Guile applications at the C level. Doing so can be tedious, in particular because the debugger is oblivious to Guile’s `SCM` type, and thus unable to display `SCM` values in any meaningful way:

(gdb) frame
#0  scm\_display (obj=0xf04310, port=0x6f9f30) at print.c:1437

To address that, Guile comes with an extension of the GNU Debugger (GDB) that contains a “pretty-printer” for `SCM` values. With this GDB extension, the C frame in the example above shows up like this:

(gdb) frame
#0  scm\_display (obj=("hello" GDB!), port=#<port file 6f9f30>) at print.c:1437

Here GDB was able to decode the list pointed to by obj, and to print it using Scheme’s read syntax.

That extension is a `.scm` file installed alongside the libguile shared library. When GDB 7.8 or later is installed and compiled with support for extensions written in Guile, the extension is automatically loaded when debugging a program linked against libguile (see [Auto-loading](https://doc.guix.gnu.org/gdb/latest/en/gdb.html#Auto_002dloading) in Debugging with GDB). Note that the directory where libguile is installed must be among GDB’s auto-loading “safe directories” (see [Auto-loading safe path](https://doc.guix.gnu.org/gdb/latest/en/gdb.html#Auto_002dloading-safe-path) in Debugging with GDB).

* * *

Previous: [Debugging Infrastructure](06_26_debugging_infrastructure.md#626-debugging-infrastructure), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
