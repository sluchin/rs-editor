#### 7.5.31 SRFI-45 - Primitives for Expressing Iterative Lazy Algorithms [¶](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md#7531-srfi-45---primitives-for-expressing-iterative-lazy-algorithms)

This subsection is based on [the specification of SRFI-45](http://srfi.schemers.org/srfi-45/srfi-45.html) written by André van Tonder.

Lazy evaluation is traditionally simulated in Scheme using `delay` and `force`. However, these primitives are not powerful enough to express a large class of lazy algorithms that are iterative. Indeed, it is folklore in the Scheme community that typical iterative lazy algorithms written using delay and force will often require unbounded memory.

This SRFI provides set of three operations: {`lazy`, `delay`, `force`}, which allow the programmer to succinctly express lazy algorithms while retaining bounded space behavior in cases that are properly tail-recursive. A general recipe for using these primitives is provided. An additional procedure `eager` is provided for the construction of eager promises in cases where efficiency is a concern.

Although this SRFI redefines `delay` and `force`, the extension is conservative in the sense that the semantics of the subset {`delay`, `force`} in isolation (i.e., as long as the program does not use `lazy`) agrees with that in R5RS. In other words, no program that uses the R5RS definitions of delay and force will break if those definition are replaced by the SRFI-45 definitions of delay and force.

Guile also adds `promise?` to the list of exports, which is not part of the official SRFI-45.

Scheme Procedure: **promise?** obj [¶](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md)

Return true if obj is an SRFI-45 promise, otherwise return false.

Scheme Syntax: **delay** expression [¶](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md)

Takes an expression of arbitrary type a and returns a promise of type `(Promise a)` which at some point in the future may be asked (by the `force` procedure) to evaluate the expression and deliver the resulting value.

Scheme Syntax: **lazy** expression [¶](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md)

Takes an expression of type `(Promise a)` and returns a promise of type `(Promise a)` which at some point in the future may be asked (by the `force` procedure) to evaluate the expression and deliver the resulting promise.

Scheme Procedure: **force** expression [¶](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md)

Takes an argument of type `(Promise a)` and returns a value of type a as follows: If a value of type a has been computed for the promise, this value is returned. Otherwise, the promise is first evaluated, then overwritten by the obtained promise or value, and then force is again applied (iteratively) to the promise.

Scheme Procedure: **eager** expression [¶](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md)

Takes an argument of type a and returns a value of type `(Promise a)`. As opposed to `delay`, the argument is evaluated eagerly. Semantically, writing `(eager expression)` is equivalent to writing

(let ((value expression)) (delay value)).

However, the former is more efficient since it does not require unnecessary creation and evaluation of thunks. We also have the equivalence

(delay expression) [\=](06_06_02_numerical_data_types.md) ([lazy](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md) ([eager](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md) expression))

The following reduction rules may be helpful for reasoning about these primitives. However, they do not express the memoization and memory usage semantics specified above:

(force (delay expression)) \-> expression
(force ([lazy](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md)  expression)) \-> (force expression)
(force ([eager](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md) value))      \-> value

#### Correct usage [¶](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md#correct-usage)

We now provide a general recipe for using the primitives {`lazy`, `delay`, `force`} to express lazy algorithms in Scheme. The transformation is best described by way of an example: Consider the stream-filter algorithm, expressed in a hypothetical lazy language as

(define ([stream-filter](07_05_28_srfi41_streams.md) p? s)
  (if ([null?](06_06_09_lists.md) s) '()
      (let ((h ([car](06_06_08_pairs.md) s))
            (t ([cdr](06_06_08_pairs.md) s)))
        (if (p? h)
            ([cons](06_06_08_pairs.md) h ([stream-filter](07_05_28_srfi41_streams.md) p? t))
            ([stream-filter](07_05_28_srfi41_streams.md) p? t)))))

This algorithm can be expressed as follows in Scheme:

(define ([stream-filter](07_05_28_srfi41_streams.md) p? s)
  ([lazy](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md)
     (if ([null?](06_06_09_lists.md) (force s)) (delay '())
         (let ((h ([car](06_06_08_pairs.md) (force s)))
               (t ([cdr](06_06_08_pairs.md) (force s))))
           (if (p? h)
               (delay ([cons](06_06_08_pairs.md) h ([stream-filter](07_05_28_srfi41_streams.md) p? t)))
               ([stream-filter](07_05_28_srfi41_streams.md) p? t))))))

In other words, we

*   wrap all constructors (e.g., `'()`, `cons`) with `delay`,
*   apply `force` to arguments of deconstructors (e.g., `car`, `cdr` and `null?`),
*   wrap procedure bodies with `(lazy ...)`.

* * *

Next: [SRFI-55 - Requiring Features](07_05_33_srfi55_requiring_features.md#7533-srfi-55---requiring-features), Previous: [SRFI-45 - Primitives for Expressing Iterative Lazy Algorithms](07_05_31_srfi45_primitives_for_expressing_iterative_lazy_algorithms.md#7531-srfi-45---primitives-for-expressing-iterative-lazy-algorithms), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
