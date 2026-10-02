### 6.20 Foreign Objects [¶](06_20_foreign_objects.md#620-foreign-objects)

This chapter contains reference information related to defining and working with foreign objects. See [Defining New Foreign Object Types](05_programming_in_c.md#55-defining-new-foreign-object-types), for a tutorial-like introduction to foreign objects.

C Type: **scm\_t\_struct\_finalize** [¶](06_20_foreign_objects.md)

This function type returns `void` and takes one `SCM` argument.

C Function: `SCM` **scm\_make\_foreign\_object\_type** `(SCM name, SCM slots, scm_t_struct_finalize finalizer)` [¶](06_20_foreign_objects.md)

Create a fresh foreign object type. name is a symbol naming the type. slots is a list of symbols, each one naming a field in the foreign object type. finalizer indicates the finalizer, and may be `NULL`.

We recommend that finalizers be avoided if possible. See [Foreign Object Memory Management](05_programming_in_c.md#554-foreign-object-memory-management). Finalizers must be async-safe and thread-safe. Again, see [Foreign Object Memory Management](05_programming_in_c.md#554-foreign-object-memory-management). If you are embedding Guile in an application that is not thread-safe, and you define foreign object types that need finalization, you might want to disable automatic finalization, and arrange to call `scm_manually_run_finalizers ()` yourself.

C Function: `int` **scm\_set\_automatic\_finalization\_enabled** `(int enabled_p)` [¶](06_20_foreign_objects.md)

Enable or disable automatic finalization. By default, Guile arranges to invoke object finalizers automatically, in a separate thread if possible. Passing a zero value for enabled\_p will disable automatic finalization for Guile as a whole. If you disable automatic finalization, you will have to call `scm_run_finalizers ()` periodically.

Unlike most other Guile functions, you can call `scm_set_automatic_finalization_enabled` before Guile has been initialized.

Return the previous status of automatic finalization.

C Function: `int` **scm\_run\_finalizers** `(void)` [¶](06_20_foreign_objects.md)

Invoke any pending finalizers. Returns the number of finalizers that were invoked. This function should be called when automatic finalization is disabled, though it may be called if it is enabled as well.

C Function: `void` **scm\_assert\_foreign\_object\_type** `(SCM type, SCM val)` [¶](06_20_foreign_objects.md)

When val is a foreign object of the given type, do nothing. Otherwise, signal an error.

C Function: `SCM` **scm\_make\_foreign\_object\_0** `(SCM type)` [¶](06_20_foreign_objects.md)

C Function: `SCM` **scm\_make\_foreign\_object\_1** `(SCM type, void *val0)` [¶](06_20_foreign_objects.md)

C Function: `SCM` **scm\_make\_foreign\_object\_2** `(SCM type, void *val0, void *val1)` [¶](06_20_foreign_objects.md)

C Function: `SCM` **scm\_make\_foreign\_object\_3** `(SCM type, void *val0, void *val1, void *val2)` [¶](06_20_foreign_objects.md)

C Function: `SCM` **scm\_make\_foreign\_object\_n** `(SCM type, size_t n, void *vals[])` [¶](06_20_foreign_objects.md)

Make a new foreign object of the type with type type and initialize the first n fields to the given values, as appropriate.

The number of fields for objects of a given type is fixed when the type is created. It is an error to give more initializers than there are fields in the value. It is perfectly fine to give fewer initializers than needed; this is convenient when some fields are of non-pointer types, and would be easier to initialize with the setters described below.

C Function: `void*` **scm\_foreign\_object\_ref** `(SCM obj, size_t n);` [¶](06_20_foreign_objects.md)

C Function: `scm_t_bits` **scm\_foreign\_object\_unsigned\_ref** `(SCM obj, size_t n);` [¶](06_20_foreign_objects.md)

C Function: `scm_t_signed_bits` **scm\_foreign\_object\_signed\_ref** `(SCM obj, size_t n);` [¶](06_20_foreign_objects.md)

Return the value of the nth field of the foreign object obj. The backing store for the fields is as wide as a `scm_t_bits` value, which is at least as wide as a pointer. The different variants handle casting in a portable way.

C Function: `void` **scm\_foreign\_object\_set\_x** `(SCM obj, size_t n, void *val);` [¶](06_20_foreign_objects.md)

C Function: `void` **scm\_foreign\_object\_unsigned\_set\_x** `(SCM obj, size_t n, scm_t_bits val);` [¶](06_20_foreign_objects.md)

C Function: `void` **scm\_foreign\_object\_signed\_set\_x** `(SCM obj, size_t n, scm_t_signed_bits val);` [¶](06_20_foreign_objects.md)

Set the value of the nth field of the foreign object obj to val, after portably converting to a `scm_t_bits` value, if needed.

One can also access foreign objects from Scheme. See [Foreign Objects and Scheme](05_programming_in_c.md#555-foreign-objects-and-scheme), for some examples.

(use-modules (system foreign-object))

Scheme Procedure: **make-foreign-object-type** name slots \[#:finalizer=#f\] \[#:supers=’()\] [¶](06_20_foreign_objects.md)

Make a new foreign object type. See the above documentation for `scm_make_foreign_object_type`; these functions are exactly equivalent, except for the way in which the finalizer gets attached to instances (an internal detail), and the fact that this function accepts an optional list of superclasses, which will be paseed to `make-class`.

The resulting value is a GOOPS class. See [GOOPS](08_00_goops.md#8-goops), for more on classes in Guile.

Scheme Syntax: **define-foreign-object-type** name constructor (slot ...) \[#:finalizer=#f\] [¶](06_20_foreign_objects.md)

A convenience macro to define a type, using `make-foreign-object-type`, and bind it to name. A constructor will be bound to constructor, and getters will be bound to each of slot....

* * *

Next: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots), Previous: [Foreign Objects](06_20_foreign_objects.md#620-foreign-objects), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
