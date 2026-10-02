#### 6.6.3 Characters [¶](06_06_03_characters.md#663-characters)

In Scheme, there is a data type to describe a single character.

Defining what exactly a character _is_ can be more complicated than it seems. Guile follows the advice of R6RS and uses The Unicode Standard to help define what a character is. So, for Guile, a character is anything in the Unicode Character Database.

The Unicode Character Database is basically a table of characters indexed using integers called ’code points’. Valid code points are in the ranges 0 to `#xD7FF` inclusive or `#xE000` to `#x10FFFF` inclusive, which is about 1.1 million code points.

Any code point that has been assigned to a character or that has otherwise been given a meaning by Unicode is called a ’designated code point’. Most of the designated code points, about 200,000 of them, indicate characters, accents or other combining marks that modify other characters, symbols, whitespace, and control characters. Some are not characters but indicators that suggest how to format or display neighboring characters.

If a code point is not a designated code point – if it has not been assigned to a character by The Unicode Standard – it is a ’reserved code point’, meaning that they are reserved for future use. Most of the code points, about 800,000, are ’reserved code points’.

By convention, a Unicode code point is written as “U+XXXX” where “XXXX” is a hexadecimal number. Please note that this convenient notation is not valid code. Guile does not interpret “U+XXXX” as a character.

In Scheme, a character literal is written as `#\name` where name is the name of the character that you want. Printable characters have their usual single character name; for example, `#\a` is a lower case `a`.

Some of the code points are ’combining characters’ that are not meant to be printed by themselves but are instead meant to modify the appearance of the previous character. For combining characters, an alternate form of the character literal is `#\` followed by U+25CC (a small, dotted circle), followed by the combining character. This allows the combining character to be drawn on the circle, not on the backslash of `#\`.

Many of the non-printing characters, such as whitespace characters and control characters, also have names.

The most commonly used non-printing characters have long character names, described in the table below.

Character Name

Codepoint

`#\nul`

U+0000

`#\alarm`

U+0007

`#\backspace`

U+0008

`#\tab`

U+0009

`#\linefeed`

U+000A

`#\newline`

U+000A

`#\vtab`

U+000B

`#\page`

U+000C

`#\return`

U+000D

`#\esc`

U+001B

`#\space`

U+0020

`#\delete`

U+007F

There are also short names for all of the “C0 control characters” (those with code points below 32). The following table lists the short name for each character.

0 = `#\nul`

1 = `#\soh`

2 = `#\stx`

3 = `#\etx`

4 = `#\eot`

5 = `#\enq`

6 = `#\ack`

7 = `#\bel`

8 = `#\bs`

9 = `#\ht`

10 = `#\lf`

11 = `#\vt`

12 = `#\ff`

13 = `#\cr`

14 = `#\so`

15 = `#\si`

16 = `#\dle`

17 = `#\dc1`

18 = `#\dc2`

19 = `#\dc3`

20 = `#\dc4`

21 = `#\nak`

22 = `#\syn`

23 = `#\etb`

24 = `#\can`

25 = `#\em`

26 = `#\sub`

27 = `#\esc`

28 = `#\fs`

29 = `#\gs`

30 = `#\rs`

31 = `#\us`

32 = `#\sp`

The short name for the “delete” character (code point U+007F) is `#\del`.

The R7RS name for the “escape” character (code point U+001B) is `#\escape`.

There are also a few alternative names left over for compatibility with previous versions of Guile.

Alternate

Standard

`#\nl`

`#\newline`

`#\np`

`#\page`

`#\null`

`#\nul`

Characters may also be written using their code point values. They can be written with as an octal number, such as `#\10` for `#\bs` or `#\177` for `#\del`.

If one prefers hex to octal, there is an additional syntax for character escapes: `#\xHHHH` – the letter ’x’ followed by a hexadecimal number of one to eight digits.

Scheme Procedure: **char?** x [¶](06_06_03_characters.md)

C Function: **scm\_char\_p** (x) [¶](06_06_03_characters.md)

Return `#t` if x is a character, else `#f`.

Fundamentally, the character comparison operations below are numeric comparisons of the character’s code points.

Scheme Procedure: **char=?** x y [¶](06_06_03_characters.md)

Return `#t` if code point of x is equal to the code point of y, else `#f`.

Scheme Procedure: **char<?** x y [¶](06_06_03_characters.md)

Return `#t` if the code point of x is less than the code point of y, else `#f`.

Scheme Procedure: **char<=?** x y [¶](06_06_03_characters.md)

Return `#t` if the code point of x is less than or equal to the code point of y, else `#f`.

Scheme Procedure: **char>?** x y [¶](06_06_03_characters.md)

Return `#t` if the code point of x is greater than the code point of y, else `#f`.

Scheme Procedure: **char>=?** x y [¶](06_06_03_characters.md)

Return `#t` if the code point of x is greater than or equal to the code point of y, else `#f`.

Case-insensitive character comparisons use _Unicode case folding_. In case folding comparisons, if a character is lowercase and has an uppercase form that can be expressed as a single character, it is converted to uppercase before comparison. All other characters undergo no conversion before the comparison occurs. This includes the German sharp S (Eszett) which is not uppercased before conversion because its uppercase form has two characters. Unicode case folding is language independent: it uses rules that are generally true, but, it cannot cover all cases for all languages.

Scheme Procedure: **char-ci=?** x y [¶](06_06_03_characters.md)

Return `#t` if the case-folded code point of x is the same as the case-folded code point of y, else `#f`.

Scheme Procedure: **char-ci<?** x y [¶](06_06_03_characters.md)

Return `#t` if the case-folded code point of x is less than the case-folded code point of y, else `#f`.

Scheme Procedure: **char-ci<=?** x y [¶](06_06_03_characters.md)

Return `#t` if the case-folded code point of x is less than or equal to the case-folded code point of y, else `#f`.

Scheme Procedure: **char-ci>?** x y [¶](06_06_03_characters.md)

Return `#t` if the case-folded code point of x is greater than the case-folded code point of y, else `#f`.

Scheme Procedure: **char-ci>=?** x y [¶](06_06_03_characters.md)

Return `#t` if the case-folded code point of x is greater than or equal to the case-folded code point of y, else `#f`.

Scheme Procedure: **char-alphabetic?** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_alphabetic\_p** (chr) [¶](06_06_03_characters.md)

Return `#t` if chr is alphabetic, else `#f`.

Scheme Procedure: **char-numeric?** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_numeric\_p** (chr) [¶](06_06_03_characters.md)

Return `#t` if chr is numeric, else `#f`.

Scheme Procedure: **char-whitespace?** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_whitespace\_p** (chr) [¶](06_06_03_characters.md)

Return `#t` if chr is whitespace, else `#f`.

Scheme Procedure: **char-upper-case?** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_upper\_case\_p** (chr) [¶](06_06_03_characters.md)

Return `#t` if chr is uppercase, else `#f`.

Scheme Procedure: **char-lower-case?** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_lower\_case\_p** (chr) [¶](06_06_03_characters.md)

Return `#t` if chr is lowercase, else `#f`.

Scheme Procedure: **char-is-both?** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_is\_both\_p** (chr) [¶](06_06_03_characters.md)

Return `#t` if chr is either uppercase or lowercase, else `#f`.

Scheme Procedure: **char-general-category** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_general\_category** (chr) [¶](06_06_03_characters.md)

Return a symbol giving the two-letter name of the Unicode general category assigned to chr or `#f` if no named category is assigned. The following table provides a list of category names along with their meanings.

Lu

Uppercase letter

Pf

Final quote punctuation

Ll

Lowercase letter

Po

Other punctuation

Lt

Titlecase letter

Sm

Math symbol

Lm

Modifier letter

Sc

Currency symbol

Lo

Other letter

Sk

Modifier symbol

Mn

Non-spacing mark

So

Other symbol

Mc

Combining spacing mark

Zs

Space separator

Me

Enclosing mark

Zl

Line separator

Nd

Decimal digit number

Zp

Paragraph separator

Nl

Letter number

Cc

Control

No

Other number

Cf

Format

Pc

Connector punctuation

Cs

Surrogate

Pd

Dash punctuation

Co

Private use

Ps

Open punctuation

Cn

Unassigned

Pe

Close punctuation

Pi

Initial quote punctuation

Scheme Procedure: **char->integer** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_to\_integer** (chr) [¶](06_06_03_characters.md)

Return the code point of chr.

Scheme Procedure: **integer->char** n [¶](06_06_03_characters.md)

C Function: **scm\_integer\_to\_char** (n) [¶](06_06_03_characters.md)

Return the character that has code point n. The integer n must be a valid code point. Valid code points are in the ranges 0 to `#xD7FF` inclusive or `#xE000` to `#x10FFFF` inclusive.

Scheme Procedure: **char-upcase** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_upcase** (chr) [¶](06_06_03_characters.md)

Return the uppercase character version of chr.

Scheme Procedure: **char-downcase** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_downcase** (chr) [¶](06_06_03_characters.md)

Return the lowercase character version of chr.

Scheme Procedure: **char-titlecase** chr [¶](06_06_03_characters.md)

C Function: **scm\_char\_titlecase** (chr) [¶](06_06_03_characters.md)

Return the titlecase character version of chr if one exists; otherwise return the uppercase version.

For most characters these will be the same, but the Unicode Standard includes certain digraph compatibility characters, such as `U+01F3` “dz”, for which the uppercase and titlecase characters are different (`U+01F1` “DZ” and `U+01F2` “Dz” in this case, respectively).

C Function: `scm_t_wchar` **scm\_c\_upcase** `(scm_t_wchar c)` [¶](06_06_03_characters.md)

C Function: `scm_t_wchar` **scm\_c\_downcase** `(scm_t_wchar c)` [¶](06_06_03_characters.md)

C Function: `scm_t_wchar` **scm\_c\_titlecase** `(scm_t_wchar c)` [¶](06_06_03_characters.md)

These C functions take an integer representation of a Unicode codepoint and return the codepoint corresponding to its uppercase, lowercase, and titlecase forms respectively. The type `scm_t_wchar` is a signed, 32-bit integer.

Characters also have “formal names”, which are defined by Unicode. These names can be accessed in Guile from the `(ice-9 unicode)` module:

(use-modules (ice-9 unicode))

Scheme Procedure: **char->formal-name** chr [¶](06_06_03_characters.md)

Return the formal all-upper-case Unicode name of ch, as a string, or `#f` if the character has no name.

Scheme Procedure: **formal-name->char** name [¶](06_06_03_characters.md)

Return the character whose formal all-upper-case Unicode name is name, or `#f` if no such character is known.

* * *

Next: [Strings](06_06_05_strings.md#665-strings), Previous: [Characters](06_06_03_characters.md#663-characters), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
