### 9.2 Data Representation [¶](09_02_data_representation.md#92-data-representation)

Scheme is a latently-typed language; this means that the system cannot, in general, determine the type of a given expression at compile time. Types only become apparent at run time. Variables do not have fixed types; a variable may hold a pair at one point, an integer at the next, and a thousand-element vector later. Instead, values, not variables, have fixed types.

In order to implement standard Scheme functions like `pair?` and `string?` and provide garbage collection, the representation of every value must contain enough information to accurately determine its type at run time. Often, Scheme systems also use this information to determine whether a program has attempted to apply an operation to an inappropriately typed value (such as taking the `car` of a string).

Because variables, pairs, and vectors may hold values of any type, Scheme implementations use a uniform representation for values — a single type large enough to hold either a complete value or a pointer to a complete value, along with the necessary typing information.

The following sections will present a simple typing system, and then make some refinements to correct its major weaknesses. We then conclude with a discussion of specific choices that Guile has made regarding garbage collection and data representation.

*   [A Simple Representation](09_02_data_representation.md#921-a-simple-representation)
*   [Faster Integers](09_02_data_representation.md#922-faster-integers)
*   [Cheaper Pairs](09_02_data_representation.md#923-cheaper-pairs)
*   [Conservative Garbage Collection](09_02_data_representation.md#924-conservative-garbage-collection)
*   [The SCM Type in Guile](09_02_data_representation.md#925-the-scm-type-in-guile)

* * *

Next: [Faster Integers](09_02_data_representation.md#922-faster-integers), Up: [Data Representation](09_02_data_representation.md#92-data-representation)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.1 A Simple Representation [¶](09_02_data_representation.md#921-a-simple-representation)

The simplest way to represent Scheme values in C would be to represent each value as a pointer to a structure containing a type indicator, followed by a union carrying the real value. Assuming that `SCM` is the name of our universal type, we can write:

enum type { integer, pair, string, vector, ... };

typedef struct value \*SCM;

struct value {
  enum type type;
  union {
    int integer;
    struct { SCM car, cdr; } pair;
    struct { int length; char \*elts; } string;
    struct { int length; SCM  \*elts; } vector;
    ...
  } value;
};

with the ellipses replaced with code for the remaining Scheme types.

This representation is sufficient to implement all of Scheme’s semantics. If x is an `SCM` value:

*   To test if x is an integer, we can write `x->type == integer`.
*   To find its value, we can write `x->value.integer`.
*   To test if x is a vector, we can write `x->type == vector`.
*   If we know x is a vector, we can write `x->value.vector.elts[0]` to refer to its first element.
*   If we know x is a pair, we can write `x->value.pair.car` to extract its car.

* * *

Next: [Cheaper Pairs](09_02_data_representation.md#923-cheaper-pairs), Previous: [A Simple Representation](09_02_data_representation.md#921-a-simple-representation), Up: [Data Representation](09_02_data_representation.md#92-data-representation)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.2 Faster Integers [¶](09_02_data_representation.md#922-faster-integers)

Unfortunately, the above representation has a serious disadvantage. In order to return an integer, an expression must allocate a `struct value`, initialize it to represent that integer, and return a pointer to it. Furthermore, fetching an integer’s value requires a memory reference, which is much slower than a register reference on most processors. Since integers are extremely common, this representation is too costly, in both time and space. Integers should be very cheap to create and manipulate.

One possible solution comes from the observation that, on many architectures, heap-allocated data (i.e., what you get when you call `malloc`) must be aligned on an eight-byte boundary. (Whether or not the machine actually requires it, we can write our own allocator for `struct value` objects that assures this is true.) In this case, the lower three bits of the structure’s address are known to be zero.

This gives us the room we need to provide an improved representation for integers. We make the following rules:

*   If the lower three bits of an `SCM` value are zero, then the SCM value is a pointer to a `struct value`, and everything proceeds as before.
*   Otherwise, the `SCM` value represents an integer, whose value appears in its upper bits.

Here is C code implementing this convention:

enum type { pair, string, vector, ... };

typedef struct value \*SCM;

struct value {
  enum type type;
  union {
    struct { SCM car, cdr; } pair;
    struct { int length; char \*elts; } string;
    struct { int length; SCM  \*elts; } vector;
    ...
  } value;
};

#define POINTER\_P(x) (((int) (x) & 7) == 0)
#define INTEGER\_P(x) (! POINTER\_P (x))

#define GET\_INTEGER(x)  ((int) (x) >> 3)
#define MAKE\_INTEGER(x) ((SCM) (((x) << 3) | 1))

Notice that `integer` no longer appears as an element of `enum type`, and the union has lost its `integer` member. Instead, we use the `POINTER_P` and `INTEGER_P` macros to make a coarse classification of values into integers and non-integers, and do further type testing as before.

Here’s how we would answer the questions posed above (again, assume x is an `SCM` value):

*   To test if x is an integer, we can write `INTEGER_P (x)`.
*   To find its value, we can write `GET_INTEGER (x)`.
*   To test if x is a vector, we can write:
    
      POINTER\_P (x) && x->type == vector
    
    Given the new representation, we must make sure x is truly a pointer before we dereference it to determine its complete type.
    
*   If we know x is a vector, we can write `x->value.vector.elts[0]` to refer to its first element, as before.
*   If we know x is a pair, we can write `x->value.pair.car` to extract its car, just as before.

This representation allows us to operate more efficiently on integers than the first. For example, if x and y are known to be integers, we can compute their sum as follows:

MAKE\_INTEGER (GET\_INTEGER (x) + GET\_INTEGER (y))

Now, integer math requires no allocation or memory references. Most real Scheme systems actually implement addition and other operations using an even more efficient algorithm, but this essay isn’t about bit-twiddling. (Hint: how do you decide when to overflow to a bignum? How would you do it in assembly?)

* * *

Next: [Conservative Garbage Collection](09_02_data_representation.md#924-conservative-garbage-collection), Previous: [Faster Integers](09_02_data_representation.md#922-faster-integers), Up: [Data Representation](09_02_data_representation.md#92-data-representation)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.3 Cheaper Pairs [¶](09_02_data_representation.md#923-cheaper-pairs)

However, there is yet another issue to confront. Most Scheme heaps contain more pairs than any other type of object; Jonathan Rees said at one point that pairs occupy 45% of the heap in his Scheme implementation, Scheme 48. However, our representation above spends three `SCM`\-sized words per pair — one for the type, and two for the CAR and CDR. Is there any way to represent pairs using only two words?

Let us refine the convention we established earlier. Let us assert that:

*   If the bottom three bits of an `SCM` value are `#b000`, then it is a pointer, as before.
*   If the bottom three bits are `#b001`, then the upper bits are an integer. This is a bit more restrictive than before.
*   If the bottom two bits are `#b010`, then the value, with the bottom three bits masked out, is the address of a pair.

Here is the new C code:

enum type { string, vector, ... };

typedef struct value \*SCM;

struct value {
  enum type type;
  union {
    struct { int length; char \*elts; } string;
    struct { int length; SCM  \*elts; } vector;
    ...
  } value;
};

struct pair {
  SCM car, cdr;
};

#define POINTER\_P(x) (((int) (x) & 7) == 0)

#define INTEGER\_P(x)  (((int) (x) & 7) == 1)
#define GET\_INTEGER(x)  ((int) (x) >> 3)
#define MAKE\_INTEGER(x) ((SCM) (((x) << 3) | 1))

#define PAIR\_P(x) (((int) (x) & 7) == 2)
#define GET\_PAIR(x) ((struct pair \*) ((int) (x) & ~7))

Notice that `enum type` and `struct value` now only contain provisions for vectors and strings; both integers and pairs have become special cases. The code above also assumes that an `int` is large enough to hold a pointer, which isn’t generally true.

Our list of examples is now as follows:

*   To test if x is an integer, we can write `INTEGER_P (x)`; this is as before.
*   To find its value, we can write `GET_INTEGER (x)`, as before.
*   To test if x is a vector, we can write:
    
      POINTER\_P (x) && x->type == vector
    
    We must still make sure that x is a pointer to a `struct value` before dereferencing it to find its type.
    
*   If we know x is a vector, we can write `x->value.vector.elts[0]` to refer to its first element, as before.
*   We can write `PAIR_P (x)` to determine if x is a pair, and then write `GET_PAIR (x)->car` to refer to its car.

This change in representation reduces our heap size by 15%. It also makes it cheaper to decide if a value is a pair, because no memory references are necessary; it suffices to check the bottom two bits of the `SCM` value. This may be significant when traversing lists, a common activity in a Scheme system.

Again, most real Scheme systems use a slightly different implementation; for example, if GET\_PAIR subtracts off the low bits of `x`, instead of masking them off, the optimizer will often be able to combine that subtraction with the addition of the offset of the structure member we are referencing, making a modified pointer as fast to use as an unmodified pointer.

* * *

Next: [The SCM Type in Guile](09_02_data_representation.md#925-the-scm-type-in-guile), Previous: [Cheaper Pairs](09_02_data_representation.md#923-cheaper-pairs), Up: [Data Representation](09_02_data_representation.md#92-data-representation)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.4 Conservative Garbage Collection [¶](09_02_data_representation.md#924-conservative-garbage-collection)

Aside from the latent typing, the major source of constraints on a Scheme implementation’s data representation is the garbage collector. The collector must be able to traverse every live object in the heap, to determine which objects are not live, and thus collectable.

There are many ways to implement this. Guile’s garbage collection is built on a library, the Boehm-Demers-Weiser conservative garbage collector (BDW-GC). The BDW-GC “just works”, for the most part. But since it is interesting to know how these things work, we include here a high-level description of what the BDW-GC does.

Garbage collection has two logical phases: a _mark_ phase, in which the set of live objects is enumerated, and a _sweep_ phase, in which objects not traversed in the mark phase are collected. Correct functioning of the collector depends on being able to traverse the entire set of live objects.

In the mark phase, the collector scans the system’s global variables and the local variables on the stack to determine which objects are immediately accessible by the C code. It then scans those objects to find the objects they point to, and so on. The collector logically sets a _mark bit_ on each object it finds, so each object is traversed only once.

When the collector can find no unmarked objects pointed to by marked objects, it assumes that any objects that are still unmarked will never be used by the program (since there is no path of dereferences from any global or local variable that reaches them) and deallocates them.

In the above paragraphs, we did not specify how the garbage collector finds the global and local variables; as usual, there are many different approaches. Frequently, the programmer must maintain a list of pointers to all global variables that refer to the heap, and another list (adjusted upon entry to and exit from each function) of local variables, for the collector’s benefit.

The list of global variables is usually not too difficult to maintain, since global variables are relatively rare. However, an explicitly maintained list of local variables (in the author’s personal experience) is a nightmare to maintain. Thus, the BDW-GC uses a technique called _conservative garbage collection_, to make the local variable list unnecessary.

The trick to conservative collection is to treat the C stack as an ordinary range of memory, and assume that _every_ word on the C stack is a pointer into the heap. Thus, the collector marks all objects whose addresses appear anywhere in the C stack, without knowing for sure how that word is meant to be interpreted.

In addition to the stack, the BDW-GC will also scan static data sections. This means that global variables are also scanned when looking for live Scheme objects.

Obviously, such a system will occasionally retain objects that are actually garbage, and should be freed. In practice, this is not a problem, as the set of conservatively-scanned locations is fixed; the Scheme stack is maintained apart from the C stack, and is scanned precisely (as opposed to conservatively). The GC-managed heap is also partitioned into parts that can contain pointers (such as vectors) and parts that can’t (such as bytevectors), limiting the potential for confusing a raw integer with a pointer to a live object.

Interested readers should see the BDW-GC web page at [http://www.hboehm.info/gc/](http://www.hboehm.info/gc/), for more information on conservative GC in general and the BDW-GC implementation in particular.

* * *

Previous: [Conservative Garbage Collection](09_02_data_representation.md#924-conservative-garbage-collection), Up: [Data Representation](09_02_data_representation.md#92-data-representation)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.5 The SCM Type in Guile [¶](09_02_data_representation.md#925-the-scm-type-in-guile)

Guile classifies Scheme objects into two kinds: those that fit entirely within an `SCM`, and those that require heap storage.

The former class are called _immediates_. The class of immediates includes small integers, characters, boolean values, the empty list, the mysterious end-of-file object, and some others.

The remaining types are called, not surprisingly, _non-immediates_. They include pairs, procedures, strings, vectors, and all other data types in Guile. For non-immediates, the `SCM` word contains a pointer to data on the heap, with further information about the object in question is stored in that data.

This section describes how the `SCM` type is actually represented and used at the C level. Interested readers should see `libguile/scm.h` for an exposition of how Guile stores type information.

In fact, there are two basic C data types to represent objects in Guile: `SCM` and `scm_t_bits`.

*   [Relationship Between `SCM` and `scm_t_bits`](09_02_data_representation.md#9251-relationship-between-scm-and-scm_t_bits)
*   [Immediate Objects](09_02_data_representation.md#9252-immediate-objects)
*   [Non-Immediate Objects](09_02_data_representation.md#9253-non-immediate-objects)
*   [Allocating Heap Objects](09_02_data_representation.md#9254-allocating-heap-objects)
*   [Heap Object Type Information](09_02_data_representation.md#9255-heap-object-type-information)
*   [Accessing Heap Object Fields](09_02_data_representation.md#9256-accessing-heap-object-fields)

* * *

Next: [Immediate Objects](09_02_data_representation.md#9252-immediate-objects), Up: [The SCM Type in Guile](09_02_data_representation.md#925-the-scm-type-in-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.5.1 Relationship Between `SCM` and `scm_t_bits` [¶](09_02_data_representation.md#9251-relationship-between-scm-and-scm_t_bits)

A variable of type `SCM` is guaranteed to hold a valid Scheme object. A variable of type `scm_t_bits`, on the other hand, may hold a representation of a `SCM` value as a C integral type, but may also hold any C value, even if it does not correspond to a valid Scheme object.

For a variable x of type `SCM`, the Scheme object’s type information is stored in a form that is not directly usable. To be able to work on the type encoding of the scheme value, the `SCM` variable has to be transformed into the corresponding representation as a `scm_t_bits` variable y by using the `SCM_UNPACK` macro. Once this has been done, the type of the scheme object x can be derived from the content of the bits of the `scm_t_bits` value y, in the way illustrated by the example earlier in this chapter (see [Cheaper Pairs](09_02_data_representation.md#923-cheaper-pairs)). Conversely, a valid bit encoding of a Scheme value as a `scm_t_bits` variable can be transformed into the corresponding `SCM` value using the `SCM_PACK` macro.

* * *

Next: [Non-Immediate Objects](09_02_data_representation.md#9253-non-immediate-objects), Previous: [Relationship Between `SCM` and `scm_t_bits`](09_02_data_representation.md#9251-relationship-between-scm-and-scm_t_bits), Up: [The SCM Type in Guile](09_02_data_representation.md#925-the-scm-type-in-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.5.2 Immediate Objects [¶](09_02_data_representation.md#9252-immediate-objects)

A Scheme object may either be an immediate, i.e. carrying all necessary information by itself, or it may contain a reference to a _heap object_ which is, as the name implies, data on the heap. Although in general it should be irrelevant for user code whether an object is an immediate or not, within Guile’s own code the distinction is sometimes of importance. Thus, the following low level macro is provided:

Macro: `int` **SCM\_IMP** `(SCM x)` [¶](09_02_data_representation.md)

A Scheme object is an immediate if it fulfills the `SCM_IMP` predicate, otherwise it holds an encoded reference to a heap object. The result of the predicate is delivered as a C style boolean value. User code and code that extends Guile should normally not be required to use this macro.

Summary:

*   Given a Scheme object x of unknown type, check first with `SCM_IMP (x)` if it is an immediate object.
*   If so, all of the type and value information can be determined from the `scm_t_bits` value that is delivered by `SCM_UNPACK (x)`.

There are a number of special values in Scheme, most of them documented elsewhere in this manual. It’s not quite the right place to put them, but for now, here’s a list of the C names given to some of these values:

Macro: `SCM` **SCM\_EOL** [¶](09_02_data_representation.md)

The Scheme empty list object, or “End Of List” object, usually written in Scheme as `'()`.

Macro: `SCM` **SCM\_EOF\_VAL** [¶](09_02_data_representation.md)

The Scheme end-of-file value. It has no standard written representation, for obvious reasons.

Macro: `SCM` **SCM\_UNSPECIFIED** [¶](09_02_data_representation.md)

The value returned by some (but not all) expressions that the Scheme standard says return an “unspecified” value.

This is sort of a weirdly literal way to take things, but the standard read-eval-print loop prints nothing when the expression returns this value, so it’s not a bad idea to return this when you can’t think of anything else helpful.

Macro: `SCM` **SCM\_UNDEFINED** [¶](09_02_data_representation.md)

The “undefined” value. Its most important property is that is not equal to any valid Scheme value. This is put to various internal uses by C code interacting with Guile.

For example, when you write a C function that is callable from Scheme and which takes optional arguments, the interpreter passes `SCM_UNDEFINED` for any arguments you did not receive.

We also use this to mark unbound variables.

Macro: `int` **SCM\_UNBNDP** `(SCM x)` [¶](09_02_data_representation.md)

Return true if x is `SCM_UNDEFINED`. Note that this is not a check to see if x is `SCM_UNBOUND`. History will not be kind to us.

* * *

Next: [Allocating Heap Objects](09_02_data_representation.md#9254-allocating-heap-objects), Previous: [Immediate Objects](09_02_data_representation.md#9252-immediate-objects), Up: [The SCM Type in Guile](09_02_data_representation.md#925-the-scm-type-in-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.5.3 Non-Immediate Objects [¶](09_02_data_representation.md#9253-non-immediate-objects)

A Scheme object of type `SCM` that does not fulfill the `SCM_IMP` predicate holds an encoded reference to a heap object. This reference can be decoded to a C pointer to a heap object using the `SCM_UNPACK_POINTER` macro. The encoding of a pointer to a heap object into a `SCM` value is done using the `SCM_PACK_POINTER` macro.

Before Guile 2.0, Guile had a custom garbage collector that allocated heap objects in units of 2-word _cells_. With the move to the BDW-GC collector in Guile 2.0, Guile can allocate heap objects of any size, and the concept of a cell is now obsolete. Still, we mention it here as the name still appears in various low-level interfaces.

Macro: `scm_t_bits *` **SCM\_UNPACK\_POINTER** `(SCM x)` [¶](09_02_data_representation.md)

Macro: `scm_t_cell *` **SCM2PTR** `(SCM x)` [¶](09_02_data_representation.md)

Extract and return the heap object pointer from a non-immediate `SCM` object x. The name `SCM2PTR` is deprecated but still common.

Macro: `SCM_PACK_POINTER` **(scm\_t\_bits** `* x)` [¶](09_02_data_representation.md)

Macro: `SCM` **PTR2SCM** `(scm_t_cell * x)` [¶](09_02_data_representation.md)

Return a `SCM` value that encodes a reference to the heap object pointer x. The name `PTR2SCM` is deprecated but still common.

Note that it is also possible to transform a non-immediate `SCM` value by using `SCM_UNPACK` into a `scm_t_bits` variable. However, the result of `SCM_UNPACK` may not be used as a pointer to a heap object: only `SCM_UNPACK_POINTER` is guaranteed to transform a `SCM` object into a valid pointer to a heap object. Also, it is not allowed to apply `SCM_PACK_POINTER` to anything that is not a valid pointer to a heap object.

Summary:

*   Only use `SCM_UNPACK_POINTER` on `SCM` values for which `SCM_IMP` is false!
*   Don’t use `(scm_t_cell *) SCM_UNPACK (x)`! Use `SCM_UNPACK_POINTER (x)` instead!
*   Don’t use `SCM_PACK_POINTER` for anything but a heap object pointer!

* * *

Next: [Heap Object Type Information](09_02_data_representation.md#9255-heap-object-type-information), Previous: [Non-Immediate Objects](09_02_data_representation.md#9253-non-immediate-objects), Up: [The SCM Type in Guile](09_02_data_representation.md#925-the-scm-type-in-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.5.4 Allocating Heap Objects [¶](09_02_data_representation.md#9254-allocating-heap-objects)

Heap objects are heap-allocated data pointed to by non-immediate `SCM` value. The first word of the heap object should contain a type code. The object may be any number of words in length, and is generally scanned by the garbage collector for additional unless the object was allocated using a “pointerless” allocation function.

You should generally not need these functions, unless you are implementing a new data type, and thoroughly understand the code in `<libguile/scm.h>`.

If you just want to allocate pairs, use `scm_cons`.

Function: `SCM` **scm\_words** `(scm_t_bits word_0, uint32_t n_words)` [¶](09_02_data_representation.md)

Allocate a new heap object containing n\_words, and initialize the first slot to word\_0, and return a non-immediate `SCM` value encoding a pointer to the object. Typically word\_0 will contain the type tag.

There are also deprecated but common variants of `scm_words` that use the term “cell” to indicate 2-word objects.

Function: `SCM` **scm\_cell** `(scm_t_bits word_0, scm_t_bits word_1)` [¶](09_02_data_representation.md)

Allocate a new 2-word heap object, initialize the two slots with word\_0 and word\_1, and return it. Just like calling `scm_words (word_0, 2)`, then initializing the second slot to word\_1.

Note that word\_0 and word\_1 are of type `scm_t_bits`. If you want to pass a `SCM` object, you need to use `SCM_UNPACK`.

Function: `SCM` **scm\_double\_cell** `(scm_t_bits word_0, scm_t_bits word_1, scm_t_bits word_2, scm_t_bits word_3)` [¶](09_02_data_representation.md)

Like `scm_cell`, but allocates a 4-word heap object.

* * *

Next: [Accessing Heap Object Fields](09_02_data_representation.md#9256-accessing-heap-object-fields), Previous: [Allocating Heap Objects](09_02_data_representation.md#9254-allocating-heap-objects), Up: [The SCM Type in Guile](09_02_data_representation.md#925-the-scm-type-in-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.5.5 Heap Object Type Information [¶](09_02_data_representation.md#9255-heap-object-type-information)

Heap objects contain a type tag and are followed by a number of word-sized slots. The interpretation of the object contents depends on the type of the object.

Macro: `scm_t_bits` **SCM\_CELL\_TYPE** `(SCM x)` [¶](09_02_data_representation.md)

Extract the first word of the heap object pointed to by x. This value holds the information about the cell type.

Macro: `void` **SCM\_SET\_CELL\_TYPE** `(SCM x, scm_t_bits t)` [¶](09_02_data_representation.md)

For a non-immediate Scheme object x, write the value t into the first word of the heap object referenced by x. The value t must hold a valid cell type.

* * *

Previous: [Heap Object Type Information](09_02_data_representation.md#9255-heap-object-type-information), Up: [The SCM Type in Guile](09_02_data_representation.md#925-the-scm-type-in-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.2.5.6 Accessing Heap Object Fields [¶](09_02_data_representation.md#9256-accessing-heap-object-fields)

For a non-immediate Scheme object x, the object type can be determined by using the `SCM_CELL_TYPE` macro described in the previous section. For each different type of heap object it is known which fields hold tagged Scheme objects and which fields hold untagged raw data. To access the different fields appropriately, the following macros are provided.

Macro: `scm_t_bits` **SCM\_CELL\_WORD** `(SCM x, unsigned int n)` [¶](09_02_data_representation.md)

Macro: `scm_t_bits` **SCM\_CELL\_WORD\_0** `(x)` [¶](09_02_data_representation.md)

Macro: `scm_t_bits` **SCM\_CELL\_WORD\_1** `(x)` [¶](09_02_data_representation.md)

Macro: `scm_t_bits` **SCM\_CELL\_WORD\_2** `(x)` [¶](09_02_data_representation.md)

Macro: `scm_t_bits` **SCM\_CELL\_WORD\_3** `(x)` [¶](09_02_data_representation.md)

Deliver the field n of the heap object referenced by the non-immediate Scheme object x as raw untagged data. Only use this macro for fields containing untagged data; don’t use it for fields containing tagged `SCM` objects.

Macro: `SCM` **SCM\_CELL\_OBJECT** `(SCM x, unsigned int n)` [¶](09_02_data_representation.md)

Macro: `SCM` **SCM\_CELL\_OBJECT\_0** `(SCM x)` [¶](09_02_data_representation.md)

Macro: `SCM` **SCM\_CELL\_OBJECT\_1** `(SCM x)` [¶](09_02_data_representation.md)

Macro: `SCM` **SCM\_CELL\_OBJECT\_2** `(SCM x)` [¶](09_02_data_representation.md)

Macro: `SCM` **SCM\_CELL\_OBJECT\_3** `(SCM x)` [¶](09_02_data_representation.md)

Deliver the field n of the heap object referenced by the non-immediate Scheme object x as a Scheme object. Only use this macro for fields containing tagged `SCM` objects; don’t use it for fields containing untagged data.

Macro: `void` **SCM\_SET\_CELL\_WORD** `(SCM x, unsigned int n, scm_t_bits w)` [¶](09_02_data_representation.md)

Macro: `void` **SCM\_SET\_CELL\_WORD\_0** `(x, w)` [¶](09_02_data_representation.md)

Macro: `void` **SCM\_SET\_CELL\_WORD\_1** `(x, w)` [¶](09_02_data_representation.md)

Macro: `void` **SCM\_SET\_CELL\_WORD\_2** `(x, w)` [¶](09_02_data_representation.md)

Macro: `void` **SCM\_SET\_CELL\_WORD\_3** `(x, w)` [¶](09_02_data_representation.md)

Write the raw value w into field number n of the heap object referenced by the non-immediate Scheme value x. Values that are written into heap objects as raw values should only be read later using the `SCM_CELL_WORD` macros.

Macro: `void` **SCM\_SET\_CELL\_OBJECT** `(SCM x, unsigned int n, SCM o)` [¶](09_02_data_representation.md)

Macro: `void` **SCM\_SET\_CELL\_OBJECT\_0** `(SCM x, SCM o)` [¶](09_02_data_representation.md)

Macro: `void` **SCM\_SET\_CELL\_OBJECT\_1** `(SCM x, SCM o)` [¶](09_02_data_representation.md)

Macro: `void` **SCM\_SET\_CELL\_OBJECT\_2** `(SCM x, SCM o)` [¶](09_02_data_representation.md)

Macro: `void` **SCM\_SET\_CELL\_OBJECT\_3** `(SCM x, SCM o)` [¶](09_02_data_representation.md)

Write the Scheme object o into field number n of the heap object referenced by the non-immediate Scheme value x. Values that are written into heap objects as objects should only be read using the `SCM_CELL_OBJECT` macros.

Summary:

*   For a non-immediate Scheme object x of unknown type, get the type information by using `SCM_CELL_TYPE (x)`.
*   As soon as the type information is available, only use the appropriate access methods to read and write data to the different heap object fields.
*   Note that field 0 stores the cell type information. Generally speaking, other data associated with a heap object is stored starting from field 1.

* * *

Next: [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine), Previous: [Data Representation](09_02_data_representation.md#92-data-representation), Up: [Guile Implementation](09_00_guile_implementation.md#9-guile-implementation)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
