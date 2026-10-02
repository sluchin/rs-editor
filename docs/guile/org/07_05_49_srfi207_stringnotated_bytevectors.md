#### 7.5.49 SRFI-207 String-notated bytevectors [¶](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)

[SRFI-207](http://srfi.schemers.org/srfi-207/srfi-207.html) provides a more human-friendly representation for binary-data via an ASCII text notation for see [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors). It also provides bytestring-oriented procedures for constructing bytevectors from sequences of integers, characters, strings, and other bytevectors, and procedures for manipulating bytevectors as if they were strings.

Binary file formats are usually not self-describing, and if they are, the descriptive portion is itself binary, which makes it hard for human beings to interpret. To assist with this problem, it is common to have a human-readable section at the beginning of the file, or in some cases at the beginning of each distinct section of the file. For historical reasons and to avoid text encoding complications, it is usual for this human-readable section to be expressed as ASCII text.

For example, ZIP files begin with the hex bytes `50 4B` which are the ASCII encoding for the characters "PK", the initials of Phil Katz, the inventor of ZIP format. As another example, the GIF image format begins with `47 49 46 38 39 61`, the ASCII encoding for "GIF89a", where "89a" is the format version. A third example is the PNG image format, where the file header begins `89 50 4E 47`. The first byte is intentionally non-ASCII, but the next three are "PNG". Furthermore, a PNG file is divided into chunks, each of which contains a 4-byte "chunk type" code. The letters in the chunk type are mnemonics for its purpose, such as "PLTE" for a palette, "bKGD" for a default background color, and "iTXt" for descriptive text in UTF-8.

When bytevectors contain string data of this kind, it is much more tractable for human programmers to deal with `#u8"\x89;PNG\r\n\x1A;\n"` rather than `#u8(0x89 0x50 0x4E 0x47 0x0D 0x0A 0x1A 0x0A)`.

In addition, this SRFI provides bytevectors with additional procedures that closely resemble those provided for strings. For example, bytevectors can be padded or trimmed, compared case-sensitively or case-insensitively, searched, joined, and split.

Most of the procedures of this SRFI begin with `bytestring-` in order to distinguish them from other bytevector procedures. This does not mean that they accept or return a separate bytestring type: bytestrings and bytevectors are exactly the same type.

*   [External Notation](07_05_49_srfi207_stringnotated_bytevectors.md#75491-external-notation)
*   [Constructors](07_05_49_srfi207_stringnotated_bytevectors.md#75492-constructors)
*   [Conversion](07_05_49_srfi207_stringnotated_bytevectors.md#75493-conversion)
*   [Selection](07_05_49_srfi207_stringnotated_bytevectors.md#75494-selection)
*   [Replacement](07_05_49_srfi207_stringnotated_bytevectors.md#75495-replacement)
*   [Comparison](07_05_49_srfi207_stringnotated_bytevectors.md#75496-comparison)
*   [Searching](07_05_49_srfi207_stringnotated_bytevectors.md#75497-searching)
*   [Joining And Splitting](07_05_49_srfi207_stringnotated_bytevectors.md#75498-joining-and-splitting)
*   [I/O](07_05_49_srfi207_stringnotated_bytevectors.md#75499-io)
*   [Exceptions](07_05_49_srfi207_stringnotated_bytevectors.md#754910-exceptions)
*   [Acknowledgements](07_05_49_srfi207_stringnotated_bytevectors.md#754911-acknowledgements)

* * *

Next: [Constructors](07_05_49_srfi207_stringnotated_bytevectors.md#75492-constructors), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.1 External Notation [¶](07_05_49_srfi207_stringnotated_bytevectors.md#75491-external-notation)

The basic form of a string-notated bytevector is `#u8"CONTENT"`. The Scheme reader will read them if bytestrings are enabled via `(read-enable 'bytestrings)`, and the Scheme writer will write them if they are enabled via `(print-enable 'bytestrings)`.

To avoid character encoding issues within string-notated bytevectors, only printable ASCII characters (that is, Unicode codepoints in the range from U+0020 to U+007E inclusive) are allowed to be used within the CONTENT of a string-notated bytevector. All other characters must be expressed through mnemonic or inline hex escapes, and `"` and `\` must also be escaped as in normal Scheme strings.

Within the CONTENT of a string-notated bytevector:

*   `\a` ⇒ 7
*   `\b` ⇒ 8
*   `\t` ⇒ 9
*   `\n` ⇒ 10
*   `\r` ⇒ 13
*   `\"` ⇒ 34
*   `\\` ⇒ 92
*   `\|` ⇒ 124
*   the sequence `\x` followed by zero or more `0` characters, followed by one or two hexadecimal digits, followed by `;` represents the integer specified by the hexadecimal digits;
*   the sequence `\` followed by zero or more intraline whitespace characters, followed by a newline, followed by zero or more further intraline whitespace characters, is ignored and corresponds to no entry in the resulting bytevector;
*   any other printable ASCII character represents the character number of that character in the ASCII/Unicode code chart; and
*   it is an error to use any other character or sequence beginning with `\` within a string-notated bytevector.

Note: The `\|` sequence is provided so that string parsing, symbol parsing, and string-notated bytevector parsing can all use the same sequences. However, we give a complete definition of the valid lexical syntax in this SRFI rather than inheriting the native syntax of strings, so that it is clear that `#u8"&iota;"` and `#u8"\xE000;"` are invalid.

When the Scheme reader encounters a string-notated bytevector, it produces a datum as if that bytevector had been written out in full. That is, `#u8"A"` is exactly equivalent to `#u8(65)`.

* * *

Next: [Conversion](07_05_49_srfi207_stringnotated_bytevectors.md#75493-conversion), Previous: [External Notation](07_05_49_srfi207_stringnotated_bytevectors.md#75491-external-notation), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.2 Constructors [¶](07_05_49_srfi207_stringnotated_bytevectors.md#75492-constructors)

Scheme Procedure: **bytestring** part … [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Converts earch part into a sequence of small integers and returns a bytevector of the corresponding bytes as follows:

*   If part is an exact integer in the range 0-255 inclusive, it is added to the result.
*   If part is an ASCII character (that is, its codepoint is in the range 0-127 inclusive), it is converted to its codepoint and added to the result.
*   If part is a bytevector, its elements are added to the result.
*   If part is a string of ASCII characters, it is converted to a sequence of codepoints which are added to the result.

Otherwise, an error satisfying `bytestring-error?` is signaled, for example:

(bytestring "lo" #\\r #x65 #u8(#x6d)) ⇒ #u8"lorem"

(bytestring "η" #\\space #u8(#x65 #x71 #x75 #x69 #x76))
⇒ raised &bytestring-error

Scheme Procedure: **make-bytestring** parts [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

If the parts are suitable arguments for `bytestring`, returns the bytevector that would result from applying `bytestring` to them. Otherwise, an error satisfying `bytestring-error?` is raised.

Scheme Procedure: **make-bytestring!** bytevector at parts [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

If the parts are suitable arguments for `bytestring`, writes the bytes of the bytevector that would be the result of calling `make-bytestring` into bytevector starting at index at. For example:

(define bv (make-bytevector 10 #x20))
(make-bytestring! bv 2 '(#\\s #\\c "he" #u8(#x6d #x65))) bv)
⇒ #u8" scheme "

* * *

Next: [Selection](07_05_49_srfi207_stringnotated_bytevectors.md#75494-selection), Previous: [Constructors](07_05_49_srfi207_stringnotated_bytevectors.md#75492-constructors), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.3 Conversion [¶](07_05_49_srfi207_stringnotated_bytevectors.md#75493-conversion)

Scheme Procedure: **bytevector->hex-string** bytevector [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **hex-string->bytevecto** string [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Converts between a bytevector and a string containing pairs of hexadecimal digits. If string is not pairs of hexadecimal digits, an error satisfying `bytestring-error?` is raised

(bytevector->hex-string #u8"Ford") ⇒ "467f7264"
(hex-string->bytevector "5a6170686f64") ⇒ #u8"Zaphod")

Scheme Procedure: **bytevector->base64** bytevector \[digits\] [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **base64->bytevecto** string \[digits\] [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Converts between a bytevector and its base-64 encoding as a string. The 64 digits are represented by the characters 0-9, A-Z, a-z, and the symbols + and /. However, there are different variants of base-64 encoding which use different representations of the 62nd and 63rd digit. If the optional argument digits (a two-character string) is provided, those two characters will be used as the 62nd and 63rd digit instead. Details can be found in [RFC 4648](https://tools.ietf.org/html/rfc4648).

If string is not in base-64 format, an error satisfying `bytestring-error?` is raised. However, characters that satisfy `char-whitespace?` are silently ignored.

(bytevector->base64 #u8(1 2 3 4 5 6)) ⇒ ⇒ "AQIDBAUG"
(bytevector->base64 #u8"Arthur Dent") ⇒ "QXJ0aHVyIERlbnQ="
(base64->bytevector "+/     /+") ⇒ #u8(#xfb #xff #xfe)

Scheme Procedure: **bytestring->list** bytevector \[start \[end\]\] [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Converts all or part of a bytevector into a list of the same length containing characters for elements in the range 32 to 127 and exact integers for all other elements.</p>

(bytestring->list #u8(#x41 #x42 1 2) 1 3) ⇒ (#\\B 1)

Scheme Procedure: **make-bytestring-generator** arg … [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Returns a generator that when invoked will return consecutive bytes of the bytevector that `bytestring` would create when applied to args, but without creating any bytevectors. The args are validated before any bytes are generated; if they are ill-formed, an error satisfying `bytestring-error?` is raised.

(generator->list (make-bytestring-generator "lorem"))
⇒ (#x6c #x6f #x72 #x65 #x6d)

* * *

Next: [Replacement](07_05_49_srfi207_stringnotated_bytevectors.md#75495-replacement), Previous: [Conversion](07_05_49_srfi207_stringnotated_bytevectors.md#75493-conversion), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.4 Selection [¶](07_05_49_srfi207_stringnotated_bytevectors.md#75494-selection)

Scheme Procedure: **bytestring-pad** bytevector len char-or-u8 [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **bytestring-pad-right** bytevector len char-or-u8 [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Returns a newly allocated bytevector with the contents of bytevector plus sufficient additional bytes at the beginning/end containing char-or-u8 (which can be either an ASCII character or an exact integer in the range 0-255) such that the length of the result is at least len.

(bytestring-pad #u8"Zaphod" 10 #\\\_) ⇒ #u8"\_\_\_\_Zaphod"
(bytestring-pad-right #u8(#x80 #x7f) 8 0) ⇒ #u8(#x80 #x7f 0 0 0 0 0 0)

Scheme Procedure: **bytestring-trim** bytevector pred [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **bytestring-trim-right** bytevector pred [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **bytestring-trim-both** bytevector pred [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Returns a newly allocated bytevector with the contents of bytevector, except that consecutive bytes at the beginning / the end / both the beginning and the end that satisfy pred are not included.

(bytestring-trim #u8"   Trillian" (lambda (b) (= b #x20)))
⇒ #u8"Trillian"
(bytestring-trim-both #u8(0 0 #x80 #x7f 0 0 0) zero?) ⇒ #u8(#x80 #x7f)

* * *

Next: [Comparison](07_05_49_srfi207_stringnotated_bytevectors.md#75496-comparison), Previous: [Selection](07_05_49_srfi207_stringnotated_bytevectors.md#75494-selection), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.5 Replacement [¶](07_05_49_srfi207_stringnotated_bytevectors.md#75495-replacement)

Scheme Procedure: **bytestring-replace** bytevector1 bytevector2 start1 end1 \[start2 end2\] [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Returns a newly allocated bytevector with the contents of bytevector1, except that the bytes indexed by start1 and end1 are not included but are replaced by the bytes of bytevector2 indexed by start2 and end2.

(bytestring-replace #u8"Vogon torture" #u8"poetry" 6 13)
⇒ #u8"Vogon poetry"

* * *

Next: [Searching](07_05_49_srfi207_stringnotated_bytevectors.md#75497-searching), Previous: [Replacement](07_05_49_srfi207_stringnotated_bytevectors.md#75495-replacement), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.6 Comparison [¶](07_05_49_srfi207_stringnotated_bytevectors.md#75496-comparison)

To compare bytevectors for equality, use the `bytevector=?` procedure from `(rnrs bytevectors)` (see [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)) or `equal?`.

Scheme Procedure: **bytestring<?** bytevector1 bytevector2 [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **bytestring>?** bytevector1 bytevector2 [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **bytestring<=?** bytevector1 bytevector2 [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **bytestring>=?** bytevector1 bytevector2 [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Returns `#t` if bytevector1 is less than / greater than / less than or equal to / greater than or equal to bytevector2. Comparisons are lexicographical: shorter bytevectors compare before longer ones, all elements being equal.

(bytestring<? #u8"Heart Of Gold" #u8"Heart of Gold") ⇒ #t
(bytestring<=? #u8(#x81 #x95) #u8(#x80 #xa0)) ⇒ #f
(bytestring>? #u8(1 2 3) #u8(1 2)) ⇒ #t

* * *

Next: [Joining And Splitting](07_05_49_srfi207_stringnotated_bytevectors.md#75498-joining-and-splitting), Previous: [Comparison](07_05_49_srfi207_stringnotated_bytevectors.md#75496-comparison), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.7 Searching [¶](07_05_49_srfi207_stringnotated_bytevectors.md#75497-searching)

Scheme Procedure: **bytestring-index** bytevector pred \[start \[end\]\] [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **bytestring-index-right** bytevector pred \[start \[end\]\] [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Searches bytevector from start to end / from end to start for the first byte that satisfies pred, and returns the index into bytevector containing that byte. In either direction, start is inclusive and end is exclusive. If there are no such bytes, returns `#f`.

(bytestring-index #u8(#x65 #x72 #x83 #x6f) (λ (b) (> b #x7f))) ⇒ 2
(bytestring-index #u8"Beeblebrox" (λ (b) (> b #x7f))) ⇒ #f
(bytestring-index-right #u8"Zaphod" odd?) ⇒ 4

Scheme Procedure: **bytestring-break** bytevector pred [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Scheme Procedure: **bytestring-span** bytevector pred [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Returns two values, a bytevector containing the maximal sequence of characters (searching from the beginning of bytevector to the end) that do not satisfy / do satisfy pred, and another bytevector containing the remaining characters.

(bytestring-break #u8(#x50 #x4b 0 0 #x1 #x5) zero?)
  ⇒ #u8(#x50 #x4b) #u8(0 0 #x1 #x5)
(bytestring-span #u8"ABCDefg" (lambda (b) (and (> b 40) (< b 91))))
  ⇒ #u8"ABCD" #u8"efg"

* * *

Next: [I/O](07_05_49_srfi207_stringnotated_bytevectors.md#75499-io), Previous: [Searching](07_05_49_srfi207_stringnotated_bytevectors.md#75497-searching), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.8 Joining And Splitting [¶](07_05_49_srfi207_stringnotated_bytevectors.md#75498-joining-and-splitting)

Scheme Procedure: **bytestring-join** bytevector-list delimiter [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Joins the bytevectors in bytevector-list together using the delimiter, which can be anything suitable as an argument to `bytestring`. The grammar argument is a symbol that determines how the delimiter is used, and defaults to `infix`. It is an error for grammar to be any symbol other than these four:

`infix`

means an infix or separator grammar: inserts the delimiter between list elements. An empty list will produce an empty bytevector

`strict-infix`

means the same as `infix` if the list is non-empty, but will signal an error satisfying `bytestring-error?` if given an empty list.

`suffix`

means a suffix or terminator grammar: inserts the delimiter after every list element.

`prefix`

means a prefix grammar: inserts the delimiter before every list element.

For example:

(bytestring-join '(#u8"Heart" #u8"of" #u8"Gold") #x20)
  ⇒ #u8"Heart of Gold"
(bytestring-join '(#u8(#xef #xbb) #u8(#xbf)) 0 'prefix)
  ⇒ #u8(0 #xef #xbb 0 #xbf)
(bytestring-join '() 0 'strict-infix)
  ⇒ ⇒ raised &bytestring-error

Scheme Procedure: **bytestring-split** bytevector delimiter \[grammar\] [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Divides the elements of bytevector and returns a list of newly allocated bytevectors using the delimiter (an ASCII character or exact integer in the range 0-255 inclusive). Delimiter bytes are not included in the result bytevectors.

The grammar argument is used to control how bytevector is divided. It has the same default and meaning as in `bytestring-join`, except that `infix` and `strict-infix` mean the same thing. That is, if grammar is `prefix` or `suffix`, then ignore any delimiter in the first or last position of bytevector respectively.

(bytestring-split #u8"Beeblebrox" #x62)
  ⇒ (#u8"Bee" #u8"le" #u8"rox")
(bytestring-split #u8(1 0 2 0) 0 'suffix)
  ⇒ (#u8(1) #u8(2))

* * *

Next: [Exceptions](07_05_49_srfi207_stringnotated_bytevectors.md#754910-exceptions), Previous: [Joining And Splitting](07_05_49_srfi207_stringnotated_bytevectors.md#75498-joining-and-splitting), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.9 I/O [¶](07_05_49_srfi207_stringnotated_bytevectors.md#75499-io)

Scheme Procedure: **read-textual-bytestring** prefix \[port\] [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Reads a string in the external format described in this SRFI from port and return it as a bytevector. If the prefix argument is false, this procedure assumes that "`#u8`" has already been read from port. If port is omitted, it defaults to the value of `(current-input-port)`. If the characters read are not in the external format, an error satisfying `bytestring-error?` is raised.

(call-with-port
    (open-input-string "#u8\\"AB\\\\xad;\\\\xf0;\\\\x0d;CD\\"")
  (lambda (port) (read-textual-bytestring #t port)))
⇒ #u8(#x41 #x42 #xad #xf0 #x0d #x43 #x44)

Scheme Procedure: **write-textual-bytestring** bytevector \[port\] [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Writes bytevector in the external format described in this SRFI to port. Bytes representing non-graphical ASCII characters are unencoded: all other bytes are encoded with a single letter if possible, otherwise with a `\x` escape. If port is omitted, it defaults to the value of `(current-output-port)`.

(call-with-port
    (open-output-string)
  (lambda (port)
    (write-textual-bytestring
     #u8(#x9 #x41 #x72 #x74 #x68 #x75 #x72 #xa)
     port)
    (get-output-string port)))
⇒ "#u8\\"\\\\tArthur\\\\n\\""

Scheme Procedure: **write-binary-bytestring** port arg … [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Outputs each arg to the binary output port port using the same interpretations as `bytestring`, but without creating any bytevectors. The args are validated before any bytes are written to port; if they are ill-formed, an error satisfying `bytestring-error?` is raised.

(call-with-port
    (open-output-bytevector)
  (lambda (port)
    (write-binary-bytestring port #\\Z #x61 #x70 "hod")
    (get-output-bytevector port)))
⇒ #u8"Zaphod"

* * *

Next: [Acknowledgements](07_05_49_srfi207_stringnotated_bytevectors.md#754911-acknowledgements), Previous: [I/O](07_05_49_srfi207_stringnotated_bytevectors.md#75499-io), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.10 Exceptions [¶](07_05_49_srfi207_stringnotated_bytevectors.md#754910-exceptions)

Scheme Procedure: **bytestring-error?** obj [¶](07_05_49_srfi207_stringnotated_bytevectors.md)

Returns `#t` if obj is a `&bytestring-error` signaled by any of the following procedures, in the circumstances they describe:

*   `bytestring`
*   `hex-string->bytestring`
*   `base64->bytestring`
*   `make-bytestring`
*   `make-bytestring!`
*   `bytestring-join`
*   `read-textual-bytestring`
*   `write-binary-bytestring`
*   `make-bytestring-generator`

* * *

Previous: [Exceptions](07_05_49_srfi207_stringnotated_bytevectors.md#754910-exceptions), Up: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.49.11 Acknowledgements [¶](07_05_49_srfi207_stringnotated_bytevectors.md#754911-acknowledgements)

Daphne Preston-Kendal devised the string notation for bytevectors; John Cowan, the procedure library; Wolfgang Corcoran-Mathe, the original, sample implementation of the procedures.

The notation is inspired by the notation used in Python since version 2.6 for `bytes` objects, which are fundamentally similar in purpose to Scheme bytevectors, especially in R7RS. In addition, many of the procedures are closely analogous to those of [SRFI 152](https://srfi.schemers.org/srfi-152/srfi-152.html).

Thanks is also due to the participants in the SRFI mailing list. In particular: Lassi Kortela corrected an embarrassing technical error; Marc Nieper-Wißkirchen explained why the `write` procedure ought not to be allowed to use this notation by default.

* * *

Previous: [SRFI-207 String-notated bytevectors](07_05_49_srfi207_stringnotated_bytevectors.md#7549-srfi-207-string-notated-bytevectors), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
