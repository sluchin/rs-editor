### 7.22 Texinfo Processing [¶](07_22_texinfo_processing.md#722-texinfo-processing)

*   [(texinfo)](07_22_texinfo_processing.md#7221-texinfo)
*   [(texinfo docbook)](07_22_texinfo_processing.md#7222-texinfo-docbook)
*   [(texinfo html)](07_22_texinfo_processing.md#7223-texinfo-html)
*   [(texinfo indexing)](07_22_texinfo_processing.md#7224-texinfo-indexing)
*   [(texinfo string-utils)](07_22_texinfo_processing.md#7225-texinfo-string-utils)
*   [(texinfo plain-text)](07_22_texinfo_processing.md#7226-texinfo-plain-text)
*   [(texinfo serialize)](07_22_texinfo_processing.md#7227-texinfo-serialize)
*   [(texinfo reflection)](07_22_texinfo_processing.md#7228-texinfo-reflection)

* * *

Next: [(texinfo docbook)](07_22_texinfo_processing.md#7222-texinfo-docbook), Up: [Texinfo Processing](07_22_texinfo_processing.md#722-texinfo-processing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.22.1 (texinfo) [¶](07_22_texinfo_processing.md#7221-texinfo)

*   [Overview](07_22_texinfo_processing.md#72211-overview)
*   [Usage](07_22_texinfo_processing.md#72212-usage)

#### 7.22.1.1 Overview [¶](07_22_texinfo_processing.md#72211-overview)

#### Texinfo processing in scheme [¶](07_22_texinfo_processing.md#texinfo-processing-in-scheme)

This module parses texinfo into SXML. TeX will always be the processor of choice for print output, of course. However, although `makeinfo` works well for info, its output in other formats is not very customizable, and the program is not extensible as a whole. This module aims to provide an extensible framework for texinfo processing that integrates texinfo into the constellation of SXML processing tools.

#### Notes on the SXML vocabulary [¶](07_22_texinfo_processing.md#notes-on-the-sxml-vocabulary)

Consider the following texinfo fragment:

 @deffn Primitive set-car! pair value
 This function...
 @end deffn

Logically, the category (Primitive), name (set-car!), and arguments (pair value) are “attributes” of the deffn, with the description as the content. However, texinfo allows for @-commands within the arguments to an environment, like `@deffn`, which means that texinfo “attributes” are PCDATA. XML attributes, on the other hand, are CDATA. For this reason, “attributes” of texinfo @-commands are called “arguments”, and are grouped under the special element, ‘%’.

Because ‘%’ is not a valid NCName, stexinfo is a superset of SXML. In the interests of interoperability, this module provides a conversion function to replace the ‘%’ with ‘texinfo-arguments’.

#### 7.22.1.2 Usage [¶](07_22_texinfo_processing.md#72212-usage)

Function: **call-with-file-and-dir** filename proc [¶](07_22_texinfo_processing.md)

Call the one-argument procedure proc with an input port that reads from filename. During the dynamic extent of proc’s execution, the current directory will be `(dirname filename)`. This is useful for parsing documents that can include files by relative path name.

Variable: **texi-command-specs** [¶](07_22_texinfo_processing.md)

Function: **texi-command-depth** command max-depth [¶](07_22_texinfo_processing.md)

Given the texinfo command command, return its nesting level, or `#f` if it nests too deep for max-depth.

Examples:

 (texi-command-depth 'chapter 4)        ⇒ 1
 (texi-command-depth 'top 4)            ⇒ 0
 (texi-command-depth 'subsection 4)     ⇒ 3
 (texi-command-depth 'appendixsubsec 4) ⇒ 3
 (texi-command-depth 'subsection 2)     ⇒ #f

Function: **texi-fragment->stexi** string-or-port [¶](07_22_texinfo_processing.md)

Parse the texinfo commands in string-or-port, and return the resultant stexi tree. The head of the tree will be the special command, `*fragment*`.

Function: **texi->stexi** port [¶](07_22_texinfo_processing.md)

Read a full texinfo document from port and return the parsed stexi tree. The parsing will start at the `@settitle` and end at `@bye` or EOF.

Function: **stexi->sxml** tree [¶](07_22_texinfo_processing.md)

Transform the stexi tree tree into sxml. This involves replacing the `%` element that keeps the texinfo arguments with an element for each argument.

FIXME: right now it just changes % to `texinfo-arguments` – that doesn’t hang with the idea of making a dtd at some point

* * *

Next: [(texinfo html)](07_22_texinfo_processing.md#7223-texinfo-html), Previous: [(texinfo)](07_22_texinfo_processing.md#7221-texinfo), Up: [Texinfo Processing](07_22_texinfo_processing.md#722-texinfo-processing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.22.2 (texinfo docbook) [¶](07_22_texinfo_processing.md#7222-texinfo-docbook)

*   [Overview](07_22_texinfo_processing.md#72221-overview)
*   [Usage](07_22_texinfo_processing.md#72222-usage)

#### 7.22.2.1 Overview [¶](07_22_texinfo_processing.md#72221-overview)

This module exports procedures for transforming a limited subset of the SXML representation of docbook into stexi. It is not complete by any means. The intention is to gather a number of routines and stylesheets so that external modules can parse specific subsets of docbook, for example that set generated by certain tools.

#### 7.22.2.2 Usage [¶](07_22_texinfo_processing.md#72222-usage)

Variable: **\*sdocbook->stexi-rules\*** [¶](07_22_texinfo_processing.md)

Variable: **\*sdocbook-block-commands\*** [¶](07_22_texinfo_processing.md)

Function: **sdocbook-flatten** sdocbook [¶](07_22_texinfo_processing.md)

"Flatten" a fragment of sdocbook so that block elements do not nest inside each other.

Docbook is a nested format, where e.g. a `refsect2` normally appears inside a `refsect1`. Logical divisions in the document are represented via the tree topology; a `refsect2` element _contains_ all of the elements in its section.

On the contrary, texinfo is a flat format, in which sections are marked off by standalone section headers like `@subsection`, and block elements do not nest inside each other.

This function takes a nested sdocbook fragment sdocbook and flattens all of the sections, such that e.g.

 (refsect1 (refsect2 (para "Hello")))

becomes

 ((refsect1) (refsect2) (para "Hello"))

Oftentimes (always?) sectioning elements have `<title>` as their first element child; users interested in processing the `refsect*` elements into proper sectioning elements like `chapter` might be interested in `replace-titles` and `filter-empty-elements`. See [replace-titles](07_22_texinfo_processing.md), and [filter-empty-elements](07_22_texinfo_processing.md).

Returns a nodeset; that is to say, an untagged list of stexi elements. See [SXPath](07_21_sxml.md#7216-sxpath), for the definition of a nodeset.

Function: **filter-empty-elements** sdocbook [¶](07_22_texinfo_processing.md)

Filters out empty elements in an sdocbook nodeset. Mostly useful after running `sdocbook-flatten`.

Function: **replace-titles** sdocbook-fragment [¶](07_22_texinfo_processing.md)

Iterate over the sdocbook nodeset sdocbook-fragment, transforming contiguous `refsect` and `title` elements into the appropriate texinfo sectioning command. Most useful after having run `sdocbook-flatten`.

For example:

 (replace-titles '((refsect1) (title "Foo") (para "Bar.")))
    ⇒ '((chapter "Foo") (para "Bar."))

* * *

Next: [(texinfo indexing)](07_22_texinfo_processing.md#7224-texinfo-indexing), Previous: [(texinfo docbook)](07_22_texinfo_processing.md#7222-texinfo-docbook), Up: [Texinfo Processing](07_22_texinfo_processing.md#722-texinfo-processing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.22.3 (texinfo html) [¶](07_22_texinfo_processing.md#7223-texinfo-html)

*   [Overview](07_22_texinfo_processing.md#72231-overview)
*   [Usage](07_22_texinfo_processing.md#72232-usage)

#### 7.22.3.1 Overview [¶](07_22_texinfo_processing.md#72231-overview)

This module implements transformation from `stexi` to HTML. Note that the output of `stexi->shtml` is actually SXML with the HTML vocabulary. This means that the output can be further processed, and that it must eventually be serialized by `sxml->xml`. See [Reading and Writing XML](07_21_sxml.md#7212-reading-and-writing-xml).

References (i.e., the `@ref` family of commands) are resolved by a _ref-resolver_. See [add-ref-resolver!](07_22_texinfo_processing.md).

#### 7.22.3.2 Usage [¶](07_22_texinfo_processing.md#72232-usage)

Function: **add-ref-resolver!** proc [¶](07_22_texinfo_processing.md)

Add proc to the head of the list of ref-resolvers. proc will be expected to take the name of a node and the name of a manual and return the URL of the referent, or `#f` to pass control to the next ref-resolver in the list.

The default ref-resolver will return the concatenation of the manual name, `#`, and the node name.

Function: **stexi->shtml** tree [¶](07_22_texinfo_processing.md)

Transform the stexi tree into shtml, resolving references via ref-resolvers. See the module commentary for more details.

Function: **urlify** str [¶](07_22_texinfo_processing.md)

* * *

Next: [(texinfo string-utils)](07_22_texinfo_processing.md#7225-texinfo-string-utils), Previous: [(texinfo html)](07_22_texinfo_processing.md#7223-texinfo-html), Up: [Texinfo Processing](07_22_texinfo_processing.md#722-texinfo-processing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.22.4 (texinfo indexing) [¶](07_22_texinfo_processing.md#7224-texinfo-indexing)

*   [Overview](07_22_texinfo_processing.md#72241-overview)
*   [Usage](07_22_texinfo_processing.md#72242-usage)

#### 7.22.4.1 Overview [¶](07_22_texinfo_processing.md#72241-overview)

Given a piece of stexi, return an index of a specified variety.

Note that currently, `stexi-extract-index` doesn’t differentiate between different kinds of index entries. That’s a bug ;)

#### 7.22.4.2 Usage [¶](07_22_texinfo_processing.md#72242-usage)

Function: **stexi-extract-index** tree manual-name kind [¶](07_22_texinfo_processing.md)

Given an stexi tree tree, index all of the entries of type kind. kind can be one of the predefined texinfo indices (`concept`, `variable`, `function`, `key`, `program`, `type`) or one of the special symbols `auto` or `all`. `auto` will scan the stext for a `(printindex)` statement, and `all` will generate an index from all entries, regardless of type.

The returned index is a list of pairs, the CAR of which is the entry (a string) and the CDR of which is a node name (a string).

* * *

Next: [(texinfo plain-text)](07_22_texinfo_processing.md#7226-texinfo-plain-text), Previous: [(texinfo indexing)](07_22_texinfo_processing.md#7224-texinfo-indexing), Up: [Texinfo Processing](07_22_texinfo_processing.md#722-texinfo-processing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.22.5 (texinfo string-utils) [¶](07_22_texinfo_processing.md#7225-texinfo-string-utils)

*   [Overview](07_22_texinfo_processing.md#72251-overview)
*   [Usage](07_22_texinfo_processing.md#72252-usage)

#### 7.22.5.1 Overview [¶](07_22_texinfo_processing.md#72251-overview)

Module ‘(texinfo string-utils)’ provides various string-related functions useful to Guile’s texinfo support.

#### 7.22.5.2 Usage [¶](07_22_texinfo_processing.md#72252-usage)

Function: **escape-special-chars** str special-chars escape-char [¶](07_22_texinfo_processing.md)

Returns a copy of str with all given special characters preceded by the given escape-char.

special-chars can either be a single character, or a string consisting of all the special characters.

;; make a string regexp-safe...
 ([escape-special-chars](07_22_texinfo_processing.md) "\*\*\*(Example String)\*\*\*"  
                      "\[\]()/\*." 
                      #\\\\)
[\=>](06_08_macros.md) "\\\\\*\\\\\*\\\\\*\\\\(Example String\\\\)\\\\\*\\\\\*\\\\\*"

;; also can escape a single char...
 ([escape-special-chars](07_22_texinfo_processing.md) "richardt@vzavenue.net"
                      #\\@
                      #\\@)
[\=>](06_08_macros.md) "richardt@@vzavenue.net"

Function: **transform-string** str match? replace \[start\] \[end\] [¶](07_22_texinfo_processing.md)

Uses match? against each character in str, and performs a replacement on each character for which matches are found.

match? may either be a function, a character, a string, or `#t`. If match? is a function, then it takes a single character as input, and should return ‘#t’ for matches. match? is a character, it is compared to each string character using `char=?`. If match? is a string, then any character in that string will be considered a match. `#t` will cause every character to be a match.

If replace is a function, it is called with the matched character as an argument, and the returned value is sent to the output string via ‘display’. If replace is anything else, it is sent through the output string via ‘display’.

Note that the replacement for the matched characters does not need to be a single character. That is what differentiates this function from ‘string-map’, and what makes it useful for applications such as converting ‘#\\&’ to ‘"&amp;"’ in web page text. Some other functions in this module are just wrappers around common uses of ‘transform-string’. Transformations not possible with this function should probably be done with regular expressions.

If start and end are given, they control which portion of the string undergoes transformation. The entire input string is still output, though. So, if start is ‘5’, then the first five characters of str will still appear in the returned string.

; these two are equivalent...
 ([transform-string](07_22_texinfo_processing.md) str #\\space #\\-) ; change all spaces to -'s
 ([transform-string](07_22_texinfo_processing.md) str (lambda (c) ([char=?](06_06_03_characters.md) #\\space c)) #\\-)

Function: **expand-tabs** str \[tab-size\] [¶](07_22_texinfo_processing.md)

Returns a copy of str with all tabs expanded to spaces. tab-size defaults to 8.

Assuming tab size of 8, this is equivalent to:

 ([transform-string](07_22_texinfo_processing.md) str #\\tab "        ")

Function: **center-string** str \[width\] \[chr\] \[rchr\] [¶](07_22_texinfo_processing.md)

Returns a copy of str centered in a field of width characters. Any needed padding is done by character chr, which defaults to ‘#\\space’. If rchr is provided, then the padding to the right will use it instead. See the examples below. left and rchr on the right. The default width is 80. The default chr and rchr is ‘#\\space’. The string is never truncated.

 ([center-string](07_22_texinfo_processing.md) "Richard Todd" 24)
[\=>](06_08_macros.md) "      Richard Todd      "

 ([center-string](07_22_texinfo_processing.md) " Richard Todd " 24 #\\=)
[\=>](06_08_macros.md) "===== Richard Todd ====="

 ([center-string](07_22_texinfo_processing.md) " Richard Todd " 24 #\\< #\\>)
[\=>](06_08_macros.md) "<<<<< Richard Todd >>>>>"

Function: **left-justify-string** str \[width\] \[chr\] [¶](07_22_texinfo_processing.md)

`left-justify-string str [width chr]`. Returns a copy of str padded with chr such that it is left justified in a field of width characters. The default width is 80. Unlike ‘string-pad’ from srfi-13, the string is never truncated.

Function: **right-justify-string** str \[width\] \[chr\] [¶](07_22_texinfo_processing.md)

Returns a copy of str padded with chr such that it is right justified in a field of width characters. The default width is 80. The default chr is ‘#\\space’. Unlike ‘string-pad’ from srfi-13, the string is never truncated.

Function: **collapse-repeated-chars** str \[chr\] \[num\] [¶](07_22_texinfo_processing.md)

Returns a copy of str with all repeated instances of chr collapsed down to at most num instances. The default value for chr is ‘#\\space’, and the default value for num is 1.

 ([collapse-repeated-chars](07_22_texinfo_processing.md) "H  e  l  l  o")
[\=>](06_08_macros.md) "H e l l o"
 ([collapse-repeated-chars](07_22_texinfo_processing.md) "H--e--l--l--o" #\\-)
[\=>](06_08_macros.md) "H-e-l-l-o"
 ([collapse-repeated-chars](07_22_texinfo_processing.md) "H-e--l---l----o" #\\- 2)
[\=>](06_08_macros.md) "H-e--l--l--o"

Function: **make-text-wrapper** \[#:line-width\] \[#:expand-tabs?\] \[#:tab-width\] \[#:collapse-whitespace?\] \[#:subsequent-indent\] \[#:initial-indent\] \[#:break-long-words?\] [¶](07_22_texinfo_processing.md)

Returns a procedure that will split a string into lines according to the given parameters.

`#:line-width`

This is the target length used when deciding where to wrap lines. Default is 80.

`#:expand-tabs?`

Boolean describing whether tabs in the input should be expanded. Default is #t.

`#:tab-width`

If tabs are expanded, this will be the number of spaces to which they expand. Default is 8.

`#:collapse-whitespace?`

Boolean describing whether the whitespace inside the existing text should be removed or not. Default is #t.

If text is already well-formatted, and is just being wrapped to fit in a different width, then set this to ‘#f’. This way, many common text conventions (such as two spaces between sentences) can be preserved if in the original text. If the input text spacing cannot be trusted, then leave this setting at the default, and all repeated whitespace will be collapsed down to a single space.

`#:initial-indent`

Defines a string that will be put in front of the first line of wrapped text. Default is the empty string, “”.

`#:subsequent-indent`

Defines a string that will be put in front of all lines of wrapped text, except the first one. Default is the empty string, “”.

`#:break-long-words?`

If a single word is too big to fit on a line, this setting tells the wrapper what to do. Defaults to #t, which will break up long words. When set to #f, the line will be allowed, even though it is longer than the defined `#:line-width`.

The return value is a procedure of one argument, the input string, which returns a list of strings, where each element of the list is one line.

Function: **fill-string** str . kwargs [¶](07_22_texinfo_processing.md)

Wraps the text given in string str according to the parameters provided in kwargs, or the default setting if they are not given. Returns a single string with the wrapped text. Valid keyword arguments are discussed in `make-text-wrapper`.

Function: **string->wrapped-lines** str . kwargs [¶](07_22_texinfo_processing.md)

`string->wrapped-lines str keywds ...`. Wraps the text given in string str according to the parameters provided in keywds, or the default setting if they are not given. Returns a list of strings representing the formatted lines. Valid keyword arguments are discussed in `make-text-wrapper`.

* * *

Next: [(texinfo serialize)](07_22_texinfo_processing.md#7227-texinfo-serialize), Previous: [(texinfo string-utils)](07_22_texinfo_processing.md#7225-texinfo-string-utils), Up: [Texinfo Processing](07_22_texinfo_processing.md#722-texinfo-processing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.22.6 (texinfo plain-text) [¶](07_22_texinfo_processing.md#7226-texinfo-plain-text)

*   [Overview](07_22_texinfo_processing.md#72261-overview)
*   [Usage](07_22_texinfo_processing.md#72262-usage)

#### 7.22.6.1 Overview [¶](07_22_texinfo_processing.md#72261-overview)

Transformation from stexi to plain-text. Strives to re-create the output from `info`; comes pretty damn close.

#### 7.22.6.2 Usage [¶](07_22_texinfo_processing.md#72262-usage)

Function: **stexi->plain-text** tree [¶](07_22_texinfo_processing.md)

Transform tree into plain text. Returns a string.

Scheme Variable: **\*line-width\*** [¶](07_22_texinfo_processing.md)

This fluid (see [Fluids and Dynamic States](06_11_controlling_the_flow_of_program_execution.md#61111-fluids-and-dynamic-states)) specifies the length of line for the purposes of line wrapping in the `stexi->plain-text` conversion.

* * *

Next: [(texinfo reflection)](07_22_texinfo_processing.md#7228-texinfo-reflection), Previous: [(texinfo plain-text)](07_22_texinfo_processing.md#7226-texinfo-plain-text), Up: [Texinfo Processing](07_22_texinfo_processing.md#722-texinfo-processing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.22.7 (texinfo serialize) [¶](07_22_texinfo_processing.md#7227-texinfo-serialize)

*   [Overview](07_22_texinfo_processing.md#72271-overview)
*   [Usage](07_22_texinfo_processing.md#72272-usage)

#### 7.22.7.1 Overview [¶](07_22_texinfo_processing.md#72271-overview)

Serialization of `stexi` to plain texinfo.

#### 7.22.7.2 Usage [¶](07_22_texinfo_processing.md#72272-usage)

Function: **stexi->texi** tree [¶](07_22_texinfo_processing.md)

Serialize the stexi tree into plain texinfo.

* * *

Previous: [(texinfo serialize)](07_22_texinfo_processing.md#7227-texinfo-serialize), Up: [Texinfo Processing](07_22_texinfo_processing.md#722-texinfo-processing)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.22.8 (texinfo reflection) [¶](07_22_texinfo_processing.md#7228-texinfo-reflection)

*   [Overview](07_22_texinfo_processing.md#72281-overview)
*   [Usage](07_22_texinfo_processing.md#72282-usage)

#### 7.22.8.1 Overview [¶](07_22_texinfo_processing.md#72281-overview)

Routines to generare `stexi` documentation for objects and modules.

Note that in this context, an _object_ is just a value associated with a location. It has nothing to do with GOOPS.

#### 7.22.8.2 Usage [¶](07_22_texinfo_processing.md#72282-usage)

Function: **module-stexi-documentation** sym-name \[%docs-resolver\] \[#:docs-resolver\] [¶](07_22_texinfo_processing.md)

Return documentation for the module named sym-name. The documentation will be formatted as `stexi` (see [texinfo](07_22_texinfo_processing.md#7221-texinfo)).

Function: **script-stexi-documentation** scriptpath [¶](07_22_texinfo_processing.md)

Return documentation for given script. The documentation will be taken from the script’s commentary, and will be returned in the `stexi` format (see [texinfo](07_22_texinfo_processing.md#7221-texinfo)).

Function: **object-stexi-documentation** \_ \[\_\] \[#:force\] [¶](07_22_texinfo_processing.md)

Function: **package-stexi-standard-copying** name version updated years copyright-holder permissions [¶](07_22_texinfo_processing.md)

Create a standard texinfo `copying` section.

years is a list of years (as integers) in which the modules being documented were released. All other arguments are strings.

Function: **package-stexi-standard-titlepage** name version updated authors [¶](07_22_texinfo_processing.md)

Create a standard GNU title page.

authors is a list of `(name . email)` pairs. All other arguments are strings.

Here is an example of the usage of this procedure:

 (package-stexi-standard-titlepage
  "Foolib"
  "3.2"
  "26 September 2006"
  '(("Alyssa P Hacker" . "alyssa@example.com"))
  '(2004 2005 2006)
  "Free Software Foundation, Inc."
  "Standard GPL permissions blurb goes here")

Function: **package-stexi-generic-menu** name entries [¶](07_22_texinfo_processing.md)

Create a menu from a generic alist of entries, the car of which should be the node name, and the cdr the description. As an exception, an entry of `#f` will produce a separator.

Function: **package-stexi-standard-menu** name modules module-descriptions extra-entries [¶](07_22_texinfo_processing.md)

Create a standard top node and menu, suitable for processing by makeinfo.

Function: **package-stexi-extended-menu** name module-pairs script-pairs extra-entries [¶](07_22_texinfo_processing.md)

Create an "extended" menu, like the standard menu but with a section for scripts.

Function: **package-stexi-standard-prologue** name filename category description copying titlepage menu [¶](07_22_texinfo_processing.md)

Create a standard prologue, suitable for later serialization to texinfo and .info creation with makeinfo.

Returns a list of stexinfo forms suitable for passing to `package-stexi-documentation` as the prologue. See [texinfo reflection package-stexi-documentation](07_22_texinfo_processing.md), [package-stexi-standard-titlepage](07_22_texinfo_processing.md), [package-stexi-standard-copying](07_22_texinfo_processing.md), and [package-stexi-standard-menu](07_22_texinfo_processing.md).

Function: **package-stexi-documentation** modules name filename prologue epilogue \[#:module-stexi-documentation-args\] \[#:scripts\] [¶](07_22_texinfo_processing.md)

Create stexi documentation for a _package_, where a package is a set of modules that is released together.

modules is expected to be a list of module names, where a module name is a list of symbols. The stexi that is returned will be titled name and a texinfo filename of filename.

prologue and epilogue are lists of stexi forms that will be spliced into the output document before and after the generated modules documentation, respectively. See [texinfo reflection package-stexi-standard-prologue](07_22_texinfo_processing.md), to create a conventional GNU texinfo prologue.

module-stexi-documentation-args is an optional argument that, if given, will be added to the argument list when `module-texi-documentation` is called. For example, it might be useful to define a `#:docs-resolver` argument.

Function: **package-stexi-documentation-for-include** modules module-descriptions \[#:module-stexi-documentation-args\] [¶](07_22_texinfo_processing.md)

Create stexi documentation for a _package_, where a package is a set of modules that is released together.

modules is expected to be a list of module names, where a module name is a list of symbols. Returns an stexinfo fragment.

Unlike `package-stexi-documentation`, this function simply produces a menu and the module documentations instead of producing a full texinfo document. This can be useful if you write part of your manual by hand, and just use `@include` to pull in the automatically generated parts.

module-stexi-documentation-args is an optional argument that, if given, will be added to the argument list when `module-texi-documentation` is called. For example, it might be useful to define a `#:docs-resolver` argument.

* * *

Next: [Guile Implementation](09_00_guile_implementation.md#9-guile-implementation), Previous: [Guile Modules](07_00_guile_modules.md#7-guile-modules), Up: [The Guile Reference Manual](00_contents.md)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
