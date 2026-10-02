#### 6.6.22 Hash Tables [¶](06_06_22_hash_tables.md#6622-hash-tables)

Hash tables are dictionaries which offer similar functionality as association lists: They provide a mapping from keys to values. The difference is that association lists need time linear in the size of elements when searching for entries, whereas hash tables can normally search in constant time. The drawback is that hash tables require a little bit more memory, and that you can not use the normal list procedures (see [Lists](06_06_09_lists.md#669-lists)) for working with them.

*   [Hash Table Examples](06_06_22_hash_tables.md#66221-hash-table-examples)
*   [Hash Table Reference](06_06_22_hash_tables.md#66222-hash-table-reference)

* * *

Next: [Hash Table Reference](06_06_22_hash_tables.md#66222-hash-table-reference), Up: [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.22.1 Hash Table Examples [¶](06_06_22_hash_tables.md#66221-hash-table-examples)

For demonstration purposes, this section gives a few usage examples of some hash table procedures, together with some explanation what they do.

First we start by creating a new hash table with 31 slots, and populate it with two key/value pairs.

(define h ([make-hash-table](06_06_22_hash_tables.md) 31))

;; This is an opaque object
h
⇒
#<hash-table 0/31>

;; Inserting into a hash table can be done with hashq-set!
([hashq-set!](06_06_22_hash_tables.md) h 'foo "bar")
⇒
"bar"

([hashq-set!](06_06_22_hash_tables.md) h 'braz "zonk")
⇒
"zonk"

;; Or with hash-create-handle!
([hashq-create-handle!](06_06_22_hash_tables.md) h 'frob #f)
⇒
(frob . #f)

You can get the value for a given key with the procedure `hashq-ref`, but the problem with this procedure is that you cannot reliably determine whether a key does exists in the table. The reason is that the procedure returns `#f` if the key is not in the table, but it will return the same value if the key is in the table and just happens to have the value `#f`, as you can see in the following examples.

([hashq-ref](06_06_22_hash_tables.md) h 'foo)
⇒
"bar"

([hashq-ref](06_06_22_hash_tables.md) h 'frob)
⇒
#f

([hashq-ref](06_06_22_hash_tables.md) h 'not-there)
⇒
#f

It is often better is to use the procedure `hashq-get-handle`, which makes a distinction between the two cases. Just like `assq`, this procedure returns a key/value-pair on success, and `#f` if the key is not found.

([hashq-get-handle](06_06_22_hash_tables.md) h 'foo)
⇒
(foo . "bar")

([hashq-get-handle](06_06_22_hash_tables.md) h 'not-there)
⇒
#f

Interesting results can be computed by using `hash-fold` to work through each element. This example will count the total number of elements:

([hash-fold](06_06_22_hash_tables.md) (lambda (key value seed) ([+](06_06_02_numerical_data_types.md) 1 seed)) 0 h)
⇒
3

The same thing can be done with the procedure `hash-count`, which can also count the number of elements matching a particular predicate. For example, count the number of elements with string values:

([hash-count](06_06_22_hash_tables.md) (lambda (key value) ([string?](06_06_05_strings.md) value)) h)
⇒
2

Counting all the elements is a simple task using `const`:

([hash-count](06_06_22_hash_tables.md) ([const](06_07_procedures.md) #t) h)
⇒
3

* * *

Previous: [Hash Table Examples](06_06_22_hash_tables.md#66221-hash-table-examples), Up: [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.22.2 Hash Table Reference [¶](06_06_22_hash_tables.md#66222-hash-table-reference)

Like the association list functions, the hash table functions come in several varieties, according to the equality test used for the keys. Plain `hash-` functions use `equal?`, `hashq-` functions use `eq?`, `hashv-` functions use `eqv?`, and the `hashx-` functions use an application supplied test.

A single `make-hash-table` creates a hash table suitable for use with any set of functions, but it’s imperative that just one set is then used consistently, or results will be unpredictable.

Hash tables are implemented as a vector indexed by a hash value formed from the key, with an association list of key/value pairs for each bucket in case distinct keys hash together. Direct access to the pairs in those lists is provided by the `-handle-` functions.

When the number of entries in a hash table goes above a threshold, the vector is made larger and the entries are rehashed, to prevent the bucket lists from becoming too long and slowing down accesses. When the number of entries goes below a threshold, the vector is shrunk to save space.

For the `hashx-` “extended” routines, an application supplies a hash function producing an integer index like `hashq` etc below, and an assoc alist search function like `assq` etc (see [Retrieving Alist Entries](06_06_20_association_lists.md#66203-retrieving-alist-entries)). Here’s an example of such functions implementing case-insensitive hashing of string keys,

(use-modules (srfi srfi-1)
             (srfi srfi-13))

(define (my-hash str size)
  (remainder (string-hash-ci str) size))
(define (my-assoc str alist)
  (find (lambda (pair) (string-ci=? str (car pair))) alist))

(define my-table (make-hash-table))
(hashx-set! my-hash my-assoc my-table "foo" 123)

(hashx-ref my-hash my-assoc my-table "FOO")
⇒ 123

In a `hashx-` hash function the aim is to spread keys across the vector, so bucket lists don’t become long. But the actual values are arbitrary as long as they’re in the range 0 to _size\-1_. Helpful functions for forming a hash value, in addition to `hashq` etc below, include `symbol-hash` (see [Symbols as Lookup Keys](06_06_06_symbols.md#6662-symbols-as-lookup-keys)), `string-hash` and `string-hash-ci` (see [String Comparison](06_06_05_strings.md#6657-string-comparison)), and `char-set-hash` (see [Character Set Predicates/Comparison](06_06_04_character_sets.md#6641-character-set-predicatescomparison)).

  

Scheme Procedure: **make-hash-table** \[size\] [¶](06_06_22_hash_tables.md)

Create a new hash table object, with an optional minimum vector size.

When size is given, the table vector will still grow and shrink automatically, as described above, but with size as a minimum. If an application knows roughly how many entries the table will hold then it can use size to avoid rehashing when initial entries are added.

Scheme Procedure: **alist->hash-table** alist [¶](06_06_22_hash_tables.md)

Scheme Procedure: **alist->hashq-table** alist [¶](06_06_22_hash_tables.md)

Scheme Procedure: **alist->hashv-table** alist [¶](06_06_22_hash_tables.md)

Scheme Procedure: **alist->hashx-table** hash assoc alist [¶](06_06_22_hash_tables.md)

Convert alist into a hash table. When keys are repeated in alist, the leftmost association takes precedence.

(use-modules (ice-9 hash-table))
(alist->hash-table '((foo . 1) (bar . 2)))

When converting to an extended hash table, custom hash and assoc procedures must be provided.

(alist->hashx-table hash assoc '((foo . 1) (bar . 2)))

Scheme Procedure: **hash-table?** obj [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_table\_p** (obj) [¶](06_06_22_hash_tables.md)

Return `#t` if obj is a abstract hash table object.

Scheme Procedure: **hash-clear!** table [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_clear\_x** (table) [¶](06_06_22_hash_tables.md)

Remove all items from table (without triggering a resize).

Scheme Procedure: **hash-ref** table key \[dflt\] [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashq-ref** table key \[dflt\] [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashv-ref** table key \[dflt\] [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashx-ref** hash assoc table key \[dflt\] [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_ref** (table, key, dflt) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashq\_ref** (table, key, dflt) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashv\_ref** (table, key, dflt) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashx\_ref** (hash, assoc, table, key, dflt) [¶](06_06_22_hash_tables.md)

Lookup key in the given hash table, and return the associated value. If key is not found, return dflt, or `#f` if dflt is not given.

Scheme Procedure: **hash-set!** table key val [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashq-set!** table key val [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashv-set!** table key val [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashx-set!** hash assoc table key val [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_set\_x** (table, key, val) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashq\_set\_x** (table, key, val) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashv\_set\_x** (table, key, val) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashx\_set\_x** (hash, assoc, table, key, val) [¶](06_06_22_hash_tables.md)

Associate val with key in the given hash table. If key is already present then it’s associated value is changed. If it’s not present then a new entry is created.

Scheme Procedure: **hash-remove!** table key [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashq-remove!** table key [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashv-remove!** table key [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashx-remove!** hash assoc table key [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_remove\_x** (table, key) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashq\_remove\_x** (table, key) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashv\_remove\_x** (table, key) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashx\_remove\_x** (hash, assoc, table, key) [¶](06_06_22_hash_tables.md)

Remove any association for key in the given hash table. If key is not in table then nothing is done.

Scheme Procedure: **hash** key size [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashq** key size [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashv** key size [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash** (key, size) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashq** (key, size) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashv** (key, size) [¶](06_06_22_hash_tables.md)

Return a hash value for key. This is a number in the range _0_ to _size\-1_, which is suitable for use in a hash table of the given size.

Note that `hashq` and `hashv` may use internal addresses of objects, so if an object is garbage collected and re-created it can have a different hash value, even when the two are notionally `eq?`. For instance with symbols,

(hashq 'something 123)   ⇒ 19
(gc)
(hashq 'something 123)   ⇒ 62

In normal use this is not a problem, since an object entered into a hash table won’t be garbage collected until removed. It’s only if hashing calculations are somehow separated from normal references that its lifetime needs to be considered.

Scheme Procedure: **hash-get-handle** table key [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashq-get-handle** table key [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashv-get-handle** table key [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashx-get-handle** hash assoc table key [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_get\_handle** (table, key) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashq\_get\_handle** (table, key) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashv\_get\_handle** (table, key) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashx\_get\_handle** (hash, assoc, table, key) [¶](06_06_22_hash_tables.md)

Return the `(key . value)` pair for key in the given hash table, or `#f` if key is not in table.

Scheme Procedure: **hash-create-handle!** table key init [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashq-create-handle!** table key init [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashv-create-handle!** table key init [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hashx-create-handle!** hash assoc table key init [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_create\_handle\_x** (table, key, init) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashq\_create\_handle\_x** (table, key, init) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashv\_create\_handle\_x** (table, key, init) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hashx\_create\_handle\_x** (hash, assoc, table, key, init) [¶](06_06_22_hash_tables.md)

Return the `(key . value)` pair for key in the given hash table. If key is not in table then create an entry for it with init as the value, and return that pair.

Scheme Procedure: **hash-map->list** proc table [¶](06_06_22_hash_tables.md)

Scheme Procedure: **hash-for-each** proc table [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_map\_to\_list** (proc, table) [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_for\_each** (proc, table) [¶](06_06_22_hash_tables.md)

Apply proc to the entries in the given hash table. Each call is `(proc key value)`. `hash-map->list` returns a list of the results from these calls, `hash-for-each` discards the results and returns an unspecified value.

Calls are made over the table entries in an unspecified order, and for `hash-map->list` the order of the values in the returned list is unspecified. Results will be unpredictable if table is modified while iterating.

For example the following returns a new alist comprising all the entries from `mytable`, in no particular order.

(hash-map->list cons mytable)

Scheme Procedure: **hash-for-each-handle** proc table [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_for\_each\_handle** (proc, table) [¶](06_06_22_hash_tables.md)

Apply proc to the entries in the given hash table. Each call is `(proc handle)`, where handle is a `(key . value)` pair. Return an unspecified value.

`hash-for-each-handle` differs from `hash-for-each` only in the argument list of proc.

Scheme Procedure: **hash-fold** proc init table [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_fold** (proc, init, table) [¶](06_06_22_hash_tables.md)

Accumulate a result by applying proc to the elements of the given hash table. Each call is `(proc key value prior-result)`, where key and value are from the table and prior-result is the return from the previous proc call. For the first call, prior-result is the given init value.

Calls are made over the table entries in an unspecified order. Results will be unpredictable if table is modified while `hash-fold` is running.

For example, the following returns a count of how many keys in `mytable` are strings.

(hash-fold (lambda (key value prior)
             (if (string? key) (1+ prior) prior))
           0 mytable)

Scheme Procedure: **hash-count** pred table [¶](06_06_22_hash_tables.md)

C Function: **scm\_hash\_count** (pred, table) [¶](06_06_22_hash_tables.md)

Return the number of elements in the given hash table that cause `(pred key value)` to return true. To quickly determine the total number of elements, use `(const #t)` for pred.

* * *

Previous: [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
