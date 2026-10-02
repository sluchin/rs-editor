#### 6.6.20 Association Lists [¶](06_06_20_association_lists.md#6620-association-lists)

An association list is a conventional data structure that is often used to implement simple key-value databases. It consists of a list of entries in which each entry is a pair. The _key_ of each entry is the `car` of the pair and the _value_ of each entry is the `cdr`.

ASSOCIATION LIST ::=  '( (KEY1 . VALUE1)
                         (KEY2 . VALUE2)
                         (KEY3 . VALUE3)
                         ...
                       )

Association lists are also known, for short, as _alists_.

The structure of an association list is just one example of the infinite number of possible structures that can be built using pairs and lists. As such, the keys and values in an association list can be manipulated using the general list structure procedures `cons`, `car`, `cdr`, `set-car!`, `set-cdr!` and so on. However, because association lists are so useful, Guile also provides specific procedures for manipulating them.

*   [Alist Key Equality](06_06_20_association_lists.md#66201-alist-key-equality)
*   [Adding or Setting Alist Entries](06_06_20_association_lists.md#66202-adding-or-setting-alist-entries)
*   [Retrieving Alist Entries](06_06_20_association_lists.md#66203-retrieving-alist-entries)
*   [Removing Alist Entries](06_06_20_association_lists.md#66204-removing-alist-entries)
*   [Sloppy Alist Functions](06_06_20_association_lists.md#66205-sloppy-alist-functions)
*   [Alist Example](06_06_20_association_lists.md#66206-alist-example)

* * *

Next: [Adding or Setting Alist Entries](06_06_20_association_lists.md#66202-adding-or-setting-alist-entries), Up: [Association Lists](06_06_20_association_lists.md#6620-association-lists)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.20.1 Alist Key Equality [¶](06_06_20_association_lists.md#66201-alist-key-equality)

All of Guile’s dedicated association list procedures, apart from `acons`, come in three flavors, depending on the level of equality that is required to decide whether an existing key in the association list is the same as the key that the procedure call uses to identify the required entry.

*   Procedures with _assq_ in their name use `eq?` to determine key equality.
*   Procedures with _assv_ in their name use `eqv?` to determine key equality.
*   Procedures with _assoc_ in their name use `equal?` to determine key equality.

`acons` is an exception because it is used to build association lists which do not require their entries’ keys to be unique.

* * *

Next: [Retrieving Alist Entries](06_06_20_association_lists.md#66203-retrieving-alist-entries), Previous: [Alist Key Equality](06_06_20_association_lists.md#66201-alist-key-equality), Up: [Association Lists](06_06_20_association_lists.md#6620-association-lists)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.20.2 Adding or Setting Alist Entries [¶](06_06_20_association_lists.md#66202-adding-or-setting-alist-entries)

`acons` adds a new entry to an association list and returns the combined association list. The combined alist is formed by consing the new entry onto the head of the alist specified in the `acons` procedure call. So the specified alist is not modified, but its contents become shared with the tail of the combined alist that `acons` returns.

In the most common usage of `acons`, a variable holding the original association list is updated with the combined alist:

(set! address-list (acons name address address-list))

In such cases, it doesn’t matter that the old and new values of `address-list` share some of their contents, since the old value is usually no longer independently accessible.

Note that `acons` adds the specified new entry regardless of whether the alist may already contain entries with keys that are, in some sense, the same as that of the new entry. Thus `acons` is ideal for building alists where there is no concept of key uniqueness.

(set! task-list (acons 3 "pay gas bill" '()))
task-list
⇒
((3 . "pay gas bill"))

(set! task-list (acons 3 "tidy bedroom" task-list))
task-list
⇒
((3 . "tidy bedroom") (3 . "pay gas bill"))

`assq-set!`, `assv-set!` and `assoc-set!` are used to add or replace an entry in an association list where there _is_ a concept of key uniqueness. If the specified association list already contains an entry whose key is the same as that specified in the procedure call, the existing entry is replaced by the new one. Otherwise, the new entry is consed onto the head of the old association list to create the combined alist. In all cases, these procedures return the combined alist.

`assq-set!` and friends _may_ destructively modify the structure of the old association list in such a way that an existing variable is correctly updated without having to `set!` it to the value returned:

address-list
⇒
(("mary" . "34 Elm Road") ("james" . "16 Bow Street"))

(assoc-set! address-list "james" "1a London Road")
⇒
(("mary" . "34 Elm Road") ("james" . "1a London Road"))

address-list
⇒
(("mary" . "34 Elm Road") ("james" . "1a London Road"))

Or they may not:

(assoc-set! address-list "bob" "11 Newington Avenue")
⇒
(("bob" . "11 Newington Avenue") ("mary" . "34 Elm Road")
 ("james" . "1a London Road"))

address-list
⇒
(("mary" . "34 Elm Road") ("james" . "1a London Road"))

The only safe way to update an association list variable when adding or replacing an entry like this is to `set!` the variable to the returned value:

(set! address-list
      (assoc-set! address-list "bob" "11 Newington Avenue"))
address-list
⇒
(("bob" . "11 Newington Avenue") ("mary" . "34 Elm Road")
 ("james" . "1a London Road"))

Because of this slight inconvenience, you may find it more convenient to use hash tables to store dictionary data. If your application will not be modifying the contents of an alist very often, this may not make much difference to you.

If you need to keep the old value of an association list in a form independent from the list that results from modification by `acons`, `assq-set!`, `assv-set!` or `assoc-set!`, use `alist-copy` to copy the old association list before modifying it.

Scheme Procedure: **acons** key value alist [¶](06_06_20_association_lists.md)

C Function: **scm\_acons** (key, value, alist) [¶](06_06_20_association_lists.md)

Add a new key-value pair to alist. A new pair is created whose car is key and whose cdr is value, and the pair is consed onto alist, and the new list is returned. This function is _not_ destructive; alist is not modified.

Scheme Procedure: **assq-set!** alist key val [¶](06_06_20_association_lists.md)

Scheme Procedure: **assv-set!** alist key value [¶](06_06_20_association_lists.md)

Scheme Procedure: **assoc-set!** alist key value [¶](06_06_20_association_lists.md)

C Function: **scm\_assq\_set\_x** (alist, key, val) [¶](06_06_20_association_lists.md)

C Function: **scm\_assv\_set\_x** (alist, key, val) [¶](06_06_20_association_lists.md)

C Function: **scm\_assoc\_set\_x** (alist, key, val) [¶](06_06_20_association_lists.md)

Reassociate key in alist with value: find any existing alist entry for key and associate it with the new value. If alist does not contain an entry for key, add a new one. Return the (possibly new) alist.

These functions do not attempt to verify the structure of alist, and so may cause unusual results if passed an object that is not an association list.

* * *

Next: [Removing Alist Entries](06_06_20_association_lists.md#66204-removing-alist-entries), Previous: [Adding or Setting Alist Entries](06_06_20_association_lists.md#66202-adding-or-setting-alist-entries), Up: [Association Lists](06_06_20_association_lists.md#6620-association-lists)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.20.3 Retrieving Alist Entries [¶](06_06_20_association_lists.md#66203-retrieving-alist-entries)

`assq`, `assv` and `assoc` find the entry in an alist for a given key, and return the `(key . value)` pair. `assq-ref`, `assv-ref` and `assoc-ref` do a similar lookup, but return just the value.

Scheme Procedure: **assq** key alist [¶](06_06_20_association_lists.md)

Scheme Procedure: **assv** key alist [¶](06_06_20_association_lists.md)

Scheme Procedure: **assoc** key alist [¶](06_06_20_association_lists.md)

C Function: **scm\_assq** (key, alist) [¶](06_06_20_association_lists.md)

C Function: **scm\_assv** (key, alist) [¶](06_06_20_association_lists.md)

C Function: **scm\_assoc** (key, alist) [¶](06_06_20_association_lists.md)

Return the first entry in alist with the given key. The return is the pair `(KEY . VALUE)` from alist. If there’s no matching entry the return is `#f`.

`assq` compares keys with `eq?`, `assv` uses `eqv?` and `assoc` uses `equal?`. See also SRFI-1 which has an extended `assoc` ([Association Lists](07_05_03_srfi1_list_library.md#7539-association-lists)).

Scheme Procedure: **assq-ref** alist key [¶](06_06_20_association_lists.md)

Scheme Procedure: **assv-ref** alist key [¶](06_06_20_association_lists.md)

Scheme Procedure: **assoc-ref** alist key [¶](06_06_20_association_lists.md)

C Function: **scm\_assq\_ref** (alist, key) [¶](06_06_20_association_lists.md)

C Function: **scm\_assv\_ref** (alist, key) [¶](06_06_20_association_lists.md)

C Function: **scm\_assoc\_ref** (alist, key) [¶](06_06_20_association_lists.md)

Return the value from the first entry in alist with the given key, or `#f` if there’s no such entry.

`assq-ref` compares keys with `eq?`, `assv-ref` uses `eqv?` and `assoc-ref` uses `equal?`.

Notice these functions have the key argument last, like other `-ref` functions, but this is opposite to what `assq` etc above use.

When the return is `#f` it can be either key not found, or an entry which happens to have value `#f` in the `cdr`. Use `assq` etc above if you need to differentiate these cases.

* * *

Next: [Sloppy Alist Functions](06_06_20_association_lists.md#66205-sloppy-alist-functions), Previous: [Retrieving Alist Entries](06_06_20_association_lists.md#66203-retrieving-alist-entries), Up: [Association Lists](06_06_20_association_lists.md#6620-association-lists)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.20.4 Removing Alist Entries [¶](06_06_20_association_lists.md#66204-removing-alist-entries)

To remove the element from an association list whose key matches a specified key, use `assq-remove!`, `assv-remove!` or `assoc-remove!` (depending, as usual, on the level of equality required between the key that you specify and the keys in the association list).

As with `assq-set!` and friends, the specified alist may or may not be modified destructively, and the only safe way to update a variable containing the alist is to `set!` it to the value that `assq-remove!` and friends return.

address-list
⇒
(("bob" . "11 Newington Avenue") ("mary" . "34 Elm Road")
 ("james" . "1a London Road"))

(set! address-list (assoc-remove! address-list "mary"))
address-list
⇒
(("bob" . "11 Newington Avenue") ("james" . "1a London Road"))

Note that, when `assq/v/oc-remove!` is used to modify an association list that has been constructed only using the corresponding `assq/v/oc-set!`, there can be at most one matching entry in the alist, so the question of multiple entries being removed in one go does not arise. If `assq/v/oc-remove!` is applied to an association list that has been constructed using `acons`, or an `assq/v/oc-set!` with a different level of equality, or any mixture of these, it removes only the first matching entry from the alist, even if the alist might contain further matching entries. For example:

(define address-list '())
(set! address-list (assq-set! address-list "mary" "11 Elm Street"))
(set! address-list (assq-set! address-list "mary" "57 Pine Drive"))
address-list
⇒
(("mary" . "57 Pine Drive") ("mary" . "11 Elm Street"))

(set! address-list (assoc-remove! address-list "mary"))
address-list
⇒
(("mary" . "11 Elm Street"))

In this example, the two instances of the string "mary" are not the same when compared using `eq?`, so the two `assq-set!` calls add two distinct entries to `address-list`. When compared using `equal?`, both "mary"s in `address-list` are the same as the "mary" in the `assoc-remove!` call, but `assoc-remove!` stops after removing the first matching entry that it finds, and so one of the "mary" entries is left in place.

Scheme Procedure: **assq-remove!** alist key [¶](06_06_20_association_lists.md)

Scheme Procedure: **assv-remove!** alist key [¶](06_06_20_association_lists.md)

Scheme Procedure: **assoc-remove!** alist key [¶](06_06_20_association_lists.md)

C Function: **scm\_assq\_remove\_x** (alist, key) [¶](06_06_20_association_lists.md)

C Function: **scm\_assv\_remove\_x** (alist, key) [¶](06_06_20_association_lists.md)

C Function: **scm\_assoc\_remove\_x** (alist, key) [¶](06_06_20_association_lists.md)

Delete the first entry in alist associated with key, and return the resulting alist.

* * *

Next: [Alist Example](06_06_20_association_lists.md#66206-alist-example), Previous: [Removing Alist Entries](06_06_20_association_lists.md#66204-removing-alist-entries), Up: [Association Lists](06_06_20_association_lists.md#6620-association-lists)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.20.5 Sloppy Alist Functions [¶](06_06_20_association_lists.md#66205-sloppy-alist-functions)

`sloppy-assq`, `sloppy-assv` and `sloppy-assoc` behave like the corresponding non-`sloppy-` procedures, except that they return `#f` when the specified association list is not well-formed, where the non-`sloppy-` versions would signal an error.

Specifically, there are two conditions for which the non-`sloppy-` procedures signal an error, which the `sloppy-` procedures handle instead by returning `#f`. Firstly, if the specified alist as a whole is not a proper list:

(assoc "mary" '((1 . 2) ("key" . "door") . "open sesame"))
⇒
ERROR: In procedure assoc in expression (assoc "mary" (quote #)):
ERROR: Wrong type argument in position 2 (expecting
   association list): ((1 . 2) ("key" . "door") . "open sesame")

(sloppy-assoc "mary" '((1 . 2) ("key" . "door") . "open sesame"))
⇒
#f

Secondly, if one of the entries in the specified alist is not a pair:

(assoc 2 '((1 . 1) 2 (3 . 9)))
⇒
ERROR: In procedure assoc in expression (assoc 2 (quote #)):
ERROR: Wrong type argument in position 2 (expecting
   association list): ((1 . 1) 2 (3 . 9))

(sloppy-assoc 2 '((1 . 1) 2 (3 . 9)))
⇒
#f

Unless you are explicitly working with badly formed association lists, it is much safer to use the non-`sloppy-` procedures, because they help to highlight coding and data errors that the `sloppy-` versions would silently cover up.

Scheme Procedure: **sloppy-assq** key alist [¶](06_06_20_association_lists.md)

C Function: **scm\_sloppy\_assq** (key, alist) [¶](06_06_20_association_lists.md)

Behaves like `assq` but does not do any error checking. Recommended only for use in Guile internals.

Scheme Procedure: **sloppy-assv** key alist [¶](06_06_20_association_lists.md)

C Function: **scm\_sloppy\_assv** (key, alist) [¶](06_06_20_association_lists.md)

Behaves like `assv` but does not do any error checking. Recommended only for use in Guile internals.

Scheme Procedure: **sloppy-assoc** key alist [¶](06_06_20_association_lists.md)

C Function: **scm\_sloppy\_assoc** (key, alist) [¶](06_06_20_association_lists.md)

Behaves like `assoc` but does not do any error checking. Recommended only for use in Guile internals.

* * *

Previous: [Sloppy Alist Functions](06_06_20_association_lists.md#66205-sloppy-alist-functions), Up: [Association Lists](06_06_20_association_lists.md#6620-association-lists)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.20.6 Alist Example [¶](06_06_20_association_lists.md#66206-alist-example)

The following example shows how alists may be used in practice.

(define capitals ([list](06_06_09_lists.md) ([cons](06_06_08_pairs.md) "New York" "Albany")
                       ([cons](06_06_08_pairs.md) "Oregon"   "Salem")
                       ([cons](06_06_08_pairs.md) "Florida"  "Miami")))

Other ways to create an alist are

(define capitals ([acons](06_06_20_association_lists.md) "New York" "Albany"
                        ([acons](06_06_20_association_lists.md) "Oregon" "Salem"
                               ([acons](06_06_20_association_lists.md) "Florida"  "Miami" '()))))

or

([use-modules](06_18_modules.md) (srfi srfi-1)) ; for alist-copy
(define capitals ([alist-copy](07_05_03_srfi1_list_library.md)
                   '(("New York" . "Albany")
                     ("Oregon"   . "Salem")
                     ("Florida"  . "Miami"))))

Here `alist-copy` is necessary if we intend to modify the alist, because a literal like `'(("New York" . "Albany") ...)` cannot be modified.

We can now operate on the alist.

;; What's the capital of Oregon?
([assoc](06_06_20_association_lists.md) "Oregon" capitals)       ⇒ ("Oregon" . "Salem")
([assoc-ref](06_06_20_association_lists.md) capitals "Oregon")   ⇒ "Salem"

;; We left out South Dakota.
([set!](07_06_r6rs_support.md) capitals
      ([assoc-set!](06_06_20_association_lists.md) capitals "South Dakota" "Pierre"))
capitals
⇒ (("South Dakota" . "Pierre")
    ("New York" . "Albany")
    ("Oregon" . "Salem")
    ("Florida" . "Miami"))

;; And we got Florida wrong.
([set!](07_06_r6rs_support.md) capitals
      ([assoc-set!](06_06_20_association_lists.md) capitals "Florida" "Tallahassee"))
capitals
⇒ (("South Dakota" . "Pierre")
    ("New York" . "Albany")
    ("Oregon" . "Salem")
    ("Florida" . "Tallahassee"))

;; After Oregon secedes, we can remove it.
([set!](07_06_r6rs_support.md) capitals
      ([assoc-remove!](06_06_20_association_lists.md) capitals "Oregon"))
capitals
⇒ (("South Dakota" . "Pierre")
    ("New York" . "Albany")
    ("Florida" . "Tallahassee"))

* * *

Next: [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables), Previous: [Association Lists](06_06_20_association_lists.md#6620-association-lists), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
