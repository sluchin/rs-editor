### 7.16 Expect [¶](07_16_expect.md#716-expect)

The macros in this section are made available with:

([use-modules](06_18_modules.md) (ice-9 [expect](07_16_expect.md)))

`expect` is a macro for selecting actions based on the output from a port. The name comes from a tool of similar functionality by Don Libes. Actions can be taken when a particular string is matched, when a timeout occurs, or when end-of-file is seen on the port. The `expect` macro is described below; `expect-strings` is a front-end to `expect` based on regexec (see the regular expression documentation).

Macro: **expect-strings** clause … [¶](07_16_expect.md)

By default, `expect-strings` will read from the current input port. The first term in each clause consists of an expression evaluating to a string pattern (regular expression). As characters are read one-by-one from the port, they are accumulated in a buffer string which is matched against each of the patterns. When a pattern matches, the remaining expression(s) in the clause are evaluated and the value of the last is returned. For example:

([with-input-from-file](06_12_input_and_output.md) "/etc/passwd"
  (lambda ()
    ([expect-strings](07_16_expect.md)
      ("^nobody" ([display](06_16_reading_and_evaluating_scheme_code.md) "Got a nobody user.\\n")
                 ([display](06_16_reading_and_evaluating_scheme_code.md) "That's no problem.\\n"))
      ("^daemon" ([display](06_16_reading_and_evaluating_scheme_code.md) "Got a daemon user.\\n")))))

The regular expression is compiled with the `REG_NEWLINE` flag, so that the ^ and $ anchors will match at any newline, not just at the start and end of the string.

There are two other ways to write a clause:

The expression(s) to evaluate can be omitted, in which case the result of the regular expression match (converted to strings, as obtained from regexec with match-pick set to "") will be returned if the pattern matches.

The symbol `=>` can be used to indicate that the expression is a procedure which will accept the result of a successful regular expression match. E.g.,

("^daemon" [\=>](06_08_macros.md) [write](06_16_reading_and_evaluating_scheme_code.md))
("^d(aemon)" [\=>](06_08_macros.md) (lambda args (for-each [write](06_16_reading_and_evaluating_scheme_code.md) args)))
("^da(em)on" [\=>](06_08_macros.md) (lambda (all [sub](09_03_a_virtual_machine_for_guile.md))
                  ([write](06_16_reading_and_evaluating_scheme_code.md) all) ([newline](06_12_input_and_output.md))
                  ([write](06_16_reading_and_evaluating_scheme_code.md) [sub](09_03_a_virtual_machine_for_guile.md)) ([newline](06_12_input_and_output.md))))

The order of the substrings corresponds to the order in which the opening brackets occur.

A number of variables can be used to control the behavior of `expect` (and `expect-strings`). Most have default top-level bindings to the value `#f`, which produces the default behavior. They can be redefined at the top level or locally bound in a form enclosing the expect expression.

`expect-port`

A port to read characters from, instead of the current input port.

`expect-timeout`

`expect` will terminate after this number of seconds, returning `#f` or the value returned by expect-timeout-proc.

`expect-timeout-proc`

A procedure called if timeout occurs. The procedure takes a single argument: the accumulated string.

`expect-eof-proc`

A procedure called if end-of-file is detected on the input port. The procedure takes a single argument: the accumulated string.

`expect-char-proc`

A procedure to be called every time a character is read from the port. The procedure takes a single argument: the character which was read.

`expect-strings-compile-flags`

Flags to be used when compiling a regular expression, which are passed to `make-regexp` See [Regexp Functions](06_13_regular_expressions.md#6131-regexp-functions). The default value is `regexp/newline`.

`expect-strings-exec-flags`

Flags to be used when executing a regular expression, which are passed to regexp-exec See [Regexp Functions](06_13_regular_expressions.md#6131-regexp-functions). The default value is `regexp/noteol`, which prevents `$` from matching the end of the string while it is still accumulating, but still allows it to match after a line break or at the end of file.

Here’s an example using all of the variables:

(let ((expect-port ([open-input-file](06_12_input_and_output.md) "/etc/passwd"))
      (expect-timeout 1)
      (expect-timeout-proc
        (lambda (s) ([display](06_16_reading_and_evaluating_scheme_code.md) "Times up!\\n")))
      (expect-eof-proc
        (lambda (s) ([display](06_16_reading_and_evaluating_scheme_code.md) "Reached the end of the file!\\n")))
      (expect-char-proc [display](06_16_reading_and_evaluating_scheme_code.md))
      (expect-strings-compile-flags ([logior](06_06_02_numerical_data_types.md) [regexp/newline](06_13_regular_expressions.md) [regexp/icase](06_13_regular_expressions.md)))
      (expect-strings-exec-flags 0))
   ([expect-strings](07_16_expect.md)
     ("^nobody"  ([display](06_16_reading_and_evaluating_scheme_code.md) "Got a nobody user\\n"))))

Macro: **expect** clause … [¶](07_16_expect.md)

`expect` is used in the same way as `expect-strings`, but tests are specified not as patterns, but as procedures. The procedures are called in turn after each character is read from the port, with two arguments: the value of the accumulated string and a flag to indicate whether end-of-file has been reached. The flag will usually be `#f`, but if end-of-file is reached, the procedures are called an additional time with the final accumulated string and `#t`.

The test is successful if the procedure returns a non-false value.

If the `=>` syntax is used, then if the test succeeds it must return a list containing the arguments to be provided to the corresponding expression.

In the following example, a string will only be matched at the beginning of the file:

(let ((expect-port ([open-input-file](06_12_input_and_output.md) "/etc/passwd")))
  ([expect](07_16_expect.md)
     ((lambda (s eof?) ([string=?](06_06_05_strings.md) s "fnord!"))
        ([display](06_16_reading_and_evaluating_scheme_code.md) "Got a nobody user!\\n"))))

The control variables described for `expect-strings` also influence the behavior of `expect`, with the exception of variables whose names begin with `expect-strings-`.

* * *

Next: [The Scheme shell (scsh)](07_18_the_scheme_shell_scsh.md#718-the-scheme-shell-scsh), Previous: [Expect](07_16_expect.md#716-expect), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
