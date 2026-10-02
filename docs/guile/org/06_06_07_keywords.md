#### 6.6.7 Keywords [¶](06_06_07_keywords.md#667-keywords)

Keywords are self-evaluating objects with a convenient read syntax that makes them easy to type.

Guile’s keyword support conforms to R5RS, and adds a (switchable) read syntax extension to permit keywords to begin with `:` as well as `#:`, or to end with `:`.

*   [Why Use Keywords?](06_06_07_keywords.md#6671-why-use-keywords)
*   [Coding With Keywords](06_06_07_keywords.md#6672-coding-with-keywords)
*   [Keyword Read Syntax](06_06_07_keywords.md#6673-keyword-read-syntax)
*   [Keyword Procedures](06_06_07_keywords.md#6674-keyword-procedures)

* * *

Next: [Coding With Keywords](06_06_07_keywords.md#6672-coding-with-keywords), Up: [Keywords](06_06_07_keywords.md#667-keywords)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.7.1 Why Use Keywords? [¶](06_06_07_keywords.md#6671-why-use-keywords)

Keywords are useful in contexts where a program or procedure wants to be able to accept a large number of optional arguments without making its interface unmanageable.

To illustrate this, consider a hypothetical `make-window` procedure, which creates a new window on the screen for drawing into using some graphical toolkit. There are many parameters that the caller might like to specify, but which could also be sensibly defaulted, for example:

*   color depth – Default: the color depth for the screen
*   background color – Default: white
*   width – Default: 600
*   height – Default: 400

If `make-window` did not use keywords, the caller would have to pass in a value for each possible argument, remembering the correct argument order and using a special value to indicate the default value for that argument:

(make-window 'default              ;; Color depth
             'default              ;; Background color
             800                   ;; Width
             100                   ;; Height
             [...](06_08_macros.md))                  ;; More make-window arguments

With keywords, on the other hand, defaulted arguments are omitted, and non-default arguments are clearly tagged by the appropriate keyword. As a result, the invocation becomes much clearer:

(make-window #:width 800 #:height 100)

On the other hand, for a simpler procedure with few arguments, the use of keywords would be a hindrance rather than a help. The primitive procedure `cons`, for example, would not be improved if it had to be invoked as

([cons](06_06_08_pairs.md) #:car x #:cdr y)

So the decision whether to use keywords or not is purely pragmatic: use them if they will clarify the procedure invocation at point of call.

* * *

Next: [Keyword Read Syntax](06_06_07_keywords.md#6673-keyword-read-syntax), Previous: [Why Use Keywords?](06_06_07_keywords.md#6671-why-use-keywords), Up: [Keywords](06_06_07_keywords.md#667-keywords)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.7.2 Coding With Keywords [¶](06_06_07_keywords.md#6672-coding-with-keywords)

If a procedure wants to support keywords, it should take a rest argument and then use whatever means is convenient to extract keywords and their corresponding arguments from the contents of that rest argument.

The following example illustrates the principle: the code for `make-window` uses a helper procedure called `get-keyword-value` to extract individual keyword arguments from the rest argument.

(define (get-keyword-value args keyword default)
  (let ((kv ([memq](06_06_09_lists.md) keyword args)))
    (if (and kv ([\>=](06_06_02_numerical_data_types.md) ([length](06_06_09_lists.md) kv) 2))
        ([cadr](06_06_08_pairs.md) kv)
        default)))

(define (make-window . args)
  (let ((depth  (get-keyword-value args #:depth  screen-depth))
        (bg     (get-keyword-value args #:bg     "white"))
        ([width](04_programming_in_scheme.md)  (get-keyword-value args #:width  800))
        (height (get-keyword-value args #:height 100))
        [...](06_08_macros.md))
    [...](06_08_macros.md)))

But you don’t need to write `get-keyword-value`. The `(ice-9 optargs)` module provides a set of powerful macros that you can use to implement keyword-supporting procedures like this:

([use-modules](06_18_modules.md) (ice-9 optargs))

(define (make-window . args)
  ([let-keywords](06_07_procedures.md) args #f ((depth  screen-depth)
                         (bg     "white")
                         ([width](04_programming_in_scheme.md)  800)
                         (height 100))
    [...](06_08_macros.md)))

Or, even more economically, like this:

([use-modules](06_18_modules.md) (ice-9 optargs))

(define\* (make-window #:key (depth  screen-depth)
                            (bg     "white")
                            ([width](04_programming_in_scheme.md)  800)
                            (height 100))
  [...](06_08_macros.md))

For further details on `let-keywords`, `define*` and other facilities provided by the `(ice-9 optargs)` module, see [Optional Arguments](06_07_procedures.md#674-optional-arguments).

To handle keyword arguments from procedures implemented in C, use `scm_c_bind_keyword_arguments` (see [Keyword Procedures](06_06_07_keywords.md#6674-keyword-procedures)).

* * *

Next: [Keyword Procedures](06_06_07_keywords.md#6674-keyword-procedures), Previous: [Coding With Keywords](06_06_07_keywords.md#6672-coding-with-keywords), Up: [Keywords](06_06_07_keywords.md#667-keywords)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.7.3 Keyword Read Syntax [¶](06_06_07_keywords.md#6673-keyword-read-syntax)

Guile, by default, only recognizes a keyword syntax that is compatible with R5RS. A token of the form `#:NAME`, where `NAME` has the same syntax as a Scheme symbol (see [Extended Read Syntax for Symbols](06_06_06_symbols.md#6665-extended-read-syntax-for-symbols)), is the external representation of the keyword named `NAME`. Keyword objects print using this syntax as well, so values containing keyword objects can be read back into Guile. When used in an expression, keywords are self-quoting objects.

If the `keywords` read option is set to `'prefix`, Guile also recognizes the alternative read syntax `:NAME`. Otherwise, tokens of the form `:NAME` are read as symbols, as required by R5RS.

If the `keywords` read option is set to `'postfix`, Guile recognizes the SRFI-88 read syntax `NAME:` (see [SRFI-88 Keyword Objects](07_05_42_srfi88_keyword_objects.md#7542-srfi-88-keyword-objects)). Otherwise, tokens of this form are read as symbols.

To enable and disable the alternative non-R5RS keyword syntax, you use the `read-set!` procedure documented [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code). Note that the `prefix` and `postfix` syntax are mutually exclusive.

([read-set!](06_16_reading_and_evaluating_scheme_code.md) keywords 'prefix)

#:type
⇒
#:type

:type
⇒
#:type

([read-set!](06_16_reading_and_evaluating_scheme_code.md) keywords 'postfix)

type:
⇒
#:type

:type
⇒
:type

([read-set!](06_16_reading_and_evaluating_scheme_code.md) keywords #f)

#:type
⇒
#:type

:type
⊣
ERROR: In expression :type:
ERROR: Unbound variable: :type
ABORT: (unbound-variable)

* * *

Previous: [Keyword Read Syntax](06_06_07_keywords.md#6673-keyword-read-syntax), Up: [Keywords](06_06_07_keywords.md#667-keywords)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.7.4 Keyword Procedures [¶](06_06_07_keywords.md#6674-keyword-procedures)

Scheme Procedure: **keyword?** obj [¶](06_06_07_keywords.md)

C Function: **scm\_keyword\_p** (obj) [¶](06_06_07_keywords.md)

Return `#t` if the argument obj is a keyword, else `#f`.

Scheme Procedure: **keyword->symbol** keyword [¶](06_06_07_keywords.md)

C Function: **scm\_keyword\_to\_symbol** (keyword) [¶](06_06_07_keywords.md)

Return the symbol with the same name as keyword.

Scheme Procedure: **symbol->keyword** symbol [¶](06_06_07_keywords.md)

C Function: **scm\_symbol\_to\_keyword** (symbol) [¶](06_06_07_keywords.md)

Return the keyword with the same name as symbol.

C Function: `int` **scm\_is\_keyword** `(SCM obj)` [¶](06_06_07_keywords.md)

Equivalent to `scm_is_true (scm_keyword_p (obj))`.

C Function: `SCM` **scm\_from\_locale\_keyword** `(const char *name)` [¶](06_06_07_keywords.md)

C Function: `SCM` **scm\_from\_locale\_keywordn** `(const char *name, size_t len)` [¶](06_06_07_keywords.md)

Equivalent to `scm_symbol_to_keyword (scm_from_locale_symbol (name))` and `scm_symbol_to_keyword (scm_from_locale_symboln (name, len))`, respectively.

Note that these functions should _not_ be used when name is a C string constant, because there is no guarantee that the current locale will match that of the execution character set, used for string and character constants. Most modern C compilers use UTF-8 by default, so in such cases we recommend `scm_from_utf8_keyword`.

C Function: `SCM` **scm\_from\_latin1\_keyword** `(const char *name)` [¶](06_06_07_keywords.md)

C Function: `SCM` **scm\_from\_utf8\_keyword** `(const char *name)` [¶](06_06_07_keywords.md)

Equivalent to `scm_symbol_to_keyword (scm_from_latin1_symbol (name))` and `scm_symbol_to_keyword (scm_from_utf8_symbol (name))`, respectively.

C Function: `void` **scm\_c\_bind\_keyword\_arguments** ``(const char *subr, SCM rest, scm_t_keyword_arguments_flags flags, SCM keyword1, SCM *argp1, …, SCM keywordN, SCM *argpN, `SCM_UNDEFINED`)`` [¶](06_06_07_keywords.md)

Extract the specified keyword arguments from rest, which is not modified. If the keyword argument keyword1 is present in rest with an associated value, that value is stored in the variable pointed to by argp1, otherwise the variable is left unchanged. Similarly for the other keywords and argument pointers up to keywordN and argpN. The argument list to `scm_c_bind_keyword_arguments` must be terminated by `SCM_UNDEFINED`.

Note that since the variables pointed to by argp1 through argpN are left unchanged if the associated keyword argument is not present, they should be initialized to their default values before calling `scm_c_bind_keyword_arguments`. Alternatively, you can initialize them to `SCM_UNDEFINED` before the call, and then use `SCM_UNBNDP` after the call to see which ones were provided.

If an unrecognized keyword argument is present in rest and flags does not contain `SCM_ALLOW_OTHER_KEYS`, or if non-keyword arguments are present and flags does not contain `SCM_ALLOW_NON_KEYWORD_ARGUMENTS`, an exception is raised. subr should be the name of the procedure receiving the keyword arguments, for purposes of error reporting.

For example:

SCM k\_delimiter;
SCM k\_grammar;
SCM sym\_infix;

SCM my\_string\_join (SCM strings, SCM rest)
{
  SCM delimiter = SCM\_UNDEFINED;
  SCM grammar   = sym\_infix;

  scm\_c\_bind\_keyword\_arguments ("my-string-join", rest, 0,
                                k\_delimiter, &delimiter,
                                k\_grammar, &grammar,
                                SCM\_UNDEFINED);

  if (SCM\_UNBNDP (delimiter))
    delimiter = scm\_from\_utf8\_string (" ");

  return scm\_string\_join (strings, delimiter, grammar);
}

void my\_init ()
{
  k\_delimiter = scm\_from\_utf8\_keyword ("delimiter");
  k\_grammar   = scm\_from\_utf8\_keyword ("grammar");
  sym\_infix   = scm\_from\_utf8\_symbol  ("infix");
  scm\_c\_define\_gsubr ("my-string-join", 1, 0, 1, my\_string\_join);
}

* * *

Next: [Lists](06_06_09_lists.md#669-lists), Previous: [Keywords](06_06_07_keywords.md#667-keywords), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
