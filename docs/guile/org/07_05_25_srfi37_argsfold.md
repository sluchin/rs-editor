#### 7.5.25 SRFI-37 - args-fold [¶](07_05_25_srfi37_argsfold.md#7525-srfi-37---args-fold)

This is a processor for GNU `getopt_long`\-style program arguments. It provides an alternative, less declarative interface than `getopt-long` in `(ice-9 getopt-long)` (see [The (ice-9 getopt-long) Module](07_04_the_ice9_getoptlong_module.md#74-the-ice-9-getopt-long-module)). Unlike `getopt-long`, it supports repeated options and any number of short and long names per option. Access it with:

([use-modules](06_18_modules.md) (srfi srfi-37))

SRFI\-37 principally provides an `option` type and the `args-fold` function. To use the library, create a set of options with `option` and use it as a specification for invoking `args-fold`.

Here is an example of a simple argument processor for the typical ‘\--version’ and ‘\--help’ options, which returns a backwards list of files given on the command line:

([args-fold](07_05_25_srfi37_argsfold.md) ([cdr](06_06_08_pairs.md) ([program-arguments](07_02_06_runtime_environment.md)))
           (let ((display-and-exit-proc
                  (lambda (msg)
                    (lambda (opt name arg loads)
                      ([display](06_16_reading_and_evaluating_scheme_code.md) msg) ([quit](04_programming_in_scheme.md))))))
             ([list](06_06_09_lists.md) ([option](04_programming_in_scheme.md) '(#\\v "version") #f #f
                           (display-and-exit-proc "Foo version 42.0\\n"))
                   ([option](04_programming_in_scheme.md) '(#\\h "help") #f #f
                           (display-and-exit-proc
                            "Usage: foo scheme-file ..."))))
           (lambda (opt name arg loads)
             ([error](04_programming_in_scheme.md) "Unrecognized option \`~A'" name))
           (lambda (op loads) ([cons](06_06_08_pairs.md) op loads))
           '())

Scheme Procedure: **option** names required-arg? optional-arg? processor [¶](07_05_25_srfi37_argsfold.md)

Return an object that specifies a single kind of program option.

names is a list of command-line option names, and should consist of characters for traditional `getopt` short options and strings for `getopt_long`\-style long options.

required-arg? and optional-arg? are mutually exclusive; one or both must be `#f`. If required-arg?, the option must be followed by an argument on the command line, such as ‘\--opt=value’ for long options, or an error will be signaled. If optional-arg?, an argument will be taken if available.

processor is a procedure that takes at least 3 arguments, called when `args-fold` encounters the option: the containing option object, the name used on the command line, and the argument given for the option (or `#f` if none). The rest of the arguments are `args-fold` “seeds”, and the processor should return seeds as well.

Scheme Procedure: **option-names** opt [¶](07_05_25_srfi37_argsfold.md)

Scheme Procedure: **option-required-arg?** opt [¶](07_05_25_srfi37_argsfold.md)

Scheme Procedure: **option-optional-arg?** opt [¶](07_05_25_srfi37_argsfold.md)

Scheme Procedure: **option-processor** opt [¶](07_05_25_srfi37_argsfold.md)

Return the specified field of opt, an option object, as described above for `option`.

Scheme Procedure: **args-fold** args options unrecognized-option-proc operand-proc seed … [¶](07_05_25_srfi37_argsfold.md)

Process args, a list of program arguments such as that returned by `(cdr (program-arguments))`, in order against options, a list of option objects as described above. All functions called take the “seeds”, or the last multiple-values as multiple arguments, starting with seed …, and must return the new seeds. Return the final seeds.

Call `unrecognized-option-proc`, which is like an option object’s processor, for any options not found in options.

Call `operand-proc` with any items on the command line that are not named options. This includes arguments after ‘\--’. It is called with the argument in question, as well as the seeds.

* * *

Next: [SRFI-39 - Parameters](07_05_27_srfi39_parameters.md#7527-srfi-39---parameters), Previous: [SRFI-37 - args-fold](07_05_25_srfi37_argsfold.md#7525-srfi-37---args-fold), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
