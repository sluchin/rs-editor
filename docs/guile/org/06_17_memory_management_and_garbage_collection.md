### 6.17 Memory Management and Garbage Collection [¶](06_17_memory_management_and_garbage_collection.md#617-memory-management-and-garbage-collection)

Guile uses a _garbage collector_ to manage most of its objects. While the garbage collector is designed to be mostly invisible, you sometimes need to interact with it explicitly.

See [Garbage Collection](05_programming_in_c.md#542-garbage-collection) for a general discussion of how garbage collection relates to using Guile from C.

*   [Function related to Garbage Collection](06_17_memory_management_and_garbage_collection.md#6171-function-related-to-garbage-collection)
*   [Memory Blocks](06_17_memory_management_and_garbage_collection.md#6172-memory-blocks)
*   [Weak References](06_17_memory_management_and_garbage_collection.md#6173-weak-references)
*   [Guardians](06_17_memory_management_and_garbage_collection.md#6174-guardians)

* * *

Next: [Memory Blocks](06_17_memory_management_and_garbage_collection.md#6172-memory-blocks), Up: [Memory Management and Garbage Collection](06_17_memory_management_and_garbage_collection.md#617-memory-management-and-garbage-collection)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.17.1 Function related to Garbage Collection [¶](06_17_memory_management_and_garbage_collection.md#6171-function-related-to-garbage-collection)

Scheme Procedure: **gc** [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_gc** () [¶](06_17_memory_management_and_garbage_collection.md)

Finds all of the “live” `SCM` objects and reclaims for further use those that are no longer accessible. You normally don’t need to call this function explicitly. Its functionality is invoked automatically as needed.

C Function: `SCM` **scm\_gc\_protect\_object** `(SCM obj)` [¶](06_17_memory_management_and_garbage_collection.md)

Protects obj from being freed by the garbage collector, when it otherwise might be. When you are done with the object, call `scm_gc_unprotect_object` on the object. Calls to `scm_gc_protect_object`/`scm_gc_unprotect_object` can be nested, and the object remains protected until it has been unprotected as many times as it was protected. It is an error to unprotect an object more times than it has been protected. Returns the SCM object it was passed.

Note that storing obj in a C global variable has the same effect[18](99_footnotes.md).

C Function: `SCM` **scm\_gc\_unprotect\_object** `(SCM obj)` [¶](06_17_memory_management_and_garbage_collection.md)

Unprotects an object from the garbage collector which was protected by `scm_gc_unprotect_object`. Returns the SCM object it was passed.

C Function: `SCM` **scm\_permanent\_object** `(SCM obj)` [¶](06_17_memory_management_and_garbage_collection.md)

Similar to `scm_gc_protect_object` in that it causes the collector to always mark the object, except that it should not be nested (only call `scm_permanent_object` on an object once), and it has no corresponding unpermanent function. Once an object is declared permanent, it will never be freed. Returns the SCM object it was passed.

C Macro: `void` **scm\_remember\_upto\_here\_1** `(SCM obj)` [¶](06_17_memory_management_and_garbage_collection.md)

C Macro: `void` **scm\_remember\_upto\_here\_2** `(SCM obj1, SCM obj2)` [¶](06_17_memory_management_and_garbage_collection.md)

Create a reference to the given object or objects, so they’re certain to be present on the stack or in a register and hence will not be freed by the garbage collector before this point.

Note that these functions can only be applied to ordinary C local variables (ie. “automatics”). Objects held in global or static variables or some malloced block or the like cannot be protected with this mechanism.

Scheme Procedure: **gc-stats** [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_gc\_stats** () [¶](06_17_memory_management_and_garbage_collection.md)

Return an association list of statistics about Guile’s current use of storage.

Scheme Procedure: **gc-live-object-stats** [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_gc\_live\_object\_stats** () [¶](06_17_memory_management_and_garbage_collection.md)

Return an alist of statistics of the current live objects.

Function: `void` **scm\_gc\_mark** `(SCM x)` [¶](06_17_memory_management_and_garbage_collection.md)

Mark the object x, and recurse on any objects x refers to. If x’s mark bit is already set, return immediately. This function must only be called during the mark-phase of garbage collection, typically from a smob _mark_ function.

* * *

Next: [Weak References](06_17_memory_management_and_garbage_collection.md#6173-weak-references), Previous: [Function related to Garbage Collection](06_17_memory_management_and_garbage_collection.md#6171-function-related-to-garbage-collection), Up: [Memory Management and Garbage Collection](06_17_memory_management_and_garbage_collection.md#617-memory-management-and-garbage-collection)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.17.2 Memory Blocks [¶](06_17_memory_management_and_garbage_collection.md#6172-memory-blocks)

In C programs, dynamic management of memory blocks is normally done with the functions malloc, realloc, and free. Guile has additional functions for dynamic memory allocation that are integrated into the garbage collector and the error reporting system.

Memory blocks that are associated with Scheme objects (for example a foreign object) should be allocated with `scm_gc_malloc` or `scm_gc_malloc_pointerless`. These two functions will either return a valid pointer or signal an error. Memory blocks allocated this way may be released explicitly; however, this is not strictly needed, and we recommend _not_ calling `scm_gc_free`. All memory allocated with `scm_gc_malloc` or `scm_gc_malloc_pointerless` is automatically reclaimed when the garbage collector no longer sees any live reference to it[19](99_footnotes.md).

When garbage collection occurs, Guile will visit the words in memory allocated with `scm_gc_malloc`, looking for live pointers. This means that if `scm_gc_malloc`\-allocated memory contains a pointer to some other part of the memory, the garbage collector notices it and prevents it from being reclaimed[20](99_footnotes.md). Conversely, memory allocated with `scm_gc_malloc_pointerless` is assumed to be “pointer-less” and is not scanned for pointers.

For memory that is not associated with a Scheme object, you can use `scm_malloc` instead of `malloc`. Like `scm_gc_malloc`, it will either return a valid pointer or signal an error. However, it will not assume that the new memory block can be freed by a garbage collection. The memory must be explicitly freed with `free`.

There is also `scm_gc_realloc` and `scm_realloc`, to be used in place of `realloc` when appropriate, and `scm_gc_calloc` and `scm_calloc`, to be used in place of `calloc` when appropriate.

The function `scm_dynwind_free` can be useful when memory should be freed with libc’s `free` when leaving a dynwind context, See [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind).

C Function: `void *` **scm\_malloc** `(size_t size)` [¶](06_17_memory_management_and_garbage_collection.md)

C Function: `void *` **scm\_calloc** `(size_t size)` [¶](06_17_memory_management_and_garbage_collection.md)

Allocate size bytes of memory and return a pointer to it. When size is 0, return `NULL`. When not enough memory is available, signal an error. This function runs the GC to free up some memory when it deems it appropriate.

The memory is allocated by the libc `malloc` function and can be freed with `free`. There is no `scm_free` function to go with `scm_malloc` to make it easier to pass memory back and forth between different modules.

The function `scm_calloc` is similar to `scm_malloc`, but initializes the block of memory to zero as well.

These functions will (indirectly) call `scm_gc_register_allocation`.

C Function: `void *` **scm\_realloc** `(void *mem, size_t new_size)` [¶](06_17_memory_management_and_garbage_collection.md)

Change the size of the memory block at mem to new\_size and return its new location. When new\_size is 0, this is the same as calling `free` on mem and `NULL` is returned. When mem is `NULL`, this function behaves like `scm_malloc` and allocates a new block of size new\_size.

When not enough memory is available, signal an error. This function runs the GC to free up some memory when it deems it appropriate.

This function will call `scm_gc_register_allocation`.

C Function: `void *` **scm\_gc\_malloc** `(size_t size, const char *what)` [¶](06_17_memory_management_and_garbage_collection.md)

C Function: `void *` **scm\_gc\_malloc\_pointerless** `(size_t size, const char *what)` [¶](06_17_memory_management_and_garbage_collection.md)

C Function: `void *` **scm\_gc\_realloc** `(void *mem, size_t old_size, size_t new_size, const char *what);` [¶](06_17_memory_management_and_garbage_collection.md)

C Function: `void *` **scm\_gc\_calloc** `(size_t size, const char *what)` [¶](06_17_memory_management_and_garbage_collection.md)

Allocate size bytes of automatically-managed memory. The memory is automatically freed when no longer referenced from any live memory block.

When garbage collection occurs, Guile will visit the words in memory allocated with `scm_gc_malloc` or `scm_gc_calloc`, looking for pointers to other memory allocations that are managed by the GC. In contrast, memory allocated by `scm_gc_malloc_pointerless` is not scanned for pointers.

The `scm_gc_realloc` call preserves the “pointerlessness” of the memory area pointed to by mem. Note that you need to pass the old size of a reallocated memory block as well. See below for a motivation.

C Function: `void` **scm\_gc\_free** `(void *mem, size_t size, const char *what)` [¶](06_17_memory_management_and_garbage_collection.md)

Explicitly free the memory block pointed to by mem, which was previously allocated by one of the above `scm_gc` functions. This function is almost always unnecessary, except for codebases that still need to compile on Guile 1.8.

Note that you need to explicitly pass the size parameter. This is done since it should normally be easy to provide this parameter (for memory that is associated with GC controlled objects) and help keep the memory management overhead very low. However, in Guile 2.x, size is always ignored.

C Function: `void` **scm\_gc\_register\_allocation** `(size_t size)` [¶](06_17_memory_management_and_garbage_collection.md)

Informs the garbage collector that size bytes have been allocated, which the collector would otherwise not have known about.

In general, Scheme will decide to collect garbage only after some amount of memory has been allocated. Calling this function will make the Scheme garbage collector know about more allocation, and thus run more often (as appropriate).

It is especially important to call this function when large unmanaged allocations, like images, may be freed by small Scheme allocations, like foreign objects.

C Function: `void` **scm\_dynwind\_free** `(void *mem)` [¶](06_17_memory_management_and_garbage_collection.md)

Equivalent to `scm_dynwind_unwind_handler (free, mem, SCM_F_WIND_EXPLICITLY)`. That is, the memory block at mem will be freed (using `free` from the C library) when the current dynwind is left.

Scheme Procedure: **malloc-stats** [¶](06_17_memory_management_and_garbage_collection.md)

Return an alist ((what . n) ...) describing number of malloced objects. what is the second argument to `scm_gc_malloc`, n is the number of objects of that type currently allocated.

This function is only available if the `GUILE_DEBUG_MALLOC` preprocessor macro was defined when Guile was compiled.

* * *

Next: [Guardians](06_17_memory_management_and_garbage_collection.md#6174-guardians), Previous: [Memory Blocks](06_17_memory_management_and_garbage_collection.md#6172-memory-blocks), Up: [Memory Management and Garbage Collection](06_17_memory_management_and_garbage_collection.md#617-memory-management-and-garbage-collection)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.17.3 Weak References [¶](06_17_memory_management_and_garbage_collection.md#6173-weak-references)

\[FIXME: This chapter is based on Mikael Djurfeldt’s answer to a question by Michael Livshin. Any mistakes are not theirs, of course. \]

Weak references let you attach bookkeeping information to data so that the additional information automatically disappears when the original data is no longer in use and gets garbage collected. In a weak key hash, the hash entry for that key disappears as soon as the key is no longer referenced from anywhere else. For weak value hashes, the same happens as soon as the value is no longer in use. Entries in a doubly weak hash disappear when either the key or the value are not used anywhere else anymore.

Object properties offer the same kind of functionality as weak key hashes in many situations. (see [Object Properties](06_09_general_utility_functions.md#692-object-properties))

Here’s an example (a little bit strained perhaps, but one of the examples is actually used in Guile):

Assume that you’re implementing a debugging system where you want to associate information about filename and position of source code expressions with the expressions themselves.

Hashtables can be used for that, but if you use ordinary hash tables it will be impossible for the scheme interpreter to "forget" old source when, for example, a file is reloaded.

To implement the mapping from source code expressions to positional information it is necessary to use weak-key tables since we don’t want the expressions to be remembered just because they are in our table.

To implement a mapping from source file line numbers to source code expressions you would use a weak-value table.

To implement a mapping from source code expressions to the procedures they constitute a doubly-weak table has to be used.

*   [Weak hash tables](06_17_memory_management_and_garbage_collection.md#61731-weak-hash-tables)
*   [Weak vectors](06_17_memory_management_and_garbage_collection.md#61732-weak-vectors)

* * *

Next: [Weak vectors](06_17_memory_management_and_garbage_collection.md#61732-weak-vectors), Up: [Weak References](06_17_memory_management_and_garbage_collection.md#6173-weak-references)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.17.3.1 Weak hash tables [¶](06_17_memory_management_and_garbage_collection.md#61731-weak-hash-tables)

Scheme Procedure: **make-weak-key-hash-table** \[size\] [¶](06_17_memory_management_and_garbage_collection.md)

Scheme Procedure: **make-weak-value-hash-table** \[size\] [¶](06_17_memory_management_and_garbage_collection.md)

Scheme Procedure: **make-doubly-weak-hash-table** \[size\] [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_make\_weak\_key\_hash\_table** (size) [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_make\_weak\_value\_hash\_table** (size) [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_make\_doubly\_weak\_hash\_table** (size) [¶](06_17_memory_management_and_garbage_collection.md)

Return a weak hash table with size buckets. As with any hash table, choosing a good size for the table requires some caution.

You can modify weak hash tables in exactly the same way you would modify regular hash tables, with the exception of the routines that act on handles. Weak tables have a different implementation behind the scenes that doesn’t have handles. see [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables), for more on `hashq-ref` et al.

Note that in a weak-key hash table, the reference to the value is strong. This means that if the value references the key, even indirectly, the key will never be collected, which can lead to a memory leak. The reverse is true for weak value tables.

Scheme Procedure: **weak-key-hash-table?** obj [¶](06_17_memory_management_and_garbage_collection.md)

Scheme Procedure: **weak-value-hash-table?** obj [¶](06_17_memory_management_and_garbage_collection.md)

Scheme Procedure: **doubly-weak-hash-table?** obj [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_weak\_key\_hash\_table\_p** (obj) [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_weak\_value\_hash\_table\_p** (obj) [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_doubly\_weak\_hash\_table\_p** (obj) [¶](06_17_memory_management_and_garbage_collection.md)

Return `#t` if obj is the specified weak hash table. Note that a doubly weak hash table is neither a weak key nor a weak value hash table.

* * *

Previous: [Weak hash tables](06_17_memory_management_and_garbage_collection.md#61731-weak-hash-tables), Up: [Weak References](06_17_memory_management_and_garbage_collection.md#6173-weak-references)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.17.3.2 Weak vectors [¶](06_17_memory_management_and_garbage_collection.md#61732-weak-vectors)

Scheme Procedure: **make-weak-vector** size \[fill\] [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_make\_weak\_vector** (size, fill) [¶](06_17_memory_management_and_garbage_collection.md)

Return a weak vector with size elements. If the optional argument fill is given, all entries in the vector will be set to fill. The default value for fill is the empty list.

Scheme Procedure: **weak-vector** elem … [¶](06_17_memory_management_and_garbage_collection.md)

Scheme Procedure: **list->weak-vector** l [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_weak\_vector** (l) [¶](06_17_memory_management_and_garbage_collection.md)

Construct a weak vector from a list: `weak-vector` uses the list of its arguments while `list->weak-vector` uses its only argument l (a list) to construct a weak vector the same way `list->vector` would.

Scheme Procedure: **weak-vector?** obj [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_weak\_vector\_p** (obj) [¶](06_17_memory_management_and_garbage_collection.md)

Return `#t` if obj is a weak vector.

Scheme Procedure: **weak-vector-ref** wvect k [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_weak\_vector\_ref** (wvect, k) [¶](06_17_memory_management_and_garbage_collection.md)

Return the kth element of the weak vector wvect, or `#f` if that element has been collected.

Scheme Procedure: **weak-vector-set!** wvect k elt [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_weak\_vector\_set\_x** (wvect, k, elt) [¶](06_17_memory_management_and_garbage_collection.md)

Set the kth element of the weak vector wvect to elt.

* * *

Previous: [Weak References](06_17_memory_management_and_garbage_collection.md#6173-weak-references), Up: [Memory Management and Garbage Collection](06_17_memory_management_and_garbage_collection.md#617-memory-management-and-garbage-collection)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.17.4 Guardians [¶](06_17_memory_management_and_garbage_collection.md#6174-guardians)

Guardians provide a way to be notified about objects that would otherwise be collected as garbage. Guarding them prevents the objects from being collected and cleanup actions can be performed on them, for example.

See R. Kent Dybvig, Carl Bruggeman, and David Eby (1993) "Guardians in a Generation-Based Garbage Collector". ACM SIGPLAN Conference on Programming Language Design and Implementation, June 1993.

Scheme Procedure: **make-guardian** [¶](06_17_memory_management_and_garbage_collection.md)

C Function: **scm\_make\_guardian** () [¶](06_17_memory_management_and_garbage_collection.md)

Create a new guardian. A guardian protects a set of objects from garbage collection, allowing a program to apply cleanup or other actions.

`make-guardian` returns a procedure representing the guardian. Calling the guardian procedure with an argument adds the argument to the guardian’s set of protected objects. Calling the guardian procedure without an argument returns one of the protected objects which are ready for garbage collection, or `#f` if no such object is available. Objects which are returned in this way are removed from the guardian.

You can put a single object into a guardian more than once and you can put a single object into more than one guardian. The object will then be returned multiple times by the guardian procedures.

An object is eligible to be returned from a guardian when it is no longer referenced from outside any guardian.

There is no guarantee about the order in which objects are returned from a guardian. If you want to impose an order on finalization actions, for example, you can do that by keeping objects alive in some global data structure until they are no longer needed for finalizing other objects.

Being an element in a weak vector, a key in a hash table with weak keys, or a value in a hash table with weak values does not prevent an object from being returned by a guardian. But as long as an object can be returned from a guardian it will not be removed from such a weak vector or hash table. In other words, a weak link does not prevent an object from being considered collectible, but being inside a guardian prevents a weak link from being broken.

A key in a weak key hash table can be thought of as having a strong reference to its associated value as long as the key is accessible. Consequently, when the key is only accessible from within a guardian, the reference from the key to the value is also considered to be coming from within a guardian. Thus, if there is no other reference to the value, it is eligible to be returned from a guardian.

* * *

Next: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface), Previous: [Memory Management and Garbage Collection](06_17_memory_management_and_garbage_collection.md#617-memory-management-and-garbage-collection), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
