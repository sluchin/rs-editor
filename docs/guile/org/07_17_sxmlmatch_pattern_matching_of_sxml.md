### 7.17 `sxml-match`: Pattern Matching of SXML [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#717-sxml-match-pattern-matching-of-sxml)

The `(sxml match)` module provides syntactic forms for pattern matching of SXML trees, in a “by example” style reminiscent of the pattern matching of the `syntax-rules` and `syntax-case` macro systems. See [SXML](07_21_sxml.md#721-sxml), for more information on SXML.

The following example[29](99_footnotes.md) provides a brief illustration, transforming a music album catalog language into HTML.

(define (album->html x)
  ([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) x
    ((album ([@](06_18_modules.md) (title ,t)) (catalog (num ,n) (fmt ,f)) [...](06_08_macros.md))
     \`(ul (li ,t)
          (li (b ,n) (i ,f)) [...](06_08_macros.md)))))

Three macros are provided: `sxml-match`, `sxml-match-let`, and `sxml-match-let*`.

Compared to a standard s-expression pattern matcher (see [Pattern Matching](07_08_pattern_matching.md#78-pattern-matching)), `sxml-match` provides the following benefits:

*   matching of SXML elements does not depend on any degree of normalization of the SXML;
*   matching of SXML attributes (within an element) is under-ordered; the order of the attributes specified within the pattern need not match the ordering with the element being matched;
*   all attributes specified in the pattern must be present in the element being matched; in the spirit that XML is ’extensible’, the element being matched may include additional attributes not specified in the pattern.

The present module is a descendant of WebIt!, and was inspired by an s-expression pattern matcher developed by Erik Hilsdale, Dan Friedman, and Kent Dybvig at Indiana University.

*   [Syntax](07_17_sxmlmatch_pattern_matching_of_sxml.md#syntax)
*   [Matching XML Elements](07_17_sxmlmatch_pattern_matching_of_sxml.md#matching-xml-elements)
*   [Ellipses in Patterns](07_17_sxmlmatch_pattern_matching_of_sxml.md#ellipses-in-patterns)
*   [Ellipses in Quasiquote’d Output](07_17_sxmlmatch_pattern_matching_of_sxml.md#ellipses-in-quasiquoted-output)
*   [Matching Nodesets](07_17_sxmlmatch_pattern_matching_of_sxml.md#matching-nodesets)
*   [Matching the “Rest” of a Nodeset](07_17_sxmlmatch_pattern_matching_of_sxml.md#matching-the-rest-of-a-nodeset)
*   [Matching the Unmatched Attributes](07_17_sxmlmatch_pattern_matching_of_sxml.md#matching-the-unmatched-attributes)
*   [Default Values in Attribute Patterns](07_17_sxmlmatch_pattern_matching_of_sxml.md#default-values-in-attribute-patterns)
*   [Guards in Patterns](07_17_sxmlmatch_pattern_matching_of_sxml.md#guards-in-patterns)
*   [Catamorphisms](07_17_sxmlmatch_pattern_matching_of_sxml.md#catamorphisms)
*   [Named-Catamorphisms](07_17_sxmlmatch_pattern_matching_of_sxml.md#named-catamorphisms)
*   [`sxml-match-let` and `sxml-match-let*`](07_17_sxmlmatch_pattern_matching_of_sxml.md#sxml-match-let-and-sxml-match-let)

#### Syntax [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#syntax)

`sxml-match` provides `case`\-like form for pattern matching of XML nodes.

Scheme Syntax: **sxml-match** input-expression clause1 clause2 … [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md)

Match input-expression, an SXML tree, according to the given clauses (one or more), each consisting of a pattern and one or more expressions to be evaluated if the pattern match succeeds. Optionally, each clause within `sxml-match` may include a _guard expression_.

The pattern notation is based on that of Scheme’s `syntax-rules` and `syntax-case` macro systems. The grammar for the `sxml-match` syntax is given below:

match-form ::= (sxml-match input-expression
                 clause+)

clause ::= \[node-pattern action-expression+\]
         | \[node-pattern (guard expression\*) action-expression+\]

node-pattern ::= literal-pattern
               | pat-var-or-cata
               | element-pattern
               | list-pattern

literal-pattern ::= string
                  | character
                  | number
                  | #t
                  | #f

attr-list-pattern ::= (@ attribute-pattern\*)
                    | (@ attribute-pattern\* . pat-var-or-cata)

attribute-pattern ::= (tag-symbol attr-val-pattern)

attr-val-pattern ::= literal-pattern
                   | pat-var-or-cata
                   | (pat-var-or-cata default-value-expr)

element-pattern ::= (tag-symbol attr-list-pattern?)
                  | (tag-symbol attr-list-pattern? nodeset-pattern)
                  | (tag-symbol attr-list-pattern?
                                nodeset-pattern? . pat-var-or-cata)

list-pattern ::= (list nodeset-pattern)
               | (list nodeset-pattern? . pat-var-or-cata)
               | (list)

nodeset-pattern ::= node-pattern
                  | node-pattern ...
                  | node-pattern nodeset-pattern
                  | node-pattern ... nodeset-pattern

pat-var-or-cata ::= (unquote var-symbol)
                  | (unquote \[var-symbol\*\])
                  | (unquote \[cata-expression -> var-symbol\*\])

Within a list or element body pattern, ellipses may appear only once, but may be followed by zero or more node patterns.

Guard expressions cannot refer to the return values of catamorphisms.

Ellipses in the output expressions must appear only in an expression context; ellipses are not allowed in a syntactic form.

The sections below illustrate specific aspects of the `sxml-match` pattern matcher.

#### Matching XML Elements [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#matching-xml-elements)

The example below illustrates the pattern matching of an XML element:

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) '(e ([@](06_18_modules.md) (i 1)) 3 4 5)
  ((e ([@](06_18_modules.md) (i ,d)) ,a ,b ,c) ([list](06_06_09_lists.md) d a b c))
  (,otherwise #f))

Each clause in `sxml-match` contains two parts: a pattern and one or more expressions which are evaluated if the pattern is successfully match. The example above matches an element `e` with an attribute `i` and three children.

Pattern variables must be “unquoted” in the pattern. The above expression binds d to `1`, a to `3`, b to `4`, and c to `5`.

#### Ellipses in Patterns [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#ellipses-in-patterns)

As in `syntax-rules`, ellipses may be used to specify a repeated pattern. Note that the pattern `item ...` specifies zero-or-more matches of the pattern `item`.

The use of ellipses in a pattern is illustrated in the code fragment below, where nested ellipses are used to match the children of repeated instances of an `a` element, within an element `d`.

(define x '(d (a 1 2 3) (a 4 5) (a 6 7 8) (a 9 10)))

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) x
  ((d (a ,b [...](06_08_macros.md)) [...](06_08_macros.md))
   ([list](06_06_09_lists.md) ([list](06_06_09_lists.md) b [...](06_08_macros.md)) [...](06_08_macros.md))))

The above expression returns a value of `((1 2 3) (4 5) (6 7 8) (9 10))`.

#### Ellipses in Quasiquote’d Output [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#ellipses-in-quasiquoted-output)

Within the body of an `sxml-match` form, a slightly extended version of quasiquote is provided, which allows the use of ellipses. This is illustrated in the example below.

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) '(e 3 4 5 6 7)
  ((e ,i [...](06_08_macros.md) 6 7) \`("start" ,([list](06_06_09_lists.md) 'wrap i) [...](06_08_macros.md) "end"))
  (,otherwise #f))

The general pattern is that `` `(something ,i ...) `` is rewritten as `` `(something ,@i) ``.

#### Matching Nodesets [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#matching-nodesets)

A nodeset pattern is designated by a list in the pattern, beginning the identifier list. The example below illustrates matching a nodeset.

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) '("i" "j" "k" "l" "m")
  (([list](06_06_09_lists.md) ,a ,b ,c ,d ,e)
   \`((p ,a) (p ,b) (p ,c) (p ,d) (p ,e))))

This example wraps each nodeset item in an HTML paragraph element. This example can be rewritten and simplified through using ellipsis:

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) '("i" "j" "k" "l" "m")
  (([list](06_06_09_lists.md) ,i [...](06_08_macros.md))
   \`((p ,i) [...](06_08_macros.md))))

This version will match nodesets of any length, and wrap each item in the nodeset in an HTML paragraph element.

#### Matching the “Rest” of a Nodeset [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#matching-the-rest-of-a-nodeset)

Matching the “rest” of a nodeset is achieved by using a `. rest)` pattern at the end of an element or nodeset pattern.

This is illustrated in the example below:

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) '(e 3 (f 4 5 6) 7)
  ((e ,a (f . ,y) ,d)
   ([list](06_06_09_lists.md) a y d)))

The above expression returns `(3 (4 5 6) 7)`.

#### Matching the Unmatched Attributes [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#matching-the-unmatched-attributes)

Sometimes it is useful to bind a list of attributes present in the element being matched, but which do not appear in the pattern. This is achieved by using a `. rest)` pattern at the end of the attribute list pattern. This is illustrated in the example below:

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) '(a ([@](06_18_modules.md) (z 1) (y 2) (x 3)) 4 5 6)
  ((a ([@](06_18_modules.md) (y ,www) . ,qqq) ,t ,u ,v)
   ([list](06_06_09_lists.md) www qqq t u v)))

The above expression matches the attribute `y` and binds a list of the remaining attributes to the variable qqq. The result of the above expression is `(2 ((z 1) (x 3)) 4 5 6)`.

This type of pattern also allows the binding of all attributes:

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) '(a ([@](06_18_modules.md) (z 1) (y 2) (x 3)))
  ((a ([@](06_18_modules.md) . ,qqq))
   qqq))

#### Default Values in Attribute Patterns [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#default-values-in-attribute-patterns)

It is possible to specify a default value for an attribute which is used if the attribute is not present in the element being matched. This is illustrated in the following example:

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) '(e 3 4 5)
  ((e ([@](06_18_modules.md) (z (,d 1))) ,a ,b ,c) ([list](06_06_09_lists.md) d a b c)))

The value `1` is used when the attribute `z` is absent from the element `e`.

#### Guards in Patterns [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#guards-in-patterns)

Guards may be added to a pattern clause via the `guard` keyword. A guard expression may include zero or more expressions which are evaluated only if the pattern is matched. The body of the clause is only evaluated if the guard expressions evaluate to `#t`.

The use of guard expressions is illustrated below:

([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) '(a 2 3)
  ((a ,n) ([guard](07_05_23_srfi34_exception_handling_for_programs.md) ([number?](06_06_02_numerical_data_types.md) n)) n)
  ((a ,m ,n) ([guard](07_05_23_srfi34_exception_handling_for_programs.md) ([number?](06_06_02_numerical_data_types.md) m) ([number?](06_06_02_numerical_data_types.md) n)) ([+](06_06_02_numerical_data_types.md) m n)))

#### Catamorphisms [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#catamorphisms)

The example below illustrates the use of explicit recursion within an `sxml-match` form. This example implements a simple calculator for the basic arithmetic operations, which are represented by the XML elements `plus`, `minus`, `times`, and `div`.

(define simple-eval
  (lambda (x)
    ([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) x
      (,i ([guard](07_05_23_srfi34_exception_handling_for_programs.md) ([integer?](06_06_02_numerical_data_types.md) i)) i)
      ((plus ,x ,y) ([+](06_06_02_numerical_data_types.md) (simple-eval x) (simple-eval y)))
      (([times](07_02_05_time.md) ,x ,y) ([\*](06_06_02_numerical_data_types.md) (simple-eval x) (simple-eval y)))
      ((minus ,x ,y) ([\-](06_06_02_numerical_data_types.md) (simple-eval x) (simple-eval y)))
      (([div](07_06_r6rs_support.md) ,x ,y) ([/](06_06_02_numerical_data_types.md) (simple-eval x) (simple-eval y)))
      (,otherwise ([error](04_programming_in_scheme.md) "simple-eval: invalid expression" x)))))

Using the catamorphism feature of `sxml-match`, a more concise version of `simple-eval` can be written. The pattern `,(x)` recursively invokes the pattern matcher on the value bound in this position.

(define simple-eval
  (lambda (x)
    ([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) x
      (,i ([guard](07_05_23_srfi34_exception_handling_for_programs.md) ([integer?](06_06_02_numerical_data_types.md) i)) i)
      ((plus ,(x) ,(y)) ([+](06_06_02_numerical_data_types.md) x y))
      (([times](07_02_05_time.md) ,(x) ,(y)) ([\*](06_06_02_numerical_data_types.md) x y))
      ((minus ,(x) ,(y)) ([\-](06_06_02_numerical_data_types.md) x y))
      (([div](07_06_r6rs_support.md) ,(x) ,(y)) ([/](06_06_02_numerical_data_types.md) x y))
      (,otherwise ([error](04_programming_in_scheme.md) "simple-eval: invalid expression" x)))))

#### Named-Catamorphisms [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#named-catamorphisms)

It is also possible to explicitly name the operator in the “cata” position. Where `,(id*)` recurs to the top of the current `sxml-match`, `,(cata -> id*)` recurs to `cata`. `cata` must evaluate to a procedure which takes one argument, and returns as many values as there are identifiers following `->`.

Named catamorphism patterns allow processing to be split into multiple, mutually recursive procedures. This is illustrated in the example below: a transformation that formats a “TV Guide” into HTML.

(define (tv-guide->html g)
  (define (cast-list cl)
    ([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) cl
      ((CastList (CastMember (Character (Name ,ch)) (Actor (Name ,a))) [...](06_08_macros.md))
       \`([div](07_06_r6rs_support.md) (ul (li ,ch ": " ,a) [...](06_08_macros.md))))))
  (define (prog p)
    ([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) p
      ((Program (Start ,start-time) (Duration ,dur) (Series ,series-title)
                (Description ,desc [...](06_08_macros.md)))
       \`([div](07_06_r6rs_support.md) (p ,start-time
                (br) ,series-title
                (br) ,desc [...](06_08_macros.md))))
      ((Program (Start ,start-time) (Duration ,dur) (Series ,series-title)
                (Description ,desc [...](06_08_macros.md))
                ,(cast-list \-> cl))
       \`([div](07_06_r6rs_support.md) (p ,start-time
                (br) ,series-title
                (br) ,desc [...](06_08_macros.md))
             ,cl))))
  ([sxml-match](07_17_sxmlmatch_pattern_matching_of_sxml.md) g
    ((TVGuide ([@](06_18_modules.md) (start ,start-date)
                 (end ,end-date))
              (Channel (Name ,nm) ,(prog \-> p) [...](06_08_macros.md)) [...](06_08_macros.md))
     \`(html (head (title "TV Guide"))
            (body (h1 "TV Guide")
                  ([div](07_06_r6rs_support.md) (h2 ,nm) ,p [...](06_08_macros.md)) [...](06_08_macros.md))))))

#### `sxml-match-let` and `sxml-match-let*` [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md#sxml-match-let-and-sxml-match-let)

Scheme Syntax: **sxml-match-let** ((pat expr) ...) expression0 expression ... [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md)

Scheme Syntax: **sxml-match-let\*** ((pat expr) ...) expression0 expression ... [¶](07_17_sxmlmatch_pattern_matching_of_sxml.md)

These forms generalize the `let` and `let*` forms of Scheme to allow an XML pattern in the binding position, rather than a simple variable.

For example, the expression below:

([sxml-match-let](07_17_sxmlmatch_pattern_matching_of_sxml.md) (((a ,i ,j) '(a 1 2)))
  ([+](06_06_02_numerical_data_types.md) i [j](09_03_a_virtual_machine_for_guile.md)))

binds the variables i and j to `1` and `2` in the XML value given.

* * *

Next: [Curried Definitions](07_19_curried_definitions.md#719-curried-definitions), Previous: [`sxml-match`: Pattern Matching of SXML](07_17_sxmlmatch_pattern_matching_of_sxml.md#717-sxml-match-pattern-matching-of-sxml), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
