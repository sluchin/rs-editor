#### 7.5.9 SRFI-10 - Hash-Comma Reader Extension [¶](07_05_09_srfi10_hashcomma_reader_extension.md#759-srfi-10---hash-comma-reader-extension)

This SRFI implements a reader extension `#,()` called hash-comma. It allows the reader to give new kinds of objects, for use both in data and as constants or literals in source code. This feature is available with

(use-modules (srfi srfi-10))

The new read syntax is of the form

#,(tag arg...)

where tag is a symbol and the args are objects taken as parameters. tags are registered with the following procedure.

Scheme Procedure: **define-reader-ctor** tag proc [¶](07_05_09_srfi10_hashcomma_reader_extension.md)

Register proc as the constructor for a hash-comma read syntax starting with symbol tag, i.e. `#,(tag arg…)`. proc is called with the given arguments `(proc arg…)` and the object it returns is the result of the read.

For example, a syntax giving a list of N copies of an object.

(define-reader-ctor 'repeat
  (lambda (obj reps)
    (make-list reps obj)))

(display '#,(repeat 99 3))
⊣ (99 99 99)

Notice the quote `'` when the `#,( )` is used. The `repeat` handler returns a list and the program must quote to use it literally, the same as any other list. Ie.

(display '#,(repeat 99 3))
⇒
(display '(99 99 99))

When a handler returns an object which is self-evaluating, like a number or a string, then there’s no need for quoting, just as there’s no need when giving those directly as literals. For example an addition,

(define-reader-ctor 'sum
  (lambda (x y)
    (+ x y)))
(display #,(sum 123 456)) ⊣ 579

Once `(srfi srfi-10)` has loaded, `#,()` is available globally, there’s no need to use `(srfi srfi-10)` in later modules. Similarly the tags registered are global and can be used anywhere once registered.

We do not recommend `#,()` reader extensions, however, and for three reasons.

First of all, this SRFI is not modular: the tag is matched by name, not as an identifier within a scope. Defining a reader extension in one part of a program can thus affect unrelated parts of a program because the tag is not scoped.

Secondly, reader extensions can be hard to manage from a time perspective: when does the reader extension take effect? See [Eval-when](06_08_macros.md#688-eval-when), for more discussion.

Finally, reader extensions can easily produce objects that can’t be reified to an object file by the compiler. For example if you define a reader extension that makes a hash table (see [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables)), then it will work fine when run with the interpreter, and you think you have a neat hack. But then if you try to compile your program, after wrangling with the `eval-when` concerns mentioned above, the compiler will carp that it doesn’t know how to serialize a hash table to disk.

In the specific case of hash tables, it would be possible for Guile to know how to pack hash tables into compiled files, but this doesn’t work in general. What if the object you produce is an instance of a record type? Guile would then have to serialize the record type to disk too, and then what happens if the program independently loads the code that defines the record type? Does it define the same type or a different type? Guile’s record types are nominal, not structural, so the answer is not clear at all.

For all of these reasons we recommend macros over reader extensions. Macros fulfill many of the same needs while preserving modular composition, and their interaction with `eval-when` is well-known. If you need brevity, instead use `read-hash-extend` and make your reader extension expand to a macro invocation. In that way we preserve scoping as much as possible. See [Reader Extensions](06_16_reading_and_evaluating_scheme_code.md#61616-reader-extensions).

* * *

Next: [SRFI-13 - String Library](07_05_11_srfi13_string_library.md#7511-srfi-13---string-library), Previous: [SRFI-10 - Hash-Comma Reader Extension](07_05_09_srfi10_hashcomma_reader_extension.md#759-srfi-10---hash-comma-reader-extension), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
