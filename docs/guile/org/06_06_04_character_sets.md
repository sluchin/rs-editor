#### 6.6.4 Character Sets [¶](06_06_04_character_sets.md#664-character-sets)

The features described in this section correspond directly to SRFI-14.

The data type _charset_ implements sets of characters (see [Characters](06_06_03_characters.md#663-characters)). Because the internal representation of character sets is not visible to the user, a lot of procedures for handling them are provided.

Character sets can be created, extended, tested for the membership of a characters and be compared to other character sets.

*   [Character Set Predicates/Comparison](06_06_04_character_sets.md#6641-character-set-predicatescomparison)
*   [Iterating Over Character Sets](06_06_04_character_sets.md#6642-iterating-over-character-sets)
*   [Creating Character Sets](06_06_04_character_sets.md#6643-creating-character-sets)
*   [Querying Character Sets](06_06_04_character_sets.md#6644-querying-character-sets)
*   [Character-Set Algebra](06_06_04_character_sets.md#6645-character-set-algebra)
*   [Standard Character Sets](06_06_04_character_sets.md#6646-standard-character-sets)

* * *

Next: [Iterating Over Character Sets](06_06_04_character_sets.md#6642-iterating-over-character-sets), Up: [Character Sets](06_06_04_character_sets.md#664-character-sets)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.4.1 Character Set Predicates/Comparison [¶](06_06_04_character_sets.md#6641-character-set-predicatescomparison)

Use these procedures for testing whether an object is a character set, or whether several character sets are equal or subsets of each other. `char-set-hash` can be used for calculating a hash value, maybe for usage in fast lookup procedures.

Scheme Procedure: **char-set?** obj [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_p** (obj) [¶](06_06_04_character_sets.md)

Return `#t` if obj is a character set, `#f` otherwise.

Scheme Procedure: **char-set=** char\_set … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_eq** (char\_sets) [¶](06_06_04_character_sets.md)

Return `#t` if all given character sets are equal.

Scheme Procedure: **char-set<=** char\_set … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_leq** (char\_sets) [¶](06_06_04_character_sets.md)

Return `#t` if every character set char\_seti is a subset of character set char\_seti+1.

Scheme Procedure: **char-set-hash** cs \[bound\] [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_hash** (cs, bound) [¶](06_06_04_character_sets.md)

Compute a hash value for the character set cs. If bound is given and non-zero, it restricts the returned value to the range 0 … bound - 1.

* * *

Next: [Creating Character Sets](06_06_04_character_sets.md#6643-creating-character-sets), Previous: [Character Set Predicates/Comparison](06_06_04_character_sets.md#6641-character-set-predicatescomparison), Up: [Character Sets](06_06_04_character_sets.md#664-character-sets)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.4.2 Iterating Over Character Sets [¶](06_06_04_character_sets.md#6642-iterating-over-character-sets)

Character set cursors are a means for iterating over the members of a character sets. After creating a character set cursor with `char-set-cursor`, a cursor can be dereferenced with `char-set-ref`, advanced to the next member with `char-set-cursor-next`. Whether a cursor has passed past the last element of the set can be checked with `end-of-char-set?`.

Additionally, mapping and (un-)folding procedures for character sets are provided.

Scheme Procedure: **char-set-cursor** cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_cursor** (cs) [¶](06_06_04_character_sets.md)

Return a cursor into the character set cs.

Scheme Procedure: **char-set-ref** cs cursor [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_ref** (cs, cursor) [¶](06_06_04_character_sets.md)

Return the character at the current cursor position cursor in the character set cs. It is an error to pass a cursor for which `end-of-char-set?` returns true.

Scheme Procedure: **char-set-cursor-next** cs cursor [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_cursor\_next** (cs, cursor) [¶](06_06_04_character_sets.md)

Advance the character set cursor cursor to the next character in the character set cs. It is an error if the cursor given satisfies `end-of-char-set?`.

Scheme Procedure: **end-of-char-set?** cursor [¶](06_06_04_character_sets.md)

C Function: **scm\_end\_of\_char\_set\_p** (cursor) [¶](06_06_04_character_sets.md)

Return `#t` if cursor has reached the end of a character set, `#f` otherwise.

Scheme Procedure: **char-set-fold** kons knil cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_fold** (kons, knil, cs) [¶](06_06_04_character_sets.md)

Fold the procedure kons over the character set cs, initializing it with knil.

Scheme Procedure: **char-set-unfold** p f g seed \[base\_cs\] [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_unfold** (p, f, g, seed, base\_cs) [¶](06_06_04_character_sets.md)

This is a fundamental constructor for character sets.

*   g is used to generate a series of “seed” values from the initial seed: seed, (g seed), (g^2 seed), (g^3 seed), …
*   p tells us when to stop – when it returns true when applied to one of the seed values.
*   f maps each seed value to a character. These characters are added to the base character set base\_cs to form the result; base\_cs defaults to the empty set.

Scheme Procedure: **char-set-unfold!** p f g seed base\_cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_unfold\_x** (p, f, g, seed, base\_cs) [¶](06_06_04_character_sets.md)

This is a fundamental constructor for character sets.

*   g is used to generate a series of “seed” values from the initial seed: seed, (g seed), (g^2 seed), (g^3 seed), …
*   p tells us when to stop – when it returns true when applied to one of the seed values.
*   f maps each seed value to a character. These characters are added to the base character set base\_cs to form the result; base\_cs defaults to the empty set.

Scheme Procedure: **char-set-for-each** proc cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_for\_each** (proc, cs) [¶](06_06_04_character_sets.md)

Apply proc to every character in the character set cs. The return value is not specified.

Scheme Procedure: **char-set-map** proc cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_map** (proc, cs) [¶](06_06_04_character_sets.md)

Map the procedure proc over every character in cs. proc must be a character -> character procedure.

* * *

Next: [Querying Character Sets](06_06_04_character_sets.md#6644-querying-character-sets), Previous: [Iterating Over Character Sets](06_06_04_character_sets.md#6642-iterating-over-character-sets), Up: [Character Sets](06_06_04_character_sets.md#664-character-sets)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.4.3 Creating Character Sets [¶](06_06_04_character_sets.md#6643-creating-character-sets)

New character sets are produced with these procedures.

Scheme Procedure: **char-set-copy** cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_copy** (cs) [¶](06_06_04_character_sets.md)

Return a newly allocated character set containing all characters in cs.

Scheme Procedure: **char-set** chr … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set** (chrs) [¶](06_06_04_character_sets.md)

Return a character set containing all given characters.

Scheme Procedure: **list->char-set** list \[base\_cs\] [¶](06_06_04_character_sets.md)

C Function: **scm\_list\_to\_char\_set** (list, base\_cs) [¶](06_06_04_character_sets.md)

Convert the character list list to a character set. If the character set base\_cs is given, the character in this set are also included in the result.

Scheme Procedure: **list->char-set!** list base\_cs [¶](06_06_04_character_sets.md)

C Function: **scm\_list\_to\_char\_set\_x** (list, base\_cs) [¶](06_06_04_character_sets.md)

Convert the character list list to a character set. The characters are added to base\_cs and base\_cs is returned.

Scheme Procedure: **string->char-set** str \[base\_cs\] [¶](06_06_04_character_sets.md)

C Function: **scm\_string\_to\_char\_set** (str, base\_cs) [¶](06_06_04_character_sets.md)

Convert the string str to a character set. If the character set base\_cs is given, the characters in this set are also included in the result.

Scheme Procedure: **string->char-set!** str base\_cs [¶](06_06_04_character_sets.md)

C Function: **scm\_string\_to\_char\_set\_x** (str, base\_cs) [¶](06_06_04_character_sets.md)

Convert the string str to a character set. The characters from the string are added to base\_cs, and base\_cs is returned.

Scheme Procedure: **char-set-filter** pred cs \[base\_cs\] [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_filter** (pred, cs, base\_cs) [¶](06_06_04_character_sets.md)

Return a character set containing every character from cs so that it satisfies pred. If provided, the characters from base\_cs are added to the result.

Scheme Procedure: **char-set-filter!** pred cs base\_cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_filter\_x** (pred, cs, base\_cs) [¶](06_06_04_character_sets.md)

Return a character set containing every character from cs so that it satisfies pred. The characters are added to base\_cs and base\_cs is returned.

Scheme Procedure: **ucs-range->char-set** lower upper \[error \[base\_cs\]\] [¶](06_06_04_character_sets.md)

C Function: **scm\_ucs\_range\_to\_char\_set** (lower, upper, error, base\_cs) [¶](06_06_04_character_sets.md)

Return a character set containing all characters whose character codes lie in the half-open range \[lower,upper).

If error is a true value, an error is signaled if the specified range contains characters which are not contained in the implemented character range. If error is `#f`, these characters are silently left out of the resulting character set.

The characters in base\_cs are added to the result, if given.

Scheme Procedure: **ucs-range->char-set!** lower upper error base\_cs [¶](06_06_04_character_sets.md)

C Function: **scm\_ucs\_range\_to\_char\_set\_x** (lower, upper, error, base\_cs) [¶](06_06_04_character_sets.md)

Return a character set containing all characters whose character codes lie in the half-open range \[lower,upper).

If error is a true value, an error is signaled if the specified range contains characters which are not contained in the implemented character range. If error is `#f`, these characters are silently left out of the resulting character set.

The characters are added to base\_cs and base\_cs is returned.

Scheme Procedure: **\->char-set** x [¶](06_06_04_character_sets.md)

C Function: **scm\_to\_char\_set** (x) [¶](06_06_04_character_sets.md)

Coerces x into a char-set. x may be a string, character or char-set. A string is converted to the set of its constituent characters; a character is converted to a singleton set; a char-set is returned as-is.

* * *

Next: [Character-Set Algebra](06_06_04_character_sets.md#6645-character-set-algebra), Previous: [Creating Character Sets](06_06_04_character_sets.md#6643-creating-character-sets), Up: [Character Sets](06_06_04_character_sets.md#664-character-sets)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.4.4 Querying Character Sets [¶](06_06_04_character_sets.md#6644-querying-character-sets)

Access the elements and other information of a character set with these procedures.

Scheme Procedure: **%char-set-dump** cs [¶](06_06_04_character_sets.md)

Returns an association list containing debugging information for cs. The association list has the following entries.

`char-set`

The char-set itself

`len`

The number of groups of contiguous code points the char-set contains

`ranges`

A list of lists where each sublist is a range of code points and their associated characters

The return value of this function cannot be relied upon to be consistent between versions of Guile and should not be used in code.

Scheme Procedure: **char-set-size** cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_size** (cs) [¶](06_06_04_character_sets.md)

Return the number of elements in character set cs.

Scheme Procedure: **char-set-count** pred cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_count** (pred, cs) [¶](06_06_04_character_sets.md)

Return the number of the elements int the character set cs which satisfy the predicate pred.

Scheme Procedure: **char-set->list** cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_to\_list** (cs) [¶](06_06_04_character_sets.md)

Return a list containing the elements of the character set cs.

Scheme Procedure: **char-set->string** cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_to\_string** (cs) [¶](06_06_04_character_sets.md)

Return a string containing the elements of the character set cs. The order in which the characters are placed in the string is not defined.

Scheme Procedure: **char-set-contains?** cs ch [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_contains\_p** (cs, ch) [¶](06_06_04_character_sets.md)

Return `#t` if the character ch is contained in the character set cs, or `#f` otherwise.

Scheme Procedure: **char-set-every** pred cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_every** (pred, cs) [¶](06_06_04_character_sets.md)

Return a true value if every character in the character set cs satisfies the predicate pred.

Scheme Procedure: **char-set-any** pred cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_any** (pred, cs) [¶](06_06_04_character_sets.md)

Return a true value if any character in the character set cs satisfies the predicate pred.

* * *

Next: [Standard Character Sets](06_06_04_character_sets.md#6646-standard-character-sets), Previous: [Querying Character Sets](06_06_04_character_sets.md#6644-querying-character-sets), Up: [Character Sets](06_06_04_character_sets.md#664-character-sets)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.4.5 Character-Set Algebra [¶](06_06_04_character_sets.md#6645-character-set-algebra)

Character sets can be manipulated with the common set algebra operation, such as union, complement, intersection etc. All of these procedures provide side-effecting variants, which modify their character set argument(s).

Scheme Procedure: **char-set-adjoin** cs chr … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_adjoin** (cs, chrs) [¶](06_06_04_character_sets.md)

Add all character arguments to the first argument, which must be a character set.

Scheme Procedure: **char-set-delete** cs chr … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_delete** (cs, chrs) [¶](06_06_04_character_sets.md)

Delete all character arguments from the first argument, which must be a character set.

Scheme Procedure: **char-set-adjoin!** cs chr … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_adjoin\_x** (cs, chrs) [¶](06_06_04_character_sets.md)

Add all character arguments to the first argument, which must be a character set.

Scheme Procedure: **char-set-delete!** cs chr … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_delete\_x** (cs, chrs) [¶](06_06_04_character_sets.md)

Delete all character arguments from the first argument, which must be a character set.

Scheme Procedure: **char-set-complement** cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_complement** (cs) [¶](06_06_04_character_sets.md)

Return the complement of the character set cs.

Note that the complement of a character set is likely to contain many reserved code points (code points that are not associated with characters). It may be helpful to modify the output of `char-set-complement` by computing its intersection with the set of designated code points, `char-set:designated`.

Scheme Procedure: **char-set-union** cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_union** (char\_sets) [¶](06_06_04_character_sets.md)

Return the union of all argument character sets.

Scheme Procedure: **char-set-intersection** cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_intersection** (char\_sets) [¶](06_06_04_character_sets.md)

Return the intersection of all argument character sets.

Scheme Procedure: **char-set-difference** cs1 cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_difference** (cs1, char\_sets) [¶](06_06_04_character_sets.md)

Return the difference of all argument character sets.

Scheme Procedure: **char-set-xor** cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_xor** (char\_sets) [¶](06_06_04_character_sets.md)

Return the exclusive-or of all argument character sets.

Scheme Procedure: **char-set-diff+intersection** cs1 cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_diff\_plus\_intersection** (cs1, char\_sets) [¶](06_06_04_character_sets.md)

Return the difference and the intersection of all argument character sets.

Scheme Procedure: **char-set-complement!** cs [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_complement\_x** (cs) [¶](06_06_04_character_sets.md)

Return the complement of the character set cs.

Scheme Procedure: **char-set-union!** cs1 cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_union\_x** (cs1, char\_sets) [¶](06_06_04_character_sets.md)

Return the union of all argument character sets.

Scheme Procedure: **char-set-intersection!** cs1 cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_intersection\_x** (cs1, char\_sets) [¶](06_06_04_character_sets.md)

Return the intersection of all argument character sets.

Scheme Procedure: **char-set-difference!** cs1 cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_difference\_x** (cs1, char\_sets) [¶](06_06_04_character_sets.md)

Return the difference of all argument character sets.

Scheme Procedure: **char-set-xor!** cs1 cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_xor\_x** (cs1, char\_sets) [¶](06_06_04_character_sets.md)

Return the exclusive-or of all argument character sets.

Scheme Procedure: **char-set-diff+intersection!** cs1 cs2 cs … [¶](06_06_04_character_sets.md)

C Function: **scm\_char\_set\_diff\_plus\_intersection\_x** (cs1, cs2, char\_sets) [¶](06_06_04_character_sets.md)

Return the difference and the intersection of all argument character sets.

* * *

Previous: [Character-Set Algebra](06_06_04_character_sets.md#6645-character-set-algebra), Up: [Character Sets](06_06_04_character_sets.md#664-character-sets)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.4.6 Standard Character Sets [¶](06_06_04_character_sets.md#6646-standard-character-sets)

In order to make the use of the character set data type and procedures useful, several predefined character set variables exist.

These character sets are locale independent and are not recomputed upon a `setlocale` call. They contain characters from the whole range of Unicode code points. For instance, `char-set:letter` contains about 100,000 characters.

Scheme Variable: **char-set:lower-case** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_lower\_case** [¶](06_06_04_character_sets.md)

All lower-case characters.

Scheme Variable: **char-set:upper-case** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_upper\_case** [¶](06_06_04_character_sets.md)

All upper-case characters.

Scheme Variable: **char-set:title-case** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_title\_case** [¶](06_06_04_character_sets.md)

All single characters that function as if they were an upper-case letter followed by a lower-case letter.

Scheme Variable: **char-set:letter** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_letter** [¶](06_06_04_character_sets.md)

All letters. This includes `char-set:lower-case`, `char-set:upper-case`, `char-set:title-case`, and many letters that have no case at all. For example, Chinese and Japanese characters typically have no concept of case.

Scheme Variable: **char-set:digit** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_digit** [¶](06_06_04_character_sets.md)

All digits.

Scheme Variable: **char-set:letter+digit** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_letter\_and\_digit** [¶](06_06_04_character_sets.md)

The union of `char-set:letter` and `char-set:digit`.

Scheme Variable: **char-set:graphic** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_graphic** [¶](06_06_04_character_sets.md)

All characters which would put ink on the paper.

Scheme Variable: **char-set:printing** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_printing** [¶](06_06_04_character_sets.md)

The union of `char-set:graphic` and `char-set:whitespace`.

Scheme Variable: **char-set:whitespace** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_whitespace** [¶](06_06_04_character_sets.md)

All whitespace characters.

Scheme Variable: **char-set:blank** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_blank** [¶](06_06_04_character_sets.md)

All horizontal whitespace characters, which notably includes `#\space` and `#\tab`.

Scheme Variable: **char-set:iso-control** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_iso\_control** [¶](06_06_04_character_sets.md)

The ISO control characters are the C0 control characters (U+0000 to U+001F), delete (U+007F), and the C1 control characters (U+0080 to U+009F).

Scheme Variable: **char-set:punctuation** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_punctuation** [¶](06_06_04_character_sets.md)

All punctuation characters, such as the characters `!"#%&'()*,-./:;?@[\\]_{}`

Scheme Variable: **char-set:symbol** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_symbol** [¶](06_06_04_character_sets.md)

All symbol characters, such as the characters ``$+<=>^`|~``.

Scheme Variable: **char-set:hex-digit** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_hex\_digit** [¶](06_06_04_character_sets.md)

The hexadecimal digits `0123456789abcdefABCDEF`.

Scheme Variable: **char-set:ascii** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_ascii** [¶](06_06_04_character_sets.md)

All ASCII characters.

Scheme Variable: **char-set:empty** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_empty** [¶](06_06_04_character_sets.md)

The empty character set.

Scheme Variable: **char-set:designated** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_designated** [¶](06_06_04_character_sets.md)

This character set contains all designated code points. This includes all the code points to which Unicode has assigned a character or other meaning.

Scheme Variable: **char-set:full** [¶](06_06_04_character_sets.md)

C Variable: **scm\_char\_set\_full** [¶](06_06_04_character_sets.md)

This character set contains all possible code points. This includes both designated and reserved code points.

* * *

Next: [Symbols](06_06_06_symbols.md#666-symbols), Previous: [Character Sets](06_06_04_character_sets.md#664-character-sets), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
