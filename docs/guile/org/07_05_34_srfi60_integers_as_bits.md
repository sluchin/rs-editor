#### 7.5.34 SRFI-60 - Integers as Bits [¶](07_05_34_srfi60_integers_as_bits.md#7534-srfi-60---integers-as-bits)

This SRFI provides various functions for treating integers as bits and for bitwise manipulations. These functions can be obtained with,

(use-modules (srfi srfi-60))

Integers are treated as infinite precision twos-complement, the same as in the core logical functions (see [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations)). And likewise bit indexes start from 0 for the least significant bit. The following functions in this SRFI are already in the Guile core,

> `logand`, `logior`, `logxor`, `lognot`, `logtest`, `logcount`, `integer-length`, `logbit?`, `ash`

  

Function: **bitwise-and** n1 ... [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **bitwise-ior** n1 ... [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **bitwise-xor** n1 ... [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **bitwise-not** n [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **any-bits-set?** j k [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **bit-set?** index n [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **arithmetic-shift** n count [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **bit-field** n start end [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **bit-count** n [¶](07_05_34_srfi60_integers_as_bits.md)

Aliases for `logand`, `logior`, `logxor`, `lognot`, `logtest`, `logbit?`, `ash`, `bit-extract` and `logcount` respectively.

Note that the name `bit-count` conflicts with `bit-count` in the core (see [Bit Vectors](06_06_11_bit_vectors.md#6611-bit-vectors)).

Function: **bitwise-if** mask n1 n0 [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **bitwise-merge** mask n1 n0 [¶](07_05_34_srfi60_integers_as_bits.md)

Return an integer with bits selected from n1 and n0 according to mask. Those bits where mask has 1s are taken from n1, and those where mask has 0s are taken from n0.

(bitwise-if 3 #b0101 #b1010) ⇒ 9

Function: **log2-binary-factors** n [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **first-set-bit** n [¶](07_05_34_srfi60_integers_as_bits.md)

Return a count of how many factors of 2 are present in n. This is also the bit index of the lowest 1 bit in n. If n is 0, the return is _\-1_.

(log2-binary-factors 6) ⇒ 1
(log2-binary-factors -8) ⇒ 3

Function: **copy-bit** index n newbit [¶](07_05_34_srfi60_integers_as_bits.md)

Return n with the bit at index set according to newbit. newbit should be `#t` to set the bit to 1, or `#f` to set it to 0. Bits other than at index are unchanged in the return.

(copy-bit 1 #b0101 #t) ⇒ 7

Function: **copy-bit-field** n newbits start end [¶](07_05_34_srfi60_integers_as_bits.md)

Return n with the bits from start (inclusive) to end (exclusive) changed to the value newbits.

The least significant bit in newbits goes to start, the next to _start+1_, etc. Anything in newbits past the end given is ignored.

(copy-bit-field #b10000 #b11 1 3) ⇒ #b10110

Function: **rotate-bit-field** n count start end [¶](07_05_34_srfi60_integers_as_bits.md)

Return n with the bit field from start (inclusive) to end (exclusive) rotated upwards by count bits.

count can be positive or negative, and it can be more than the field width (it’ll be reduced modulo the width).

(rotate-bit-field #b0110 2 1 4) ⇒ #b1010

Function: **reverse-bit-field** n start end [¶](07_05_34_srfi60_integers_as_bits.md)

Return n with the bits from start (inclusive) to end (exclusive) reversed.

(reverse-bit-field #b101001 2 4) ⇒ #b100101

Function: **integer->list** n \[len\] [¶](07_05_34_srfi60_integers_as_bits.md)

Return bits from n in the form of a list of `#t` for 1 and `#f` for 0. The least significant len bits are returned, and the first list element is the most significant of those bits. If len is not given, the default is `(integer-length n)` (see [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations)).

(integer->list 6)   ⇒ (#t #t #f)
(integer->list 1 4) ⇒ (#f #f #f #t)

Function: **list->integer** lst [¶](07_05_34_srfi60_integers_as_bits.md)

Function: **booleans->integer** bool… [¶](07_05_34_srfi60_integers_as_bits.md)

Return an integer formed bitwise from the given lst list of booleans, or for `booleans->integer` from the bool arguments.

Each boolean is `#t` for a 1 and `#f` for a 0. The first element becomes the most significant bit in the return.

(list->integer '(#t #f #t #f)) ⇒ 10

* * *

Next: [SRFI-62 - S-expression comments.](07_05_36_srfi62_sexpression_comments.md#7536-srfi-62---s-expression-comments), Previous: [SRFI-60 - Integers as Bits](07_05_34_srfi60_integers_as_bits.md#7534-srfi-60---integers-as-bits), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
