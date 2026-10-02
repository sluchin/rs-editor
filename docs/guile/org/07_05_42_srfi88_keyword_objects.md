#### 7.5.42 SRFI-88 Keyword Objects [¶](07_05_42_srfi88_keyword_objects.md#7542-srfi-88-keyword-objects)

[SRFI-88](http://srfi.schemers.org/srfi-88/srfi-88.html) provides _keyword objects_, which are equivalent to Guile’s keywords (see [Keywords](06_06_07_keywords.md#667-keywords)). SRFI-88 keywords can be entered using the _postfix keyword syntax_, which consists of an identifier followed by `:` (see [`postfix` keyword syntax](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code)). SRFI-88 can be made available with:

(use-modules (srfi srfi-88))

Doing so installs the right reader option for keyword syntax, using `(read-set! keywords 'postfix)`. It also provides the procedures described below.

Scheme Procedure: **keyword?** obj [¶](07_05_42_srfi88_keyword_objects.md)

Return `#t` if obj is a keyword. This is the same procedure as the same-named built-in procedure (see [`keyword?`](06_06_07_keywords.md#6674-keyword-procedures)).

(keyword? foo:)         ⇒ #t
(keyword? 'foo:)        ⇒ #t
(keyword? "foo")        ⇒ #f

Scheme Procedure: **keyword->string** kw [¶](07_05_42_srfi88_keyword_objects.md)

Return the name of kw as a string, i.e., without the trailing colon. The returned string may not be modified, e.g., with `string-set!`.

(keyword->string foo:)  ⇒ "foo"

Scheme Procedure: **string->keyword** str [¶](07_05_42_srfi88_keyword_objects.md)

Return the keyword object whose name is str.

(keyword->string (string->keyword "a b c"))     ⇒ "a b c"

* * *

Next: [SRFI-105 Curly-infix expressions.](07_05_44_srfi105_curlyinfix_expressions.md#7544-srfi-105-curly-infix-expressions), Previous: [SRFI-88 Keyword Objects](07_05_42_srfi88_keyword_objects.md#7542-srfi-88-keyword-objects), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
