#### 6.6.5 Strings [¶](06_06_05_strings.md#665-strings)

Strings are fixed-length sequences of characters. They can be created by calling constructor procedures, but they can also literally get entered at the REPL or in Scheme source files.

Strings always carry the information about how many characters they are composed of with them, so there is no special end-of-string character, like in C. That means that Scheme strings can contain any character, even the ‘#\\nul’ character ‘\\0’.

To use strings efficiently, you need to know a bit about how Guile implements them. In Guile, a string consists of two parts, a head and the actual memory where the characters are stored. When a string (or a substring of it) is copied, only a new head gets created, the memory is usually not copied. The two heads start out pointing to the same memory.

When one of these two strings is modified, as with `string-set!`, their common memory does get copied so that each string has its own memory and modifying one does not accidentally modify the other as well. Thus, Guile’s strings are ‘copy on write’; the actual copying of their memory is delayed until one string is written to.

This implementation makes functions like `substring` very efficient in the common case that no modifications are done to the involved strings.

If you do know that your strings are getting modified right away, you can use `substring/copy` instead of `substring`. This function performs the copy immediately at the time of creation. This is more efficient, especially in a multi-threaded program. Also, `substring/copy` can avoid the problem that a short substring holds on to the memory of a very large original string that could otherwise be recycled.

If you want to avoid the copy altogether, so that modifications of one string show up in the other, you can use `substring/shared`. The strings created by this procedure are called _mutation sharing substrings_ since the substring and the original string share modifications to each other.

If you want to prevent modifications, use `substring/read-only`.

Guile provides all procedures of SRFI-13 and a few more.

*   [String Read Syntax](06_06_05_strings.md#6651-string-read-syntax)
*   [String Predicates](06_06_05_strings.md#6652-string-predicates)
*   [String Constructors](06_06_05_strings.md#6653-string-constructors)
*   [List/String conversion](06_06_05_strings.md#6654-liststring-conversion)
*   [String Selection](06_06_05_strings.md#6655-string-selection)
*   [String Modification](06_06_05_strings.md#6656-string-modification)
*   [String Comparison](06_06_05_strings.md#6657-string-comparison)
*   [String Searching](06_06_05_strings.md#6658-string-searching)
*   [Alphabetic Case Mapping](06_06_05_strings.md#6659-alphabetic-case-mapping)
*   [Reversing and Appending Strings](06_06_05_strings.md#66510-reversing-and-appending-strings)
*   [Mapping, Folding, and Unfolding](06_06_05_strings.md#66511-mapping-folding-and-unfolding)
*   [Miscellaneous String Operations](06_06_05_strings.md#66512-miscellaneous-string-operations)
*   [Representing Strings as Bytes](06_06_05_strings.md#66513-representing-strings-as-bytes)
*   [Conversion to/from C](06_06_05_strings.md#66514-conversion-tofrom-c)
*   [String Internals](06_06_05_strings.md#66515-string-internals)

* * *

Next: [String Predicates](06_06_05_strings.md#6652-string-predicates), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.1 String Read Syntax [¶](06_06_05_strings.md#6651-string-read-syntax)

The read syntax for strings is an arbitrarily long sequence of characters enclosed in double quotes (`"`).

Backslash is an escape character and can be used to insert the following special characters. `\"` and `\\` are R5RS standard, `\|` is R7RS standard, the next seven are R6RS standard — notice they follow C syntax — and the remaining four are Guile extensions.

`\\`

Backslash character.

`\"`

Double quote character (an unescaped `"` is otherwise the end of the string).

`\|`

Vertical bar character.

`\a`

Bell character (ASCII 7).

`\f`

Formfeed character (ASCII 12).

`\n`

Newline character (ASCII 10).

`\r`

Carriage return character (ASCII 13).

`\t`

Tab character (ASCII 9).

`\v`

Vertical tab character (ASCII 11).

`\b`

Backspace character (ASCII 8).

`\0`

NUL character (ASCII 0).

`\(`

Open parenthesis. This is intended for use at the beginning of lines in multiline strings to avoid confusing Emacs lisp modes.

`\` followed by newline (ASCII 10)

Nothing. This way if `\` is the last character in a line, the string will continue with the first character from the next line, without a line break.

If the `hungry-eol-escapes` reader option is enabled, which is not the case by default, leading whitespace on the next line is discarded.

"foo\\
  bar"
⇒ "foo  bar"
([read-enable](06_16_reading_and_evaluating_scheme_code.md) 'hungry-eol-escapes)
"foo\\
  bar"
⇒ "foobar"

`\xHH`

Character code given by two hexadecimal digits. For example `\x7f` for an ASCII DEL (127).

`\uHHHH`

Character code given by four hexadecimal digits. For example `\u0100` for a capital A with macron (U+0100).

`\UHHHHHH`

Character code given by six hexadecimal digits. For example `\U010402`.

The following are examples of string literals:

"foo"
"bar plonk"
"Hello World"
"\\"Hi\\", he said."

The three escape sequences `\xHH`, `\uHHHH` and `\UHHHHHH` were chosen to not break compatibility with code written for previous versions of Guile. The R6RS specification suggests a different, incompatible syntax for hex escapes: `\xHHHH;` – a character code followed by one to eight hexadecimal digits terminated with a semicolon. If this escape format is desired instead, it can be enabled with the reader option `r6rs-hex-escapes`.

([read-enable](06_16_reading_and_evaluating_scheme_code.md) 'r6rs-hex-escapes)

For more on reader options, See [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code).

* * *

Next: [String Constructors](06_06_05_strings.md#6653-string-constructors), Previous: [String Read Syntax](06_06_05_strings.md#6651-string-read-syntax), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.2 String Predicates [¶](06_06_05_strings.md#6652-string-predicates)

The following procedures can be used to check whether a given string fulfills some specified property.

Scheme Procedure: **string?** obj [¶](06_06_05_strings.md)

C Function: **scm\_string\_p** (obj) [¶](06_06_05_strings.md)

Return `#t` if obj is a string, else `#f`.

C Function: `int` **scm\_is\_string** `(SCM obj)` [¶](06_06_05_strings.md)

Returns `1` if obj is a string, `0` otherwise.

Scheme Procedure: **string-null?** str [¶](06_06_05_strings.md)

C Function: **scm\_string\_null\_p** (str) [¶](06_06_05_strings.md)

Return `#t` if str’s length is zero, and `#f` otherwise.

([string-null?](06_06_05_strings.md) "")  ⇒ #t
y                    ⇒ "foo"
([string-null?](06_06_05_strings.md) y)     ⇒ #f

Scheme Procedure: **string-any** char\_pred s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_any** (char\_pred, s, start, end) [¶](06_06_05_strings.md)

Check if char\_pred is true for any character in string s.

char\_pred can be a character to check for any equal to that, or a character set (see [Character Sets](06_06_04_character_sets.md#664-character-sets)) to check for any in that set, or a predicate procedure to call.

For a procedure, calls `(char_pred c)` are made successively on the characters from start to end. If char\_pred returns true (ie. non-`#f`), `string-any` stops and that return value is the return from `string-any`. The call on the last character (ie. at _end\-1_), if that point is reached, is a tail call.

If there are no characters in s (ie. start equals end) then the return is `#f`.

Scheme Procedure: **string-every** char\_pred s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_every** (char\_pred, s, start, end) [¶](06_06_05_strings.md)

Check if char\_pred is true for every character in string s.

char\_pred can be a character to check for every character equal to that, or a character set (see [Character Sets](06_06_04_character_sets.md#664-character-sets)) to check for every character being in that set, or a predicate procedure to call.

For a procedure, calls `(char_pred c)` are made successively on the characters from start to end. If char\_pred returns `#f`, `string-every` stops and returns `#f`. The call on the last character (ie. at _end\-1_), if that point is reached, is a tail call and the return from that call is the return from `string-every`.

If there are no characters in s (ie. start equals end) then the return is `#t`.

* * *

Next: [List/String conversion](06_06_05_strings.md#6654-liststring-conversion), Previous: [String Predicates](06_06_05_strings.md#6652-string-predicates), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.3 String Constructors [¶](06_06_05_strings.md#6653-string-constructors)

The string constructor procedures create new string objects, possibly initializing them with some specified character data. See also See [String Selection](06_06_05_strings.md#6655-string-selection), for ways to create strings from existing strings.

Scheme Procedure: **string** char… [¶](06_06_05_strings.md)

Return a newly allocated string made from the given character arguments.

(string #\\x #\\y #\\z) ⇒ "xyz"
(string)             ⇒ ""

Scheme Procedure: **list->string** lst [¶](06_06_05_strings.md)

C Function: **scm\_string** (lst) [¶](06_06_05_strings.md)

Return a newly allocated string made from a list of characters.

(list->string '(#\\a #\\b #\\c)) ⇒ "abc"

Scheme Procedure: **reverse-list->string** lst [¶](06_06_05_strings.md)

C Function: **scm\_reverse\_list\_to\_string** (lst) [¶](06_06_05_strings.md)

Return a newly allocated string made from a list of characters, in reverse order.

(reverse-list->string '(#\\a #\\B #\\c)) ⇒ "cBa"

Scheme Procedure: **make-string** k \[chr\] [¶](06_06_05_strings.md)

C Function: **scm\_make\_string** (k, chr) [¶](06_06_05_strings.md)

Return a newly allocated string of length k. If chr is given, then all elements of the string are initialized to chr, otherwise the contents of the string are unspecified.

C Function: `SCM` **scm\_c\_make\_string** `(size_t len, SCM chr)` [¶](06_06_05_strings.md)

Like `scm_make_string`, but expects the length as a `size_t`.

Scheme Procedure: **string-tabulate** proc len [¶](06_06_05_strings.md)

C Function: **scm\_string\_tabulate** (proc, len) [¶](06_06_05_strings.md)

proc is an integer->char procedure. Construct a string of size len by applying proc to each index to produce the corresponding string element. The order in which proc is applied to the indices is not specified.

Scheme Procedure: **string-join** ls \[delimiter \[grammar\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_join** (ls, delimiter, grammar) [¶](06_06_05_strings.md)

Append the string in the string list ls, using the string delimiter as a delimiter between the elements of ls. delimiter defaults to ‘ ’, that is, strings in ls are appended with the space character in between them. grammar is a symbol which specifies how the delimiter is placed between the strings, and defaults to the symbol `infix`.

`infix`

Insert the separator between list elements. An empty string will produce an empty list.

`strict-infix`

Like `infix`, but will raise an error if given the empty list.

`suffix`

Insert the separator after every list element.

`prefix`

Insert the separator before each list element.

* * *

Next: [String Selection](06_06_05_strings.md#6655-string-selection), Previous: [String Constructors](06_06_05_strings.md#6653-string-constructors), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.4 List/String conversion [¶](06_06_05_strings.md#6654-liststring-conversion)

When processing strings, it is often convenient to first convert them into a list representation by using the procedure `string->list`, work with the resulting list, and then convert it back into a string. These procedures are useful for similar tasks.

Scheme Procedure: **string->list** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_to\_list** (str, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_to\_list** (str) [¶](06_06_05_strings.md)

Convert the string str into a list of characters.

Scheme Procedure: **string-split** str char\_pred [¶](06_06_05_strings.md)

C Function: **scm\_string\_split** (str, char\_pred) [¶](06_06_05_strings.md)

Split the string str into a list of substrings delimited by appearances of characters that

*   equal char\_pred, if it is a character,
*   satisfy the predicate char\_pred, if it is a procedure,
*   are in the set char\_pred, if it is a character set.

Note that an empty substring between separator characters will result in an empty string in the result list.

([string-split](06_06_05_strings.md) "root:x:0:0:root:/root:/bin/bash" #\\:)
⇒
("root" "x" "0" "0" "root" "/root" "/bin/bash")

([string-split](06_06_05_strings.md) "::" #\\:)
⇒
("" "" "")

([string-split](06_06_05_strings.md) "" #\\:)
⇒
("")

* * *

Next: [String Modification](06_06_05_strings.md#6656-string-modification), Previous: [List/String conversion](06_06_05_strings.md#6654-liststring-conversion), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.5 String Selection [¶](06_06_05_strings.md#6655-string-selection)

Portions of strings can be extracted by these procedures. `string-ref` delivers individual characters whereas `substring` can be used to extract substrings from longer strings.

Scheme Procedure: **string-length** string [¶](06_06_05_strings.md)

C Function: **scm\_string\_length** (string) [¶](06_06_05_strings.md)

Return the number of characters in string.

C Function: `size_t` **scm\_c\_string\_length** `(SCM str)` [¶](06_06_05_strings.md)

Return the number of characters in str as a `size_t`.

Scheme Procedure: **string-ref** str k [¶](06_06_05_strings.md)

C Function: **scm\_string\_ref** (str, k) [¶](06_06_05_strings.md)

Return character k of str using zero-origin indexing. k must be a valid index of str.

C Function: `SCM` **scm\_c\_string\_ref** `(SCM str, size_t k)` [¶](06_06_05_strings.md)

Return character k of str using zero-origin indexing. k must be a valid index of str.

Scheme Procedure: **string-copy** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_copy** (str, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_copy** (str) [¶](06_06_05_strings.md)

Return a copy of the given string str.

The returned string shares storage with str initially, but it is copied as soon as one of the two strings is modified.

Scheme Procedure: **substring** str start \[end\] [¶](06_06_05_strings.md)

C Function: **scm\_substring** (str, start, end) [¶](06_06_05_strings.md)

Return a new string formed from the characters of str beginning with index start (inclusive) and ending with index end (exclusive). str must be a string, start and end must be exact integers satisfying:

0 <= start <= end <= `(string-length str)`.

The returned string shares storage with str initially, but it is copied as soon as one of the two strings is modified.

Scheme Procedure: **substring/shared** str start \[end\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_shared** (str, start, end) [¶](06_06_05_strings.md)

Like `substring`, but the strings continue to share their storage even if they are modified. Thus, modifications to str show up in the new string, and vice versa.

Scheme Procedure: **substring/copy** str start \[end\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_copy** (str, start, end) [¶](06_06_05_strings.md)

Like `substring`, but the storage for the new string is copied immediately.

Scheme Procedure: **substring/read-only** str start \[end\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_read\_only** (str, start, end) [¶](06_06_05_strings.md)

Like `substring`, but the resulting string can not be modified.

C Function: `SCM` **scm\_c\_substring** `(SCM str, size_t start, size_t end)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_c\_substring\_shared** `(SCM str, size_t start, size_t end)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_c\_substring\_copy** `(SCM str, size_t start, size_t end)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_c\_substring\_read\_only** `(SCM str, size_t start, size_t end)` [¶](06_06_05_strings.md)

Like `scm_substring`, etc. but the bounds are given as a `size_t`.

Scheme Procedure: **string-take** s n [¶](06_06_05_strings.md)

C Function: **scm\_string\_take** (s, n) [¶](06_06_05_strings.md)

Return the n first characters of s.

Scheme Procedure: **string-drop** s n [¶](06_06_05_strings.md)

C Function: **scm\_string\_drop** (s, n) [¶](06_06_05_strings.md)

Return all but the first n characters of s.

Scheme Procedure: **string-take-right** s n [¶](06_06_05_strings.md)

C Function: **scm\_string\_take\_right** (s, n) [¶](06_06_05_strings.md)

Return the n last characters of s.

Scheme Procedure: **string-drop-right** s n [¶](06_06_05_strings.md)

C Function: **scm\_string\_drop\_right** (s, n) [¶](06_06_05_strings.md)

Return all but the last n characters of s.

Scheme Procedure: **string-pad** s len \[chr \[start \[end\]\]\] [¶](06_06_05_strings.md)

Scheme Procedure: **string-pad-right** s len \[chr \[start \[end\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_pad** (s, len, chr, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_pad\_right** (s, len, chr, start, end) [¶](06_06_05_strings.md)

Take characters start to end from the string s and either pad with chr or truncate them to give len characters.

`string-pad` pads or truncates on the left, so for example

(string-pad "x" 3)     ⇒ "  x"
(string-pad "abcde" 3) ⇒ "cde"

`string-pad-right` pads or truncates on the right, so for example

(string-pad-right "x" 3)     ⇒ "x  "
(string-pad-right "abcde" 3) ⇒ "abc"

Scheme Procedure: **string-trim** s \[char\_pred \[start \[end\]\]\] [¶](06_06_05_strings.md)

Scheme Procedure: **string-trim-right** s \[char\_pred \[start \[end\]\]\] [¶](06_06_05_strings.md)

Scheme Procedure: **string-trim-both** s \[char\_pred \[start \[end\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_trim** (s, char\_pred, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_trim\_right** (s, char\_pred, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_trim\_both** (s, char\_pred, start, end) [¶](06_06_05_strings.md)

Trim occurrences of char\_pred from the ends of s.

`string-trim` trims char\_pred characters from the left (start) of the string, `string-trim-right` trims them from the right (end) of the string, `string-trim-both` trims from both ends.

char\_pred can be a character, a character set, or a predicate procedure to call on each character. If char\_pred is not given the default is whitespace as per `char-set:whitespace` (see [Standard Character Sets](06_06_04_character_sets.md#6646-standard-character-sets)).

(string-trim " x ")              ⇒ "x "
(string-trim-right "banana" #\\a) ⇒ "banan"
(string-trim-both ".,xy:;" char-set:punctuation)
                  ⇒ "xy"
(string-trim-both "xyzzy" (lambda (c)
                             (or (eqv? c #\\x)
                                 (eqv? c #\\y))))
                  ⇒ "zz"

* * *

Next: [String Comparison](06_06_05_strings.md#6657-string-comparison), Previous: [String Selection](06_06_05_strings.md#6655-string-selection), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.6 String Modification [¶](06_06_05_strings.md#6656-string-modification)

These procedures are for modifying strings in-place. This means that the result of the operation is not a new string; instead, the original string’s memory representation is modified.

Scheme Procedure: **string-set!** str k chr [¶](06_06_05_strings.md)

C Function: **scm\_string\_set\_x** (str, k, chr) [¶](06_06_05_strings.md)

Store chr in element k of str and return an unspecified value. k must be a valid index of str.

C Function: `void` **scm\_c\_string\_set\_x** `(SCM str, size_t k, SCM chr)` [¶](06_06_05_strings.md)

Like `scm_string_set_x`, but the index is given as a `size_t`.

Scheme Procedure: **string-fill!** str chr \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_fill\_x** (str, chr, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_fill\_x** (str, chr) [¶](06_06_05_strings.md)

Stores chr in every element of the given str and returns an unspecified value.

Scheme Procedure: **substring-fill!** str start end fill [¶](06_06_05_strings.md)

C Function: **scm\_substring\_fill\_x** (str, start, end, fill) [¶](06_06_05_strings.md)

Change every character in str between start and end to fill.

(define y ([string-copy](06_06_05_strings.md) "abcdefg"))
([substring-fill!](06_06_05_strings.md) y 1 3 #\\r)
y
⇒ "arrdefg"

Scheme Procedure: **substring-move!** str1 start1 end1 str2 start2 [¶](06_06_05_strings.md)

C Function: **scm\_substring\_move\_x** (str1, start1, end1, str2, start2) [¶](06_06_05_strings.md)

Copy the substring of str1 bounded by start1 and end1 into str2 beginning at position start2. str1 and str2 can be the same string.

Scheme Procedure: **string-copy!** target tstart s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_copy\_x** (target, tstart, s, start, end) [¶](06_06_05_strings.md)

Copy the sequence of characters from index range \[start, end) in string s to string target, beginning at index tstart. The characters are copied left-to-right or right-to-left as needed – the copy is guaranteed to work, even if target and s are the same string. It is an error if the copy operation runs off the end of the target string.

* * *

Next: [String Searching](06_06_05_strings.md#6658-string-searching), Previous: [String Modification](06_06_05_strings.md#6656-string-modification), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.7 String Comparison [¶](06_06_05_strings.md#6657-string-comparison)

The procedures in this section are similar to the character ordering predicates (see [Characters](06_06_03_characters.md#663-characters)), but are defined on character sequences.

The first set is specified in R5RS and has names that end in `?`. The second set is specified in SRFI-13 and the names have not ending `?`.

The predicates ending in `-ci` ignore the character case when comparing strings. For now, case-insensitive comparison is done using the R5RS rules, where every lower-case character that has a single character upper-case form is converted to uppercase before comparison. See See [the `(ice-9 i18n)` module](06_25_support_for_internationalization.md#6252-text-collation), for locale-dependent string comparison.

Scheme Procedure: **string=?** s1 s2 s3 … [¶](06_06_05_strings.md)

Lexicographic equality predicate; return `#t` if all strings are the same length and contain the same characters in the same positions, otherwise return `#f`.

The procedure `string-ci=?` treats upper and lower case letters as though they were the same character, but `string=?` treats upper and lower case as distinct characters.

Scheme Procedure: **string<?** s1 s2 s3 … [¶](06_06_05_strings.md)

Lexicographic ordering predicate; return `#t` if, for every pair of consecutive string arguments str\_i and str\_i+1, str\_i is lexicographically less than str\_i+1.

Scheme Procedure: **string<=?** s1 s2 s3 … [¶](06_06_05_strings.md)

Lexicographic ordering predicate; return `#t` if, for every pair of consecutive string arguments str\_i and str\_i+1, str\_i is lexicographically less than or equal to str\_i+1.

Scheme Procedure: **string>?** s1 s2 s3 … [¶](06_06_05_strings.md)

Lexicographic ordering predicate; return `#t` if, for every pair of consecutive string arguments str\_i and str\_i+1, str\_i is lexicographically greater than str\_i+1.

Scheme Procedure: **string>=?** s1 s2 s3 … [¶](06_06_05_strings.md)

Lexicographic ordering predicate; return `#t` if, for every pair of consecutive string arguments str\_i and str\_i+1, str\_i is lexicographically greater than or equal to str\_i+1.

Scheme Procedure: **string-ci=?** s1 s2 s3 … [¶](06_06_05_strings.md)

Case-insensitive string equality predicate; return `#t` if all strings are the same length and their component characters match (ignoring case) at each position; otherwise return `#f`.

Scheme Procedure: **string-ci<?** s1 s2 s3 … [¶](06_06_05_strings.md)

Case insensitive lexicographic ordering predicate; return `#t` if, for every pair of consecutive string arguments str\_i and str\_i+1, str\_i is lexicographically less than str\_i+1 regardless of case.

Scheme Procedure: **string-ci<=?** s1 s2 s3 … [¶](06_06_05_strings.md)

Case insensitive lexicographic ordering predicate; return `#t` if, for every pair of consecutive string arguments str\_i and str\_i+1, str\_i is lexicographically less than or equal to str\_i+1 regardless of case.

Scheme Procedure: **string-ci>?** s1 s2 s3 … [¶](06_06_05_strings.md)

Case insensitive lexicographic ordering predicate; return `#t` if, for every pair of consecutive string arguments str\_i and str\_i+1, str\_i is lexicographically greater than str\_i+1 regardless of case.

Scheme Procedure: **string-ci>=?** s1 s2 s3 … [¶](06_06_05_strings.md)

Case insensitive lexicographic ordering predicate; return `#t` if, for every pair of consecutive string arguments str\_i and str\_i+1, str\_i is lexicographically greater than or equal to str\_i+1 regardless of case.

Scheme Procedure: **string-compare** s1 s2 proc\_lt proc\_eq proc\_gt \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_compare** (s1, s2, proc\_lt, proc\_eq, proc\_gt, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Apply proc\_lt, proc\_eq, proc\_gt to the mismatch index, depending upon whether s1 is less than, equal to, or greater than s2. The mismatch index is the largest index i such that for every 0 <= j < i, s1\[j\] = s2\[j\] – that is, i is the first position that does not match.

Scheme Procedure: **string-compare-ci** s1 s2 proc\_lt proc\_eq proc\_gt \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_compare\_ci** (s1, s2, proc\_lt, proc\_eq, proc\_gt, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Apply proc\_lt, proc\_eq, proc\_gt to the mismatch index, depending upon whether s1 is less than, equal to, or greater than s2. The mismatch index is the largest index i such that for every 0 <= j < i, s1\[j\] = s2\[j\] – that is, i is the first position where the lowercased letters do not match.

Scheme Procedure: **string=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_eq** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 and s2 are not equal, a true value otherwise.

Scheme Procedure: **string<>** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_neq** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 and s2 are equal, a true value otherwise.

Scheme Procedure: **string<** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_lt** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 is greater or equal to s2, a true value otherwise.

Scheme Procedure: **string>** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_gt** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 is less or equal to s2, a true value otherwise.

Scheme Procedure: **string<=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_le** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 is greater to s2, a true value otherwise.

Scheme Procedure: **string>=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_ge** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 is less to s2, a true value otherwise.

Scheme Procedure: **string-ci=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_ci\_eq** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 and s2 are not equal, a true value otherwise. The character comparison is done case-insensitively.

Scheme Procedure: **string-ci<>** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_ci\_neq** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 and s2 are equal, a true value otherwise. The character comparison is done case-insensitively.

Scheme Procedure: **string-ci<** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_ci\_lt** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 is greater or equal to s2, a true value otherwise. The character comparison is done case-insensitively.

Scheme Procedure: **string-ci>** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_ci\_gt** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 is less or equal to s2, a true value otherwise. The character comparison is done case-insensitively.

Scheme Procedure: **string-ci<=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_ci\_le** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 is greater to s2, a true value otherwise. The character comparison is done case-insensitively.

Scheme Procedure: **string-ci>=** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_ci\_ge** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return `#f` if s1 is less to s2, a true value otherwise. The character comparison is done case-insensitively.

Scheme Procedure: **string-hash** s \[bound \[start \[end\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_hash** (s, bound, start, end) [¶](06_06_05_strings.md)

Compute a hash value for s. The optional argument bound is a non-negative exact integer specifying the range of the hash function. A positive value restricts the return value to the range \[0,bound).

Scheme Procedure: **string-hash-ci** s \[bound \[start \[end\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_hash\_ci** (s, bound, start, end) [¶](06_06_05_strings.md)

Compute a hash value for s. The optional argument bound is a non-negative exact integer specifying the range of the hash function. A positive value restricts the return value to the range \[0,bound).

Because the same visual appearance of an abstract Unicode character can be obtained via multiple sequences of Unicode characters, even the case-insensitive string comparison functions described above may return `#f` when presented with strings containing different representations of the same character. For example, the Unicode character “LATIN SMALL LETTER S WITH DOT BELOW AND DOT ABOVE” can be represented with a single character (U+1E69) or by the character “LATIN SMALL LETTER S” (U+0073) followed by the combining marks “COMBINING DOT BELOW” (U+0323) and “COMBINING DOT ABOVE” (U+0307).

For this reason, it is often desirable to ensure that the strings to be compared are using a mutually consistent representation for every character. The Unicode standard defines two methods of normalizing the contents of strings: Decomposition, which breaks composite characters into a set of constituent characters with an ordering defined by the Unicode Standard; and composition, which performs the converse.

There are two decomposition operations. “Canonical decomposition” produces character sequences that share the same visual appearance as the original characters, while “compatibility decomposition” produces ones whose visual appearances may differ from the originals but which represent the same abstract character.

These operations are encapsulated in the following set of normalization forms:

_NFD_

Characters are decomposed to their canonical forms.

_NFKD_

Characters are decomposed to their compatibility forms.

_NFC_

Characters are decomposed to their canonical forms, then composed.

_NFKC_

Characters are decomposed to their compatibility forms, then composed.

The functions below put their arguments into one of the forms described above.

Scheme Procedure: **string-normalize-nfd** s [¶](06_06_05_strings.md)

C Function: **scm\_string\_normalize\_nfd** (s) [¶](06_06_05_strings.md)

Return the `NFD` normalized form of s.

Scheme Procedure: **string-normalize-nfkd** s [¶](06_06_05_strings.md)

C Function: **scm\_string\_normalize\_nfkd** (s) [¶](06_06_05_strings.md)

Return the `NFKD` normalized form of s.

Scheme Procedure: **string-normalize-nfc** s [¶](06_06_05_strings.md)

C Function: **scm\_string\_normalize\_nfc** (s) [¶](06_06_05_strings.md)

Return the `NFC` normalized form of s.

Scheme Procedure: **string-normalize-nfkc** s [¶](06_06_05_strings.md)

C Function: **scm\_string\_normalize\_nfkc** (s) [¶](06_06_05_strings.md)

Return the `NFKC` normalized form of s.

* * *

Next: [Alphabetic Case Mapping](06_06_05_strings.md#6659-alphabetic-case-mapping), Previous: [String Comparison](06_06_05_strings.md#6657-string-comparison), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.8 String Searching [¶](06_06_05_strings.md#6658-string-searching)

Scheme Procedure: **string-index** s char\_pred \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_index** (s, char\_pred, start, end) [¶](06_06_05_strings.md)

Search through the string s from left to right, returning the index of the first occurrence of a character which

*   equals char\_pred, if it is character,
*   satisfies the predicate char\_pred, if it is a procedure,
*   is in the set char\_pred, if it is a character set.

Return `#f` if no match is found.

Scheme Procedure: **string-rindex** s char\_pred \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_rindex** (s, char\_pred, start, end) [¶](06_06_05_strings.md)

Search through the string s from right to left, returning the index of the last occurrence of a character which

*   equals char\_pred, if it is character,
*   satisfies the predicate char\_pred, if it is a procedure,
*   is in the set if char\_pred is a character set.

Return `#f` if no match is found.

Scheme Procedure: **string-prefix-length** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_prefix\_length** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return the length of the longest common prefix of the two strings.

Scheme Procedure: **string-prefix-length-ci** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_prefix\_length\_ci** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return the length of the longest common prefix of the two strings, ignoring character case.

Scheme Procedure: **string-suffix-length** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_suffix\_length** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return the length of the longest common suffix of the two strings.

Scheme Procedure: **string-suffix-length-ci** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_suffix\_length\_ci** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return the length of the longest common suffix of the two strings, ignoring character case.

Scheme Procedure: **string-prefix?** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_prefix\_p** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Is s1 a prefix of s2?

Scheme Procedure: **string-prefix-ci?** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_prefix\_ci\_p** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Is s1 a prefix of s2, ignoring character case?

Scheme Procedure: **string-suffix?** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_suffix\_p** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Is s1 a suffix of s2?

Scheme Procedure: **string-suffix-ci?** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_suffix\_ci\_p** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Is s1 a suffix of s2, ignoring character case?

Scheme Procedure: **string-index-right** s char\_pred \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_index\_right** (s, char\_pred, start, end) [¶](06_06_05_strings.md)

Search through the string s from right to left, returning the index of the last occurrence of a character which

*   equals char\_pred, if it is character,
*   satisfies the predicate char\_pred, if it is a procedure,
*   is in the set if char\_pred is a character set.

Return `#f` if no match is found.

Scheme Procedure: **string-skip** s char\_pred \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_skip** (s, char\_pred, start, end) [¶](06_06_05_strings.md)

Search through the string s from left to right, returning the index of the first occurrence of a character which

*   does not equal char\_pred, if it is character,
*   does not satisfy the predicate char\_pred, if it is a procedure,
*   is not in the set if char\_pred is a character set.

Scheme Procedure: **string-skip-right** s char\_pred \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_skip\_right** (s, char\_pred, start, end) [¶](06_06_05_strings.md)

Search through the string s from right to left, returning the index of the last occurrence of a character which

*   does not equal char\_pred, if it is character,
*   does not satisfy the predicate char\_pred, if it is a procedure,
*   is not in the set if char\_pred is a character set.

Scheme Procedure: **string-count** s char\_pred \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_count** (s, char\_pred, start, end) [¶](06_06_05_strings.md)

Return the count of the number of characters in the string s which

*   equals char\_pred, if it is character,
*   satisfies the predicate char\_pred, if it is a procedure.
*   is in the set char\_pred, if it is a character set.

Scheme Procedure: **string-contains** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_contains** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Does string s1 contain string s2? Return the index in s1 where s2 occurs as a substring, or false. The optional start/end indices restrict the operation to the indicated substrings.

Scheme Procedure: **string-contains-ci** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_contains\_ci** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Does string s1 contain string s2? Return the index in s1 where s2 occurs as a substring, or false. The optional start/end indices restrict the operation to the indicated substrings. Character comparison is done case-insensitively.

* * *

Next: [Reversing and Appending Strings](06_06_05_strings.md#66510-reversing-and-appending-strings), Previous: [String Searching](06_06_05_strings.md#6658-string-searching), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.9 Alphabetic Case Mapping [¶](06_06_05_strings.md#6659-alphabetic-case-mapping)

These are procedures for mapping strings to their upper- or lower-case equivalents, respectively, or for capitalizing strings.

They use the basic case mapping rules for Unicode characters. No special language or context rules are considered. The resulting strings are guaranteed to be the same length as the input strings.

See [the `(ice-9 i18n)` module](06_25_support_for_internationalization.md#6253-character-case-mapping), for locale-dependent case conversions.

Scheme Procedure: **string-upcase** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_upcase** (str, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_upcase** (str) [¶](06_06_05_strings.md)

Upcase every character in `str`.

Scheme Procedure: **string-upcase!** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_upcase\_x** (str, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_upcase\_x** (str) [¶](06_06_05_strings.md)

Destructively upcase every character in `str`.

([string-upcase!](06_06_05_strings.md) y)
⇒ "ARRDEFG"
y
⇒ "ARRDEFG"

Scheme Procedure: **string-downcase** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_downcase** (str, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_downcase** (str) [¶](06_06_05_strings.md)

Downcase every character in str.

Scheme Procedure: **string-downcase!** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_substring\_downcase\_x** (str, start, end) [¶](06_06_05_strings.md)

C Function: **scm\_string\_downcase\_x** (str) [¶](06_06_05_strings.md)

Destructively downcase every character in str.

y
⇒ "ARRDEFG"
([string-downcase!](06_06_05_strings.md) y)
⇒ "arrdefg"
y
⇒ "arrdefg"

Scheme Procedure: **string-capitalize** str [¶](06_06_05_strings.md)

C Function: **scm\_string\_capitalize** (str) [¶](06_06_05_strings.md)

Return a freshly allocated string with the characters in str, where the first character of every word is capitalized.

Scheme Procedure: **string-capitalize!** str [¶](06_06_05_strings.md)

C Function: **scm\_string\_capitalize\_x** (str) [¶](06_06_05_strings.md)

Upcase the first character of every word in str destructively and return str.

y                      ⇒ "hello world"
([string-capitalize!](06_06_05_strings.md) y) ⇒ "Hello World"
y                      ⇒ "Hello World"

Scheme Procedure: **string-titlecase** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_titlecase** (str, start, end) [¶](06_06_05_strings.md)

Titlecase every first character in a word in str.

Scheme Procedure: **string-titlecase!** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_titlecase\_x** (str, start, end) [¶](06_06_05_strings.md)

Destructively titlecase every first character in a word in str.

* * *

Next: [Mapping, Folding, and Unfolding](06_06_05_strings.md#66511-mapping-folding-and-unfolding), Previous: [Alphabetic Case Mapping](06_06_05_strings.md#6659-alphabetic-case-mapping), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.10 Reversing and Appending Strings [¶](06_06_05_strings.md#66510-reversing-and-appending-strings)

Scheme Procedure: **string-reverse** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_reverse** (str, start, end) [¶](06_06_05_strings.md)

Reverse the string str. The optional arguments start and end delimit the region of str to operate on.

Scheme Procedure: **string-reverse!** str \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_reverse\_x** (str, start, end) [¶](06_06_05_strings.md)

Reverse the string str in-place. The optional arguments start and end delimit the region of str to operate on. The return value is unspecified.

Scheme Procedure: **string-append** arg … [¶](06_06_05_strings.md)

C Function: **scm\_string\_append** (args) [¶](06_06_05_strings.md)

Return a newly allocated string whose characters form the concatenation of the given strings, arg ....

(let ((h "hello "))
  (string-append h "world"))
⇒ "hello world"

Scheme Procedure: **string-append/shared** arg … [¶](06_06_05_strings.md)

C Function: **scm\_string\_append\_shared** (args) [¶](06_06_05_strings.md)

Like `string-append`, but the result may share memory with the argument strings.

Scheme Procedure: **string-concatenate** ls [¶](06_06_05_strings.md)

C Function: **scm\_string\_concatenate** (ls) [¶](06_06_05_strings.md)

Append the elements (which must be strings) of ls together into a single string. Guaranteed to return a freshly allocated string.

Scheme Procedure: **string-concatenate-reverse** ls \[final\_string \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_concatenate\_reverse** (ls, final\_string, end) [¶](06_06_05_strings.md)

Without optional arguments, this procedure is equivalent to

([string-concatenate](06_06_05_strings.md) ([reverse](06_06_09_lists.md) ls))

If the optional argument final\_string is specified, it is consed onto the beginning to ls before performing the list-reverse and string-concatenate operations. If end is given, only the characters of final\_string up to index end are used.

Guaranteed to return a freshly allocated string.

Scheme Procedure: **string-concatenate/shared** ls [¶](06_06_05_strings.md)

C Function: **scm\_string\_concatenate\_shared** (ls) [¶](06_06_05_strings.md)

Like `string-concatenate`, but the result may share memory with the strings in the list ls.

Scheme Procedure: **string-concatenate-reverse/shared** ls \[final\_string \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_concatenate\_reverse\_shared** (ls, final\_string, end) [¶](06_06_05_strings.md)

Like `string-concatenate-reverse`, but the result may share memory with the strings in the ls arguments.

* * *

Next: [Miscellaneous String Operations](06_06_05_strings.md#66512-miscellaneous-string-operations), Previous: [Reversing and Appending Strings](06_06_05_strings.md#66510-reversing-and-appending-strings), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.11 Mapping, Folding, and Unfolding [¶](06_06_05_strings.md#66511-mapping-folding-and-unfolding)

Scheme Procedure: **string-map** proc s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_map** (proc, s, start, end) [¶](06_06_05_strings.md)

proc is a char->char procedure, it is mapped over s. The order in which the procedure is applied to the string elements is not specified.

Scheme Procedure: **string-map!** proc s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_map\_x** (proc, s, start, end) [¶](06_06_05_strings.md)

proc is a char->char procedure, it is mapped over s. The order in which the procedure is applied to the string elements is not specified. The string s is modified in-place, the return value is not specified.

Scheme Procedure: **string-for-each** proc s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_for\_each** (proc, s, start, end) [¶](06_06_05_strings.md)

proc is mapped over s in left-to-right order. The return value is not specified.

Scheme Procedure: **string-for-each-index** proc s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_for\_each\_index** (proc, s, start, end) [¶](06_06_05_strings.md)

Call `(proc i)` for each index i in s, from left to right.

For example, to change characters to alternately upper and lower case,

(define str (string-copy "studly"))
(string-for-each-index
    (lambda (i)
      (string-set! str i
        ((if (even? i) char-upcase char-downcase)
         (string-ref str i))))
    str)
str ⇒ "StUdLy"

Scheme Procedure: **string-fold** kons knil s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_fold** (kons, knil, s, start, end) [¶](06_06_05_strings.md)

Fold kons over the characters of s, with knil as the terminating element, from left to right. kons must expect two arguments: The actual character and the last result of kons’ application.

Scheme Procedure: **string-fold-right** kons knil s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_fold\_right** (kons, knil, s, start, end) [¶](06_06_05_strings.md)

Fold kons over the characters of s, with knil as the terminating element, from right to left. kons must expect two arguments: The actual character and the last result of kons’ application.

Scheme Procedure: **string-unfold** p f g seed \[base \[make\_final\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_unfold** (p, f, g, seed, base, make\_final) [¶](06_06_05_strings.md)

*   g is used to generate a series of _seed_ values from the initial seed: seed, (g seed), (g^2 seed), (g^3 seed), …
*   p tells us when to stop – when it returns true when applied to one of these seed values.
*   f maps each seed value to the corresponding character in the result string. These chars are assembled into the string in a left-to-right order.
*   base is the optional initial/leftmost portion of the constructed string; it default to the empty string.
*   make\_final is applied to the terminal seed value (on which p returns true) to produce the final/rightmost portion of the constructed string. The default is nothing extra.

Scheme Procedure: **string-unfold-right** p f g seed \[base \[make\_final\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_unfold\_right** (p, f, g, seed, base, make\_final) [¶](06_06_05_strings.md)

*   g is used to generate a series of _seed_ values from the initial seed: seed, (g seed), (g^2 seed), (g^3 seed), …
*   p tells us when to stop – when it returns true when applied to one of these seed values.
*   f maps each seed value to the corresponding character in the result string. These chars are assembled into the string in a right-to-left order.
*   base is the optional initial/rightmost portion of the constructed string; it default to the empty string.
*   make\_final is applied to the terminal seed value (on which p returns true) to produce the final/leftmost portion of the constructed string. It defaults to `(lambda (x) )`.

* * *

Next: [Representing Strings as Bytes](06_06_05_strings.md#66513-representing-strings-as-bytes), Previous: [Mapping, Folding, and Unfolding](06_06_05_strings.md#66511-mapping-folding-and-unfolding), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.12 Miscellaneous String Operations [¶](06_06_05_strings.md#66512-miscellaneous-string-operations)

Scheme Procedure: **xsubstring** s from \[to \[start \[end\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_xsubstring** (s, from, to, start, end) [¶](06_06_05_strings.md)

This is the _extended substring_ procedure that implements replicated copying of a substring of some string.

s is a string, start and end are optional arguments that demarcate a substring of s, defaulting to 0 and the length of s. Replicate this substring up and down index space, in both the positive and negative directions. `xsubstring` returns the substring of this string beginning at index from, and ending at to, which defaults to from + (end - start).

Scheme Procedure: **string-xcopy!** target tstart s sfrom \[sto \[start \[end\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_xcopy\_x** (target, tstart, s, sfrom, sto, start, end) [¶](06_06_05_strings.md)

Exactly the same as `xsubstring`, but the extracted text is written into the string target starting at index tstart. The operation is not defined if `(eq? target s)` or these arguments share storage – you cannot copy a string on top of itself.

Scheme Procedure: **string-replace** s1 s2 \[start1 \[end1 \[start2 \[end2\]\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_replace** (s1, s2, start1, end1, start2, end2) [¶](06_06_05_strings.md)

Return the string s1, but with the characters start1 … end1 replaced by the characters start2 … end2 from s2.

Scheme Procedure: **string-tokenize** s \[token\_set \[start \[end\]\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_tokenize** (s, token\_set, start, end) [¶](06_06_05_strings.md)

Split the string s into a list of substrings, where each substring is a maximal non-empty contiguous sequence of characters from the character set token\_set, which defaults to `char-set:graphic`. If start or end indices are provided, they restrict `string-tokenize` to operating on the indicated substring of s.

Scheme Procedure: **string-filter** char\_pred s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_filter** (char\_pred, s, start, end) [¶](06_06_05_strings.md)

Filter the string s, retaining only those characters which satisfy char\_pred.

If char\_pred is a procedure, it is applied to each character as a predicate, if it is a character, it is tested for equality and if it is a character set, it is tested for membership.

Scheme Procedure: **string-delete** char\_pred s \[start \[end\]\] [¶](06_06_05_strings.md)

C Function: **scm\_string\_delete** (char\_pred, s, start, end) [¶](06_06_05_strings.md)

Delete characters satisfying char\_pred from s.

If char\_pred is a procedure, it is applied to each character as a predicate, if it is a character, it is tested for equality and if it is a character set, it is tested for membership.

The following additional functions are available in the module `(ice-9 string-fun)`. They can be used with:

(use-modules (ice-9 string-fun))

Scheme Procedure: **string-replace-substring** str substring replacement [¶](06_06_05_strings.md)

Return a new string where every instance of substring in string str has been replaced by replacement. For example:

([string-replace-substring](06_06_05_strings.md) "a ring of strings" "ring" "rut")
⇒ "a rut of struts"

* * *

Next: [Conversion to/from C](06_06_05_strings.md#66514-conversion-tofrom-c), Previous: [Miscellaneous String Operations](06_06_05_strings.md#66512-miscellaneous-string-operations), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.13 Representing Strings as Bytes [¶](06_06_05_strings.md#66513-representing-strings-as-bytes)

In the cold world outside of Guile, not all strings are treated in the same way. Out there there are only bytes, and there are many ways of representing a strings (sequences of characters) as binary data (sequences of bytes).

As a user, usually you don’t have to think about this very much. When you type on your keyboard, your system encodes your keystrokes as bytes according to the locale that you have configured on your computer. Guile uses the locale to decode those bytes back into characters – hopefully the same characters that you typed in.

All is not so clear when dealing with a system with multiple users, such as a web server. Your web server might get a request from one user for data encoded in the ISO-8859-1 character set, and then another request from a different user for UTF-8 data.

Guile provides an _iconv_ module for converting between strings and sequences of bytes. See [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors), for more on how Guile represents raw byte sequences. This module gets its name from the common UNIX command of the same name.

Note that often it is sufficient to just read and write strings from ports instead of using these functions. To do this, specify the port encoding using `set-port-encoding!`. See [Ports](06_12_input_and_output.md#6121-ports), for more on ports and character encodings.

Unlike the rest of the procedures in this section, you have to load the `iconv` module before having access to these procedures:

(use-modules (ice-9 iconv))

Scheme Procedure: **string->bytevector** string encoding \[conversion-strategy\] [¶](06_06_05_strings.md)

Encode string as a sequence of bytes.

The string will be encoded in the character set specified by the encoding string. If the string has characters that cannot be represented in the encoding, by default this procedure raises an `encoding-error`. Pass a conversion-strategy argument to specify other behaviors.

The return value is a bytevector. See [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors), for more on bytevectors. See [Ports](06_12_input_and_output.md#6121-ports), for more on character encodings and conversion strategies.

Scheme Procedure: **bytevector->string** bytevector encoding \[conversion-strategy\] [¶](06_06_05_strings.md)

Decode bytevector into a string.

The bytes will be decoded from the character set by the encoding string. If the bytes do not form a valid encoding, by default this procedure raises an `decoding-error`. As with `string->bytevector`, pass the optional conversion-strategy argument to modify this behavior. See [Ports](06_12_input_and_output.md#6121-ports), for more on character encodings and conversion strategies.

Scheme Procedure: **call-with-output-encoded-string** encoding proc \[conversion-strategy\] [¶](06_06_05_strings.md)

Like `call-with-output-string`, but instead of returning a string, returns a encoding of the string according to encoding, as a bytevector. This procedure can be more efficient than collecting a string and then converting it via `string->bytevector`.

* * *

Next: [String Internals](06_06_05_strings.md#66515-string-internals), Previous: [Representing Strings as Bytes](06_06_05_strings.md#66513-representing-strings-as-bytes), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.14 Conversion to/from C [¶](06_06_05_strings.md#66514-conversion-tofrom-c)

When creating a Scheme string from a C string or when converting a Scheme string to a C string, the concept of character encoding becomes important.

In C, a string is just a sequence of bytes, and the character encoding describes the relation between these bytes and the actual characters that make up the string. For Scheme strings, character encoding is not an issue (most of the time), since in Scheme you usually treat strings as character sequences, not byte sequences.

Converting to C and converting from C each have their own challenges.

When converting from C to Scheme, it is important that the sequence of bytes in the C string be valid with respect to its encoding. ASCII strings, for example, can’t have any bytes greater than 127. An ASCII byte greater than 127 is considered _ill-formed_ and cannot be converted into a Scheme character.

Problems can occur in the reverse operation as well. Not all character encodings can hold all possible Scheme characters. Some encodings, like ASCII for example, can only describe a small subset of all possible characters. So, when converting to C, one must first decide what to do with Scheme characters that can’t be represented in the C string.

Converting a Scheme string to a C string will often allocate fresh memory to hold the result. You must take care that this memory is properly freed eventually. In many cases, this can be achieved by using `scm_dynwind_free` inside an appropriate dynwind context, See [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind).

C Function: `SCM` **scm\_from\_locale\_string** `(const char *str)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_from\_locale\_stringn** `(const char *str, size_t len)` [¶](06_06_05_strings.md)

Creates a new Scheme string that has the same contents as str when interpreted in the character encoding of the current locale.

For `scm_from_locale_string`, str must be null-terminated.

For `scm_from_locale_stringn`, len specifies the length of str in bytes, and str does not need to be null-terminated. If len is `(size_t)-1`, then str does need to be null-terminated and the real length will be found with `strlen`.

If the C string is ill-formed, an error will be raised.

Note that these functions should _not_ be used to convert C string constants, because there is no guarantee that the current locale will match that of the execution character set, used for string and character constants. Most modern C compilers use UTF-8 by default, so to convert C string constants we recommend `scm_from_utf8_string`.

C Function: `SCM` **scm\_take\_locale\_string** `(char *str)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_take\_locale\_stringn** `(char *str, size_t len)` [¶](06_06_05_strings.md)

Like `scm_from_locale_string` and `scm_from_locale_stringn`, respectively, but also frees str with `free` eventually. Thus, you can use this function when you would free str anyway immediately after creating the Scheme string. In certain cases, Guile can then use str directly as its internal representation.

C Function: `char *` **scm\_to\_locale\_string** `(SCM str)` [¶](06_06_05_strings.md)

C Function: `char *` **scm\_to\_locale\_stringn** `(SCM str, size_t *lenp)` [¶](06_06_05_strings.md)

Returns a C string with the same contents as str in the character encoding of the current locale. The C string must be freed with `free` eventually, maybe by using `scm_dynwind_free`, See [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind).

For `scm_to_locale_string`, the returned string is null-terminated and an error is signaled when str contains `#\nul` characters.

For `scm_to_locale_stringn` and lenp not `NULL`, str might contain `#\nul` characters and the length of the returned string in bytes is stored in `*lenp`. The returned string will not be null-terminated in this case. If lenp is `NULL`, `scm_to_locale_stringn` behaves like `scm_to_locale_string`.

If a character in str cannot be represented in the character encoding of the current locale, the default port conversion strategy is used. See [Ports](06_12_input_and_output.md#6121-ports), for more on conversion strategies.

If the conversion strategy is `error`, an error will be raised. If it is `substitute`, a replacement character, such as a question mark, will be inserted in its place. If it is `escape`, a hex escape will be inserted in its place.

C Function: `size_t` **scm\_to\_locale\_stringbuf** `(SCM str, char *buf, size_t max_len)` [¶](06_06_05_strings.md)

Puts str as a C string in the current locale encoding into the memory pointed to by buf. The buffer at buf has room for max\_len bytes and `scm_to_local_stringbuf` will never store more than that. No terminating `'\0'` will be stored.

The return value of `scm_to_locale_stringbuf` is the number of bytes that are needed for all of str, regardless of whether buf was large enough to hold them. Thus, when the return value is larger than max\_len, only max\_len bytes have been stored and you probably need to try again with a larger buffer.

For most situations, string conversion should occur using the current locale, such as with the functions above. But there may be cases where one wants to convert strings from a character encoding other than the locale’s character encoding. For these cases, the lower-level functions `scm_to_stringn` and `scm_from_stringn` are provided. These functions should seldom be necessary if one is properly using locales.

C Type: **scm\_t\_string\_failed\_conversion\_handler** [¶](06_06_05_strings.md)

This is an enumerated type that can take one of three values: `SCM_FAILED_CONVERSION_ERROR`, `SCM_FAILED_CONVERSION_QUESTION_MARK`, and `SCM_FAILED_CONVERSION_ESCAPE_SEQUENCE`. They are used to indicate a strategy for handling characters that cannot be converted to or from a given character encoding. `SCM_FAILED_CONVERSION_ERROR` indicates that a conversion should throw an error if some characters cannot be converted. `SCM_FAILED_CONVERSION_QUESTION_MARK` indicates that a conversion should replace unconvertable characters with the question mark character. And, `SCM_FAILED_CONVERSION_ESCAPE_SEQUENCE` requests that a conversion should replace an unconvertable character with an escape sequence.

While all three strategies apply when converting Scheme strings to C, only `SCM_FAILED_CONVERSION_ERROR` and `SCM_FAILED_CONVERSION_QUESTION_MARK` can be used when converting C strings to Scheme.

C Function: `char` **\*scm\_to\_stringn** `(SCM str, size_t *lenp, const char *encoding, scm_t_string_failed_conversion_handler handler)` [¶](06_06_05_strings.md)

This function returns a newly allocated C string from the Guile string str. The length of the returned string in bytes will be returned in lenp. The character encoding of the C string is passed as the ASCII, null-terminated C string encoding. The handler parameter gives a strategy for dealing with characters that cannot be converted into encoding.

If lenp is `NULL`, this function will return a null-terminated C string. It will throw an error if the string contains a null character.

The Scheme interface to this function is `string->bytevector`, from the `ice-9 iconv` module. See [Representing Strings as Bytes](06_06_05_strings.md#66513-representing-strings-as-bytes).

C Function: `SCM` **scm\_from\_stringn** `(const char *str, size_t len, const char *encoding, scm_t_string_failed_conversion_handler handler)` [¶](06_06_05_strings.md)

This function returns a scheme string from the C string str. The length in bytes of the C string is input as len. The encoding of the C string is passed as the ASCII, null-terminated C string `encoding`. The handler parameters suggests a strategy for dealing with unconvertable characters.

The Scheme interface to this function is `bytevector->string`. See [Representing Strings as Bytes](06_06_05_strings.md#66513-representing-strings-as-bytes).

The following conversion functions are provided as a convenience for the most commonly used encodings.

C Function: `SCM` **scm\_from\_latin1\_string** `(const char *str)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_from\_utf8\_string** `(const char *str)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_from\_utf32\_string** `(const scm_t_wchar *str)` [¶](06_06_05_strings.md)

Return a scheme string from the null-terminated C string str, which is ISO-8859-1-, UTF-8-, or UTF-32-encoded. These functions should be used to convert hard-coded C string constants into Scheme strings.

C Function: `SCM` **scm\_from\_latin1\_stringn** `(const char *str, size_t len)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_from\_utf8\_stringn** `(const char *str, size_t len)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_from\_utf32\_stringn** `(const scm_t_wchar *str, size_t len)` [¶](06_06_05_strings.md)

Return a scheme string from C string str, which is ISO-8859-1-, UTF-8-, or UTF-32-encoded, of length len. len is the number of bytes pointed to by str for `scm_from_latin1_stringn` and `scm_from_utf8_stringn`; it is the number of elements (code points) in str in the case of `scm_from_utf32_stringn`.

C function: `char` **\*scm\_to\_latin1\_stringn** `(SCM str, size_t *lenp)` [¶](06_06_05_strings.md)

C function: `char` **\*scm\_to\_utf8\_stringn** `(SCM str, size_t *lenp)` [¶](06_06_05_strings.md)

C function: `scm_t_wchar` **\*scm\_to\_utf32\_stringn** `(SCM str, size_t *lenp)` [¶](06_06_05_strings.md)

Return a newly allocated, ISO-8859-1-, UTF-8-, or UTF-32-encoded C string from Scheme string str. An error is thrown when str cannot be converted to the specified encoding. If lenp is `NULL`, the returned C string will be null terminated, and an error will be thrown if the C string would otherwise contain null characters. If lenp is not `NULL`, the string is not null terminated, and the length of the returned string is returned in lenp. The length returned is the number of bytes for `scm_to_latin1_stringn` and `scm_to_utf8_stringn`; it is the number of elements (code points) for `scm_to_utf32_stringn`.

It is not often the case, but sometimes when you are dealing with the implementation details of a port, you need to encode and decode strings according to the encoding and conversion strategy of the port. There are some convenience functions for that purpose as well.

C Function: `SCM` **scm\_from\_port\_string** `(const char *str, SCM port)` [¶](06_06_05_strings.md)

C Function: `SCM` **scm\_from\_port\_stringn** `(const char *str, size_t len, SCM port)` [¶](06_06_05_strings.md)

C Function: `char*` **scm\_to\_port\_string** `(SCM str, SCM port)` [¶](06_06_05_strings.md)

C Function: `char*` **scm\_to\_port\_stringn** `(SCM str, size_t *lenp, SCM port)` [¶](06_06_05_strings.md)

Like `scm_from_stringn` and friends, except they take their encoding and conversion strategy from a given port object.

* * *

Previous: [Conversion to/from C](06_06_05_strings.md#66514-conversion-tofrom-c), Up: [Strings](06_06_05_strings.md#665-strings)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.5.15 String Internals [¶](06_06_05_strings.md#66515-string-internals)

Guile stores each string in memory as a contiguous array of Unicode code points along with an associated set of attributes. If all of the code points of a string have an integer range between 0 and 255 inclusive, the code point array is stored as one byte per code point: it is stored as an ISO-8859-1 (aka Latin-1) string. If any of the code points of the string has an integer value greater that 255, the code point array is stored as four bytes per code point: it is stored as a UTF-32 string.

Conversion between the one-byte-per-code-point and four-bytes-per-code-point representations happens automatically as necessary.

No API is provided to set the internal representation of strings; however, there are pair of procedures available to query it. These are debugging procedures. Using them in production code is discouraged, since the details of Guile’s internal representation of strings may change from release to release.

Scheme Procedure: **string-bytes-per-char** str [¶](06_06_05_strings.md)

C Function: **scm\_string\_bytes\_per\_char** (str) [¶](06_06_05_strings.md)

Return the number of bytes used to encode a Unicode code point in string str. The result is one or four.

Scheme Procedure: **%string-dump** str [¶](06_06_05_strings.md)

C Function: **scm\_sys\_string\_dump** (str) [¶](06_06_05_strings.md)

Returns an association list containing debugging information for str. The association list has the following entries.

`string`

The string itself.

`start`

The start index of the string into its stringbuf

`length`

The length of the string

`shared`

If this string is a substring, it returns its parent string. Otherwise, it returns `#f`

`read-only`

`#t` if the string is read-only

`stringbuf-chars`

A new string containing this string’s stringbuf’s characters

`stringbuf-length`

The number of characters in this stringbuf

`stringbuf-shared`

`#t` if this stringbuf is shared

`stringbuf-wide`

`#t` if this stringbuf’s characters are stored in a 32-bit buffer, or `#f` if they are stored in an 8-bit buffer

* * *

Next: [Keywords](06_06_07_keywords.md#667-keywords), Previous: [Strings](06_06_05_strings.md#665-strings), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
