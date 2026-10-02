#### 6.6.10 Vectors [¶](06_06_10_vectors.md#6610-vectors)

Vectors are sequences of Scheme objects. Unlike lists, the length of a vector, once the vector is created, cannot be changed. The advantage of vectors over lists is that the time required to access one element of a vector given its _position_ (synonymous with _index_), a zero-origin number, is constant, whereas lists have an access time linear to the position of the accessed element in the list.

Vectors can contain any kind of Scheme object; it is even possible to have different types of objects in the same vector. For vectors containing vectors, you may wish to use [Arrays](06_06_13_arrays.md#6613-arrays) instead. Note, too, that vectors are a special case of one dimensional non-uniform arrays and that array procedures operate happily on vectors.

Also see [SRFI-43 - Vector Library](07_05_30_srfi43_vector_library.md#7530-srfi-43---vector-library), [R6RS Support](07_06_r6rs_support.md#76-r6rs-support), or [R7RS Support](07_07_r7rs_support.md#77-r7rs-support), for more comprehensive vector libraries.

*   [Read Syntax for Vectors](06_06_10_vectors.md#66101-read-syntax-for-vectors)
*   [Dynamic Vector Creation and Validation](06_06_10_vectors.md#66102-dynamic-vector-creation-and-validation)
*   [Accessing and Modifying Vector Contents](06_06_10_vectors.md#66103-accessing-and-modifying-vector-contents)
*   [Vector Accessing from C](06_06_10_vectors.md#66104-vector-accessing-from-c)
*   [Uniform Numeric Vectors](06_06_10_vectors.md#66105-uniform-numeric-vectors)

* * *

Next: [Dynamic Vector Creation and Validation](06_06_10_vectors.md#66102-dynamic-vector-creation-and-validation), Up: [Vectors](06_06_10_vectors.md#6610-vectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.10.1 Read Syntax for Vectors [¶](06_06_10_vectors.md#66101-read-syntax-for-vectors)

Vectors can literally be entered in source code, just like strings, characters or some of the other data types. The read syntax for vectors is as follows: A sharp sign (`#`), followed by an opening parentheses, all elements of the vector in their respective read syntax, and finally a closing parentheses. Like strings, vectors do not have to be quoted.

The following are examples of the read syntax for vectors; where the first vector only contains numbers and the second three different object types: a string, a symbol and a number in hexadecimal notation.

#(1 2 3)
#("Hello" foo #xdeadbeef)

* * *

Next: [Accessing and Modifying Vector Contents](06_06_10_vectors.md#66103-accessing-and-modifying-vector-contents), Previous: [Read Syntax for Vectors](06_06_10_vectors.md#66101-read-syntax-for-vectors), Up: [Vectors](06_06_10_vectors.md#6610-vectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.10.2 Dynamic Vector Creation and Validation [¶](06_06_10_vectors.md#66102-dynamic-vector-creation-and-validation)

Instead of creating a vector implicitly by using the read syntax just described, you can create a vector dynamically by calling one of the `vector` and `list->vector` primitives with the list of Scheme values that you want to place into a vector. The size of the vector thus created is determined implicitly by the number of arguments given.

Scheme Procedure: **vector** arg … [¶](06_06_10_vectors.md)

Scheme Procedure: **list->vector** l [¶](06_06_10_vectors.md)

C Function: **scm\_vector** (l) [¶](06_06_10_vectors.md)

Return a newly allocated vector composed of the given arguments. Analogous to `list`.

([vector](06_06_10_vectors.md) 'a 'b 'c) ⇒ #(a b c)

The inverse operation is `vector->list`:

Scheme Procedure: **vector->list** v [¶](06_06_10_vectors.md)

C Function: **scm\_vector\_to\_list** (v) [¶](06_06_10_vectors.md)

Return a newly allocated list composed of the elements of v.

([vector->list](06_06_10_vectors.md) #(dah dah didah)) ⇒  (dah dah didah)
([list->vector](06_06_10_vectors.md) '(dididit dah)) ⇒  #(dididit dah)

To allocate a vector with an explicitly specified size, use `make-vector`. With this primitive you can also specify an initial value for the vector elements (the same value for all elements, that is):

Scheme Procedure: **make-vector** len \[fill\] [¶](06_06_10_vectors.md)

C Function: **scm\_make\_vector** (len, fill) [¶](06_06_10_vectors.md)

Return a newly allocated vector of len elements. If a second argument is given, then each position is initialized to fill. Otherwise the initial contents of each position is unspecified.

C Function: `SCM` **scm\_c\_make\_vector** `(size_t k, SCM fill)` [¶](06_06_10_vectors.md)

Like `scm_make_vector`, but the length is given as a `size_t`.

To check whether an arbitrary Scheme value _is_ a vector, use the `vector?` primitive:

Scheme Procedure: **vector?** obj [¶](06_06_10_vectors.md)

C Function: **scm\_vector\_p** (obj) [¶](06_06_10_vectors.md)

Return `#t` if obj is a vector, otherwise return `#f`.

C Function: `int` **scm\_is\_vector** `(SCM obj)` [¶](06_06_10_vectors.md)

Return non-zero when obj is a vector, otherwise return `zero`.

* * *

Next: [Vector Accessing from C](06_06_10_vectors.md#66104-vector-accessing-from-c), Previous: [Dynamic Vector Creation and Validation](06_06_10_vectors.md#66102-dynamic-vector-creation-and-validation), Up: [Vectors](06_06_10_vectors.md#6610-vectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.10.3 Accessing and Modifying Vector Contents [¶](06_06_10_vectors.md#66103-accessing-and-modifying-vector-contents)

`vector-length` and `vector-ref` return information about a given vector, respectively its size and the elements that are contained in the vector.

Scheme Procedure: **vector-length** vector [¶](06_06_10_vectors.md)

C Function: **scm\_vector\_length** (vector) [¶](06_06_10_vectors.md)

Return the number of elements in vector as an exact integer.

C Function: `size_t` **scm\_c\_vector\_length** `(SCM vec)` [¶](06_06_10_vectors.md)

Return the number of elements in vec as a `size_t`.

Scheme Procedure: **vector-ref** vec k [¶](06_06_10_vectors.md)

C Function: **scm\_vector\_ref** (vec, k) [¶](06_06_10_vectors.md)

Return the contents of position k of vec. k must be a valid index of vec.

([vector-ref](06_06_10_vectors.md) #(1 1 2 3 5 8 13 21) 5) ⇒ 8
([vector-ref](06_06_10_vectors.md) #(1 1 2 3 5 8 13 21)
    (let ((i ([round](06_06_02_numerical_data_types.md) ([\*](06_06_02_numerical_data_types.md) 2 ([acos](06_06_02_numerical_data_types.md) \-1)))))
      (if ([inexact?](06_06_02_numerical_data_types.md) i)
        ([inexact->exact](06_06_02_numerical_data_types.md) i)
           i))) ⇒ 13

C Function: `SCM` **scm\_c\_vector\_ref** `(SCM vec, size_t k)` [¶](06_06_10_vectors.md)

Return the contents of position k (a `size_t`) of vec.

A vector created by one of the dynamic vector constructor procedures (see [Dynamic Vector Creation and Validation](06_06_10_vectors.md#66102-dynamic-vector-creation-and-validation)) can be modified using the following procedures.

_NOTE:_ According to R5RS, it is an error to use any of these procedures on a literally read vector, because such vectors should be considered as constants. Currently, however, Guile does not detect this error.

Scheme Procedure: **vector-set!** vec k obj [¶](06_06_10_vectors.md)

C Function: **scm\_vector\_set\_x** (vec, k, obj) [¶](06_06_10_vectors.md)

Store obj in position k of vec. k must be a valid index of vec. The value returned by ‘vector-set!’ is unspecified.

(let ((vec ([vector](06_06_10_vectors.md) 0 '(2 2 2 2) "Anna")))
  ([vector-set!](06_06_10_vectors.md) vec 1 '("Sue" "Sue"))
  vec) ⇒  #(0 ("Sue" "Sue") "Anna")

C Function: `void` **scm\_c\_vector\_set\_x** `(SCM vec, size_t k, SCM obj)` [¶](06_06_10_vectors.md)

Store obj in position k (a `size_t`) of vec.

Scheme Procedure: **vector-fill!** vec fill \[start \[end\]\] [¶](06_06_10_vectors.md)

C Function: **scm\_vector\_fill\_x** (vec, fill) [¶](06_06_10_vectors.md)

Store fill in every position of vec in the range \[start ... end). start defaults to 0 and end defaults to the length of vec.

The value returned by `vector-fill!` is unspecified.

Scheme Procedure: **vector-copy** vec \[start \[end\]\] [¶](06_06_10_vectors.md)

C Function: **scm\_vector\_copy** (vec) [¶](06_06_10_vectors.md)

Returns a freshly allocated vector containing the elements of vec in the range \[start ... end). start defaults to 0 and end defaults to the length of vec.

Scheme Procedure: **vector-copy!** dst at src \[start \[end\]\] [¶](06_06_10_vectors.md)

Copy the block of elements from vector src in the range \[start ... end) into vector dst, starting at position at. at and start default to 0 and end defaults to the length of src.

It is an error for dst to have a length less than at + (end - start).

The order in which elements are copied is unspecified, except that if the source and destination overlap, copying takes place as if the source is first copied into a temporary vector and then into the destination.

The value returned by `vector-copy!` is unspecified.

Scheme Procedure: **vector-move-left!** vec1 start1 end1 vec2 start2 [¶](06_06_10_vectors.md)

C Function: **scm\_vector\_move\_left\_x** (vec1, start1, end1, vec2, start2) [¶](06_06_10_vectors.md)

Copy elements from vec1, positions start1 to end1, to vec2 starting at position start2. start1 and start2 are inclusive indices; end1 is exclusive.

`vector-move-left!` copies elements in leftmost order. Therefore, in the case where vec1 and vec2 refer to the same vector, `vector-move-left!` is usually appropriate when start1 is greater than start2.

The value returned by `vector-move-left!` is unspecified.

Scheme Procedure: **vector-move-right!** vec1 start1 end1 vec2 start2 [¶](06_06_10_vectors.md)

C Function: **scm\_vector\_move\_right\_x** (vec1, start1, end1, vec2, start2) [¶](06_06_10_vectors.md)

Copy elements from vec1, positions start1 to end1, to vec2 starting at position start2. start1 and start2 are inclusive indices; end1 is exclusive.

`vector-move-right!` copies elements in rightmost order. Therefore, in the case where vec1 and vec2 refer to the same vector, `vector-move-right!` is usually appropriate when start1 is less than start2.

The value returned by `vector-move-right!` is unspecified.

* * *

Next: [Uniform Numeric Vectors](06_06_10_vectors.md#66105-uniform-numeric-vectors), Previous: [Accessing and Modifying Vector Contents](06_06_10_vectors.md#66103-accessing-and-modifying-vector-contents), Up: [Vectors](06_06_10_vectors.md#6610-vectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.10.4 Vector Accessing from C [¶](06_06_10_vectors.md#66104-vector-accessing-from-c)

A vector can be read and modified from C with the functions [`scm_c_vector_ref`](06_06_10_vectors.md) and [`scm_c_vector_set_x`](06_06_10_vectors.md). In addition to these functions, there are two other ways to access vectors from C that might be more efficient in certain situations: you can use the unsafe _vector macros_; or you can use the general framework for accessing all kinds of arrays (see [Accessing Arrays from C](06_06_13_arrays.md#66135-accessing-arrays-from-c)), which is more verbose, but can deal efficiently with all kinds of vectors (and arrays). For arrays of rank 1 whose backing store is a vector, you can use the `scm_vector_elements` and `scm_vector_writable_elements` functions as shortcuts.

C Macro: `size_t` **SCM\_SIMPLE\_VECTOR\_LENGTH** `(SCM vec)` [¶](06_06_10_vectors.md)

Evaluates to the length of the vector vec. No type checking is done.

C Macro: `SCM` **SCM\_SIMPLE\_VECTOR\_REF** `(SCM vec, size_t idx)` [¶](06_06_10_vectors.md)

Evaluates to the element at position idx in the vector vec. No type or range checking is done.

C Macro: `void` **SCM\_SIMPLE\_VECTOR\_SET** `(SCM vec, size_t idx, SCM val)` [¶](06_06_10_vectors.md)

Sets the element at position idx in the vector vec to val. No type or range checking is done.

C Function: `const SCM *` **scm\_vector\_elements** `(SCM array, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](06_06_10_vectors.md)

Acquire a [handle](06_06_13_arrays.md#66135-accessing-arrays-from-c) for array and return a read-only pointer to its elements. array must be either a vector, or an array of rank 1 whose backing store is a vector; otherwise an error is signaled. The handle must eventually be released with [`scm_array_handle_release`](06_06_10_vectors.md).

The variables pointed to by lenp and incp are filled with the number of elements of the array and the increment (number of elements) between successive elements, respectively. Successive elements of array need not be contiguous in their underlying “root vector” returned here; hence the increment is not necessarily equal to 1 and may well be negative too (see [Shared Arrays](06_06_13_arrays.md#66133-shared-arrays)).

The following example shows the typical way to use this function. It creates a list of all elements of array (in reverse order).

scm\_t\_array\_handle handle;
size\_t i, len;
ssize\_t inc;
const SCM \*elt;
SCM list;

elt = scm\_vector\_elements (array, &handle, &len, &inc);
list = SCM\_EOL;
for (i = 0; i < len; i++, elt += inc)
  list = scm\_cons (\*elt, list);
scm\_array\_handle\_release (&handle);

C Function: `SCM *` **scm\_vector\_writable\_elements** `(SCM array, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](06_06_10_vectors.md)

Like `scm_vector_elements` but the pointer can be used to modify the array.

The following example shows the typical way to use this function. It fills an array with `#t`.

scm\_t\_array\_handle handle;
size\_t i, len;
ssize\_t inc;
SCM \*elt;

elt = scm\_vector\_writable\_elements (array, &handle, &len, &inc);
for (i = 0; i < len; i++, elt += inc)
  \*elt = SCM\_BOOL\_T;
scm\_array\_handle\_release (&handle);

* * *

Previous: [Vector Accessing from C](06_06_10_vectors.md#66104-vector-accessing-from-c), Up: [Vectors](06_06_10_vectors.md#6610-vectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.10.5 Uniform Numeric Vectors [¶](06_06_10_vectors.md#66105-uniform-numeric-vectors)

A uniform numeric vector is a vector whose elements are all of a single numeric type. Guile offers uniform numeric vectors for signed and unsigned 8-bit, 16-bit, 32-bit, and 64-bit integers, two sizes of floating point values, and complex floating-point numbers of these two sizes. See [SRFI-4 - Homogeneous numeric vector datatypes](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#755-srfi-4---homogeneous-numeric-vector-datatypes), for more information.

For many purposes, bytevectors work just as well as uniform vectors, and have the advantage that they integrate well with binary input and output. See [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors), for more information on bytevectors.

* * *

Next: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors), Previous: [Vectors](06_06_10_vectors.md#6610-vectors), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
