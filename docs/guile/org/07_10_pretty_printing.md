### 7.10 Pretty Printing [¶](07_10_pretty_printing.md#710-pretty-printing)

The module `(ice-9 pretty-print)` provides the procedure `pretty-print`, which provides nicely formatted output of Scheme objects. This is especially useful for deeply nested or complex data structures, such as lists and vectors.

The module is loaded by entering the following:

([use-modules](06_18_modules.md) (ice-9 [pretty-print](04_programming_in_scheme.md)))

This makes the procedure `pretty-print` available. As an example how `pretty-print` will format the output, see the following:

([pretty-print](04_programming_in_scheme.md) '(define (foo) (lambda (x)
(cond (([zero?](06_06_02_numerical_data_types.md) x) #t) (([negative?](06_06_02_numerical_data_types.md) x) \-x) (else
(if ([\=](06_06_02_numerical_data_types.md) x 1) 2 ([\*](06_06_02_numerical_data_types.md) x x x)))))))
⊣
(define (foo)
  (lambda (x)
    (cond (([zero?](06_06_02_numerical_data_types.md) x) #t)
          (([negative?](06_06_02_numerical_data_types.md) x) \-x)
          (else (if ([\=](06_06_02_numerical_data_types.md) x 1) 2 ([\*](06_06_02_numerical_data_types.md) x x x))))))

Scheme Procedure: **pretty-print** obj \[port\] \[keyword-options\] [¶](07_10_pretty_printing.md)

Print the textual representation of the Scheme object obj to port. port defaults to the current output port, if not given.

The further keyword-options are keywords and parameters as follows,

`#:display?` flag

If flag is true then print using `display`. The default is `#f` which means use `write` style. See [Writing Scheme Values](06_16_reading_and_evaluating_scheme_code.md#6164-writing-scheme-values).

`#:per-line-prefix` string

Print the given string as a prefix on each line. The default is no prefix.

`#:width` columns

Print within the given columns. The default is 79.

`#:max-expr-width` columns

The maximum width of an expression. The default is 50.

Also exported by the `(ice-9 pretty-print)` module is `truncated-print`, a procedure to print Scheme datums, truncating the output to a certain number of characters. This is useful when you need to present an arbitrary datum to the user, but you only have one line in which to do so.

(define [exp](06_06_02_numerical_data_types.md) '(a b #(c d e) f . g))
([truncated-print](07_10_pretty_printing.md) [exp](06_06_02_numerical_data_types.md) #:width 10) ([newline](06_12_input_and_output.md))
⊣ (a b . #)
([truncated-print](07_10_pretty_printing.md) [exp](06_06_02_numerical_data_types.md) #:width 15) ([newline](06_12_input_and_output.md))
⊣ (a b # f . g)
([truncated-print](07_10_pretty_printing.md) [exp](06_06_02_numerical_data_types.md) #:width 18) ([newline](06_12_input_and_output.md))
⊣ (a b #(c [...](06_08_macros.md)) . #)
([truncated-print](07_10_pretty_printing.md) [exp](06_06_02_numerical_data_types.md) #:width 20) ([newline](06_12_input_and_output.md))
⊣ (a b #(c d e) f . g)
([truncated-print](07_10_pretty_printing.md) "The quick brown fox" #:width 20) ([newline](06_12_input_and_output.md))
⊣ "The quick brown..."
([truncated-print](07_10_pretty_printing.md) ([current-module](06_18_modules.md)) #:width 20) ([newline](06_12_input_and_output.md))
⊣ #<directory (gui...>

`truncated-print` will not output a trailing newline. If an expression does not fit in the given width, it will be truncated – possibly ellipsized[27](99_footnotes.md), or in the worst case, displayed as `#`.

Scheme Procedure: **truncated-print** obj \[port\] \[keyword-options\] [¶](07_10_pretty_printing.md)

Print obj, truncating the output, if necessary, to make it fit into width characters. By default, obj will be printed using `write`, though that behavior can be overridden via the display? keyword argument.

The default behavior is to print depth-first, meaning that the entire remaining width will be available to each sub-expression of obj – e.g., if obj is a vector, each member of obj. One can attempt to “ration” the available width, trying to allocate it equally to each sub-expression, via the breadth-first? keyword argument.

The further keyword-options are keywords and parameters as follows,

`#:display?` flag

If flag is true then print using `display`. The default is `#f` which means use `write` style. see [Writing Scheme Values](06_16_reading_and_evaluating_scheme_code.md#6164-writing-scheme-values).

`#:width` columns

Print within the given columns. The default is 79.

`#:breadth-first?` flag

If flag is true, then allocate the available width breadth-first among elements of a compound data structure (list, vector, pair, etc.). The default is `#f` which means that any element is allowed to consume all of the available width.

* * *

Next: [File Tree Walk](07_12_file_tree_walk.md#712-file-tree-walk), Previous: [Pretty Printing](07_10_pretty_printing.md#710-pretty-printing), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
