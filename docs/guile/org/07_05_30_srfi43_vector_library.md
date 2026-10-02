#### 7.5.30 SRFI-43 - Vector Library [¶](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library)

This subsection is based on the [specification of SRFI-43](http://srfi.schemers.org/srfi-43/srfi-43.html) by Taylor Campbell.

SRFI-43 implements a comprehensive library of vector operations. It can be made available with:

(use-modules (srfi srfi-43))

*   [SRFI-43 Constructors](07_05_30_srfi43_vector_library.md#75301-srfi-43-constructors)
*   [SRFI-43 Predicates](07_05_30_srfi43_vector_library.md#75302-srfi-43-predicates)
*   [SRFI-43 Selectors](07_05_30_srfi43_vector_library.md#75303-srfi-43-selectors)
*   [SRFI-43 Iteration](07_05_30_srfi43_vector_library.md#75304-srfi-43-iteration)
*   [SRFI-43 Searching](07_05_30_srfi43_vector_library.md#75305-srfi-43-searching)
*   [SRFI-43 Mutators](07_05_30_srfi43_vector_library.md#75306-srfi-43-mutators)
*   [SRFI-43 Conversion](07_05_30_srfi43_vector_library.md#75307-srfi-43-conversion)

* * *

Next: [SRFI-43 Predicates](07_05_30_srfi43_vector_library.md#75302-srfi-43-predicates), Up: [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.30.1 SRFI-43 Constructors [¶](07_05_30_srfi43_vector_library.md#75301-srfi-43-constructors)

Scheme Procedure: **make-vector** size \[fill\] [¶](07_05_30_srfi43_vector_library.md)

Create and return a vector of size size, optionally filling it with fill. The default value of fill is unspecified.

(make-vector 5 3) ⇒ #(3 3 3 3 3)

Scheme Procedure: **vector** x … [¶](07_05_30_srfi43_vector_library.md)

Create and return a vector whose elements are x ....

(vector 0 1 2 3 4) ⇒ #(0 1 2 3 4)

Scheme Procedure: **vector-unfold** f length initial-seed … [¶](07_05_30_srfi43_vector_library.md)

The fundamental vector constructor. Create a vector whose length is length and iterates across each index k from 0 up to length - 1, applying f at each iteration to the current index and current seeds, in that order, to receive n + 1 values: the element to put in the kth slot of the new vector, and n new seeds for the next iteration. It is an error for the number of seeds to vary between iterations.

(vector-unfold (lambda (i x) (values x (- x 1)))
               10 0)
⇒ #(0 -1 -2 -3 -4 -5 -6 -7 -8 -9)

(vector-unfold values 10)
⇒ #(0 1 2 3 4 5 6 7 8 9)

Scheme Procedure: **vector-unfold-right** f length initial-seed … [¶](07_05_30_srfi43_vector_library.md)

Like `vector-unfold`, but it uses f to generate elements from right-to-left, rather than left-to-right.

(vector-unfold-right (lambda (i x) (values x (+ x 1)))
                     10 0)
⇒ #(9 8 7 6 5 4 3 2 1 0)

Scheme Procedure: **vector-copy** vec \[start \[end \[fill\]\]\] [¶](07_05_30_srfi43_vector_library.md)

Allocate a new vector whose length is end - start and fills it with elements from vec, taking elements from vec starting at index start and stopping at index end. start defaults to 0 and end defaults to the value of `(vector-length vec)`. If end extends beyond the length of vec, the slots in the new vector that obviously cannot be filled by elements from vec are filled with fill, whose default value is unspecified.

(vector-copy '#(a b c d e f g h i))
⇒ #(a b c d e f g h i)

(vector-copy '#(a b c d e f g h i) 6)
⇒ #(g h i)

(vector-copy '#(a b c d e f g h i) 3 6)
⇒ #(d e f)

(vector-copy '#(a b c d e f g h i) 6 12 'x)
⇒ #(g h i x x x)

Scheme Procedure: **vector-reverse-copy** vec \[start \[end\]\] [¶](07_05_30_srfi43_vector_library.md)

Like `vector-copy`, but it copies the elements in the reverse order from vec.

(vector-reverse-copy '#(5 4 3 2 1 0) 1 5)
⇒ #(1 2 3 4)

Scheme Procedure: **vector-append** vec … [¶](07_05_30_srfi43_vector_library.md)

Return a newly allocated vector that contains all elements in order from the subsequent locations in vec ....

(vector-append '#(a) '#(b c d))
⇒ #(a b c d)

Scheme Procedure: **vector-concatenate** list-of-vectors [¶](07_05_30_srfi43_vector_library.md)

Append each vector in list-of-vectors. Equivalent to `(apply vector-append list-of-vectors)`.

(vector-concatenate '(#(a b) #(c d)))
⇒ #(a b c d)

* * *

Next: [SRFI-43 Selectors](07_05_30_srfi43_vector_library.md#75303-srfi-43-selectors), Previous: [SRFI-43 Constructors](07_05_30_srfi43_vector_library.md#75301-srfi-43-constructors), Up: [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.30.2 SRFI-43 Predicates [¶](07_05_30_srfi43_vector_library.md#75302-srfi-43-predicates)

Scheme Procedure: **vector?** obj [¶](07_05_30_srfi43_vector_library.md)

Return true if obj is a vector, else return false.

Scheme Procedure: **vector-empty?** vec [¶](07_05_30_srfi43_vector_library.md)

Return true if vec is empty, i.e. its length is 0, else return false.

Scheme Procedure: **vector=** elt=? vec … [¶](07_05_30_srfi43_vector_library.md)

Return true if the vectors vec … have equal lengths and equal elements according to elt=?. elt=? is always applied to two arguments. Element comparison must be consistent with `eq?` in the following sense: if `(eq? a b)` returns true, then `(elt=? a b)` must also return true. The order in which comparisons are performed is unspecified.

* * *

Next: [SRFI-43 Iteration](07_05_30_srfi43_vector_library.md#75304-srfi-43-iteration), Previous: [SRFI-43 Predicates](07_05_30_srfi43_vector_library.md#75302-srfi-43-predicates), Up: [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.30.3 SRFI-43 Selectors [¶](07_05_30_srfi43_vector_library.md#75303-srfi-43-selectors)

Scheme Procedure: **vector-ref** vec i [¶](07_05_30_srfi43_vector_library.md)

Return the element at index i in vec. Indexing is based on zero.

Scheme Procedure: **vector-length** vec [¶](07_05_30_srfi43_vector_library.md)

Return the length of vec.

* * *

Next: [SRFI-43 Searching](07_05_30_srfi43_vector_library.md#75305-srfi-43-searching), Previous: [SRFI-43 Selectors](07_05_30_srfi43_vector_library.md#75303-srfi-43-selectors), Up: [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.30.4 SRFI-43 Iteration [¶](07_05_30_srfi43_vector_library.md#75304-srfi-43-iteration)

Scheme Procedure: **vector-fold** kons knil vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

The fundamental vector iterator. kons is iterated over each index in all of the vectors, stopping at the end of the shortest; kons is applied as

(kons i state ([vector-ref](06_06_10_vectors.md) vec1 i) ([vector-ref](06_06_10_vectors.md) vec2 i) [...](06_08_macros.md))

where state is the current state value, and i is the current index. The current state value begins with knil, and becomes whatever kons returned at the respective iteration. The iteration is strictly left-to-right.

Scheme Procedure: **vector-fold-right** kons knil vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Similar to `vector-fold`, but it iterates right-to-left instead of left-to-right.

Scheme Procedure: **vector-map** f vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Return a new vector of the shortest size of the vector arguments. Each element at index i of the new vector is mapped from the old vectors by

(f i ([vector-ref](06_06_10_vectors.md) vec1 i) ([vector-ref](06_06_10_vectors.md) vec2 i) [...](06_08_macros.md))

The dynamic order of application of f is unspecified.

Scheme Procedure: **vector-map!** f vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Similar to `vector-map`, but rather than mapping the new elements into a new vector, the new mapped elements are destructively inserted into vec1. The dynamic order of application of f is unspecified.

Scheme Procedure: **vector-for-each** f vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Call `(f i (vector-ref vec1 i) (vector-ref vec2 i) ...)` for each index i less than the length of the shortest vector passed. The iteration is strictly left-to-right.

Scheme Procedure: **vector-count** pred? vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Count the number of parallel elements in the vectors that satisfy pred?, which is applied, for each index i less than the length of the smallest vector, to i and each parallel element in the vectors at that index, in order.

(vector-count (lambda (i elt) (even? elt))
              '#(3 1 4 1 5 9 2 5 6))
⇒ 3
(vector-count (lambda (i x y) (< x y))
              '#(1 3 6 9) '#(2 4 6 8 10 12))
⇒ 2

* * *

Next: [SRFI-43 Mutators](07_05_30_srfi43_vector_library.md#75306-srfi-43-mutators), Previous: [SRFI-43 Iteration](07_05_30_srfi43_vector_library.md#75304-srfi-43-iteration), Up: [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.30.5 SRFI-43 Searching [¶](07_05_30_srfi43_vector_library.md#75305-srfi-43-searching)

Scheme Procedure: **vector-index** pred? vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Find and return the index of the first elements in vec1 vec2 … that satisfy pred?. If no matching element is found by the end of the shortest vector, return `#f`.

(vector-index even? '#(3 1 4 1 5 9))
⇒ 2
(vector-index < '#(3 1 4 1 5 9 2 5 6) '#(2 7 1 8 2))
⇒ 1
(vector-index = '#(3 1 4 1 5 9 2 5 6) '#(2 7 1 8 2))
⇒ #f

Scheme Procedure: **vector-index-right** pred? vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Like `vector-index`, but it searches right-to-left, rather than left-to-right. Note that the SRFI 43 specification requires that all the vectors must have the same length, but both the SRFI 43 reference implementation and Guile’s implementation allow vectors with unequal lengths, and start searching from the last index of the shortest vector.

Scheme Procedure: **vector-skip** pred? vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Find and return the index of the first elements in vec1 vec2 … that do not satisfy pred?. If no matching element is found by the end of the shortest vector, return `#f`. Equivalent to `vector-index` but with the predicate inverted.

(vector-skip number? '#(1 2 a b 3 4 c d)) ⇒ 2

Scheme Procedure: **vector-skip-right** pred? vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Like `vector-skip`, but it searches for a non-matching element right-to-left, rather than left-to-right. Note that the SRFI 43 specification requires that all the vectors must have the same length, but both the SRFI 43 reference implementation and Guile’s implementation allow vectors with unequal lengths, and start searching from the last index of the shortest vector.

Scheme Procedure: **vector-binary-search** vec value cmp \[start \[end\]\] [¶](07_05_30_srfi43_vector_library.md)

Find and return an index of vec between start and end whose value is value using a binary search. If no matching element is found, return `#f`. The default start is 0 and the default end is the length of vec.

cmp must be a procedure of two arguments such that `(cmp a b)` returns a negative integer if _a < b_, a positive integer if _a > b_, or zero if _a = b_. The elements of vec must be sorted in non-decreasing order according to cmp.

Note that SRFI 43 does not document the start and end arguments, but both its reference implementation and Guile’s implementation support them.

(define (char-cmp c1 c2)
  (cond ((char<? c1 c2) -1)
        ((char>? c1 c2) 1)
        (else 0)))

(vector-binary-search '#(#\\a #\\b #\\c #\\d #\\e #\\f #\\g #\\h)
                      #\\g
                      char-cmp)
⇒ 6

Scheme Procedure: **vector-any** pred? vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

Find the first parallel set of elements from vec1 vec2 … for which pred? returns a true value. If such a parallel set of elements exists, `vector-any` returns the value that pred? returned for that set of elements. The iteration is strictly left-to-right.

Scheme Procedure: **vector-every** pred? vec1 vec2 … [¶](07_05_30_srfi43_vector_library.md)

If, for every index i between 0 and the length of the shortest vector argument, the set of elements `(vector-ref vec1 i)` `(vector-ref vec2 i)` … satisfies pred?, `vector-every` returns the value that pred? returned for the last set of elements, at the last index of the shortest vector. Otherwise it returns `#f`. The iteration is strictly left-to-right.

* * *

Next: [SRFI-43 Conversion](07_05_30_srfi43_vector_library.md#75307-srfi-43-conversion), Previous: [SRFI-43 Searching](07_05_30_srfi43_vector_library.md#75305-srfi-43-searching), Up: [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.30.6 SRFI-43 Mutators [¶](07_05_30_srfi43_vector_library.md#75306-srfi-43-mutators)

Scheme Procedure: **vector-set!** vec i value [¶](07_05_30_srfi43_vector_library.md)

Assign the contents of the location at i in vec to value.

Scheme Procedure: **vector-swap!** vec i j [¶](07_05_30_srfi43_vector_library.md)

Swap the values of the locations in vec at i and j.

Scheme Procedure: **vector-fill!** vec fill \[start \[end\]\] [¶](07_05_30_srfi43_vector_library.md)

Assign the value of every location in vec between start and end to fill. start defaults to 0 and end defaults to the length of vec.

Scheme Procedure: **vector-reverse!** vec \[start \[end\]\] [¶](07_05_30_srfi43_vector_library.md)

Destructively reverse the contents of vec between start and end. start defaults to 0 and end defaults to the length of vec.

Scheme Procedure: **vector-copy!** target tstart source \[sstart \[send\]\] [¶](07_05_30_srfi43_vector_library.md)

Copy a block of elements from source to target, both of which must be vectors, starting in target at tstart and starting in source at sstart, ending when (send - sstart) elements have been copied. It is an error for target to have a length less than (tstart + send - sstart). sstart defaults to 0 and send defaults to the length of source.

Scheme Procedure: **vector-reverse-copy!** target tstart source \[sstart \[send\]\] [¶](07_05_30_srfi43_vector_library.md)

Like `vector-copy!`, but this copies the elements in the reverse order. It is an error if target and source are identical vectors and the target and source ranges overlap; however, if tstart = sstart, `vector-reverse-copy!` behaves as `(vector-reverse! target tstart send)` would.

* * *

Previous: [SRFI-43 Mutators](07_05_30_srfi43_vector_library.md#75306-srfi-43-mutators), Up: [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.30.7 SRFI-43 Conversion [¶](07_05_30_srfi43_vector_library.md#75307-srfi-43-conversion)

Scheme Procedure: **vector->list** vec \[start \[end\]\] [¶](07_05_30_srfi43_vector_library.md)

Return a newly allocated list containing the elements in vec between start and end. start defaults to 0 and end defaults to the length of vec.

Scheme Procedure: **reverse-vector->list** vec \[start \[end\]\] [¶](07_05_30_srfi43_vector_library.md)

Like `vector->list`, but the resulting list contains the specified range of elements of vec in reverse order.

Scheme Procedure: **list->vector** proper-list \[start \[end\]\] [¶](07_05_30_srfi43_vector_library.md)

Return a newly allocated vector of the elements from proper-list with indices between start and end. start defaults to 0 and end defaults to the length of proper-list. Note that SRFI 43 does not document the start and end arguments, but both its reference implementation and Guile’s implementation support them.

Scheme Procedure: **reverse-list->vector** proper-list \[start \[end\]\] [¶](07_05_30_srfi43_vector_library.md)

Like `list->vector`, but the resulting vector contains the specified range of elements of proper-list in reverse order. Note that SRFI 43 does not document the start and end arguments, but both its reference implementation and Guile’s implementation support them.

* * *

Next: [SRFI-46 Basic syntax-rules Extensions](07_05_32_srfi46_basic_syntaxrules_extensions.md#7532-srfi-46-basic-syntax-rules-extensions), Previous: [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
