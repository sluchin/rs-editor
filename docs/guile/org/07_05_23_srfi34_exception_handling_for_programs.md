#### 7.5.23 SRFI-34 - Exception handling for programs [¶](07_05_23_srfi34_exception_handling_for_programs.md#7523-srfi-34---exception-handling-for-programs)

Guile provides an implementation of [SRFI-34’s exception handling mechanisms](http://srfi.schemers.org/srfi-34/srfi-34.html) as an alternative to its own built-in mechanisms (see [Exceptions](06_11_controlling_the_flow_of_program_execution.md#6118-exceptions)). It can be made available as follows:

([use-modules](06_18_modules.md) (srfi srfi-34))

See [Raising and Handling Exceptions](06_11_controlling_the_flow_of_program_execution.md#61182-raising-and-handling-exceptions), for more on `with-exception-handler` and `raise` (known as `raise-exception` in core Guile).

SRFI-34’s `guard` form is syntactic sugar over `with-exception-handler`:

Syntax: **guard** (var clause …) body … [¶](07_05_23_srfi34_exception_handling_for_programs.md)

Evaluate body with an exception handler that binds the raised object to var and within the scope of that binding evaluates clause… as if they were the clauses of a cond expression. That implicit cond expression is evaluated with the continuation and dynamic environment of the guard expression.

If every clause’s test evaluates to false and there is no `else` clause, then `raise` is re-invoked on the raised object within the dynamic environment of the original call to `raise` except that the current exception handler is that of the `guard` expression.

* * *

Next: [SRFI-37 - args-fold](07_05_25_srfi37_argsfold.md#7525-srfi-37---args-fold), Previous: [SRFI-34 - Exception handling for programs](07_05_23_srfi34_exception_handling_for_programs.md#7523-srfi-34---exception-handling-for-programs), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
