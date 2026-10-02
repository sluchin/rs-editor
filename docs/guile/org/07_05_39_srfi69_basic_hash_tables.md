#### 7.5.39 SRFI-69 - Basic hash tables [¶](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---basic-hash-tables)

This is a portable wrapper around Guile’s built-in hash table and weak table support. See [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables), for information on that built-in support. Above that, this hash-table interface provides association of equality and hash functions with tables at creation time, so variants of each function are not required, as well as a procedure that takes care of most uses for Guile hash table handles, which this SRFI does not provide as such.

Access it with:

([use-modules](06_18_modules.md) (srfi srfi-69))

*   [Creating hash tables](07_05_39_srfi69_basic_hash_tables.md#75391-creating-hash-tables)
*   [Accessing table items](07_05_39_srfi69_basic_hash_tables.md#75392-accessing-table-items)
*   [Table properties](07_05_39_srfi69_basic_hash_tables.md#75393-table-properties)
*   [Hash table algorithms](07_05_39_srfi69_basic_hash_tables.md#75394-hash-table-algorithms)

* * *

Next: [Accessing table items](07_05_39_srfi69_basic_hash_tables.md#75392-accessing-table-items), Up: [SRFI-69 - Basic hash tables](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---basic-hash-tables)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.39.1 Creating hash tables [¶](07_05_39_srfi69_basic_hash_tables.md#75391-creating-hash-tables)

Scheme Procedure: **make-hash-table** \[equal-proc hash-proc #:weak weakness start-size\] [¶](07_05_39_srfi69_basic_hash_tables.md)

Create and answer a new hash table with equal-proc as the equality function and hash-proc as the hashing function.

By default, equal-proc is `equal?`. It can be any two-argument procedure, and should answer whether two keys are the same for this table’s purposes.

By default hash-proc assumes that `equal-proc` is no coarser than `equal?` unless it is literally `string-ci=?`. If provided, hash-proc should be a two-argument procedure that takes a key and the current table size, and answers a reasonably good hash integer between 0 (inclusive) and the size (exclusive).

weakness should be `#f` or a symbol indicating how “weak” the hash table is:

`#f`

An ordinary non-weak hash table. This is the default.

`key`

When the key has no more non-weak references at GC, remove that entry.

`value`

When the value has no more non-weak references at GC, remove that entry.

`key-or-value`

When either has no more non-weak references at GC, remove the association.

As a legacy of the time when Guile couldn’t grow hash tables, start-size is an optional integer argument that specifies the approximate starting size for the hash table, which will be rounded to an algorithmically-sounder number.

By _coarser_ than `equal?`, we mean that for all x and y values where `(equal-proc x y)`, `(equal? x y)` as well. If that does not hold for your equal-proc, you must provide a hash-proc.

In the case of weak tables, remember that _references_ above always refers to `eq?`\-wise references. Just because you have a reference to some string `"foo"` doesn’t mean that an association with key `"foo"` in a weak-key table _won’t_ be collected; it only counts as a reference if the two `"foo"`s are `eq?`, regardless of equal-proc. As such, it is usually only sensible to use `eq?` and `hashq` as the equivalence and hash functions for a weak table. See [Weak References](06_17_memory_management_and_garbage_collection.md#6173-weak-references), for more information on Guile’s built-in weak table support.

Scheme Procedure: **alist->hash-table** alist \[equal-proc hash-proc #:weak weakness start-size\] [¶](07_05_39_srfi69_basic_hash_tables.md)

As with `make-hash-table`, but initialize it with the associations in alist. Where keys are repeated in alist, the leftmost association takes precedence.

* * *

Next: [Table properties](07_05_39_srfi69_basic_hash_tables.md#75393-table-properties), Previous: [Creating hash tables](07_05_39_srfi69_basic_hash_tables.md#75391-creating-hash-tables), Up: [SRFI-69 - Basic hash tables](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---basic-hash-tables)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.39.2 Accessing table items [¶](07_05_39_srfi69_basic_hash_tables.md#75392-accessing-table-items)

Scheme Procedure: **hash-table-ref** table key \[default-thunk\] [¶](07_05_39_srfi69_basic_hash_tables.md)

Scheme Procedure: **hash-table-ref/default** table key default [¶](07_05_39_srfi69_basic_hash_tables.md)

Answer the value associated with key in table. If key is not present, answer the result of invoking the thunk default-thunk, which signals an error instead by default.

`hash-table-ref/default` is a variant that requires a third argument, default, and answers default itself instead of invoking it.

Scheme Procedure: **hash-table-set!** table key new-value [¶](07_05_39_srfi69_basic_hash_tables.md)

Set key to new-value in table.

Scheme Procedure: **hash-table-delete!** table key [¶](07_05_39_srfi69_basic_hash_tables.md)

Remove the association of key in table, if present. If absent, do nothing.

Scheme Procedure: **hash-table-exists?** table key [¶](07_05_39_srfi69_basic_hash_tables.md)

Answer whether key has an association in table.

Scheme Procedure: **hash-table-update!** table key modifier \[default-thunk\] [¶](07_05_39_srfi69_basic_hash_tables.md)

Scheme Procedure: **hash-table-update!/default** table key modifier default [¶](07_05_39_srfi69_basic_hash_tables.md)

Replace key’s associated value in table by invoking modifier with one argument, the old value.

If key is not present, and default-thunk is provided, invoke it with no arguments to get the “old value” to be passed to modifier as above. If default-thunk is not provided in such a case, signal an error.

`hash-table-update!/default` is a variant that requires the fourth argument, which is used directly as the “old value” rather than as a thunk to be invoked to retrieve the “old value”.

* * *

Next: [Hash table algorithms](07_05_39_srfi69_basic_hash_tables.md#75394-hash-table-algorithms), Previous: [Accessing table items](07_05_39_srfi69_basic_hash_tables.md#75392-accessing-table-items), Up: [SRFI-69 - Basic hash tables](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---basic-hash-tables)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.39.3 Table properties [¶](07_05_39_srfi69_basic_hash_tables.md#75393-table-properties)

Scheme Procedure: **hash-table-size** table [¶](07_05_39_srfi69_basic_hash_tables.md)

Answer the number of associations in table. This is guaranteed to run in constant time for non-weak tables.

Scheme Procedure: **hash-table-keys** table [¶](07_05_39_srfi69_basic_hash_tables.md)

Answer an unordered list of the keys in table.

Scheme Procedure: **hash-table-values** table [¶](07_05_39_srfi69_basic_hash_tables.md)

Answer an unordered list of the values in table.

Scheme Procedure: **hash-table-walk** table proc [¶](07_05_39_srfi69_basic_hash_tables.md)

Invoke proc once for each association in table, passing the key and value as arguments.

Scheme Procedure: **hash-table-fold** table proc init [¶](07_05_39_srfi69_basic_hash_tables.md)

Invoke `(proc key value previous)` for each key and value in table, where previous is the result of the previous invocation, using init as the first previous value. Answer the final proc result.

Scheme Procedure: **hash-table->alist** table [¶](07_05_39_srfi69_basic_hash_tables.md)

Answer an alist where each association in table is an association in the result.

* * *

Previous: [Table properties](07_05_39_srfi69_basic_hash_tables.md#75393-table-properties), Up: [SRFI-69 - Basic hash tables](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---basic-hash-tables)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.39.4 Hash table algorithms [¶](07_05_39_srfi69_basic_hash_tables.md#75394-hash-table-algorithms)

Each hash table carries an _equivalence function_ and a _hash function_, used to implement key lookups. Beginning users should follow the rules for consistency of the default hash-proc specified above. Advanced users can use these to implement their own equivalence and hash functions for specialized lookup semantics.

Scheme Procedure: **hash-table-equivalence-function** hash-table [¶](07_05_39_srfi69_basic_hash_tables.md)

Scheme Procedure: **hash-table-hash-function** hash-table [¶](07_05_39_srfi69_basic_hash_tables.md)

Answer the equivalence and hash function of hash-table, respectively.

Scheme Procedure: **hash** obj \[size\] [¶](07_05_39_srfi69_basic_hash_tables.md)

Scheme Procedure: **string-hash** obj \[size\] [¶](07_05_39_srfi69_basic_hash_tables.md)

Scheme Procedure: **string-ci-hash** obj \[size\] [¶](07_05_39_srfi69_basic_hash_tables.md)

Scheme Procedure: **hash-by-identity** obj \[size\] [¶](07_05_39_srfi69_basic_hash_tables.md)

Answer a hash value appropriate for equality predicate `equal?`, `string=?`, `string-ci=?`, and `eq?`, respectively.

`hash` is a backwards-compatible replacement for Guile’s built-in `hash`.

* * *

Next: [SRFI-87 => in case clauses](07_05_41_srfi87_in_case_clauses.md#7541-srfi-87--in-case-clauses), Previous: [SRFI-69 - Basic hash tables](07_05_39_srfi69_basic_hash_tables.md#7539-srfi-69---basic-hash-tables), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
