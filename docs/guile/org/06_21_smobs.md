### 6.21 Smobs [¶](06_21_smobs.md#621-smobs)

A _smob_ is a “small object”. Before foreign objects were introduced in Guile 2.0.12 (see [Foreign Objects](06_20_foreign_objects.md#620-foreign-objects)), smobs were the preferred way to for C code to define new kinds of Scheme objects. With the exception of the so-called “applicable SMOBs” discussed below, smobs are now a legacy interface and are headed for eventual deprecation. See [Deprecation](06_02_deprecation.md#62-deprecation). New code should use the foreign object interface.

This section contains reference information related to defining and working with smobs. For a tutorial-like introduction to smobs, see “Defining New Types (Smobs)” in previous versions of this manual.

Function: `scm_t_bits` **scm\_make\_smob\_type** `(const char *name, size_t size)` [¶](06_21_smobs.md)

This function adds a new smob type, named name, with instance size size, to the system. The return value is a tag that is used in creating instances of the type.

If size is 0, the default _free_ function will do nothing.

If size is not 0, the default _free_ function will deallocate the memory block pointed to by `SCM_SMOB_DATA` with `scm_gc_free`. The what parameter in the call to `scm_gc_free` will be name.

Default values are provided for the _mark_, _free_, _print_, and _equalp_ functions. If you want to customize any of these functions, the call to `scm_make_smob_type` should be immediately followed by calls to one or several of `scm_set_smob_mark`, `scm_set_smob_free`, `scm_set_smob_print`, and/or `scm_set_smob_equalp`.

C Function: `void` **scm\_set\_smob\_free** `(scm_t_bits tc, size_t (*free) (SCM obj))` [¶](06_21_smobs.md)

This function sets the smob freeing procedure (sometimes referred to as a _finalizer_) for the smob type specified by the tag tc. tc is the tag returned by `scm_make_smob_type`.

The free procedure must deallocate all resources that are directly associated with the smob instance obj. It must assume that all `SCM` values that it references have already been freed and are thus invalid.

It must also not call any libguile function or macro except `scm_gc_free`, `SCM_SMOB_FLAGS`, `SCM_SMOB_DATA`, `SCM_SMOB_DATA_2`, and `SCM_SMOB_DATA_3`.

The free procedure must return 0.

Note that defining a freeing procedure is not necessary if the resources associated with obj consists only of memory allocated with `scm_gc_malloc` or `scm_gc_malloc_pointerless` because this memory is automatically reclaimed by the garbage collector when it is no longer needed (see [`scm_gc_malloc`](06_17_memory_management_and_garbage_collection.md#6172-memory-blocks)).

Smob free functions must be thread-safe. See [Foreign Object Memory Management](05_programming_in_c.md#554-foreign-object-memory-management), for a discussion on finalizers and concurrency. If you are embedding Guile in an application that is not thread-safe, and you define smob types that need finalization, you might want to disable automatic finalization, and arrange to call `scm_manually_run_finalizers ()` yourself. See [Foreign Objects](06_20_foreign_objects.md#620-foreign-objects).

C Function: `void` **scm\_set\_smob\_mark** `(scm_t_bits tc, SCM (*mark) (SCM obj))` [¶](06_21_smobs.md)

This function sets the smob marking procedure for the smob type specified by the tag tc. tc is the tag returned by `scm_make_smob_type`.

Defining a marking procedure is almost always the wrong thing to do. It is much, much preferable to allocate smob data with the `scm_gc_malloc` and `scm_gc_malloc_pointerless` functions, and allow the GC to trace pointers automatically.

Any mark procedures you see currently almost surely date from the time of Guile 1.8, before the switch to the Boehm-Demers-Weiser collector. Such smob implementations should be changed to just use `scm_gc_malloc` and friends, and to lose their mark function.

If you decide to keep the mark function, note that it may be called on objects that are on the free list. Please read and digest the comments from the BDW GC’s `gc/gc_mark.h` header.

The mark procedure must cause `scm_gc_mark` to be called for every `SCM` value that is directly referenced by the smob instance obj. One of these `SCM` values can be returned from the procedure and Guile will call `scm_gc_mark` for it. This can be used to avoid deep recursions for smob instances that form a list.

It must not call any libguile function or macro except `scm_gc_mark`, `SCM_SMOB_FLAGS`, `SCM_SMOB_DATA`, `SCM_SMOB_DATA_2`, and `SCM_SMOB_DATA_3`.

C Function: `void` **scm\_set\_smob\_print** `(scm_t_bits tc, int (*print) (SCM obj, SCM port, scm_print_state* pstate))` [¶](06_21_smobs.md)

This function sets the smob printing procedure for the smob type specified by the tag tc. tc is the tag returned by `scm_make_smob_type`.

The print procedure should output a textual representation of the smob instance obj to port, using information in pstate.

The textual representation should be of the form `#<name ...>`. This ensures that `read` will not interpret it as some other Scheme value.

It is often best to ignore pstate and just print to port with `scm_display`, `scm_write`, `scm_simple_format`, and `scm_puts`.

C Function: `void` **scm\_set\_smob\_equalp** `(scm_t_bits tc, SCM (*equalp) (SCM obj1, SCM obj2))` [¶](06_21_smobs.md)

This function sets the smob equality-testing predicate for the smob type specified by the tag tc. tc is the tag returned by `scm_make_smob_type`.

The equalp procedure should return `SCM_BOOL_T` when obj1 is `equal?` to obj2. Else it should return `SCM_BOOL_F`. Both obj1 and obj2 are instances of the smob type tc.

C Function: `void` **scm\_assert\_smob\_type** `(scm_t_bits tag, SCM val)` [¶](06_21_smobs.md)

When val is a smob of the type indicated by tag, do nothing. Else, signal an error.

C Macro: `int` **SCM\_SMOB\_PREDICATE** `(scm_t_bits tag, SCM exp)` [¶](06_21_smobs.md)

Return true if exp is a smob instance of the type indicated by tag, or false otherwise. The expression exp can be evaluated more than once, so it shouldn’t contain any side effects.

C Function: `SCM` **scm\_new\_smob** `(scm_t_bits tag, void *data)` [¶](06_21_smobs.md)

C Function: `SCM` **scm\_new\_double\_smob** `(scm_t_bits tag, void *data, void *data2, void *data3)` [¶](06_21_smobs.md)

Make a new smob of the type with tag tag and smob data data, data2, and data3, as appropriate.

The tag is what has been returned by `scm_make_smob_type`. The initial values data, data2, and data3 are of type `scm_t_bits`; when you want to use them for `SCM` values, these values need to be converted to a `scm_t_bits` first by using `SCM_UNPACK`.

The flags of the smob instance start out as zero.

C Macro: `scm_t_bits` **SCM\_SMOB\_FLAGS** `(SCM obj)` [¶](06_21_smobs.md)

Return the 16 extra bits of the smob obj. No meaning is predefined for these bits, you can use them freely.

C Macro: `scm_t_bits` **SCM\_SET\_SMOB\_FLAGS** `(SCM obj, scm_t_bits flags)` [¶](06_21_smobs.md)

Set the 16 extra bits of the smob obj to flags. No meaning is predefined for these bits, you can use them freely.

C Macro: `scm_t_bits` **SCM\_SMOB\_DATA** `(SCM obj)` [¶](06_21_smobs.md)

C Macro: `scm_t_bits` **SCM\_SMOB\_DATA\_2** `(SCM obj)` [¶](06_21_smobs.md)

C Macro: `scm_t_bits` **SCM\_SMOB\_DATA\_3** `(SCM obj)` [¶](06_21_smobs.md)

Return the first (second, third) immediate word of the smob obj as a `scm_t_bits` value. When the word contains a `SCM` value, use `SCM_SMOB_OBJECT` (etc.) instead.

C Macro: `void` **SCM\_SET\_SMOB\_DATA** `(SCM obj, scm_t_bits val)` [¶](06_21_smobs.md)

C Macro: `void` **SCM\_SET\_SMOB\_DATA\_2** `(SCM obj, scm_t_bits val)` [¶](06_21_smobs.md)

C Macro: `void` **SCM\_SET\_SMOB\_DATA\_3** `(SCM obj, scm_t_bits val)` [¶](06_21_smobs.md)

Set the first (second, third) immediate word of the smob obj to val. When the word should be set to a `SCM` value, use `SCM_SMOB_SET_OBJECT` (etc.) instead.

C Macro: `SCM` **SCM\_SMOB\_OBJECT** `(SCM obj)` [¶](06_21_smobs.md)

C Macro: `SCM` **SCM\_SMOB\_OBJECT\_2** `(SCM obj)` [¶](06_21_smobs.md)

C Macro: `SCM` **SCM\_SMOB\_OBJECT\_3** `(SCM obj)` [¶](06_21_smobs.md)

Return the first (second, third) immediate word of the smob obj as a `SCM` value. When the word contains a `scm_t_bits` value, use `SCM_SMOB_DATA` (etc.) instead.

C Macro: `void` **SCM\_SET\_SMOB\_OBJECT** `(SCM obj, SCM val)` [¶](06_21_smobs.md)

C Macro: `void` **SCM\_SET\_SMOB\_OBJECT\_2** `(SCM obj, SCM val)` [¶](06_21_smobs.md)

C Macro: `void` **SCM\_SET\_SMOB\_OBJECT\_3** `(SCM obj, SCM val)` [¶](06_21_smobs.md)

Set the first (second, third) immediate word of the smob obj to val. When the word should be set to a `scm_t_bits` value, use `SCM_SMOB_SET_DATA` (etc.) instead.

C Macro: `SCM *` **SCM\_SMOB\_OBJECT\_LOC** `(SCM obj)` [¶](06_21_smobs.md)

C Macro: `SCM *` **SCM\_SMOB\_OBJECT\_2\_LOC** `(SCM obj)` [¶](06_21_smobs.md)

C Macro: `SCM *` **SCM\_SMOB\_OBJECT\_3\_LOC** `(SCM obj)` [¶](06_21_smobs.md)

Return a pointer to the first (second, third) immediate word of the smob obj. Note that this is a pointer to `SCM`. If you need to work with `scm_t_bits` values, use `SCM_PACK` and `SCM_UNPACK`, as appropriate.

Function: `SCM` **scm\_markcdr** `(SCM x)` [¶](06_21_smobs.md)

Mark the references in the smob x, assuming that x’s first data word contains an ordinary Scheme object, and x refers to no other objects. This function simply returns x’s first data word.

* * *

Next: [Configuration, Features and Runtime Options](06_23_configuration_features_and_runtime_options.md#623-configuration-features-and-runtime-options), Previous: [Smobs](06_21_smobs.md#621-smobs), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
