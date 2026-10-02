### 6.3 The SCM Type [¶](06_03_the_scm_type.md#63-the-scm-type)

Guile represents all Scheme values with the single C type `SCM`. For an introduction to this topic, See [Dynamic Types](05_programming_in_c.md#541-dynamic-types).

C Type: **SCM** [¶](06_03_the_scm_type.md)

`SCM` is the user level abstract C type that is used to represent all of Guile’s Scheme objects, no matter what the Scheme object type is. No C operation except assignment is guaranteed to work with variables of type `SCM`, so you should only use macros and functions to work with `SCM` values. Values are converted between C data types and the `SCM` type with utility functions and macros.

C Type: **scm\_t\_bits** [¶](06_03_the_scm_type.md)

`scm_t_bits` is an unsigned integral data type that is guaranteed to be large enough to hold all information that is required to represent any Scheme object. While this data type is mostly used to implement Guile’s internals, the use of this type is also necessary to write certain kinds of extensions to Guile.

C Type: **scm\_t\_signed\_bits** [¶](06_03_the_scm_type.md)

This is a signed integral type of the same size as `scm_t_bits`.

C Macro: `scm_t_bits` **SCM\_UNPACK** `(SCM x)` [¶](06_03_the_scm_type.md)

Transforms the `SCM` value x into its representation as an integral type. Only after applying `SCM_UNPACK` it is possible to access the bits and contents of the `SCM` value.

C Macro: `SCM` **SCM\_PACK** `(scm_t_bits x)` [¶](06_03_the_scm_type.md)

Takes a valid integral representation of a Scheme object and transforms it into its representation as a `SCM` value.

* * *

Next: [Snarfing Macros](06_05_snarfing_macros.md#65-snarfing-macros), Previous: [The SCM Type](06_03_the_scm_type.md#63-the-scm-type), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
