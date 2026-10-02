#### 7.5.5 SRFI-4 - Homogeneous numeric vector datatypes [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#755-srfi-4---homogeneous-numeric-vector-datatypes)

SRFI-4 provides an interface to uniform numeric vectors: vectors whose elements are all of a single numeric type. Guile offers uniform numeric vectors for signed and unsigned 8-bit, 16-bit, 32-bit, and 64-bit integers, two sizes of floating point values, and, as an extension to SRFI-4, complex floating-point numbers of these two sizes.

The standard SRFI-4 procedures and data types may be included via loading the appropriate module:

(use-modules (srfi srfi-4))

This module is currently a part of the default Guile environment, but it is a good practice to explicitly import the module. In the future, using SRFI-4 procedures without importing the SRFI-4 module will cause a deprecation message to be printed. (Of course, one may call the C functions at any time. Would that C had modules!)

*   [SRFI-4 - Overview](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7551-srfi-4---overview)
*   [SRFI-4 - API](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7552-srfi-4---api)
*   [SRFI-4 - Relation to bytevectors](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7553-srfi-4---relation-to-bytevectors)
*   [SRFI-4 - Guile extensions](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7554-srfi-4---guile-extensions)

* * *

Next: [SRFI-4 - API](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7552-srfi-4---api), Up: [SRFI-4 - Homogeneous numeric vector datatypes](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#755-srfi-4---homogeneous-numeric-vector-datatypes)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.5.1 SRFI-4 - Overview [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7551-srfi-4---overview)

Uniform numeric vectors can be useful since they consume less memory than the non-uniform, general vectors. Also, since the types they can store correspond directly to C types, it is easier to work with them efficiently on a low level. Consider image processing as an example, where you want to apply a filter to some image. While you could store the pixels of an image in a general vector and write a general convolution function, things are much more efficient with uniform vectors: the convolution function knows that all pixels are unsigned 8-bit values (say), and can use a very tight inner loop.

This is implemented in Scheme by having the compiler notice calls to the SRFI-4 accessors, and inline them to appropriate compiled code. From C you have access to the raw array; functions for efficiently working with uniform numeric vectors from C are listed at the end of this section.

Uniform numeric vectors are the special case of one dimensional uniform numeric arrays.

There are 12 standard kinds of uniform numeric vectors, and they all have their own complement of constructors, accessors, and so on. Procedures that operate on a specific kind of uniform numeric vector have a “tag” in their name, indicating the element type.

`u8`

unsigned 8-bit integers

`s8`

signed 8-bit integers

`u16`

unsigned 16-bit integers

`s16`

signed 16-bit integers

`u32`

unsigned 32-bit integers

`s32`

signed 32-bit integers

`u64`

unsigned 64-bit integers

`s64`

signed 64-bit integers

`f32`

the C type `float`

`f64`

the C type `double`

In addition, Guile supports uniform arrays of complex numbers, with the nonstandard tags:

`c32`

complex numbers in rectangular form with the real and imaginary part being a `float`

`c64`

complex numbers in rectangular form with the real and imaginary part being a `double`

The external representation (ie. read syntax) for these vectors is similar to normal Scheme vectors, but with an additional tag from the tables above indicating the vector’s type. For example,

#u16(1 2 3)
#f64(3.1415 2.71)

Note that the read syntax for floating-point here conflicts with `#f` for false. In Standard Scheme one can write `(1 #f3)` for a three element list `(1 #f 3)`, but for Guile `(1 #f3)` is invalid. `(1 #f 3)` is almost certainly what one should write anyway to make the intention clear, so this is rarely a problem.

* * *

Next: [SRFI-4 - Relation to bytevectors](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7553-srfi-4---relation-to-bytevectors), Previous: [SRFI-4 - Overview](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7551-srfi-4---overview), Up: [SRFI-4 - Homogeneous numeric vector datatypes](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#755-srfi-4---homogeneous-numeric-vector-datatypes)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.5.2 SRFI-4 - API [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7552-srfi-4---api)

Note that the `c32` and `c64` functions are only available from `(srfi srfi-4 gnu)`.

Scheme Procedure: **u8vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s8vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u16vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s16vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u32vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s32vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u64vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s64vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f32vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f64vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c32vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c64vector?** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u8vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s8vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u16vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s16vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u32vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s32vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u64vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s64vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f32vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f64vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c32vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c64vector\_p** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return `#t` if obj is a homogeneous numeric vector of the indicated type.

Scheme Procedure: **make-u8vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-s8vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-u16vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-s16vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-u32vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-s32vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-u64vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-s64vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-f32vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-f64vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-c32vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **make-c64vector** n \[value\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_u8vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_s8vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_u16vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_s16vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_u32vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_s32vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_u64vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_s64vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_f32vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_f64vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_c32vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_make\_c64vector** (n, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return a newly allocated homogeneous numeric vector holding n elements of the indicated type. If value is given, the vector is initialized with that value, otherwise the contents are unspecified.

Scheme Procedure: **u8vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s8vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u16vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s16vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u32vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s32vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u64vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s64vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f32vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f64vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c32vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c64vector** value … [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u8vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s8vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u16vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s16vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u32vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s32vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u64vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s64vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f32vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f64vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c32vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c64vector** (values) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return a newly allocated homogeneous numeric vector of the indicated type, holding the given parameter values. The vector length is the number of parameters given.

Scheme Procedure: **u8vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s8vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u16vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s16vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u32vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s32vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u64vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s64vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f32vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f64vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c32vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c64vector-length** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u8vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s8vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u16vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s16vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u32vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s32vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u64vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s64vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f32vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f64vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c32vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c64vector\_length** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return the number of elements in vec.

Scheme Procedure: **u8vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s8vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u16vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s16vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u32vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s32vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u64vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s64vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f32vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f64vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c32vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c64vector-ref** vec i [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u8vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s8vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u16vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s16vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u32vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s32vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u64vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s64vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f32vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f64vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c32vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c64vector\_ref** (vec, i) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return the element at index i in vec. The first element in vec is index 0.

Scheme Procedure: **u8vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s8vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u16vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s16vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u32vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s32vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u64vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s64vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f32vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f64vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c32vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c64vector-set!** vec i value [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u8vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s8vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u16vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s16vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u32vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s32vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u64vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s64vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f32vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f64vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c32vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c64vector\_set\_x** (vec, i, value) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Set the element at index i in vec to value. The first element in vec is index 0. The return value is unspecified.

Scheme Procedure: **u8vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s8vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u16vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s16vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u32vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s32vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u64vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s64vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f32vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f64vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c32vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c64vector->list** vec [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u8vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s8vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u16vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s16vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u32vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s32vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_u64vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_s64vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f32vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_f64vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c32vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_c64vector\_to\_list** (vec) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return a newly allocated list holding all elements of vec.

Scheme Procedure: **list->u8vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->s8vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->u16vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->s16vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->u32vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->s32vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->u64vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->s64vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->f32vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->f64vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->c32vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **list->c64vector** lst [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_u8vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_s8vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_u16vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_s16vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_u32vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_s32vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_u64vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_s64vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_f32vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_f64vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_c32vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_list\_to\_c64vector** (lst) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return a newly allocated homogeneous numeric vector of the indicated type, initialized with the elements of the list lst.

C Function: `SCM` **scm\_take\_u8vector** `(const scm_t_uint8 *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_s8vector** `(const scm_t_int8 *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_u16vector** `(const scm_t_uint16 *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_s16vector** `(const scm_t_int16 *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_u32vector** `(const scm_t_uint32 *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_s32vector** `(const scm_t_int32 *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_u64vector** `(const scm_t_uint64 *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_s64vector** `(const scm_t_int64 *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_f32vector** `(const float *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_f64vector** `(const double *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_c32vector** `(const float *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `SCM` **scm\_take\_c64vector** `(const double *data, size_t len)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return a new uniform numeric vector of the indicated type and length that uses the memory pointed to by data to store its elements. This memory will eventually be freed with `free`. The argument len specifies the number of elements in data, not its size in bytes.

The `c32` and `c64` variants take a pointer to a C array of `float`s or `double`s. The real parts of the complex numbers are at even indices in that array, the corresponding imaginary parts are at the following odd index.

C Function: `const scm_t_uint8 *` **scm\_u8vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const scm_t_int8 *` **scm\_s8vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const scm_t_uint16 *` **scm\_u16vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const scm_t_int16 *` **scm\_s16vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const scm_t_uint32 *` **scm\_u32vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const scm_t_int32 *` **scm\_s32vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const scm_t_uint64 *` **scm\_u64vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const scm_t_int64 *` **scm\_s64vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const float *` **scm\_f32vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const double *` **scm\_f64vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const float *` **scm\_c32vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `const double *` **scm\_c64vector\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Like `scm_vector_elements` (see [Vector Accessing from C](06_06_10_vectors.md#66104-vector-accessing-from-c)), but returns a pointer to the elements of a uniform numeric vector of the indicated kind.

C Function: `scm_t_uint8 *` **scm\_u8vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `scm_t_int8 *` **scm\_s8vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `scm_t_uint16 *` **scm\_u16vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `scm_t_int16 *` **scm\_s16vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `scm_t_uint32 *` **scm\_u32vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `scm_t_int32 *` **scm\_s32vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `scm_t_uint64 *` **scm\_u64vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `scm_t_int64 *` **scm\_s64vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `float *` **scm\_f32vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `double *` **scm\_f64vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `float *` **scm\_c32vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: `double *` **scm\_c64vector\_writable\_elements** `(SCM vec, scm_t_array_handle *handle, size_t *lenp, ssize_t *incp)` [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Like `scm_vector_writable_elements` (see [Vector Accessing from C](06_06_10_vectors.md#66104-vector-accessing-from-c)), but returns a pointer to the elements of a uniform numeric vector of the indicated kind.

* * *

Next: [SRFI-4 - Guile extensions](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7554-srfi-4---guile-extensions), Previous: [SRFI-4 - API](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7552-srfi-4---api), Up: [SRFI-4 - Homogeneous numeric vector datatypes](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#755-srfi-4---homogeneous-numeric-vector-datatypes)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.5.3 SRFI-4 - Relation to bytevectors [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7553-srfi-4---relation-to-bytevectors)

Guile implements SRFI-4 vectors using bytevectors (see [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)). Often when you have a numeric vector, you end up wanting to write its bytes somewhere, or have access to the underlying bytes, or read in bytes from somewhere else. Bytevectors are very good at this sort of thing. But the SRFI-4 APIs are nicer to use when doing number-crunching, because they are addressed by element and not by byte.

So as a compromise, Guile allows all bytevector functions to operate on numeric vectors. They address the underlying bytes in the native endianness, as one would expect.

Following the same reasoning, that it’s just bytes underneath, Guile also allows uniform vectors of a given type to be accessed as if they were of any type. One can fill a `u32vector`, and access its elements with `u8vector-ref`. One can use `f64vector-ref` on bytevectors. It’s all the same to Guile.

In this way, uniform numeric vectors may be written to and read from input/output ports using the procedures that operate on bytevectors.

See [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors), for more information.

* * *

Previous: [SRFI-4 - Relation to bytevectors](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7553-srfi-4---relation-to-bytevectors), Up: [SRFI-4 - Homogeneous numeric vector datatypes](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#755-srfi-4---homogeneous-numeric-vector-datatypes)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.5.4 SRFI-4 - Guile extensions [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7554-srfi-4---guile-extensions)

Guile defines some useful extensions to SRFI-4, which are not available in the default Guile environment. They may be imported by loading the extensions module:

(use-modules (srfi srfi-4 gnu))

Scheme Procedure: **srfi-4-vector-type-size** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return the size, in bytes, of each element of SRFI-4 vector obj. For example, `(srfi-4-vector-type-size #u32())` returns `4`.

Scheme Procedure: **any->u8vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->s8vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->u16vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->s16vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->u32vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->s32vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->u64vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->s64vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->f32vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->f64vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->c32vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **any->c64vector** obj [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_u8vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_s8vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_u16vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_s16vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_u32vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_s32vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_u64vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_s64vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_f32vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_f64vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_c32vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

C Function: **scm\_any\_to\_c64vector** (obj) [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Return a (maybe newly allocated) uniform numeric vector of the indicated type, initialized with the elements of obj, which must be a list, a vector, or a uniform vector. When obj is already a suitable uniform numeric vector, it is returned unchanged.

Scheme Procedure: **u8vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s8vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u16vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s16vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u32vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s32vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u64vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s64vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f32vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f64vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c32vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c64vector-copy!** dst at src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Copy a block of elements from src to dst, both of which must be vectors of the indicated type, starting in dst at at and starting in src at start and ending at end. It is an error for dst to have a length less than at + (end - start). at and start default to 0 and end defaults to the length of src.

If source and destination overlap, copying takes place as if the source is first copied into a temporary vector and then into the destination.

See also [`vector-copy!`](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md).

Scheme Procedure: **u8vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s8vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u16vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s16vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u32vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s32vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **u64vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **s64vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f32vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **f64vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c32vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Scheme Procedure: **c64vector-copy** src \[start \[end\]\] [¶](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md)

Returns a freshly allocated vector of the indicated type, which must be the same as that of src, containing the elements of src between start and end. start defaults to 0 and end defaults to the length of src.

See also [`vector-copy`](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md).

* * *

Next: [SRFI-8 - receive](07_05_07_srfi8_receive.md#757-srfi-8---receive), Previous: [SRFI-4 - Homogeneous numeric vector datatypes](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#755-srfi-4---homogeneous-numeric-vector-datatypes), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
