#### 7.5.28 SRFI-41 - Streams [¶](07_05_28_srfi41_streams.md#7528-srfi-41---streams)

This subsection is based on the [specification of SRFI-41](http://srfi.schemers.org/srfi-41/srfi-41.html) by Philip L. Bewig.

This SRFI implements streams, sometimes called lazy lists, a sequential data structure containing elements computed only on demand. A stream is either null or is a pair with a stream in its cdr. Since elements of a stream are computed only when accessed, streams can be infinite. Once computed, the value of a stream element is cached in case it is needed again. SRFI-41 can be made available with:

(use-modules (srfi srfi-41))

*   [SRFI-41 Stream Fundamentals](07_05_28_srfi41_streams.md#75281-srfi-41-stream-fundamentals)
*   [SRFI-41 Stream Primitives](07_05_28_srfi41_streams.md#75282-srfi-41-stream-primitives)
*   [SRFI-41 Stream Library](07_05_28_srfi41_streams.md#75283-srfi-41-stream-library)

* * *

Next: [SRFI-41 Stream Primitives](07_05_28_srfi41_streams.md#75282-srfi-41-stream-primitives), Up: [SRFI-41 - Streams](07_05_28_srfi41_streams.md#7528-srfi-41---streams)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.28.1 SRFI-41 Stream Fundamentals [¶](07_05_28_srfi41_streams.md#75281-srfi-41-stream-fundamentals)

SRFI-41 Streams are based on two mutually-recursive abstract data types: An object of the `stream` abstract data type is a promise that, when forced, is either `stream-null` or is an object of type `stream-pair`. An object of the `stream-pair` abstract data type contains a `stream-car` and a `stream-cdr`, which must be a `stream`. The essential feature of streams is the systematic suspensions of the recursive promises between the two data types.

The object stored in the `stream-car` of a `stream-pair` is a promise that is forced the first time the `stream-car` is accessed; its value is cached in case it is needed again. The object may have any type, and different stream elements may have different types. If the `stream-car` is never accessed, the object stored there is never evaluated. Likewise, the `stream-cdr` is a promise to return a stream, and is only forced on demand.

* * *

Next: [SRFI-41 Stream Library](07_05_28_srfi41_streams.md#75283-srfi-41-stream-library), Previous: [SRFI-41 Stream Fundamentals](07_05_28_srfi41_streams.md#75281-srfi-41-stream-fundamentals), Up: [SRFI-41 - Streams](07_05_28_srfi41_streams.md#7528-srfi-41---streams)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.28.2 SRFI-41 Stream Primitives [¶](07_05_28_srfi41_streams.md#75282-srfi-41-stream-primitives)

This library provides eight operators: constructors for `stream-null` and `stream-pair`s, type predicates for streams and the two kinds of streams, accessors for both fields of a `stream-pair`, and a lambda that creates procedures that return streams.

Scheme Variable: **stream-null** [¶](07_05_28_srfi41_streams.md)

A promise that, when forced, is a single object, distinguishable from all other objects, that represents the null stream. `stream-null` is immutable and unique.

Scheme Syntax: **stream-cons** object-expr stream-expr [¶](07_05_28_srfi41_streams.md)

Creates a newly-allocated stream containing a promise that, when forced, is a `stream-pair` with object-expr in its `stream-car` and stream-expr in its `stream-cdr`. Neither object-expr nor stream-expr is evaluated when `stream-cons` is called.

Once created, a `stream-pair` is immutable; there is no `stream-set-car!` or `stream-set-cdr!` that modifies an existing stream-pair. There is no dotted-pair or improper stream as with lists.

Scheme Procedure: **stream?** object [¶](07_05_28_srfi41_streams.md)

Returns true if object is a stream, otherwise returns false. If object is a stream, its promise will not be forced. If `(stream? obj)` returns true, then one of `(stream-null? obj)` or `(stream-pair? obj)` will return true and the other will return false.

Scheme Procedure: **stream-null?** object [¶](07_05_28_srfi41_streams.md)

Returns true if object is the distinguished null stream, otherwise returns false. If object is a stream, its promise will be forced.

Scheme Procedure: **stream-pair?** object [¶](07_05_28_srfi41_streams.md)

Returns true if object is a `stream-pair` constructed by `stream-cons`, otherwise returns false. If object is a stream, its promise will be forced.

Scheme Procedure: **stream-car** stream [¶](07_05_28_srfi41_streams.md)

Returns the object stored in the `stream-car` of stream. An error is signaled if the argument is not a `stream-pair`. This causes the object-expr passed to `stream-cons` to be evaluated if it had not yet been; the value is cached in case it is needed again.

Scheme Procedure: **stream-cdr** stream [¶](07_05_28_srfi41_streams.md)

Returns the stream stored in the `stream-cdr` of stream. An error is signaled if the argument is not a `stream-pair`.

Scheme Syntax: **stream-lambda** formals body … [¶](07_05_28_srfi41_streams.md)

Creates a procedure that returns a promise to evaluate the body of the procedure. The last body expression to be evaluated must yield a stream. As with normal `lambda`, formals may be a single variable name, in which case all the formal arguments are collected into a single list, or a list of variable names, which may be null if there are no arguments, proper if there are an exact number of arguments, or dotted if a fixed number of arguments is to be followed by zero or more arguments collected into a list. Body must contain at least one expression, and may contain internal definitions preceding any expressions to be evaluated.

(define strm123
  (stream-cons 1
    (stream-cons 2
      (stream-cons 3
        stream-null))))

(stream-car strm123) ⇒ 1
(stream-car (stream-cdr strm123) ⇒ 2

(stream-pair?
  (stream-cdr
    (stream-cons (/ 1 0) stream-null))) ⇒ #f

(stream? (list 1 2 3)) ⇒ #f

(define iter
  (stream-lambda (f x)
    (stream-cons x (iter f (f x)))))

(define nats (iter (lambda (x) (+ x 1)) 0))

(stream-car (stream-cdr nats)) ⇒ 1

(define stream-add
  (stream-lambda (s1 s2)
    (stream-cons
      (+ (stream-car s1) (stream-car s2))
      (stream-add (stream-cdr s1)
                  (stream-cdr s2)))))

(define evens (stream-add nats nats))

(stream-car evens) ⇒ 0
(stream-car (stream-cdr evens)) ⇒ 2
(stream-car (stream-cdr (stream-cdr evens))) ⇒ 4

* * *

Previous: [SRFI-41 Stream Primitives](07_05_28_srfi41_streams.md#75282-srfi-41-stream-primitives), Up: [SRFI-41 - Streams](07_05_28_srfi41_streams.md#7528-srfi-41---streams)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.28.3 SRFI-41 Stream Library [¶](07_05_28_srfi41_streams.md#75283-srfi-41-stream-library)

Scheme Syntax: **define-stream** (name args …) body … [¶](07_05_28_srfi41_streams.md)

Creates a procedure that returns a stream, and may appear anywhere a normal `define` may appear, including as an internal definition. It may contain internal definitions of its own. The defined procedure takes arguments in the same way as `stream-lambda`. `define-stream` is syntactic sugar on `stream-lambda`; see also `stream-let`, which is also a sugaring of `stream-lambda`.

A simple version of `stream-map` that takes only a single input stream calls itself recursively:

(define-stream (stream-map proc strm)
  (if (stream-null? strm)
      stream-null
      (stream-cons
        (proc (stream-car strm))
        (stream-map proc (stream-cdr strm))))))

Scheme Procedure: **list->stream** list [¶](07_05_28_srfi41_streams.md)

Returns a newly-allocated stream containing the elements from list.

Scheme Procedure: **port->stream** \[port\] [¶](07_05_28_srfi41_streams.md)

Returns a newly-allocated stream containing in its elements the characters on the port. If port is not given it defaults to the current input port. The returned stream has finite length and is terminated by `stream-null`.

It looks like one use of `port->stream` would be this:

(define s ;wrong!
  (with-input-from-file filename
    (lambda () (port->stream))))

But that fails, because `with-input-from-file` is eager, and closes the input port prematurely, before the first character is read. To read a file into a stream, say:

(define-stream (file->stream filename)
  (let ((p (open-input-file filename)))
    (stream-let loop ((c (read-char p)))
      (if (eof-object? c)
          (begin (close-input-port p)
                 stream-null)
          (stream-cons c
            (loop (read-char p)))))))

Scheme Syntax: **stream** object-expr … [¶](07_05_28_srfi41_streams.md)

Creates a newly-allocated stream containing in its elements the objects, in order. The object-exprs are evaluated when they are accessed, not when the stream is created. If no objects are given, as in (stream), the null stream is returned. See also `list->stream`.

(define strm123 (stream 1 2 3))

; (/ 1 0) not evaluated when stream is created
(define s (stream 1 (/ 1 0) -1))

Scheme Procedure: **stream->list** \[n\] stream [¶](07_05_28_srfi41_streams.md)

Returns a newly-allocated list containing in its elements the first n items in stream. If stream has less than n items, all the items in the stream will be included in the returned list. If n is not given it defaults to infinity, which means that unless stream is finite `stream->list` will never return.

(stream->list 10
  (stream-map (lambda (x) (\* x x))
    (stream-from 0)))
  ⇒ (0 1 4 9 16 25 36 49 64 81)

Scheme Procedure: **stream-append** stream … [¶](07_05_28_srfi41_streams.md)

Returns a newly-allocated stream containing in its elements those elements contained in its input streams, in order of input. If any of the input streams is infinite, no elements of any of the succeeding input streams will appear in the output stream. See also `stream-concat`.

Scheme Procedure: **stream-concat** stream [¶](07_05_28_srfi41_streams.md)

Takes a stream consisting of one or more streams and returns a newly-allocated stream containing all the elements of the input streams. If any of the streams in the input stream is infinite, any remaining streams in the input stream will never appear in the output stream. See also `stream-append`.

Scheme Procedure: **stream-constant** object … [¶](07_05_28_srfi41_streams.md)

Returns a newly-allocated stream containing in its elements the objects, repeating in succession forever.

(stream-constant 1) ⇒ 1 1 1 ...
(stream-constant #t #f) ⇒ #t #f #t #f #t #f ...

Scheme Procedure: **stream-drop** n stream [¶](07_05_28_srfi41_streams.md)

Returns the suffix of the input stream that starts at the next element after the first n elements. The output stream shares structure with the input stream; thus, promises forced in one instance of the stream are also forced in the other instance of the stream. If the input stream has less than n elements, `stream-drop` returns the null stream. See also `stream-take`.

Scheme Procedure: **stream-drop-while** pred stream [¶](07_05_28_srfi41_streams.md)

Returns the suffix of the input stream that starts at the first element x for which `(pred x)` returns false. The output stream shares structure with the input stream. See also `stream-take-while`.

Scheme Procedure: **stream-filter** pred stream [¶](07_05_28_srfi41_streams.md)

Returns a newly-allocated stream that contains only those elements x of the input stream which satisfy the predicate `pred`.

(stream-filter odd? (stream-from 0))
   ⇒ 1 3 5 7 9 ...

Scheme Procedure: **stream-fold** proc base stream [¶](07_05_28_srfi41_streams.md)

Applies a binary procedure proc to base and the first element of stream to compute a new base, then applies the procedure to the new base and the next element of stream to compute a succeeding base, and so on, accumulating a value that is finally returned as the value of `stream-fold` when the end of the stream is reached. stream must be finite, or `stream-fold` will enter an infinite loop. See also `stream-scan`, which is similar to `stream-fold`, but useful for infinite streams. For readers familiar with other functional languages, this is a left-fold; there is no corresponding right-fold, since right-fold relies on finite streams that are fully-evaluated, in which case they may as well be converted to a list.

Scheme Procedure: **stream-for-each** proc stream … [¶](07_05_28_srfi41_streams.md)

Applies proc element-wise to corresponding elements of the input streams for side-effects; it returns nothing. `stream-for-each` stops as soon as any of its input streams is exhausted.

Scheme Procedure: **stream-from** first \[step\] [¶](07_05_28_srfi41_streams.md)

Creates a newly-allocated stream that contains first as its first element and increments each succeeding element by step. If step is not given it defaults to 1. first and step may be of any numeric type. `stream-from` is frequently useful as a generator in `stream-of` expressions. See also `stream-range` for a similar procedure that creates finite streams.

Scheme Procedure: **stream-iterate** proc base [¶](07_05_28_srfi41_streams.md)

Creates a newly-allocated stream containing base in its first element and applies proc to each element in turn to determine the succeeding element. See also `stream-unfold` and `stream-unfolds`.

Scheme Procedure: **stream-length** stream [¶](07_05_28_srfi41_streams.md)

Returns the number of elements in the stream; it does not evaluate its elements. `stream-length` may only be used on finite streams; it enters an infinite loop with infinite streams.

Scheme Syntax: **stream-let** tag ((var expr) …) body … [¶](07_05_28_srfi41_streams.md)

Creates a local scope that binds each variable to the value of its corresponding expression. It additionally binds tag to a procedure which takes the bound variables as arguments and body as its defining expressions, binding the tag with `stream-lambda`. tag is in scope within body, and may be called recursively. When the expanded expression defined by the `stream-let` is evaluated, `stream-let` evaluates the expressions in its body in an environment containing the newly-bound variables, returning the value of the last expression evaluated, which must yield a stream.

`stream-let` provides syntactic sugar on `stream-lambda`, in the same manner as normal `let` provides syntactic sugar on normal `lambda`. However, unlike normal `let`, the tag is required, not optional, because unnamed `stream-let` is meaningless.

For example, `stream-member` returns the first `stream-pair` of the input strm with a `stream-car` x that satisfies `(eql? obj x)`, or the null stream if x is not present in strm.

(define-stream (stream-member eql? obj strm)
  (stream-let loop ((strm strm))
    (cond ((stream-null? strm) strm)
          ((eql? obj (stream-car strm)) strm)
          (else (loop (stream-cdr strm))))))

Scheme Procedure: **stream-map** proc stream … [¶](07_05_28_srfi41_streams.md)

Applies proc element-wise to corresponding elements of the input streams, returning a newly-allocated stream containing elements that are the results of those procedure applications. The output stream has as many elements as the minimum-length input stream, and may be infinite.

Scheme Syntax: **stream-match** stream clause … [¶](07_05_28_srfi41_streams.md)

Provides pattern-matching for streams. The input stream is an expression that evaluates to a stream. Clauses are of the form `(pattern [fender] expression)`, consisting of a pattern that matches a stream of a particular shape, an optional fender that must succeed if the pattern is to match, and an expression that is evaluated if the pattern matches. There are four types of patterns:

*   () matches the null stream.
*   (pat0 pat1 …) matches a finite stream with length exactly equal to the number of pattern elements.
*   (pat0 pat1 … `.` pat-rest) matches an infinite stream, or a finite stream with length at least as great as the number of pattern elements before the literal dot.
*   pat matches an entire stream. Should always appear last in the list of clauses; it’s not an error to appear elsewhere, but subsequent clauses could never match.

Each pattern element may be either:

*   An identifier, which matches any stream element. Additionally, the value of the stream element is bound to the variable named by the identifier, which is in scope in the fender and expression of the corresponding clause. Each identifier in a single pattern must be unique.
*   A literal underscore (`_`), which matches any stream element but creates no bindings.

The patterns are tested in order, left-to-right, until a matching pattern is found; if fender is present, it must evaluate to a true value for the match to be successful. Pattern variables are bound in the corresponding fender and expression. Once the matching pattern is found, the corresponding expression is evaluated and returned as the result of the match. An error is signaled if no pattern matches the input stream.

`stream-match` is often used to distinguish null streams from non-null streams, binding head and tail:

(define (len strm)
  (stream-match strm
    (() 0)
    ((head . tail) (+ 1 (len tail)))))

Fenders can test the common case where two stream elements must be identical; the `else` pattern is an identifier bound to the entire stream, not a keyword as in `cond`.

(stream-match strm
  ((x y . \_) (equal? x y) 'ok)
  (else 'error))

A more complex example uses two nested matchers to match two different stream arguments; `(stream-merge lt? . strms)` stably merges two or more streams ordered by the `lt?` predicate:

(define-stream (stream-merge lt? . strms)
  (define-stream (merge xx yy)
    (stream-match xx (() yy) ((x . xs)
      (stream-match yy (() xx) ((y . ys)
        (if (lt? y x)
            (stream-cons y (merge xx ys))
            (stream-cons x (merge xs yy))))))))
  (stream-let loop ((strms strms))
    (cond ((null? strms) stream-null)
          ((null? (cdr strms)) (car strms))
          (else (merge (car strms)
                       (apply stream-merge lt?
                         (cdr strms)))))))

Scheme Syntax: **stream-of** expr clause … [¶](07_05_28_srfi41_streams.md)

Provides the syntax of stream comprehensions, which generate streams by means of looping expressions. The result is a stream of objects of the type returned by expr. There are four types of clauses:

*   (var `in` stream-expr) loops over the elements of stream-expr, in order from the start of the stream, binding each element of the stream in turn to var. `stream-from` and `stream-range` are frequently useful as generators for stream-expr.
*   (var `is` expr) binds var to the value obtained by evaluating expr.
*   (pred expr) includes in the output stream only those elements x which satisfy the predicate pred.

The scope of variables bound in the stream comprehension is the clauses to the right of the binding clause (but not the binding clause itself) plus the result expression.

When two or more generators are present, the loops are processed as if they are nested from left to right; that is, the rightmost generator varies fastest. A consequence of this is that only the first generator may be infinite and all subsequent generators must be finite. If no generators are present, the result of a stream comprehension is a stream containing the result expression; thus, ‘(stream-of 1)’ produces a finite stream containing only the element 1.

(stream-of (\* x x)
  (x in (stream-range 0 10))
  (even? x))
  ⇒ 0 4 16 36 64

(stream-of (list a b)
  (a in (stream-range 1 4))
  (b in (stream-range 1 3)))
  ⇒ (1 1) (1 2) (2 1) (2 2) (3 1) (3 2)

(stream-of (list i j)
  (i in (stream-range 1 5))
  (j in (stream-range (+ i 1) 5)))
  ⇒ (1 2) (1 3) (1 4) (2 3) (2 4) (3 4)

Scheme Procedure: **stream-range** first past \[step\] [¶](07_05_28_srfi41_streams.md)

Creates a newly-allocated stream that contains first as its first element and increments each succeeding element by step. The stream is finite and ends before past, which is not an element of the stream. If step is not given it defaults to 1 if first is less than past and -1 otherwise. first, past and step may be of any real numeric type. `stream-range` is frequently useful as a generator in `stream-of` expressions. See also `stream-from` for a similar procedure that creates infinite streams.

(stream-range 0 10) ⇒ 0 1 2 3 4 5 6 7 8 9
(stream-range 0 10 2) ⇒ 0 2 4 6 8

Successive elements of the stream are calculated by adding step to first, so if any of first, past or step are inexact, the length of the output stream may differ from `(ceiling (- (/ (- past first) step) 1)`.

Scheme Procedure: **stream-ref** stream n [¶](07_05_28_srfi41_streams.md)

Returns the nth element of stream, counting from zero. An error is signaled if n is greater than or equal to the length of stream.

(define (fact n)
  (stream-ref
    (stream-scan \* 1 (stream-from 1))
    n))

Scheme Procedure: **stream-reverse** stream [¶](07_05_28_srfi41_streams.md)

Returns a newly-allocated stream containing the elements of the input stream but in reverse order. `stream-reverse` may only be used with finite streams; it enters an infinite loop with infinite streams. `stream-reverse` does not force evaluation of the elements of the stream.

Scheme Procedure: **stream-scan** proc base stream [¶](07_05_28_srfi41_streams.md)

Accumulates the partial folds of an input stream into a newly-allocated output stream. The output stream is the base followed by `(stream-fold proc base (stream-take i stream))` for each of the first i elements of stream.

(stream-scan + 0 (stream-from 1))
  ⇒ (stream 0 1 3 6 10 15 ...)

(stream-scan \* 1 (stream-from 1))
  ⇒ (stream 1 1 2 6 24 120 ...)

Scheme Procedure: **stream-take** n stream [¶](07_05_28_srfi41_streams.md)

Returns a newly-allocated stream containing the first n elements of the input stream. If the input stream has less than n elements, so does the output stream. See also `stream-drop`.

Scheme Procedure: **stream-take-while** pred stream [¶](07_05_28_srfi41_streams.md)

Takes a predicate and a `stream` and returns a newly-allocated stream containing those elements `x` that form the maximal prefix of the input stream which satisfy pred. See also `stream-drop-while`.

Scheme Procedure: **stream-unfold** map pred gen base [¶](07_05_28_srfi41_streams.md)

The fundamental recursive stream constructor. It constructs a stream by repeatedly applying gen to successive values of base, in the manner of `stream-iterate`, then applying map to each of the values so generated, appending each of the mapped values to the output stream as long as `(pred? base)` returns a true value. See also `stream-iterate` and `stream-unfolds`.

The expression below creates the finite stream ‘0 1 4 9 16 25 36 49 64 81’. Initially the base is 0, which is less than 10, so map squares the base and the mapped value becomes the first element of the output stream. Then gen increments the base by 1, so it becomes 1; this is less than 10, so map squares the new base and 1 becomes the second element of the output stream. And so on, until the base becomes 10, when pred stops the recursion and stream-null ends the output stream.

(stream-unfold
  (lambda (x) (expt x 2)) ; map
  (lambda (x) (< x 10))   ; pred?
  (lambda (x) (+ x 1))    ; gen
  0)                      ; base

Scheme Procedure: **stream-unfolds** proc seed [¶](07_05_28_srfi41_streams.md)

Returns n newly-allocated streams containing those elements produced by successive calls to the generator proc, which takes the current seed as its argument and returns n+1 values

(proc seed) ⇒ seed result\_0 … result\_n-1

where the returned seed is the input seed to the next call to the generator and result\_i indicates how to produce the next element of the ith result stream:

*   (value): value is the next car of the result stream.
*   `#f`: no value produced by this iteration of the generator proc for the result stream.
*   (): the end of the result stream.

It may require multiple calls of proc to produce the next element of any particular result stream. See also `stream-iterate` and `stream-unfold`.

(define (stream-partition pred? strm)
  (stream-unfolds
    (lambda (s)
      (if (stream-null? s)
          (values s '() '())
          (let ((a (stream-car s))
                (d (stream-cdr s)))
            (if (pred? a)
                (values d (list a) #f)
                (values d #f (list a))))))
    strm))

(call-with-values
  (lambda ()
    (stream-partition odd?
      (stream-range 1 6)))
  (lambda (odds evens)
    (list (stream->list odds)
          (stream->list evens))))
  ⇒ ((1 3 5) (2 4))

Scheme Procedure: **stream-zip** stream … [¶](07_05_28_srfi41_streams.md)

Returns a newly-allocated stream in which each element is a list (not a stream) of the corresponding elements of the input streams. The output stream is as long as the shortest input stream, if any of the input streams is finite, or is infinite if all the input streams are infinite.

* * *

Next: [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library), Previous: [SRFI-41 - Streams](07_05_28_srfi41_streams.md#7528-srfi-41---streams), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
