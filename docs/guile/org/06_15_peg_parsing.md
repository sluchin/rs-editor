### 6.15 PEG Parsing [¶](06_15_peg_parsing.md#615-peg-parsing)

Parsing Expression Grammars (PEGs) are a way of specifying formal languages for text processing. They can be used either for matching (like regular expressions) or for building recursive descent parsers (like lex/yacc). Guile uses a superset of PEG syntax that allows more control over what information is preserved during parsing.

Wikipedia has a clear and concise introduction to PEGs if you want to familiarize yourself with the syntax: [http://en.wikipedia.org/wiki/Parsing\_expression\_grammar](http://en.wikipedia.org/wiki/Parsing_expression_grammar).

The paper that introduced PEG contains a more detailed description of how PEG works, and describes its syntax in detail: [https://bford.info/pub/lang/peg.pdf](https://bford.info/pub/lang/peg.pdf)

The `(ice-9 peg)` module works by compiling PEGs down to lambda expressions. These can either be stored in variables at compile-time by the define macros (`define-peg-pattern` and `define-peg-string-patterns`) or calculated explicitly at runtime with the compile functions (`compile-peg-pattern` and `peg-string-compile`).

They can then be used for either parsing (`match-pattern`) or searching (`search-for-pattern`). For convenience, `search-for-pattern` also takes pattern literals in case you want to inline a simple search (people often use regular expressions this way).

The rest of this documentation consists of a syntax reference, an API reference, and a tutorial.

*   [PEG Syntax Reference](06_15_peg_parsing.md#6151-peg-syntax-reference)
*   [PEG API Reference](06_15_peg_parsing.md#6152-peg-api-reference)
*   [PEG Tutorial](06_15_peg_parsing.md#6153-peg-tutorial)
*   [PEG Internals](06_15_peg_parsing.md#6154-peg-internals)

* * *

Next: [PEG API Reference](06_15_peg_parsing.md#6152-peg-api-reference), Up: [PEG Parsing](06_15_peg_parsing.md#615-peg-parsing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.15.1 PEG Syntax Reference [¶](06_15_peg_parsing.md#6151-peg-syntax-reference)

#### Normal PEG Syntax: [¶](06_15_peg_parsing.md#normal-peg-syntax)

PEG Pattern: **sequence** a b [¶](06_15_peg_parsing.md)

Parses a. If this succeeds, continues to parse b from the end of the text parsed as a. Succeeds if both a and b succeed.

`"a b"`

`(and a b)`

PEG Pattern: **ordered choice** a b [¶](06_15_peg_parsing.md)

Parses a. If this fails, backtracks and parses b. Succeeds if either a or b succeeds.

`"a/b"`

`(or a b)`

PEG Pattern: **zero or more** a [¶](06_15_peg_parsing.md)

Parses a as many times in a row as it can, starting each a at the end of the text parsed by the previous a. Always succeeds.

`"a*"`

`(* a)`

PEG Pattern: **one or more** a [¶](06_15_peg_parsing.md)

Parses a as many times in a row as it can, starting each a at the end of the text parsed by the previous a. Succeeds if at least one a was parsed.

`"a+"`

`(+ a)`

PEG Pattern: **optional** a [¶](06_15_peg_parsing.md)

Tries to parse a. Succeeds if a succeeds.

`"a?"`

`(? a)`

PEG Pattern: **followed by** a [¶](06_15_peg_parsing.md)

Makes sure it is possible to parse a, but does not actually parse it. Succeeds if a would succeed.

`"&a"`

`(followed-by a)`

PEG Pattern: **not followed by** a [¶](06_15_peg_parsing.md)

Makes sure it is impossible to parse a, but does not actually parse it. Succeeds if a would fail.

`"!a"`

`(not-followed-by a)`

PEG Pattern: **string literal** “abc” [¶](06_15_peg_parsing.md)

Parses the string "abc". Succeeds if that parsing succeeds.

`"'abc'"`

`"abc"`

PEG Pattern: **any character** [¶](06_15_peg_parsing.md)

Parses any single character. Succeeds unless there is no more text to be parsed.

`"."`

`peg-any`

PEG Pattern: **character class** a b [¶](06_15_peg_parsing.md)

Alternative syntax for “Ordered Choice a b” if a and b are characters.

`"[ab]"`

`(or "a" "b")`

PEG Pattern: **range of characters** a z [¶](06_15_peg_parsing.md)

Parses any character falling between a and z.

`"[a-z]"`

`(range #\a #\z)`

PEG Pattern: **inverse range of characters** a z [¶](06_15_peg_parsing.md)

Parses any character not falling between a and z.

`"[^a-z]"`

`(not-in-range #\a #\z)`

Example:

"(a !b / c &d\*) 'e'+"

Would be:

(and
 (or
  (and a ([not-followed-by](06_15_peg_parsing.md) b))
  (and c ([followed-by](06_15_peg_parsing.md) ([\*](06_06_02_numerical_data_types.md) d))))
 ([+](06_06_02_numerical_data_types.md) "e"))

#### Extended Syntax [¶](06_15_peg_parsing.md#extended-syntax)

There is some extra syntax for S-expressions.

PEG Pattern: **ignore** a [¶](06_15_peg_parsing.md)

Ignore the text matching a

PEG Pattern: **capture** a [¶](06_15_peg_parsing.md)

Capture the text matching a.

PEG Pattern: **peg** a [¶](06_15_peg_parsing.md)

Embed the PEG pattern a using string syntax.

Example:

"!a / 'b'"

Is equivalent to

(or ([peg](06_15_peg_parsing.md) "!a") "b")

and

(or ([not-followed-by](06_15_peg_parsing.md) a) "b")

* * *

Next: [PEG Tutorial](06_15_peg_parsing.md#6153-peg-tutorial), Previous: [PEG Syntax Reference](06_15_peg_parsing.md#6151-peg-syntax-reference), Up: [PEG Parsing](06_15_peg_parsing.md#615-peg-parsing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.15.2 PEG API Reference [¶](06_15_peg_parsing.md#6152-peg-api-reference)

#### Define Macros [¶](06_15_peg_parsing.md#define-macros)

The most straightforward way to define a PEG is by using one of the define macros (both of these macroexpand into `define` expressions). These macros bind parsing functions to variables. These parsing functions may be invoked by `match-pattern` or `search-for-pattern`, which return a PEG match record. Raw data can be retrieved from this record with the PEG match deconstructor functions. More complicated (and perhaps enlightening) examples can be found in the tutorial.

Scheme Macro: **define-peg-string-patterns** peg-string [¶](06_15_peg_parsing.md)

Defines all the nonterminals in the PEG peg-string. More precisely, `define-peg-string-patterns` takes a superset of PEGs. A normal PEG has a `<-` between the nonterminal and the pattern. `define-peg-string-patterns` uses this symbol to determine what information it should propagate up the parse tree. The normal `<-` propagates the matched text up the parse tree, `<--` propagates the matched text up the parse tree tagged with the name of the nonterminal, and `<` discards that matched text and propagates nothing up the parse tree. Also, nonterminals may include “-” character, while in normal PEG it is not allowed.

For example, if we:

(define-peg-string-patterns 
  "as <- 'a'+
bs <- 'b'+
as-or-bs <- as/bs")
(define-peg-string-patterns 
  "as-tag <-- 'a'+
bs-tag <-- 'b'+
as-or-bs-tag <-- as-tag/bs-tag")

Then:

([match-pattern](06_15_peg_parsing.md) as-or-bs "aabbcc") ⇒
#<peg start: 0 end: 2 string: aabbcc tree: aa>
([match-pattern](06_15_peg_parsing.md) as-or-bs-tag "aabbcc") ⇒
#<peg start: 0 end: 2 string: aabbcc tree: (as-or-bs-tag (as-tag aa))[\>](06_06_02_numerical_data_types.md)

Note that in doing this, we have bound 6 variables at the toplevel (as, bs, as-or-bs, as-tag, bs-tag, and as-or-bs-tag).

Scheme Macro: **define-peg-pattern** name capture-type peg-sexp [¶](06_15_peg_parsing.md)

Defines a single nonterminal name. capture-type determines how much information is passed up the parse tree. peg-sexp is a PEG in S-expression form.

Possible values for capture-type:

`all`

passes the matched text up the parse tree tagged with the name of the nonterminal.

`body`

passes the matched text up the parse tree.

`none`

passes nothing up the parse tree.

For Example, if we:

(define-peg-pattern as body ([+](06_06_02_numerical_data_types.md) "a"))
(define-peg-pattern bs body ([+](06_06_02_numerical_data_types.md) "b"))
(define-peg-pattern as-or-bs body (or as bs))
(define-peg-pattern as-tag all ([+](06_06_02_numerical_data_types.md) "a"))
(define-peg-pattern bs-tag all ([+](06_06_02_numerical_data_types.md) "b"))
(define-peg-pattern as-or-bs-tag all (or as-tag bs-tag))

Then:

([match-pattern](06_15_peg_parsing.md) as-or-bs "aabbcc") ⇒ 
#<peg start: 0 end: 2 string: aabbcc tree: aa>
([match-pattern](06_15_peg_parsing.md) as-or-bs-tag "aabbcc") ⇒ 
#<peg start: 0 end: 2 string: aabbcc tree: (as-or-bs-tag (as-tag aa))[\>](06_06_02_numerical_data_types.md)

Note that in doing this, we have bound 6 variables at the toplevel (as, bs, as-or-bs, as-tag, bs-tag, and as-or-bs-tag).

#### Compile Functions [¶](06_15_peg_parsing.md#compile-functions)

It is sometimes useful to be able to compile anonymous PEG patterns at runtime. These functions let you do that using either syntax.

Scheme Procedure: **peg-string-compile** peg-string capture-type [¶](06_15_peg_parsing.md)

Compiles the PEG pattern in peg-string propagating according to capture-type (capture-type can be any of the values from `define-peg-pattern`).

Scheme Procedure: **compile-peg-pattern** peg-sexp capture-type [¶](06_15_peg_parsing.md)

Compiles the PEG pattern in peg-sexp propagating according to capture-type (capture-type can be any of the values from `define-peg-pattern`).

The functions return syntax objects, which can be useful if you want to use them in macros. If all you want is to define a new nonterminal, you can do the following:

(define [exp](06_06_02_numerical_data_types.md) '([+](06_06_02_numerical_data_types.md) "a"))
(define as ([compile](04_programming_in_scheme.md) ([compile-peg-pattern](06_15_peg_parsing.md) [exp](06_06_02_numerical_data_types.md) 'body)))

You can use this nonterminal with all of the regular PEG functions:

([match-pattern](06_15_peg_parsing.md) as "aaaaa") ⇒
#<peg start: 0 end: 5 string: aaaaa tree: aaaaa>

#### Parsing & Matching Functions [¶](06_15_peg_parsing.md#parsing--matching-functions)

For our purposes, “parsing” means parsing a string into a tree starting from the first character, while “matching” means searching through the string for a substring. In practice, the only difference between the two functions is that `match-pattern` gives up if it can’t find a valid substring starting at index 0 and `search-for-pattern` keeps looking. They are both equally capable of “parsing” and “matching” given those constraints.

Scheme Procedure: **match-pattern** nonterm string [¶](06_15_peg_parsing.md)

Parses string using the PEG stored in nonterm. If no match was found, `match-pattern` returns false. If a match was found, a PEG match record is returned.

The `capture-type` argument to `define-peg-pattern` allows you to choose what information to hold on to while parsing. The options are:

`all`

tag the matched text with the nonterminal

`body`

just the matched text

`none`

nothing

(define-peg-pattern as all ([+](06_06_02_numerical_data_types.md) "a"))
([match-pattern](06_15_peg_parsing.md) as "aabbcc") ⇒ 
#<peg start: 0 end: 2 string: aabbcc tree: (as aa)[\>](06_06_02_numerical_data_types.md)

(define-peg-pattern as body ([+](06_06_02_numerical_data_types.md) "a"))
([match-pattern](06_15_peg_parsing.md) as "aabbcc") ⇒ 
#<peg start: 0 end: 2 string: aabbcc tree: aa>

(define-peg-pattern as none ([+](06_06_02_numerical_data_types.md) "a"))
([match-pattern](06_15_peg_parsing.md) as "aabbcc") ⇒ 
#<peg start: 0 end: 2 string: aabbcc tree: ()[\>](06_06_02_numerical_data_types.md)

(define-peg-pattern bs body ([+](06_06_02_numerical_data_types.md) "b"))
([match-pattern](06_15_peg_parsing.md) bs "aabbcc") ⇒ 
#f

Scheme Macro: **search-for-pattern** nonterm-or-peg string [¶](06_15_peg_parsing.md)

Searches through string looking for a matching subexpression. nonterm-or-peg can either be a nonterminal or a literal PEG pattern. When a literal PEG pattern is provided, `search-for-pattern` works very similarly to the regular expression searches many hackers are used to. If no match was found, `search-for-pattern` returns false. If a match was found, a PEG match record is returned.

(define-peg-pattern as body ([+](06_06_02_numerical_data_types.md) "a"))
([search-for-pattern](06_15_peg_parsing.md) as "aabbcc") ⇒ 
#<peg start: 0 end: 2 string: aabbcc tree: aa>
([search-for-pattern](06_15_peg_parsing.md) ([+](06_06_02_numerical_data_types.md) "a") "aabbcc") ⇒ 
#<peg start: 0 end: 2 string: aabbcc tree: aa>
([search-for-pattern](06_15_peg_parsing.md) "'a'+" "aabbcc") ⇒ 
#<peg start: 0 end: 2 string: aabbcc tree: aa>

(define-peg-pattern as all ([+](06_06_02_numerical_data_types.md) "a"))
([search-for-pattern](06_15_peg_parsing.md) as "aabbcc") ⇒ 
#<peg start: 0 end: 2 string: aabbcc tree: (as aa)[\>](06_06_02_numerical_data_types.md)

(define-peg-pattern bs body ([+](06_06_02_numerical_data_types.md) "b"))
([search-for-pattern](06_15_peg_parsing.md) bs "aabbcc") ⇒ 
#<peg start: 2 end: 4 string: aabbcc tree: bb>
([search-for-pattern](06_15_peg_parsing.md) ([+](06_06_02_numerical_data_types.md) "b") "aabbcc") ⇒ 
#<peg start: 2 end: 4 string: aabbcc tree: bb>
([search-for-pattern](06_15_peg_parsing.md) "'b'+" "aabbcc") ⇒ 
#<peg start: 2 end: 4 string: aabbcc tree: bb>

(define-peg-pattern zs body ([+](06_06_02_numerical_data_types.md) "z"))
([search-for-pattern](06_15_peg_parsing.md) zs "aabbcc") ⇒ 
#f
([search-for-pattern](06_15_peg_parsing.md) ([+](06_06_02_numerical_data_types.md) "z") "aabbcc") ⇒ 
#f
([search-for-pattern](06_15_peg_parsing.md) "'z'+" "aabbcc") ⇒ 
#f

#### PEG Match Records [¶](06_15_peg_parsing.md#peg-match-records)

The `match-pattern` and `search-for-pattern` functions both return PEG match records. Actual information can be extracted from these with the following functions.

Scheme Procedure: **peg:string** match-record [¶](06_15_peg_parsing.md)

Returns the original string that was parsed in the creation of `match-record`.

Scheme Procedure: **peg:start** match-record [¶](06_15_peg_parsing.md)

Returns the index of the first parsed character in the original string (from `peg:string`). If this is the same as `peg:end`, nothing was parsed.

Scheme Procedure: **peg:end** match-record [¶](06_15_peg_parsing.md)

Returns one more than the index of the last parsed character in the original string (from `peg:string`). If this is the same as `peg:start`, nothing was parsed.

Scheme Procedure: **peg:substring** match-record [¶](06_15_peg_parsing.md)

Returns the substring parsed by `match-record`. This is equivalent to `(substring (peg:string match-record) (peg:start match-record) (peg:end match-record))`.

Scheme Procedure: **peg:tree** match-record [¶](06_15_peg_parsing.md)

Returns the tree parsed by `match-record`.

Scheme Procedure: **peg-record?** match-record [¶](06_15_peg_parsing.md)

Returns true if `match-record` is a PEG match record, or false otherwise.

Example:

(define-peg-pattern bs all ([peg](06_15_peg_parsing.md) "'b'+"))

([search-for-pattern](06_15_peg_parsing.md) bs "aabbcc") ⇒
#<peg start: 2 end: 4 string: aabbcc tree: (bs bb)[\>](06_06_02_numerical_data_types.md)

(let ((pm ([search-for-pattern](06_15_peg_parsing.md) bs "aabbcc")))
   \`(([string](06_06_05_strings.md) ,([peg:string](06_15_peg_parsing.md) pm))
     (start ,([peg:start](06_15_peg_parsing.md) pm))
     (end ,([peg:end](06_15_peg_parsing.md) pm))
     ([substring](06_06_05_strings.md) ,([peg:substring](06_15_peg_parsing.md) pm))
     (tree ,([peg:tree](06_15_peg_parsing.md) pm))
     ([record?](06_06_17_records.md) ,([peg-record?](06_15_peg_parsing.md) pm)))) ⇒
(([string](06_06_05_strings.md) "aabbcc")
 (start 2)
 (end 4)
 ([substring](06_06_05_strings.md) "bb")
 (tree (bs "bb"))
 ([record?](06_06_17_records.md) #t))

#### Miscellaneous [¶](06_15_peg_parsing.md#miscellaneous)

Scheme Procedure: **context-flatten** tst lst [¶](06_15_peg_parsing.md)

Takes a predicate tst and a list lst. Flattens lst until all elements are either atoms or satisfy tst. If lst itself satisfies tst, `(list lst)` is returned (this is a flat list whose only element satisfies tst).

([context-flatten](06_15_peg_parsing.md) (lambda (x) (and ([number?](06_06_02_numerical_data_types.md) ([car](06_06_08_pairs.md) x)) ([\=](06_06_02_numerical_data_types.md) ([car](06_06_08_pairs.md) x) 1))) '(2 2 (1 1 (2 2)) (2 2 (1 1)))) ⇒ 
(2 2 (1 1 (2 2)) 2 2 (1 1))
([context-flatten](06_15_peg_parsing.md) (lambda (x) (and ([number?](06_06_02_numerical_data_types.md) ([car](06_06_08_pairs.md) x)) ([\=](06_06_02_numerical_data_types.md) ([car](06_06_08_pairs.md) x) 1))) '(1 1 (1 1 (2 2)) (2 2 (1 1)))) ⇒ 
((1 1 (1 1 (2 2)) (2 2 (1 1))))

If you’re wondering why this is here, take a look at the tutorial.

Scheme Procedure: **keyword-flatten** terms lst [¶](06_15_peg_parsing.md)

A less general form of `context-flatten`. Takes a list of terminal atoms `terms` and flattens lst until all elements are either atoms, or lists which have an atom from `terms` as their first element.

([keyword-flatten](06_15_peg_parsing.md) '(a b) '(c a b (a c) (b c) (c (b a) (c a)))) ⇒
(c a b (a c) (b c) c (b a) c a)

If you’re wondering why this is here, take a look at the tutorial.

* * *

Next: [PEG Internals](06_15_peg_parsing.md#6154-peg-internals), Previous: [PEG API Reference](06_15_peg_parsing.md#6152-peg-api-reference), Up: [PEG Parsing](06_15_peg_parsing.md#615-peg-parsing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.15.3 PEG Tutorial [¶](06_15_peg_parsing.md#6153-peg-tutorial)

#### Parsing /etc/passwd [¶](06_15_peg_parsing.md#parsing-etcpasswd)

This example will show how to parse /etc/passwd using PEGs.

First we define an example /etc/passwd file:

(define \*etc-passwd\*
  "root:x:0:0:root:/root:/bin/bash
daemon:x:1:1:daemon:/usr/sbin:/bin/sh
bin:x:2:2:bin:/bin:/bin/sh
sys:x:3:3:sys:/dev:/bin/sh
nobody:x:65534:65534:nobody:/nonexistent:/bin/sh
messagebus:x:103:107::/var/run/dbus:/bin/false
")

As a first pass at this, we might want to have all the entries in /etc/passwd in a list.

Doing this with string-based PEG syntax would look like this:

(define-peg-string-patterns
  "passwd <- entry\* !.
entry <-- (! NL .)\* NL\*
NL < '\\n'")

A `passwd` file is 0 or more entries (`entry*`) until the end of the file (`!.` (`.` is any character, so `!.` means “not anything”)). We want to capture the data in the nonterminal `passwd`, but not tag it with the name, so we use `<-`.

An entry is a series of 0 or more characters that aren’t newlines (`(! NL .)*`) followed by 0 or more newlines (`NL*`). We want to tag all the entries with `entry`, so we use `<--`.

A newline is just a literal newline (`'\n'`). We don’t want a bunch of newlines cluttering up the output, so we use `<` to throw away the captured data.

Here is the same PEG defined using S-expressions:

(define-peg-pattern passwd body (and ([\*](06_06_02_numerical_data_types.md) entry) ([not-followed-by](06_15_peg_parsing.md) peg-any)))
(define-peg-pattern entry all (and ([\*](06_06_02_numerical_data_types.md) (and ([not-followed-by](06_15_peg_parsing.md) NL) peg-any))
			       ([\*](06_06_02_numerical_data_types.md) NL)))
(define-peg-pattern NL none "\\n")

Obviously this is much more verbose. On the other hand, it’s more explicit, and thus easier to build automatically. However, there are some tricks that make S-expressions easier to use in some cases. One is the `ignore` keyword; the string syntax has no way to say “throw away this text” except breaking it out into a separate nonterminal. For instance, to throw away the newlines we had to define `NL`. In the S-expression syntax, we could have simply written `(ignore "\n")`. Also, for the cases where string syntax is really much cleaner, the `peg` keyword can be used to embed string syntax in S-expression syntax. For instance, we could have written:

(define-peg-pattern passwd body ([peg](06_15_peg_parsing.md) "entry\* !."))

However we define it, parsing `*etc-passwd*` with the `passwd` nonterminal yields the same results:

([peg:tree](06_15_peg_parsing.md) ([match-pattern](06_15_peg_parsing.md) passwd \*etc-passwd\*)) ⇒
((entry "root:x:0:0:root:/root:/bin/bash")
 (entry "daemon:x:1:1:daemon:/usr/sbin:/bin/sh")
 (entry "bin:x:2:2:bin:/bin:/bin/sh")
 (entry "sys:x:3:3:sys:/dev:/bin/sh")
 (entry "nobody:x:65534:65534:nobody:/nonexistent:/bin/sh")
 (entry "messagebus:x:103:107::/var/run/dbus:/bin/false"))

However, here is something to be wary of:

([peg:tree](06_15_peg_parsing.md) ([match-pattern](06_15_peg_parsing.md) passwd "one entry")) ⇒
(entry "one entry")

By default, the parse trees generated by PEGs are compressed as much as possible without losing information. It may not look like this is what you want at first, but uncompressed parse trees are an enormous headache (there’s no easy way to predict how deep particular lists will nest, there are empty lists littered everywhere, etc. etc.). One side-effect of this, however, is that sometimes the compressor is too aggressive. No information is discarded when `((entry "one entry"))` is compressed to `(entry "one entry")`, but in this particular case it probably isn’t what we want.

There are two functions for easily dealing with this: `keyword-flatten` and `context-flatten`. The `keyword-flatten` function takes a list of keywords and a list to flatten, then tries to coerce the list such that the first element of all sublists is one of the keywords. The `context-flatten` function is similar, but instead of a list of keywords it takes a predicate that should indicate whether a given sublist is good enough (refer to the API reference for more details).

What we want here is `keyword-flatten`.

([keyword-flatten](06_15_peg_parsing.md) '(entry) ([peg:tree](06_15_peg_parsing.md) ([match-pattern](06_15_peg_parsing.md) passwd \*etc-passwd\*))) ⇒
((entry "root:x:0:0:root:/root:/bin/bash")
 (entry "daemon:x:1:1:daemon:/usr/sbin:/bin/sh")
 (entry "bin:x:2:2:bin:/bin:/bin/sh")
 (entry "sys:x:3:3:sys:/dev:/bin/sh")
 (entry "nobody:x:65534:65534:nobody:/nonexistent:/bin/sh")
 (entry "messagebus:x:103:107::/var/run/dbus:/bin/false"))
([keyword-flatten](06_15_peg_parsing.md) '(entry) ([peg:tree](06_15_peg_parsing.md) ([match-pattern](06_15_peg_parsing.md) passwd "one entry"))) ⇒
((entry "one entry"))

Of course, this is a somewhat contrived example. In practice we would probably just tag the `passwd` nonterminal to remove the ambiguity (using either the `all` keyword for S-expressions or the `<--` symbol for strings)..

(define-peg-pattern tag-passwd all ([peg](06_15_peg_parsing.md) "entry\* !."))
([peg:tree](06_15_peg_parsing.md) ([match-pattern](06_15_peg_parsing.md) tag-passwd \*etc-passwd\*)) ⇒
(tag-passwd
  (entry "root:x:0:0:root:/root:/bin/bash")
  (entry "daemon:x:1:1:daemon:/usr/sbin:/bin/sh")
  (entry "bin:x:2:2:bin:/bin:/bin/sh")
  (entry "sys:x:3:3:sys:/dev:/bin/sh")
  (entry "nobody:x:65534:65534:nobody:/nonexistent:/bin/sh")
  (entry "messagebus:x:103:107::/var/run/dbus:/bin/false"))
([peg:tree](06_15_peg_parsing.md) ([match-pattern](06_15_peg_parsing.md) tag-passwd "one entry"))
(tag-passwd 
  (entry "one entry"))

If you’re ever uncertain about the potential results of parsing something, remember the two absolute rules:

1.  No parsing information will ever be discarded.
2.  There will never be any lists with fewer than 2 elements.

For the purposes of (1), "parsing information" means things tagged with the `any` keyword or the `<--` symbol. Plain strings will be concatenated.

Let’s extend this example a bit more and actually pull some useful information out of the passwd file:

(define-peg-string-patterns
  "passwd <-- entry\* !.
entry <-- login C pass C uid C gid C nameORcomment C homedir C shell NL\*
login <-- text
pass <-- text
uid <-- \[0-9\]\*
gid <-- \[0-9\]\*
nameORcomment <-- text
homedir <-- path
shell <-- path
path <-- (SLASH pathELEMENT)\*
pathELEMENT <-- (!NL !C  !'/' .)\*
text <- (!NL !C  .)\*
C < ':'
NL < '\\n'
SLASH < '/'")

This produces rather pretty parse trees:

(passwd
  (entry (login "root")
         (pass "x")
         (uid "0")
         (gid "0")
         (nameORcomment "root")
         (homedir (path (pathELEMENT "root")))
         (shell (path (pathELEMENT "bin") (pathELEMENT "bash"))))
  (entry (login "daemon")
         (pass "x")
         (uid "1")
         (gid "1")
         (nameORcomment "daemon")
         (homedir
           (path (pathELEMENT "usr") (pathELEMENT "sbin")))
         (shell (path (pathELEMENT "bin") (pathELEMENT "sh"))))
  (entry (login "bin")
         (pass "x")
         (uid "2")
         (gid "2")
         (nameORcomment "bin")
         (homedir (path (pathELEMENT "bin")))
         (shell (path (pathELEMENT "bin") (pathELEMENT "sh"))))
  (entry (login "sys")
         (pass "x")
         (uid "3")
         (gid "3")
         (nameORcomment "sys")
         (homedir (path (pathELEMENT "dev")))
         (shell (path (pathELEMENT "bin") (pathELEMENT "sh"))))
  (entry (login "nobody")
         (pass "x")
         (uid "65534")
         (gid "65534")
         (nameORcomment "nobody")
         (homedir (path (pathELEMENT "nonexistent")))
         (shell (path (pathELEMENT "bin") (pathELEMENT "sh"))))
  (entry (login "messagebus")
         (pass "x")
         (uid "103")
         (gid "107")
         nameORcomment
         (homedir
           (path (pathELEMENT "var")
                 (pathELEMENT "run")
                 (pathELEMENT "dbus")))
         (shell (path (pathELEMENT "bin") (pathELEMENT "false")))))

Notice that when there’s no entry in a field (e.g. `nameORcomment` for messagebus) the symbol is inserted. This is the “don’t throw away any information” rule—we successfully matched a `nameORcomment` of 0 characters (since we used `*` when defining it). This is usually what you want, because it allows you to e.g. use `list-ref` to pull out elements (since they all have known offsets).

If you’d prefer not to have symbols for empty matches, you can replace the `*` with a `+` and add a `?` after the `nameORcomment` in `entry`. Then it will try to parse 1 or more characters, fail (inserting nothing into the parse tree), but continue because it didn’t have to match the nameORcomment to continue.

#### Embedding Arithmetic Expressions [¶](06_15_peg_parsing.md#embedding-arithmetic-expressions)

We can parse simple mathematical expressions with the following PEG:

(define-peg-string-patterns
  "expr <- sum
sum <-- (product ('+' / '-') sum) / product
product <-- (value ('\*' / '/') product) / value
value <-- number / '(' expr ')'
number <-- \[0-9\]+")

Then:

([peg:tree](06_15_peg_parsing.md) ([match-pattern](06_15_peg_parsing.md) expr "1+1/2\*3+(1+1)/2")) ⇒
(sum (product (value (number "1")))
     "+"
     (sum (product
            (value (number "1"))
            "/"
            (product
              (value (number "2"))
              "\*"
              (product (value (number "3")))))
          "+"
          (sum (product
                 (value "("
                        (sum (product (value (number "1")))
                             "+"
                             (sum (product (value (number "1")))))
                        ")")
                 "/"
                 (product (value (number "2")))))))

There is very little wasted effort in this PEG. The `number` nonterminal has to be tagged because otherwise the numbers might run together with the arithmetic expressions during the string concatenation stage of parse-tree compression (the parser will see “1” followed by “/” and decide to call it “1/”). When in doubt, tag.

It is very easy to turn these parse trees into lisp expressions:

(define (parse-sum sum left . rest)
  (if ([null?](06_06_09_lists.md) rest)
      ([apply](06_16_reading_and_evaluating_scheme_code.md) parse-product left)
      ([list](06_06_09_lists.md) ([string->symbol](06_06_06_symbols.md) ([car](06_06_08_pairs.md) rest))
	    ([apply](06_16_reading_and_evaluating_scheme_code.md) parse-product left)
	    ([apply](06_16_reading_and_evaluating_scheme_code.md) parse-sum ([cadr](06_06_08_pairs.md) rest)))))

(define (parse-product product left . rest)
  (if ([null?](06_06_09_lists.md) rest)
      ([apply](06_16_reading_and_evaluating_scheme_code.md) parse-value left)
      ([list](06_06_09_lists.md) ([string->symbol](06_06_06_symbols.md) ([car](06_06_08_pairs.md) rest))
	    ([apply](06_16_reading_and_evaluating_scheme_code.md) parse-value left)
	    ([apply](06_16_reading_and_evaluating_scheme_code.md) parse-product ([cadr](06_06_08_pairs.md) rest)))))

(define (parse-value value [first](07_05_03_srfi1_list_library.md) . rest)
  (if ([null?](06_06_09_lists.md) rest)
      ([string->number](06_06_02_numerical_data_types.md) ([cadr](06_06_08_pairs.md) [first](07_05_03_srfi1_list_library.md)))
      ([apply](06_16_reading_and_evaluating_scheme_code.md) parse-sum ([car](06_06_08_pairs.md) rest))))

(define parse-expr parse-sum)

(Notice all these functions look very similar; for a more complicated PEG, it would be worth abstracting.)

Then:

([apply](06_16_reading_and_evaluating_scheme_code.md) parse-expr ([peg:tree](06_15_peg_parsing.md) ([match-pattern](06_15_peg_parsing.md) expr "1+1/2\*3+(1+1)/2"))) ⇒
([+](06_06_02_numerical_data_types.md) 1 ([+](06_06_02_numerical_data_types.md) ([/](06_06_02_numerical_data_types.md) 1 ([\*](06_06_02_numerical_data_types.md) 2 3)) ([/](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) 1 1) 2)))

But wait! The associativity is wrong! Where it says `(/ 1 (* 2 3))`, it should say `(* (/ 1 2) 3)`.

It’s tempting to try replacing e.g. `"sum <-- (product ('+' / '-') sum) / product"` with `"sum <-- (sum ('+' / '-') product) / product"`, but this is a Bad Idea. PEGs don’t support left recursion. To see why, imagine what the parser will do here. When it tries to parse `sum`, it first has to try and parse `sum`. But to do that, it first has to try and parse `sum`. This will continue until the stack gets blown off.

So how does one parse left-associative binary operators with PEGs? Honestly, this is one of their major shortcomings. There’s no general-purpose way of doing this, but here the repetition operators are a good choice:

([use-modules](06_18_modules.md) (srfi srfi-1))

(define-peg-string-patterns
  "expr <- sum
sum <-- (product ('+' / '-'))\* product
product <-- (value ('\*' / '/'))\* value
value <-- number / '(' expr ')'
number <-- \[0-9\]+")

;; take a deep breath...
(define (make-left-parser next-func)
  (lambda (sum [first](07_05_03_srfi1_list_library.md) . rest) ;; general form, comments below assume
    ;; that we're dealing with a sum expression
    (if ([null?](06_06_09_lists.md) rest) ;; form (sum (product ...))
      ([apply](06_16_reading_and_evaluating_scheme_code.md) next-func [first](07_05_03_srfi1_list_library.md))
      (if ([string?](06_06_05_strings.md) ([cadr](06_06_08_pairs.md) [first](07_05_03_srfi1_list_library.md)));; form (sum ((product ...) "+") (product ...))
	  ([list](06_06_09_lists.md) ([string->symbol](06_06_06_symbols.md) ([cadr](06_06_08_pairs.md) [first](07_05_03_srfi1_list_library.md)))
		([apply](06_16_reading_and_evaluating_scheme_code.md) next-func ([car](06_06_08_pairs.md) [first](07_05_03_srfi1_list_library.md)))
		([apply](06_16_reading_and_evaluating_scheme_code.md) next-func ([car](06_06_08_pairs.md) rest)))
          ;; form (sum (((product ...) "+") ((product ...) "+")) (product ...))
	  ([car](06_06_08_pairs.md) 
	   ([reduce](07_05_03_srfi1_list_library.md) ;; walk through the list and build a left-associative tree
	    (lambda (l r)
	      ([list](06_06_09_lists.md) ([list](06_06_09_lists.md) ([cadr](06_06_08_pairs.md) r) ([car](06_06_08_pairs.md) r) ([apply](06_16_reading_and_evaluating_scheme_code.md) next-func ([car](06_06_08_pairs.md) l)))
		    ([string->symbol](06_06_06_symbols.md) ([cadr](06_06_08_pairs.md) l))))
	    'ignore
	    ([append](06_06_09_lists.md) ;; make a list of all the products
             ;; the first one should be pre-parsed
	     ([list](06_06_09_lists.md) ([list](06_06_09_lists.md) ([apply](06_16_reading_and_evaluating_scheme_code.md) next-func ([caar](06_06_08_pairs.md) [first](07_05_03_srfi1_list_library.md)))
			 ([string->symbol](06_06_06_symbols.md) ([cadar](06_06_08_pairs.md) [first](07_05_03_srfi1_list_library.md)))))
	     ([cdr](06_06_08_pairs.md) [first](07_05_03_srfi1_list_library.md))
             ;; the last one has to be added in
	     ([list](06_06_09_lists.md) ([append](06_06_09_lists.md) rest '("done"))))))))))

(define (parse-value value [first](07_05_03_srfi1_list_library.md) . rest)
  (if ([null?](06_06_09_lists.md) rest)
      ([string->number](06_06_02_numerical_data_types.md) ([cadr](06_06_08_pairs.md) [first](07_05_03_srfi1_list_library.md)))
      ([apply](06_16_reading_and_evaluating_scheme_code.md) parse-sum ([car](06_06_08_pairs.md) rest))))
(define parse-product (make-left-parser parse-value))
(define parse-sum (make-left-parser parse-product))
(define parse-expr parse-sum)

Then:

([apply](06_16_reading_and_evaluating_scheme_code.md) parse-expr ([peg:tree](06_15_peg_parsing.md) ([match-pattern](06_15_peg_parsing.md) expr "1+1/2\*3+(1+1)/2"))) ⇒
([+](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) 1 ([\*](06_06_02_numerical_data_types.md) ([/](06_06_02_numerical_data_types.md) 1 2) 3)) ([/](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) 1 1) 2))

As you can see, this is much uglier (it could be made prettier by using `context-flatten`, but the way it’s written above makes it clear how we deal with the three ways the zero-or-more `*` expression can parse). Fortunately, most of the time we can get away with only using right-associativity.

#### Simplified Functions [¶](06_15_peg_parsing.md#simplified-functions)

For a more tantalizing example, consider the following grammar that parses (highly) simplified C functions:

(define-peg-string-patterns
  "cfunc <-- cSP ctype cSP cname cSP cargs cLB cSP cbody cRB
ctype <-- cidentifier
cname <-- cidentifier
cargs <-- cLP (! (cSP cRP) carg cSP (cCOMMA / cRP) cSP)\* cSP
carg <-- cSP ctype cSP cname
cbody <-- cstatement \*
cidentifier <- \[a-zA-z\]\[a-zA-Z0-9\_\]\*
cstatement <-- (!';'.)\*cSC cSP
cSC < ';'
cCOMMA < ','
cLP < '('
cRP < ')'
cLB < '{'
cRB < '}'
cSP < \[ \\t\\n\]\*")

Then:

([match-pattern](06_15_peg_parsing.md) cfunc "int square(int a) { return a\*a;}") ⇒
(32
 (cfunc (ctype "int")
        (cname "square")
        (cargs (carg (ctype "int") (cname "a")))
        (cbody (cstatement "return a\*a"))))

And:

([match-pattern](06_15_peg_parsing.md) cfunc "int mod(int a, int b) { int c = a/b;return a-b\*c; }") ⇒
(52
 (cfunc (ctype "int")
        (cname "mod")
        (cargs (carg (ctype "int") (cname "a"))
               (carg (ctype "int") (cname "b")))
        (cbody (cstatement "int c = a/b")
               (cstatement "return a- b\*c"))))

By wrapping all the `carg` nonterminals in a `cargs` nonterminal, we were able to remove any ambiguity in the parsing structure and avoid having to call `context-flatten` on the output of `match-pattern`. We used the same trick with the `cstatement` nonterminals, wrapping them in a `cbody` nonterminal.

The whitespace nonterminal `cSP` used here is a (very) useful instantiation of a common pattern for matching syntactically irrelevant information. Since it’s tagged with `<` and ends with `*` it won’t clutter up the parse trees (all the empty lists will be discarded during the compression step) and it will never cause parsing to fail.

* * *

Previous: [PEG Tutorial](06_15_peg_parsing.md#6153-peg-tutorial), Up: [PEG Parsing](06_15_peg_parsing.md#615-peg-parsing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.15.4 PEG Internals [¶](06_15_peg_parsing.md#6154-peg-internals)

A PEG parser takes a string as input and attempts to parse it as a given nonterminal. The key idea of the PEG implementation is that every nonterminal is just a function that takes a string as an argument and attempts to parse that string as its nonterminal. The functions always start from the beginning, but a parse is considered successful if there is material left over at the end.

This makes it easy to model different PEG parsing operations. For instance, consider the PEG grammar `"ab"`, which could also be written `(and "a" "b")`. It matches the string “ab”. Here’s how that might be implemented in the PEG style:

(define (match-and-a-b str)
  (match-a str)
  (match-b str))

As you can see, the use of functions provides an easy way to model sequencing. In a similar way, one could model `(or a b)` with something like the following:

(define (match-or-a-b str)
  (or (match-a str) (match-b str)))

Here the semantics of a PEG `or` expression map naturally onto Scheme’s `or` operator. This function will attempt to run `(match-a str)`, and return its result if it succeeds. Otherwise it will run `(match-b str)`.

Of course, the code above wouldn’t quite work. We need some way for the parsing functions to communicate. The actual interface used is below.

#### Parsing Function Interface [¶](06_15_peg_parsing.md#parsing-function-interface)

A parsing function takes three arguments - a string, the length of that string, and the position in that string it should start parsing at. In effect, the parsing functions pass around substrings in pieces - the first argument is a buffer of characters, and the second two give a range within that buffer that the parsing function should look at.

Parsing functions return either #f, if they failed to match their nonterminal, or a list whose first element must be an integer representing the final position in the string they matched and whose cdr can be any other data the function wishes to return, or ’() if it doesn’t have any more data.

The one caveat is that if the extra data it returns is a list, any adjacent strings in that list will be appended by `match-pattern`. For instance, if a parsing function returns `(13 ("a" "b" "c"))`, `match-pattern` will take `(13 ("abc"))` as its value.

For example, here is a function to match “ab” using the actual interface.

(define (match-a-b str len pos)
   (and ([<=](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) pos 2) len)
        ([string=](06_06_05_strings.md) str "ab" pos ([+](06_06_02_numerical_data_types.md) pos 2))
        ([list](06_06_09_lists.md) ([+](06_06_02_numerical_data_types.md) pos 2) '()))) ; we return no extra information

The above function can be used to match a string by running `(match-pattern match-a-b "ab")`.

#### Code Generators and Extensible Syntax [¶](06_15_peg_parsing.md#code-generators-and-extensible-syntax)

PEG expressions, such as those in a `define-peg-pattern` form, are interpreted internally in two steps.

First, any string PEG is expanded into an s-expression PEG by the code in the `(ice-9 peg string-peg)` module.

Then, the s-expression PEG that results is compiled into a parsing function by the `(ice-9 peg codegen)` module. In particular, the function `compile-peg-pattern` is called on the s-expression. It then decides what to do based on the form it is passed.

The PEG syntax can be expanded by providing `compile-peg-pattern` more options for what to do with its forms. The extended syntax will be associated with a symbol, for instance `my-parsing-form`, and will be called on all PEG expressions of the form

(my-parsing-form [...](06_08_macros.md))

The parsing function should take two arguments. The first will be a syntax object containing a list with all of the arguments to the form (but not the form’s name), and the second will be the `capture-type` argument that is passed to `define-peg-pattern`.

New functions can be registered by calling `(add-peg-compiler! symbol function)`, where `symbol` is the symbol that will indicate a form of this type and `function` is the code generating function described above. The function `add-peg-compiler!` is exported from the `(ice-9 peg codegen)` module.

* * *

Next: [Memory Management and Garbage Collection](06_17_memory_management_and_garbage_collection.md#617-memory-management-and-garbage-collection), Previous: [PEG Parsing](06_15_peg_parsing.md#615-peg-parsing), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
