### 7.8 Pattern Matching [¶](07_08_pattern_matching.md#78-pattern-matching)

The `(ice-9 match)` module provides a _pattern matcher_, written by Alex Shinn, and compatible with Andrew K. Wright’s pattern matcher found in many Scheme implementations.

A pattern matcher can match an object against several patterns and extract the elements that make it up. Patterns can represent any Scheme object: lists, strings, symbols, records, etc. They can optionally contain _pattern variables_. When a matching pattern is found, an expression associated with the pattern is evaluated, optionally with all pattern variables bound to the corresponding elements of the object:

(let ((l '(hello (world))))
  (match l           ;; <- the input object
    (('hello (who))  ;; <- the pattern
     who)))          ;; <- the expression evaluated upon matching
⇒ world

In this example, list l matches the pattern `('hello (who))`, because it is a two-element list whose first element is the symbol `hello` and whose second element is a one-element list. Here who is a pattern variable. `match`, the pattern matcher, locally binds who to the value contained in this one-element list—i.e., the symbol `world`. An error would be raised if l did not match the pattern.

The same object can be matched against a simpler pattern:

(let ((l '(hello (world))))
  (match l
    ((x y)
     (values x y))))
⇒ hello
⇒ (world)

Here pattern `(x y)` matches any two-element list, regardless of the types of these elements. Pattern variables x and y are bound to, respectively, the first and second element of l.

Patterns can be composed, and nested. For instance, `...` (ellipsis) means that the previous pattern may be matched zero or more times in a list:

(match lst
  (((heads tails ...) ...)
   heads))

This expression returns the first element of each list within lst. For proper lists of proper lists, it is equivalent to `(map car lst)`. However, it performs additional checks to make sure that lst and the lists therein are proper lists, as prescribed by the pattern, raising an error if they are not.

Compared to hand-written code, pattern matching noticeably improves clarity and conciseness—no need to resort to series of `car` and `cdr` calls when matching lists, for instance. It also improves robustness, by making sure the input _completely_ matches the pattern—conversely, hand-written code often trades robustness for conciseness. And of course, `match` is a macro, and the code it expands to is just as efficient as equivalent hand-written code.

The pattern matcher is defined as follows:

Scheme Syntax: **match** exp clause1 clause2 … [¶](07_08_pattern_matching.md)

Match object exp against the patterns in clause1 clause2 … in the order in which they appear. Return the value produced by the first matching clause. If no clause matches, throw an exception with key `match-error`.

Each clause has the form `(pattern body1 body2 …)`. Each pattern must follow the syntax described below. Each body is an arbitrary Scheme expression, possibly referring to pattern variables of pattern.

The syntax and interpretation of patterns is as follows:

        patterns:                       matches:

pat ::= identifier                      anything, and binds identifier
      | \_                               anything
      | ()                              the empty list
      | #t                              #t
      | #f                              #f
      | string                          a string
      | number                          a number
      | character                       a character
      | 'sexp                           an s-expression
      | 'symbol                         a symbol (special case of s-expr)
      | (pat\_1 ... pat\_n)               list of n elements
      | (pat\_1 ... pat\_n . pat\_{n+1})   list of n or more
      | (pat\_1 ... pat\_n pat\_n+1 ooo)   list of n or more, each element
                                          of remainder must match pat\_n+1
      | #(pat\_1 ... pat\_n)              vector of n elements
      | #(pat\_1 ... pat\_n pat\_n+1 ooo)  vector of n or more, each element
                                          of remainder must match pat\_n+1
      | #&pat                           box
      | ($ record-name pat\_1 ... pat\_n) a record
      | (= field pat)                   a \`\`field'' of an object
      | (and pat\_1 ... pat\_n)           if all of pat\_1 thru pat\_n match
      | (or pat\_1 ... pat\_n)            if any of pat\_1 thru pat\_n match
      | (not pat\_1 ... pat\_n)           if all pat\_1 thru pat\_n don't match
      | (? predicate pat\_1 ... pat\_n)   if predicate true and all of
                                          pat\_1 thru pat\_n match
      | (set! identifier)               anything, and binds setter
      | (get! identifier)               anything, and binds getter
      | \`qp                             a quasi-pattern
      | (identifier \*\*\* pat)            matches pat in a tree and binds
                                        identifier to the path leading
                                        to the object that matches pat

ooo ::= ...                             zero or more
      | \_\_\_                             zero or more
      | ..1                             1 or more

        quasi-patterns:                 matches:

qp  ::= ()                              the empty list
      | #t                              #t
      | #f                              #f
      | string                          a string
      | number                          a number
      | character                       a character
      | identifier                      a symbol
      | (qp\_1 ... qp\_n)                 list of n elements
      | (qp\_1 ... qp\_n . qp\_{n+1})      list of n or more
      | (qp\_1 ... qp\_n qp\_n+1 ooo)      list of n or more, each element
                                          of remainder must match qp\_n+1
      | #(qp\_1 ... qp\_n)                vector of n elements
      | #(qp\_1 ... qp\_n qp\_n+1 ooo)     vector of n or more, each element
                                          of remainder must match qp\_n+1
      | #&qp                            box
      | ,pat                            a pattern
      | ,@pat                           a pattern

The names `quote`, `quasiquote`, `unquote`, `unquote-splicing`, `?`, `_`, `$`, `and`, `or`, `not`, `set!`, `get!`, `...`, and `___` cannot be used as pattern variables.

Here is a more complex example:

(use-modules (srfi srfi-9))

(let ()
  (define-record-type person
    (make-person name friends)
    person?
    (name    person-name)
    (friends person-friends))

  (letrec ((alice (make-person "Alice" (delay (list bob))))
           (bob   (make-person "Bob" (delay (list alice)))))
    (match alice
      (($ person name (= force (($ person "Bob"))))
       (list 'friend-of-bob name))
      (\_ #f))))

⇒ (friend-of-bob "Alice")

Here the `$` pattern is used to match a SRFI-9 record of type person containing two or more slots. The value of the first slot is bound to name. The `=` pattern is used to apply `force` on the second slot, and then checking that the result matches the given pattern. In other words, the complete pattern matches any person whose second slot is a promise that evaluates to a one-element list containing a person whose first slot is `"Bob"`.

The `(ice-9 match)` module also provides the following convenient syntactic sugar macros wrapping around `match`.

Scheme Syntax: **match-lambda** clause1 clause2 … [¶](07_08_pattern_matching.md)

Create a procedure of one argument that matches its argument against each clause, and returns the result of evaluating the corresponding expressions.

(match-lambda clause1 clause2 ...)
≍
(lambda (arg) (match arg clause1 clause2 ...))

((match-lambda
   (('hello (who))
    who))
 '(hello (world)))
⇒ world

Scheme Syntax: **match-lambda\*** clause1 clause2 … [¶](07_08_pattern_matching.md)

Create a procedure of any number of arguments that matches its argument list against each clause, and returns the result of evaluating the corresponding expressions.

(match-lambda\* clause1 clause2 ...)
≍
(lambda args (match args clause1 clause2 ...))

((match-lambda\*
   (('hello (who))
    who))
 'hello '(world))
⇒ world

Scheme Syntax: **match-let** ((pattern expression) …) body [¶](07_08_pattern_matching.md)

Match each pattern to the corresponding expression, and evaluate the body with all matched variables in scope. Raise an error if any of the expressions fail to match. `match-let` is analogous to named let and can also be used for recursive functions which match on their arguments as in `match-lambda*`.

(match-let (((x y) (list 1 2))
            ((a b) (list 3 4)))
  (list a b x y))
⇒
(3 4 1 2)

Scheme Syntax: **match-let** variable ((pattern init) …) body [¶](07_08_pattern_matching.md)

Similar to `match-let`, but analogously to _named let_, locally bind VARIABLE to a new procedure which accepts as many arguments as there are INIT expressions. The procedure is initially applied to the results of evaluating the INIT expressions. When called, the procedure matches each argument against the corresponding PATTERN, and returns the result(s) of evaluating the BODY expressions. See [Iteration](06_11_controlling_the_flow_of_program_execution.md#6114-iteration-mechanisms), for more on _named let_.

Scheme Syntax: **match-let\*** ((variable expression) …) body [¶](07_08_pattern_matching.md)

Similar to `match-let`, but analogously to `let*`, match and bind the variables in sequence, with preceding match variables in scope.

(match-let\* (((x y) (list 1 2))
             ((a b) (list x 4)))
  (list a b x y))
≍
(match-let (((x y) (list 1 2)))
  (match-let (((a b) (list x 4)))
    (list a b x y)))
⇒
(1 4 1 2)

Scheme Syntax: **match-letrec** ((variable expression) …) body [¶](07_08_pattern_matching.md)

Similar to `match-let`, but analogously to `letrec`, match and bind the variables with all match variables in scope.

Guile also comes with a pattern matcher specifically tailored to SXML trees, See [`sxml-match`: Pattern Matching of SXML](07_17_sxmlmatch_pattern_matching_of_sxml.md#717-sxml-match-pattern-matching-of-sxml).

* * *

Next: [Pretty Printing](07_10_pretty_printing.md#710-pretty-printing), Previous: [Pattern Matching](07_08_pattern_matching.md#78-pattern-matching), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
