### 6.13 Regular Expressions [¶](06_13_regular_expressions.md#613-regular-expressions)

A _regular expression_ (or _regexp_) is a pattern that describes a whole class of strings. A full description of regular expressions and their syntax is beyond the scope of this manual.

If your system does not include a POSIX regular expression library, and you have not linked Guile with a third-party regexp library such as Rx, these functions will not be available. You can tell whether your Guile installation includes regular expression support by checking whether `(provided? 'regex)` returns true.

The following regexp and string matching features are provided by the `(ice-9 regex)` module. Before using the described functions, you should load this module by executing `(use-modules (ice-9 regex))`.

*   [Regexp Functions](06_13_regular_expressions.md#6131-regexp-functions)
*   [Match Structures](06_13_regular_expressions.md#6132-match-structures)
*   [Backslash Escapes](06_13_regular_expressions.md#6133-backslash-escapes)

* * *

Next: [Match Structures](06_13_regular_expressions.md#6132-match-structures), Up: [Regular Expressions](06_13_regular_expressions.md#613-regular-expressions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.13.1 Regexp Functions [¶](06_13_regular_expressions.md#6131-regexp-functions)

By default, Guile supports POSIX extended regular expressions. That means that the characters ‘(’, ‘)’, ‘+’ and ‘?’ are special, and must be escaped if you wish to match the literal characters and there is no support for “non-greedy” variants of ‘\*’, ‘+’ or ‘?’.

This regular expression interface was modeled after that implemented by SCSH, the Scheme Shell. It is intended to be upwardly compatible with SCSH regular expressions.

Zero bytes (`#\nul`) cannot be used in regex patterns or input strings, since the underlying C functions treat that as the end of string. If there’s a zero byte an error is thrown.

Internally, patterns and input strings are converted to the current locale’s encoding, and then passed to the C library’s regular expression routines (see [Regular Expressions](https://doc.guix.gnu.org/libc/latest/en/libc.html#Regular-Expressions) in The GNU C Library Reference Manual). The returned match structures always point to characters in the strings, not to individual bytes, even in the case of multi-byte encodings. This ensures that the match structures are correct when performing matching with characters that have a multi-byte representation in the locale encoding. Note, however, that using characters which cannot be represented in the locale encoding can lead to surprising results.

Scheme Procedure: **string-match** pattern str \[start\] [¶](06_13_regular_expressions.md)

Compile the string pattern into a regular expression and compare it with str. The optional numeric argument start specifies the position of str at which to begin matching.

`string-match` returns a _match structure_ which describes what, if anything, was matched by the regular expression. See [Match Structures](06_13_regular_expressions.md#6132-match-structures). If str does not match pattern at all, `string-match` returns `#f`.

Two examples of a match follow. In the first example, the pattern matches the four digits in the match string. In the second, the pattern matches nothing.

(string-match "\[0-9\]\[0-9\]\[0-9\]\[0-9\]" "blah2002")
⇒ #("blah2002" (4 . 8))

(string-match "\[A-Za-z\]" "123456")
⇒ #f

Each time `string-match` is called, it must compile its pattern argument into a regular expression structure. This operation is expensive, which makes `string-match` inefficient if the same regular expression is used several times (for example, in a loop). For better performance, you can compile a regular expression in advance and then match strings against the compiled regexp.

Scheme Procedure: **make-regexp** pat flag… [¶](06_13_regular_expressions.md)

C Function: **scm\_make\_regexp** (pat, flaglst) [¶](06_13_regular_expressions.md)

Compile the regular expression described by pat, and return the compiled regexp structure. If pat does not describe a legal regular expression, `make-regexp` throws a `regular-expression-syntax` error.

The flag arguments change the behavior of the compiled regular expression. The following values may be supplied:

Variable: **regexp/icase** [¶](06_13_regular_expressions.md)

Consider uppercase and lowercase letters to be the same when matching.

Variable: **regexp/newline** [¶](06_13_regular_expressions.md)

If a newline appears in the target string, then permit the ‘^’ and ‘$’ operators to match immediately after or immediately before the newline, respectively. Also, the ‘.’ and ‘\[^...\]’ operators will never match a newline character. The intent of this flag is to treat the target string as a buffer containing many lines of text, and the regular expression as a pattern that may match a single one of those lines.

Variable: **regexp/basic** [¶](06_13_regular_expressions.md)

Compile a basic (“obsolete”) regexp instead of the extended (“modern”) regexps that are the default. Basic regexps do not consider ‘|’, ‘+’ or ‘?’ to be special characters, and require the ‘{...}’ and ‘(...)’ metacharacters to be backslash-escaped (see [Backslash Escapes](06_13_regular_expressions.md#6133-backslash-escapes)). There are several other differences between basic and extended regular expressions, but these are the most significant.

Variable: **regexp/extended** [¶](06_13_regular_expressions.md)

Compile an extended regular expression rather than a basic regexp. This is the default behavior; this flag will not usually be needed. If a call to `make-regexp` includes both `regexp/basic` and `regexp/extended` flags, the one which comes last will override the earlier one.

Scheme Procedure: **regexp-exec** rx str \[start \[flags\]\] [¶](06_13_regular_expressions.md)

C Function: **scm\_regexp\_exec** (rx, str, start, flags) [¶](06_13_regular_expressions.md)

Match the compiled regular expression rx against `str`. If the optional integer start argument is provided, begin matching from that position in the string. Return a match structure describing the results of the match, or `#f` if no match could be found.

The flags argument changes the matching behavior. The following flag values may be supplied, use `logior` (see [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations)) to combine them,

Variable: **regexp/notbol** [¶](06_13_regular_expressions.md)

Consider that the start offset into str is not the beginning of a line and should not match operator ‘^’.

If rx was created with the `regexp/newline` option above, ‘^’ will still match after a newline in str.

Variable: **regexp/noteol** [¶](06_13_regular_expressions.md)

Consider that the end of str is not the end of a line and should not match operator ‘$’.

If rx was created with the `regexp/newline` option above, ‘$’ will still match before a newline in str.

;; Regexp to match uppercase letters
(define r ([make-regexp](06_13_regular_expressions.md) "\[A-Z\]\*"))

;; Regexp to match letters, ignoring case
(define ri ([make-regexp](06_13_regular_expressions.md) "\[A-Z\]\*" [regexp/icase](06_13_regular_expressions.md)))

;; Search for bob using regexp r
([match:substring](06_13_regular_expressions.md) ([regexp-exec](06_13_regular_expressions.md) r "bob"))
⇒ ""                  ; no match
;; Search for bob using regexp ri
([match:substring](06_13_regular_expressions.md) ([regexp-exec](06_13_regular_expressions.md) ri "Bob"))
⇒ "Bob"               ; matched case insensitive

Scheme Procedure: **regexp?** obj [¶](06_13_regular_expressions.md)

C Function: **scm\_regexp\_p** (obj) [¶](06_13_regular_expressions.md)

Return `#t` if obj is a compiled regular expression, or `#f` otherwise.

  

Scheme Procedure: **list-matches** regexp str \[flags\] [¶](06_13_regular_expressions.md)

Return a list of match structures which are the non-overlapping matches of regexp in str. regexp can be either a pattern string or a compiled regexp. The flags argument is as per `regexp-exec` above.

(map match:substring (list-matches "\[a-z\]+" "abc 42 def 78"))
⇒ ("abc" "def")

Scheme Procedure: **fold-matches** regexp str init proc \[flags\] [¶](06_13_regular_expressions.md)

Apply proc to the non-overlapping matches of regexp in str, to build a result. regexp can be either a pattern string or a compiled regexp. The flags argument is as per `regexp-exec` above.

proc is called as `(proc match prev)` where match is a match structure and prev is the previous return from proc. For the first call prev is the given init parameter. `fold-matches` returns the final value from proc.

For example to count matches,

(fold-matches "\[a-z\]\[0-9\]" "abc x1 def y2" 0
              (lambda (match count)
                (1+ count)))
⇒ 2

  

Regular expressions are commonly used to find patterns in one string and replace them with the contents of another string. The following functions are convenient ways to do this.

Scheme Procedure: **regexp-substitute** port match item … [¶](06_13_regular_expressions.md)

Write to port selected parts of the match structure match. Or if port is `#f` then form a string from those parts and return that.

Each item specifies a part to be written, and may be one of the following,

*   A string. String arguments are written out verbatim.
*   An integer. The submatch with that number is written (`match:substring`). Zero is the entire match.
*   The symbol ‘pre’. The portion of the matched string preceding the regexp match is written (`match:prefix`).
*   The symbol ‘post’. The portion of the matched string following the regexp match is written (`match:suffix`).

For example, changing a match and retaining the text before and after,

(regexp-substitute #f (string-match "\[0-9\]+" "number 25 is good")
                   'pre "37" 'post)
⇒ "number 37 is good"

Or matching a YYYYMMDD format date such as ‘20020828’ and re-ordering and hyphenating the fields.

(define date-regex
   "(\[0-9\]\[0-9\]\[0-9\]\[0-9\])(\[0-9\]\[0-9\])(\[0-9\]\[0-9\])")
(define s "Date 20020429 12am.")
([regexp-substitute](06_13_regular_expressions.md) #f ([string-match](06_13_regular_expressions.md) date-regex s)
                   'pre 2 "-" 3 "-" 1 'post " (" 0 ")")
⇒ "Date 04-29-2002 12am. (20020429)"

Scheme Procedure: **regexp-substitute/global** port regexp target item… [¶](06_13_regular_expressions.md)

Write to port selected parts of matches of regexp in target. If port is `#f` then form a string from those parts and return that. regexp can be a string or a compiled regex.

This is similar to `regexp-substitute`, but allows global substitutions on target. Each item behaves as per `regexp-substitute`, with the following differences,

*   A function. Called as `(item match)` with the match structure for the regexp match, it should return a string to be written to port.
*   The symbol ‘post’. This doesn’t output anything, but instead causes `regexp-substitute/global` to recurse on the unmatched portion of target.
    
    This _must_ be supplied to perform a global search and replace on target; without it `regexp-substitute/global` returns after a single match and output.
    

For example, to collapse runs of tabs and spaces to a single hyphen each,

(regexp-substitute/global #f "\[ \\t\]+"  "this   is   the text"
                          'pre "-" 'post)
⇒ "this-is-the-text"

Or using a function to reverse the letters in each word,

(regexp-substitute/global #f "\[a-z\]+"  "to do and not-do"
  'pre (lambda (m) (string-reverse (match:substring m))) 'post)
⇒ "ot od dna ton-od"

Without the `post` symbol, just one regexp match is made. For example the following is the date example from `regexp-substitute` above, without the need for the separate `string-match` call.

(define date-regex
   "(\[0-9\]\[0-9\]\[0-9\]\[0-9\])(\[0-9\]\[0-9\])(\[0-9\]\[0-9\])")
(define s "Date 20020429 12am.")
([regexp-substitute/global](06_13_regular_expressions.md) #f date-regex s
                          'pre 2 "-" 3 "-" 1 'post " (" 0 ")")

⇒ "Date 04-29-2002 12am. (20020429)"

* * *

Next: [Backslash Escapes](06_13_regular_expressions.md#6133-backslash-escapes), Previous: [Regexp Functions](06_13_regular_expressions.md#6131-regexp-functions), Up: [Regular Expressions](06_13_regular_expressions.md#613-regular-expressions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.13.2 Match Structures [¶](06_13_regular_expressions.md#6132-match-structures)

A _match structure_ is the object returned by `string-match` and `regexp-exec`. It describes which portion of a string, if any, matched the given regular expression. Match structures include: a reference to the string that was checked for matches; the starting and ending positions of the regexp match; and, if the regexp included any parenthesized subexpressions, the starting and ending positions of each submatch.

In each of the regexp match functions described below, the `match` argument must be a match structure returned by a previous call to `string-match` or `regexp-exec`. Most of these functions return some information about the original target string that was matched against a regular expression; we will call that string target for easy reference.

Scheme Procedure: **regexp-match?** obj [¶](06_13_regular_expressions.md)

Return `#t` if obj is a match structure returned by a previous call to `regexp-exec`, or `#f` otherwise.

Scheme Procedure: **match:substring** match \[n\] [¶](06_13_regular_expressions.md)

Return the portion of target matched by subexpression number n. Submatch 0 (the default) represents the entire regexp match. If the regular expression as a whole matched, but the subexpression number n did not match, return `#f`.

(define s ([string-match](06_13_regular_expressions.md) "\[0-9\]\[0-9\]\[0-9\]\[0-9\]" "blah2002foo"))
([match:substring](06_13_regular_expressions.md) s)
⇒ "2002"

;; match starting at offset 6 in the string
([match:substring](06_13_regular_expressions.md)
  ([string-match](06_13_regular_expressions.md) "\[0-9\]\[0-9\]\[0-9\]\[0-9\]" "blah987654" 6))
⇒ "7654"

Scheme Procedure: **match:start** match \[n\] [¶](06_13_regular_expressions.md)

Return the starting position of submatch number n.

In the following example, the result is 4, since the match starts at character index 4:

(define s ([string-match](06_13_regular_expressions.md) "\[0-9\]\[0-9\]\[0-9\]\[0-9\]" "blah2002foo"))
([match:start](06_13_regular_expressions.md) s)
⇒ 4

Scheme Procedure: **match:end** match \[n\] [¶](06_13_regular_expressions.md)

Return the ending position of submatch number n.

In the following example, the result is 8, since the match runs between characters 4 and 8 (i.e. the “2002”).

(define s ([string-match](06_13_regular_expressions.md) "\[0-9\]\[0-9\]\[0-9\]\[0-9\]" "blah2002foo"))
([match:end](06_13_regular_expressions.md) s)
⇒ 8

Scheme Procedure: **match:prefix** match [¶](06_13_regular_expressions.md)

Return the unmatched portion of target preceding the regexp match.

(define s ([string-match](06_13_regular_expressions.md) "\[0-9\]\[0-9\]\[0-9\]\[0-9\]" "blah2002foo"))
([match:prefix](06_13_regular_expressions.md) s)
⇒ "blah"

Scheme Procedure: **match:suffix** match [¶](06_13_regular_expressions.md)

Return the unmatched portion of target following the regexp match.

(define s ([string-match](06_13_regular_expressions.md) "\[0-9\]\[0-9\]\[0-9\]\[0-9\]" "blah2002foo"))
([match:suffix](06_13_regular_expressions.md) s)
⇒ "foo"

Scheme Procedure: **match:count** match [¶](06_13_regular_expressions.md)

Return the number of parenthesized subexpressions from match. Note that the entire regular expression match itself counts as a subexpression, and failed submatches are included in the count.

Scheme Procedure: **match:string** match [¶](06_13_regular_expressions.md)

Return the original target string.

(define s ([string-match](06_13_regular_expressions.md) "\[0-9\]\[0-9\]\[0-9\]\[0-9\]" "blah2002foo"))
([match:string](06_13_regular_expressions.md) s)
⇒ "blah2002foo"

* * *

Previous: [Match Structures](06_13_regular_expressions.md#6132-match-structures), Up: [Regular Expressions](06_13_regular_expressions.md#613-regular-expressions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.13.3 Backslash Escapes [¶](06_13_regular_expressions.md#6133-backslash-escapes)

Sometimes you will want a regexp to match characters like ‘\*’ or ‘$’ exactly. For example, to check whether a particular string represents a menu entry from an Info node, it would be useful to match it against a regexp like ‘^\* \[^:\]\*::’. However, this won’t work; because the asterisk is a metacharacter, it won’t match the ‘\*’ at the beginning of the string. In this case, we want to make the first asterisk un-magic.

You can do this by preceding the metacharacter with a backslash character ‘\\’. (This is also called _quoting_ the metacharacter, and is known as a _backslash escape_.) When Guile sees a backslash in a regular expression, it considers the following glyph to be an ordinary character, no matter what special meaning it would ordinarily have. Therefore, we can make the above example work by changing the regexp to ‘^\\\* \[^:\]\*::’. The ‘\\\*’ sequence tells the regular expression engine to match only a single asterisk in the target string.

Since the backslash is itself a metacharacter, you may force a regexp to match a backslash in the target string by preceding the backslash with itself. For example, to find variable references in a TeX program, you might want to find occurrences of the string ‘\\let\\’ followed by any number of alphabetic characters. The regular expression ‘\\\\let\\\\\[A-Za-z\]\*’ would do this: the double backslashes in the regexp each match a single backslash in the target string.

Scheme Procedure: **regexp-quote** str [¶](06_13_regular_expressions.md)

Quote each special character found in str with a backslash, and return the resulting string.

**Very important:** Using backslash escapes in Guile source code (as in Emacs Lisp or C) can be tricky, because the backslash character has special meaning for the Guile reader. For example, if Guile encounters the character sequence ‘\\n’ in the middle of a string while processing Scheme code, it replaces those characters with a newline character. Similarly, the character sequence ‘\\t’ is replaced by a horizontal tab. Several of these _escape sequences_ are processed by the Guile reader before your code is executed. Unrecognized escape sequences are ignored: if the characters ‘\\\*’ appear in a string, they will be translated to the single character ‘\*’.

This translation is obviously undesirable for regular expressions, since we want to be able to include backslashes in a string in order to escape regexp metacharacters. Therefore, to make sure that a backslash is preserved in a string in your Guile program, you must use _two_ consecutive backslashes:

(define Info-menu-entry-pattern ([make-regexp](06_13_regular_expressions.md) "^\\\\\* \[^:\]\*"))

The string in this example is preprocessed by the Guile reader before any code is executed. The resulting argument to `make-regexp` is the string ‘^\\\* \[^:\]\*’, which is what we really want.

This also means that in order to write a regular expression that matches a single backslash character, the regular expression string in the source code must include _four_ backslashes. Each consecutive pair of backslashes gets translated by the Guile reader to a single backslash, and the resulting double-backslash is interpreted by the regexp engine as matching a single backslash character. Hence:

(define tex-variable-pattern ([make-regexp](06_13_regular_expressions.md) "\\\\\\\\let\\\\\\\\=\[A-Za-z\]\*"))

The reason for the unwieldiness of this syntax is historical. Both regular expression pattern matchers and Unix string processing systems have traditionally used backslashes with the special meanings described above. The POSIX regular expression specification and ANSI C standard both require these semantics. Attempting to abandon either convention would cause other kinds of compatibility problems, possibly more severe ones. Therefore, without extending the Scheme reader to support strings with different quoting conventions (an ungainly and confusing extension when implemented in other languages), we must adhere to this cumbersome escape syntax.

* * *

Next: [PEG Parsing](06_15_peg_parsing.md#615-peg-parsing), Previous: [Regular Expressions](06_13_regular_expressions.md#613-regular-expressions), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
