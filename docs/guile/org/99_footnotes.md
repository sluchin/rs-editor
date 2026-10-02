#### Footnotes

##### [(1)](99_footnotes.md)

“ice-9” is a is a reference to the fictional substance in Kurt Vonnegut’s novel, Cat’s Cradle (see [Status, or: Your Help Needed](09_01_a_brief_history_of_guile.md#915-status-or-your-help-needed)).

##### [(2)](99_footnotes.md)

These definitions are approximate. For the whole and detailed truth, see [R5RS syntax](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Formal-syntax-and-semantics) in The Revised(5) Report on the Algorithmic Language Scheme.

##### [(3)](99_footnotes.md)

The `GUILE_INSTALL_LOCALE` environment variable was ignored in Guile versions prior to 2.0.9.

##### [(4)](99_footnotes.md)

The `guile` and `guild` variables defined starting from Guile version 2.0.12.

##### [(5)](99_footnotes.md)

Note that Guile does not scan the C heap for references, so a reference to a `SCM` object from a memory segment allocated with `malloc` will have to use some other means to keep the `SCM` object alive. See [Function related to Garbage Collection](06_17_memory_management_and_garbage_collection.md#6171-function-related-to-garbage-collection).

##### [(6)](99_footnotes.md)

In Guile 1.8, a thread blocking in guile mode would prevent garbage collection to occur. Thus, threads had to leave guile mode whenever they could block. This is no longer needed with Guile 2.x.

##### [(7)](99_footnotes.md)

A _white box_ test plan is one that incorporates knowledge of the internal design of the application under test.

##### [(8)](99_footnotes.md)

Of course, in the world of free software, you always have the freedom to modify the application’s source code to your own requirements. Here we are concerned with the extension options that the application has provided for without your needing to modify its source code.

##### [(9)](99_footnotes.md)

Strictly speaking, Scheme does not have a real datatype _list_. Lists are made up of _chained pairs_, and only exist by definition—a list is a chain of pairs which looks like a list.

##### [(10)](99_footnotes.md)

Note that there is no separation character between the list elements, like a comma or a semicolon.

##### [(11)](99_footnotes.md)

Big-endian and little-endian are the most common “endiannesses”, but others do exist. For instance, the GNU MP library allows _word order_ to be specified independently of _byte order_ (see [Integer Import and Export](https://www.gmplib.org/manual/Integer-Import-and-Export.html#Integer-Import-and-Export) in The GNU Multiple Precision Arithmetic Library Manual).

##### [(12)](99_footnotes.md)

R6RS only defines `(bytevector-fill! bv fill)`. Arguments start and end are a Guile extension (cf. [`vector-fill!`](99_footnotes.md), [`string-fill!`](99_footnotes.md)).

##### [(13)](99_footnotes.md)

Working definitions would be:

(define foo-ref [vector-ref](06_06_10_vectors.md))
(define foo-set! [vector-set!](06_06_10_vectors.md))
(define f ([make-vector](06_06_10_vectors.md) 2 #f))

##### [(14)](99_footnotes.md)

These days such embedded languages are often referred to as _embedded domain-specific languages_, or EDSLs.

##### [(15)](99_footnotes.md)

Language lawyers probably see the need here for use of `literal-identifier=?` rather than `free-identifier=?`, and would probably be correct. Patches accepted.

##### [(16)](99_footnotes.md)

Archived from [the original](http://sites.google.com/site/evalapply/eccentric.txt) on 2013-05-03.

##### [(17)](99_footnotes.md)

Described in the paper Keeping it Clean with Syntax Parameters by Barzilay, Culpepper and Flatt.

##### [(18)](99_footnotes.md)

In Guile up to version 1.8, C global variables were not visited by the garbage collector in the mark phase; hence, `scm_gc_protect_object` was the only way in C to prevent a Scheme object from being freed.

##### [(19)](99_footnotes.md)

In Guile up to version 1.8, memory allocated with `scm_gc_malloc` _had_ to be freed with `scm_gc_free`.

##### [(20)](99_footnotes.md)

In Guile up to 1.8, memory allocated with `scm_gc_malloc` was _not_ visited by the collector in the mark phase. Consequently, the GC had to be told explicitly about pointers to live objects contained in the memory block, e.g., _via_ SMOB mark functions (see [`scm_set_smob_mark`](06_21_smobs.md#621-smobs))

##### [(21)](99_footnotes.md)

In Guile 2.2 and earlier, _all_ the module bindings would become available; symbol-list was just the list of bindings that will first trigger the load.

##### [(22)](99_footnotes.md)

A contribution to Guile in the form of a high-level FFI would be most welcome.

##### [(23)](99_footnotes.md)

For concision and style, programmers often like to refer to internationalization as “i18n”.

##### [(24)](99_footnotes.md)

Users of `gettext` might be a bit surprised that `G_` is the conventional abbreviation for `gettext`. In most other languages, the conventional shorthand is `_`. Guile uses `G_` because `_` is already taken, as it is bound to a syntactic keyword used by `syntax-rules`, `match`, and other macros.

##### [(25)](99_footnotes.md)

This module is only available on systems where the `popen` feature is provided (see [Common Feature Symbols](06_23_configuration_features_and_runtime_options.md#62322-common-feature-symbols)).

##### [(26)](99_footnotes.md)

Yes, the P is for protocol, but this phrase appears repeatedly in RFC 2616.

##### [(27)](99_footnotes.md)

On Unicode-capable ports, the ellipsis is represented by character ‘HORIZONTAL ELLIPSIS’ (U+2026), otherwise it is represented by three dots.

##### [(28)](99_footnotes.md)

The `~h` format specifier first appeared in Guile version 2.0.6.

##### [(29)](99_footnotes.md)

This example is taken from a paper by Krishnamurthi et al. Their paper was the first to show the usefulness of the `syntax-rules` style of pattern matching for transformation of XML, though the language described, XT3D, is an XML language.

##### [(30)](99_footnotes.md)

Usually — but see also the `#:allocation` slot option.

##### [(31)](99_footnotes.md)

Of course Guile already provides complex numbers, and `<complex>` is in fact a predefined class in GOOPS; but the definition here is still useful as an example.

##### [(32)](99_footnotes.md)

`<number>` is the direct superclass of the predefined class `<complex>`; `<complex>` is the superclass of `<real>`, and `<real>` is the superclass of `<integer>`.

##### [(33)](99_footnotes.md)

But note that `x` in `(math 2D-vectors)` doesn’t share methods with `x` in `(math 3D-vectors)`, so modularity is still preserved.

##### [(34)](99_footnotes.md)

The parameter list for a `define-method` follows the conventions used for Scheme procedures. In particular it can use the dot notation or a symbol to denote an arbitrary number of parameters

##### [(35)](99_footnotes.md)

This section is an adaptation of material from Jeff Dalton’s (J.Dalton@ed.ac.uk) Brief introduction to CLOS

##### [(36)](99_footnotes.md)

PAIP is the common abbreviation for Paradigms of Artificial Intelligence Programming, an old but still useful text on Lisp. Norvig’s retrospective sums up the lessons of PAIP, and can be found at [http://norvig.com/Lisp-retro.html](http://norvig.com/Lisp-retro.html).

##### [(37)](99_footnotes.md)

Even the lowest-level machine code can be thought to be interpreted by the CPU, and indeed is often implemented by compiling machine instructions to “micro-operations”.
