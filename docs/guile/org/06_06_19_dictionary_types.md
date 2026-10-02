#### 6.6.19 Dictionary Types [¶](06_06_19_dictionary_types.md#6619-dictionary-types)

A _dictionary_ object is a data structure used to index information in a user-defined way. In standard Scheme, the main aggregate data types are lists and vectors. Lists are not really indexed at all, and vectors are indexed only by number (e.g. `(vector-ref foo 5)`). Often you will find it useful to index your data on some other type; for example, in a library catalog you might want to look up a book by the name of its author. Dictionaries are used to help you organize information in such a way.

An _association list_ (or _alist_ for short) is a list of key-value pairs. Each pair represents a single quantity or object; the `car` of the pair is a key which is used to identify the object, and the `cdr` is the object’s value.

A _hash table_ also permits you to index objects with arbitrary keys, but in a way that makes looking up any one object extremely fast. A well-designed hash system makes hash table lookups almost as fast as conventional array or vector references.

Alists are popular among Lisp programmers because they use only the language’s primitive operations (lists, _car_, _cdr_ and the equality primitives). No changes to the language core are necessary. Therefore, with Scheme’s built-in list manipulation facilities, it is very convenient to handle data stored in an association list. Also, alists are highly portable and can be easily implemented on even the most minimal Lisp systems.

However, alists are inefficient, especially for storing large quantities of data. Because we want Guile to be useful for large software systems as well as small ones, Guile provides a rich set of tools for using either association lists or hash tables.

* * *

Next: [VList-Based Hash Lists or “VHashes”](06_06_21_vlistbased_hash_lists_or_vhashes.md#6621-vlist-based-hash-lists-or-vhashes), Previous: [Dictionary Types](06_06_19_dictionary_types.md#6619-dictionary-types), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
