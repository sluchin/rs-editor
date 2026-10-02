#### 7.5.26 SRFI-38 - External Representation for Data With Shared Structure [¶](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md#7526-srfi-38---external-representation-for-data-with-shared-structure)

This subsection is based on [the specification of SRFI-38](http://srfi.schemers.org/srfi-38/srfi-38.html) written by Ray Dillinger.

This SRFI creates an alternative external representation for data written and read using `write-with-shared-structure` and `read-with-shared-structure`. It is identical to the grammar for external representation for data written and read with `write` and `read` given in section 7 of R5RS, except that the single production

<datum> --> <simple datum> | <compound datum>

is replaced by the following five productions:

<datum> --> <defining datum> | <nondefining datum> | <defined datum>
<defining datum> -->  #<indexnum>=<nondefining datum>
<defined datum> --> #<indexnum>#
<nondefining datum> --> <simple datum> | <compound datum>
<indexnum> --> <digit 10>+

Scheme procedure: **write-with-shared-structure** obj [¶](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md)

Scheme procedure: **write-with-shared-structure** obj port [¶](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md)

Scheme procedure: **write-with-shared-structure** obj port optarg [¶](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md)

Writes an external representation of obj to the given port. Strings that appear in the written representation are enclosed in doublequotes, and within those strings backslash and doublequote characters are escaped by backslashes. Character objects are written using the `#\` notation.

Objects which denote locations rather than values (cons cells, vectors, and non-zero-length strings in R5RS scheme; also Guile’s structs, bytevectors and ports and hash-tables), if they appear at more than one point in the data being written, are preceded by ‘#N\=’ the first time they are written and replaced by ‘#N#’ all subsequent times they are written, where N is a natural number used to identify that particular object. If objects which denote locations occur only once in the structure, then `write-with-shared-structure` must produce the same external representation for those objects as `write`.

`write-with-shared-structure` terminates in finite time and produces a finite representation when writing finite data.

`write-with-shared-structure` returns an unspecified value. The port argument may be omitted, in which case it defaults to the value returned by `(current-output-port)`. The optarg argument may also be omitted. If present, its effects on the output and return value are unspecified but `write-with-shared-structure` must still write a representation that can be read by `read-with-shared-structure`. Some implementations may wish to use optarg to specify formatting conventions, numeric radixes, or return values. Guile’s implementation ignores optarg.

For example, the code

(begin (define a ([cons](06_06_08_pairs.md) 'val1 'val2))
       ([set-cdr!](06_06_08_pairs.md) a a)
       ([write-with-shared-structure](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md) a))

should produce the output `#1=(val1 . #1#)`. This shows a cons cell whose `cdr` contains itself.

Scheme procedure: **read-with-shared-structure** [¶](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md)

Scheme procedure: **read-with-shared-structure** port [¶](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md)

`read-with-shared-structure` converts the external representations of Scheme objects produced by `write-with-shared-structure` into Scheme objects. That is, it is a parser for the nonterminal ‘<datum>’ in the augmented external representation grammar defined above. `read-with-shared-structure` returns the next object parsable from the given input port, updating port to point to the first character past the end of the external representation of the object.

If an end-of-file is encountered in the input before any characters are found that can begin an object, then an end-of-file object is returned. The port remains open, and further attempts to read it (by `read-with-shared-structure` or `read` will also return an end-of-file object. If an end of file is encountered after the beginning of an object’s external representation, but the external representation is incomplete and therefore not parsable, an error is signaled.

The port argument may be omitted, in which case it defaults to the value returned by `(current-input-port)`. It is an error to read from a closed port.

* * *

Next: [SRFI-41 - Streams](07_05_28_srfi41_streams.md#7528-srfi-41---streams), Previous: [SRFI-38 - External Representation for Data With Shared Structure](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md#7526-srfi-38---external-representation-for-data-with-shared-structure), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
