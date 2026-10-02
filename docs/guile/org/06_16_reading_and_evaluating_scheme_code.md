### 6.16 Reading and Evaluating Scheme Code [¶](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)

This chapter describes Guile functions that are concerned with reading, loading, evaluating, and compiling Scheme code at run time.

*   [Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions)
*   [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code)
*   [Reading Scheme Code, For the Compiler](06_16_reading_and_evaluating_scheme_code.md#6163-reading-scheme-code-for-the-compiler)
*   [Writing Scheme Values](06_16_reading_and_evaluating_scheme_code.md#6164-writing-scheme-values)
*   [Procedures for On the Fly Evaluation](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation)
*   [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code)
*   [Loading Scheme Code from File](06_16_reading_and_evaluating_scheme_code.md#6167-loading-scheme-code-from-file)
*   [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths)
*   [Character Encoding of Source Files](06_16_reading_and_evaluating_scheme_code.md#6169-character-encoding-of-source-files)
*   [Delayed Evaluation](06_16_reading_and_evaluating_scheme_code.md#61610-delayed-evaluation)
*   [Local Evaluation](06_16_reading_and_evaluating_scheme_code.md#61611-local-evaluation)
*   [Local Inclusion](06_16_reading_and_evaluating_scheme_code.md#61612-local-inclusion)
*   [Sandboxed Evaluation](06_16_reading_and_evaluating_scheme_code.md#61613-sandboxed-evaluation)
*   [REPL Servers](06_16_reading_and_evaluating_scheme_code.md#61614-repl-servers)
*   [Cooperative REPL Servers](06_16_reading_and_evaluating_scheme_code.md#61615-cooperative-repl-servers)

* * *

Next: [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.1 Scheme Syntax: Standard and Guile Extensions [¶](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions)

*   [Expression Syntax](06_16_reading_and_evaluating_scheme_code.md#61611-expression-syntax)
*   [Comments](06_16_reading_and_evaluating_scheme_code.md#61612-comments)
*   [Block Comments](06_16_reading_and_evaluating_scheme_code.md#61613-block-comments)
*   [Case Sensitivity](06_16_reading_and_evaluating_scheme_code.md#61614-case-sensitivity)
*   [Keyword Syntax](06_16_reading_and_evaluating_scheme_code.md#61615-keyword-syntax)
*   [Reader Extensions](06_16_reading_and_evaluating_scheme_code.md#61616-reader-extensions)

* * *

Next: [Comments](06_16_reading_and_evaluating_scheme_code.md#61612-comments), Up: [Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.1.1 Expression Syntax [¶](06_16_reading_and_evaluating_scheme_code.md#61611-expression-syntax)

An expression to be evaluated takes one of the following forms.

`symbol`

A symbol is evaluated by dereferencing. A binding of that symbol is sought and the value there used. For example,

(define x 123)
x ⇒ 123

`(proc args…)`

A parenthesized expression is a function call. proc and each argument are evaluated, then the function (which proc evaluated to) is called with those arguments.

The order in which proc and the arguments are evaluated is unspecified, so be careful when using expressions with side effects.

(max 1 2 3) ⇒ 3

(define (get-some-proc)  min)
((get-some-proc) 1 2 3) ⇒ 1

The same sort of parenthesized form is used for a macro invocation, but in that case the arguments are not evaluated. See the descriptions of macros for more on this (see [Macros](06_08_macros.md#68-macros), and see [Syntax-rules Macros](06_08_macros.md#682-syntax-rules-macros)).

`constant`

Number, string, character and boolean constants evaluate “to themselves”, so can appear as literals.

123     ⇒ 123
99.9    ⇒ 99.9
"hello" ⇒ "hello"
#\\z     ⇒ #\\z
#t      ⇒ #t

Note that an application must not attempt to modify literal strings, since they may be in read-only memory.

`(quote data)` [¶](06_16_reading_and_evaluating_scheme_code.md)

`'data`

Quoting is used to obtain a literal symbol (instead of a variable reference), a literal list (instead of a function call), or a literal vector. `'` is simply a shorthand for a `quote` form. For example,

'x                   ⇒ x
'(1 2 3)             ⇒ (1 2 3)
'#(1 (2 3) 4)        ⇒ #(1 (2 3) 4)
(quote x)            ⇒ x
(quote (1 2 3))      ⇒ (1 2 3)
(quote #(1 (2 3) 4)) ⇒ #(1 (2 3) 4)

Note that an application must not attempt to modify literal lists or vectors obtained from a `quote` form, since they may be in read-only memory.

`(quasiquote data)` [¶](06_16_reading_and_evaluating_scheme_code.md)

`` `data ``

Backquote quasi-quotation is like `quote`, but selected sub-expressions are evaluated. This is a convenient way to construct a list or vector structure most of which is constant, but at certain points should have expressions substituted.

The same effect can always be had with suitable `list`, `cons` or `vector` calls, but quasi-quoting is often easier.

`(unquote expr)` [¶](06_16_reading_and_evaluating_scheme_code.md)

`,expr`

Within the quasiquote data, `unquote` or `,` indicates an expression to be evaluated and inserted. The comma syntax `,` is simply a shorthand for an `unquote` form. For example,

\`(1 2 (\* 9 9) 3 4)       ⇒ (1 2 (\* 9 9) 3 4)
\`(1 2 ,(\* 9 9) 3 4)      ⇒ (1 2 81 3 4)
\`(1 (unquote (+ 1 1)) 3) ⇒ (1 2 3)
\`#(1 ,(/ 12 2))          ⇒ #(1 6)

`(unquote-splicing expr)` [¶](06_16_reading_and_evaluating_scheme_code.md)

`,@expr`

Within the quasiquote data, `unquote-splicing` or `,@` indicates an expression to be evaluated and the elements of the returned list inserted. expr must evaluate to a list. The “comma-at” syntax `,@` is simply a shorthand for an `unquote-splicing` form.

(define x '(2 3))
\`(1 ,x 4)                           ⇒ (1 (2 3) 4)
\`(1 ,@x 4)                         ⇒ (1 2 3 4)
\`(1 (unquote-splicing (map 1+ x)))  ⇒ (1 3 4)
\`#(9 ,@x 9)                        ⇒ #(9 2 3 9)

Notice `,@` differs from plain `,` in the way one level of nesting is stripped. For `,@` the elements of a returned list are inserted, whereas with `,` it would be the list itself inserted.

* * *

Next: [Block Comments](06_16_reading_and_evaluating_scheme_code.md#61613-block-comments), Previous: [Expression Syntax](06_16_reading_and_evaluating_scheme_code.md#61611-expression-syntax), Up: [Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.1.2 Comments [¶](06_16_reading_and_evaluating_scheme_code.md#61612-comments)

Comments in Scheme source files are written by starting them with a semicolon character (`;`). The comment then reaches up to the end of the line. Comments can begin at any column, and the may be inserted on the same line as Scheme code.

; Comment
;; Comment too
(define x 1)        ; Comment after expression
(let ((y 1))
  ;; Display something.
  ([display](06_16_reading_and_evaluating_scheme_code.md) y)
;;; Comment at left margin.
  ([display](06_16_reading_and_evaluating_scheme_code.md) ([+](06_06_02_numerical_data_types.md) y 1)))

It is common to use a single semicolon for comments following expressions on a line, to use two semicolons for comments which are indented like code, and three semicolons for comments which start at column 0, even if they are inside an indented code block. This convention is used when indenting code in Emacs’ Scheme mode.

* * *

Next: [Case Sensitivity](06_16_reading_and_evaluating_scheme_code.md#61614-case-sensitivity), Previous: [Comments](06_16_reading_and_evaluating_scheme_code.md#61612-comments), Up: [Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.1.3 Block Comments [¶](06_16_reading_and_evaluating_scheme_code.md#61613-block-comments)

In addition to the standard line comments defined by R5RS, Guile has another comment type for multiline comments, called _block comments_. This type of comment begins with the character sequence `#!` and ends with the characters `!#`.

These comments are compatible with the block comments in the Scheme Shell scsh (see [The Scheme shell (scsh)](07_18_the_scheme_shell_scsh.md#718-the-scheme-shell-scsh)). The characters `#!` were chosen because they are the magic characters used in shell scripts for indicating that the name of the program for executing the script follows on the same line.

Thus a Guile script often starts like this.

#! /usr/local/bin/guile \-s
!#

More details on Guile scripting can be found in the scripting section (see [Guile Scripting](04_programming_in_scheme.md#43-guile-scripting)).

Similarly, Guile (starting from version 2.0) supports nested block comments as specified by R6RS and [SRFI-30](http://srfi.schemers.org/srfi-30/srfi-30.html):

([+](06_06_02_numerical_data_types.md) 1 #| this is a #| nested |# block comment |# 2)
⇒ 3

For backward compatibility, this syntax can be overridden with `read-hash-extend` (see [`read-hash-extend`](06_16_reading_and_evaluating_scheme_code.md#61616-reader-extensions)).

There is one special case where the contents of a comment can actually affect the interpretation of code. When a character encoding declaration, such as `coding: utf-8` appears in one of the first few lines of a source file, it indicates to Guile’s default reader that this source code file is not ASCII. For details see [Character Encoding of Source Files](06_16_reading_and_evaluating_scheme_code.md#6169-character-encoding-of-source-files).

* * *

Next: [Keyword Syntax](06_16_reading_and_evaluating_scheme_code.md#61615-keyword-syntax), Previous: [Block Comments](06_16_reading_and_evaluating_scheme_code.md#61613-block-comments), Up: [Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.1.4 Case Sensitivity [¶](06_16_reading_and_evaluating_scheme_code.md#61614-case-sensitivity)

Scheme as defined in R5RS is not case sensitive when reading symbols. Guile, on the contrary is case sensitive by default, so the identifiers

guile-whuzzy
Guile-Whuzzy

are the same in R5RS Scheme, but are different in Guile.

It is possible to turn off case sensitivity in Guile by setting the reader option `case-insensitive`. For more information on reader options, See [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code).

([read-enable](06_16_reading_and_evaluating_scheme_code.md) 'case-insensitive)

It is also possible to disable (or enable) case sensitivity within a single file by placing the reader directives `#!fold-case` (or `#!no-fold-case`) within the file itself.

* * *

Next: [Reader Extensions](06_16_reading_and_evaluating_scheme_code.md#61616-reader-extensions), Previous: [Case Sensitivity](06_16_reading_and_evaluating_scheme_code.md#61614-case-sensitivity), Up: [Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.1.5 Keyword Syntax [¶](06_16_reading_and_evaluating_scheme_code.md#61615-keyword-syntax)

* * *

Previous: [Keyword Syntax](06_16_reading_and_evaluating_scheme_code.md#61615-keyword-syntax), Up: [Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.1.6 Reader Extensions [¶](06_16_reading_and_evaluating_scheme_code.md#61616-reader-extensions)

Scheme Procedure: **read-hash-extend** chr proc [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_read\_hash\_extend** (chr, proc) [¶](06_16_reading_and_evaluating_scheme_code.md)

Install the procedure proc for reading expressions starting with the character sequence `#` and chr. proc will be called with two arguments: the character chr and the port to read further data from. The object returned will be the return value of `read`. Passing `#f` for proc will remove a previous setting.

* * *

Next: [Reading Scheme Code, For the Compiler](06_16_reading_and_evaluating_scheme_code.md#6163-reading-scheme-code-for-the-compiler), Previous: [Scheme Syntax: Standard and Guile Extensions](06_16_reading_and_evaluating_scheme_code.md#6161-scheme-syntax-standard-and-guile-extensions), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.2 Reading Scheme Code [¶](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code)

Scheme Procedure: **read** \[port\] [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_read** (port) [¶](06_16_reading_and_evaluating_scheme_code.md)

Read an s-expression from the input port port, or from the current input port if port is not specified. Any whitespace before the next token is discarded.

The behavior of Guile’s Scheme reader can be modified by manipulating its read options.

Scheme Procedure: **read-options** \[setting\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Display the current settings of the global read options. If setting is omitted, only a short form of the current read options is printed. Otherwise if setting is the symbol `help`, a complete options description is displayed.

The set of available options, and their default values, may be had by invoking `read-options` at the prompt.

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([read-options](06_16_reading_and_evaluating_scheme_code.md))
(square-brackets keywords #f positions)
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([read-options](06_16_reading_and_evaluating_scheme_code.md) 'help)
positions         yes   Record positions of source code expressions.
case-insensitive  no    Convert symbols to lower case.
keywords          #f    Style of keyword recognition: #f, 'prefix or 'postfix.
r6rs-hex-escapes  no    Use R6RS variable-length character and [string](06_06_05_strings.md) hex escapes.
square-brackets   yes   Treat \`\[' and \`\]' as parentheses, for R6RS compatibility.
hungry-eol-escapes no   In strings, consume leading whitespace after an
                        escaped end-of-line.
curly-infix       no    Support SRFI-105 curly infix expressions.
r7rs-symbols      no    Support R7RS |...| [symbol](06_06_06_symbols.md) notation.
bytestrings       no    Support SRFI-207 #u8"\\xce;\\xbb; calculus" bytestrings

Note that Guile also includes a preliminary mechanism for setting read options on a per-port basis. For instance, the `case-insensitive` read option is set (or unset) on the port when the reader encounters the `#!fold-case` or `#!no-fold-case` reader directives. Similarly, the `#!curly-infix` reader directive sets the `curly-infix` read option on the port, and `#!curly-infix-and-bracket-lists` sets `curly-infix` and unsets `square-brackets` on the port (see [SRFI-105 Curly-infix expressions.](07_05_44_srfi105_curlyinfix_expressions.md#7544-srfi-105-curly-infix-expressions)). There is currently no other way to access or set the per-port read options.

The boolean options may be toggled with `read-enable` and `read-disable`. The non-boolean `keywords` option must be set using `read-set!`.

Scheme Procedure: **read-enable** option-name [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Procedure: **read-disable** option-name [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Syntax: **read-set!** option-name value [¶](06_16_reading_and_evaluating_scheme_code.md)

Modify the read options. `read-enable` should be used with boolean options and switches them on, `read-disable` switches them off.

`read-set!` can be used to set an option to a specific value. Due to historical oddities, it is a macro that expects an unquoted option name.

For example, to make `read` fold all symbols to their lower case (perhaps for compatibility with older Scheme code), you can enter:

([read-enable](06_16_reading_and_evaluating_scheme_code.md) 'case-insensitive)

For more information on the effect of the `r6rs-hex-escapes` and `hungry-eol-escapes` options, see (see [String Read Syntax](06_06_05_strings.md#6651-string-read-syntax)).

For more information on the `r7rs-symbols` option, see (see [Extended Read Syntax for Symbols](06_06_06_symbols.md#6665-extended-read-syntax-for-symbols)).

* * *

Next: [Writing Scheme Values](06_16_reading_and_evaluating_scheme_code.md#6164-writing-scheme-values), Previous: [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.3 Reading Scheme Code, For the Compiler [¶](06_16_reading_and_evaluating_scheme_code.md#6163-reading-scheme-code-for-the-compiler)

When something goes wrong with a Scheme program, the user will want to know how to fix it. This starts with identifying where the error occurred: we want to associate a source location with each component part of source code, and propagate that source location information through to the compiler or interpreter.

For that, Guile provides `read-syntax`.

Scheme Procedure: **read-syntax** \[port\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Read an s-expression from the input port port, or from the current input port if port is not specified.

If, after skipping white space and comments, no more bytes are available from port, return the end-of-file object. See [Binary I/O](06_12_input_and_output.md#6122-binary-io). Otherwise, return an annotated datum. An annotated datum is a syntax object which associates a source location with a datum. For example:

(call-with-input-string "  foo" read-syntax)
; ⇒ #<syntax:unknown file:1:2 foo>
(call-with-input-string "(foo)" read-syntax)
; ⇒
; #<syntax:unknown file:1:0
;   (#<syntax unknown file:1:1 foo>)>

As the second example shows, all fields of pairs and vectors are also annotated, recursively.

Most users are familiar with syntax objects in the context of macros, which use syntax objects to associate scope information with identifiers. See [Macros](06_08_macros.md#68-macros). Here we use syntax objects to associate source location information with any datum, but without attaching scope information. The Scheme compiler (`compile`) and the interpreter (`eval`) can accept syntax objects directly as input, allowing them to associate source information with resulting code. See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), and See [Procedures for On the Fly Evaluation](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation).

Note that there is a legacy interface for getting source locations into the Scheme compiler or interpreter, which is to use a side table that associates “source properties” with each subdatum returned by `read`, instead of wrapping the datums directly as in `read-syntax`. This has the disadvantage of not being able to annotate all kinds of datums. See [Source Properties](06_26_debugging_infrastructure.md#6263-source-properties), for more information.

* * *

Next: [Procedures for On the Fly Evaluation](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation), Previous: [Reading Scheme Code, For the Compiler](06_16_reading_and_evaluating_scheme_code.md#6163-reading-scheme-code-for-the-compiler), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.4 Writing Scheme Values [¶](06_16_reading_and_evaluating_scheme_code.md#6164-writing-scheme-values)

Any scheme value may be written to a port. Not all values may be read back in (see [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code)), however.

Scheme Procedure: **write** obj \[port\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Send a representation of obj to port or to the current output port if not given.

The output is designed to be machine readable, and can be read back with `read` (see [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code)). Strings are printed in double quotes, with escapes if necessary, and characters are printed in ‘#\\’ notation.

Scheme Procedure: **display** obj \[port\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Send a representation of obj to port or to the current output port if not given.

The output is designed for human readability, it differs from `write` in that strings are printed without double quotes and escapes, and characters are printed as per `write-char`, not in ‘#\\’ form.

As was the case with the Scheme reader, there are a few options that affect the behavior of the Scheme printer.

Scheme Procedure: **print-options** \[setting\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Display the current settings of the read options. If setting is omitted, only a short form of the current read options is printed. Otherwise if setting is the symbol `help`, a complete options description is displayed.

The set of available options, and their default values, may be had by invoking `print-options` at the prompt.

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([print-options](06_16_reading_and_evaluating_scheme_code.md))
(quote-keywordish-symbols reader highlight-suffix "}" highlight-prefix "{")
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([print-options](06_16_reading_and_evaluating_scheme_code.md) 'help)
highlight-prefix          {       The [string](06_06_05_strings.md) to print before highlighted values.
highlight-suffix          }       The [string](06_06_05_strings.md) to print after highlighted values.
quote-keywordish-symbols  reader  How to print symbols that have a colon
                                  as their [first](07_05_03_srfi1_list_library.md) or [last](07_05_03_srfi1_list_library.md) character. The
                                  value '#f' does [not](06_06_01_booleans.md) [quote](07_06_r6rs_support.md) the colons;
                                  '#t' quotes them; 'reader' quotes them
                                  [when](06_11_controlling_the_flow_of_program_execution.md) the reader [option](04_programming_in_scheme.md) 'keywords' is
                                  [not](06_06_01_booleans.md) '#f'.
escape-newlines           yes     Render newlines as \\n [when](06_11_controlling_the_flow_of_program_execution.md) printing
                                  using \`write'. 
r7rs-symbols              no      Escape symbols using R7RS |...| [symbol](06_06_06_symbols.md)
                                  notation.
bytestrings               no      Render bytevectors as bytestrings (SRFI-207)

These options may be modified with the print-set! syntax.

Scheme Syntax: **print-set!** option-name value [¶](06_16_reading_and_evaluating_scheme_code.md)

Modify the print options. Due to historical oddities, `print-set!` is a macro that expects an unquoted option name.

* * *

Next: [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), Previous: [Writing Scheme Values](06_16_reading_and_evaluating_scheme_code.md#6164-writing-scheme-values), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.5 Procedures for On the Fly Evaluation [¶](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation)

Scheme has the lovely property that its expressions may be represented as data. The `eval` procedure takes a Scheme datum and evaluates it as code.

Scheme Procedure: **eval** exp module\_or\_state [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_eval** (exp, module\_or\_state) [¶](06_16_reading_and_evaluating_scheme_code.md)

Evaluate exp, a list representing a Scheme expression, in the top-level environment specified by module\_or\_state. While exp is evaluated (using `primitive-eval`), module\_or\_state is made the current module. The current module is reset to its previous value when `eval` returns. XXX - dynamic states. Example: (eval ’(+ 1 2) (interaction-environment))

Scheme Procedure: **interaction-environment** [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_interaction\_environment** () [¶](06_16_reading_and_evaluating_scheme_code.md)

Return a specifier for the environment that contains implementation–defined bindings, typically a superset of those listed in the report. The intent is that this procedure will return the environment in which the implementation would evaluate expressions dynamically typed by the user.

See [Environments](06_18_modules.md#61812-environments), for other environments.

One does not always receive code as Scheme data, of course, and this is especially the case for Guile’s other language implementations (see [Support for Other Languages](06_24_support_for_other_languages.md#624-support-for-other-languages)). For the case in which all you have is a string, we have `eval-string`. There is a legacy version of this procedure in the default environment, but you really want the one from `(ice-9 eval-string)`, so load it up:

(use-modules (ice-9 eval-string))

Scheme Procedure: **eval-string** string \[#:module=#f\] \[#:file=#f\] \[#:line=#f\] \[#:column=#f\] \[#:lang=(current-language)\] \[#:compile?=#f\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Parse string according to the current language, normally Scheme. Evaluate or compile the expressions it contains, in order, returning the last expression.

If the module keyword argument is set, save a module excursion (see [Module System Reflection](06_18_modules.md#6188-module-system-reflection)) and set the current module to module before evaluation.

The file, line, and column keyword arguments can be used to indicate that the source string begins at a particular source location.

Finally, lang is a language, defaulting to the current language, and the expression is compiled if compile? is true or there is no evaluator for the given language.

C Function: **scm\_eval\_string** (string) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_eval\_string\_in\_module** (string, module) [¶](06_16_reading_and_evaluating_scheme_code.md)

These C bindings call `eval-string` from `(ice-9 eval-string)`, evaluating within module or the current module.

C Function: `SCM` **scm\_c\_eval\_string** `(const char *string)` [¶](06_16_reading_and_evaluating_scheme_code.md)

`scm_eval_string`, but taking a C string in locale encoding instead of an `SCM`.

Scheme Procedure: **apply** proc arg … arglst [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_apply\_0** (proc, arglst) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_apply\_1** (proc, arg1, arglst) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_apply\_2** (proc, arg1, arg2, arglst) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_apply\_3** (proc, arg1, arg2, arg3, arglst) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_apply** (proc, arg, rest) [¶](06_16_reading_and_evaluating_scheme_code.md)

Call proc with arguments arg … and the elements of the arglst list.

`scm_apply` takes parameters corresponding to a Scheme level `(lambda (proc arg1 . rest) ...)`. So arg1 and all but the last element of the rest list make up arg …, and the last element of rest is the arglst list. Or if rest is the empty list `SCM_EOL` then there’s no arg …, and (arg1) is the arglst.

arglst is not modified, but the rest list passed to `scm_apply` is modified.

C Function: **scm\_call\_0** (proc) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_call\_1** (proc, arg1) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_call\_2** (proc, arg1, arg2) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_call\_3** (proc, arg1, arg2, arg3) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_call\_4** (proc, arg1, arg2, arg3, arg4) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_call\_5** (proc, arg1, arg2, arg3, arg4, arg5) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_call\_6** (proc, arg1, arg2, arg3, arg4, arg5, arg6) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_call\_7** (proc, arg1, arg2, arg3, arg4, arg5, arg6, arg7) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_call\_8** (proc, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_call\_9** (proc, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) [¶](06_16_reading_and_evaluating_scheme_code.md)

Call proc with the given arguments.

C Function: **scm\_call** (proc, ...) [¶](06_16_reading_and_evaluating_scheme_code.md)

Call proc with any number of arguments. The argument list must be terminated by `SCM_UNDEFINED`. For example:

scm\_call (scm\_c\_public\_ref ("guile", "+"),
          scm\_from\_int (1),
          scm\_from\_int (2),
          SCM\_UNDEFINED);

C Function: **scm\_call\_n** (proc, argv, nargs) [¶](06_16_reading_and_evaluating_scheme_code.md)

Call proc with the array of arguments argv, as a `SCM*`. The length of the arguments should be passed in nargs, as a `size_t`.

Scheme Procedure: **primitive-eval** exp [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_primitive\_eval** (exp) [¶](06_16_reading_and_evaluating_scheme_code.md)

Evaluate exp in the top-level environment specified by the current module.

* * *

Next: [Loading Scheme Code from File](06_16_reading_and_evaluating_scheme_code.md#6167-loading-scheme-code-from-file), Previous: [Procedures for On the Fly Evaluation](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.6 Compiling Scheme Code [¶](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code)

The `eval` procedure directly interprets the S-expression representation of Scheme. An alternate strategy for evaluation is to determine ahead of time what computations will be necessary to evaluate the expression, and then use that recipe to produce the desired results. This is known as _compilation_.

While it is possible to compile simple Scheme expressions such as `(+ 2 2)` or even `"Hello world!"`, compilation is most interesting in the context of procedures. Compiling a lambda expression produces a compiled procedure, which is just like a normal procedure except typically much faster, because it can bypass the generic interpreter.

Functions from system modules in a Guile installation are normally compiled already, so they load and run quickly.

Note that well-written Scheme programs will not typically call the procedures in this section, for the same reason that it is often bad taste to use `eval`. By default, Guile automatically compiles any files it encounters that have not been compiled yet (see [`--auto-compile`](04_programming_in_scheme.md#42-invoking-guile)). The compiler can also be invoked explicitly from the shell as `guild compile foo.scm`.

(Why are calls to `eval` and `compile` usually in bad taste? Because they are limited, in that they can only really make sense for top-level expressions. Also, most needs for “compile-time” computation are fulfilled by macros and closures. Of course one good counterexample is the REPL itself, or any code that reads expressions from a port.)

Automatic compilation generally works transparently, without any need for user intervention. However Guile does not yet do proper dependency tracking, so that if file a.scm uses macros from b.scm, and b.scm changes, `a.scm` would not be automatically recompiled. To forcibly invalidate the auto-compilation cache, pass the `--fresh-auto-compile` option to Guile, or set the `GUILE_AUTO_COMPILE` environment variable to `fresh` (instead of to `0` or `1`).

For more information on the compiler itself, see [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine). For information on the virtual machine, see [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile).

The command-line interface to Guile’s compiler is the `guild compile` command:

Command: **guild compile** \[option...\] file... [¶](06_16_reading_and_evaluating_scheme_code.md)

Compile file, a source file, and store bytecode in the compilation cache or in the file specified by the \-o option. The following options are available:

\-L dir

\--load-path=dir

Add dir to the front of the module load path.

\-o ofile

\--output=ofile

Write output bytecode to ofile. By convention, bytecode file names end in `.go`. When \-o is omitted, the output file name is as for `compile-file` (see below).

\-x extension

Recognize extension as a valid source file name extension.

For example, to compile R6RS code, you might want to pass `-x .sls` so that files ending in .sls can be found.

\-W warning [¶](06_16_reading_and_evaluating_scheme_code.md)

\--warn=warning

Enable specific warning passes; use `-Whelp` for a list of available options. The default is `-W1`, which enables a number of common warnings. Pass `-W0` to disable all warnings.

\-O opt [¶](06_16_reading_and_evaluating_scheme_code.md)

\--optimize=opt

Enable or disable specific compiler optimizations; use `-Ohelp` for a list of available options. The default is `-O2`, which enables most optimizations. `-O0` is recommended if compilation speed is more important than the speed of the compiled code. Pass `-Ono-opt` to disable a specific compiler pass. Any number of `-O` options can be passed to the compiler, with later ones taking precedence.

\--r6rs

\--r7rs

Compile in an environment whose default bindings, reader options, and load paths are adapted for specific Scheme standards. See [R6RS Support](07_06_r6rs_support.md#76-r6rs-support), and See [R7RS Support](07_07_r7rs_support.md#77-r7rs-support).

\-f lang

\--from=lang

Use lang as the source language of file. If this option is omitted, `scheme` is assumed.

\-t lang

\--to=lang

Use lang as the target language of file. If this option is omitted, `rtl` is assumed.

\-T target

\--target=target

Produce code for target instead of %host-type (see [%host-type](06_23_configuration_features_and_runtime_options.md#6231-configuration-build-and-installation)). Target must be a valid GNU triplet, such as `armv5tel-unknown-linux-gnueabi` (see [Specifying Target Triplets](https://www.gnu.org/software/autoconf/manual/autoconf.html#Specifying-Target-Triplets) in GNU Autoconf Manual).

Each file is assumed to be UTF-8-encoded, unless it contains a coding declaration as recognized by `file-encoding` (see [Character Encoding of Source Files](06_16_reading_and_evaluating_scheme_code.md#6169-character-encoding-of-source-files)).

The compiler can also be invoked directly by Scheme code. These interfaces are in their own module:

(use-modules (system base compile))

Scheme Procedure: **compile** exp \[#:env=#f\] \[#:from=(current-language)\] \[#:to=value\] \[#:opts=’()\] \[#:optimization-level=(default-optimization-level)\] \[#:warning-level=(default-warning-level)\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Compile the expression exp in the environment env. If exp is a procedure, the result will be a compiled procedure; otherwise `compile` is mostly equivalent to `eval`.

For a discussion of languages and compiler options, See [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine).

Scheme Procedure: **compile-file** file \[#:output-file=#f\] \[#:from=(current-language)\] \[#:to=’rtl\] \[#:env=(default-environment from)\] \[#:opts=’()\] \[#:optimization-level=(default-optimization-level)\] \[#:warning-level=(default-warning-level)\] \[#:canonicalization=’relative\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Compile the file named file.

Output will be written to a output-file. If you do not supply an output file name, output is written to a file in the cache directory, as computed by `(compiled-file-name file)`.

from and to specify the source and target languages. See [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine), for more information on these options, and on env and opts.

As with `guild compile`, file is assumed to be UTF-8-encoded unless it contains a coding declaration.

Scheme Parameter: **default-optimization-level** [¶](06_16_reading_and_evaluating_scheme_code.md)

The default optimization level, as an integer from 0 to 9. The default is 2.

Scheme Parameter: **default-warning-level** [¶](06_16_reading_and_evaluating_scheme_code.md)

The default warning level, as an integer from 0 to 9. The default is 1.

See [Parameters](06_11_controlling_the_flow_of_program_execution.md#61112-parameters), for more on how to set parameters.

Scheme Procedure: **compiled-file-name** file [¶](06_16_reading_and_evaluating_scheme_code.md)

Compute a cached location for a compiled version of a Scheme file named file.

This file will usually be below the $HOME/.cache/guile/ccache directory, depending on the value of the `XDG_CACHE_HOME` environment variable. The intention is that `compiled-file-name` provides a fallback location for caching auto-compiled files. If you want to place a compile file in the `%load-compiled-path`, you should pass the output-file option to `compile-file`, explicitly.

Scheme Variable: **%auto-compilation-options** [¶](06_16_reading_and_evaluating_scheme_code.md)

This variable contains the options passed to the `compile-file` procedure when auto-compiling source files. By default, it enables useful compilation warnings. It can be customized from ~/.guile.

* * *

Next: [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths), Previous: [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.7 Loading Scheme Code from File [¶](06_16_reading_and_evaluating_scheme_code.md#6167-loading-scheme-code-from-file)

Scheme Procedure: **load** filename \[reader\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Load filename and evaluate its contents in the top-level environment.

reader if provided should be either `#f`, or a procedure with the signature `(lambda (port) …)` which reads the next expression from port. If reader is `#f` or absent, Guile’s built-in `read` procedure is used (see [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code)).

The reader argument takes effect by setting the value of the `current-reader` fluid (see below) before loading the file, and restoring its previous value when loading is complete. The Scheme code inside filename can itself change the current reader procedure on the fly by setting `current-reader` fluid.

If the variable `%load-hook` is defined, it should be bound to a procedure that will be called before any code is loaded. See documentation for `%load-hook` later in this section.

Scheme Procedure: **load-compiled** filename [¶](06_16_reading_and_evaluating_scheme_code.md)

Load the compiled file named filename.

Compiling a source file (see [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)) and then calling `load-compiled` on the resulting file is equivalent to calling `load` on the source file.

Scheme Procedure: **primitive-load** filename [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_primitive\_load** (filename) [¶](06_16_reading_and_evaluating_scheme_code.md)

Load the file named filename and evaluate its contents in the top-level environment. filename must either be a full pathname or be a pathname relative to the current directory. If the variable `%load-hook` is defined, it should be bound to a procedure that will be called before any code is loaded. See the documentation for `%load-hook` later in this section.

C Function: `SCM` **scm\_c\_primitive\_load** `(const char *filename)` [¶](06_16_reading_and_evaluating_scheme_code.md)

`scm_primitive_load`, but taking a C string instead of an `SCM`.

Variable: **current-reader** [¶](06_16_reading_and_evaluating_scheme_code.md)

`current-reader` holds the read procedure that is currently being used by the above loading procedures to read expressions (from the file that they are loading). `current-reader` is a fluid, so it has an independent value in each dynamic root and should be read and set using `fluid-ref` and `fluid-set!` (see [Fluids and Dynamic States](06_11_controlling_the_flow_of_program_execution.md#61111-fluids-and-dynamic-states)).

Changing `current-reader` is typically useful to introduce local syntactic changes, such that code following the `fluid-set!` call is read using the newly installed reader. The `current-reader` change should take place at evaluation time when the code is evaluated, or at compilation time when the code is compiled:

(eval-when (compile eval)
  (fluid-set! current-reader my-own-reader))

The `eval-when` form above ensures that the `current-reader` change occurs at the right time.

Variable: **%load-hook** [¶](06_16_reading_and_evaluating_scheme_code.md)

A procedure to be called `(%load-hook filename)` whenever a file is loaded, or `#f` for no such call. `%load-hook` is used by all of the loading functions (`load` and `primitive-load`, and `load-from-path` and `primitive-load-path` documented in the next section).

For example an application can set this to show what’s loaded,

(set! %load-hook (lambda (filename)
                   (format #t "Loading ~a ...\\n" filename)))
(load-from-path "foo.scm")
⊣ Loading /usr/local/share/guile/site/foo.scm ...

Scheme Procedure: **current-load-port** [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_current\_load\_port** () [¶](06_16_reading_and_evaluating_scheme_code.md)

Return the current-load-port. The load port is used internally by `primitive-load`.

* * *

Next: [Character Encoding of Source Files](06_16_reading_and_evaluating_scheme_code.md#6169-character-encoding-of-source-files), Previous: [Loading Scheme Code from File](06_16_reading_and_evaluating_scheme_code.md#6167-loading-scheme-code-from-file), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.8 Load Paths [¶](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths)

The procedure in the previous section look for Scheme code in the file system at specific location. Guile also has some procedures to search the load path for code.

Variable: **%load-path** [¶](06_16_reading_and_evaluating_scheme_code.md)

List of directories which should be searched for Scheme modules and libraries. When Guile starts up, `%load-path` is initialized to the default load path `(list (%library-dir) (%site-dir) (%global-site-dir) (%package-data-dir))`. The `GUILE_LOAD_PATH` environment variable can be used to prepend or append additional directories (see [Environment Variables](04_programming_in_scheme.md#422-environment-variables)).

See [Configuration, Build and Installation](06_23_configuration_features_and_runtime_options.md#6231-configuration-build-and-installation), for more on `%site-dir` and related procedures.

Scheme Procedure: **load-from-path** filename [¶](06_16_reading_and_evaluating_scheme_code.md)

Similar to `load`, but searches for filename in the load paths. Preferentially loads a compiled version of the file, if it is available and up-to-date.

A user can extend the load path by calling `add-to-load-path`.

Scheme Syntax: **add-to-load-path** dir [¶](06_16_reading_and_evaluating_scheme_code.md)

Add dir to the load path.

For example, a script might include this form to add the directory that it is in to the load path:

(add-to-load-path (dirname (current-filename)))

It’s better to use `add-to-load-path` than to modify `%load-path` directly, because `add-to-load-path` takes care of modifying the path both at compile-time and at run-time.

Scheme Procedure: **primitive-load-path** filename \[exception-on-not-found\] [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_primitive\_load\_path** (filename) [¶](06_16_reading_and_evaluating_scheme_code.md)

Search `%load-path` for the file named filename and load it into the top-level environment. If filename is a relative pathname and is not found in the list of search paths, an error is signaled. Preferentially loads a compiled version of the file, if it is available and up-to-date.

If filename is a relative pathname and is not found in the list of search paths, one of three things may happen, depending on the optional second argument, exception-on-not-found. If it is `#f`, `#f` will be returned. If it is a procedure, it will be called with no arguments. (This allows a distinction to be made between exceptions raised by loading a file, and exceptions related to the loader itself.) Otherwise an error is signaled.

For compatibility with Guile 1.8 and earlier, the C function takes only one argument, which can be either a string (the file name) or an argument list.

Scheme Procedure: **%search-load-path** filename [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_sys\_search\_load\_path** (filename) [¶](06_16_reading_and_evaluating_scheme_code.md)

Search `%load-path` for the file named filename, which must be readable by the current user. If filename is found in the list of paths to search or is an absolute pathname, return its full pathname. Otherwise, return `#f`. Filenames may have any of the optional extensions in the `%load-extensions` list; `%search-load-path` will try each extension automatically.

Variable: **%load-extensions** [¶](06_16_reading_and_evaluating_scheme_code.md)

A list of default file extensions for files containing Scheme code. `%search-load-path` tries each of these extensions when looking for a file to load. By default, `%load-extensions` is bound to the list `("" ".scm")`.

As mentioned above, when Guile searches the `%load-path` for a source file, it will also search the `%load-compiled-path` for a corresponding compiled file. If the compiled file is as new or newer than the source file, it will be loaded instead of the source file, using `load-compiled`.

Variable: **%load-compiled-path** [¶](06_16_reading_and_evaluating_scheme_code.md)

Like `%load-path`, but for compiled files. By default, this path has two entries: one for compiled files from Guile itself, and one for site packages. The `GUILE_LOAD_COMPILED_PATH` environment variable can be used to prepend or append additional directories (see [Environment Variables](04_programming_in_scheme.md#422-environment-variables)).

When `primitive-load-path` searches the `%load-compiled-path` for a corresponding compiled file for a relative path it does so by appending `.go` to the relative path. For example, searching for `ice-9/popen` could find `/usr/lib/guile/3.0/ccache/ice-9/popen.go`, and use it instead of `/usr/share/guile/3.0/ice-9/popen.scm`.

If `primitive-load-path` does not find a corresponding `.go` file in the `%load-compiled-path`, or the `.go` file is out of date, it will search for a corresponding auto-compiled file in the fallback path, possibly creating one if one does not exist.

See [Installing Site Packages](04_programming_in_scheme.md#47-installing-site-packages), for more on how to correctly install site packages. See [Modules and the File System](06_18_modules.md#6184-modules-and-the-file-system), for more on the relationship between load paths and modules. See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), for more on the fallback path and auto-compilation.

Finally, there are a couple of helper procedures for general path manipulation.

Scheme Procedure: **parse-path** path \[tail\] [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_parse\_path** (path, tail) [¶](06_16_reading_and_evaluating_scheme_code.md)

Parse path, which is expected to be a colon-separated string, into a list and return the resulting list with tail appended. If path is `#f`, tail is returned.

Scheme Procedure: **parse-path-with-ellipsis** path base [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_parse\_path\_with\_ellipsis** (path, base) [¶](06_16_reading_and_evaluating_scheme_code.md)

Parse path, which is expected to be a colon-separated string, into a list and return the resulting list with base (a list) spliced in place of the `...` path component, if present, or else base is added to the end. If path is `#f`, base is returned.

Scheme Procedure: **search-path** path filename \[extensions \[require-exts?\]\] [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_search\_path** (path, filename, rest) [¶](06_16_reading_and_evaluating_scheme_code.md)

Search path for a directory containing a file named filename. The file must be readable, and not a directory. If we find one, return its full filename; otherwise, return `#f`. If filename is absolute, return it unchanged. If given, extensions is a list of strings; for each directory in path, we search for filename concatenated with each extension. If require-exts? is true, require that the returned file name have one of the given extensions; if require-exts? is not given, it defaults to `#f`.

For compatibility with Guile 1.8 and earlier, the C function takes only three arguments.

* * *

Next: [Delayed Evaluation](06_16_reading_and_evaluating_scheme_code.md#61610-delayed-evaluation), Previous: [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.9 Character Encoding of Source Files [¶](06_16_reading_and_evaluating_scheme_code.md#6169-character-encoding-of-source-files)

Scheme source code files are usually encoded in ASCII or UTF-8, but the built-in reader can interpret other character encodings as well. When Guile loads Scheme source code, it uses the `file-encoding` procedure (described below) to try to guess the encoding of the file. In the absence of any hints, UTF-8 is assumed. One way to provide a hint about the encoding of a source file is to place a coding declaration in the top 500 characters of the file.

A coding declaration has the form `coding: XXXXXX`, where `XXXXXX` is the name of a character encoding in which the source code file has been encoded. The coding declaration must appear in a scheme comment. It can either be a semicolon-initiated comment, or the first block `#!` comment in the file.

The name of the character encoding in the coding declaration is typically lower case and containing only letters, numbers, and hyphens, as recognized by `set-port-encoding!` (see [`set-port-encoding!`](06_12_input_and_output.md#6121-ports)). Common examples of character encoding names are `utf-8` and `iso-8859-1`, [as defined by IANA](http://www.iana.org/assignments/character-sets). Thus, the coding declaration is mostly compatible with Emacs.

However, there are some differences in encoding names recognized by Emacs and encoding names defined by IANA, the latter being essentially a subset of the former. For instance, `latin-1` is a valid encoding name for Emacs, but it’s not according to the IANA standard, which Guile follows; instead, you should use `iso-8859-1`, which is both understood by Emacs and dubbed by IANA (IANA writes it uppercase but Emacs wants it lowercase and Guile is case insensitive.)

For source code, only a subset of all possible character encodings can be interpreted by the built-in source code reader. Only those character encodings in which ASCII text appears unmodified can be used. This includes `UTF-8` and `ISO-8859-1` through `ISO-8859-15`. The multi-byte character encodings `UTF-16` and `UTF-32` may not be used because they are not compatible with ASCII.

There might be a scenario in which one would want to read non-ASCII code from a port, such as with the function `read`, instead of with `load`. If the port’s character encoding is the same as the encoding of the code to be read by the port, not other special handling is necessary. The port will automatically do the character encoding conversion. The functions `setlocale` or by `set-port-encoding!` are used to set port encodings (see [Ports](06_12_input_and_output.md#6121-ports)).

If a port is used to read code of unknown character encoding, it can accomplish this in three steps. First, the character encoding of the port should be set to ISO-8859-1 using `set-port-encoding!`. Then, the procedure `file-encoding`, described below, is used to scan for a coding declaration when reading from the port. As a side effect, it rewinds the port after its scan is complete. After that, the port’s character encoding should be set to the encoding returned by `file-encoding`, if any, again by using `set-port-encoding!`. Then the code can be read as normal.

Alternatively, one can use the `#:guess-encoding` keyword argument of `open-file` and related procedures. See [File Ports](06_12_input_and_output.md#612101-file-ports).

Scheme Procedure: **file-encoding** port [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_file\_encoding** (port) [¶](06_16_reading_and_evaluating_scheme_code.md)

Attempt to scan the first few hundred bytes from the port for hints about its character encoding. Return a string containing the encoding name or `#f` if the encoding cannot be determined. The port is rewound.

Currently, the only supported method is to look for an Emacs-like character coding declaration (see [how Emacs recognizes file encoding](https://www.gnu.org/software/emacs/manual/html_mono/emacs.html#Recognize-Coding) in The GNU Emacs Reference Manual). The coding declaration is of the form `coding: XXXXX` and must appear in a Scheme comment. Additional heuristics may be added in the future.

* * *

Next: [Local Evaluation](06_16_reading_and_evaluating_scheme_code.md#61611-local-evaluation), Previous: [Character Encoding of Source Files](06_16_reading_and_evaluating_scheme_code.md#6169-character-encoding-of-source-files), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.10 Delayed Evaluation [¶](06_16_reading_and_evaluating_scheme_code.md#61610-delayed-evaluation)

Promises are a convenient way to defer a calculation until its result is actually needed, and to run such a calculation only once. Also see [SRFI-45 - Primitives for Expressing Iterative Lazy Algorithms](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md#7531-srfi-45---primitives-for-expressing-iterative-lazy-algorithms).

syntax: **delay** expr [¶](06_16_reading_and_evaluating_scheme_code.md)

Return a promise object which holds the given expr expression, ready to be evaluated by a later `force`.

Scheme Procedure: **promise?** obj [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_promise\_p** (obj) [¶](06_16_reading_and_evaluating_scheme_code.md)

Return true if obj is a promise.

Scheme Procedure: **force** p [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_force** (p) [¶](06_16_reading_and_evaluating_scheme_code.md)

Return the value obtained from evaluating the expr in the given promise p. If p has previously been forced then its expr is not evaluated again, instead the value obtained at that time is simply returned.

During a `force`, an expr can call `force` again on its own promise, resulting in a recursive evaluation of that expr. The first evaluation to return gives the value for the promise. Higher evaluations run to completion in the normal way, but their results are ignored, `force` always returns the first value.

* * *

Next: [Local Inclusion](06_16_reading_and_evaluating_scheme_code.md#61612-local-inclusion), Previous: [Delayed Evaluation](06_16_reading_and_evaluating_scheme_code.md#61610-delayed-evaluation), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.11 Local Evaluation [¶](06_16_reading_and_evaluating_scheme_code.md#61611-local-evaluation)

Guile includes a facility to capture a lexical environment, and later evaluate a new expression within that environment. This code is implemented in a module.

(use-modules (ice-9 local-eval))

syntax: **the-environment** [¶](06_16_reading_and_evaluating_scheme_code.md)

Captures and returns a lexical environment for use with `local-eval` or `local-compile`.

Scheme Procedure: **local-eval** exp env [¶](06_16_reading_and_evaluating_scheme_code.md)

C Function: **scm\_local\_eval** (exp, env) [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Procedure: **local-compile** exp env \[opts=()\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Evaluate or compile the expression exp in the lexical environment env.

Here is a simple example, illustrating that it is the variable that gets captured, not just its value at one point in time.

(define e (let ((x 100)) (the-environment)))
(define fetch-x (local-eval '(lambda () x) e))
(fetch-x)
⇒ 100
(local-eval '(set! x 42) e)
(fetch-x)
⇒ 42

While exp is evaluated within the lexical environment of `(the-environment)`, it has the dynamic environment of the call to `local-eval`.

`local-eval` and `local-compile` can only evaluate expressions, not definitions.

(local-eval '(define foo 42)
            (let ((x 100)) (the-environment)))
⇒ syntax error: definition in expression context

Note that the current implementation of `(the-environment)` only captures “normal” lexical bindings, and pattern variables bound by `syntax-case`. It does not currently capture local syntax transformers bound by `let-syntax`, `letrec-syntax` or non-top-level `define-syntax` forms. Any attempt to reference such captured syntactic keywords via `local-eval` or `local-compile` produces an error.

* * *

Next: [Sandboxed Evaluation](06_16_reading_and_evaluating_scheme_code.md#61613-sandboxed-evaluation), Previous: [Local Evaluation](06_16_reading_and_evaluating_scheme_code.md#61611-local-evaluation), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.12 Local Inclusion [¶](06_16_reading_and_evaluating_scheme_code.md#61612-local-inclusion)

This section has discussed various means of linking Scheme code together: fundamentally, loading up files at run-time using `load` and `load-compiled`. Guile provides another option to compose parts of programs together at expansion-time instead of at run-time.

Scheme Syntax: **include** file-name [¶](06_16_reading_and_evaluating_scheme_code.md)

Open file-name, at expansion-time, and read the Scheme forms that it contains, splicing them into the location of the `include`, within a `begin`.

If file-name is a relative path, it is searched for relative to the path that contains the file that the `include` form appears in.

If you are a C programmer, if `load` in Scheme is like `dlopen` in C, consider `include` to be like the C preprocessor’s `#include`. When you use `include`, it is as if the contents of the included file were typed in instead of the `include` form.

Because the code is included at compile-time, it is available to the macroexpander. Syntax definitions in the included file are available to later code in the form in which the `include` appears, without the need for `eval-when`. (See [Eval-when](06_08_macros.md#688-eval-when).)

For the same reason, compiling a form that uses `include` results in one compilation unit, composed of multiple files. Loading the compiled file is one `stat` operation for the compilation unit, instead of `2*n` in the case of `load` (once for each loaded source file, and once each corresponding compiled file, in the best case).

Unlike `load`, `include` also works within nested lexical contexts. It so happens that the optimizer works best within a lexical context, because all of the uses of bindings in a lexical context are visible, so composing files by including them within a `(let () ...)` can sometimes lead to important speed improvements.

On the other hand, `include` does have all the disadvantages of early binding: once the code with the `include` is compiled, no change to the included file is reflected in the future behavior of the including form.

Also, the particular form of `include`, which requires an absolute path, or a path relative to the current directory at compile-time, is not very amenable to compiling the source in one place, but then installing the source to another place. For this reason, Guile provides another form, `include-from-path`, which looks for the source file to include within a load path.

Scheme Syntax: **include-from-path** file-name [¶](06_16_reading_and_evaluating_scheme_code.md)

Like `include`, but instead of expecting `file-name` to be an absolute file name, it is expected to be a relative path to search in the `%load-path`.

`include-from-path` is more useful when you want to install all of the source files for a package (as you should!). It makes it possible to evaluate an installed file from source, instead of relying on the `.go` file being up to date.

* * *

Next: [REPL Servers](06_16_reading_and_evaluating_scheme_code.md#61614-repl-servers), Previous: [Local Inclusion](06_16_reading_and_evaluating_scheme_code.md#61612-local-inclusion), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.13 Sandboxed Evaluation [¶](06_16_reading_and_evaluating_scheme_code.md#61613-sandboxed-evaluation)

Sometimes you would like to evaluate code that comes from an untrusted party. The safest way to do this is to buy a new computer, evaluate the code on that computer, then throw the machine away. However if you are unwilling to take this simple approach, Guile does include a limited “sandbox” facility that can allow untrusted code to be evaluated with some confidence.

To use the sandboxed evaluator, load its module:

(use-modules (ice-9 sandbox))

Guile’s sandboxing facility starts with the ability to restrict the time and space used by a piece of code.

Scheme Procedure: **call-with-time-limit** limit thunk limit-reached [¶](06_16_reading_and_evaluating_scheme_code.md)

Call thunk, but cancel it if limit seconds of wall-clock time have elapsed. If the computation is canceled, call limit-reached in tail position. thunk must not disable interrupts or prevent an abort via a `dynamic-wind` unwind handler.

Scheme Procedure: **call-with-allocation-limit** limit thunk limit-reached [¶](06_16_reading_and_evaluating_scheme_code.md)

Call thunk, but cancel it if limit bytes have been allocated. If the computation is canceled, call limit-reached in tail position. thunk must not disable interrupts or prevent an abort via a `dynamic-wind` unwind handler.

This limit applies to both stack and heap allocation. The computation will not be aborted before limit bytes have been allocated, but for the heap allocation limit, the check may be postponed until the next garbage collection.

Note that as a current shortcoming, the heap size limit applies to all threads; concurrent allocation by other unrelated threads counts towards the allocation limit.

Scheme Procedure: **call-with-time-and-allocation-limits** time-limit allocation-limit thunk [¶](06_16_reading_and_evaluating_scheme_code.md)

Invoke thunk in a dynamic extent in which its execution is limited to time-limit seconds of wall-clock time, and its allocation to allocation-limit bytes. thunk must not disable interrupts or prevent an abort via a `dynamic-wind` unwind handler.

If successful, return all values produced by invoking thunk. Any uncaught exception thrown by the thunk will propagate out. If the time or allocation limit is exceeded, an exception will be thrown to the `limit-exceeded` key.

The time limit and stack limit are both very precise, but the heap limit only gets checked asynchronously, after a garbage collection. In particular, if the heap is already very large, the number of allocated bytes between garbage collections will be large, and therefore the precision of the check is reduced.

Additionally, due to the mechanism used by the allocation limit (the `after-gc-hook`), large single allocations like `(make-vector #e1e7)` are only detected after the allocation completes, even if the allocation itself causes garbage collection. It’s possible therefore for user code to not only exceed the allocation limit set, but also to exhaust all available memory, causing out-of-memory conditions at any allocation site. Failure to allocate memory in Guile itself should be safe and cause an exception to be thrown, but most systems are not designed to handle `malloc` failures. An allocation failure may therefore exercise unexpected code paths in your system, so it is a weakness of the sandbox (and therefore an interesting point of attack).

The main sandbox interface is `eval-in-sandbox`.

Scheme Procedure: **eval-in-sandbox** exp \[#:time-limit 0.1\] \[#:allocation-limit #e10e6\] \[#:bindings all-pure-bindings\] \[#:module (make-sandbox-module bindings)\] \[#:sever-module? #t\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Evaluate the Scheme expression exp within an isolated "sandbox". Limit its execution to time-limit seconds of wall-clock time, and limit its allocation to allocation-limit bytes.

The evaluation will occur in module, which defaults to the result of calling `make-sandbox-module` on bindings, which itself defaults to `all-pure-bindings`. This is the core of the sandbox: creating a scope for the expression that is _safe_.

A safe sandbox module has two characteristics. Firstly, it will not allow the expression being evaluated to avoid being canceled due to time or allocation limits. This ensures that the expression terminates in a timely fashion.

Secondly, a safe sandbox module will prevent the evaluation from receiving information from previous evaluations, or from affecting future evaluations. All combinations of binding sets exported by `(ice-9 sandbox)` form safe sandbox modules.

The bindings should be given as a list of import sets. One import set is a list whose car names an interface, like `(ice-9 q)`, and whose cdr is a list of imports. An import is either a bare symbol or a pair of `(out . in)`, where out and in are both symbols and denote the name under which a binding is exported from the module, and the name under which to make the binding available, respectively. Note that bindings is only used as an input to the default initializer for the module argument; if you pass `#:module`, bindings is unused. If sever-module? is true (the default), the module will be unlinked from the global module tree after the evaluation returns, to allow mod to be garbage-collected.

If successful, return all values produced by exp. Any uncaught exception thrown by the expression will propagate out. If the time or allocation limit is exceeded, an exception will be thrown to the `limit-exceeded` key.

Constructing a safe sandbox module is tricky in general. Guile defines an easy way to construct safe modules from predefined sets of bindings. Before getting to that interface, here are some general notes on safety.

1.  The time and allocation limits rely on the ability to interrupt and cancel a computation. For this reason, no binding included in a sandbox module should be able to indefinitely postpone interrupt handling, nor should a binding be able to prevent an abort. In practice this second consideration means that `dynamic-wind` should not be included in any binding set.
2.  The time and allocation limits apply only to the `eval-in-sandbox` call. If the call returns a procedure which is later called, no limit is “automatically” in place. Users of `eval-in-sandbox` have to be very careful to reimpose limits when calling procedures that escape from sandboxes.
3.  Similarly, the dynamic environment of the `eval-in-sandbox` call is not necessarily in place when any procedure that escapes from the sandbox is later called.
    
    This detail prevents us from exposing `primitive-eval` to the sandbox, for two reasons. The first is that it’s possible for legacy code to forge references to any binding, if the `allow-legacy-syntax-objects?` parameter is true. The default for this parameter is true; see [Syntax Transformer Helpers](06_08_macros.md#684-syntax-transformer-helpers) for the details. The parameter is bound to `#f` for the duration of the `eval-in-sandbox` call itself, but that will not be in place during calls to escaped procedures.
    
    The second reason we don’t expose `primitive-eval` is that `primitive-eval` implicitly works in the current module, which for an escaped procedure will probably be different than the module that is current for the `eval-in-sandbox` call itself.
    
    The common denominator here is that if an interface exposed to the sandbox relies on dynamic environments, it is easy to mistakenly grant the sandboxed procedure additional capabilities in the form of bindings that it should not have access to. For this reason, the default sets of predefined bindings do not depend on any dynamically scoped value.
    
4.  Mutation may allow a sandboxed evaluation to break some invariant in users of data supplied to it. A lot of code culturally doesn’t expect mutation, but if you hand mutable data to a sandboxed evaluation and you also grant mutating capabilities to that evaluation, then the sandboxed code may indeed mutate that data. The default set of bindings to the sandbox do not include any mutating primitives.
    
    Relatedly, `set!` may allow a sandbox to mutate a primitive, invalidating many system-wide invariants. Guile is currently quite permissive when it comes to imported bindings and mutability. Although `set!` to a module-local or lexically bound variable would be fine, we don’t currently have an easy way to disallow `set!` to an imported binding, so currently no binding set includes `set!`.
    
5.  Mutation may allow a sandboxed evaluation to keep state, or make a communication mechanism with other code. On the one hand this sounds cool, but on the other hand maybe this is part of your threat model. Again, the default set of bindings doesn’t include mutating primitives, preventing sandboxed evaluations from keeping state.
6.  The sandbox should probably not be able to open a network connection, or write to a file, or open a file from disk. The default binding set includes no interaction with the operating system.

If you, dear reader, find the above discussion interesting, you will enjoy Jonathan Rees’ dissertation, “A Security Kernel Based on the Lambda Calculus”.

Scheme Variable: **all-pure-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

All “pure” bindings that together form a safe subset of those bindings available by default to Guile user code.

Scheme Variable: **all-pure-and-impure-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Like `all-pure-bindings`, but additionally including mutating primitives like `vector-set!`. This set is still safe in the sense mentioned above, with the caveats about mutation.

The components of these composite sets are as follows:

Scheme Variable: **alist-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **array-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **bit-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **bitvector-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **char-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **char-set-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **clock-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **core-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **error-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **fluid-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **hash-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **iteration-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **keyword-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **list-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **macro-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **nil-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **number-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **pair-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **predicate-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **procedure-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **promise-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **prompt-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **regexp-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **sort-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **srfi-4-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **string-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **symbol-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **unspecified-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **variable-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **vector-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **version-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

The components of `all-pure-bindings`.

Scheme Variable: **mutating-alist-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-array-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-bitvector-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-fluid-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-hash-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-list-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-pair-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-sort-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-srfi-4-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-string-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-variable-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Variable: **mutating-vector-bindings** [¶](06_16_reading_and_evaluating_scheme_code.md)

The additional components of `all-pure-and-impure-bindings`.

Finally, what do you do with a binding set? What is a binding set anyway? `make-sandbox-module` is here for you.

Scheme Procedure: **make-sandbox-module** bindings [¶](06_16_reading_and_evaluating_scheme_code.md)

Return a fresh module that only contains bindings.

The bindings should be given as a list of import sets. One import set is a list whose car names an interface, like `(ice-9 q)`, and whose cdr is a list of imports. An import is either a bare symbol or a pair of `(out . in)`, where out and in are both symbols and denote the name under which a binding is exported from the module, and the name under which to make the binding available, respectively.

So you see that binding sets are just lists, and `all-pure-and-impure-bindings` is really just the result of appending all of the component binding sets.

* * *

Next: [Cooperative REPL Servers](06_16_reading_and_evaluating_scheme_code.md#61615-cooperative-repl-servers), Previous: [Sandboxed Evaluation](06_16_reading_and_evaluating_scheme_code.md#61613-sandboxed-evaluation), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.14 REPL Servers [¶](06_16_reading_and_evaluating_scheme_code.md#61614-repl-servers)

The procedures in this section are provided by

([use-modules](06_18_modules.md) ([system](07_02_07_processes.md) repl server))

When an application is written in Guile, it is often convenient to allow the user to be able to interact with it by evaluating Scheme expressions in a REPL.

The procedures of this module allow you to spawn a _REPL server_, which permits interaction over a local or TCP connection. Guile itself uses them internally to implement the \--listen switch, [Command-line Options](04_programming_in_scheme.md#421-command-line-options).

Scheme Procedure: **make-tcp-server-socket** \[#:host=#f\] \[#:addr\] \[#:port=37146\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Return a stream socket bound to a given address addr and port number port. If the host is given, and addr is not, then the host string is converted to an address. If neither is given, we use the loopback address.

Scheme Procedure: **make-unix-domain-server-socket** \[#:path="/tmp/guile-socket"\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Return a UNIX domain socket, bound to a given path.

Scheme Procedure: **run-server** \[server-socket\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Scheme Procedure: **spawn-server** \[server-socket\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Create and run a REPL, making it available over the given server-socket. If server-socket is not provided, it defaults to the socket created by calling `make-tcp-server-socket` with no arguments.

`run-server` runs the server in the current thread, whereas `spawn-server` runs the server in a new thread.

Scheme Procedure: **stop-server-and-clients!** [¶](06_16_reading_and_evaluating_scheme_code.md)

Closes the connection on all running server sockets.

Please note that in the current implementation, the REPL threads are canceled without unwinding their stacks. If any of them are holding mutexes or are within a critical section, the results are unspecified.

* * *

Previous: [REPL Servers](06_16_reading_and_evaluating_scheme_code.md#61614-repl-servers), Up: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.16.15 Cooperative REPL Servers [¶](06_16_reading_and_evaluating_scheme_code.md#61615-cooperative-repl-servers)

The procedures in this section are provided by

([use-modules](06_18_modules.md) ([system](07_02_07_processes.md) repl coop-server))

Whereas ordinary REPL servers run in their own threads (see [REPL Servers](06_16_reading_and_evaluating_scheme_code.md#61614-repl-servers)), sometimes it is more convenient to provide REPLs that run at specified times within an existing thread, for example in programs utilizing an event loop or in single-threaded programs. This allows for safe access and mutation of a program’s data structures from the REPL, without concern for thread synchronization.

Although the REPLs are run in the thread that calls `spawn-coop-repl-server` and `poll-coop-repl-server`, dedicated threads are spawned so that the calling thread is not blocked. The spawned threads read input for the REPLs and to listen for new connections.

Cooperative REPL servers must be polled periodically to evaluate any pending expressions by calling `poll-coop-repl-server` with the object returned from `spawn-coop-repl-server`. The thread that calls `poll-coop-repl-server` will be blocked for as long as the expression takes to be evaluated or if the debugger is entered.

Scheme Procedure: **spawn-coop-repl-server** \[server-socket\] [¶](06_16_reading_and_evaluating_scheme_code.md)

Create and return a new cooperative REPL server object, and spawn a new thread to listen for connections on server-socket. Proper functioning of the REPL server requires that `poll-coop-repl-server` be called periodically on the returned server object.

Scheme Procedure: **poll-coop-repl-server** coop-server [¶](06_16_reading_and_evaluating_scheme_code.md)

Poll the cooperative REPL server coop-server and apply a pending operation if there is one, such as evaluating an expression typed at the REPL prompt. This procedure must be called from the same thread that called `spawn-coop-repl-server`.

* * *

Next: [Modules](06_18_modules.md#618-modules), Previous: [Reading and Evaluating Scheme Code](06_16_reading_and_evaluating_scheme_code.md#616-reading-and-evaluating-scheme-code), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
