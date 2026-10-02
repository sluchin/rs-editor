#### 6.6.6 Symbols [¶](06_06_06_symbols.md#666-symbols)

Symbols in Scheme are widely used in three ways: as items of discrete data, as lookup keys for alists and hash tables, and to denote variable references.

A _symbol_ is similar to a string in that it is defined by a sequence of characters. The sequence of characters is known as the symbol’s _name_. In the usual case — that is, where the symbol’s name doesn’t include any characters that could be confused with other elements of Scheme syntax — a symbol is written in a Scheme program by writing the sequence of characters that make up the name, _without_ any quotation marks or other special syntax. For example, the symbol whose name is “multiply-by-2” is written, simply:

multiply-by-2

Notice how this differs from a _string_ with contents “multiply-by-2”, which is written with double quotation marks, like this:

"multiply-by-2"

Looking beyond how they are written, symbols are different from strings in two important respects.

The first important difference is uniqueness. If the same-looking string is read twice from two different places in a program, the result is two _different_ string objects whose contents just happen to be the same. If, on the other hand, the same-looking symbol is read twice from two different places in a program, the result is the _same_ symbol object both times.

Given two read symbols, you can use `eq?` to test whether they are the same (that is, have the same name). `eq?` is the most efficient comparison operator in Scheme, and comparing two symbols like this is as fast as comparing, for example, two numbers. Given two strings, on the other hand, you must use `equal?` or `string=?`, which are much slower comparison operators, to determine whether the strings have the same contents.

(define sym1 ([quote](07_06_r6rs_support.md) hello))
(define sym2 ([quote](07_06_r6rs_support.md) hello))
([eq?](06_09_general_utility_functions.md) sym1 sym2) ⇒ #t

(define str1 "hello")
(define str2 "hello")
([eq?](06_09_general_utility_functions.md) str1 str2) ⇒ #f
([equal?](06_09_general_utility_functions.md) str1 str2) ⇒ #t

The second important difference is that symbols, unlike strings, are not self-evaluating. This is why we need the `(quote …)`s in the example above: `(quote hello)` evaluates to the symbol named "hello" itself, whereas an unquoted `hello` is _read_ as the symbol named "hello" and evaluated as a variable reference … about which more below (see [Symbols as Denoting Variables](06_06_06_symbols.md#6663-symbols-as-denoting-variables)).

*   [Symbols as Discrete Data](06_06_06_symbols.md#6661-symbols-as-discrete-data)
*   [Symbols as Lookup Keys](06_06_06_symbols.md#6662-symbols-as-lookup-keys)
*   [Symbols as Denoting Variables](06_06_06_symbols.md#6663-symbols-as-denoting-variables)
*   [Operations Related to Symbols](06_06_06_symbols.md#6664-operations-related-to-symbols)
*   [Extended Read Syntax for Symbols](06_06_06_symbols.md#6665-extended-read-syntax-for-symbols)
*   [Uninterned Symbols](06_06_06_symbols.md#6666-uninterned-symbols)

* * *

Next: [Symbols as Lookup Keys](06_06_06_symbols.md#6662-symbols-as-lookup-keys), Up: [Symbols](06_06_06_symbols.md#666-symbols)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.6.1 Symbols as Discrete Data [¶](06_06_06_symbols.md#6661-symbols-as-discrete-data)

Numbers and symbols are similar to the extent that they both lend themselves to `eq?` comparison. But symbols are more descriptive than numbers, because a symbol’s name can be used directly to describe the concept for which that symbol stands.

For example, imagine that you need to represent some colors in a computer program. Using numbers, you would have to choose arbitrarily some mapping between numbers and colors, and then take care to use that mapping consistently:

;; 1=red, 2=green, 3=purple
(if ([eq?](06_09_general_utility_functions.md) (color-of vehicle) 1)
    [...](06_08_macros.md))

You can make the mapping more explicit and the code more readable by defining constants:

(define red 1)
(define green 2)
(define purple 3)

(if ([eq?](06_09_general_utility_functions.md) (color-of vehicle) red)
    [...](06_08_macros.md))

But the simplest and clearest approach is not to use numbers at all, but symbols whose names specify the colors that they refer to:

(if ([eq?](06_09_general_utility_functions.md) (color-of vehicle) 'red)
    [...](06_08_macros.md))

The descriptive advantages of symbols over numbers increase as the set of concepts that you want to describe grows. Suppose that a car object can have other properties as well, such as whether it has or uses:

*   automatic or manual transmission
*   leaded or unleaded fuel
*   power steering (or not).

Then a car’s combined property set could be naturally represented and manipulated as a list of symbols:

(properties-of vehicle1)
⇒
(red manual unleaded power-steering)

(if ([memq](06_06_09_lists.md) 'power-steering (properties-of vehicle1))
    ([display](06_16_reading_and_evaluating_scheme_code.md) "Unfit people can drive this vehicle.\\n")
    ([display](06_16_reading_and_evaluating_scheme_code.md) "You'll need strong arms to drive this vehicle!\\n"))
⊣
Unfit people can drive this vehicle.

Remember, the fundamental property of symbols that we are relying on here is that an occurrence of `'red` in one part of a program is an _indistinguishable_ symbol from an occurrence of `'red` in another part of a program; this means that symbols can usefully be compared using `eq?`. At the same time, symbols have naturally descriptive names. This combination of efficiency and descriptive power makes them ideal for use as discrete data.

* * *

Next: [Symbols as Denoting Variables](06_06_06_symbols.md#6663-symbols-as-denoting-variables), Previous: [Symbols as Discrete Data](06_06_06_symbols.md#6661-symbols-as-discrete-data), Up: [Symbols](06_06_06_symbols.md#666-symbols)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.6.2 Symbols as Lookup Keys [¶](06_06_06_symbols.md#6662-symbols-as-lookup-keys)

Given their efficiency and descriptive power, it is natural to use symbols as the keys in an association list or hash table.

To illustrate this, consider a more structured representation of the car properties example from the preceding subsection. Rather than mixing all the properties up together in a flat list, we could use an association list like this:

(define car1-properties '((color . red)
                          (transmission . manual)
                          (fuel . unleaded)
                          (steering . power-assisted)))

Notice how this structure is more explicit and extensible than the flat list. For example it makes clear that `manual` refers to the transmission rather than, say, the windows or the locking of the car. It also allows further properties to use the same symbols among their possible values without becoming ambiguous:

(define car1-properties '((color . red)
                          (transmission . manual)
                          (fuel . unleaded)
                          (steering . power-assisted)
                          (seat-color . red)
                          (locking . manual)))

With a representation like this, it is easy to use the efficient `assq-XXX` family of procedures (see [Association Lists](06_06_20_association_lists.md#6620-association-lists)) to extract or change individual pieces of information:

([assq-ref](06_06_20_association_lists.md) car1-properties 'fuel) ⇒ unleaded
([assq-ref](06_06_20_association_lists.md) car1-properties 'transmission) ⇒ manual

([assq-set!](06_06_20_association_lists.md) car1-properties 'seat-color 'black)
⇒
((color . red)
 (transmission . manual)
 (fuel . unleaded)
 (steering . power-assisted)
 (seat-color . black)
 (locking . manual)))

Hash tables also have keys, and exactly the same arguments apply to the use of symbols in hash tables as in association lists. The hash value that Guile uses to decide where to add a symbol-keyed entry to a hash table can be obtained by calling the `symbol-hash` procedure:

Scheme Procedure: **symbol-hash** symbol [¶](06_06_06_symbols.md)

C Function: **scm\_symbol\_hash** (symbol) [¶](06_06_06_symbols.md)

Return a hash value for symbol.

See [Hash Tables](06_06_22_hash_tables.md#6622-hash-tables) for information about hash tables in general, and for why you might choose to use a hash table rather than an association list.

* * *

Next: [Operations Related to Symbols](06_06_06_symbols.md#6664-operations-related-to-symbols), Previous: [Symbols as Lookup Keys](06_06_06_symbols.md#6662-symbols-as-lookup-keys), Up: [Symbols](06_06_06_symbols.md#666-symbols)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.6.3 Symbols as Denoting Variables [¶](06_06_06_symbols.md#6663-symbols-as-denoting-variables)

When an unquoted symbol in a Scheme program is evaluated, it is interpreted as a variable reference, and the result of the evaluation is the appropriate variable’s value.

For example, when the expression `(string-length "abcd")` is read and evaluated, the sequence of characters `string-length` is read as the symbol whose name is "string-length". This symbol is associated with a variable whose value is the procedure that implements string length calculation. Therefore evaluation of the `string-length` symbol results in that procedure.

The details of the connection between an unquoted symbol and the variable to which it refers are explained elsewhere. See [Definitions and Variable Bindings](06_10_definitions_and_variable_bindings.md#610-definitions-and-variable-bindings), for how associations between symbols and variables are created, and [Modules](06_18_modules.md#618-modules), for how those associations are affected by Guile’s module system.

* * *

Next: [Extended Read Syntax for Symbols](06_06_06_symbols.md#6665-extended-read-syntax-for-symbols), Previous: [Symbols as Denoting Variables](06_06_06_symbols.md#6663-symbols-as-denoting-variables), Up: [Symbols](06_06_06_symbols.md#666-symbols)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.6.4 Operations Related to Symbols [¶](06_06_06_symbols.md#6664-operations-related-to-symbols)

Given any Scheme value, you can determine whether it is a symbol using the `symbol?` primitive:

Scheme Procedure: **symbol?** obj [¶](06_06_06_symbols.md)

C Function: **scm\_symbol\_p** (obj) [¶](06_06_06_symbols.md)

Return `#t` if obj is a symbol, otherwise return `#f`.

C Function: `int` **scm\_is\_symbol** `(SCM val)` [¶](06_06_06_symbols.md)

Equivalent to `scm_is_true (scm_symbol_p (val))`.

Once you know that you have a symbol, you can obtain its name as a string by calling `symbol->string`. Note that Guile differs by default from R5RS on the details of `symbol->string` as regards case-sensitivity:

Scheme Procedure: **symbol->string** s [¶](06_06_06_symbols.md)

C Function: **scm\_symbol\_to\_string** (s) [¶](06_06_06_symbols.md)

Return the name of symbol s as a string. By default, Guile reads symbols case-sensitively, so the string returned will have the same case variation as the sequence of characters that caused s to be created.

If Guile is set to read symbols case-insensitively (as specified by R5RS), and s comes into being as part of a literal expression (see [Literal expressions](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Literal-expressions) in The Revised^5 Report on Scheme) or by a call to the `read` or `string-ci->symbol` procedures, Guile converts any alphabetic characters in the symbol’s name to lower case before creating the symbol object, so the string returned here will be in lower case.

If s was created by `string->symbol`, the case of characters in the string returned will be the same as that in the string that was passed to `string->symbol`, regardless of Guile’s case-sensitivity setting at the time s was created.

It is an error to apply mutation procedures like `string-set!` to strings returned by this procedure.

Most symbols are created by writing them literally in code. However it is also possible to create symbols programmatically using the following procedures:

Scheme Procedure: **symbol** char… [¶](06_06_06_symbols.md)

Return a newly allocated symbol made from the given character arguments.

(symbol #\\x #\\y #\\z) ⇒ xyz

Scheme Procedure: **list->symbol** lst [¶](06_06_06_symbols.md)

Return a newly allocated symbol made from a list of characters.

(list->symbol '(#\\a #\\b #\\c)) ⇒ abc

Scheme Procedure: **symbol-append** arg … [¶](06_06_06_symbols.md)

Return a newly allocated symbol whose characters form the concatenation of the given symbols, arg ....

(let ((h 'hello))
  (symbol-append h 'world))
⇒ helloworld

Scheme Procedure: **string->symbol** string [¶](06_06_06_symbols.md)

C Function: **scm\_string\_to\_symbol** (string) [¶](06_06_06_symbols.md)

Return the symbol whose name is string. This procedure can create symbols with names containing special characters or letters in the non-standard case, but it is usually a bad idea to create such symbols because in some implementations of Scheme they cannot be read as themselves.

Scheme Procedure: **string-ci->symbol** str [¶](06_06_06_symbols.md)

C Function: **scm\_string\_ci\_to\_symbol** (str) [¶](06_06_06_symbols.md)

Return the symbol whose name is str. If Guile is currently reading symbols case-insensitively, str is converted to lowercase before the returned symbol is looked up or created.

The following examples illustrate Guile’s detailed behavior as regards the case-sensitivity of symbols:

([read-enable](06_16_reading_and_evaluating_scheme_code.md) 'case-insensitive)   ; R5RS compliant behavior
([symbol->string](06_06_06_symbols.md) 'flying-fish)    ⇒ "flying-fish"
([symbol->string](06_06_06_symbols.md) 'Martin)         ⇒ "martin"
([symbol->string](06_06_06_symbols.md)
   ([string->symbol](06_06_06_symbols.md) "Malvina"))   ⇒ "Malvina"

([eq?](06_09_general_utility_functions.md) 'mISSISSIppi 'mississippi)  ⇒ #t
([string->symbol](06_06_06_symbols.md) "mISSISSIppi")   ⇒ mISSISSIppi
([eq?](06_09_general_utility_functions.md) 'bitBlt ([string->symbol](06_06_06_symbols.md) "bitBlt")) ⇒ #f
([eq?](06_09_general_utility_functions.md) 'LolliPop
  ([string->symbol](06_06_06_symbols.md) ([symbol->string](06_06_06_symbols.md) 'LolliPop))) ⇒ #t
([string=?](06_06_05_strings.md) "K. Harper, M.D."
  ([symbol->string](06_06_06_symbols.md)
    ([string->symbol](06_06_06_symbols.md) "K. Harper, M.D."))) ⇒ #t

([read-disable](06_16_reading_and_evaluating_scheme_code.md) 'case-insensitive)   ; Guile default behavior
([symbol->string](06_06_06_symbols.md) 'flying-fish)    ⇒ "flying-fish"
([symbol->string](06_06_06_symbols.md) 'Martin)         ⇒ "Martin"
([symbol->string](06_06_06_symbols.md)
   ([string->symbol](06_06_06_symbols.md) "Malvina"))   ⇒ "Malvina"

([eq?](06_09_general_utility_functions.md) 'mISSISSIppi 'mississippi)  ⇒ #f
([string->symbol](06_06_06_symbols.md) "mISSISSIppi")   ⇒ mISSISSIppi
([eq?](06_09_general_utility_functions.md) 'bitBlt ([string->symbol](06_06_06_symbols.md) "bitBlt")) ⇒ #t
([eq?](06_09_general_utility_functions.md) 'LolliPop
  ([string->symbol](06_06_06_symbols.md) ([symbol->string](06_06_06_symbols.md) 'LolliPop))) ⇒ #t
([string=?](06_06_05_strings.md) "K. Harper, M.D."
  ([symbol->string](06_06_06_symbols.md)
    ([string->symbol](06_06_06_symbols.md) "K. Harper, M.D."))) ⇒ #t

From C, there are lower level functions that construct a Scheme symbol from a C string in the current locale encoding.

When you want to do more from C, you should convert between symbols and strings using `scm_symbol_to_string` and `scm_string_to_symbol` and work with the strings.

C Function: `SCM` **scm\_from\_latin1\_symbol** `(const char *name)` [¶](06_06_06_symbols.md)

C Function: `SCM` **scm\_from\_utf8\_symbol** `(const char *name)` [¶](06_06_06_symbols.md)

Construct and return a Scheme symbol whose name is specified by the null-terminated C string name. These are appropriate when the C string is hard-coded in the source code.

C Function: `SCM` **scm\_from\_locale\_symbol** `(const char *name)` [¶](06_06_06_symbols.md)

C Function: `SCM` **scm\_from\_locale\_symboln** `(const char *name, size_t len)` [¶](06_06_06_symbols.md)

Construct and return a Scheme symbol whose name is specified by name. For `scm_from_locale_symbol`, name must be null terminated; for `scm_from_locale_symboln` the length of name is specified explicitly by len.

Note that these functions should _not_ be used when name is a C string constant, because there is no guarantee that the current locale will match that of the execution character set, used for string and character constants. Most modern C compilers use UTF-8 by default, so in such cases we recommend `scm_from_utf8_symbol`.

C Function: `SCM` **scm\_take\_locale\_symbol** `(char *str)` [¶](06_06_06_symbols.md)

C Function: `SCM` **scm\_take\_locale\_symboln** `(char *str, size_t len)` [¶](06_06_06_symbols.md)

Like `scm_from_locale_symbol` and `scm_from_locale_symboln`, respectively, but also frees str with `free` eventually. Thus, you can use this function when you would free str anyway immediately after creating the Scheme string. In certain cases, Guile can then use str directly as its internal representation.

The size of a symbol can also be obtained from C:

C Function: `size_t` **scm\_c\_symbol\_length** `(SCM sym)` [¶](06_06_06_symbols.md)

Return the number of characters in sym.

Finally, some applications, especially those that generate new Scheme code dynamically, need to generate symbols for use in the generated code. The `gensym` primitive meets this need:

Scheme Procedure: **gensym** \[prefix\] [¶](06_06_06_symbols.md)

C Function: **scm\_gensym** (prefix) [¶](06_06_06_symbols.md)

Create a new symbol with a name constructed from a prefix and a counter value. The string prefix can be specified as an optional argument. Default prefix is ‘ g’. The counter is increased by 1 at each call. There is no provision for resetting the counter.

The symbols generated by `gensym` are _likely_ to be unique, since their names begin with a space and it is only otherwise possible to generate such symbols if a programmer goes out of their way to do so. Uniqueness can be guaranteed by instead using uninterned symbols (see [Uninterned Symbols](06_06_06_symbols.md#6666-uninterned-symbols)), though they can’t be usefully written out and read back in.

* * *

Next: [Uninterned Symbols](06_06_06_symbols.md#6666-uninterned-symbols), Previous: [Operations Related to Symbols](06_06_06_symbols.md#6664-operations-related-to-symbols), Up: [Symbols](06_06_06_symbols.md#666-symbols)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.6.5 Extended Read Syntax for Symbols [¶](06_06_06_symbols.md#6665-extended-read-syntax-for-symbols)

The read syntax for a symbol is a sequence of letters, digits, and _extended alphabetic characters_, beginning with a character that cannot begin a number. In addition, the special cases of `+`, `-`, and `...` are read as symbols even though numbers can begin with `+`, `-` or `.`.

Extended alphabetic characters may be used within identifiers as if they were letters. The set of extended alphabetic characters is:

! $ % & \* + - . / : < = > ? @ ^ \_ ~

In addition to the standard read syntax defined above (which is taken from R5RS (see [Formal syntax](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Formal-syntax) in The Revised^5 Report on Scheme)), Guile provides an extended symbol read syntax that allows the inclusion of unusual characters such as space characters, newlines and parentheses. If (for whatever reason) you need to write a symbol containing characters not mentioned above, you can do so as follows.

*   Begin the symbol with the characters `#{`,
*   write the characters of the symbol and
*   finish the symbol with the characters `}#`.

Here are a few examples of this form of read syntax. The first symbol needs to use extended syntax because it contains a space character, the second because it contains a line break, and the last because it looks like a number.

#{foo bar}#

#{what
ever}#

#{4242}#

Although Guile provides this extended read syntax for symbols, widespread usage of it is discouraged because it is not portable and not very readable.

Alternatively, if you enable the `r7rs-symbols` read option (see see [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code)), you can write arbitrary symbols using the same notation used for strings, except delimited by vertical bars instead of double quotes.

|foo bar|
|\\x3BB; is a greek lambda|
|\\| is a vertical bar|

Note that there’s also an `r7rs-symbols` print option (see [Writing Scheme Values](06_16_reading_and_evaluating_scheme_code.md#6164-writing-scheme-values)). To enable the use of this notation, evaluate one or both of the following expressions:

(read-enable  'r7rs-symbols)
(print-enable 'r7rs-symbols)

* * *

Previous: [Extended Read Syntax for Symbols](06_06_06_symbols.md#6665-extended-read-syntax-for-symbols), Up: [Symbols](06_06_06_symbols.md#666-symbols)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.6.6 Uninterned Symbols [¶](06_06_06_symbols.md#6666-uninterned-symbols)

What makes symbols useful is that they are automatically kept unique. There are no two symbols that are distinct objects but have the same name. But of course, there is no rule without exception. In addition to the normal symbols that have been discussed up to now, you can also create special _uninterned_ symbols that behave slightly differently.

To understand what is different about them and why they might be useful, we look at how normal symbols are actually kept unique.

Whenever Guile wants to find the symbol with a specific name, for example during `read` or when executing `string->symbol`, it first looks into a table of all existing symbols to find out whether a symbol with the given name already exists. When this is the case, Guile just returns that symbol. When not, a new symbol with the name is created and entered into the table so that it can be found later.

Sometimes you might want to create a symbol that is guaranteed ‘fresh’, i.e. a symbol that did not exist previously. You might also want to somehow guarantee that no one else will ever unintentionally stumble across your symbol in the future. These properties of a symbol are often needed when generating code during macro expansion. When introducing new temporary variables, you want to guarantee that they don’t conflict with variables in other people’s code.

The simplest way to arrange for this is to create a new symbol but not enter it into the global table of all symbols. That way, no one will ever get access to your symbol by chance. Symbols that are not in the table are called _uninterned_. Of course, symbols that _are_ in the table are called _interned_.

You create new uninterned symbols with the function `make-symbol`. You can test whether a symbol is interned or not with `symbol-interned?`.

Uninterned symbols break the rule that the name of a symbol uniquely identifies the symbol object. Because of this, they can not be written out and read back in like interned symbols. Currently, Guile has no support for reading uninterned symbols. Note that the function `gensym` does not return uninterned symbols for this reason.

Scheme Procedure: **make-symbol** name [¶](06_06_06_symbols.md)

C Function: **scm\_make\_symbol** (name) [¶](06_06_06_symbols.md)

Return a new uninterned symbol with the name name. The returned symbol is guaranteed to be unique and future calls to `string->symbol` will not return it.

Scheme Procedure: **symbol-interned?** symbol [¶](06_06_06_symbols.md)

C Function: **scm\_symbol\_interned\_p** (symbol) [¶](06_06_06_symbols.md)

Return `#t` if symbol is interned, otherwise return `#f`.

For example:

(define foo-1 ([string->symbol](06_06_06_symbols.md) "foo"))
(define foo-2 ([string->symbol](06_06_06_symbols.md) "foo"))
(define foo-3 ([make-symbol](06_06_06_symbols.md) "foo"))
(define foo-4 ([make-symbol](06_06_06_symbols.md) "foo"))

([eq?](06_09_general_utility_functions.md) foo-1 foo-2)
⇒ #t
; Two interned symbols with the same name are the same object,
([eq?](06_09_general_utility_functions.md) foo-1 foo-3)
⇒ #f
; but a call to make-symbol with the same name returns a
; distinct object.
([eq?](06_09_general_utility_functions.md) foo-3 foo-4)
⇒ #f
; A call to make-symbol always returns a new object, even for
; the same name.
foo-3
⇒ #<uninterned-symbol foo 8085290>
; Uninterned symbols print differently from interned symbols,
([symbol?](06_06_06_symbols.md) foo-3)
⇒ #t
; but they are still symbols,
([symbol-interned?](06_06_06_symbols.md) foo-3)
⇒ #f
; just not interned.

* * *

Next: [Pairs](06_06_08_pairs.md#668-pairs), Previous: [Symbols](06_06_06_symbols.md#666-symbols), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
