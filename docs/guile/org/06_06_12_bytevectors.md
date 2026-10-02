#### 6.6.12 Bytevectors [¶](06_06_12_bytevectors.md#6612-bytevectors)

A _bytevector_ is a raw byte string. The `(rnrs bytevectors)` module provides the programming interface specified by the [Revised^6 Report on the Algorithmic Language Scheme (R6RS)](http://www.r6rs.org/). It contains procedures to manipulate bytevectors and interpret their contents in a number of ways: as signed or unsigned integer of various sizes and endianness, as IEEE-754 floating point numbers, or as strings. It is a useful tool to encode and decode binary data. The [R7RS](07_07_r7rs_support.md#77-r7rs-support) offers its own set of bytevector procedures (see [Bytevector Procedures in R7RS](06_06_12_bytevectors.md#66129-bytevector-procedures-in-r7rs)).

The R6RS (Section 4.3.4) specifies an external representation for bytevectors, whereby the octets (integers in the range 0–255) contained in the bytevector are represented as a list prefixed by `#vu8`:

#vu8(1 53 204)

denotes a 3-byte bytevector containing the octets 1, 53, and 204. Like string literals, booleans, etc., bytevectors are “self-quoting”, i.e., they do not need to be quoted:

#vu8(1 53 204)
⇒ #vu8(1 53 204)

Bytevectors can be used with the binary input/output primitives (see [Binary I/O](06_12_input_and_output.md#6122-binary-io)).

*   [Endianness](06_06_12_bytevectors.md#66121-endianness)
*   [Manipulating Bytevectors](06_06_12_bytevectors.md#66122-manipulating-bytevectors)
*   [Interpreting Bytevector Contents as Integers](06_06_12_bytevectors.md#66123-interpreting-bytevector-contents-as-integers)
*   [Converting Bytevectors to/from Integer Lists](06_06_12_bytevectors.md#66124-converting-bytevectors-tofrom-integer-lists)
*   [Interpreting Bytevector Contents as Floating Point Numbers](06_06_12_bytevectors.md#66125-interpreting-bytevector-contents-as-floating-point-numbers)
*   [Interpreting Bytevector Contents as Unicode Strings](06_06_12_bytevectors.md#66126-interpreting-bytevector-contents-as-unicode-strings)
*   [Accessing Bytevectors with the Array API](06_06_12_bytevectors.md#66127-accessing-bytevectors-with-the-array-api)
*   [Accessing Bytevectors with the SRFI-4 API](06_06_12_bytevectors.md#66128-accessing-bytevectors-with-the-srfi-4-api)
*   [Bytevector Procedures in R7RS](06_06_12_bytevectors.md#66129-bytevector-procedures-in-r7rs)
*   [Bytevector Slices](06_06_12_bytevectors.md#661210-bytevector-slices)

* * *

Next: [Manipulating Bytevectors](06_06_12_bytevectors.md#66122-manipulating-bytevectors), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.1 Endianness [¶](06_06_12_bytevectors.md#66121-endianness)

Some of the following procedures take an endianness parameter. The _endianness_ is defined as the order of bytes in multi-byte numbers: numbers encoded in _big endian_ have their most significant bytes written first, whereas numbers encoded in _little endian_ have their least significant bytes first[11](99_footnotes.md).

Little-endian is the native endianness of the IA32 architecture and its derivatives, while big-endian is native to SPARC and PowerPC, among others. The `native-endianness` procedure returns the native endianness of the machine it runs on.

Scheme Procedure: **native-endianness** [¶](06_06_12_bytevectors.md)

C Function: **scm\_native\_endianness** () [¶](06_06_12_bytevectors.md)

Return a value denoting the native endianness of the host machine.

Scheme Macro: **endianness** symbol [¶](06_06_12_bytevectors.md)

Return an object denoting the endianness specified by symbol. If symbol is neither `big` nor `little` then an error is raised at expand-time.

C Variable: **scm\_endianness\_big** [¶](06_06_12_bytevectors.md)

C Variable: **scm\_endianness\_little** [¶](06_06_12_bytevectors.md)

The objects denoting big- and little-endianness, respectively.

* * *

Next: [Interpreting Bytevector Contents as Integers](06_06_12_bytevectors.md#66123-interpreting-bytevector-contents-as-integers), Previous: [Endianness](06_06_12_bytevectors.md#66121-endianness), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.2 Manipulating Bytevectors [¶](06_06_12_bytevectors.md#66122-manipulating-bytevectors)

Bytevectors can be created, copied, and analyzed with the following procedures and C functions.

Scheme Procedure: **make-bytevector** len \[fill\] [¶](06_06_12_bytevectors.md)

C Function: **scm\_make\_bytevector** (len, fill) [¶](06_06_12_bytevectors.md)

C Function: **scm\_c\_make\_bytevector** (size\_t len) [¶](06_06_12_bytevectors.md)

Return a new bytevector of len bytes. Optionally, if fill is given, fill it with fill; fill must be in the range \[-128,255\].

Scheme Procedure: **bytevector?** obj [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_p** (obj) [¶](06_06_12_bytevectors.md)

Return true if obj is a bytevector.

C Function: `int` **scm\_is\_bytevector** `(SCM obj)` [¶](06_06_12_bytevectors.md)

Equivalent to `scm_is_true (scm_bytevector_p (obj))`.

Scheme Procedure: **bytevector-length** bv [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_length** (bv) [¶](06_06_12_bytevectors.md)

Return the length in bytes of bytevector bv.

C Function: `size_t` **scm\_c\_bytevector\_length** `(SCM bv)` [¶](06_06_12_bytevectors.md)

Likewise, return the length in bytes of bytevector bv.

Scheme Procedure: **bytevector=?** bv1 bv2 [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_eq\_p** (bv1, bv2) [¶](06_06_12_bytevectors.md)

Return `#t` if bv1 equals bv2—i.e., if they have the same length and contents.

Scheme Procedure: **bytevector-fill!** bv fill \[start \[end\]\] [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_fill\_x** (bv, fill) [¶](06_06_12_bytevectors.md)

Fill positions \[start ... end) of bytevector bv with byte fill. start defaults to 0 and end defaults to the length of bv.[12](99_footnotes.md)

Scheme Procedure: **bytevector-copy!** source source-start target target-start len [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_copy\_x** (source, source\_start, target, target\_start, len) [¶](06_06_12_bytevectors.md)

Copy len bytes from source into target, starting reading from source-start (an index index within source) and writing at target-start.

It is permitted for the source and target regions to overlap. In that case, copying takes place as if the source is first copied into a temporary bytevector and then into the destination.

Scheme Procedure: **bytevector-copy** bv [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_copy** (bv) [¶](06_06_12_bytevectors.md)

Return a newly allocated copy of bv.

C Function: `scm_t_uint8` **scm\_c\_bytevector\_ref** `(SCM bv, size_t index)` [¶](06_06_12_bytevectors.md)

Return the byte at index in bytevector bv.

C Function: `void` **scm\_c\_bytevector\_set\_x** `(SCM bv, size_t index, scm_t_uint8 value)` [¶](06_06_12_bytevectors.md)

Set the byte at index in bv to value.

Low-level C macros are available. They do not perform any type-checking; as such they should be used with care.

C Macro: `size_t` **SCM\_BYTEVECTOR\_LENGTH** `(bv)` [¶](06_06_12_bytevectors.md)

Return the length in bytes of bytevector bv.

C Macro: `signed char *` **SCM\_BYTEVECTOR\_CONTENTS** `(bv)` [¶](06_06_12_bytevectors.md)

Return a pointer to the contents of bytevector bv.

* * *

Next: [Converting Bytevectors to/from Integer Lists](06_06_12_bytevectors.md#66124-converting-bytevectors-tofrom-integer-lists), Previous: [Manipulating Bytevectors](06_06_12_bytevectors.md#66122-manipulating-bytevectors), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.3 Interpreting Bytevector Contents as Integers [¶](06_06_12_bytevectors.md#66123-interpreting-bytevector-contents-as-integers)

The contents of a bytevector can be interpreted as a sequence of integers of any given size, sign, and endianness.

(let ((bv ([make-bytevector](06_06_12_bytevectors.md) 4)))
  ([bytevector-u8-set!](06_06_12_bytevectors.md) bv 0 #x12)
  ([bytevector-u8-set!](06_06_12_bytevectors.md) bv 1 #x34)
  ([bytevector-u8-set!](06_06_12_bytevectors.md) bv 2 #x56)
  ([bytevector-u8-set!](06_06_12_bytevectors.md) bv 3 #x78)

  (map (lambda (number)
         ([number->string](06_06_02_numerical_data_types.md) number 16))
       ([list](06_06_09_lists.md) ([bytevector-u8-ref](06_06_12_bytevectors.md) bv 0)
             ([bytevector-u16-ref](06_06_12_bytevectors.md) bv 0 ([endianness](06_06_12_bytevectors.md) big))
             ([bytevector-u32-ref](06_06_12_bytevectors.md) bv 0 ([endianness](06_06_12_bytevectors.md) little)))))

⇒ ("12" "1234" "78563412")

The most generic procedures to interpret bytevector contents as integers are described below.

Scheme Procedure: **bytevector-uint-ref** bv index endianness size [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_uint\_ref** (bv, index, endianness, size) [¶](06_06_12_bytevectors.md)

Return the size\-byte long unsigned integer at index index in bv, decoded according to endianness.

Scheme Procedure: **bytevector-sint-ref** bv index endianness size [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_sint\_ref** (bv, index, endianness, size) [¶](06_06_12_bytevectors.md)

Return the size\-byte long signed integer at index index in bv, decoded according to endianness.

Scheme Procedure: **bytevector-uint-set!** bv index value endianness size [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_uint\_set\_x** (bv, index, value, endianness, size) [¶](06_06_12_bytevectors.md)

Set the size\-byte long unsigned integer at index to value, encoded according to endianness.

Scheme Procedure: **bytevector-sint-set!** bv index value endianness size [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_sint\_set\_x** (bv, index, value, endianness, size) [¶](06_06_12_bytevectors.md)

Set the size\-byte long signed integer at index to value, encoded according to endianness.

The following procedures are similar to the ones above, but specialized to a given integer size:

Scheme Procedure: **bytevector-u8-ref** bv index [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s8-ref** bv index [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u16-ref** bv index endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s16-ref** bv index endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u32-ref** bv index endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s32-ref** bv index endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u64-ref** bv index endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s64-ref** bv index endianness [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u8\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s8\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u16\_ref** (bv, index, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s16\_ref** (bv, index, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u32\_ref** (bv, index, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s32\_ref** (bv, index, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u64\_ref** (bv, index, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s64\_ref** (bv, index, endianness) [¶](06_06_12_bytevectors.md)

Return the unsigned n\-bit (signed) integer (where n is 8, 16, 32 or 64) from bv at index, decoded according to endianness.

Scheme Procedure: **bytevector-u8-set!** bv index value [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s8-set!** bv index value [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u16-set!** bv index value endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s16-set!** bv index value endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u32-set!** bv index value endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s32-set!** bv index value endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u64-set!** bv index value endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s64-set!** bv index value endianness [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u8\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s8\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u16\_set\_x** (bv, index, value, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s16\_set\_x** (bv, index, value, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u32\_set\_x** (bv, index, value, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s32\_set\_x** (bv, index, value, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u64\_set\_x** (bv, index, value, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s64\_set\_x** (bv, index, value, endianness) [¶](06_06_12_bytevectors.md)

Store value as an n\-bit (signed) integer (where n is 8, 16, 32 or 64) in bv at index, encoded according to endianness.

Finally, a variant specialized for the host’s endianness is available for each of these functions (with the exception of the `u8` and `s8` accessors, as endianness is about byte order and there is only 1 byte):

Scheme Procedure: **bytevector-u16-native-ref** bv index [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s16-native-ref** bv index [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u32-native-ref** bv index [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s32-native-ref** bv index [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u64-native-ref** bv index [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s64-native-ref** bv index [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u16\_native\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s16\_native\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u32\_native\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s32\_native\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u64\_native\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s64\_native\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

Return the unsigned n\-bit (signed) integer (where n is 8, 16, 32 or 64) from bv at index, decoded according to the host’s native endianness.

Scheme Procedure: **bytevector-u16-native-set!** bv index value [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s16-native-set!** bv index value [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u32-native-set!** bv index value [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s32-native-set!** bv index value [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-u64-native-set!** bv index value [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-s64-native-set!** bv index value [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u16\_native\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s16\_native\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u32\_native\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s32\_native\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_u64\_native\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_s64\_native\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

Store value as an n\-bit (signed) integer (where n is 8, 16, 32 or 64) in bv at index, encoded according to the host’s native endianness.

* * *

Next: [Interpreting Bytevector Contents as Floating Point Numbers](06_06_12_bytevectors.md#66125-interpreting-bytevector-contents-as-floating-point-numbers), Previous: [Interpreting Bytevector Contents as Integers](06_06_12_bytevectors.md#66123-interpreting-bytevector-contents-as-integers), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.4 Converting Bytevectors to/from Integer Lists [¶](06_06_12_bytevectors.md#66124-converting-bytevectors-tofrom-integer-lists)

Bytevector contents can readily be converted to/from lists of signed or unsigned integers:

([bytevector->sint-list](06_06_12_bytevectors.md) ([u8-list->bytevector](06_06_12_bytevectors.md) ([make-list](06_06_09_lists.md) 4 255))
                       ([endianness](06_06_12_bytevectors.md) little) 2)
⇒ (\-1 \-1)

Scheme Procedure: **bytevector->u8-list** bv [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_to\_u8\_list** (bv) [¶](06_06_12_bytevectors.md)

Return a newly allocated list of unsigned 8-bit integers from the contents of bv.

Scheme Procedure: **u8-list->bytevector** lst [¶](06_06_12_bytevectors.md)

C Function: **scm\_u8\_list\_to\_bytevector** (lst) [¶](06_06_12_bytevectors.md)

Return a newly allocated bytevector consisting of the unsigned 8-bit integers listed in lst.

Scheme Procedure: **bytevector->uint-list** bv endianness size [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_to\_uint\_list** (bv, endianness, size) [¶](06_06_12_bytevectors.md)

Return a list of unsigned integers of size bytes representing the contents of bv, decoded according to endianness.

Scheme Procedure: **bytevector->sint-list** bv endianness size [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_to\_sint\_list** (bv, endianness, size) [¶](06_06_12_bytevectors.md)

Return a list of signed integers of size bytes representing the contents of bv, decoded according to endianness.

Scheme Procedure: **uint-list->bytevector** lst endianness size [¶](06_06_12_bytevectors.md)

C Function: **scm\_uint\_list\_to\_bytevector** (lst, endianness, size) [¶](06_06_12_bytevectors.md)

Return a new bytevector containing the unsigned integers listed in lst and encoded on size bytes according to endianness.

Scheme Procedure: **sint-list->bytevector** lst endianness size [¶](06_06_12_bytevectors.md)

C Function: **scm\_sint\_list\_to\_bytevector** (lst, endianness, size) [¶](06_06_12_bytevectors.md)

Return a new bytevector containing the signed integers listed in lst and encoded on size bytes according to endianness.

* * *

Next: [Interpreting Bytevector Contents as Unicode Strings](06_06_12_bytevectors.md#66126-interpreting-bytevector-contents-as-unicode-strings), Previous: [Converting Bytevectors to/from Integer Lists](06_06_12_bytevectors.md#66124-converting-bytevectors-tofrom-integer-lists), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.5 Interpreting Bytevector Contents as Floating Point Numbers [¶](06_06_12_bytevectors.md#66125-interpreting-bytevector-contents-as-floating-point-numbers)

Bytevector contents can also be accessed as IEEE-754 single- or double-precision floating point numbers (respectively 32 and 64-bit long) using the procedures described here.

Scheme Procedure: **bytevector-ieee-single-ref** bv index endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-ieee-double-ref** bv index endianness [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_ieee\_single\_ref** (bv, index, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_ieee\_double\_ref** (bv, index, endianness) [¶](06_06_12_bytevectors.md)

Return the IEEE-754 single-precision floating point number from bv at index according to endianness.

Scheme Procedure: **bytevector-ieee-single-set!** bv index value endianness [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-ieee-double-set!** bv index value endianness [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_ieee\_single\_set\_x** (bv, index, value, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_ieee\_double\_set\_x** (bv, index, value, endianness) [¶](06_06_12_bytevectors.md)

Store real number value in bv at index according to endianness.

Specialized procedures are also available:

Scheme Procedure: **bytevector-ieee-single-native-ref** bv index [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-ieee-double-native-ref** bv index [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_ieee\_single\_native\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_ieee\_double\_native\_ref** (bv, index) [¶](06_06_12_bytevectors.md)

Return the IEEE-754 single-precision floating point number from bv at index according to the host’s native endianness.

Scheme Procedure: **bytevector-ieee-single-native-set!** bv index value [¶](06_06_12_bytevectors.md)

Scheme Procedure: **bytevector-ieee-double-native-set!** bv index value [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_ieee\_single\_native\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_ieee\_double\_native\_set\_x** (bv, index, value) [¶](06_06_12_bytevectors.md)

Store real number value in bv at index according to the host’s native endianness.

* * *

Next: [Accessing Bytevectors with the Array API](06_06_12_bytevectors.md#66127-accessing-bytevectors-with-the-array-api), Previous: [Interpreting Bytevector Contents as Floating Point Numbers](06_06_12_bytevectors.md#66125-interpreting-bytevector-contents-as-floating-point-numbers), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.6 Interpreting Bytevector Contents as Unicode Strings [¶](06_06_12_bytevectors.md#66126-interpreting-bytevector-contents-as-unicode-strings)

Bytevector contents can also be interpreted as Unicode strings encoded in one of the most commonly available encoding formats. See [Representing Strings as Bytes](06_06_05_strings.md#66513-representing-strings-as-bytes), for a more generic interface.

([utf8->string](06_06_12_bytevectors.md) ([u8-list->bytevector](06_06_12_bytevectors.md) '(99 97 102 101)))
⇒ "cafe"

([string->utf8](06_06_12_bytevectors.md) "café") ;; SMALL LATIN LETTER E WITH ACUTE ACCENT
⇒ #vu8(99 97 102 195 169)

Scheme Procedure: **string-utf8-length** `str` [¶](06_06_12_bytevectors.md)

C function: `SCM` **scm\_string\_utf8\_length** `(str)` [¶](06_06_12_bytevectors.md)

C function: `size_t` **scm\_c\_string\_utf8\_length** `(str)` [¶](06_06_12_bytevectors.md)

Return the number of bytes in the UTF-8 representation of str.

Scheme Procedure: **string->utf8** str [¶](06_06_12_bytevectors.md)

Scheme Procedure: **string->utf16** str \[endianness\] [¶](06_06_12_bytevectors.md)

Scheme Procedure: **string->utf32** str \[endianness\] [¶](06_06_12_bytevectors.md)

C Function: **scm\_string\_to\_utf8** (str) [¶](06_06_12_bytevectors.md)

C Function: **scm\_string\_to\_utf16** (str, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_string\_to\_utf32** (str, endianness) [¶](06_06_12_bytevectors.md)

Return a newly allocated bytevector that contains the UTF-8, UTF-16, or UTF-32 (aka. UCS-4) encoding of str. For UTF-16 and UTF-32, endianness should be the symbol `big` or `little`; when omitted, it defaults to big endian.

Scheme Procedure: **utf8->string** utf [¶](06_06_12_bytevectors.md)

Scheme Procedure: **utf16->string** utf \[endianness\] [¶](06_06_12_bytevectors.md)

Scheme Procedure: **utf32->string** utf \[endianness\] [¶](06_06_12_bytevectors.md)

C Function: **scm\_utf8\_to\_string** (utf) [¶](06_06_12_bytevectors.md)

C Function: **scm\_utf16\_to\_string** (utf, endianness) [¶](06_06_12_bytevectors.md)

C Function: **scm\_utf32\_to\_string** (utf, endianness) [¶](06_06_12_bytevectors.md)

Return a newly allocated string that contains from the UTF-8-, UTF-16-, or UTF-32-decoded contents of bytevector utf. For UTF-16 and UTF-32, endianness should be the symbol `big` or `little`; when omitted, it defaults to big endian.

* * *

Next: [Accessing Bytevectors with the SRFI-4 API](06_06_12_bytevectors.md#66128-accessing-bytevectors-with-the-srfi-4-api), Previous: [Interpreting Bytevector Contents as Unicode Strings](06_06_12_bytevectors.md#66126-interpreting-bytevector-contents-as-unicode-strings), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.7 Accessing Bytevectors with the Array API [¶](06_06_12_bytevectors.md#66127-accessing-bytevectors-with-the-array-api)

As an extension to the R6RS, Guile allows bytevectors to be manipulated with the _array_ procedures (see [Arrays](06_06_13_arrays.md#6613-arrays)). When using these APIs, bytes are accessed one at a time as 8-bit unsigned integers:

(define bv #vu8(0 1 2 3))

(array? bv)
⇒ #t

(array-rank bv)
⇒ 1

(array-ref bv 2)
⇒ 2

;; Note the different argument order on array-set!.
(array-set! bv 77 2)
(array-ref bv 2)
⇒ 77

(array-type bv)
⇒ vu8

* * *

Next: [Bytevector Procedures in R7RS](06_06_12_bytevectors.md#66129-bytevector-procedures-in-r7rs), Previous: [Accessing Bytevectors with the Array API](06_06_12_bytevectors.md#66127-accessing-bytevectors-with-the-array-api), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.8 Accessing Bytevectors with the SRFI-4 API [¶](06_06_12_bytevectors.md#66128-accessing-bytevectors-with-the-srfi-4-api)

Bytevectors may also be accessed with the SRFI-4 API. See [SRFI-4 - Relation to bytevectors](07_05_05_srfi4_homogeneous_numeric_vector_datatypes.md#7553-srfi-4---relation-to-bytevectors), for more information.

* * *

Next: [Bytevector Slices](06_06_12_bytevectors.md#661210-bytevector-slices), Previous: [Accessing Bytevectors with the SRFI-4 API](06_06_12_bytevectors.md#66128-accessing-bytevectors-with-the-srfi-4-api), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.9 Bytevector Procedures in R7RS [¶](06_06_12_bytevectors.md#66129-bytevector-procedures-in-r7rs)

The [R7RS](07_07_r7rs_support.md#77-r7rs-support) (Section 6.9) defines a set of bytevector manipulation procedures, accessible with

(use-modules (scheme base))

Of these, [`make-bytevector`](06_06_12_bytevectors.md), [`bytevector?`](06_06_12_bytevectors.md), [`bytevector-length`](06_06_12_bytevectors.md), [`bytevector-u8-ref`](06_06_12_bytevectors.md) and [`bytevector-u8-set!`](06_06_12_bytevectors.md) have the same definition as in R6RS. The procedures listed below either have a different definition in R7RS and R6RS, or are not defined in R6RS.

Scheme Procedure: **bytevector** arg … [¶](06_06_12_bytevectors.md)

Return a newly allocated bytevector composed of the given arguments. Analogous to `list`.

([bytevector](06_06_12_bytevectors.md) 2 3 4) ⇒ #vu8(2 3 4)

See also [`u8-list->bytevector`](06_06_12_bytevectors.md).

Scheme Procedure: **bytevector-copy** bv \[start \[end\]\] [¶](06_06_12_bytevectors.md)

Returns a newly allocated bytevector containing the elements of bv in the range \[start ... end). start defaults to 0 and end defaults to the length of bv.

(define bv #vu8(0 1 2 3 4 5))
([bytevector-copy](06_06_12_bytevectors.md) bv) ⇒ #vu8(0 1 2 3 4 5)
([bytevector-copy](06_06_12_bytevectors.md) bv 2) ⇒ #vu8(2 3 4 5)
([bytevector-copy](06_06_12_bytevectors.md) bv 2 4) ⇒ #vu8(2 3)

See also [the R6RS version](06_06_12_bytevectors.md).

Scheme Procedure: **bytevector-copy!** dst at src \[start \[end\]\] [¶](06_06_12_bytevectors.md)

Copy the block of elements from bytevector src in the range \[start ... end) into bytevector dst, starting at position at. start defaults to 0 and end defaults to the length of src. It is an error for dst to have a length less than at + (end - start).

See also [the R6RS version](06_06_12_bytevectors.md). With

([use-modules](06_18_modules.md) ((rnrs bytevectors) #:prefix r6:)
             ((scheme base) #:prefix r7:))

the following calls are equivalent:

(r6:bytevector-copy! source source-start target target-start len)
(r7:bytevector-copy! target target-start source source-start ([+](06_06_02_numerical_data_types.md) source-start len))

Scheme Procedure: **bytevector-append** arg … [¶](06_06_12_bytevectors.md)

Return a newly allocated bytevector whose characters form the concatenation of the given bytevectors arg ...

([bytevector-append](06_06_12_bytevectors.md) #vu8(0 1 2) #vu8(3 4 5))
⇒ #vu8(0 1 2 3 4 5)

* * *

Previous: [Bytevector Procedures in R7RS](06_06_12_bytevectors.md#66129-bytevector-procedures-in-r7rs), Up: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.12.10 Bytevector Slices [¶](06_06_12_bytevectors.md#661210-bytevector-slices)

As an extension to the R6RS specification, the `(rnrs bytevectors gnu)` module provides the `bytevector-slice` procedure, which returns a bytevector aliasing part of an existing bytevector.

Scheme Procedure: **bytevector-slice** bv offset \[size\] [¶](06_06_12_bytevectors.md)

C Function: **scm\_bytevector\_slice** (bv, offset, size) [¶](06_06_12_bytevectors.md)

Return the slice of bv starting at offset and counting size bytes. When size is omitted, the slice covers all of bv starting from offset. The returned slice shares storage with bv: changes to the slice are visible in bv and vice-versa.

When bv is actually a SRFI-4 uniform vector, its element type is preserved unless offset and size are not aligned on its element type size.

Here is an example showing how to use it:

([use-modules](06_18_modules.md) (rnrs bytevectors)
             (rnrs bytevectors gnu))

(define bv ([u8-list->bytevector](06_06_12_bytevectors.md) ([iota](07_05_03_srfi1_list_library.md) 10)))
(define slice ([bytevector-slice](06_06_12_bytevectors.md) bv 2 3))

slice
⇒ #vu8(2 3 4)

([bytevector-u8-set!](06_06_12_bytevectors.md) slice 0 77)
slice
⇒ #vu8(77 3 4)

bv
⇒ #vu8(0 1 77 3 4 5 6 7 8 9)

* * *

Next: [VLists](06_06_14_vlists.md#6614-vlists), Previous: [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
