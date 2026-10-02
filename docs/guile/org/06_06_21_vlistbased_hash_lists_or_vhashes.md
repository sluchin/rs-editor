#### 6.6.21 VList-Based Hash Lists or “VHashes” [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md#6621-vlist-based-hash-lists-or-vhashes)

The `(ice-9 vlist)` module provides an implementation of _VList-based hash lists_ (see [VLists](06_06_14_vlists.md#6614-vlists)). VList-based hash lists, or _vhashes_, are an immutable dictionary type similar to association lists that maps _keys_ to _values_. However, unlike association lists, accessing a value given its key is typically a constant-time operation.

The VHash programming interface of `(ice-9 vlist)` is mostly the same as that of association lists found in SRFI-1, with procedure names prefixed by `vhash-` instead of `alist-` (see [Association Lists](07_05_03_srfi1_list_library.md#7539-association-lists)).

In addition, vhashes can be manipulated using VList operations:

(vlist-head (vhash-consq 'a 1 vlist-null))
⇒ (a . 1)

(define vh1 (vhash-consq 'b 2 (vhash-consq 'a 1 vlist-null)))
(define vh2 (vhash-consq 'c 3 (vlist-tail vh1)))

(vhash-assq 'a vh2)
⇒ (a . 1)
(vhash-assq 'b vh2)
⇒ #f
(vhash-assq 'c vh2)
⇒ (c . 3)
(vlist->list vh2)
⇒ ((c . 3) (a . 1))

However, keep in mind that procedures that construct new VLists (`vlist-map`, `vlist-filter`, etc.) return raw VLists, not vhashes:

(define vh (alist->vhash '((a . 1) (b . 2) (c . 3)) hashq))
(vhash-assq 'a vh)
⇒ (a . 1)

(define vl
  ;; This will create a raw vlist.
  (vlist-filter (lambda (key+value) (odd? (cdr key+value))) vh))
(vhash-assq 'a vl)
⇒ ERROR: Wrong type argument in position 2

(vlist->list vl)
⇒ ((a . 1) (c . 3))

Scheme Procedure: **vhash?** obj [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Return true if obj is a vhash.

Scheme Procedure: **vhash-cons** key value vhash \[hash-proc\] [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Scheme Procedure: **vhash-consq** key value vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Scheme Procedure: **vhash-consv** key value vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Return a new hash list based on vhash where key is associated with value, using hash-proc to compute the hash of key. vhash must be either `vlist-null` or a vhash returned by a previous call to `vhash-cons`. hash-proc defaults to `hash` (see [`hash` procedure](06_06_22_hash_tables.md#66222-hash-table-reference)). With `vhash-consq`, the `hashq` hash function is used; with `vhash-consv` the `hashv` hash function is used.

All `vhash-cons` calls made to construct a vhash should use the same hash-proc. Failing to do that, the result is undefined.

Scheme Procedure: **vhash-assoc** key vhash \[equal? \[hash-proc\]\] [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Scheme Procedure: **vhash-assq** key vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Scheme Procedure: **vhash-assv** key vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Return the first key/value pair from vhash whose key is equal to key according to the equal? equality predicate (which defaults to `equal?`), and using hash-proc (which defaults to `hash`) to compute the hash of key. The second form uses `eq?` as the equality predicate and `hashq` as the hash function; the last form uses `eqv?` and `hashv`.

Note that it is important to consistently use the same hash function for hash-proc as was passed to `vhash-cons`. Failing to do that, the result is unpredictable.

Scheme Procedure: **vhash-delete** key vhash \[equal? \[hash-proc\]\] [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Scheme Procedure: **vhash-delq** key vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Scheme Procedure: **vhash-delv** key vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Remove all associations from vhash with key, comparing keys with equal? (which defaults to `equal?`), and computing the hash of key using hash-proc (which defaults to `hash`). The second form uses `eq?` as the equality predicate and `hashq` as the hash function; the last one uses `eqv?` and `hashv`.

Again the choice of hash-proc must be consistent with previous calls to `vhash-cons`.

Scheme Procedure: **vhash-fold** proc init vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Scheme Procedure: **vhash-fold-right** proc init vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Fold over the key/value elements of vhash in the given direction, with each call to proc having the form `(proc key value result)`, where result is the result of the previous call to proc and init the value of result for the first call to proc.

Scheme Procedure: **vhash-fold\*** proc init key vhash \[equal? \[hash\]\] [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Scheme Procedure: **vhash-foldq\*** proc init key vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Scheme Procedure: **vhash-foldv\*** proc init key vhash [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Fold over all the values associated with key in vhash, with each call to proc having the form `(proc value result)`, where result is the result of the previous call to proc and init the value of result for the first call to proc.

Keys in vhash are hashed using hash are compared using equal?. The second form uses `eq?` as the equality predicate and `hashq` as the hash function; the third one uses `eqv?` and `hashv`.

Example:

(define vh
  (alist->vhash '((a . 1) (a . 2) (z . 0) (a . 3))))

(vhash-fold\* cons '() 'a vh)
⇒ (3 2 1)

(vhash-fold\* cons '() 'z vh)
⇒ (0)

Scheme Procedure: **alist->vhash** alist \[hash-proc\] [¶](06_06_21_vlistbased_hash_lists_or_vhashes.md)

Return the vhash corresponding to alist, an association list, using hash-proc to compute key hashes. When omitted, hash-proc defaults to `hash`.

* * *

Next: [Other Types](06_06_23_other_types.md#6623-other-types), Previous: [VList-Based Hash Lists or “VHashes”](06_06_21_vlistbased_hash_lists_or_vhashes.md#6621-vlist-based-hash-lists-or-vhashes), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
