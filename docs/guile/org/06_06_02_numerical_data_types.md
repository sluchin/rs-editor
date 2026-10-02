#### 6.6.2 Numerical data types [¶](06_06_02_numerical_data_types.md#662-numerical-data-types)

Guile supports a rich “tower” of numerical types — integer, rational, real and complex — and provides an extensive set of mathematical and scientific functions for operating on numerical data. This section of the manual documents those types and functions.

You may also find it illuminating to read R5RS’s presentation of numbers in Scheme, which is particularly clear and accessible: see [Numbers](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Numbers) in R5RS.

*   [Scheme’s Numerical “Tower”](06_06_02_numerical_data_types.md#6621-schemes-numerical-tower)
*   [Integers](06_06_02_numerical_data_types.md#6622-integers)
*   [Real and Rational Numbers](06_06_02_numerical_data_types.md#6623-real-and-rational-numbers)
*   [Complex Numbers](06_06_02_numerical_data_types.md#6624-complex-numbers)
*   [Exact and Inexact Numbers](06_06_02_numerical_data_types.md#6625-exact-and-inexact-numbers)
*   [Read Syntax for Numerical Data](06_06_02_numerical_data_types.md#6626-read-syntax-for-numerical-data)
*   [Operations on Integer Values](06_06_02_numerical_data_types.md#6627-operations-on-integer-values)
*   [Comparison Predicates](06_06_02_numerical_data_types.md#6628-comparison-predicates)
*   [Converting Numbers To and From Strings](06_06_02_numerical_data_types.md#6629-converting-numbers-to-and-from-strings)
*   [Complex Number Operations](06_06_02_numerical_data_types.md#66210-complex-number-operations)
*   [Arithmetic Functions](06_06_02_numerical_data_types.md#66211-arithmetic-functions)
*   [Scientific Functions](06_06_02_numerical_data_types.md#66212-scientific-functions)
*   [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations)
*   [Random Number Generation](06_06_02_numerical_data_types.md#66214-random-number-generation)

* * *

Next: [Integers](06_06_02_numerical_data_types.md#6622-integers), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.1 Scheme’s Numerical “Tower” [¶](06_06_02_numerical_data_types.md#6621-schemes-numerical-tower)

Scheme’s numerical “tower” consists of the following categories of numbers:

_integers_

Whole numbers, positive or negative; e.g. –5, 0, 18.

_rationals_

The set of numbers that can be expressed as _p/q_ where p and q are integers; e.g. _9/16_ works, but pi (an irrational number) doesn’t. These include integers (_n/1_).

_real numbers_

The set of numbers that describes all possible positions along a one-dimensional line. This includes rationals as well as irrational numbers.

_complex numbers_

The set of numbers that describes all possible positions in a two dimensional space. This includes real as well as imaginary numbers (_a+bi_, where a is the _real part_, b is the _imaginary part_, and _i_ is the square root of −1.)

It is called a tower because each category “sits on” the one that follows it, in the sense that every integer is also a rational, every rational is also real, and every real number is also a complex number (but with zero imaginary part).

In addition to the classification into integers, rationals, reals and complex numbers, Scheme also distinguishes between whether a number is represented exactly or not. For example, the result of _2\*sin(pi/4)_ is exactly _2^(1/2)_, but Guile can represent neither _pi/4_ nor _2^(1/2)_ exactly. Instead, it stores an inexact approximation, using the C type `double`.

Guile can represent exact rationals of any magnitude, inexact rationals that fit into a C `double`, and inexact complex numbers with `double` real and imaginary parts.

The `number?` predicate may be applied to any Scheme value to discover whether the value is any of the supported numerical types.

Scheme Procedure: **number?** obj [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_number\_p** (obj) [¶](06_06_02_numerical_data_types.md)

Return `#t` if obj is any kind of number, else `#f`.

For example:

([number?](06_06_02_numerical_data_types.md) 3)
⇒ #t

([number?](06_06_02_numerical_data_types.md) "hello there!")
⇒ #f

(define pi 3.141592654)
([number?](06_06_02_numerical_data_types.md) pi)
⇒ #t

C Function: `int` **scm\_is\_number** `(SCM obj)` [¶](06_06_02_numerical_data_types.md)

This is equivalent to `scm_is_true (scm_number_p (obj))`.

The next few subsections document each of Guile’s numerical data types in detail.

* * *

Next: [Real and Rational Numbers](06_06_02_numerical_data_types.md#6623-real-and-rational-numbers), Previous: [Scheme’s Numerical “Tower”](06_06_02_numerical_data_types.md#6621-schemes-numerical-tower), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.2 Integers [¶](06_06_02_numerical_data_types.md#6622-integers)

Integers are whole numbers, that is numbers with no fractional part, such as 2, 83, and −3789.

Integers in Guile can be arbitrarily big, as shown by the following example.

(define (factorial n)
  (let loop ((n n) (product 1))
    (if ([\=](06_06_02_numerical_data_types.md) n 0)
        product
        (loop ([\-](06_06_02_numerical_data_types.md) n 1) ([\*](06_06_02_numerical_data_types.md) product n)))))

(factorial 3)
⇒ 6

(factorial 20)
⇒ 2432902008176640000

([\-](06_06_02_numerical_data_types.md) (factorial 45))
⇒ \-119622220865480194561963161495657715064383733760000000000

Readers whose background is in programming languages where integers are limited by the need to fit into just 4 or 8 bytes of memory may find this surprising, or suspect that Guile’s representation of integers is inefficient. In fact, Guile achieves a near optimal balance of convenience and efficiency by using the host computer’s native representation of integers where possible, and a more general representation where the required number does not fit in the native form. Conversion between these two representations is automatic and completely invisible to the Scheme level programmer.

C has a host of different integer types, and Guile offers a host of functions to convert between them and the `SCM` representation. For example, a C `int` can be handled with `scm_to_int` and `scm_from_int`. Guile also defines a few C integer types of its own, to help with differences between systems.

C integer types that are not covered can be handled with the generic `scm_to_signed_integer` and `scm_from_signed_integer` for signed types, or with `scm_to_unsigned_integer` and `scm_from_unsigned_integer` for unsigned types.

Scheme integers can be exact and inexact. For example, a number written as `3.0` with an explicit decimal-point is inexact, but it is also an integer. The functions `integer?` and `scm_is_integer` report true for such a number, but the functions `exact-integer?`, `scm_is_exact_integer`, `scm_is_signed_integer`, and `scm_is_unsigned_integer` only allow exact integers and thus report false. Likewise, the conversion functions like `scm_to_signed_integer` only accept exact integers.

The motivation for this behavior is that the inexactness of a number should not be lost silently. If you want to allow inexact integers, you can explicitly insert a call to `inexact->exact` or to its C equivalent `scm_inexact_to_exact`. (Only inexact integers will be converted by this call into exact integers; inexact non-integers will become exact fractions.)

Scheme Procedure: **integer?** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_integer\_p** (x) [¶](06_06_02_numerical_data_types.md)

Return `#t` if x is an exact or inexact integer number, else return `#f`.

([integer?](06_06_02_numerical_data_types.md) 487)
⇒ #t

([integer?](06_06_02_numerical_data_types.md) 3.0)
⇒ #t

([integer?](06_06_02_numerical_data_types.md) \-3.4)
⇒ #f

([integer?](06_06_02_numerical_data_types.md) +inf.0)
⇒ #f

C Function: `int` **scm\_is\_integer** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

This is equivalent to `scm_is_true (scm_integer_p (x))`.

Scheme Procedure: **exact-integer?** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_exact\_integer\_p** (x) [¶](06_06_02_numerical_data_types.md)

Return `#t` if x is an exact integer number, else return `#f`.

([exact-integer?](06_06_02_numerical_data_types.md) 37)
⇒ #t

([exact-integer?](06_06_02_numerical_data_types.md) 3.0)
⇒ #f

C Function: `int` **scm\_is\_exact\_integer** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

This is equivalent to `scm_is_true (scm_exact_integer_p (x))`.

C Type: **scm\_t\_int8** [¶](06_06_02_numerical_data_types.md)

C Type: **scm\_t\_uint8** [¶](06_06_02_numerical_data_types.md)

C Type: **scm\_t\_int16** [¶](06_06_02_numerical_data_types.md)

C Type: **scm\_t\_uint16** [¶](06_06_02_numerical_data_types.md)

C Type: **scm\_t\_int32** [¶](06_06_02_numerical_data_types.md)

C Type: **scm\_t\_uint32** [¶](06_06_02_numerical_data_types.md)

C Type: **scm\_t\_int64** [¶](06_06_02_numerical_data_types.md)

C Type: **scm\_t\_uint64** [¶](06_06_02_numerical_data_types.md)

C Type: **scm\_t\_intmax** [¶](06_06_02_numerical_data_types.md)

C Type: **scm\_t\_uintmax** [¶](06_06_02_numerical_data_types.md)

The C types are equivalent to the corresponding ISO C types but are defined on all platforms, with the exception of `scm_t_int64` and `scm_t_uint64`, which are only defined when a 64-bit type is available. For example, `scm_t_int8` is equivalent to `int8_t`.

You can regard these definitions as a stop-gap measure until all platforms provide these types. If you know that all the platforms that you are interested in already provide these types, it is better to use them directly instead of the types provided by Guile.

C Function: `int` **scm\_is\_signed\_integer** `(SCM x, scm_t_intmax min, scm_t_intmax max)` [¶](06_06_02_numerical_data_types.md)

C Function: `int` **scm\_is\_unsigned\_integer** `(SCM x, scm_t_uintmax min, scm_t_uintmax max)` [¶](06_06_02_numerical_data_types.md)

Return `1` when x represents an exact integer that is between min and max, inclusive.

These functions can be used to check whether a `SCM` value will fit into a given range, such as the range of a given C integer type. If you just want to convert a `SCM` value to a given C integer type, use one of the conversion functions directly.

C Function: `scm_t_intmax` **scm\_to\_signed\_integer** `(SCM x, scm_t_intmax min, scm_t_intmax max)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_uintmax` **scm\_to\_unsigned\_integer** `(SCM x, scm_t_uintmax min, scm_t_uintmax max)` [¶](06_06_02_numerical_data_types.md)

When x represents an exact integer that is between min and max inclusive, return that integer. Else signal an error, either a ‘wrong-type’ error when x is not an exact integer, or an ‘out-of-range’ error when it doesn’t fit the given range.

C Function: `SCM` **scm\_from\_signed\_integer** `(scm_t_intmax x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_unsigned\_integer** `(scm_t_uintmax x)` [¶](06_06_02_numerical_data_types.md)

Return the `SCM` value that represents the integer x. This function will always succeed and will always return an exact number.

C Function: `char` **scm\_to\_char** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `signed char` **scm\_to\_schar** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `unsigned char` **scm\_to\_uchar** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `short` **scm\_to\_short** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `unsigned short` **scm\_to\_ushort** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `int` **scm\_to\_int** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `unsigned int` **scm\_to\_uint** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `long` **scm\_to\_long** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `unsigned long` **scm\_to\_ulong** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `long long` **scm\_to\_long\_long** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `unsigned long long` **scm\_to\_ulong\_long** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `size_t` **scm\_to\_size\_t** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `ssize_t` **scm\_to\_ssize\_t** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_uintptr` **scm\_to\_uintptr\_t** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_ptrdiff` **scm\_to\_ptrdiff\_t** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_int8` **scm\_to\_int8** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_uint8` **scm\_to\_uint8** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_int16` **scm\_to\_int16** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_uint16` **scm\_to\_uint16** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_int32` **scm\_to\_int32** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_uint32` **scm\_to\_uint32** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_int64` **scm\_to\_int64** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_uint64` **scm\_to\_uint64** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_intmax` **scm\_to\_intmax** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_uintmax` **scm\_to\_uintmax** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_intptr` **scm\_to\_intptr\_t** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

C Function: `scm_t_uintptr` **scm\_to\_uintptr\_t** `(SCM x)` [¶](06_06_02_numerical_data_types.md)

When x represents an exact integer that fits into the indicated C type, return that integer. Else signal an error, either a ‘wrong-type’ error when x is not an exact integer, or an ‘out-of-range’ error when it doesn’t fit the given range.

The functions `scm_to_long_long`, `scm_to_ulong_long`, `scm_to_int64`, and `scm_to_uint64` are only available when the corresponding types are.

C Function: `SCM` **scm\_from\_char** `(char x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_schar** `(signed char x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_uchar** `(unsigned char x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_short** `(short x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_ushort** `(unsigned short x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_int** `(int x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_uint** `(unsigned int x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_long** `(long x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_ulong** `(unsigned long x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_long\_long** `(long long x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_ulong\_long** `(unsigned long long x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_size\_t** `(size_t x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_ssize\_t** `(ssize_t x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_uintptr\_t** `(uintptr_t x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_ptrdiff\_t** `(scm_t_ptrdiff x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_int8** `(scm_t_int8 x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_uint8** `(scm_t_uint8 x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_int16** `(scm_t_int16 x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_uint16** `(scm_t_uint16 x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_int32** `(scm_t_int32 x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_uint32** `(scm_t_uint32 x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_int64** `(scm_t_int64 x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_uint64** `(scm_t_uint64 x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_intmax** `(scm_t_intmax x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_uintmax** `(scm_t_uintmax x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_intptr\_t** `(scm_t_intptr x)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_from\_uintptr\_t** `(scm_t_uintptr x)` [¶](06_06_02_numerical_data_types.md)

Return the `SCM` value that represents the integer x. These functions will always succeed and will always return an exact number.

C Function: `void` **scm\_to\_mpz** `(SCM val, mpz_t rop)` [¶](06_06_02_numerical_data_types.md)

Assign val to the multiple precision integer rop. val must be an exact integer, otherwise an error will be signaled. rop must have been initialized with `mpz_init` before this function is called. When rop is no longer needed the occupied space must be freed with `mpz_clear`. See [Initializing Integers](https://www.gmplib.org/manual/Initializing-Integers.html#Initializing-Integers) in GNU MP Manual, for details.

C Function: `SCM` **scm\_from\_mpz** `(mpz_t val)` [¶](06_06_02_numerical_data_types.md)

Return the `SCM` value that represents val.

* * *

Next: [Complex Numbers](06_06_02_numerical_data_types.md#6624-complex-numbers), Previous: [Integers](06_06_02_numerical_data_types.md#6622-integers), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.3 Real and Rational Numbers [¶](06_06_02_numerical_data_types.md#6623-real-and-rational-numbers)

Mathematically, the real numbers are the set of numbers that describe all possible points along a continuous, infinite, one-dimensional line. The rational numbers are the set of all numbers that can be written as fractions p/q, where p and q are integers. All rational numbers are also real, but there are real numbers that are not rational, for example _the square root of 2_, and _pi_.

Guile can represent both exact and inexact rational numbers, but it cannot represent precise finite irrational numbers. Exact rationals are represented by storing the numerator and denominator as two exact integers. Inexact rationals are stored as floating point numbers using the C type `double`.

Exact rationals are written as a fraction of integers. There must be no whitespace around the slash:

1/2
\-22/7

Even though the actual encoding of inexact rationals is in binary, it may be helpful to think of it as a decimal number with a limited number of significant figures and a decimal point somewhere, since this corresponds to the standard notation for non-whole numbers. For example:

0.34
\-0.00000142857931198
\-5648394822220000000000.0
4.0

The limited precision of Guile’s encoding means that any finite “real” number in Guile can be written in a rational form, by multiplying and then dividing by sufficient powers of 10 (or in fact, 2). For example, ‘\-0.00000142857931198’ is the same as −142857931198 divided by 100000000000000000. In Guile’s current incarnation, therefore, the `rational?` and `real?` predicates are equivalent for finite numbers.

Dividing by an exact zero leads to an error message, as one might expect. However, dividing by an inexact zero does not produce an error. Instead, the result of the division is either plus or minus infinity, depending on the sign of the divided number and the sign of the zero divisor (some platforms support signed zeroes ‘\-0.0’ and ‘+0.0’; ‘0.0’ is the same as ‘+0.0’).

Dividing zero by an inexact zero yields a NaN (‘not a number’) value, although they are actually considered numbers by Scheme. Attempts to compare a NaN value with any number (including itself) using `=`, `<`, `>`, `<=` or `>=` always returns `#f`. Although a NaN value is not `=` to itself, it is both `eqv?` and `equal?` to itself and other NaN values. However, the preferred way to test for them is by using `nan?`.

The real NaN values and infinities are written ‘+nan.0’, ‘+inf.0’ and ‘\-inf.0’. This syntax is also recognized by `read` as an extension to the usual Scheme syntax. These special values are considered by Scheme to be inexact real numbers but not rational. Note that non-real complex numbers may also contain infinities or NaN values in their real or imaginary parts. To test a real number to see if it is infinite, a NaN value, or neither, use `inf?`, `nan?`, or `finite?`, respectively. Every real number in Scheme belongs to precisely one of those three classes.

On platforms that follow IEEE 754 for their floating point arithmetic, the ‘+inf.0’, ‘\-inf.0’, and ‘+nan.0’ values are implemented using the corresponding IEEE 754 values. They behave in arithmetic operations like IEEE 754 describes it, i.e., `(= +nan.0 +nan.0)` ⇒ `#f`.

Scheme Procedure: **real?** obj [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_real\_p** (obj) [¶](06_06_02_numerical_data_types.md)

Return `#t` if obj is a real number, else `#f`. Note that the sets of integer and rational values form subsets of the set of real numbers, so the predicate will also be fulfilled if obj is an integer number or a rational number.

Scheme Procedure: **rational?** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_rational\_p** (x) [¶](06_06_02_numerical_data_types.md)

Return `#t` if x is a rational number, `#f` otherwise. Note that the set of integer values forms a subset of the set of rational numbers, i.e. the predicate will also be fulfilled if x is an integer number.

Scheme Procedure: **rationalize** x eps [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_rationalize** (x, eps) [¶](06_06_02_numerical_data_types.md)

Returns the _simplest_ rational number differing from x by no more than eps.

As required by R5RS, `rationalize` only returns an exact result when both its arguments are exact. Thus, you might need to use `inexact->exact` on the arguments.

([rationalize](06_06_02_numerical_data_types.md) ([inexact->exact](06_06_02_numerical_data_types.md) 1.2) 1/100)
⇒ 6/5

Scheme Procedure: **inf?** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_inf\_p** (x) [¶](06_06_02_numerical_data_types.md)

Return `#t` if the real number x is ‘+inf.0’ or ‘\-inf.0’. Otherwise return `#f`.

Scheme Procedure: **nan?** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_nan\_p** (x) [¶](06_06_02_numerical_data_types.md)

Return `#t` if the real number x is ‘+nan.0’, or `#f` otherwise.

Scheme Procedure: **finite?** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_finite\_p** (x) [¶](06_06_02_numerical_data_types.md)

Return `#t` if the real number x is neither infinite nor a NaN, `#f` otherwise.

Scheme Procedure: **nan** [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_nan** () [¶](06_06_02_numerical_data_types.md)

Return ‘+nan.0’, a NaN value.

Scheme Procedure: **inf** [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_inf** () [¶](06_06_02_numerical_data_types.md)

Return ‘+inf.0’, positive infinity.

Scheme Procedure: **numerator** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_numerator** (x) [¶](06_06_02_numerical_data_types.md)

Return the numerator of the rational number x.

Scheme Procedure: **denominator** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_denominator** (x) [¶](06_06_02_numerical_data_types.md)

Return the denominator of the rational number x.

C Function: `int` **scm\_is\_real** `(SCM val)` [¶](06_06_02_numerical_data_types.md)

C Function: `int` **scm\_is\_rational** `(SCM val)` [¶](06_06_02_numerical_data_types.md)

Equivalent to `scm_is_true (scm_real_p (val))` and `scm_is_true (scm_rational_p (val))`, respectively.

C Function: `double` **scm\_to\_double** `(SCM val)` [¶](06_06_02_numerical_data_types.md)

Returns the number closest to val that is representable as a `double`. Returns infinity for a val that is too large in magnitude. The argument val must be a real number.

C Function: `SCM` **scm\_from\_double** `(double val)` [¶](06_06_02_numerical_data_types.md)

Return the `SCM` value that represents val. The returned value is inexact according to the predicate `inexact?`, but it will be exactly equal to val.

* * *

Next: [Exact and Inexact Numbers](06_06_02_numerical_data_types.md#6625-exact-and-inexact-numbers), Previous: [Real and Rational Numbers](06_06_02_numerical_data_types.md#6623-real-and-rational-numbers), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.4 Complex Numbers [¶](06_06_02_numerical_data_types.md#6624-complex-numbers)

Complex numbers are the set of numbers that describe all possible points in a two-dimensional space. The two coordinates of a particular point in this space are known as the _real_ and _imaginary_ parts of the complex number that describes that point.

In Guile, complex numbers are written in rectangular form as the sum of their real and imaginary parts, using the symbol `i` to indicate the imaginary part.

3+4i
⇒
3.0+4.0i

([\*](06_06_02_numerical_data_types.md) 3-8i 2.3+0.3i)
⇒
9.3-17.5i

Polar form can also be used, with an ‘@’ between magnitude and angle,

1@3.141592 ⇒ \-1.0      (approx)
\-1@1.57079 ⇒ 0.0-1.0i  (approx)

Guile represents a complex number as a pair of inexact reals, so the real and imaginary parts of a complex number have the same properties of inexactness and limited precision as single inexact real numbers.

Note that each part of a complex number may contain any inexact real value, including the special values ‘+nan.0’, ‘+inf.0’ and ‘\-inf.0’, as well as either of the signed zeroes ‘0.0’ or ‘\-0.0’.

Scheme Procedure: **complex?** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_complex\_p** (z) [¶](06_06_02_numerical_data_types.md)

Return `#t` if z is a complex number, `#f` otherwise. Note that the sets of real, rational and integer values form subsets of the set of complex numbers, i.e. the predicate will also be fulfilled if z is a real, rational or integer number.

C Function: `int` **scm\_is\_complex** `(SCM val)` [¶](06_06_02_numerical_data_types.md)

Equivalent to `scm_is_true (scm_complex_p (val))`.

* * *

Next: [Read Syntax for Numerical Data](06_06_02_numerical_data_types.md#6626-read-syntax-for-numerical-data), Previous: [Complex Numbers](06_06_02_numerical_data_types.md#6624-complex-numbers), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.5 Exact and Inexact Numbers [¶](06_06_02_numerical_data_types.md#6625-exact-and-inexact-numbers)

R5RS requires that, with few exceptions, a calculation involving inexact numbers always produces an inexact result. To meet this requirement, Guile distinguishes between an exact integer value such as ‘5’ and the corresponding inexact integer value which, to the limited precision available, has no fractional part, and is printed as ‘5.0’. Guile will only convert the latter value to the former when forced to do so by an invocation of the `inexact->exact` procedure.

The only exception to the above requirement is when the values of the inexact numbers do not affect the result. For example `(expt n 0)` is ‘1’ for any value of `n`, therefore `(expt 5.0 0)` is permitted to return an exact ‘1’.

Scheme Procedure: **exact?** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_exact\_p** (z) [¶](06_06_02_numerical_data_types.md)

Return `#t` if the number z is exact, `#f` otherwise.

([exact?](06_06_02_numerical_data_types.md) 2)
⇒ #t

([exact?](06_06_02_numerical_data_types.md) 0.5)
⇒ #f

([exact?](06_06_02_numerical_data_types.md) ([/](06_06_02_numerical_data_types.md) 2))
⇒ #t

C Function: `int` **scm\_is\_exact** `(SCM z)` [¶](06_06_02_numerical_data_types.md)

Return a `1` if the number z is exact, and `0` otherwise. This is equivalent to `scm_is_true (scm_exact_p (z))`.

An alternate approach to testing the exactness of a number is to use `scm_is_signed_integer` or `scm_is_unsigned_integer`.

Scheme Procedure: **inexact?** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_inexact\_p** (z) [¶](06_06_02_numerical_data_types.md)

Return `#t` if the number z is inexact, `#f` else.

C Function: `int` **scm\_is\_inexact** `(SCM z)` [¶](06_06_02_numerical_data_types.md)

Return a `1` if the number z is inexact, and `0` otherwise. This is equivalent to `scm_is_true (scm_inexact_p (z))`.

Scheme Procedure: **inexact->exact** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_inexact\_to\_exact** (z) [¶](06_06_02_numerical_data_types.md)

Return an exact number that is numerically closest to z, when there is one. For inexact rationals, Guile returns the exact rational that is numerically equal to the inexact rational. Inexact complex numbers with a non-zero imaginary part can not be made exact.

([inexact->exact](06_06_02_numerical_data_types.md) 0.5)
⇒ 1/2

The following happens because 12/10 is not exactly representable as a `double` (on most platforms). However, when reading a decimal number that has been marked exact with the “#e” prefix, Guile is able to represent it correctly.

([inexact->exact](06_06_02_numerical_data_types.md) 1.2)
⇒ 5404319552844595/4503599627370496

#e1.2
⇒ 6/5

Scheme Procedure: **exact->inexact** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_exact\_to\_inexact** (z) [¶](06_06_02_numerical_data_types.md)

Convert the number z to its inexact representation.

* * *

Next: [Operations on Integer Values](06_06_02_numerical_data_types.md#6627-operations-on-integer-values), Previous: [Exact and Inexact Numbers](06_06_02_numerical_data_types.md#6625-exact-and-inexact-numbers), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.6 Read Syntax for Numerical Data [¶](06_06_02_numerical_data_types.md#6626-read-syntax-for-numerical-data)

The read syntax for integers is a string of digits, optionally preceded by a minus or plus character, a code indicating the base in which the integer is encoded, and a code indicating whether the number is exact or inexact. The supported base codes are:

`#b`

`#B`

the integer is written in binary (base 2)

`#o`

`#O`

the integer is written in octal (base 8)

`#d`

`#D`

the integer is written in decimal (base 10)

`#x`

`#X`

the integer is written in hexadecimal (base 16)

If the base code is omitted, the integer is assumed to be decimal. The following examples show how these base codes are used.

\-13
⇒ \-13

#d-13
⇒ \-13

#x-13
⇒ \-19

#b+1101
⇒ 13

#o377
⇒ 255

The codes for indicating exactness (which can, incidentally, be applied to all numerical values) are:

`#e`

`#E`

the number is exact

`#i`

`#I`

the number is inexact.

If the exactness indicator is omitted, the number is exact unless it contains a radix point. Since Guile can not represent exact complex numbers, an error is signaled when asking for them.

([exact?](06_06_02_numerical_data_types.md) 1.2)
⇒ #f

([exact?](06_06_02_numerical_data_types.md) #e1.2)
⇒ #t

([exact?](06_06_02_numerical_data_types.md) #e+1i)
ERROR: Wrong type argument

Guile also understands the syntax ‘+inf.0’ and ‘\-inf.0’ for plus and minus infinity, respectively. The value must be written exactly as shown, that is, they always must have a sign and exactly one zero digit after the decimal point. It also understands ‘+nan.0’ and ‘\-nan.0’ for the special ‘not-a-number’ value. The sign is ignored for ‘not-a-number’ and the value is always printed as ‘+nan.0’.

* * *

Next: [Comparison Predicates](06_06_02_numerical_data_types.md#6628-comparison-predicates), Previous: [Read Syntax for Numerical Data](06_06_02_numerical_data_types.md#6626-read-syntax-for-numerical-data), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.7 Operations on Integer Values [¶](06_06_02_numerical_data_types.md#6627-operations-on-integer-values)

Scheme Procedure: **odd?** n [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_odd\_p** (n) [¶](06_06_02_numerical_data_types.md)

Return `#t` if n is an odd number, `#f` otherwise.

Scheme Procedure: **even?** n [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_even\_p** (n) [¶](06_06_02_numerical_data_types.md)

Return `#t` if n is an even number, `#f` otherwise.

Scheme Procedure: **quotient** n d [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **remainder** n d [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_quotient** (n, d) [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_remainder** (n, d) [¶](06_06_02_numerical_data_types.md)

Return the quotient or remainder from n divided by d. The quotient is rounded towards zero, and the remainder will have the same sign as n. In all cases quotient and remainder satisfy _n = q\*d + r_.

([remainder](06_06_02_numerical_data_types.md) 13 4) ⇒ 1
([remainder](06_06_02_numerical_data_types.md) \-13 4) ⇒ \-1

See also `truncate-quotient`, `truncate-remainder` and related operations in [Arithmetic Functions](06_06_02_numerical_data_types.md#66211-arithmetic-functions).

Scheme Procedure: **modulo** n d [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_modulo** (n, d) [¶](06_06_02_numerical_data_types.md)

Return the remainder from n divided by d, with the same sign as d.

([modulo](06_06_02_numerical_data_types.md) 13 4) ⇒ 1
([modulo](06_06_02_numerical_data_types.md) \-13 4) ⇒ 3
([modulo](06_06_02_numerical_data_types.md) 13 \-4) ⇒ \-3
([modulo](06_06_02_numerical_data_types.md) \-13 \-4) ⇒ \-1

See also `floor-quotient`, `floor-remainder` and related operations in [Arithmetic Functions](06_06_02_numerical_data_types.md#66211-arithmetic-functions).

Scheme Procedure: **gcd** x… [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_gcd** (x, y) [¶](06_06_02_numerical_data_types.md)

Return the greatest common divisor of all arguments. If called without arguments, 0 is returned.

The C function `scm_gcd` always takes two arguments, while the Scheme function can take an arbitrary number.

Scheme Procedure: **lcm** x… [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_lcm** (x, y) [¶](06_06_02_numerical_data_types.md)

Return the least common multiple of the arguments. If called without arguments, 1 is returned.

The C function `scm_lcm` always takes two arguments, while the Scheme function can take an arbitrary number.

Scheme Procedure: **modulo-expt** n k m [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_modulo\_expt** (n, k, m) [¶](06_06_02_numerical_data_types.md)

Return n raised to the integer exponent k, modulo m.

([modulo-expt](06_06_02_numerical_data_types.md) 2 3 5)
   ⇒ 3

Scheme Procedure: **exact-integer-sqrt** `k` [¶](06_06_02_numerical_data_types.md)

C Function: `void` **scm\_exact\_integer\_sqrt** `(SCM k, SCM *s, SCM *r)` [¶](06_06_02_numerical_data_types.md)

Return two exact non-negative integers s and r such that _k = s^2 + r_ and _s^2 <= k < (s + 1)^2_. An error is raised if k is not an exact non-negative integer.

([exact-integer-sqrt](06_06_02_numerical_data_types.md) 10) ⇒ 3 and 1

* * *

Next: [Converting Numbers To and From Strings](06_06_02_numerical_data_types.md#6629-converting-numbers-to-and-from-strings), Previous: [Operations on Integer Values](06_06_02_numerical_data_types.md#6627-operations-on-integer-values), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.8 Comparison Predicates [¶](06_06_02_numerical_data_types.md#6628-comparison-predicates)

The C comparison functions below always takes two arguments, while the Scheme functions can take an arbitrary number. Also keep in mind that the C functions return one of the Scheme boolean values `SCM_BOOL_T` or `SCM_BOOL_F` which are both true as far as C is concerned. Thus, always write `scm_is_true (scm_num_eq_p (x, y))` when testing the two Scheme numbers `x` and `y` for equality, for example.

Scheme Procedure: **\=** [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_num\_eq\_p** (x, y) [¶](06_06_02_numerical_data_types.md)

Return `#t` if all parameters are numerically equal.

Scheme Procedure: **<** [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_less\_p** (x, y) [¶](06_06_02_numerical_data_types.md)

Return `#t` if the list of parameters is monotonically increasing.

Scheme Procedure: **\>** [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_gr\_p** (x, y) [¶](06_06_02_numerical_data_types.md)

Return `#t` if the list of parameters is monotonically decreasing.

Scheme Procedure: **<=** [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_leq\_p** (x, y) [¶](06_06_02_numerical_data_types.md)

Return `#t` if the list of parameters is monotonically non-decreasing.

Scheme Procedure: **\>=** [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_geq\_p** (x, y) [¶](06_06_02_numerical_data_types.md)

Return `#t` if the list of parameters is monotonically non-increasing.

Scheme Procedure: **zero?** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_zero\_p** (z) [¶](06_06_02_numerical_data_types.md)

Return `#t` if z is an exact or inexact number equal to zero.

Scheme Procedure: **positive?** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_positive\_p** (x) [¶](06_06_02_numerical_data_types.md)

Return `#t` if x is an exact or inexact number greater than zero.

Scheme Procedure: **negative?** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_negative\_p** (x) [¶](06_06_02_numerical_data_types.md)

Return `#t` if x is an exact or inexact number less than zero.

* * *

Next: [Complex Number Operations](06_06_02_numerical_data_types.md#66210-complex-number-operations), Previous: [Comparison Predicates](06_06_02_numerical_data_types.md#6628-comparison-predicates), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.9 Converting Numbers To and From Strings [¶](06_06_02_numerical_data_types.md#6629-converting-numbers-to-and-from-strings)

The following procedures read and write numbers according to their external representation as defined by R5RS (see [R5RS Lexical Structure](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Lexical-structure) in The Revised^5 Report on the Algorithmic Language Scheme). See [the `(ice-9 i18n)` module](06_25_support_for_internationalization.md#6254-number-input-and-output), for locale-dependent number parsing.

Scheme Procedure: **number->string** n \[radix\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_number\_to\_string** (n, radix) [¶](06_06_02_numerical_data_types.md)

Return a string holding the external representation of the number n in the given radix. If n is inexact, a radix of 10 will be used.

Scheme Procedure: **string->number** string \[radix\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_string\_to\_number** (string, radix) [¶](06_06_02_numerical_data_types.md)

Return a number of the maximally precise representation expressed by the given string. radix must be an exact integer, either 2, 8, 10, or 16. If supplied, radix is a default radix that may be overridden by an explicit radix prefix in string (e.g. "#o177"). If radix is not supplied, then the default radix is 10. If string is not a syntactically valid notation for a number, then `string->number` returns `#f`.

C Function: `SCM` **scm\_c\_locale\_stringn\_to\_number** `(const char *string, size_t len, unsigned radix)` [¶](06_06_02_numerical_data_types.md)

As per `string->number` above, but taking a C string, as pointer and length. The string characters should be in the current locale encoding (`locale` in the name refers only to that, there’s no locale-dependent parsing).

* * *

Next: [Arithmetic Functions](06_06_02_numerical_data_types.md#66211-arithmetic-functions), Previous: [Converting Numbers To and From Strings](06_06_02_numerical_data_types.md#6629-converting-numbers-to-and-from-strings), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.10 Complex Number Operations [¶](06_06_02_numerical_data_types.md#66210-complex-number-operations)

Scheme Procedure: **make-rectangular** real\_part imaginary\_part [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_make\_rectangular** (real\_part, imaginary\_part) [¶](06_06_02_numerical_data_types.md)

Return a complex number constructed of the given real-part and imaginary-part parts.

Scheme Procedure: **make-polar** mag ang [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_make\_polar** (mag, ang) [¶](06_06_02_numerical_data_types.md)

Return the complex number mag \* e^(i \* ang).

Scheme Procedure: **real-part** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_real\_part** (z) [¶](06_06_02_numerical_data_types.md)

Return the real part of the number z.

Scheme Procedure: **imag-part** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_imag\_part** (z) [¶](06_06_02_numerical_data_types.md)

Return the imaginary part of the number z.

Scheme Procedure: **magnitude** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_magnitude** (z) [¶](06_06_02_numerical_data_types.md)

Return the magnitude of the number z. This is the same as `abs` for real arguments, but also allows complex numbers.

Scheme Procedure: **angle** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_angle** (z) [¶](06_06_02_numerical_data_types.md)

Return the angle of the complex number z.

C Function: `SCM` **scm\_c\_make\_rectangular** `(double re, double im)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_c\_make\_polar** `(double x, double y)` [¶](06_06_02_numerical_data_types.md)

Like `scm_make_rectangular` or `scm_make_polar`, respectively, but these functions take `double`s as their arguments.

C Function: `double` **scm\_c\_real\_part** `(z)` [¶](06_06_02_numerical_data_types.md)

C Function: `double` **scm\_c\_imag\_part** `(z)` [¶](06_06_02_numerical_data_types.md)

Returns the real or imaginary part of z as a `double`.

C Function: `double` **scm\_c\_magnitude** `(z)` [¶](06_06_02_numerical_data_types.md)

C Function: `double` **scm\_c\_angle** `(z)` [¶](06_06_02_numerical_data_types.md)

Returns the magnitude or angle of z as a `double`.

* * *

Next: [Scientific Functions](06_06_02_numerical_data_types.md#66212-scientific-functions), Previous: [Complex Number Operations](06_06_02_numerical_data_types.md#66210-complex-number-operations), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.11 Arithmetic Functions [¶](06_06_02_numerical_data_types.md#66211-arithmetic-functions)

The C arithmetic functions below always takes two arguments, while the Scheme functions can take an arbitrary number. When you need to invoke them with just one argument, for example to compute the equivalent of `(- x)`, pass `SCM_UNDEFINED` as the second one: `scm_difference (x, SCM_UNDEFINED)`.

Scheme Procedure: **+** z1 … [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_sum** (z1, z2) [¶](06_06_02_numerical_data_types.md)

Return the sum of all parameter values. Return 0 if called without any parameters.

Scheme Procedure: **\-** z1 z2 … [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_difference** (z1, z2) [¶](06_06_02_numerical_data_types.md)

If called with one argument z1, -z1 is returned. Otherwise the sum of all but the first argument are subtracted from the first argument.

Scheme Procedure: **\*** z1 … [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_product** (z1, z2) [¶](06_06_02_numerical_data_types.md)

Return the product of all arguments. If called without arguments, 1 is returned.

Scheme Procedure: **/** z1 z2 … [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_divide** (z1, z2) [¶](06_06_02_numerical_data_types.md)

Divide the first argument by the product of the remaining arguments. If called with one argument z1, 1/z1 is returned.

Scheme Procedure: **1+** z [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_oneplus** (z) [¶](06_06_02_numerical_data_types.md)

Return _z + 1_.

Scheme Procedure: **1-** z [¶](06_06_02_numerical_data_types.md)

C function: **scm\_oneminus** (z) [¶](06_06_02_numerical_data_types.md)

Return _z - 1_.

Scheme Procedure: **abs** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_abs** (x) [¶](06_06_02_numerical_data_types.md)

Return the absolute value of x.

x must be a number with zero imaginary part. To calculate the magnitude of a complex number, use `magnitude` instead.

Scheme Procedure: **max** x1 x2 … [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_max** (x1, x2) [¶](06_06_02_numerical_data_types.md)

Return the maximum of all parameter values.

Scheme Procedure: **min** x1 x2 … [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_min** (x1, x2) [¶](06_06_02_numerical_data_types.md)

Return the minimum of all parameter values.

Scheme Procedure: **truncate** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_truncate\_number** (x) [¶](06_06_02_numerical_data_types.md)

Round the inexact number x towards zero.

Scheme Procedure: **round** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_round\_number** (x) [¶](06_06_02_numerical_data_types.md)

Round the inexact number x to the nearest integer. When exactly halfway between two integers, round to the even one.

Scheme Procedure: **floor** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_floor** (x) [¶](06_06_02_numerical_data_types.md)

Round the number x towards minus infinity.

Scheme Procedure: **ceiling** x [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_ceiling** (x) [¶](06_06_02_numerical_data_types.md)

Round the number x towards infinity.

C Function: `double` **scm\_c\_truncate** `(double x)` [¶](06_06_02_numerical_data_types.md)

C Function: `double` **scm\_c\_round** `(double x)` [¶](06_06_02_numerical_data_types.md)

Like `scm_truncate_number` or `scm_round_number`, respectively, but these functions take and return `double` values.

Scheme Procedure: **euclidean/** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **euclidean-quotient** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **euclidean-remainder** `x y` [¶](06_06_02_numerical_data_types.md)

C Function: `void` **scm\_euclidean\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_euclidean\_quotient** `(SCM x, SCM y)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_euclidean\_remainder** `(SCM x, SCM y)` [¶](06_06_02_numerical_data_types.md)

These procedures accept two real numbers x and y, where the divisor y must be non-zero. `euclidean-quotient` returns the integer q and `euclidean-remainder` returns the real number r such that _x = q\*y + r_ and _0 <= r < |y|_. `euclidean/` returns both q and r, and is more efficient than computing each separately. Note that when _y > 0_, `euclidean-quotient` returns _floor(x/y)_, otherwise it returns _ceiling(x/y)_.

Note that these operators are equivalent to the R6RS operators `div`, `mod`, and `div-and-mod`.

([euclidean-quotient](06_06_02_numerical_data_types.md) 123 10) ⇒ 12
([euclidean-remainder](06_06_02_numerical_data_types.md) 123 10) ⇒ 3
([euclidean/](06_06_02_numerical_data_types.md) 123 10) ⇒ 12 and 3
([euclidean/](06_06_02_numerical_data_types.md) 123 \-10) ⇒ \-12 and 3
([euclidean/](06_06_02_numerical_data_types.md) \-123 10) ⇒ \-13 and 7
([euclidean/](06_06_02_numerical_data_types.md) \-123 \-10) ⇒ 13 and 7
([euclidean/](06_06_02_numerical_data_types.md) \-123.2 \-63.5) ⇒ 2.0 and 3.8
([euclidean/](06_06_02_numerical_data_types.md) 16/3 \-10/7) ⇒ \-3 and 22/21

Scheme Procedure: **floor/** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **floor-quotient** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **floor-remainder** `x y` [¶](06_06_02_numerical_data_types.md)

C Function: `void` **scm\_floor\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_floor\_quotient** `(x, y)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_floor\_remainder** `(x, y)` [¶](06_06_02_numerical_data_types.md)

These procedures accept two real numbers x and y, where the divisor y must be non-zero. `floor-quotient` returns the integer q and `floor-remainder` returns the real number r such that _q = floor(x/y)_ and _x = q\*y + r_. `floor/` returns both q and r, and is more efficient than computing each separately. Note that r, if non-zero, will have the same sign as y.

When x and y are integers, `floor-remainder` is equivalent to the R5RS integer-only operator `modulo`.

([floor-quotient](06_06_02_numerical_data_types.md) 123 10) ⇒ 12
([floor-remainder](06_06_02_numerical_data_types.md) 123 10) ⇒ 3
([floor/](06_06_02_numerical_data_types.md) 123 10) ⇒ 12 and 3
([floor/](06_06_02_numerical_data_types.md) 123 \-10) ⇒ \-13 and \-7
([floor/](06_06_02_numerical_data_types.md) \-123 10) ⇒ \-13 and 7
([floor/](06_06_02_numerical_data_types.md) \-123 \-10) ⇒ 12 and \-3
([floor/](06_06_02_numerical_data_types.md) \-123.2 \-63.5) ⇒ 1.0 and \-59.7
([floor/](06_06_02_numerical_data_types.md) 16/3 \-10/7) ⇒ \-4 and \-8/21

Scheme Procedure: **ceiling/** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **ceiling-quotient** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **ceiling-remainder** `x y` [¶](06_06_02_numerical_data_types.md)

C Function: `void` **scm\_ceiling\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_ceiling\_quotient** `(x, y)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_ceiling\_remainder** `(x, y)` [¶](06_06_02_numerical_data_types.md)

These procedures accept two real numbers x and y, where the divisor y must be non-zero. `ceiling-quotient` returns the integer q and `ceiling-remainder` returns the real number r such that _q = ceiling(x/y)_ and _x = q\*y + r_. `ceiling/` returns both q and r, and is more efficient than computing each separately. Note that r, if non-zero, will have the opposite sign of y.

([ceiling-quotient](06_06_02_numerical_data_types.md) 123 10) ⇒ 13
([ceiling-remainder](06_06_02_numerical_data_types.md) 123 10) ⇒ \-7
([ceiling/](06_06_02_numerical_data_types.md) 123 10) ⇒ 13 and \-7
([ceiling/](06_06_02_numerical_data_types.md) 123 \-10) ⇒ \-12 and 3
([ceiling/](06_06_02_numerical_data_types.md) \-123 10) ⇒ \-12 and \-3
([ceiling/](06_06_02_numerical_data_types.md) \-123 \-10) ⇒ 13 and 7
([ceiling/](06_06_02_numerical_data_types.md) \-123.2 \-63.5) ⇒ 2.0 and 3.8
([ceiling/](06_06_02_numerical_data_types.md) 16/3 \-10/7) ⇒ \-3 and 22/21

Scheme Procedure: **truncate/** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **truncate-quotient** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **truncate-remainder** `x y` [¶](06_06_02_numerical_data_types.md)

C Function: `void` **scm\_truncate\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_truncate\_quotient** `(x, y)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_truncate\_remainder** `(x, y)` [¶](06_06_02_numerical_data_types.md)

These procedures accept two real numbers x and y, where the divisor y must be non-zero. `truncate-quotient` returns the integer q and `truncate-remainder` returns the real number r such that q is _x/y_ rounded toward zero, and _x = q\*y + r_. `truncate/` returns both q and r, and is more efficient than computing each separately. Note that r, if non-zero, will have the same sign as x.

When x and y are integers, these operators are equivalent to the R5RS integer-only operators `quotient` and `remainder`.

([truncate-quotient](06_06_02_numerical_data_types.md) 123 10) ⇒ 12
([truncate-remainder](06_06_02_numerical_data_types.md) 123 10) ⇒ 3
([truncate/](06_06_02_numerical_data_types.md) 123 10) ⇒ 12 and 3
([truncate/](06_06_02_numerical_data_types.md) 123 \-10) ⇒ \-12 and 3
([truncate/](06_06_02_numerical_data_types.md) \-123 10) ⇒ \-12 and \-3
([truncate/](06_06_02_numerical_data_types.md) \-123 \-10) ⇒ 12 and \-3
([truncate/](06_06_02_numerical_data_types.md) \-123.2 \-63.5) ⇒ 1.0 and \-59.7
([truncate/](06_06_02_numerical_data_types.md) 16/3 \-10/7) ⇒ \-3 and 22/21

Scheme Procedure: **centered/** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **centered-quotient** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **centered-remainder** `x y` [¶](06_06_02_numerical_data_types.md)

C Function: `void` **scm\_centered\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_centered\_quotient** `(SCM x, SCM y)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_centered\_remainder** `(SCM x, SCM y)` [¶](06_06_02_numerical_data_types.md)

These procedures accept two real numbers x and y, where the divisor y must be non-zero. `centered-quotient` returns the integer q and `centered-remainder` returns the real number r such that _x = q\*y + r_ and _\-|y/2| <= r < |y/2|_. `centered/` returns both q and r, and is more efficient than computing each separately.

Note that `centered-quotient` returns _x/y_ rounded to the nearest integer. When _x/y_ lies exactly half-way between two integers, the tie is broken according to the sign of y. If _y > 0_, ties are rounded toward positive infinity, otherwise they are rounded toward negative infinity. This is a consequence of the requirement that _\-|y/2| <= r < |y/2|_.

Note that these operators are equivalent to the R6RS operators `div0`, `mod0`, and `div0-and-mod0`.

([centered-quotient](06_06_02_numerical_data_types.md) 123 10) ⇒ 12
([centered-remainder](06_06_02_numerical_data_types.md) 123 10) ⇒ 3
([centered/](06_06_02_numerical_data_types.md) 123 10) ⇒ 12 and 3
([centered/](06_06_02_numerical_data_types.md) 123 \-10) ⇒ \-12 and 3
([centered/](06_06_02_numerical_data_types.md) \-123 10) ⇒ \-12 and \-3
([centered/](06_06_02_numerical_data_types.md) \-123 \-10) ⇒ 12 and \-3
([centered/](06_06_02_numerical_data_types.md) 125 10) ⇒ 13 and \-5
([centered/](06_06_02_numerical_data_types.md) 127 10) ⇒ 13 and \-3
([centered/](06_06_02_numerical_data_types.md) 135 10) ⇒ 14 and \-5
([centered/](06_06_02_numerical_data_types.md) \-123.2 \-63.5) ⇒ 2.0 and 3.8
([centered/](06_06_02_numerical_data_types.md) 16/3 \-10/7) ⇒ \-4 and \-8/21

Scheme Procedure: **round/** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **round-quotient** `x y` [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **round-remainder** `x y` [¶](06_06_02_numerical_data_types.md)

C Function: `void` **scm\_round\_divide** `(SCM x, SCM y, SCM *q, SCM *r)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_round\_quotient** `(x, y)` [¶](06_06_02_numerical_data_types.md)

C Function: `SCM` **scm\_round\_remainder** `(x, y)` [¶](06_06_02_numerical_data_types.md)

These procedures accept two real numbers x and y, where the divisor y must be non-zero. `round-quotient` returns the integer q and `round-remainder` returns the real number r such that _x = q\*y + r_ and q is _x/y_ rounded to the nearest integer, with ties going to the nearest even integer. `round/` returns both q and r, and is more efficient than computing each separately.

Note that `round/` and `centered/` are almost equivalent, but their behavior differs when _x/y_ lies exactly half-way between two integers. In this case, `round/` chooses the nearest even integer, whereas `centered/` chooses in such a way to satisfy the constraint _\-|y/2| <= r < |y/2|_, which is stronger than the corresponding constraint for `round/`, _\-|y/2| <= r <= |y/2|_. In particular, when x and y are integers, the number of possible remainders returned by `centered/` is _|y|_, whereas the number of possible remainders returned by `round/` is _|y|+1_ when y is even.

([round-quotient](06_06_02_numerical_data_types.md) 123 10) ⇒ 12
([round-remainder](06_06_02_numerical_data_types.md) 123 10) ⇒ 3
([round/](06_06_02_numerical_data_types.md) 123 10) ⇒ 12 and 3
([round/](06_06_02_numerical_data_types.md) 123 \-10) ⇒ \-12 and 3
([round/](06_06_02_numerical_data_types.md) \-123 10) ⇒ \-12 and \-3
([round/](06_06_02_numerical_data_types.md) \-123 \-10) ⇒ 12 and \-3
([round/](06_06_02_numerical_data_types.md) 125 10) ⇒ 12 and 5
([round/](06_06_02_numerical_data_types.md) 127 10) ⇒ 13 and \-3
([round/](06_06_02_numerical_data_types.md) 135 10) ⇒ 14 and \-5
([round/](06_06_02_numerical_data_types.md) \-123.2 \-63.5) ⇒ 2.0 and 3.8
([round/](06_06_02_numerical_data_types.md) 16/3 \-10/7) ⇒ \-4 and \-8/21

* * *

Next: [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations), Previous: [Arithmetic Functions](06_06_02_numerical_data_types.md#66211-arithmetic-functions), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.12 Scientific Functions [¶](06_06_02_numerical_data_types.md#66212-scientific-functions)

The following procedures accept any kind of number as arguments, including complex numbers.

Scheme Procedure: **sqrt** z [¶](06_06_02_numerical_data_types.md)

Return the square root of z. Of the two possible roots (positive and negative), the one with a positive real part is returned, or if that’s zero then a positive imaginary part. Thus,

(sqrt 9.0)       ⇒ 3.0
(sqrt -9.0)      ⇒ 0.0+3.0i
(sqrt 1.0+1.0i)  ⇒ 1.09868411346781+0.455089860562227i
(sqrt -1.0-1.0i) ⇒ 0.455089860562227-1.09868411346781i

Scheme Procedure: **expt** z1 z2 [¶](06_06_02_numerical_data_types.md)

Return z1 raised to the power of z2.

Scheme Procedure: **sin** z [¶](06_06_02_numerical_data_types.md)

Return the sine of z.

Scheme Procedure: **cos** z [¶](06_06_02_numerical_data_types.md)

Return the cosine of z.

Scheme Procedure: **tan** z [¶](06_06_02_numerical_data_types.md)

Return the tangent of z.

Scheme Procedure: **asin** z [¶](06_06_02_numerical_data_types.md)

Return the arcsine of z.

Scheme Procedure: **acos** z [¶](06_06_02_numerical_data_types.md)

Return the arccosine of z.

Scheme Procedure: **atan** z [¶](06_06_02_numerical_data_types.md)

Scheme Procedure: **atan** y x [¶](06_06_02_numerical_data_types.md)

Return the arctangent of z, or of _y/x_.

Scheme Procedure: **exp** z [¶](06_06_02_numerical_data_types.md)

Return e to the power of z, where e is the base of natural logarithms (2.71828…).

Scheme Procedure: **log** z [¶](06_06_02_numerical_data_types.md)

Return the natural logarithm of z.

Scheme Procedure: **log10** z [¶](06_06_02_numerical_data_types.md)

Return the base 10 logarithm of z.

Scheme Procedure: **sinh** z [¶](06_06_02_numerical_data_types.md)

Return the hyperbolic sine of z.

Scheme Procedure: **cosh** z [¶](06_06_02_numerical_data_types.md)

Return the hyperbolic cosine of z.

Scheme Procedure: **tanh** z [¶](06_06_02_numerical_data_types.md)

Return the hyperbolic tangent of z.

Scheme Procedure: **asinh** z [¶](06_06_02_numerical_data_types.md)

Return the hyperbolic arcsine of z.

Scheme Procedure: **acosh** z [¶](06_06_02_numerical_data_types.md)

Return the hyperbolic arccosine of z.

Scheme Procedure: **atanh** z [¶](06_06_02_numerical_data_types.md)

Return the hyperbolic arctangent of z.

* * *

Next: [Random Number Generation](06_06_02_numerical_data_types.md#66214-random-number-generation), Previous: [Scientific Functions](06_06_02_numerical_data_types.md#66212-scientific-functions), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.13 Bitwise Operations [¶](06_06_02_numerical_data_types.md#66213-bitwise-operations)

For the following bitwise functions, negative numbers are treated as infinite precision twos-complements. For instance _\-6_ is bits _...111010_, with infinitely many ones on the left. It can be seen that adding 6 (binary 110) to such a bit pattern gives all zeros.

Scheme Procedure: **logand** n1 n2 … [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_logand** (n1, n2) [¶](06_06_02_numerical_data_types.md)

Return the bitwise AND of the integer arguments.

([logand](06_06_02_numerical_data_types.md)) ⇒ \-1
([logand](06_06_02_numerical_data_types.md) 7) ⇒ 7
([logand](06_06_02_numerical_data_types.md) #b111 #b011 #b001) ⇒ 1

Scheme Procedure: **logior** n1 n2 … [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_logior** (n1, n2) [¶](06_06_02_numerical_data_types.md)

Return the bitwise OR of the integer arguments.

([logior](06_06_02_numerical_data_types.md)) ⇒ 0
([logior](06_06_02_numerical_data_types.md) 7) ⇒ 7
([logior](06_06_02_numerical_data_types.md) #b000 #b001 #b011) ⇒ 3

Scheme Procedure: **logxor** n1 n2 … [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_loxor** (n1, n2) [¶](06_06_02_numerical_data_types.md)

Return the bitwise XOR of the integer arguments. A bit is set in the result if it is set in an odd number of arguments.

([logxor](06_06_02_numerical_data_types.md)) ⇒ 0
([logxor](06_06_02_numerical_data_types.md) 7) ⇒ 7
([logxor](06_06_02_numerical_data_types.md) #b000 #b001 #b011) ⇒ 2
([logxor](06_06_02_numerical_data_types.md) #b000 #b001 #b011 #b011) ⇒ 1

Scheme Procedure: **lognot** n [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_lognot** (n) [¶](06_06_02_numerical_data_types.md)

Return the integer which is the ones-complement of the integer argument, ie. each 0 bit is changed to 1 and each 1 bit to 0.

([number->string](06_06_02_numerical_data_types.md) ([lognot](06_06_02_numerical_data_types.md) #b10000000) 2)
   ⇒ "-10000001"
([number->string](06_06_02_numerical_data_types.md) ([lognot](06_06_02_numerical_data_types.md) #b0) 2)
   ⇒ "-1"

Scheme Procedure: **logtest** j k [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_logtest** (j, k) [¶](06_06_02_numerical_data_types.md)

Test whether j and k have any 1 bits in common. This is equivalent to `(not (zero? (logand j k)))`, but without actually calculating the `logand`, just testing for non-zero.

([logtest](06_06_02_numerical_data_types.md) #b0100 #b1011) ⇒ #f
([logtest](06_06_02_numerical_data_types.md) #b0100 #b0111) ⇒ #t

Scheme Procedure: **logbit?** index j [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_logbit\_p** (index, j) [¶](06_06_02_numerical_data_types.md)

Test whether bit number index in j is set. index starts from 0 for the least significant bit.

([logbit?](06_06_02_numerical_data_types.md) 0 #b1101) ⇒ #t
([logbit?](06_06_02_numerical_data_types.md) 1 #b1101) ⇒ #f
([logbit?](06_06_02_numerical_data_types.md) 2 #b1101) ⇒ #t
([logbit?](06_06_02_numerical_data_types.md) 3 #b1101) ⇒ #t
([logbit?](06_06_02_numerical_data_types.md) 4 #b1101) ⇒ #f

Scheme Procedure: **ash** n count [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_ash** (n, count) [¶](06_06_02_numerical_data_types.md)

Return _floor(n \* 2^{count})_. n and count must be exact integers.

With n viewed as an infinite-precision twos-complement integer, `ash` means a left shift introducing zero bits when count is positive, or a right shift dropping bits when count is negative. This is an “arithmetic” shift.

([number->string](06_06_02_numerical_data_types.md) ([ash](06_06_02_numerical_data_types.md) #b1 3) 2)     ⇒ "1000"
([number->string](06_06_02_numerical_data_types.md) ([ash](06_06_02_numerical_data_types.md) #b1010 \-1) 2) ⇒ "101"

;; -23 is bits ...11101001, -6 is bits ...111010
([ash](06_06_02_numerical_data_types.md) \-23 \-2) ⇒ \-6

Scheme Procedure: **round-ash** n count [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_round\_ash** (n, count) [¶](06_06_02_numerical_data_types.md)

Return _round(n \* 2^count)_. n and count must be exact integers.

With n viewed as an infinite-precision twos-complement integer, `round-ash` means a left shift introducing zero bits when count is positive, or a right shift rounding to the nearest integer (with ties going to the nearest even integer) when count is negative. This is a rounded “arithmetic” shift.

([number->string](06_06_02_numerical_data_types.md) ([round-ash](06_06_02_numerical_data_types.md) #b1 3) 2)     ⇒ \\"1000\\"
([number->string](06_06_02_numerical_data_types.md) ([round-ash](06_06_02_numerical_data_types.md) #b1010 \-1) 2) ⇒ \\"101\\"
([number->string](06_06_02_numerical_data_types.md) ([round-ash](06_06_02_numerical_data_types.md) #b1010 \-2) 2) ⇒ \\"10\\"
([number->string](06_06_02_numerical_data_types.md) ([round-ash](06_06_02_numerical_data_types.md) #b1011 \-2) 2) ⇒ \\"11\\"
([number->string](06_06_02_numerical_data_types.md) ([round-ash](06_06_02_numerical_data_types.md) #b1101 \-2) 2) ⇒ \\"11\\"
([number->string](06_06_02_numerical_data_types.md) ([round-ash](06_06_02_numerical_data_types.md) #b1110 \-2) 2) ⇒ \\"100\\"

Scheme Procedure: **logcount** n [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_logcount** (n) [¶](06_06_02_numerical_data_types.md)

Return the number of bits in integer n. If n is positive, the 1-bits in its binary representation are counted. If negative, the 0-bits in its two’s-complement binary representation are counted. If zero, 0 is returned.

([logcount](06_06_02_numerical_data_types.md) #b10101010)
   ⇒ 4
([logcount](06_06_02_numerical_data_types.md) 0)
   ⇒ 0
([logcount](06_06_02_numerical_data_types.md) \-2)
   ⇒ 1

Scheme Procedure: **integer-length** n [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_integer\_length** (n) [¶](06_06_02_numerical_data_types.md)

Return the number of bits necessary to represent n.

For positive n this is how many bits to the most significant one bit. For negative n it’s how many bits to the most significant zero bit in twos complement form.

([integer-length](06_06_02_numerical_data_types.md) #b10101010) ⇒ 8
([integer-length](06_06_02_numerical_data_types.md) #b1111)     ⇒ 4
([integer-length](06_06_02_numerical_data_types.md) 0)          ⇒ 0
([integer-length](06_06_02_numerical_data_types.md) \-1)         ⇒ 0
([integer-length](06_06_02_numerical_data_types.md) \-256)       ⇒ 8
([integer-length](06_06_02_numerical_data_types.md) \-257)       ⇒ 9

Scheme Procedure: **integer-expt** n k [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_integer\_expt** (n, k) [¶](06_06_02_numerical_data_types.md)

Return n raised to the power k. k must be an exact integer, n can be any number.

Negative k is supported, and results in _1/n^abs(k)_ in the usual way. _n^0_ is 1, as usual, and that includes _0^0_ is 1.

([integer-expt](06_06_02_numerical_data_types.md) 2 5)   ⇒ 32
([integer-expt](06_06_02_numerical_data_types.md) \-3 3)  ⇒ \-27
([integer-expt](06_06_02_numerical_data_types.md) 5 \-3)  ⇒ 1/125
([integer-expt](06_06_02_numerical_data_types.md) 0 0)   ⇒ 1

Scheme Procedure: **bit-extract** n start end [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_bit\_extract** (n, start, end) [¶](06_06_02_numerical_data_types.md)

Return the integer composed of the start (inclusive) through end (exclusive) bits of n. The startth bit becomes the 0-th bit in the result.

([number->string](06_06_02_numerical_data_types.md) ([bit-extract](06_06_02_numerical_data_types.md) #b1101101010 0 4) 2)
   ⇒ "1010"
([number->string](06_06_02_numerical_data_types.md) ([bit-extract](06_06_02_numerical_data_types.md) #b1101101010 4 9) 2)
   ⇒ "10110"

* * *

Previous: [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations), Up: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.2.14 Random Number Generation [¶](06_06_02_numerical_data_types.md#66214-random-number-generation)

Pseudo-random numbers are generated from a random state object, which can be created with `seed->random-state` or `datum->random-state`. An external representation (i.e. one which can written with `write` and read with `read`) of a random state object can be obtained via `random-state->datum`. The state parameter to the various functions below is optional, it defaults to the state object in the `*random-state*` variable.

Scheme Procedure: **copy-random-state** \[state\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_copy\_random\_state** (state) [¶](06_06_02_numerical_data_types.md)

Return a copy of the random state state.

Scheme Procedure: **random** n \[state\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_random** (n, state) [¶](06_06_02_numerical_data_types.md)

Return a number in \[0, n).

Accepts a positive integer or real n and returns a number of the same type between zero (inclusive) and n (exclusive). The values returned have a uniform distribution.

Scheme Procedure: **random:exp** \[state\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_random\_exp** (state) [¶](06_06_02_numerical_data_types.md)

Return an inexact real in an exponential distribution with mean 1. For an exponential distribution with mean u use `(* u (random:exp))`.

Scheme Procedure: **random:hollow-sphere!** vect \[state\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_random\_hollow\_sphere\_x** (vect, state) [¶](06_06_02_numerical_data_types.md)

Fills vect with inexact real random numbers the sum of whose squares is equal to 1.0. Thinking of vect as coordinates in space of dimension n _\=_ `(vector-length vect)`, the coordinates are uniformly distributed over the surface of the unit n-sphere.

Scheme Procedure: **random:normal** \[state\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_random\_normal** (state) [¶](06_06_02_numerical_data_types.md)

Return an inexact real in a normal distribution. The distribution used has mean 0 and standard deviation 1. For a normal distribution with mean m and standard deviation d use `(+ m (* d (random:normal)))`.

Scheme Procedure: **random:normal-vector!** vect \[state\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_random\_normal\_vector\_x** (vect, state) [¶](06_06_02_numerical_data_types.md)

Fills vect with inexact real random numbers that are independent and standard normally distributed (i.e., with mean 0 and variance 1).

Scheme Procedure: **random:solid-sphere!** vect \[state\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_random\_solid\_sphere\_x** (vect, state) [¶](06_06_02_numerical_data_types.md)

Fills vect with inexact real random numbers the sum of whose squares is less than 1.0. Thinking of vect as coordinates in space of dimension n _\=_ `(vector-length vect)`, the coordinates are uniformly distributed within the unit n\-sphere.

Scheme Procedure: **random:uniform** \[state\] [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_random\_uniform** (state) [¶](06_06_02_numerical_data_types.md)

Return a uniformly distributed inexact real random number in \[0,1).

Scheme Procedure: **seed->random-state** seed [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_seed\_to\_random\_state** (seed) [¶](06_06_02_numerical_data_types.md)

Return a new random state using seed.

Scheme Procedure: **datum->random-state** datum [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_datum\_to\_random\_state** (datum) [¶](06_06_02_numerical_data_types.md)

Return a new random state from datum, which should have been obtained by `random-state->datum`.

Scheme Procedure: **random-state->datum** state [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_random\_state\_to\_datum** (state) [¶](06_06_02_numerical_data_types.md)

Return a datum representation of state that may be written out and read back with the Scheme reader.

Scheme Procedure: **random-state-from-platform** [¶](06_06_02_numerical_data_types.md)

C Function: **scm\_random\_state\_from\_platform** () [¶](06_06_02_numerical_data_types.md)

Construct a new random state seeded from a platform-specific source of entropy, appropriate for use in non-security-critical applications. Currently /dev/urandom is tried first, or else the seed is based on the time, date, process ID, an address from a freshly allocated heap cell, an address from the local stack frame, and a high-resolution timer if available.

Variable: **\*random-state\*** [¶](06_06_02_numerical_data_types.md)

The global random state used by the above functions when the state parameter is not given.

Note that the initial value of `*random-state*` is the same every time Guile starts up. Therefore, if you don’t pass a state parameter to the above procedures, and you don’t set `*random-state*` to `(seed->random-state your-seed)`, where `your-seed` is something that _isn’t_ the same every time, you’ll get the same sequence of “random” numbers on every run.

For example, unless the relevant source code has changed, `(map random (cdr (iota 30)))`, if the first use of random numbers since Guile started up, will always give:

(map [random](06_06_02_numerical_data_types.md) ([cdr](06_06_08_pairs.md) ([iota](07_05_03_srfi1_list_library.md) 19)))
⇒
(0 1 1 2 2 2 1 2 6 7 10 0 5 3 12 5 5 12)

To seed the random state in a sensible way for non-security-critical applications, do this during initialization of your program:

([set!](07_06_r6rs_support.md) [\*random-state\*](06_06_02_numerical_data_types.md) ([random-state-from-platform](06_06_02_numerical_data_types.md)))

* * *

Next: [Character Sets](06_06_04_character_sets.md#664-character-sets), Previous: [Numerical data types](06_06_02_numerical_data_types.md#662-numerical-data-types), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
