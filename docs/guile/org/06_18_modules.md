### 6.18 Modules [¶](06_18_modules.md#618-modules)

When programs become large, naming conflicts can occur when a function or global variable defined in one file has the same name as a function or global variable in another file. Even just a _similarity_ between function names can cause hard-to-find bugs, since a programmer might type the wrong function name.

The approach used to tackle this problem is called _information encapsulation_, which consists of packaging functional units into a given name space that is clearly separated from other name spaces.

The language features that allow this are usually called _the module system_ because programs are broken up into modules that are compiled separately (or loaded separately in an interpreter).

Older languages, like C, have limited support for name space manipulation and protection. In C a variable or function is public by default, and can be made local to a module with the `static` keyword. But you cannot reference public variables and functions from another module with different names.

More advanced module systems have become a common feature in recently designed languages: ML, Python, Perl, and Modula 3 all allow the _renaming_ of objects from a foreign module, so they will not clutter the global name space.

In addition, Guile offers variables as first-class objects. They can be used for interacting with the module system.

*   [General Information about Modules](06_18_modules.md#6181-general-information-about-modules)
*   [Using Guile Modules](06_18_modules.md#6182-using-guile-modules)
*   [Creating Guile Modules](06_18_modules.md#6183-creating-guile-modules)
*   [Modules and the File System](06_18_modules.md#6184-modules-and-the-file-system)
*   [R6RS Version References](06_18_modules.md#6185-r6rs-version-references)
*   [R6RS Libraries](06_18_modules.md#6186-r6rs-libraries)
*   [Variables](06_18_modules.md#6187-variables)
*   [Module System Reflection](06_18_modules.md#6188-module-system-reflection)
*   [Declarative Modules](06_18_modules.md#6189-declarative-modules)
*   [Accessing Modules from C](06_18_modules.md#61810-accessing-modules-from-c)
*   [provide and require](06_18_modules.md#61811-provide-and-require)
*   [Environments](06_18_modules.md#61812-environments)

* * *

Next: [Using Guile Modules](06_18_modules.md#6182-using-guile-modules), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.1 General Information about Modules [¶](06_18_modules.md#6181-general-information-about-modules)

A Guile module can be thought of as a collection of named procedures, variables and macros. More precisely, it is a set of _bindings_ of symbols (names) to Scheme objects.

Within a module, all bindings are visible. Certain bindings can be declared _public_, in which case they are added to the module’s so-called _export list_; this set of public bindings is called the module’s _public interface_ (see [Creating Guile Modules](06_18_modules.md#6183-creating-guile-modules)).

A client module _uses_ a providing module’s bindings by either accessing the providing module’s public interface, or by building a custom interface (and then accessing that). In a custom interface, the client module can _select_ which bindings to access and can also algorithmically _rename_ bindings. In contrast, when using the providing module’s public interface, the entire export list is available without renaming (see [Using Guile Modules](06_18_modules.md#6182-using-guile-modules)).

All Guile modules have a unique _module name_, for example `(ice-9 popen)` or `(srfi srfi-11)`. Module names are lists of one or more symbols.

When Guile goes to use an interface from a module, for example `(ice-9 popen)`, Guile first looks to see if it has loaded `(ice-9 popen)` for any reason. If the module has not been loaded yet, Guile searches a _load path_ for a file that might define it, and loads that file.

The following subsections go into more detail on using, creating, installing, and otherwise manipulating modules and the module system.

* * *

Next: [Creating Guile Modules](06_18_modules.md#6183-creating-guile-modules), Previous: [General Information about Modules](06_18_modules.md#6181-general-information-about-modules), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.2 Using Guile Modules [¶](06_18_modules.md#6182-using-guile-modules)

To use a Guile module is to access either its public interface or a custom interface (see [General Information about Modules](06_18_modules.md#6181-general-information-about-modules)). Both types of access are handled by the syntactic form `use-modules`, which accepts one or more interface specifications and, upon evaluation, arranges for those interfaces to be available to the current module. This process may include locating and loading code for a given module if that code has not yet been loaded, following `%load-path` (see [Modules and the File System](06_18_modules.md#6184-modules-and-the-file-system)).

An _interface specification_ has one of two forms. The first variation is simply to name the module, in which case its public interface is the one accessed. For example:

([use-modules](06_18_modules.md) (ice-9 popen))

Here, the interface specification is `(ice-9 popen)`, and the result is that the current module now has access to `open-pipe`, `close-pipe`, `open-input-pipe`, and so on (see [Pipes](07_02_10_pipes.md#7210-pipes)).

Note in the previous example that if the current module had already defined `open-pipe`, that definition would be overwritten by the definition in `(ice-9 popen)`. For this reason (and others), there is a second variation of interface specification that not only names a module to be accessed, but also selects bindings from it and renames them to suit the current module’s needs. For example:

([use-modules](06_18_modules.md) ((ice-9 popen)
              #:select (([open-pipe](07_02_10_pipes.md) . pipe-open) [close-pipe](07_02_10_pipes.md))
              #:renamer ([symbol-prefix-proc](06_18_modules.md) 'unixy:)))

or more simply:

([use-modules](06_18_modules.md) ((ice-9 popen)
              #:select (([open-pipe](07_02_10_pipes.md) . pipe-open) [close-pipe](07_02_10_pipes.md))
              #:prefix unixy:))

Here, the interface specification is more complex than before, and the result is that a custom interface with only two bindings is created and subsequently accessed by the current module. The mapping of old to new names is as follows:

(ice-9 popen) sees:             current module sees:
open-pipe                       unixy:pipe-open
close-pipe                      unixy:close-pipe

This example also shows how to use the convenience procedure `symbol-prefix-proc`.

You can also directly refer to bindings in a module by using the `@` syntax. For example, instead of using the `use-modules` statement from above and writing `unixy:pipe-open` to refer to the `pipe-open` from the `(ice-9 popen)`, you could also write `(@ (ice-9 popen) open-pipe)`. Thus an alternative to the complete `use-modules` statement would be

(define unixy:pipe-open ([@](06_18_modules.md) (ice-9 popen) [open-pipe](07_02_10_pipes.md)))
(define unixy:close-pipe ([@](06_18_modules.md) (ice-9 popen) [close-pipe](07_02_10_pipes.md)))

There is also `@@`, which can be used like `@`, but does not check whether the variable that is being accessed is actually exported. Thus, `@@` can be thought of as the impolite version of `@` and should only be used as a last resort or for debugging, for example.

Note that just as with a `use-modules` statement, any module that has not yet been loaded will be loaded when referenced by a `@` or `@@` form.

You can also use the `@` and `@@` syntaxes as the target of a `set!` when the binding refers to a variable.

Scheme Procedure: **symbol-prefix-proc** prefix-sym [¶](06_18_modules.md)

Return a procedure that prefixes its arg (a symbol) with prefix-sym.

syntax: **use-modules** spec … [¶](06_18_modules.md)

Resolve each interface specification spec into an interface and arrange for these to be accessible by the current module. The return value is unspecified.

spec can be a list of symbols, in which case it names a module whose public interface is found and used.

spec can also be of the form:

 (MODULE-NAME \[#:select SELECTION\]
              \[#:hide HIDE\]
              \[#:prefix PREFIX\]
              \[#:renamer RENAMER\])

in which case a custom interface is newly created and used. module-name is a list of symbols, as above; selection is a list of selection-specs; hide is a list of bindings which should not be imported; prefix is a symbol that is prepended to imported names; and renamer is a procedure that takes a symbol and returns its new name. A selection-spec is either a symbol or a pair of symbols `(ORIG . SEEN)`, where orig is the name in the used module and seen is the name in the using module. Note that seen is also modified by prefix and renamer.

The `#:select`, `#:hide`, `#:prefix`, and `#:renamer` clauses are optional. If all are omitted, this form behaves identically to the previous one. If the `#:select` clause is omitted, prefix and renamer operate on the used module’s public interface.

The `#:hide` operates on list of bindings in the module being imported, before any renaming is performed. If both `#:select` and `#:hide` contain a binding, the `#:hide` wins.

In addition to the above, spec can also include a `#:version` clause, of the form:

 #:version VERSION-SPEC

where version-spec is an R6RS-compatible version reference. An error will be signaled in the case in which a module with the same name has already been loaded, if that module specifies a version and that version is not compatible with version-spec. See [R6RS Version References](06_18_modules.md#6185-r6rs-version-references), for more on version references.

If the module name is not resolvable, `use-modules` will signal an error.

See also `scm_c_use_module` for the C API.

syntax: **@** module-name binding-name [¶](06_18_modules.md)

Refer to the binding named binding-name in module module-name. The binding must have been exported by the module.

syntax: **@@** module-name binding-name [¶](06_18_modules.md)

Refer to the binding named binding-name in module module-name. The binding must not have been exported by the module. This syntax is only intended for debugging purposes or as a last resort. See [Declarative Modules](06_18_modules.md#6189-declarative-modules), for some limitations on the use of `@@`.

* * *

Next: [Modules and the File System](06_18_modules.md#6184-modules-and-the-file-system), Previous: [Using Guile Modules](06_18_modules.md#6182-using-guile-modules), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.3 Creating Guile Modules [¶](06_18_modules.md#6183-creating-guile-modules)

When you want to create your own modules, you have to take the following steps:

*   Create a Scheme source file and add all variables and procedures you wish to export, or which are required by the exported procedures.
*   Add a `define-module` form at the beginning.
*   Export all bindings which should be in the public interface, either by using `define-public` or `export` (both documented below).

syntax: **define-module** module-name option … [¶](06_18_modules.md)

module-name is a list of one or more symbols.

(define-module (ice-9 popen))

`define-module` makes this module available to Guile programs under the given module-name.

option … are keyword/value pairs which specify more about the defined module. The recognized options and their meaning are shown in the following table.

`#:use-module interface-specification`

Equivalent to a `(use-modules interface-specification)` (see [Using Guile Modules](06_18_modules.md#6182-using-guile-modules)).

`#:autoload module symbol-list` [¶](06_18_modules.md)

Load module when any of symbol-list are accessed. For example,

(define-module (my mod)
  #:autoload (srfi srfi-1) (partition delete-duplicates))
...
(when something
  (set! foo (delete-duplicates ...)))

When a module is autoloaded, only the bindings in symbol-list become available[21](99_footnotes.md).

An autoload is a good way to put off loading a big module until it’s really needed, for instance for faster startup or if it will only be needed in certain circumstances.

`#:export list` [¶](06_18_modules.md)

Export all identifiers in list which must be a list of symbols or pairs of symbols. This is equivalent to `(export list)` in the module body.

`#:re-export list` [¶](06_18_modules.md)

Re-export all identifiers in list which must be a list of symbols or pairs of symbols. The symbols in list must be imported by the current module from other modules. This is equivalent to `re-export` below.

`#:replace list` [¶](06_18_modules.md)

Export all identifiers in list (a list of symbols or pairs of symbols) and mark them as _replacing bindings_. In the module user’s name space, this will have the effect of replacing any binding with the same name that is not also “replacing”. Normally a replacement results in an “override” warning message, `#:replace` avoids that.

In general, a module that exports a binding for which the `(guile)` module already has a definition should use `#:replace` instead of `#:export`. `#:replace`, in a sense, lets Guile know that the module _purposefully_ replaces a core binding. It is important to note, however, that this binding replacement is confined to the name space of the module user. In other words, the value of the core binding in question remains unchanged for other modules.

Note that although it is often a good idea for the replaced binding to remain compatible with a binding in `(guile)`, to avoid surprising the user, sometimes the bindings will be incompatible. For example, SRFI-19 exports its own version of `current-time` (see [SRFI-19 Time](07_05_16_srfi19_timedate_library.md#75162-srfi-19-time)) which is not compatible with the core `current-time` function (see [Time](07_02_05_time.md#725-time)). Guile assumes that a user importing a module knows what she is doing, and uses `#:replace` for this binding rather than `#:export`.

A `#:replace` clause is equivalent to `(export! list)` in the module body.

The `#:duplicates` (see below) provides fine-grain control about duplicate binding handling on the module-user side.

`#:re-export-and-replace list` [¶](06_18_modules.md)

Like `#:re-export`, but also marking the bindings as replacements in the sense of `#:replace`.

`#:version list` [¶](06_18_modules.md)

Specify a version for the module in the form of list, a list of zero or more exact, non-negative integers. The corresponding `#:version` option in the `use-modules` form allows callers to restrict the value of this option in various ways.

`#:duplicates list` [¶](06_18_modules.md)

Tell Guile to handle duplicate bindings for the bindings imported by the current module according to the policy defined by list, a list of symbols. list must contain symbols representing a duplicate binding handling policy chosen among the following:

`check`

Raises an error when a binding is imported from more than one place.

`warn`

Issue a warning when a binding is imported from more than one place and leave the responsibility of actually handling the duplication to the next duplicate binding handler.

`replace`

When a new binding is imported that has the same name as a previously imported binding, then do the following:

1.  If the old binding was said to be _replacing_ (via the `#:replace` option above) and the new binding is not replacing, the keep the old binding.
2.  If the old binding was not said to be replacing and the new binding is replacing, then replace the old binding with the new one.
3.  If neither the old nor the new binding is replacing, then keep the old one.

`warn-override-core`

Issue a warning when a core binding is being overwritten and actually override the core binding with the new one.

`first`

In case of duplicate bindings, the firstly imported binding is always the one which is kept.

`last`

In case of duplicate bindings, the lastly imported binding is always the one which is kept.

`noop`

In case of duplicate bindings, leave the responsibility to the next duplicate handler.

If list contains more than one symbol, then the duplicate binding handlers which appear first will be used first when resolving a duplicate binding situation. As mentioned above, some resolution policies may explicitly leave the responsibility of handling the duplication to the next handler in list.

If GOOPS has been loaded before the `#:duplicates` clause is processed, there are additional strategies available for dealing with generic functions. See [Merging Generics](08_06_methods_and_generic_functions.md#863-merging-generics), for more information.

The default duplicate binding resolution policy is given by the `default-duplicate-binding-handler` procedure, and is

(replace warn-override-core warn [last](07_05_03_srfi1_list_library.md))

`#:pure` [¶](06_18_modules.md)

Create a _pure_ module, that is a module which does not contain any of the standard procedure bindings except for the syntax forms. This is useful if you want to create _safe_ modules, that is modules which do not know anything about dangerous procedures.

syntax: **export** variable … [¶](06_18_modules.md)

Add all variables (which must be symbols or pairs of symbols) to the list of exported bindings of the current module. If variable is a pair, its `car` gives the name of the variable as seen by the current module and its `cdr` specifies a name for the binding in the current module’s public interface.

syntax: **define-public** … [¶](06_18_modules.md)

Equivalent to `(begin (define foo ...) (export foo))`.

syntax: **re-export** variable … [¶](06_18_modules.md)

Add all variables (which must be symbols or pairs of symbols) to the list of re-exported bindings of the current module. Pairs of symbols are handled as in `export`. Re-exported bindings must be imported by the current module from some other module.

syntax: **export!** variable … [¶](06_18_modules.md)

Like `export`, but marking the exported variables as replacing. Using a module with replacing bindings will cause any existing bindings to be replaced without issuing any warnings. See the discussion of `#:replace` above.

* * *

Next: [R6RS Version References](06_18_modules.md#6185-r6rs-version-references), Previous: [Creating Guile Modules](06_18_modules.md#6183-creating-guile-modules), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.4 Modules and the File System [¶](06_18_modules.md#6184-modules-and-the-file-system)

Typical programs only use a small subset of modules installed on a Guile system. In order to keep startup time down, Guile only loads modules when a program uses them, on demand.

When a program evaluates `(use-modules (ice-9 popen))`, and the module is not loaded, Guile searches for a conventionally-named file in the _load path_.

In this case, loading `(ice-9 popen)` will eventually cause Guile to run `(primitive-load-path "ice-9/popen")`. `primitive-load-path` will search for a file ice-9/popen in the `%load-path` (see [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths)). For each directory in `%load-path`, Guile will try to find the file name, concatenated with the extensions from `%load-extensions`. By default, this will cause Guile to `stat` ice-9/popen.scm, and then ice-9/popen. See [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths), for more on `primitive-load-path`.

If a corresponding compiled .go file is found in the `%load-compiled-path` or in the fallback path, and is as fresh as the source file, it will be loaded instead of the source file. If no compiled file is found, Guile may try to compile the source file and cache away the resulting .go file. See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), for more on compilation.

Once Guile finds a suitable source or compiled file is found, the file will be loaded. If, after loading the file, the module under consideration is still not defined, Guile will signal an error.

For more information on where and how to install Scheme modules, See [Installing Site Packages](04_programming_in_scheme.md#47-installing-site-packages).

* * *

Next: [R6RS Libraries](06_18_modules.md#6186-r6rs-libraries), Previous: [Modules and the File System](06_18_modules.md#6184-modules-and-the-file-system), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.5 R6RS Version References [¶](06_18_modules.md#6185-r6rs-version-references)

Guile’s module system includes support for locating modules based on a declared version specifier of the same form as the one described in R6RS (see [R6RS Library Form](https://doc.guix.gnu.org/guile/latest/en/r6rs.html#Library-form) in The Revised^6 Report on the Algorithmic Language Scheme). By using the `#:version` keyword in a `define-module` form, a module may specify a version as a list of zero or more exact, non-negative integers.

This version can then be used to locate the module during the module search process. Client modules and callers of the `use-modules` function may specify constraints on the versions of target modules by providing a _version reference_, which has one of the following forms:

 (sub-version-reference [...](06_08_macros.md))
 (and version-reference [...](06_08_macros.md))
 (or version-reference [...](06_08_macros.md))
 ([not](06_06_01_booleans.md) version-reference)

in which sub-version-reference is in turn one of:

 (sub-version)
 ([\>=](06_06_02_numerical_data_types.md) sub-version)
 ([<=](06_06_02_numerical_data_types.md) sub-version)
 (and sub-version-reference [...](06_08_macros.md))
 (or sub-version-reference [...](06_08_macros.md))
 ([not](06_06_01_booleans.md) sub-version-reference)

in which sub-version is an exact, non-negative integer as above. A version reference matches a declared module version if each element of the version reference matches a corresponding element of the module version, according to the following rules:

*   The `and` sub-form matches a version or version element if every element in the tail of the sub-form matches the specified version or version element.
*   The `or` sub-form matches a version or version element if any element in the tail of the sub-form matches the specified version or version element.
*   The `not` sub-form matches a version or version element if the tail of the sub-form does not match the version or version element.
*   The `>=` sub-form matches a version element if the element is greater than or equal to the sub-version in the tail of the sub-form.
*   The `<=` sub-form matches a version element if the version is less than or equal to the sub-version in the tail of the sub-form.
*   A sub-version matches a version element if one is eqv? to the other.

For example, a module declared as:

 (define-module (mylib mymodule) #:version (1 2 0))

would be successfully loaded by any of the following `use-modules` expressions:

 ([use-modules](06_18_modules.md) ((mylib mymodule) #:version (1 2 ([\>=](06_06_02_numerical_data_types.md) 0))))
 ([use-modules](06_18_modules.md) ((mylib mymodule) #:version (or (1 2 0) (1 2 1))))
 ([use-modules](06_18_modules.md) ((mylib mymodule) #:version ((and ([\>=](06_06_02_numerical_data_types.md) 1) ([not](06_06_01_booleans.md) 2)) 2 0)))

* * *

Next: [Variables](06_18_modules.md#6187-variables), Previous: [R6RS Version References](06_18_modules.md#6185-r6rs-version-references), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.6 R6RS Libraries [¶](06_18_modules.md#6186-r6rs-libraries)

In addition to the API described in the previous sections, you also have the option to create modules using the portable `library` form described in R6RS (see [R6RS Library Form](https://doc.guix.gnu.org/guile/latest/en/r6rs.html#Library-form) in The Revised^6 Report on the Algorithmic Language Scheme), and to import libraries created in this format by other programmers. Guile’s R6RS library implementation takes advantage of the flexibility built into the module system by expanding the R6RS library form into a corresponding Guile `define-module` form that specifies equivalent import and export requirements and includes the same body expressions. The library expression:

  (library (mylib (1 2))
    (export mybinding)
    (import (otherlib (3))))

is equivalent to the module definition:

  (define-module (mylib)
    #:version (1 2)
    #:use-module ((otherlib) #:version (3))
    #:export (mybinding))

Central to the mechanics of R6RS libraries is the concept of import and export _levels_, which control the visibility of bindings at various phases of a library’s lifecycle — macros necessary to expand forms in the library’s body need to be available at expand time; variables used in the body of a procedure exported by the library must be available at runtime. R6RS specifies the optional `for` sub-form of an _import set_ specification (see below) as a mechanism by which a library author can indicate that a particular library import should take place at a particular phase with respect to the lifecycle of the importing library.

Guile’s library implementation uses a technique called _implicit phasing_ (first described by Abdulaziz Ghuloum and R. Kent Dybvig), which allows the expander and compiler to automatically determine the necessary visibility of a binding imported from another library. As such, the `for` sub-form described below is ignored by Guile (but may be required by Schemes in which phasing is explicit).

Scheme Syntax: **library** name (export export-spec ...) (import import-spec ...) body ... [¶](06_18_modules.md)

Defines a new library with the specified name, exports, and imports, and evaluates the specified body expressions in this library’s environment.

The library name is a non-empty list of identifiers, optionally ending with a version specification of the form described above (see [Creating Guile Modules](06_18_modules.md#6183-creating-guile-modules)).

Each export-spec is the name of a variable defined or imported by the library, or must take the form `(rename (internal-name external-name) ...)`, where the identifier internal-name names a variable defined or imported by the library and external-name is the name by which the variable is seen by importing libraries.

Each import-spec must be either an _import set_ (see below) or must be of the form `(for import-set import-level ...)`, where each import-level is one of:

  run
  [expand](04_programming_in_scheme.md)
  (meta level)

where level is an integer. Note that since Guile does not require explicit phase specification, any import-sets found inside of `for` sub-forms will be “unwrapped” during expansion and processed as if they had been specified directly.

Import sets in turn take one of the following forms:

  library-reference
  (library library-reference)
  (only import-set identifier [...](06_08_macros.md))
  (except import-set identifier [...](06_08_macros.md))
  (prefix import-set identifier)
  (rename import-set (internal-identifier external-identifier) [...](06_08_macros.md))

where library-reference is a non-empty list of identifiers ending with an optional version reference (see [R6RS Version References](06_18_modules.md#6185-r6rs-version-references)), and the other sub-forms have the following semantics, defined recursively on nested import-sets:

*   The `library` sub-form is used to specify libraries for import whose names begin with the identifier “library.”
*   The `only` sub-form imports only the specified identifiers from the given import-set.
*   The `except` sub-form imports all of the bindings exported by import-set except for those that appear in the specified list of identifiers.
*   The `prefix` sub-form imports all of the bindings exported by import-set, first prefixing them with the specified identifier.
*   The `rename` sub-form imports all of the identifiers exported by import-set. The binding for each internal-identifier among these identifiers is made visible to the importing library as the corresponding external-identifier; all other bindings are imported using the names provided by import-set.

Note that because Guile translates R6RS libraries into module definitions, an import specification may be used to declare a dependency on a native Guile module — although doing so may make your libraries less portable to other Schemes.

Scheme Syntax: **import** import-spec ... [¶](06_18_modules.md)

Import into the current environment the libraries specified by the given import specifications, where each import-spec takes the same form as in the `library` form described above.

* * *

Next: [Module System Reflection](06_18_modules.md#6188-module-system-reflection), Previous: [R6RS Libraries](06_18_modules.md#6186-r6rs-libraries), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.7 Variables [¶](06_18_modules.md#6187-variables)

Each module has its own hash table, sometimes known as an _obarray_, that maps the names defined in that module to their corresponding variable objects.

A variable is a box-like object that can hold any Scheme value. It is said to be _undefined_ if its box holds a special Scheme value that denotes undefined-ness (which is different from all other Scheme values, including for example `#f`); otherwise the variable is _defined_.

On its own, a variable object is anonymous. A variable is said to be _bound_ when it is associated with a name in some way, usually a symbol in a module obarray. When this happens, the name is said to be bound to the variable, in that module.

(That’s the theory, anyway. In practice, defined-ness and bound-ness sometimes get confused, because Lisp and Scheme implementations have often conflated — or deliberately drawn no distinction between — a name that is unbound and a name that is bound to a variable whose value is undefined. We will try to be clear about the difference and explain any confusion where it is unavoidable.)

Variables do not have a read syntax. Most commonly they are created and bound implicitly by `define` expressions: a top-level `define` expression of the form

(define name value)

creates a variable with initial value value and binds it to the name name in the current module. But they can also be created dynamically by calling one of the constructor procedures `make-variable` and `make-undefined-variable`.

Scheme Procedure: **make-undefined-variable** [¶](06_18_modules.md)

C Function: **scm\_make\_undefined\_variable** () [¶](06_18_modules.md)

Return a variable that is initially unbound.

Scheme Procedure: **make-variable** init [¶](06_18_modules.md)

C Function: **scm\_make\_variable** (init) [¶](06_18_modules.md)

Return a variable initialized to value init.

Scheme Procedure: **variable-bound?** var [¶](06_18_modules.md)

C Function: **scm\_variable\_bound\_p** (var) [¶](06_18_modules.md)

Return `#t` if var is bound to a value, or `#f` otherwise. Throws an error if var is not a variable object.

Scheme Procedure: **variable-ref** var [¶](06_18_modules.md)

C Function: **scm\_variable\_ref** (var) [¶](06_18_modules.md)

Dereference var and return its value. var must be a variable object; see `make-variable` and `make-undefined-variable`.

Scheme Procedure: **variable-set!** var val [¶](06_18_modules.md)

C Function: **scm\_variable\_set\_x** (var, val) [¶](06_18_modules.md)

Set the value of the variable var to val. var must be a variable object, val can be any value. Return an unspecified value.

Scheme Procedure: **variable-unset!** var [¶](06_18_modules.md)

C Function: **scm\_variable\_unset\_x** (var) [¶](06_18_modules.md)

Unset the value of the variable var, leaving var unbound.

Scheme Procedure: **variable?** obj [¶](06_18_modules.md)

C Function: **scm\_variable\_p** (obj) [¶](06_18_modules.md)

Return `#t` if obj is a variable object, else return `#f`.

* * *

Next: [Declarative Modules](06_18_modules.md#6189-declarative-modules), Previous: [Variables](06_18_modules.md#6187-variables), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.8 Module System Reflection [¶](06_18_modules.md#6188-module-system-reflection)

The previous sections have described a declarative view of the module system. You can also work with it programmatically by accessing and modifying various parts of the Scheme objects that Guile uses to implement the module system.

At any time, there is a _current module_. This module is the one where a top-level `define` and similar syntax will add new bindings. You can find other module objects with `resolve-module`, for example.

These module objects can be used as the second argument to `eval`.

Scheme Procedure: **current-module** [¶](06_18_modules.md)

C Function: **scm\_current\_module** () [¶](06_18_modules.md)

Return the current module object.

Scheme Procedure: **set-current-module** module [¶](06_18_modules.md)

C Function: **scm\_set\_current\_module** (module) [¶](06_18_modules.md)

Set the current module to module and return the previous current module.

Scheme Procedure: **save-module-excursion** thunk [¶](06_18_modules.md)

Call thunk within a `dynamic-wind` such that the module that is current at invocation time is restored when thunk’s dynamic extent is left (see [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind)).

More precisely, if thunk escapes non-locally, the current module (at the time of escape) is saved, and the original current module (at the time thunk’s dynamic extent was last entered) is restored. If thunk’s dynamic extent is re-entered, then the current module is saved, and the previously saved inner module is set current again.

Scheme Procedure: **resolve-module** name \[autoload=#t\] \[version=#f\] \[#:ensure=#t\] [¶](06_18_modules.md)

C Function: **scm\_resolve\_module** (name) [¶](06_18_modules.md)

Find the module named name and return it. When it has not already been defined and autoload is true, try to auto-load it. When it can’t be found that way either, create an empty module if ensure is true, otherwise return `#f`. If version is true, ensure that the resulting module is compatible with the given version reference (see [R6RS Version References](06_18_modules.md#6185-r6rs-version-references)). The name is a list of symbols.

Scheme Procedure: **resolve-interface** name \[#:select=#f\] \[#:hide=’()\] \[#:prefix=#f\] \[#:renamer=#f\] \[#:version=#f\] [¶](06_18_modules.md)

Find the module named name as with `resolve-module` and return its interface. The interface of a module is also a module object, but it contains only the exported bindings.

Scheme Procedure: **module-uses** module [¶](06_18_modules.md)

Return a list of the interfaces used by module.

Scheme Procedure: **module-use!** module interface [¶](06_18_modules.md)

Add interface to the front of the use-list of module. Both arguments should be module objects, and interface should very likely be a module returned by `resolve-interface`.

Scheme Procedure: **reload-module** module [¶](06_18_modules.md)

Revisit the source file that corresponds to module. Raises an error if no source file is associated with the given module.

As mentioned in the previous section, modules contain a mapping between identifiers (as symbols) and storage locations (as variables). Guile defines a number of procedures to allow access to this mapping. If you are programming in C, [Accessing Modules from C](06_18_modules.md#61810-accessing-modules-from-c).

Scheme Procedure: **module-variable** module name [¶](06_18_modules.md)

Return the variable bound to name (a symbol) in module, or `#f` if name is unbound.

Scheme Procedure: **module-add!** module name var [¶](06_18_modules.md)

Define a new binding between name (a symbol) and var (a variable) in module.

Scheme Procedure: **module-ref** module name [¶](06_18_modules.md)

Look up the value bound to name in module. Like `module-variable`, but also does a `variable-ref` on the resulting variable, raising an error if name is unbound.

Scheme Procedure: **module-define!** module name value [¶](06_18_modules.md)

Locally bind name to value in module. If name was already locally bound in module, i.e., defined locally and not by an imported module, the value stored in the existing variable will be updated. Otherwise, a new variable will be added to the module, via `module-add!`.

Scheme Procedure: **module-set!** module name value [¶](06_18_modules.md)

Update the binding of name in module to value, raising an error if name is not already bound in module.

There are many other reflective procedures available in the default environment. If you find yourself using one of them, please contact the Guile developers so that we can commit to stability for that interface.

* * *

Next: [Accessing Modules from C](06_18_modules.md#61810-accessing-modules-from-c), Previous: [Module System Reflection](06_18_modules.md#6188-module-system-reflection), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.9 Declarative Modules [¶](06_18_modules.md#6189-declarative-modules)

The first-class access to modules and module variables described in the previous subsection is very powerful and allows Guile users to build many tools to dynamically learn things about their Guile systems. However, as Scheme godparent Mathias Felleisen wrote in “On the Expressive Power of Programming Languages”, a more expressive language is necessarily harder to reason about. There are transformations that Guile’s compiler would like to make which can’t be done if every top-level definition is subject to mutation at any time.

Consider this module:

(define-module (boxes)
  #:export (make-box box-ref box-set! box-swap!))

(define (make-box x) (list x))
(define (box-ref box) (car box))
(define (box-set! box x) (set-car! box x))
(define (box-swap! box x)
  (let ((y (box-ref box)))
    (box-set! box x)
    y))

Ideally you’d like for the `box-ref` in `box-swap!` to be inlined to `car`. Guile’s compiler can do this, but only if it knows that `box-ref`’s definition is what it appears to be in the text. However, in the general case it could be that a programmer could reach into the `(boxes)` module at any time and change the value of `box-ref`.

To allow Guile to reason about the values of top-levels from a module, a module can be marked as _declarative_. This flag applies only to the subset of top-level definitions that are themselves declarative: those that are defined within the compilation unit, and not assigned (`set!`) or redefined within the compilation unit.

To explicitly mark a module as being declarative, pass the `#:declarative?` keyword argument when declaring a module:

(define-module (boxes)
  #:export (make-box box-ref box-set! box-swap!)
  #:declarative? #t)

By default, modules are compiled declaratively if the `user-modules-declarative?` parameter is true when the module is compiled.

Scheme Parameter: **user-modules-declarative?** [¶](06_18_modules.md)

A boolean indicating whether definitions in modules created by `define-module` or implicitly as part of a compilation unit without an explicit module can be treated as declarative.

Because it’s usually what you want, the default value of `user-modules-declarative?` is `#t`.

#### Should I Mark My Module As Declarative? [¶](06_18_modules.md#should-i-mark-my-module-as-declarative)

In the vast majority of use cases, declarative modules are what you want. However, there are exceptions.

Consider the `(boxes)` module above. Let’s say you want to be able to go in and change the definition of `box-set!` at run-time:

scheme@(guile-user)> (use-modules (boxes))
scheme@(guile-user)> ,module boxes
scheme@(boxes)> (define (box-set! x y) (set-car! x (pk y)))

However, considering that `(boxes)` is a declarative module, it could be that `box-swap!` inlined the call to `box-set!` – so it may be that you are surprised if you call `(box-swap! x y)` and you don’t see the new definition being used. (Note, however, that Guile has no guarantees about what definitions its compiler will or will not inline.)

If you want to allow the definition of `box-set!` to be changed and to have all of its uses updated, then probably the best option is to edit the module and reload the whole thing:

scheme@(guile-user)> ,reload (boxes)

The advantage of the reloading approach is that you maintain the optimizations that declarative modules enable, while also being able to live-update the code. If the module keeps precious program state, those definitions can be marked as `define-once` to prevent reloads from overwriting them. See [Top Level Variable Definitions](06_10_definitions_and_variable_bindings.md#6101-top-level-variable-definitions), for more on `define-once`. Incidentally, `define-once` also prevents declarative-definition optimizations, so if there’s a limited subset of redefinable bindings, `define-once` could be an interesting tool to mark those definitions as works-in-progress for interactive program development.

To users, whether a module is declarative or not is mostly immaterial: besides normal use via `use-modules`, users can reference and redefine public or private bindings programmatically or interactively. The only difference is that changing a declarative definition may not change all of its uses. If this use-case is important to you, and if reloading whole modules is insufficient, then you can mark all definitions in a module as non-declarative by adding `#:declarative? #f` to the module definition.

The default of whether modules are declarative or not can be controlled via the `(user-modules-declarative?)` parameter mentioned above, but care should be taken to set this parameter when the modules are compiled, e.g. via `(eval-when (expand) (user-modules-declarative? #f))`. See [Eval-when](06_08_macros.md#688-eval-when).

Alternately you can prevent declarative-definition optimizations by compiling at the `-O1` optimization level instead of the default `-O2`, or via explicitly passing `-Ono-letrectify` to the `guild compile` invocation. See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), for more on compiler options.

One final note. Currently, definitions from declarative modules can only be inlined within the module they are defined in, and within a compilation unit. This may change in the future to allow Guile to inline imported declarative definitions as well (cross-module inlining). To Guile, whether a definition is inlinable or not is a property of the definition, not its use. We hope to improve compiler tooling in the future to allow the user to identify definitions that are out of date when a declarative binding is redefined.

* * *

Next: [provide and require](06_18_modules.md#61811-provide-and-require), Previous: [Declarative Modules](06_18_modules.md#6189-declarative-modules), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.10 Accessing Modules from C [¶](06_18_modules.md#61810-accessing-modules-from-c)

The last sections have described how modules are used in Scheme code, which is the recommended way of creating and accessing modules. You can also work with modules from C, but it is more cumbersome.

The following procedures are available.

C Function: `SCM` **scm\_c\_call\_with\_current\_module** `(SCM module, SCM (*func)(void *), void *data)` [¶](06_18_modules.md)

Call func and make module the current module during the call. The argument data is passed to func. The return value of `scm_c_call_with_current_module` is the return value of func.

C Function: `SCM` **scm\_public\_variable** `(SCM module_name, SCM name)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_c\_public\_variable** `(const char *module_name, const char *name)` [¶](06_18_modules.md)

Find a the variable bound to the symbol name in the public interface of the module named module\_name.

module\_name should be a list of symbols, when represented as a Scheme object, or a space-separated string, in the `const char *` case. See `scm_c_define_module` below, for more examples.

Signals an error if no module was found with the given name. If name is not bound in the module, just returns `#f`.

C Function: `SCM` **scm\_private\_variable** `(SCM module_name, SCM name)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_c\_private\_variable** `(const char *module_name, const char *name)` [¶](06_18_modules.md)

Like `scm_public_variable`, but looks in the internals of the module named module\_name instead of the public interface. Logically, these procedures should only be called on modules you write.

C Function: `SCM` **scm\_public\_lookup** `(SCM module_name, SCM name)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_c\_public\_lookup** `(const char *module_name, const char *name)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_private\_lookup** `(SCM module_name, SCM name)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_c\_private\_lookup** `(const char *module_name, const char *name)` [¶](06_18_modules.md)

Like `scm_public_variable` or `scm_private_variable`, but if the name is not bound in the module, signals an error. Returns a variable, always.

static SCM eval\_string\_var;

/\* NOTE: It is important that the call to 'my\_init'
   happens-before all calls to 'my\_eval\_string'. \*/
void my\_init (void)
{
  eval\_string\_var = scm\_c\_public\_lookup ("ice-9 eval-string",
                                         "eval-string");
}

SCM my\_eval\_string (SCM str)
{
  return scm\_call\_1 (scm\_variable\_ref (eval\_string\_var), str);
}

C Function: `SCM` **scm\_public\_ref** `(SCM module_name, SCM name)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_c\_public\_ref** `(const char *module_name, const char *name)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_private\_ref** `(SCM module_name, SCM name)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_c\_private\_ref** `(const char *module_name, const char *name)` [¶](06_18_modules.md)

Like `scm_public_lookup` or `scm_private_lookup`, but additionally dereferences the variable. If the variable object is unbound, signals an error. Returns the value bound to name in module\_name.

In addition, there are a number of other lookup-related procedures. We suggest that you use the `scm_public_` and `scm_private_` family of procedures instead, if possible.

C Function: `SCM` **scm\_c\_lookup** `(const char *name)` [¶](06_18_modules.md)

Return the variable bound to the symbol indicated by name in the current module. If there is no such binding or the symbol is not bound to a variable, signal an error.

C Function: `SCM` **scm\_lookup** `(SCM name)` [¶](06_18_modules.md)

Like `scm_c_lookup`, but the symbol is specified directly.

C Function: `SCM` **scm\_c\_module\_lookup** `(SCM module, const char *name)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_module\_lookup** `(SCM module, SCM name)` [¶](06_18_modules.md)

Like `scm_c_lookup` and `scm_lookup`, but the specified module is used instead of the current one.

C Function: `SCM` **scm\_module\_variable** `(SCM module, SCM name)` [¶](06_18_modules.md)

Like `scm_module_lookup`, but if the binding does not exist, just returns `#f` instead of raising an error.

To define a value, use `scm_define`:

C Function: `SCM` **scm\_c\_define** `(const char *name, SCM val)` [¶](06_18_modules.md)

Bind the symbol indicated by name to a variable in the current module and set that variable to val. When name is already bound to a variable, use that. Else create a new variable.

C Function: `SCM` **scm\_define** `(SCM name, SCM val)` [¶](06_18_modules.md)

Like `scm_c_define`, but the symbol is specified directly.

C Function: `SCM` **scm\_c\_module\_define** `(SCM module, const char *name, SCM val)` [¶](06_18_modules.md)

C Function: `SCM` **scm\_module\_define** `(SCM module, SCM name, SCM val)` [¶](06_18_modules.md)

Like `scm_c_define` and `scm_define`, but the specified module is used instead of the current one.

In some rare cases, you may need to access the variable that `scm_module_define` would have accessed, without changing the binding of the existing variable, if one is present. In that case, use `scm_module_ensure_local_variable`:

C Function: `SCM` **scm\_module\_ensure\_local\_variable** `(SCM module, SCM sym)` [¶](06_18_modules.md)

Like `scm_module_define`, but if the sym is already locally bound in that module, the variable’s existing binding is not reset. Returns a variable.

C Function: `SCM` **scm\_module\_reverse\_lookup** `(SCM module, SCM variable)` [¶](06_18_modules.md)

Find the symbol that is bound to variable in module. When no such binding is found, return `#f`.

C Function: `SCM` **scm\_c\_define\_module** `(const char *name, void (*init)(void *), void *data)` [¶](06_18_modules.md)

Define a new module named name and make it current while init is called, passing it data. Return the module.

The parameter name is a string with the symbols that make up the module name, separated by spaces. For example, ‘"foo bar"’ names the module ‘(foo bar)’.

When there already exists a module named name, it is used unchanged, otherwise, an empty module is created.

C Function: `SCM` **scm\_c\_resolve\_module** `(const char *name)` [¶](06_18_modules.md)

Find the module name name and return it. When it has not already been defined, try to auto-load it. When it can’t be found that way either, create an empty module. The name is interpreted as for `scm_c_define_module`.

C Function: `SCM` **scm\_c\_use\_module** `(const char *name)` [¶](06_18_modules.md)

Add the module named name to the uses list of the current module, as with `(use-modules name)`. The name is interpreted as for `scm_c_define_module`.

C Function: `void` **scm\_c\_export** `(const char *name, ...)` [¶](06_18_modules.md)

Add the bindings designated by name, ... to the public interface of the current module. The list of names is terminated by `NULL`.

* * *

Next: [Environments](06_18_modules.md#61812-environments), Previous: [Accessing Modules from C](06_18_modules.md#61810-accessing-modules-from-c), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.11 provide and require [¶](06_18_modules.md#61811-provide-and-require)

Aubrey Jaffer, mostly to support his portable Scheme library SLIB, implemented a provide/require mechanism for many Scheme implementations. Library files in SLIB _provide_ a feature, and when user programs _require_ that feature, the library file is loaded in.

For example, the file random.scm in the SLIB package contains the line

([provide](06_23_configuration_features_and_runtime_options.md) 'random)

so to use its procedures, a user would type

(require 'random)

and they would magically become available, _but still have the same names!_ So this method is nice, but not as good as a full-featured module system.

When SLIB is used with Guile, provide and require can be used to access its facilities.

* * *

Previous: [provide and require](06_18_modules.md#61811-provide-and-require), Up: [Modules](06_18_modules.md#618-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.18.12 Environments [¶](06_18_modules.md#61812-environments)

Scheme, as defined in R5RS, does _not_ have a full module system. However it does define the concept of a top-level _environment_. Such an environment maps identifiers (symbols) to Scheme objects such as procedures and lists: [The Concept of Closure](03_hello_scheme.md#34-the-concept-of-closure). In other words, it implements a set of _bindings_.

Environments in R5RS can be passed as the second argument to `eval` (see [Procedures for On the Fly Evaluation](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation)). Three procedures are defined to return environments: `scheme-report-environment`, `null-environment` and `interaction-environment` (see [Procedures for On the Fly Evaluation](06_16_reading_and_evaluating_scheme_code.md#6165-procedures-for-on-the-fly-evaluation)).

In addition, in Guile any module can be used as an R5RS environment, i.e., passed as the second argument to `eval`.

Note: the following two procedures are available only when the `(ice-9 r5rs)` module is loaded:

([use-modules](06_18_modules.md) (ice-9 r5rs))

Scheme Procedure: **scheme-report-environment** version [¶](06_18_modules.md)

Scheme Procedure: **null-environment** version [¶](06_18_modules.md)

version must be the exact integer ‘5’, corresponding to revision 5 of the Scheme report (the Revised^5 Report on Scheme). `scheme-report-environment` returns a specifier for an environment that is empty except for all bindings defined in the report that are either required or both optional and supported by the implementation. `null-environment` returns a specifier for an environment that is empty except for the (syntactic) bindings for all syntactic keywords defined in the report that are either required or both optional and supported by the implementation.

Currently Guile does not support values of version for other revisions of the report.

The effect of assigning (through the use of `eval`) a variable bound in a `scheme-report-environment` (for example `car`) is unspecified. Currently the environments specified by `scheme-report-environment` are not immutable in Guile.

* * *

Next: [Foreign Objects](06_20_foreign_objects.md#620-foreign-objects), Previous: [Modules](06_18_modules.md#618-modules), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
