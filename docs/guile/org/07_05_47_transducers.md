#### 7.5.47 Transducers [¶](07_05_47_transducers.md#7547-transducers)

Some of the most common operations used in the Scheme language are those transforming lists: map, filter, take and so on. They work well, are well understood, and are used daily by most Scheme programmers. They are however not general because they only work on lists, and they do not compose very well since combining N of them builds `(- N 1)` intermediate lists.

Transducers are oblivious to what kind of process they are used in, and are composable without building intermediate collections. This means we can create a transducer that squares all odd numbers:

(compose (tfilter odd?) (tmap (lambda (x) (\* x x))))

and reuse it with lists, vectors, or in just about any context where data flows in one direction. We could use it as a processing step for asynchronous channels, with an event framework as a pre-processing step, or even in lazy contexts where you pass a lazy collection and a transducer to a function and get a new lazy collection back.

The traditional Scheme approach of having collection-specific procedures is not changed. We instead specify a general form of transformations that complement these procedures. The benefits are obvious: a clear, well-understood way of describing common transformations in a way that is faster than just chaining the collection-specific counterparts. For guile in particular this means a lot better GC performance.

Notice however that `(compose …)` composes transducers left-to-right, due to how transducers are initiated.

*   [SRFI-171 General Discussion](07_05_47_transducers.md#75471-srfi-171-general-discussion)
*   [Applying Transducers](07_05_47_transducers.md#75472-applying-transducers)
*   [Reducers](07_05_47_transducers.md#75473-reducers)
*   [Transducers](07_05_47_transducers.md#75474-transducers)
*   [Helper functions for writing transducers](07_05_47_transducers.md#75475-helper-functions-for-writing-transducers)

* * *

Next: [Applying Transducers](07_05_47_transducers.md#75472-applying-transducers), Up: [Transducers](07_05_47_transducers.md#7547-transducers)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.47.1 SRFI-171 General Discussion [¶](07_05_47_transducers.md#75471-srfi-171-general-discussion)

#### The concept of reducers [¶](07_05_47_transducers.md#the-concept-of-reducers)

The central part of transducers are 3-arity reducing procedures.

*   no arguments: Produces the identity of the reducer.
*   (result-so-far): completion. Returns `result-so-far` either with or without transforming it first.
*   (result-so-far input) combines `result-so-far` and `input` to produce a new `result-so-far`.

In the case of a summing `+` reducer, the reducer would produce, in arity order: `0`, `result-so-far`, `(+ result-so-far input)`. This happens to be exactly what the regular `+` does.

#### The concept of transducers [¶](07_05_47_transducers.md#the-concept-of-transducers)

A transducer is a one-arity procedure that takes a reducer and produces a reducing function that behaves as follows:

*   no arguments: calls reducer with no arguments (producing its identity)
*   (result-so-far): Maybe transform the result-so-far and call reducer with it.
*   (result-so-far input) Maybe do something to input and maybe call the reducer with result-so-far and the maybe-transformed input.

A simple example is as following:

(list-transduce (tfilter odd?) + '(1 2 3 4 5))

This first returns a transducer filtering all odd elements, then it runs `+` without arguments to retrieve its identity. It then starts the transduction by passing `+` to the transducer returned by `(tfilter odd?)` which returns a reducing function. It works not unlike reduce from SRFI 1, but also checks whether one of the intermediate transducers returns a "reduced" value (implemented as a SRFI 9 record), which means the reduction finished early.

Because transducers compose and the final reduction is only executed in the last step, composed transducers will not build any intermediate result or collections. Although the normal way of thinking about application of composed functions is right to left, due to how the transduction is built it is applied left to right. `(compose (tfilter odd?) (tmap sqrt))` will create a transducer that first filters out any odd values and then computes the square root of the rest.

#### State [¶](07_05_47_transducers.md#state)

Even though transducers appear to be somewhat of a generalisation of `map` and friends, this is not really true. Since transducers don’t know in which context they are being used, some transducers must keep state where their collection-specific counterparts do not. The transducers that keep state do so using hidden mutable state, and as such all the caveats of mutation, parallelism, and multi-shot continuations apply. Each transducer keeping state is clearly described as doing so in the documentation.

#### Naming [¶](07_05_47_transducers.md#naming)

Reducers exported from the transducers module are named as in their SRFI-1 counterpart, but prepended with an r. Transducers also follow that naming, but are prepended with a t.

* * *

Next: [Reducers](07_05_47_transducers.md#75473-reducers), Previous: [SRFI-171 General Discussion](07_05_47_transducers.md#75471-srfi-171-general-discussion), Up: [Transducers](07_05_47_transducers.md#7547-transducers)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.47.2 Applying Transducers [¶](07_05_47_transducers.md#75472-applying-transducers)

Scheme Procedure: **list-transduce** xform f lst [¶](07_05_47_transducers.md)

Scheme Procedure: **list-transduce** xform f identity lst [¶](07_05_47_transducers.md)

Initialize the transducer xform by passing the reducer f to it. If no identity is provided, f runs without arguments to return the reducer identity. It then reduces over lst using the identity as the seed.

If one of the transducers finishes early (such as `ttake` or `tdrop`), it communicates this by returning a reduced value, which in the guile implementation is just a value wrapped in a SRFI 9 record type named “reduced”. If such a value is returned by the transducer, `list-transduce` must stop execution and return an unreduced value immediately.

Scheme Procedure: **vector-transduce** xform f vec [¶](07_05_47_transducers.md)

Scheme Procedure: **vector-transduce** xform f identity vec [¶](07_05_47_transducers.md)

Scheme Procedure: **string-transduce** xform f str [¶](07_05_47_transducers.md)

Scheme Procedure: **string-transduce** xform f identity str [¶](07_05_47_transducers.md)

Scheme Procedure: **bytevector-u8-transduce** xform f bv [¶](07_05_47_transducers.md)

Scheme Procedure: **bytevector-u8-transduce** xform f identity bv [¶](07_05_47_transducers.md)

Scheme Procedure: **generator-transduce** xform f gen [¶](07_05_47_transducers.md)

Scheme Procedure: **generator-transduce** xform f identity gen [¶](07_05_47_transducers.md)

Same as `list-transduce`, but for vectors, strings, u8-bytevectors and SRFI-158-styled generators respectively.

Scheme Procedure: **port-transduce** xform f reader [¶](07_05_47_transducers.md)

Scheme Procedure: **port-transduce** xform f reader port [¶](07_05_47_transducers.md)

Scheme Procedure: **port-transduce** xform f identity reader port [¶](07_05_47_transducers.md)

Same as `list-transduce` but for ports. Called without a port, it reduces over the results of applying reader until the EOF-object is returned, presumably to read from `current-input-port`. With a port reader is applied to port instead of without any arguments. If identity is provided, that is used as the initial identity in the reduction.

* * *

Next: [Transducers](07_05_47_transducers.md#75474-transducers), Previous: [Applying Transducers](07_05_47_transducers.md#75472-applying-transducers), Up: [Transducers](07_05_47_transducers.md#7547-transducers)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.47.3 Reducers [¶](07_05_47_transducers.md#75473-reducers)

Scheme Procedure: **rcons** [¶](07_05_47_transducers.md)

a simple consing reducer. When called without values, it returns its identity, `'()`. With one value, which will be a list, it reverses the list (using `reverse!`). When called with two values, it conses the second value to the first.

(list-transduce (tmap (lambda (x) (+ x 1)) rcons (list 0 1 2 3))
⇒ (1 2 3 4)

Scheme Procedure: **reverse-rcons** [¶](07_05_47_transducers.md)

same as rcons, but leaves the values in their reversed order.

(list-transduce (tmap (lambda (x) (+ x 1))) reverse-rcons (list 0 1 2 3))
⇒ (4 3 2 1)

Scheme Procedure: **rany** pred? [¶](07_05_47_transducers.md)

The reducer version of any. Returns `(reduced (pred? value))` if any `(pred? value)` returns non-#f. The identity is #f.

(list-transduce (tmap (lambda (x) (+ x 1))) (rany odd?) (list 1 3 5))
⇒ #f

(list-transduce (tmap (lambda (x) (+ x 1))) (rany odd?) (list 1 3 4 5))
⇒ #t

Scheme Procedure: **revery** pred? [¶](07_05_47_transducers.md)

The reducer version of every. Stops the transduction and returns `(reduced #f)` if any `(pred? value)` returns #f. If every `(pred? value)` returns true, it returns the result of the last invocation of `(pred? value)`. The identity is #t.

(list-transduce
  (tmap (lambda (x) (+ x 1)))
  (revery (lambda (v) (if (odd? v) v #f)))
  (list 2 4 6))
  ⇒ 7

(list-transduce (tmap (lambda (x) (+ x 1)) (revery odd?) (list 2 4 5 6))
⇒ #f

Scheme Procedure: **rcount** [¶](07_05_47_transducers.md)

A simple counting reducer. Counts the values that pass through the transduction.

(list-transduce (tfilter odd?) rcount (list 1 2 3 4)) ⇒ 2.

* * *

Next: [Helper functions for writing transducers](07_05_47_transducers.md#75475-helper-functions-for-writing-transducers), Previous: [Reducers](07_05_47_transducers.md#75473-reducers), Up: [Transducers](07_05_47_transducers.md#7547-transducers)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.47.4 Transducers [¶](07_05_47_transducers.md#75474-transducers)

Scheme Procedure: **tmap** proc [¶](07_05_47_transducers.md)

Returns a transducer that applies proc to all values. Stateless.

Scheme Procedure: **tfilter** pred? [¶](07_05_47_transducers.md)

Returns a transducer that removes values for which pred? returns #f.

Stateless.

Scheme Procedure: **tremove** pred? [¶](07_05_47_transducers.md)

Returns a transducer that removes values for which pred? returns non-#f.

Stateless

Scheme Procedure: **tfilter-map** proc [¶](07_05_47_transducers.md)

The same as `(compose (tmap proc) (tfilter values))`. Stateless.

Scheme Procedure: **treplace** mapping [¶](07_05_47_transducers.md)

The argument mapping is an association list (using `equal?` to compare keys), a hash-table, a one-argument procedure taking one argument and either producing that same argument or a replacement value.

Returns a transducer which checks for the presence of any value passed through it in mapping. If a mapping is found, the value of that mapping is returned, otherwise it just returns the original value.

Does not keep internal state, but modifying the mapping while it’s in use by treplace is an error.

Scheme Procedure: **tdrop** n [¶](07_05_47_transducers.md)

Returns a transducer that discards the first n values.

Stateful.

Scheme Procedure: **ttake** n [¶](07_05_47_transducers.md)

Returns a transducer that discards all values and stops the transduction after the first n values have been let through. Any subsequent values are ignored.

Stateful.

Scheme Procedure: **tdrop-while** pred? [¶](07_05_47_transducers.md)

Returns a transducer that discards the first values for which pred? returns true.

Stateful.

Scheme Procedure: **ttake-while** pred? [¶](07_05_47_transducers.md)

Scheme Procedure: **ttake-while** pred? retf [¶](07_05_47_transducers.md)

Returns a transducer that stops the transduction after pred? has returned #f. Any subsequent values are ignored and the last successful value is returned. retf is a function that gets called whenever pred? returns false. The arguments passed are the result so far and the input for which pred? returns `#f`. The default function is `(lambda (result input) result)`.

Stateful.

Scheme Procedure: **tconcatenate** [¶](07_05_47_transducers.md)

tconcatenate _is_ a transducer that concatenates the content of each value (that must be a list) into the reduction.

(list-transduce tconcatenate rcons '((1 2) (3 4 5) (6 (7 8) 9)))
⇒ (1 2 3 4 5 6 (7 8) 9)

Scheme Procedure: **tappend-map** proc [¶](07_05_47_transducers.md)

The same as `(compose (tmap proc) tconcatenate)`.

Scheme Procedure: **tflatten** [¶](07_05_47_transducers.md)

tflatten _is_ a transducer that flattens an input consisting of lists.

(list-transduce tflatten rcons '((1 2) 3 (4 (5 6) 7 8) 9)
⇒ (1 2 3 4 5 6 7 8 9)

Scheme Procedure: **tdelete-neighbor-duplicates** [¶](07_05_47_transducers.md)

Scheme Procedure: **tdelete-neighbor-duplicates** equality-predicate [¶](07_05_47_transducers.md)

Returns a transducer that removes any directly following duplicate elements. The default equality-predicate is `equal?`.

Stateful.

Scheme Procedure: **tdelete-duplicates** [¶](07_05_47_transducers.md)

Scheme Procedure: **tdelete-duplicates** equality-predicate [¶](07_05_47_transducers.md)

Returns a transducer that removes any subsequent duplicate elements compared using equality-predicate. The default equality-predicate is `equal?`.

Stateful.

Scheme Procedure: **tsegment** n [¶](07_05_47_transducers.md)

Returns a transducer that groups inputs into lists of n elements. When the transduction stops, it flushes any remaining collection, even if it contains fewer than n elements.

Stateful.

Scheme Procedure: **tpartition** pred? [¶](07_05_47_transducers.md)

Returns a transducer that groups inputs in lists by whenever `(pred? input)` changes value.

Stateful.

Scheme Procedure: **tadd-between** value [¶](07_05_47_transducers.md)

Returns a transducer which interposes value between each value and the next. This does not compose gracefully with transducers like `ttake`, as you might end up ending the transduction on `value`.

Stateful.

Scheme Procedure: **tenumerate** [¶](07_05_47_transducers.md)

Scheme Procedure: **tenumerate** start [¶](07_05_47_transducers.md)

Returns a transducer that indexes values passed through it, starting at start, which defaults to 0. The indexing is done through cons pairs like `(index . input)`.

(list-transduce (tenumerate 1) rcons (list 'first 'second 'third))
⇒ ((1 . first) (2 . second) (3 . third))

Stateful.

Scheme Procedure: **tlog** [¶](07_05_47_transducers.md)

Scheme Procedure: **tlog** logger [¶](07_05_47_transducers.md)

Returns a transducer that can be used to log or print values and results. The result of the logger procedure is discarded. The default logger is `(lambda (result input) (write input) (newline))`.

Stateless.

#### Guile-specific transducers [¶](07_05_47_transducers.md#guile-specific-transducers)

These transducers are available in the `(srfi srfi-171 gnu)` library, and are provided outside the standard described by the SRFI-171 document.

Scheme Procedure: **tbatch** reducer [¶](07_05_47_transducers.md)

Scheme Procedure: **tbatch** transducer reducer [¶](07_05_47_transducers.md)

A batching transducer that accumulates results using reducer or `((transducer) reducer)` until it returns a reduced value. This can be used to generalize something like `tsegment`:

;; This behaves exactly like (tsegment 4).
(list-transduce (tbatch (ttake 4) rcons) rcons (iota 10))
⇒ ((0 1 2 3) (4 5 6 7) (8 9))

Scheme Procedure: **tfold** reducer [¶](07_05_47_transducers.md)

Scheme Procedure: **tfold** reducer seed [¶](07_05_47_transducers.md)

A folding transducer that yields the result of `(reducer seed value)`, saving its result between iterations.

(list-transduce (tfold +) rcons (iota 10))
⇒ (0 1 3 6 10 15 21 28 36 45)

* * *

Previous: [Transducers](07_05_47_transducers.md#75474-transducers), Up: [Transducers](07_05_47_transducers.md#7547-transducers)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.47.5 Helper functions for writing transducers [¶](07_05_47_transducers.md#75475-helper-functions-for-writing-transducers)

These functions are in the `(srfi srfi-171 meta)` module and are only usable when you want to write your own transducers.

Scheme Procedure: **reduced** value [¶](07_05_47_transducers.md)

Wraps a value in a `<reduced>` container, signaling that the reduction should stop.

Scheme Procedure: **reduced?** value [¶](07_05_47_transducers.md)

Returns #t if value is a `<reduced>` record.

Scheme Procedure: **unreduce** reduced-container [¶](07_05_47_transducers.md)

Returns the value in reduced-container.

Scheme Procedure: **ensure-reduced** value [¶](07_05_47_transducers.md)

Wraps value in a `<reduced>` container if it is not already reduced.

Scheme Procedure: **preserving-reduced** reducer [¶](07_05_47_transducers.md)

Wraps `reducer` in another reducer that encapsulates any returned reduced value in another reduced container. This is useful in places where you re-use a reducer with \[collection\]-reduce. If the reducer returns a reduced value, \[collection\]-reduce unwraps it. Unless handled, this leads to the reduction continuing.

Scheme Procedure: **list-reduce** f identity lst [¶](07_05_47_transducers.md)

The reducing function used internally by `list-transduce`. f is a reducer as returned by a transducer. identity is the identity (sometimes called "seed") of the reduction. lst is a list. If f returns a reduced value, the reduction stops immediately and the unreduced value is returned.

Scheme Procedure: **vector-reduce** f identity vec [¶](07_05_47_transducers.md)

The vector version of list-reduce.

Scheme Procedure: **string-reduce** f identity str [¶](07_05_47_transducers.md)

The string version of list-reduce.

Scheme Procedure: **bytevector-u8-reduce** f identity bv [¶](07_05_47_transducers.md)

The bytevector-u8 version of list-reduce.

Scheme Procedure: **port-reduce** f identity reader port [¶](07_05_47_transducers.md)

The port version of list-reduce. It reduces over port using reader until reader returns the EOF object.

Scheme Procedure: **generator-reduce** f identity gen [¶](07_05_47_transducers.md)

The generator version of list-reduce. It reduces over `gen` until it returns the EOF object

* * *

Next: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors), Previous: [Transducers](07_05_47_transducers.md#7547-transducers), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
