#### 7.5.3 SRFI-1 - List library [¶](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)

The list library defined in SRFI-1 contains a lot of useful list processing procedures for construction, examining, destructuring and manipulating lists and pairs.

Since SRFI-1 also defines some procedures which are already contained in R5RS and thus are supported by the Guile core library, some list and pair procedures which appear in the SRFI-1 document may not appear in this section. So when looking for a particular list/pair processing procedure, you should also have a look at the sections [Lists](06_06_09_lists.md#669-lists) and [Pairs](06_06_08_pairs.md#668-pairs).

*   [Constructors](07_05_03_srfi1_list_library.md#7531-constructors)
*   [Predicates](07_05_03_srfi1_list_library.md#7532-predicates)
*   [Selectors](07_05_03_srfi1_list_library.md#7533-selectors)
*   [Length, Append, Concatenate, etc.](07_05_03_srfi1_list_library.md#7534-length-append-concatenate-etc)
*   [Fold, Unfold & Map](07_05_03_srfi1_list_library.md#7535-fold-unfold--map)
*   [Filtering and Partitioning](07_05_03_srfi1_list_library.md#7536-filtering-and-partitioning)
*   [Searching](07_05_03_srfi1_list_library.md#7537-searching)
*   [Deleting](07_05_03_srfi1_list_library.md#7538-deleting)
*   [Association Lists](07_05_03_srfi1_list_library.md#7539-association-lists)
*   [Set Operations on Lists](07_05_03_srfi1_list_library.md#75310-set-operations-on-lists)

* * *

Next: [Predicates](07_05_03_srfi1_list_library.md#7532-predicates), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.1 Constructors [¶](07_05_03_srfi1_list_library.md#7531-constructors)

New lists can be constructed by calling one of the following procedures.

Scheme Procedure: **xcons** d a [¶](07_05_03_srfi1_list_library.md)

Like `cons`, but with interchanged arguments. Useful mostly when passed to higher-order procedures.

Scheme Procedure: **list-tabulate** n init-proc [¶](07_05_03_srfi1_list_library.md)

Return an n\-element list, where each list element is produced by applying the procedure init-proc to the corresponding list index. The order in which init-proc is applied to the indices is not specified.

Scheme Procedure: **list-copy** lst [¶](07_05_03_srfi1_list_library.md)

Return a new list containing the elements of the list lst.

This function differs from the core `list-copy` (see [List Constructors](06_06_09_lists.md#6693-list-constructors)) in accepting improper lists too. And if lst is not a pair at all then it’s treated as the final tail of an improper list and simply returned.

Scheme Procedure: **circular-list** elt1 elt2 … [¶](07_05_03_srfi1_list_library.md)

Return a circular list containing the given arguments elt1 elt2 ….

Scheme Procedure: **iota** count \[start step\] [¶](07_05_03_srfi1_list_library.md)

Return a list containing count numbers, starting from start and adding step each time. The default start is 0, the default step is 1. For example,

(iota 6)        ⇒ (0 1 2 3 4 5)
(iota 4 2.5 -2) ⇒ (2.5 0.5 -1.5 -3.5)

This function takes its name from the corresponding primitive in the APL language.

* * *

Next: [Selectors](07_05_03_srfi1_list_library.md#7533-selectors), Previous: [Constructors](07_05_03_srfi1_list_library.md#7531-constructors), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.2 Predicates [¶](07_05_03_srfi1_list_library.md#7532-predicates)

The procedures in this section test specific properties of lists.

Scheme Procedure: **proper-list?** obj [¶](07_05_03_srfi1_list_library.md)

Return `#t` if obj is a proper list, or `#f` otherwise. This is the same as the core `list?` (see [List Predicates](06_06_09_lists.md#6692-list-predicates)).

A proper list is a list which ends with the empty list `()` in the usual way. The empty list `()` itself is a proper list too.

(proper-list? '(1 2 3))  ⇒ #t
(proper-list? '())       ⇒ #t

Scheme Procedure: **circular-list?** obj [¶](07_05_03_srfi1_list_library.md)

Return `#t` if obj is a circular list, or `#f` otherwise.

A circular list is a list where at some point the `cdr` refers back to a previous pair in the list (either the start or some later point), so that following the `cdr`s takes you around in a circle, with no end.

(define x (list 1 2 3 4))
(set-cdr! (last-pair x) (cddr x))
x ⇒ (1 2 3 4 3 4 3 4 ...)
(circular-list? x)  ⇒ #t

Scheme Procedure: **dotted-list?** obj [¶](07_05_03_srfi1_list_library.md)

Return `#t` if obj is a dotted list, or `#f` otherwise.

A dotted list is a list where the `cdr` of the last pair is not the empty list `()`. Any non-pair obj is also considered a dotted list, with length zero.

(dotted-list? '(1 2 . 3))  ⇒ #t
(dotted-list? 99)          ⇒ #t

It will be noted that any Scheme object passes exactly one of the above three tests `proper-list?`, `circular-list?` and `dotted-list?`. Non-lists are `dotted-list?`, finite lists are either `proper-list?` or `dotted-list?`, and infinite lists are `circular-list?`.

  

Scheme Procedure: **null-list?** lst [¶](07_05_03_srfi1_list_library.md)

Return `#t` if lst is the empty list `()`, `#f` otherwise. If something else than a proper or circular list is passed as lst, an error is signaled. This procedure is recommended for checking for the end of a list in contexts where dotted lists are not allowed.

Scheme Procedure: **not-pair?** obj [¶](07_05_03_srfi1_list_library.md)

Return `#t` is obj is not a pair, `#f` otherwise. This is shorthand notation `(not (pair? obj))` and is supposed to be used for end-of-list checking in contexts where dotted lists are allowed.

Scheme Procedure: **list=** elt= list1 … [¶](07_05_03_srfi1_list_library.md)

Return `#t` if all argument lists are equal, `#f` otherwise. List equality is determined by testing whether all lists have the same length and the corresponding elements are equal in the sense of the equality predicate elt=. If no or only one list is given, `#t` is returned.

* * *

Next: [Length, Append, Concatenate, etc.](07_05_03_srfi1_list_library.md#7534-length-append-concatenate-etc), Previous: [Predicates](07_05_03_srfi1_list_library.md#7532-predicates), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.3 Selectors [¶](07_05_03_srfi1_list_library.md#7533-selectors)

Scheme Procedure: **first** pair [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **second** pair [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **third** pair [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **fourth** pair [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **fifth** pair [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **sixth** pair [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **seventh** pair [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **eighth** pair [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **ninth** pair [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **tenth** pair [¶](07_05_03_srfi1_list_library.md)

These are synonyms for `car`, `cadr`, `caddr`, ….

Scheme Procedure: **car+cdr** pair [¶](07_05_03_srfi1_list_library.md)

Return two values, the CAR and the CDR of pair.

([car+cdr](07_05_03_srfi1_list_library.md) '(0 1 2 3))
⇒
0
(1 2 3)

Scheme Procedure: **take** lst i [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **take!** lst i [¶](07_05_03_srfi1_list_library.md)

Return a list containing the first i elements of lst.

`take!` may modify the structure of the argument list lst in order to produce the result.

Scheme Procedure: **drop** lst i [¶](07_05_03_srfi1_list_library.md)

Return a list containing all but the first i elements of lst.

Scheme Procedure: **take-right** lst i [¶](07_05_03_srfi1_list_library.md)

Return a list containing the i last elements of lst. The return shares a common tail with lst.

Scheme Procedure: **drop-right** lst i [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **drop-right!** lst i [¶](07_05_03_srfi1_list_library.md)

Return a list containing all but the i last elements of lst.

`drop-right` always returns a new list, even when i is zero. `drop-right!` may modify the structure of the argument list lst in order to produce the result.

Scheme Procedure: **split-at** lst i [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **split-at!** lst i [¶](07_05_03_srfi1_list_library.md)

Return two values, a list containing the first i elements of the list lst and a list containing the remaining elements.

`split-at!` may modify the structure of the argument list lst in order to produce the result.

Scheme Procedure: **last** lst [¶](07_05_03_srfi1_list_library.md)

Return the last element of the non-empty, finite list lst.

* * *

Next: [Fold, Unfold & Map](07_05_03_srfi1_list_library.md#7535-fold-unfold--map), Previous: [Selectors](07_05_03_srfi1_list_library.md#7533-selectors), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.4 Length, Append, Concatenate, etc. [¶](07_05_03_srfi1_list_library.md#7534-length-append-concatenate-etc)

Scheme Procedure: **length+** lst [¶](07_05_03_srfi1_list_library.md)

Return the length of the argument list lst. When lst is a circular list, `#f` is returned.

Scheme Procedure: **concatenate** list-of-lists [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **concatenate!** list-of-lists [¶](07_05_03_srfi1_list_library.md)

Construct a list by appending all lists in list-of-lists.

`concatenate!` may modify the structure of the given lists in order to produce the result.

`concatenate` is the same as `(apply append list-of-lists)`. It exists because some Scheme implementations have a limit on the number of arguments a function takes, which the `apply` might exceed. In Guile there is no such limit.

Scheme Procedure: **append-reverse** rev-head tail [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **append-reverse!** rev-head tail [¶](07_05_03_srfi1_list_library.md)

Reverse rev-head, append tail to it, and return the result. This is equivalent to `(append (reverse rev-head) tail)`, but its implementation is more efficient.

(append-reverse '(1 2 3) '(4 5 6)) ⇒ (3 2 1 4 5 6)

`append-reverse!` may modify rev-head in order to produce the result.

Scheme Procedure: **zip** lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Return a list as long as the shortest of the argument lists, where each element is a list. The first list contains the first elements of the argument lists, the second list contains the second elements, and so on.

Scheme Procedure: **unzip1** lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **unzip2** lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **unzip3** lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **unzip4** lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **unzip5** lst [¶](07_05_03_srfi1_list_library.md)

`unzip1` takes a list of lists, and returns a list containing the first elements of each list, `unzip2` returns two lists, the first containing the first elements of each lists and the second containing the second elements of each lists, and so on.

Scheme Procedure: **count** pred lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Return a count of the number of times pred returns true when called on elements from the given lists.

pred is called with N parameters `(pred elem1 … elemN )`, each element being from the corresponding list. The first call is with the first element of each list, the second with the second element from each, and so on.

Counting stops when the end of the shortest list is reached. At least one list must be non-circular.

* * *

Next: [Filtering and Partitioning](07_05_03_srfi1_list_library.md#7536-filtering-and-partitioning), Previous: [Length, Append, Concatenate, etc.](07_05_03_srfi1_list_library.md#7534-length-append-concatenate-etc), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.5 Fold, Unfold & Map [¶](07_05_03_srfi1_list_library.md#7535-fold-unfold--map)

Scheme Procedure: **fold** proc init lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **fold-right** proc init lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Apply proc to the elements of lst1 lst2 … to build a result, and return that result.

Each proc call is `(proc elem1 elem2 … previous)`, where elem1 is from lst1, elem2 is from lst2, and so on. previous is the return from the previous call to proc, or the given init for the first call. If any list is empty, just init is returned.

`fold` works through the list elements from first to last. The following shows a list reversal and the calls it makes,

(fold cons '() '(1 2 3))

(cons 1 '())
(cons 2 '(1))
(cons 3 '(2 1)
⇒ (3 2 1)

`fold-right` works through the list elements from last to first, ie. from the right. So for example the following finds the longest string, and the last among equal longest,

(fold-right (lambda (str prev)
              (if (> (string-length str) (string-length prev))
                  str
                  prev))
            ""
            '("x" "abc" "xyz" "jk"))
⇒ "xyz"

If lst1 lst2 … have different lengths, `fold` stops when the end of the shortest is reached; `fold-right` commences at the last element of the shortest. Ie. elements past the length of the shortest are ignored in the other lsts. At least one lst must be non-circular.

`fold` should be preferred over `fold-right` if the order of processing doesn’t matter, or can be arranged either way, since `fold` is a little more efficient.

The way `fold` builds a result from iterating is quite general, it can do more than other iterations like say `map` or `filter`. The following for example removes adjacent duplicate elements from a list,

(define (delete-adjacent-duplicates lst)
  (fold-right (lambda (elem ret)
                (if (equal? elem (first ret))
                    ret
                    (cons elem ret)))
              (list (last lst))
              lst))
(delete-adjacent-duplicates '(1 2 3 3 4 4 4 5))
⇒ (1 2 3 4 5)

Clearly the same sort of thing can be done with a `for-each` and a variable in which to build the result, but a self-contained proc can be re-used in multiple contexts, where a `for-each` would have to be written out each time.

Scheme Procedure: **pair-fold** proc init lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **pair-fold-right** proc init lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

The same as `fold` and `fold-right`, but apply proc to the pairs of the lists instead of the list elements.

Scheme Procedure: **reduce** proc default lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **reduce-right** proc default lst [¶](07_05_03_srfi1_list_library.md)

`reduce` is a variant of `fold`, where the first call to proc is on two elements from lst, rather than one element and a given initial value.

If lst is empty, `reduce` returns default (this is the only use for default). If lst has just one element then that’s the return value. Otherwise proc is called on the elements of lst.

Each proc call is `(proc elem previous)`, where elem is from lst (the second and subsequent elements of lst), and previous is the return from the previous call to proc. The first element of lst is the previous for the first call to proc.

For example, the following adds a list of numbers, the calls made to `+` are shown. (Of course `+` accepts multiple arguments and can add a list directly, with `apply`.)

(reduce + 0 '(5 6 7)) ⇒ 18

(+ 6 5)  ⇒ 11
(+ 7 11) ⇒ 18

`reduce` can be used instead of `fold` where the init value is an “identity”, meaning a value which under proc doesn’t change the result, in this case 0 is an identity since `(+ 5 0)` is just 5. `reduce` avoids that unnecessary call.

`reduce-right` is a similar variation on `fold-right`, working from the end (ie. the right) of lst. The last element of lst is the previous for the first call to proc, and the elem values go from the second last.

`reduce` should be preferred over `reduce-right` if the order of processing doesn’t matter, or can be arranged either way, since `reduce` is a little more efficient.

Scheme Procedure: **unfold** p f g seed \[tail-gen\] [¶](07_05_03_srfi1_list_library.md)

`unfold` is defined as follows:

([unfold](07_05_03_srfi1_list_library.md) p f g seed) [\=](06_06_02_numerical_data_types.md)
   (if (p seed) (tail-gen seed)
       ([cons](06_06_08_pairs.md) (f seed)
             ([unfold](07_05_03_srfi1_list_library.md) p f g (g seed))))

p

Determines when to stop unfolding.

f

Maps each seed value to the corresponding list element.

g

Maps each seed value to next seed value.

seed

The state value for the unfold.

tail-gen

Creates the tail of the list; defaults to `(lambda (x) '())`.

g produces a series of seed values, which are mapped to list elements by f. These elements are put into a list in left-to-right order, and p tells when to stop unfolding.

Scheme Procedure: **unfold-right** p f g seed \[tail\] [¶](07_05_03_srfi1_list_library.md)

Construct a list with the following loop.

(let lp ((seed seed) (lis tail))
   (if (p seed) lis
       (lp (g seed)
           ([cons](06_06_08_pairs.md) (f seed) lis))))

p

Determines when to stop unfolding.

f

Maps each seed value to the corresponding list element.

g

Maps each seed value to next seed value.

seed

The state value for the unfold.

tail

The tail of the list; defaults to `'()`.

Scheme Procedure: **map** f lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Map the procedure over the list(s) lst1, lst2, … and return a list containing the results of the procedure applications. This procedure is extended with respect to R5RS, because the argument lists may have different lengths. The result list will have the same length as the shortest argument lists. The order in which f will be applied to the list element(s) is not specified.

Scheme Procedure: **for-each** f lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Apply the procedure f to each pair of corresponding elements of the list(s) lst1, lst2, …. The return value is not specified. This procedure is extended with respect to R5RS, because the argument lists may have different lengths. The shortest argument list determines the number of times f is called. f will be applied to the list elements in left-to-right order.

Scheme Procedure: **append-map** f lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **append-map!** f lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Equivalent to

([apply](06_16_reading_and_evaluating_scheme_code.md) [append](06_06_09_lists.md) (map f clist1 clist2 [...](06_08_macros.md)))

and

([apply](06_16_reading_and_evaluating_scheme_code.md) [append!](06_06_09_lists.md) (map f clist1 clist2 [...](06_08_macros.md)))

Map f over the elements of the lists, just as in the `map` function. However, the results of the applications are appended together to make the final result. `append-map` uses `append` to append the results together; `append-map!` uses `append!`.

The dynamic order in which the various applications of f are made is not specified.

Scheme Procedure: **map!** f lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Linear-update variant of `map` – `map!` is allowed, but not required, to alter the cons cells of lst1 to construct the result list.

The dynamic order in which the various applications of f are made is not specified. In the n-ary case, lst2, lst3, … must have at least as many elements as lst1.

Scheme Procedure: **pair-for-each** f lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Like `for-each`, but applies the procedure f to the pairs from which the argument lists are constructed, instead of the list elements. The return value is not specified.

Scheme Procedure: **filter-map** f lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Like `map`, but only results from the applications of f which are true are saved in the result list.

* * *

Next: [Searching](07_05_03_srfi1_list_library.md#7537-searching), Previous: [Fold, Unfold & Map](07_05_03_srfi1_list_library.md#7535-fold-unfold--map), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.6 Filtering and Partitioning [¶](07_05_03_srfi1_list_library.md#7536-filtering-and-partitioning)

Filtering means to collect all elements from a list which satisfy a specific condition. Partitioning a list means to make two groups of list elements, one which contains the elements satisfying a condition, and the other for the elements which don’t.

The `filter` and `filter!` functions are implemented in the Guile core, See [List Modification](06_06_09_lists.md#6696-list-modification).

Scheme Procedure: **partition** pred lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **partition!** pred lst [¶](07_05_03_srfi1_list_library.md)

Split lst into those elements which do and don’t satisfy the predicate pred.

The return is two values (see [Returning and Accepting Multiple Values](06_11_controlling_the_flow_of_program_execution.md#6117-returning-and-accepting-multiple-values)), the first being a list of all elements from lst which satisfy pred, the second a list of those which do not.

The elements in the result lists are in the same order as in lst but the order in which the calls `(pred elem)` are made on the list elements is unspecified.

`partition` does not change lst, but one of the returned lists may share a tail with it. `partition!` may modify lst to construct its return.

Scheme Procedure: **remove** pred lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **remove!** pred lst [¶](07_05_03_srfi1_list_library.md)

Return a list containing all elements from lst which do not satisfy the predicate pred. The elements in the result list have the same order as in lst. The order in which pred is applied to the list elements is not specified.

`remove!` is allowed, but not required to modify the structure of the input list.

* * *

Next: [Deleting](07_05_03_srfi1_list_library.md#7538-deleting), Previous: [Filtering and Partitioning](07_05_03_srfi1_list_library.md#7536-filtering-and-partitioning), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.7 Searching [¶](07_05_03_srfi1_list_library.md#7537-searching)

The procedures for searching elements in lists either accept a predicate or a comparison object for determining which elements are to be searched.

Scheme Procedure: **find** pred lst [¶](07_05_03_srfi1_list_library.md)

Return the first element of lst that satisfies the predicate pred and `#f` if no such element is found.

Scheme Procedure: **find-tail** pred lst [¶](07_05_03_srfi1_list_library.md)

Return the first pair of lst whose CAR satisfies the predicate pred and `#f` if no such element is found.

Scheme Procedure: **take-while** pred lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **take-while!** pred lst [¶](07_05_03_srfi1_list_library.md)

Return the longest initial prefix of lst whose elements all satisfy the predicate pred.

`take-while!` is allowed, but not required to modify the input list while producing the result.

Scheme Procedure: **drop-while** pred lst [¶](07_05_03_srfi1_list_library.md)

Drop the longest initial prefix of lst whose elements all satisfy the predicate pred.

Scheme Procedure: **span** pred lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **span!** pred lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **break** pred lst [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **break!** pred lst [¶](07_05_03_srfi1_list_library.md)

`span` splits the list lst into the longest initial prefix whose elements all satisfy the predicate pred, and the remaining tail. `break` inverts the sense of the predicate.

`span!` and `break!` are allowed, but not required to modify the structure of the input list lst in order to produce the result.

Note that the name `break` conflicts with the `break` binding established by `while` (see [Iteration mechanisms](06_11_controlling_the_flow_of_program_execution.md#6114-iteration-mechanisms)). Applications wanting to use `break` from within a `while` loop will need to make a new define under a different name.

Scheme Procedure: **any** pred lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Test whether any set of elements from lst1 lst2 … satisfies pred. If so, the return value is the return value from the successful pred call, or if not, the return value is `#f`.

If there are n list arguments, then pred must be a predicate taking n arguments. Each pred call is `(pred elem1 elem2 … )` taking an element from each lst. The calls are made successively for the first, second, etc. elements of the lists, stopping when pred returns non-`#f`, or when the end of the shortest list is reached.

The pred call on the last set of elements (i.e., when the end of the shortest list has been reached), if that point is reached, is a tail call.

Scheme Procedure: **every** pred lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Test whether every set of elements from lst1 lst2 … satisfies pred. If so, the return value is the return from the final pred call, or if not, the return value is `#f`.

If there are n list arguments, then pred must be a predicate taking n arguments. Each pred call is `(pred elem1 elem2 …)` taking an element from each lst. The calls are made successively for the first, second, etc. elements of the lists, stopping if pred returns `#f`, or when the end of any of the lists is reached.

The pred call on the last set of elements (i.e., when the end of the shortest list has been reached) is a tail call.

If one of lst1 lst2 …is empty then no calls to pred are made, and the return value is `#t`.

Scheme Procedure: **list-index** pred lst1 lst2 … [¶](07_05_03_srfi1_list_library.md)

Return the index of the first set of elements, one from each of lst1 lst2 …, which satisfies pred.

pred is called as `(elem1 elem2 …)`. Searching stops when the end of the shortest lst is reached. The return index starts from 0 for the first set of elements. If no set of elements pass, then the return value is `#f`.

(list-index odd? '(2 4 6 9))      ⇒ 3
(list-index = '(1 2 3) '(3 1 2))  ⇒ #f

Scheme Procedure: **member** x lst \[=\] [¶](07_05_03_srfi1_list_library.md)

Return the first sublist of lst whose CAR is equal to x. If x does not appear in lst, return `#f`.

Equality is determined by `equal?`, or by the equality predicate \= if given. \= is called `(= x elem)`, ie. with the given x first, so for example to find the first element greater than 5,

(member 5 '(3 5 1 7 2 9) <) ⇒ (7 2 9)

This version of `member` extends the core `member` (see [List Searching](06_06_09_lists.md#6697-list-searching)) by accepting an equality predicate.

* * *

Next: [Association Lists](07_05_03_srfi1_list_library.md#7539-association-lists), Previous: [Searching](07_05_03_srfi1_list_library.md#7537-searching), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.8 Deleting [¶](07_05_03_srfi1_list_library.md#7538-deleting)

Scheme Procedure: **delete** x lst \[=\] [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **delete!** x lst \[=\] [¶](07_05_03_srfi1_list_library.md)

Return a list containing the elements of lst but with those equal to x deleted. The returned elements will be in the same order as they were in lst.

Equality is determined by the \= predicate, or `equal?` if not given. An equality call is made just once for each element, but the order in which the calls are made on the elements is unspecified.

The equality calls are always `(= x elem)`, ie. the given x is first. This means for instance elements greater than 5 can be deleted with `(delete 5 lst <)`.

`delete` does not modify lst, but the return might share a common tail with lst. `delete!` may modify the structure of lst to construct its return.

These functions extend the core `delete` and `delete!` (see [List Modification](06_06_09_lists.md#6696-list-modification)) in accepting an equality predicate. See also `lset-difference` (see [Set Operations on Lists](07_05_03_srfi1_list_library.md#75310-set-operations-on-lists)) for deleting multiple elements from a list.

Scheme Procedure: **delete-duplicates** lst \[=\] [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **delete-duplicates!** lst \[=\] [¶](07_05_03_srfi1_list_library.md)

Return a list containing the elements of lst but without duplicates.

When elements are equal, only the first in lst is retained. Equal elements can be anywhere in lst, they don’t have to be adjacent. The returned list will have the retained elements in the same order as they were in lst.

Equality is determined by the \= predicate, or `equal?` if not given. Calls `(= x y)` are made with element x being before y in lst. A call is made at most once for each combination, but the sequence of the calls across the elements is unspecified.

`delete-duplicates` does not modify lst, but the return might share a common tail with lst. `delete-duplicates!` may modify the structure of lst to construct its return.

In the worst case, this is an _O(N^2)_ algorithm because it must check each element against all those preceding it. For long lists it is more efficient to sort and then compare only adjacent elements.

* * *

Next: [Set Operations on Lists](07_05_03_srfi1_list_library.md#75310-set-operations-on-lists), Previous: [Deleting](07_05_03_srfi1_list_library.md#7538-deleting), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.9 Association Lists [¶](07_05_03_srfi1_list_library.md#7539-association-lists)

Association lists are described in detail in section [Association Lists](06_06_20_association_lists.md#6620-association-lists). The present section only documents the additional procedures for dealing with association lists defined by SRFI-1.

Scheme Procedure: **assoc** key alist \[=\] [¶](07_05_03_srfi1_list_library.md)

Return the pair from alist which matches key. This extends the core `assoc` (see [Retrieving Alist Entries](06_06_20_association_lists.md#66203-retrieving-alist-entries)) by taking an optional \= comparison procedure.

The default comparison is `equal?`. If an \= parameter is given it’s called `(= key alistcar)`, i.e. the given target key is the first argument, and a `car` from alist is second.

For example a case-insensitive string lookup,

(assoc "yy" '(("XX" . 1) ("YY" . 2)) string-ci=?)
⇒ ("YY" . 2)

Scheme Procedure: **alist-cons** key datum alist [¶](07_05_03_srfi1_list_library.md)

Cons a new association key and datum onto alist and return the result. This is equivalent to

([cons](06_06_08_pairs.md) ([cons](06_06_08_pairs.md) key datum) alist)

`acons` (see [Adding or Setting Alist Entries](06_06_20_association_lists.md#66202-adding-or-setting-alist-entries)) in the Guile core does the same thing.

Scheme Procedure: **alist-copy** alist [¶](07_05_03_srfi1_list_library.md)

Return a newly allocated copy of alist, that means that the spine of the list as well as the pairs are copied.

Scheme Procedure: **alist-delete** key alist \[=\] [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **alist-delete!** key alist \[=\] [¶](07_05_03_srfi1_list_library.md)

Return a list containing the elements of alist but with those elements whose keys are equal to key deleted. The returned elements will be in the same order as they were in alist.

Equality is determined by the \= predicate, or `equal?` if not given. The order in which elements are tested is unspecified, but each equality call is made `(= key alistkey)`, i.e. the given key parameter is first and the key from alist second. This means for instance all associations with a key greater than 5 can be removed with `(alist-delete 5 alist <)`.

`alist-delete` does not modify alist, but the return might share a common tail with alist. `alist-delete!` may modify the list structure of alist to construct its return.

* * *

Previous: [Association Lists](07_05_03_srfi1_list_library.md#7539-association-lists), Up: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.3.10 Set Operations on Lists [¶](07_05_03_srfi1_list_library.md#75310-set-operations-on-lists)

Lists can be used to represent sets of objects. The procedures in this section operate on such lists as sets.

Note that lists are not an efficient way to implement large sets. The procedures here typically take time _mxn_ when operating on m and n element lists. Other data structures like trees, bitsets (see [Bit Vectors](06_06_11_bit_vectors.md#6611-bit-vectors)) or hash tables (see [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables)) are faster.

All these procedures take an equality predicate as the first argument. This predicate is used for testing the objects in the list sets for sameness. This predicate must be consistent with `eq?` (see [Equality](06_09_general_utility_functions.md#691-equality)) in the sense that if two list elements are `eq?` then they must also be equal under the predicate. This simply means a given object must be equal to itself.

Scheme Procedure: **lset<=** \= list … [¶](07_05_03_srfi1_list_library.md)

Return `#t` if each list is a subset of the one following it. I.e., list1 is a subset of list2, list2 is a subset of list3, etc., for as many lists as given. If only one list or no lists are given, the return value is `#t`.

A list x is a subset of y if each element of x is equal to some element in y. Elements are compared using the given \= procedure, called as `(= xelem yelem)`.

(lset<= eq?)                      ⇒ #t
(lset<= eqv? '(1 2 3) '(1))       ⇒ #f
(lset<= eqv? '(1 3 2) '(4 3 1 2)) ⇒ #t

Scheme Procedure: **lset=** \= list … [¶](07_05_03_srfi1_list_library.md)

Return `#t` if all argument lists are set-equal. list1 is compared to list2, list2 to list3, etc., for as many lists as given. If only one list or no lists are given, the return value is `#t`.

Two lists x and y are set-equal if each element of x is equal to some element of y and conversely each element of y is equal to some element of x. The order of the elements in the lists doesn’t matter. Element equality is determined with the given \= procedure, called as `(= xelem yelem)`, but exactly which calls are made is unspecified.

(lset= eq?)                      ⇒ #t
(lset= eqv? '(1 2 3) '(3 2 1))   ⇒ #t
(lset= string-ci=? '("a" "A" "b") '("B" "b" "a")) ⇒ #t

Scheme Procedure: **lset-adjoin** \= list elem … [¶](07_05_03_srfi1_list_library.md)

Add to list any of the given elems not already in the list. elems are `cons`ed onto the start of list (so the return value shares a common tail with list), but the order that the elems are added is unspecified.

The given \= procedure is used for comparing elements, called as `(= listelem elem)`, i.e., the second argument is one of the given elem parameters.

(lset-adjoin eqv? '(1 2 3) 4 1 5) ⇒ (5 4 1 2 3)

Scheme Procedure: **lset-union** \= list … [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **lset-union!** \= list … [¶](07_05_03_srfi1_list_library.md)

Return the union of the argument list sets. The result is built by taking the union of list1 and list2, then the union of that with list3, etc., for as many lists as given. For one list argument that list itself is the result, for no list arguments the result is the empty list.

The union of two lists x and y is formed as follows. If x is empty then the result is y. Otherwise start with x as the result and consider each y element (from first to last). A y element not equal to something already in the result is `cons`ed onto the result.

The given \= procedure is used for comparing elements, called as `(= relem yelem)`. The first argument is from the result accumulated so far, and the second is from the list being union-ed in. But exactly which calls are made is otherwise unspecified.

Notice that duplicate elements in list1 (or the first non-empty list) are preserved, but that repeated elements in subsequent lists are only added once.

(lset-union eqv?)                          ⇒ ()
(lset-union eqv? '(1 2 3))                 ⇒ (1 2 3)
(lset-union eqv? '(1 2 1 3) '(2 4 5) '(5)) ⇒ (5 4 1 2 1 3)

`lset-union` doesn’t change the given lists but the result may share a tail with the first non-empty list. `lset-union!` can modify all of the given lists to form the result.

Scheme Procedure: **lset-intersection** \= list1 list2 … [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **lset-intersection!** \= list1 list2 … [¶](07_05_03_srfi1_list_library.md)

Return the intersection of list1 with the other argument lists, meaning those elements of list1 which are also in all of list2 etc. For one list argument, just that list is returned.

The test for an element of list1 to be in the return is simply that it’s equal to some element in each of list2 etc. Notice this means an element appearing twice in list1 but only once in each of list2 etc will go into the return twice. The return has its elements in the same order as they were in list1.

The given \= procedure is used for comparing elements, called as `(= elem1 elemN)`. The first argument is from list1 and the second is from one of the subsequent lists. But exactly which calls are made and in what order is unspecified.

(lset-intersection eqv? '(x y))                        ⇒ (x y)
(lset-intersection eqv? '(1 2 3) '(4 3 2))             ⇒ (2 3)
(lset-intersection eqv? '(1 1 2 2) '(1 2) '(2 1) '(2)) ⇒ (2 2)

The return from `lset-intersection` may share a tail with list1. `lset-intersection!` may modify list1 to form its result.

Scheme Procedure: **lset-difference** \= list1 list2 … [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **lset-difference!** \= list1 list2 … [¶](07_05_03_srfi1_list_library.md)

Return list1 with any elements in list2, list3 etc removed (ie. subtracted). For one list argument, just that list is returned.

The given \= procedure is used for comparing elements, called as `(= elem1 elemN)`. The first argument is from list1 and the second from one of the subsequent lists. But exactly which calls are made and in what order is unspecified.

(lset-difference eqv? '(x y))             ⇒ (x y)
(lset-difference eqv? '(1 2 3) '(3 1))    ⇒ (2)
(lset-difference eqv? '(1 2 3) '(3) '(2)) ⇒ (1)

The return from `lset-difference` may share a tail with list1. `lset-difference!` may modify list1 to form its result.

Scheme Procedure: **lset-diff+intersection** \= list1 list2 … [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **lset-diff+intersection!** \= list1 list2 … [¶](07_05_03_srfi1_list_library.md)

Return two values (see [Returning and Accepting Multiple Values](06_11_controlling_the_flow_of_program_execution.md#6117-returning-and-accepting-multiple-values)), the difference and intersection of the argument lists as per `lset-difference` and `lset-intersection` above.

For two list arguments this partitions list1 into those elements of list1 which are in list2 and not in list2. (But for more than two arguments there can be elements of list1 which are neither part of the difference nor the intersection.)

One of the return values from `lset-diff+intersection` may share a tail with list1. `lset-diff+intersection!` may modify list1 to form its results.

Scheme Procedure: **lset-xor** \= list … [¶](07_05_03_srfi1_list_library.md)

Scheme Procedure: **lset-xor!** \= list … [¶](07_05_03_srfi1_list_library.md)

Return an XOR of the argument lists. For two lists this means those elements which are in exactly one of the lists. For more than two lists it means those elements which appear in an odd number of the lists.

To be precise, the XOR of two lists x and y is formed by taking those elements of x not equal to any element of y, plus those elements of y not equal to any element of x. Equality is determined with the given \= procedure, called as `(= e1 e2)`. One argument is from x and the other from y, but which way around is unspecified. Exactly which calls are made is also unspecified, as is the order of the elements in the result.

(lset-xor eqv? '(x y))             ⇒ (x y)
(lset-xor eqv? '(1 2 3) '(4 3 2))  ⇒ (4 1)

The return from `lset-xor` may share a tail with one of the list arguments. `lset-xor!` may modify list1 to form its result.

* * *

Next: [SRFI-4 - Homogeneous numeric vector datatypes](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#755-srfi-4---homogeneous-numeric-vector-datatypes), Previous: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
