#### 7.5.37 SRFI-64: A Scheme API for Test Suites [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#7537-srfi-64-a-scheme-api-for-test-suites)

*   [SRFI-64 Abstract](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75371-srfi-64-abstract)
*   [SRFI-64 Rationale](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75372-srfi-64-rationale)
*   [SRFI-64 Writing Basic Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75373-srfi-64-writing-basic-test-suites)
*   [SRFI-64 Conditonal Test Suites and Other Advanced Features](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75374-srfi-64-conditonal-test-suites-and-other-advanced-features)
*   [SRFI-64 Test Runner](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75375-srfi-64-test-runner)
*   [SRFI-64 Test Results](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75376-srfi-64-test-results)
*   [SRFI-64 Writing a New Test Runner](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75377-srfi-64-writing-a-new-test-runner)

* * *

Next: [SRFI-64 Rationale](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75372-srfi-64-rationale), Up: [SRFI-64: A Scheme API for Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#7537-srfi-64-a-scheme-api-for-test-suites)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.37.1 SRFI-64 Abstract [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75371-srfi-64-abstract)

This defines an API for writing _test suites_, to make it easy to portably test Scheme APIs, libraries, applications, and implementations. A test suite is a collection of _test cases_ that execute in the context of a _test-runner_. This specification also supports writing new test-runners, to allow customization of reporting and processing the result of running test suites.

* * *

Next: [SRFI-64 Writing Basic Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75373-srfi-64-writing-basic-test-suites), Previous: [SRFI-64 Abstract](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75371-srfi-64-abstract), Up: [SRFI-64: A Scheme API for Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#7537-srfi-64-a-scheme-api-for-test-suites)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.37.2 SRFI-64 Rationale [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75372-srfi-64-rationale)

The Scheme community needs a standard for writing test suites. Every SRFI or other library should come with a test suite. Such a test suite must be portable, without requiring any non-standard features, such as modules. The test suite implementation or "runner" need not be portable, but it is desirable that it be possible to write a portable basic implementation.

There are other testing frameworks written in Scheme, including [RackUnit](https://docs.racket-lang.org/rackunit/). However RackUnit is not portable. It is also a bit on the verbose side. It would be useful to have a bridge between this framework and RackUnit so RackUnit tests could run under this framework and vice versa. There exists also at least one Scheme wrapper providing a Scheme interface to the “standard” [JUnit](https://www.junit.org/) API for Java. It would be useful to have a bridge so that tests written using this framework can run under a JUnit runner. Neither of these features are part of this specification.

This API makes use of implicit dynamic state, including an implicit “test runner”. This makes the API convenient and terse to use, but it may be a little less elegant and “compositional” than using explicit test objects, such as JUnit-style frameworks. It is not claimed to follow either object-oriented or functional design principles, but I hope it is useful and convenient to use and extend.

This proposal allows converting a Scheme source file to a test suite by just adding a few macros. You don’t have to write the entire file in a new form, thus you don’t have to re-indent it.

All names defined by the API start with the prefix ‘test-’. All function-like forms are defined as syntax. They may be implemented as functions or macros or built-ins. The reason for specifying them as syntax is to allow specific tests to be skipped without evaluating sub-expressions, or for implementations to add features such as printing line numbers or catching exceptions.

* * *

Next: [SRFI-64 Conditonal Test Suites and Other Advanced Features](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75374-srfi-64-conditonal-test-suites-and-other-advanced-features), Previous: [SRFI-64 Rationale](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75372-srfi-64-rationale), Up: [SRFI-64: A Scheme API for Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#7537-srfi-64-a-scheme-api-for-test-suites)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.37.3 SRFI-64 Writing Basic Test Suites [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75373-srfi-64-writing-basic-test-suites)

Let’s start with a simple example. This is a complete self-contained test-suite.

;; Initialize and give a name to a simple testsuite.
([test-begin](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "vec-test")
(define v ([make-vector](06_06_10_vectors.md) 5 99))
;; Require that an expression evaluate to true.
([test-assert](07_05_37_srfi64_a_scheme_api_for_test_suites.md) ([vector?](06_06_10_vectors.md) v))
;; Test that an expression is eqv? to some other expression.
([test-eqv](07_05_37_srfi64_a_scheme_api_for_test_suites.md) 99 ([vector-ref](06_06_10_vectors.md) v 2))
([vector-set!](06_06_10_vectors.md) v 2 7)
([test-eqv](07_05_37_srfi64_a_scheme_api_for_test_suites.md) 7 ([vector-ref](06_06_10_vectors.md) v 2))
;; Finish the testsuite, and report results.
([test-end](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "vec-test")

This testsuite could be saved in its own source file. Nothing else is needed: We do not require any top-level forms, so it is easy to wrap an existing program or test to this form, without adding indentation. It is also easy to add new tests, without having to name individual tests (though that is optional).

Test cases are executed in the context of a _test runner_, which is an object that accumulates and reports test results. This specification defines how to create and use custom test runners, but implementations should also provide a default test runner. It is suggested (but not required) that loading the above file in a top-level environment will cause the tests to be executed using an implementation-specified default test runner, and `test-end` will cause a summary to be displayed in an implementation-specified manner. The SRFI 64 implementation used in Guile provides such a default test runner; running the above snippet at the REPL prints:

\*\*\* Entering test group: vec-test \*\*\*
$1 = #t
\* PASS:
$2 = ((pass . 1))
\* PASS:
$3 = ((pass . 2))
\* PASS:
$4 = ((pass . 3))
\*\*\* Leaving test group: vec-test \*\*\*
\*\*\* Test suite finished. \*\*\*
\*\*\* # of expected passes    : 3

It also returns the `<test-runner>` object.

#### Simple test-cases [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#simple-test-cases)

Primitive test cases test that a given condition is true. They may have a name. The core test case form is `test-assert`:

Scheme Syntax: **test-assert** \[test-name\] expression [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

This evaluates the expression. The test passes if the result is true; if the result is false, a test failure is reported. The test also fails if an exception is raised, assuming the implementation has a way to catch exceptions. How the failure is reported depends on the test runner environment. The test-name is a string that names the test case. (Though the test-name is a string literal in the examples, it is an expression. It is evaluated only once.) It is used when reporting errors, and also when skipping tests, as described below. It is an error to invoke `test-assert`if there is no current test runner.

The following forms may be more convenient than using `test-assert` directly:

Scheme Syntax: **test-eqv** \[test-name\] expected test-expr [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

This is equivalent to:

([test-assert](07_05_37_srfi64_a_scheme_api_for_test_suites.md) \[test-name\] ([eqv?](06_09_general_utility_functions.md) expected test-expr))

Similarly `test-equal` and `test-eq` are shorthand for `test-assert` combined with `equal?` or `eq?`, respectively:

Scheme Syntax: **test-equal** \[test-name\] expected test-expr [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Syntax: **test-eq** \[test-name\] expected test-expr [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Here is a simple example:

(define (mean x y) ([/](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) x y) 2.0))
([test-eqv](07_05_37_srfi64_a_scheme_api_for_test_suites.md) 4 (mean 3 5))

For testing approximate equality of inexact reals we can use `test-approximate`:

Scheme Syntax: **test-approximate** \[test-name\] expected test-expr error [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

This is equivalent to (except that each argument is only evaluated once):

([test-assert](07_05_37_srfi64_a_scheme_api_for_test_suites.md) \[test-name\]
  (and ([\>=](06_06_02_numerical_data_types.md) test-expr ([\-](06_06_02_numerical_data_types.md) expected [error](04_programming_in_scheme.md)))
       ([<=](06_06_02_numerical_data_types.md) test-expr ([+](06_06_02_numerical_data_types.md) expected [error](04_programming_in_scheme.md)))))

Here’s an example:

([test-approximate](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "is 22/7 within 1% of π?"
 3.1415926535
 22/7
 1/100)

#### Tests for catching errors [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#tests-for-catching-errors)

We need a way to specify that evaluation _should_ fail. This verifies that errors are detected when required.

Scheme Syntax: **test-error** \[\[test-name\] error-type\] test-expr [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Evaluating test-expr is expected to signal an error. The kind of error is indicated by error-type.

If the error-type is left out, or it is `#t`, it means "some kind of unspecified error should be signaled". For example:

([test-error](07_05_37_srfi64_a_scheme_api_for_test_suites.md) #t ([vector-ref](06_06_10_vectors.md) '#(1 2) 9))

This specification leaves it implementation-defined (or for a future specification) what form test-error may take, though all implementations must allow `#t`. Some implementations may support [SRFI-35’s conditions](https://srfi.schemers.org/srfi-35/srfi-35.html), but these are only standardized for [SRFI-36’s I/O conditions](https://srfi.schemers.org/srfi-36/srfi-36.html), which are seldom useful in test suites. An implementation may also allow implementation-specific “exception types”. For example Java-based implementations may allow the names of Java exception classes:

;; Kawa-specific example
([test-error](07_05_37_srfi64_a_scheme_api_for_test_suites.md) <java.lang.IndexOutOfBoundsException> ([vector-ref](06_06_10_vectors.md) '#(1 2) 9))

An implementation that cannot catch exceptions should skip `test-error` forms.

The SRFI-64 implementation in Guile supports specifying error-type as either:

*   `#f`, meaning the test is _not_ expected to produce an error
*   `#t`, meaning the test is expected to produce an error, of any type
*   A native exception type, as created via `make-exception-type` or `make-condition-type` from SRFI-35
*   A predicate, which will be applied to the exception caught to determine whether is it of the right type
*   A symbol, for the exception kind of legacy `make-exception-from-throw` style exceptions.

Below are some examples valid in Guile:

([test-error](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "expect old-style exception kind"
 'numerical-overflow
 ([/](06_06_02_numerical_data_types.md) 1 0))

([use-modules](06_18_modules.md) (ice-9 exceptions)) ;for standard exception types
([test-error](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "expect a native exception type"
 [&warning](07_06_r6rs_support.md)
 ([raise-exception](06_11_controlling_the_flow_of_program_execution.md) ([make-warning](06_11_controlling_the_flow_of_program_execution.md))))

([test-error](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "expect a native exception, using predicate"
 [warning?](06_11_controlling_the_flow_of_program_execution.md)
 ([raise-exception](06_11_controlling_the_flow_of_program_execution.md) ([make-warning](06_11_controlling_the_flow_of_program_execution.md))))

([use-modules](06_18_modules.md) (srfi srfi-35))

([test-error](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "expect a serious SRFI 35 condition type"
 [&serious](07_06_r6rs_support.md)
 ([raise-exception](06_11_controlling_the_flow_of_program_execution.md) ([condition](07_05_24_srfi35_conditions.md) ([&serious](07_06_r6rs_support.md)))))

([test-error](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "expect a serious SRFI 35 condition type, using predicate"
 [serious-condition?](07_05_24_srfi35_conditions.md)
 ([raise-exception](06_11_controlling_the_flow_of_program_execution.md) ([condition](07_05_24_srfi35_conditions.md) ([&serious](07_06_r6rs_support.md)))))

#### Testing syntax [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#testing-syntax)

Testing syntax is tricky, especially if we want to check that invalid syntax is causing an error. The following utility function can help:

Scheme Procedure: **test-read-eval-string** string [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

This function parses string (using `read`) and evaluates the result. The result of evaluation is returned from `test-read-eval-string`. An error is signalled if there are unread characters after the `read` is done. For example: `(test-read-eval-string "(+ 3 4)")` _evaluates to_ `7`. `(test-read-eval-string "(+ 3 4")` _signals an error_. `(test-read-eval-string "(+ 3 4) ")` _signals an error_, because there is extra “junk” (_i.e._ a space) after the list is read.

The `test-read-eval-string` used in tests:

([test-equal](07_05_37_srfi64_a_scheme_api_for_test_suites.md) 7 ([test-read-eval-string](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "(+ 3 4)"))
([test-error](07_05_37_srfi64_a_scheme_api_for_test_suites.md) ([test-read-eval-string](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "(+ 3"))
([test-equal](07_05_37_srfi64_a_scheme_api_for_test_suites.md) #\\newline ([test-read-eval-string](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "#\\\\newline"))
([test-error](07_05_37_srfi64_a_scheme_api_for_test_suites.md) ([test-read-eval-string](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "#\\\\newlin"))
;; Skip the next 2 tests unless srfi-62 is available.
([test-skip](07_05_37_srfi64_a_scheme_api_for_test_suites.md) ([cond-expand](07_05_02_subsection_2.md) (srfi-62 0) (else 2)))
([test-equal](07_05_37_srfi64_a_scheme_api_for_test_suites.md) 5 ([test-read-eval-string](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "(+ 1 #;(\* 2 3) 4)"))
([test-equal](07_05_37_srfi64_a_scheme_api_for_test_suites.md) '(x z) (test-read-string "(list 'x #;'y 'z)"))

#### Test groups and paths [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#test-groups-and-paths)

A _test group_ is a named sequence of forms containing testcases, expressions, and definitions. Entering a group sets the _test group name_; leaving a group restores the previous group name. These are dynamic (run-time) operations, and a group has no other effect or identity. Test groups are informal groupings: they are neither Scheme values, nor are they syntactic forms. A test group may contain nested inner test groups. The _test group path_ is a list of the currently-active (entered) test group names, oldest (outermost) first.

Scheme Syntax: **test-begin** suite-name \[count\] [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

A `test-begin` enters a new test group. The suite-name becomes the current test group name, and is added to the end of the test group path. Portable test suites should use a string literal for suite-name; the effect of expressions or other kinds of literals is unspecified.

_Rationale:_ In some ways using symbols would be preferable. However, we want human-readable names, and standard Scheme does not provide a way to include spaces or mixed-case text in literal symbols.

The optional count must match the number of test-cases executed by this group. (Nested test groups count as a single test case for this count.) This extra test may be useful to catch cases where a test doesn’t get executed because of some unexpected error.

Additionally, if there is no currently executing test runner, one is installed in an implementation-defined manner.

Scheme Syntax: **test-end** \[suite-name\] [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

A `test-end` leaves the current test group. An error is reported if the suite-name does not match the current test group name.

Additionally, if the matching `test-begin`installed a new test-runner, then the `test-end` will uninstall it, after reporting the accumulated test results in an implementation-defined manner.

Scheme Syntax: **test-group** suite-name decl-or-expr … [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Equivalent to:

(if ([not](06_06_01_booleans.md) (test-to-skip% (var suite-name)))
  ([dynamic-wind](06_11_controlling_the_flow_of_program_execution.md)
    (lambda () ([test-begin](07_05_37_srfi64_a_scheme_api_for_test_suites.md) (var suite-name)))
    (lambda () (var decl-or-expr) [...](06_08_macros.md))
    (lambda () ([test-end](07_05_37_srfi64_a_scheme_api_for_test_suites.md) (var suite-name)))))

This is usually equivalent to executing the decl-or-exprs within the named test group. However, the entire group is skipped if it matched an active `test-skip` (see later). Also, the `test-end` is executed in case of an exception.

#### Handling set-up and cleanup [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#handling-set-up-and-cleanup)

Scheme Syntax: **test-group-with-cleanup** suite-name decl-or-expr … cleanup-form [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Execute each of the decl-or-expr forms in order (as in a <body>), and then execute the cleanup-form. The latter should be executed even if one of a decl-or-expr forms raises an exception (assuming the implementation has a way to catch exceptions).

For example:

(let ((f ([open-output-file](06_12_input_and_output.md) "log")))
  ([test-group-with-cleanup](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "test-file"
    (do-a-bunch-of-tests f)
    ([close-output-port](07_06_r6rs_support.md) f)))

* * *

Next: [SRFI-64 Test Runner](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75375-srfi-64-test-runner), Previous: [SRFI-64 Writing Basic Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75373-srfi-64-writing-basic-test-suites), Up: [SRFI-64: A Scheme API for Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#7537-srfi-64-a-scheme-api-for-test-suites)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.37.4 SRFI-64 Conditonal Test Suites and Other Advanced Features [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75374-srfi-64-conditonal-test-suites-and-other-advanced-features)

The following describes features for controlling which tests to execute, or specifying that some tests are _expected_ to fail.

#### Test specifiers [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#test-specifiers)

Sometimes we want to only run certain tests, or we know that certain tests are expected to fail. A _test specifier_ is a one-argument function that takes a test-runner and returns a boolean. The specifier may be run before a test is performed, and the result may control whether the test is executed. For convenience, a specifier may also be a non-procedure value, which is coerced to a specifier procedure, as described below for count and name.

A simple example is:

(if (var some-condition)  ([test-skip](07_05_37_srfi64_a_scheme_api_for_test_suites.md) 2)) ;; skip next 2 tests

Scheme Procedure: **test-match-name** name [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

The resulting specifier matches if the current test name (as returned by `test-runner-test-name`) is `equal?` to name.

Scheme Syntax: **test-match-nth** n \[count\] [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

This evaluates to a _stateful_ predicate: A counter keeps track of how many times it has been called. The predicate matches the n’th time it is called (where `1` is the first time), and the next ‘(- count 1)’ times, where count defaults to `1`.

Scheme Syntax: **test-match-any** specifier … [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

The resulting specifier matches if any specifier matches. Each specifier is applied, in order, so side-effects from a later specifier happen even if an earlier specifier is true.

Scheme Syntax: **test-match-all** specifier … [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

The resulting specifier matches if each specifier matches. Each specifier is applied, in order, so side-effects from a later specifier happen even if an earlier specifier is false.

count _(i.e. an integer)_ Convenience short-hand for: ‘(test-match-nth 1 count)’.

name _(i.e. a string)_ Convenience short-hand for ‘(test-match-name name)’.

#### Skipping selected tests [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#skipping-selected-tests)

In some cases you may want to skip a test.

Scheme Syntax: **test-skip** specifier [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Evaluating `test-skip` adds the resulting specifier to the set of currently active skip-specifiers. Before each test (or `test-group`) the set of active skip-specifiers are applied to the active test-runner. If any specifier matches, then the test is skipped.

For convenience, if the specifier is a string that is syntactic sugar for `(test-match-name specifier)`. For example:

([test-skip](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "test-b")
([test-assert](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "test-a")   ;; executed
([test-assert](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "test-b")   ;; skipped

Any skip specifiers introduced by a `test-skip` are removed by a following non-nested `test-end`.

([test-begin](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "group1")
([test-skip](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "test-a")
([test-assert](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "test-a")   ;; skipped
([test-end](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "group1")      ;; Undoes the prior test-skip
([test-assert](07_05_37_srfi64_a_scheme_api_for_test_suites.md) "test-a")   ;; executed

#### Expected failures [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#expected-failures)

Sometimes you know a test case will fail, but you don’t have time to or can’t fix it. Maybe a certain feature only works on certain platforms. However, you want the test-case to be there to remind you to fix it. You want to note that such tests are expected to fail.

Scheme Syntax: **test-expect-fail** specifier [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Matching tests (where matching is defined as in `test-skip`) are expected to fail. This only affects test reporting, not test execution. For example:

([test-expect-fail](07_05_37_srfi64_a_scheme_api_for_test_suites.md) 2)
([test-eqv](07_05_37_srfi64_a_scheme_api_for_test_suites.md) [...](06_08_macros.md)) ;; expected to fail
([test-eqv](07_05_37_srfi64_a_scheme_api_for_test_suites.md) [...](06_08_macros.md)) ;; expected to fail
([test-eqv](07_05_37_srfi64_a_scheme_api_for_test_suites.md) [...](06_08_macros.md)) ;; expected to pass

* * *

Next: [SRFI-64 Test Results](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75376-srfi-64-test-results), Previous: [SRFI-64 Conditonal Test Suites and Other Advanced Features](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75374-srfi-64-conditonal-test-suites-and-other-advanced-features), Up: [SRFI-64: A Scheme API for Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#7537-srfi-64-a-scheme-api-for-test-suites)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.37.5 SRFI-64 Test Runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75375-srfi-64-test-runner)

A _test-runner_ is an object that runs a test-suite, and manages the state. The test group path, and the sets skip and expected-fail specifiers are part of the test-runner. A test-runner will also typically accumulate statistics about executed tests,

Scheme Procedure: **test-runner?** value [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

True if and only if value is a test-runner object.

Scheme Parameter: **test-runner-current** [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Parameter: **test-runner-current** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Get or set the current test-runner.

Scheme Procedure: **test-runner-get** [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Same as `(test-runner-current)`, but throws an exception if there is no current test-runner.

Scheme Procedure: **test-runner-simple** [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Creates a new simple test-runner, that prints errors and a summary on the standard output port.

Scheme Procedure: **test-runner-null** [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Creates a new test-runner, that does nothing with the test results. This is mainly meant for extending when writing a custom runner.

Scheme Procedure: **test-runner-create** [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Create a new test-runner. Equivalent to ‘((test-runner-factory))’.

Scheme Parameter: **test-runner-factory** [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Parameter: **test-runner-factory** factory [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Get or set the current test-runner factory. A factory is a zero-argument function that creates a new test-runner. The default value is `test-runner-simple`.

#### Running specific tests with a specified runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#running-specific-tests-with-a-specified-runner)

Scheme Procedure: **test-apply** \[runner\] specifier … procedure [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Calls procedure with no arguments using the specified runner as the current test-runner. If runner is omitted, then `(test-runner-current)` is used. (If there is no current runner, one is created as in `test-begin`.) If one or more specifiers are listed then only tests matching the specifiers are executed. A specifier has the same form as one used for `test-skip`. A test is executed if it matches any of the specifiers in the `test-apply` _and_ does not match any active `test-skip` specifiers.

Scheme Syntax: **test-with-runner** runner decl-or-expr … [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Executes each decl-or-expr in order in a context where the current test-runner is runner.

* * *

Next: [SRFI-64 Writing a New Test Runner](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75377-srfi-64-writing-a-new-test-runner), Previous: [SRFI-64 Test Runner](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75375-srfi-64-test-runner), Up: [SRFI-64: A Scheme API for Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#7537-srfi-64-a-scheme-api-for-test-suites)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.37.6 SRFI-64 Test Results [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75376-srfi-64-test-results)

Running a test sets various status properties in the current test-runner. This can be examined by a custom test-runner, or (more rarely) in a test-suite.

#### Result Kind [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#result-kind)

Running a test may yield one of the following status symbols:

`'pass`

The test passed, as expected.

`'fail`

The test failed (and was not expected to).

`'xfail`

The test failed and was expected to.

`'xpass`

The test passed, but was expected to fail.

`'skip`

The test was skipped.

Scheme Procedure: **test-result-kind** \[runner\] [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Returns one of the above result codes from the most recent tests. Returns `#f` if no tests have been run yet. If we’ve started on a new test, but don’t have a result yet, then the result kind is `'xfail` if the test is expected to fail, `'skip` if the test is supposed to be skipped, or `#f` otherwise.

Scheme Procedure: **test-passed?** \[runner\] [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

True if the value of ‘(test-result-kind \[runner\])’ is one of `'pass` or `'xpass`. This is a convenient shorthand that might be useful in a test suite to only run certain tests if the previous test passed.

#### Test result properties [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#test-result-properties)

A test runner also maintains a set of more detailed “result properties” associated with the current or most recent test. (I.e. the properties of the most recent test are available as long as a new test hasn’t started.) Each property has a name (a symbol) and a value (any value). Some properties are standard or set by the implementation; implementations can add more.

Scheme Procedure: **test-result-ref** runner pname \[default\] [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Returns the property value associated with the pname property name (a symbol). If there is no value associated with pname return default, or `#f` if default isn’t specified.

Scheme Syntax: **test-result-set!** runner pname value [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Sets the property value associated with the pname property name to value. Usually implementation code should call this function, but it may be useful for a custom test-runner to add extra properties.

Scheme Procedure: **test-result-remove** runner pname [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Remove the property with the name pname.

Scheme Procedure: **test-result-clear** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Remove all result properties. The implementation automatically calls `test-result-clear` at the start of a `test-assert` and similar procedures.

Scheme Procedure: **test-result-alist** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Returns an association list of the current result properties. It is unspecified if the result shares state with the test-runner. The result should not be modified; on the other hand, the result may be implicitly modified by future `test-result-set!` or `test-result-remove` calls. However, a `test-result-clear` does not modify the returned alist. Thus you can “archive” result objects from previous runs.

#### Standard result properties [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#standard-result-properties)

The set of available result properties is implementation-specific. However, it is suggested that the following might be provided:

`'result-kind`

The result kind, as defined previously. This is the only mandatory result property. `(test-result-kind runner)` is equivalent to: `(test-result-ref runner 'result-kind)`

`'source-file`

`'source-line`

If known, the location of test statements (such as `test-assert`) in test suite source code.

`'source-form`

The source form, if meaningful and known.

`'expected-value`

The expected non-error result, if meaningful and known.

`'expected-error`

The error-typespecified in a `test-error`, if it meaningful and known.

`'actual-value`

The actual non-error result value, if meaningful and known.

`'actual-error`

The error value, if an error was signalled and it is known. The actual error value is implementation-defined.

* * *

Previous: [SRFI-64 Test Results](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75376-srfi-64-test-results), Up: [SRFI-64: A Scheme API for Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#7537-srfi-64-a-scheme-api-for-test-suites)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.37.7 SRFI-64 Writing a New Test Runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#75377-srfi-64-writing-a-new-test-runner)

This section specifies how to write a test-runner. It can be ignored if you just want to write test-cases.

#### Call-back Functions [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#call-back-functions)

These call-back functions are “methods” (in the object-oriented sense) of a test-runner. A method `test-runner-on-event` is called by the implementation when event happens.

To define (set) the callback function for event use the following expression. (This is normally done when initializing a test-runner.)

`(test-runner-on-event! runner event-function)`

An event-function takes a test-runner argument, and possibly other arguments, depending on the event.

To extract (get) the callback function for event do this: `(test-runner-on-event runner)`

To extract call the callback function for event use the following expression. (This is normally done by the implementation core.) ‘((test-runner-on-event runner) runner other-args …)’.

The following call-back hooks are available.

Scheme Procedure: **test-runner-on-test-begin** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-runner-on-test-begin!** runner on-test-begin-function [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **on-test-begin-function** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

The on-test-begin-function is called at the start of an individual testcase, before the test expression (and expected value) are evaluated.

Scheme Procedure: **test-runner-on-test-end** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-runner-on-test-end!** runner on-test-end-function [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **on-test-end-function** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

The on-test-end-function is called at the end of an individual testcase, when the result of the test is available.

Scheme Procedure: **test-runner-on-group-begin** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-runner-on-group-begin!** runner on-group-begin-function [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **on-group-begin-function** runner suite-name count [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

The on-group-begin-function is called by a `test-begin`, including at the start of a `test-group`. The suite-name is a Scheme string, and count is an integer or `#f`.

Scheme Procedure: **test-runner-on-group-end** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-runner-on-group-end!** runner on-group-end-function [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **on-group-end-function** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

The on-group-end-function is called by a `test-end`, including at the end of a `test-group`.

Scheme Procedure: **test-runner-on-bad-count** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-runner-on-bad-count!** runner on-bad-count-function [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **on-bad-count-function** runner actual-count expected-count [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Called from `test-end` (before the on-group-end-function is called) if an expected-count was specified by the matching `test-begin` and the expected-count does not match the actual-count of tests actually executed or skipped.

Scheme Procedure: **test-runner-on-bad-end-name** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-runner-on-bad-end-name!** runner on-bad-end-name-function [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **on-bad-end-name-function** runner begin-name end-name [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Called from `test-end` (before the on-group-end-function is called) if a suite-name was specified, and it did not that the name in the matching `test-begin`.

Scheme Procedure: **test-runner-on-final** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-runner-on-final!** runner on-final-function [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **on-final-function** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

The on-final-function takes one parameter (a test-runner) and typically displays a summary (count) of the tests. The on-final-function is called after called the on-group-end-function correspondiong to the outermost `test-end`. The default value is `test-on-final-simple` which writes to the standard output port the number of tests of the various kinds.

The default test-runner returned by `test-runner-simple` uses the following call-back functions:

Scheme Procedure: **test-on-test-begin-simple** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-on-test-end-simple** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-on-group-begin-simple** runner suite-name count [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-on-group-end-simple** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-on-bad-count-simple** runner actual-count expected-count [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-on-bad-end-name-simple** runner begin-name end-name [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

You can call those if you want to write your own test-runner.

#### Test-runner components [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#test-runner-components)

The following functions are for accessing the other components of a test-runner. They would normally only be used to write a new test-runner or a match-predicate.

Scheme Procedure: **test-runner-pass-count** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Returns the number of tests that passed, and were expected to pass.

Scheme Procedure: **test-runner-fail-count** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Returns the number of tests that failed, but were expected to pass.

Scheme Procedure: **test-runner-xpass-count** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Returns the number of tests that passed, but were expected to fail.

Scheme Procedure: **test-runner-xfail-count** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Returns the number of tests that failed, and were expected to pass.

Scheme Procedure: **test-runner-skip-count** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Returns the number of tests or test groups that were skipped.

Scheme Procedure: **test-runner-test-name** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Returns the name of the current test or test group, as a string. During execution of `test-begin` this is the name of the test group; during the execution of an actual test, this is the name of the test-case. If no name was specified, the name is the empty string.

Scheme Procedure: **test-runner-group-path** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

A list of names of groups we’re nested in, with the outermost group first.

Scheme Procedure: **test-runner-group-stack** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

A list of names of groups we’re nested in, with the outermost group last. (This is more efficient than `test-runner-group-path`, since it doesn’t require any copying.)

Scheme Procedure: **test-runner-aux-value** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Scheme Procedure: **test-runner-aux-value!** runner on-test [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Get or set the `aux-value` field of a test-runner. This field is not used by this API or the `test-runner-simple` test-runner, but may be used by custom test-runners to store extra state.

Scheme Procedure: **test-runner-reset** runner [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md)

Resets the state of the runner to its initial state.

#### Example [¶](07_05_37_srfi64_a_scheme_api_for_test_suites.md#example)

This is an example of a simple custom test-runner. Loading this program before running a test-suite will install it as the default test runner.

(define (my-simple-runner filename)
  (let ((runner ([test-runner-null](07_05_37_srfi64_a_scheme_api_for_test_suites.md)))
	(port ([open-output-file](06_12_input_and_output.md) filename))
        (num-passed 0)
        (num-failed 0))
    ([test-runner-on-test-end!](07_05_37_srfi64_a_scheme_api_for_test_suites.md) runner
      (lambda (runner)
        (case ([test-result-kind](07_05_37_srfi64_a_scheme_api_for_test_suites.md) runner)
          ((pass xpass) ([set!](07_06_r6rs_support.md) num-passed ([+](06_06_02_numerical_data_types.md) num-passed 1)))
          ((fail xfail) ([set!](07_06_r6rs_support.md) num-failed ([+](06_06_02_numerical_data_types.md) num-failed 1)))
          (else #t))))
    ([test-runner-on-final!](07_05_37_srfi64_a_scheme_api_for_test_suites.md) runner
       (lambda (runner)
          ([format](07_05_20_srfi28_basic_format_strings.md) port "Passing tests: ~d.~%Failing tests: ~d.~%"
                  num-passed num-failed)
	  ([close-output-port](07_06_r6rs_support.md) port)))
    runner))
([test-runner-factory](07_05_37_srfi64_a_scheme_api_for_test_suites.md)
 (lambda () (my-simple-runner "/tmp/my-test.log")))

* * *

Next: [SRFI-69 - Basic hash tables](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---basic-hash-tables), Previous: [SRFI-64: A Scheme API for Test Suites](07_05_37_srfi64_a_scheme_api_for_test_suites.md#7537-srfi-64-a-scheme-api-for-test-suites), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
