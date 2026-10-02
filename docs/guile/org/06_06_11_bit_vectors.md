#### 6.6.11 Bit Vectors [¶](06_06_11_bit_vectors.md#6611-bit-vectors)

Bit vectors are zero-origin, one-dimensional arrays of booleans. They are displayed as a sequence of `0`s and `1`s prefixed by `#*`, e.g.,

(make-bitvector 8 #f) ⇒
#\*00000000

Bit vectors are the special case of one dimensional bit arrays, and can thus be used with the array procedures, See [Arrays](06_06_13_arrays.md#6613-arrays).

Scheme Procedure: **bitvector?** obj [¶](06_06_11_bit_vectors.md)

Return `#t` when obj is a bitvector, else return `#f`.

Scheme Procedure: **make-bitvector** len \[fill\] [¶](06_06_11_bit_vectors.md)

Create a new bitvector of length len and optionally initialize all elements to fill.

Scheme Procedure: **bitvector** bit … [¶](06_06_11_bit_vectors.md)

Create a new bitvector with the arguments as elements.

Scheme Procedure: **bitvector-length** vec [¶](06_06_11_bit_vectors.md)

Return the length of the bitvector vec.

Scheme Procedure: **bitvector-bit-set?** vec idx [¶](06_06_11_bit_vectors.md)

Scheme Procedure: **bitvector-bit-clear?** vec idx [¶](06_06_11_bit_vectors.md)

Return `#t` if the bit at index idx of the bitvector vec is set (for `bitvector-bit-set?`) or clear (for `bitvector-bit-clear?`).

Scheme Procedure: **bitvector-set-bit!** vec idx [¶](06_06_11_bit_vectors.md)

Scheme Procedure: **bitvector-clear-bit!** vec idx [¶](06_06_11_bit_vectors.md)

Set (for `bitvector-set-bit!`) or clear (for `bitvector-clear-bit!`) the bit at index idx of the bitvector vec.

Scheme Procedure: **bitvector-set-all-bits!** vec [¶](06_06_11_bit_vectors.md)

Scheme Procedure: **bitvector-clear-all-bits!** vec [¶](06_06_11_bit_vectors.md)

Scheme Procedure: **bitvector-flip-all-bits!** vec [¶](06_06_11_bit_vectors.md)

Set, clear, or flip all bits of vec.

Scheme Procedure: **list->bitvector** list [¶](06_06_11_bit_vectors.md)

C Function: **scm\_list\_to\_bitvector** (list) [¶](06_06_11_bit_vectors.md)

Return a new bitvector initialized with the elements of list.

Scheme Procedure: **bitvector->list** vec [¶](06_06_11_bit_vectors.md)

C Function: **scm\_bitvector\_to\_list** (vec) [¶](06_06_11_bit_vectors.md)

Return a new list initialized with the elements of the bitvector vec.

Scheme Procedure: **bitvector-copy** bitvector \[start \[end\]\] [¶](06_06_11_bit_vectors.md)

C Function: **scm\_bitvector\_copy** (bitvector, start, end) [¶](06_06_11_bit_vectors.md)

Returns a freshly allocated bitvector containing the elements of bitvector in the range \[start ... end). start defaults to 0 and end defaults to the length of bitvector.

Scheme Procedure: **bitvector-count** bitvector [¶](06_06_11_bit_vectors.md)

Return a count of how many entries in bitvector are set.

(bitvector-count #\*000111000)  ⇒ 3

Scheme Procedure: **bitvector-count-bits** bitvector bits [¶](06_06_11_bit_vectors.md)

Return a count of how many entries in bitvector are set, with the bitvector bits selecting the entries to consider. bitvector must be at least as long as bits.

For example,

(bitvector-count-bits #\*01110111 #\*11001101) ⇒ 3

Scheme Procedure: **bitvector-position** bitvector bool start [¶](06_06_11_bit_vectors.md)

C Function: **scm\_bitvector\_position** (bitvector, bool, start) [¶](06_06_11_bit_vectors.md)

Return the index of the first occurrence of bool in bitvector, starting from start. If there is no bool entry between start and the end of bitvector, then return `#f`. For example,

(bitvector-position #\*000101 #t 0)  ⇒ 3
(bitvector-position #\*0001111 #f 3) ⇒ #f

Scheme Procedure: **bitvector-set-bits!** bitvector bits [¶](06_06_11_bit_vectors.md)

Set entries of bitvector to `#t`, with bits selecting the bits to set. The return value is unspecified. bitvector must be at least as long as bits.

(define bv (bitvector-copy #\*11000010))
(bitvector-set-bits! bv #\*10010001)
bv
⇒ #\*11010011

Scheme Procedure: **bitvector-clear-bits!** bitvector bits [¶](06_06_11_bit_vectors.md)

Set entries of bitvector to `#f`, with bits selecting the bits to clear. The return value is unspecified. bitvector must be at least as long as bits.

(define bv (bitvector-copy #\*11000010))
(bitvector-clear-bits! bv #\*10010001)
bv
⇒ #\*01000010

C Function: `int` **scm\_is\_bitvector** `(SCM obj)` [¶](06_06_11_bit_vectors.md)

C Function: `SCM` **scm\_c\_make\_bitvector** `(size_t len, SCM fill)` [¶](06_06_11_bit_vectors.md)

C Function: `int` **scm\_bitvector\_bit\_is\_set** `(SCM vec, size_t idx)` [¶](06_06_11_bit_vectors.md)

C Function: `int` **scm\_bitvector\_bit\_is\_clear** `(SCM vec, size_t idx)` [¶](06_06_11_bit_vectors.md)

C Function: `void` **scm\_c\_bitvector\_set\_bit\_x** `(SCM vec, size_t idx)` [¶](06_06_11_bit_vectors.md)

C Function: `void` **scm\_c\_bitvector\_clear\_bit\_x** `(SCM vec, size_t idx)` [¶](06_06_11_bit_vectors.md)

C Function: `void` **scm\_c\_bitvector\_set\_bits\_x** `(SCM vec, SCM bits)` [¶](06_06_11_bit_vectors.md)

C Function: `void` **scm\_c\_bitvector\_clear\_bits\_x** `(SCM vec, SCM bits)` [¶](06_06_11_bit_vectors.md)

C Function: `void` **scm\_c\_bitvector\_set\_all\_bits\_x** `(SCM vec)` [¶](06_06_11_bit_vectors.md)

C Function: `void` **scm\_c\_bitvector\_clear\_all\_bits\_x** `(SCM vec)` [¶](06_06_11_bit_vectors.md)

C Function: `void` **scm\_c\_bitvector\_flip\_all\_bits\_x** `(SCM vec)` [¶](06_06_11_bit_vectors.md)

C Function: `size_t` **scm\_c\_bitvector\_length** `(SCM bitvector)` [¶](06_06_11_bit_vectors.md)

C Function: `size_t` **scm\_c\_bitvector\_count** `(SCM bitvector)` [¶](06_06_11_bit_vectors.md)

C Function: `size_t` **scm\_c\_bitvector\_count\_bits** `(SCM bitvector, SCM bits)` [¶](06_06_11_bit_vectors.md)

C API for the corresponding Scheme bitvector interfaces.

C Function: `const scm_t_uint32 *` **scm\_bitvector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *offp, size_t *lenp, ssize_t *incp)` [¶](06_06_11_bit_vectors.md)

Like `scm_vector_elements` (see [Vector Accessing from C](06_06_10_vectors.md#66104-vector-accessing-from-c)), but for bitvectors. The variable pointed to by offp is set to the value returned by `scm_array_handle_bit_elements_offset`. See `scm_array_handle_bit_elements` for how to use the returned pointer and the offset.

C Function: `scm_t_uint32 *` **scm\_bitvector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *offp, size_t *lenp, ssize_t *incp)` [¶](06_06_11_bit_vectors.md)

Like `scm_bitvector_elements`, but the pointer is good for reading and writing.

* * *

Next: [Arrays](06_06_13_arrays.md#6613-arrays), Previous: [Bit Vectors](06_06_11_bit_vectors.md#6611-bit-vectors), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
