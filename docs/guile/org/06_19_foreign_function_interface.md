### 6.19 Foreign Function Interface [¶](06_19_foreign_function_interface.md#619-foreign-function-interface)

Sometimes you need to use libraries written in C or Rust or some other non-Scheme language. More rarely, you might need to write some C to extend Guile. This section describes how to load these “foreign libraries”, look up data and functions inside them, and so on.

*   [Foreign Libraries](06_19_foreign_function_interface.md#6191-foreign-libraries)
*   [Foreign Extensions](06_19_foreign_function_interface.md#6192-foreign-extensions)
*   [Foreign Pointers](06_19_foreign_function_interface.md#6193-foreign-pointers)
*   [Foreign Types](06_19_foreign_function_interface.md#6194-foreign-types)
*   [Foreign Functions](06_19_foreign_function_interface.md#6195-foreign-functions)
*   [Void Pointers and Byte Access](06_19_foreign_function_interface.md#6196-void-pointers-and-byte-access)
*   [Foreign Structs](06_19_foreign_function_interface.md#6197-foreign-structs)
*   [More Foreign Functions](06_19_foreign_function_interface.md#6198-more-foreign-functions)

* * *

Next: [Foreign Extensions](06_19_foreign_function_interface.md#6192-foreign-extensions), Up: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.19.1 Foreign Libraries [¶](06_19_foreign_function_interface.md#6191-foreign-libraries)

Just as Guile can load up Scheme libraries at run-time, Guile can also load some system libraries written in C or other low-level languages. We refer to these as dynamically-loadable modules as _foreign libraries_, to distinguish them from native libraries written in Scheme or other languages implemented by Guile.

Foreign libraries usually come in two forms. Some foreign libraries are part of the operating system, such as the compression library `libz`. These shared libraries are built in such a way that many programs can use their functionality without duplicating their code. When a program written in C is built, it can declare that it uses a specific set of shared libraries. When the program is run, the operating system takes care of locating and loading the shared libraries.

The operating system components that can dynamically load and link shared libraries when a program is run are also available programmatically during a program’s execution. This is the interface that’s most useful for Guile, and this is what we mean in Guile when we refer to _dynamic linking_. Dynamic linking at run-time is sometimes called _dlopening_, to distinguish it from the dynamic linking that happens at program start-up.

The other kind of foreign library is sometimes known as a module, plug-in, bundle, or an extension. These foreign libraries aren’t meant to be linked to by C programs, but rather only to be dynamically loaded at run-time – they extend some main program with functionality, but don’t stand on their own. Sometimes a Guile library will implement some of its functionality in a loadable module.

In either case, the interface on the Guile side is the same. You load the interface using `load-foreign-library`. The resulting foreign library object implements a simple lookup interface whereby the user can get addresses of data or code exported by the library. There is no facility to inspect foreign libraries; you have to know what’s in there already before you look.

Routines for loading foreign libraries and accessing their contents are implemented in the `(system foreign-library)` module.

(use-modules (system foreign-library))

Scheme Procedure: **load-foreign-library** \[library\] \[#:extensions=system-library-extensions\] \[#:search-ltdl-library-path?=#t\] \[#:search-path=search-path\] \[#:search-system-paths?=#t\] \[#:lazy?=#t\] \[#:global=#f\] \[#:host-type-rename?=#t\] \[#:allow-dll-version-suffix?=#t\] [¶](06_19_foreign_function_interface.md)

This procedure finds the shared library denoted by library (a string) and dynamically links it into the running Guile application. On success, the procedure returns a Scheme object suitable for representing the linked object file. Otherwise an error is thrown.

In the common usage, the library parameter is a filename with no path and with no filename extension, such as `.so`, `.dylib` or `.dll`. The procedure will search for the library in a set of standard locations using the common filename extensions for the OS. The optional parameters can customize this behavior.

When library has directory elements or a filename extension, a more targeted search is performed.

For each system, Guile has a default set of extensions that it will try. On GNU systems, the default extension set is just `.so`; on Windows, just `.dll`; and on Darwin (Mac OS), it is `.bundle`, `.so`, and `.dylib`. Pass `#:extensions extensions` to override the default extensions list. If library contains one of the extensions, no extensions are tried, so it is possible to specify the extension if you know exactly what file to load.

Unless library denotes an absolute file name or otherwise contains a directory separator (`/`, and also `\` on Windows), Guile will search for the library in the directories listed in search-paths. The default search path has three components, which can all be overridden by colon-delimited (semicolon on Windows) environment variables:

`GUILE_EXTENSIONS_PATH`

This is the environment variable for users to add directories containing Guile extensions to the search path. The default value has no entries. This environment variable was added in Guile 3.0.6.

`LTDL_LIBRARY_PATH`

When search-ltdl-library-path? is true, this environment variable can also be used to add directories to the search path. For each directory given in this environment variable, two directories are added to the search path: the given directory (for example, `D`) and a `.libs` subdirectory (`D/.libs`).

For more information on the rationale, see the note below.

`GUILE_SYSTEM_EXTENSIONS_PATH`

The last path in Guile’s search path belongs to Guile itself, and defaults to the libdir and the extensiondir, in that order. For example, if you install to /opt/guile, these would probably be /opt/guile/lib and `/opt/guile/lib/guile/3.0/extensions`, respectively. See [Parallel Installations](05_programming_in_c.md#51-parallel-installations), for more details on `extensionsdir`.

For DLL-using systems, it searches bindir rather than libdir, so /opt/guile/bin in this example.

Finally, if no library is found in the search path, and if library is not absolute and does not include directory separators, and if search-system-paths? is true, the operating system may have its own logic for where to locate library. For example, on GNU, there will be a default set of paths (often /usr/lib and /lib, though it depends on the system), and the `LD_LIBRARY_PATH` environment variable can add additional paths. On DLL-using systems, the `PATH` is searched. Other operating systems have other conventions.

Falling back to the operating system for search is usually not a great thing; it is a recipe for making programs that work on one machine but not on others. Still, when wrapping system libraries, it can be the only way to get things working at all.

If lazy? is true (the default), Guile will request the operating system to resolve symbols used by the loaded library as they are first used. If global? is true, symbols defined by the loaded library will be available when other modules need to resolve symbols; the default is `#f`, which keeps symbols local.

If host-type-rename? is true (the default) library names may be modified based on the current `%host-type`. On Cygwin hosts, the search behavior is modified such that a filename that starts with “lib” will be searched for under the name “cyg”, as is customary for Cygwin. Similarly, for MSYS hosts, “lib” becomes “msys-”.

If dll-version-suffix? is true (the default), the search behavior is modified such that when searching for a DLL, it will also search for DLLs with version suffixes. For example, a search for libtiff.dll will also allow libtiff-1.dll. When the unversioned DLL is not found and multiple versioned DLLs exists, it will return the versioned DLL with the highest version. Note that when searching, directories take precedence. It does not return the highest versioned DLL among all search directories collectively; it returns the highest versioned in the first directory to have the DLL.

If library argument is omitted, it defaults to `#f`. If `library` is false, the resulting foreign library gives access to all symbols available for dynamic linking in the currently running executable.

The environment variables mentioned above are parsed when the foreign-library module is first loaded and bound to parameters. Null path components, for example the three components of `GUILE_SYSTEM_EXTENSIONS_PATH="::"`, are ignored.

Scheme Parameter: **guile-extensions-path** [¶](06_19_foreign_function_interface.md)

Scheme Parameter: **ltdl-library-path** [¶](06_19_foreign_function_interface.md)

Scheme Parameter: **guile-system-extensions-path** [¶](06_19_foreign_function_interface.md)

Parameters whose initial values are taken from `GUILE_EXTENSIONS_PATH`, `LTDL_LIBRARY_PATH`, and `GUILE_SYSTEM_EXTENSIONS_PATH`, respectively. See [Parameters](06_11_controlling_the_flow_of_program_execution.md#61112-parameters). The current values of these parameters are used when building the search path when `load-foreign-library` is called, unless the caller explicitly passes a `#:search-path` argument.

Scheme Procedure: **foreign-library?** obj [¶](06_19_foreign_function_interface.md)

Return `#t` if obj is a foreign library, or `#f` otherwise.

Before Guile 3.0.6, Guile loaded foreign libraries using `libltdl`, the dynamic library loader provided by libtool. This loader used `LTDL_LIBRARY_PATH`, and for backwards compatibility we still support that path.

However, `libltdl` would not only open `.so` (or `.dll` and so on) files, but also the `.la` files created by libtool. In installed libraries – libraries that are in the target directories of `make install` – `.la` files are never needed, to the extent that most GNU/Linux distributions remove them entirely. It is sufficient to just load the `.so` (or `.dll` and so on) files, which are always located in the same directory as the `.la` files.

But for uninstalled dynamic libraries, like those in a build tree, the situation is a bit of a mess. If you have a project that uses libtool to build libraries – which is the case for Guile, and for most projects using autotools – and you build foo.so in directory D, libtool will put foo.la in D, but foo.so gets put into D/.libs.

Users were mostly oblivious to this situation, as `libltdl` had special logic to be able to read the `.la` file to know where to find the `.so`, even from an uninstalled build tree, preventing the existence of .libs from leaking out to the user.

We don’t use libltdl now, essentially for flexibility and error-reporting reasons. But, to keep this old use-case working, if search-ltdl-library-path? is true, we add each entry of `LTDL_LIBRARY_PATH` to the default extensions load path, additionally adding the .libs subdirectories for each entry, in case there are .so files there instead of alongside the .la files.

* * *

Next: [Foreign Pointers](06_19_foreign_function_interface.md#6193-foreign-pointers), Previous: [Foreign Libraries](06_19_foreign_function_interface.md#6191-foreign-libraries), Up: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.19.2 Foreign Extensions [¶](06_19_foreign_function_interface.md#6192-foreign-extensions)

One way to use shared libraries is to extend Guile. Such loadable modules generally define one distinguished initialization function that, when called, will use the `libguile` API to define procedures in the current module.

Concretely, you might extend Guile with an implementation of the Bessel function, `j0`:

#include <math.h>
#include <libguile.h>

SCM
j0\_wrapper (SCM x)
{
  return scm\_from\_double (j0 (scm\_to\_double (x, "j0")));
}

void
init\_math\_bessel (void)
{
  scm\_c\_define\_gsubr ("j0", 1, 0, 0, j0\_wrapper);
}

The C source file would then need to be compiled into a shared library. On GNU/Linux, the compiler invocation might look like this:

gcc -shared -o bessel.so -fPIC bessel.c

A good default place to put shared libraries that extend Guile is into the extensions dir. From the command line or a build script, invoke `pkg-config --variable=extensionsdir guile-3.0` to print the extensions dir. See [Parallel Installations](05_programming_in_c.md#51-parallel-installations), for more details.

Guile can load up `bessel.so` via `load-extension`.

Scheme Procedure: **load-extension** lib init [¶](06_19_foreign_function_interface.md)

C Function: **scm\_load\_extension** (lib, init) [¶](06_19_foreign_function_interface.md)

Load and initialize the extension designated by LIB and INIT.

The normal way for a extension to be used is to write a small Scheme file that defines a module, and to load the extension into this module. When the module is auto-loaded, the extension is loaded as well. For example:

(define-module (math bessel)
  #:export (j0))

([load-extension](06_19_foreign_function_interface.md) "bessel" "init\_math\_bessel")

This `load-extension` invocation loads the `bessel` library via `(load-foreign-library "bessel")`, then looks up the `init_math_bessel` symbol in the library, treating it as a function of no arguments, and calls that function.

If you decide to put your extension outside the default search path for `load-foreign-library`, probably you should adapt the Scheme module to specify its absolute path. For example, if you use `automake` to build your extension and place it in `$(pkglibdir)`, you might define a build-parameters module that gets created by the build system:

(define-module (math config)
  #:export (extensiondir))
(define extensiondir "PKGLIBDIR")

This file would be `config.scm.in`. You would define a `make` rule to substitute in the absolute installed file name:

config.scm: config.scm.in
        sed 's|PKGLIBDIR|$(pkglibdir)|' <$< >$ 

Then your `(math bessel)` would import `(math config)`, then `(load-extension (in-vicinity extensiondir "bessel") "init_math_bessel")`.

An alternate approach would be to rebind the `guile-extensions-path` parameter, or its corresponding environment variable, but note that changing those parameters applies to other users of `load-foreign-library` as well.

Note that the new primitives that the extension adds to Guile with `scm_c_define_gsubr` (see [Primitive Procedures](06_07_procedures.md#672-primitive-procedures)) or with any of the other mechanisms are placed into the module that is current when the `scm_c_define_gsubr` is executed, so to be clear about what goes where it’s best to include the `load-extension` in a module, as above. Alternately, the C code can use `scm_c_define_module` to specify which module is being created:

static void
do\_init (void \*unused)
{
  scm\_c\_define\_gsubr ("j0", 1, 0, 0, j0\_wrapper);
  scm\_c\_export ("j0", NULL);
}

void
init\_math\_bessel ()
{
  scm\_c\_define\_module ("math bessel", do\_init, NULL);
}

And yet... if what we want is just the `j0` function, it seems like a lot of ceremony to have to compile a Guile-specific wrapper library complete with an initialization function and wrapper module to allow Guile users to call it. There is another way, but to get there, we have to talk about function pointers and function types first. See [Foreign Functions](06_19_foreign_function_interface.md#6195-foreign-functions), to skip to the good parts.

* * *

Next: [Foreign Types](06_19_foreign_function_interface.md#6194-foreign-types), Previous: [Foreign Extensions](06_19_foreign_function_interface.md#6192-foreign-extensions), Up: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.19.3 Foreign Pointers [¶](06_19_foreign_function_interface.md#6193-foreign-pointers)

Foreign libraries are essentially key-value mappings, where the keys are names of definitions and the values are the addresses of those definitions. To look up the address of a definition, use `foreign-library-pointer` from the `(system foreign-library)` module.

Scheme Procedure: **foreign-library-pointer** lib name [¶](06_19_foreign_function_interface.md)

Return a “wrapped pointer” for the symbol name in the shared object referred to by lib. The returned pointer points to a C object.

As a convenience, if lib is not a foreign library, it will be passed to `load-foreign-library`.

If we continue with the `bessel.so` example from before, we can get the address of the `init_math_bessel` function via:

(use-modules (system foreign-library))
(define init (foreign-library-pointer "bessel" "init\_math\_bessel"))
init
⇒ #<pointer 0x7fb35b1b4688>

A value returned by `foreign-library-pointer` is a Scheme wrapper for a C pointer. Pointers are a data type in Guile that is disjoint from all other types. The next section discusses ways to dereference pointers, but before then we describe the usual type predicates and so on.

Note that the rest of the interfaces in this section are part of the `(system foreign)` library:

(use-modules (system foreign))

Scheme Procedure: **pointer-address** pointer [¶](06_19_foreign_function_interface.md)

C Function: **scm\_pointer\_address** (pointer) [¶](06_19_foreign_function_interface.md)

Return the numerical value of pointer.

(pointer-address init)
⇒ 139984413364296 ; YMMV

Scheme Procedure: **make-pointer** address \[finalizer\] [¶](06_19_foreign_function_interface.md)

Return a foreign pointer object pointing to address. If finalizer is passed, it should be a pointer to a one-argument C function that will be called when the pointer object becomes unreachable.

Scheme Procedure: **pointer?** obj [¶](06_19_foreign_function_interface.md)

Return `#t` if obj is a pointer object, or `#f` otherwise.

Scheme Variable: **%null-pointer** [¶](06_19_foreign_function_interface.md)

A foreign pointer whose value is 0.

Scheme Procedure: **null-pointer?** pointer [¶](06_19_foreign_function_interface.md)

Return `#t` if pointer is the null pointer, `#f` otherwise.

For the purpose of passing SCM values directly to foreign functions, and allowing them to return SCM values, Guile also supports some unsafe casting operators.

Scheme Procedure: **scm->pointer** scm [¶](06_19_foreign_function_interface.md)

Return a foreign pointer object with the `object-address` of scm.

Scheme Procedure: **pointer->scm** pointer [¶](06_19_foreign_function_interface.md)

Unsafely cast pointer to a Scheme object. Cross your fingers!

Sometimes you want to give C extensions access to the dynamic FFI. At that point, the names get confusing, because “pointer” can refer to a `SCM` object that wraps a pointer, or to a `void*` value. We will try to use “pointer object” to refer to Scheme objects, and “pointer value” to refer to `void *` values.

C Function: `SCM` **scm\_from\_pointer** `(void *ptr, void (*finalizer) (void*))` [¶](06_19_foreign_function_interface.md)

Create a pointer object from a pointer value.

If finalizer is non-null, Guile arranges to call it on the pointer value at some point after the pointer object becomes collectible.

C Function: `void*` **scm\_to\_pointer** `(SCM obj)` [¶](06_19_foreign_function_interface.md)

Unpack the pointer value from a pointer object.

* * *

Next: [Foreign Functions](06_19_foreign_function_interface.md#6195-foreign-functions), Previous: [Foreign Pointers](06_19_foreign_function_interface.md#6193-foreign-pointers), Up: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.19.4 Foreign Types [¶](06_19_foreign_function_interface.md#6194-foreign-types)

From Scheme’s perspective, foreign pointers are shards of chaos. The user can create a foreign pointer for any address, and do with it what they will. The only thing that lends a sense of order to the whole is a shared hallucination that certain storage locations have certain types. When making Scheme wrappers for foreign interfaces, we hide the madness by explicitly representing the the data types of parameters and fields.

These “foreign type values” may be constructed using the constants and procedures from the `(system foreign)` module, which may be loaded like this:

(use-modules (system foreign))

`(system foreign)` exports a number of values expressing the basic C types.

Scheme Variable: **int8** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **uint8** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **uint16** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **int16** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **uint32** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **int32** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **uint64** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **int64** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **float** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **double** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **complex-double** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **complex-float** [¶](06_19_foreign_function_interface.md)

These values represent the C numeric types of the specified sizes and signednesses. `complex-float` and `complex-double` stand for C99 `float _Complex` and `double _Complex` respectively.

In addition there are some convenience bindings for indicating types of platform-dependent size.

Scheme Variable: **int** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **unsigned-int** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **long** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **unsigned-long** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **short** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **unsigned-short** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **size\_t** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **ssize\_t** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **ptrdiff\_t** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **intptr\_t** [¶](06_19_foreign_function_interface.md)

Scheme Variable: **uintptr\_t** [¶](06_19_foreign_function_interface.md)

Values exported by the `(system foreign)` module, representing C numeric types. For example, `long` may be `equal?` to `int64` on a 64-bit platform.

Scheme Variable: **void** [¶](06_19_foreign_function_interface.md)

The `void` type. It can be used as the first argument to `pointer->procedure` to wrap a C function that returns nothing.

In addition, the symbol `*` is used by convention to denote pointer types. Procedures detailed in the following sections, such as `pointer->procedure`, accept it as a type descriptor.

* * *

Next: [Void Pointers and Byte Access](06_19_foreign_function_interface.md#6196-void-pointers-and-byte-access), Previous: [Foreign Types](06_19_foreign_function_interface.md#6194-foreign-types), Up: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.19.5 Foreign Functions [¶](06_19_foreign_function_interface.md#6195-foreign-functions)

The most natural thing to do with a dynamic library is to grovel around in it for a function pointer: a _foreign function_. Load the `(system foreign)` module to use these Scheme interfaces.

(use-modules (system foreign))

Scheme Procedure: **pointer->procedure** return\_type func\_ptr arg\_types \[#:return-errno?=#f\] [¶](06_19_foreign_function_interface.md)

C Function: **scm\_pointer\_to\_procedure** (return\_type, func\_ptr, arg\_types) [¶](06_19_foreign_function_interface.md)

C Function: **scm\_pointer\_to\_procedure\_with\_errno** (return\_type, func\_ptr, arg\_types) [¶](06_19_foreign_function_interface.md)

Make a foreign function.

Given the foreign void pointer func\_ptr, its argument and return types arg\_types and return\_type, return a procedure that will pass arguments to the foreign function and return appropriate values.

arg\_types should be a list of foreign types. `return_type` should be a foreign type. See [Foreign Types](06_19_foreign_function_interface.md#6194-foreign-types), for more information on foreign types.

If return-errno? is true, or when calling `scm_pointer_to_procedure_with_errno`, the returned procedure will return two values, with `errno` as the second value.

Finally, in `(system foreign-library)` there is a convenient wrapper function, joining together `foreign-library-pointer` and `pointer->procedure`:

Scheme Procedure: **foreign-library-function** lib name \[#:return-type=void\] \[#:arg-types=’()\] \[#:return-errno?=#f\] [¶](06_19_foreign_function_interface.md)

Load the address of name from lib, and treat it as a function taking arguments arg-types and returning return-type, optionally also with errno.

An invocation of `foreign-library-function` is entirely equivalent to:

(pointer->procedure return-type
                    (foreign-library-pointer lib name)
                    arg-types
                    #:return-errno? return-errno?).

Pulling all this together, here is a better definition of `(math bessel)`:

(define-module (math bessel)
  #:use-module (system foreign)
  #:use-module (system foreign-library)
  #:export (j0))

(define j0
  (foreign-library-function "libm" "j0"
                            #:return-type double
                            #:arg-types (list double)))

That’s it! No C at all.

Before going on to more detailed examples, the next two sections discuss how to deal with data that is more complex than, say, `int8`. See [More Foreign Functions](06_19_foreign_function_interface.md#6198-more-foreign-functions), to continue with foreign function examples.

* * *

Next: [Foreign Structs](06_19_foreign_function_interface.md#6197-foreign-structs), Previous: [Foreign Functions](06_19_foreign_function_interface.md#6195-foreign-functions), Up: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.19.6 Void Pointers and Byte Access [¶](06_19_foreign_function_interface.md#6196-void-pointers-and-byte-access)

Wrapped pointers are untyped, so they are essentially equivalent to C `void` pointers. As in C, the memory region pointed to by a pointer can be accessed at the byte level. This is achieved using _bytevectors_ (see [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)). The `(rnrs bytevectors)` module contains procedures that can be used to convert byte sequences to Scheme objects such as strings, floating point numbers, or integers.

Load the `(system foreign)` module to use these Scheme interfaces.

(use-modules (system foreign))

Scheme Procedure: **pointer->bytevector** pointer len \[offset \[uvec\_type\]\] [¶](06_19_foreign_function_interface.md)

C Function: **scm\_pointer\_to\_bytevector** (pointer, len, offset, uvec\_type) [¶](06_19_foreign_function_interface.md)

Return a bytevector aliasing the len bytes pointed to by pointer.

The user may specify an alternate default interpretation for the memory by passing the uvec\_type argument, to indicate that the memory is an array of elements of that type. uvec\_type should be something that `array-type` would return, like `f32` or `s16`.

When offset is passed, it specifies the offset in bytes relative to pointer of the memory region aliased by the returned bytevector.

Mutating the returned bytevector mutates the memory pointed to by pointer, so buckle your seatbelts.

Scheme Procedure: **bytevector->pointer** bv \[offset\] [¶](06_19_foreign_function_interface.md)

C Function: **scm\_bytevector\_to\_pointer** (bv, offset) [¶](06_19_foreign_function_interface.md)

Return a pointer aliasing the memory pointed to by bv or offset bytes after bv when offset is passed.

In addition to these primitives, convenience procedures are available:

Scheme Procedure: **dereference-pointer** pointer [¶](06_19_foreign_function_interface.md)

Assuming pointer points to a memory region that holds a pointer, return this pointer.

Scheme Procedure: **string->pointer** string \[encoding\] [¶](06_19_foreign_function_interface.md)

Return a foreign pointer to a nul-terminated copy of string in the given encoding, defaulting to the current locale encoding. The C string is freed when the returned foreign pointer becomes unreachable.

This is the Scheme equivalent of `scm_to_stringn`.

Scheme Procedure: **pointer->string** pointer \[length\] \[encoding\] [¶](06_19_foreign_function_interface.md)

Return the string representing the C string pointed to by pointer. If length is omitted or `-1`, the string is assumed to be nul-terminated. Otherwise length is the number of bytes in memory pointed to by pointer. The C string is assumed to be in the given encoding, defaulting to the current locale encoding.

This is the Scheme equivalent of `scm_from_stringn`.

Most object-oriented C libraries use pointers to specific data structures to identify objects. It is useful in such cases to reify the different pointer types as disjoint Scheme types. The `define-wrapped-pointer-type` macro simplifies this.

Scheme Syntax: **define-wrapped-pointer-type** type-name pred wrap unwrap print [¶](06_19_foreign_function_interface.md)

Define helper procedures to wrap pointer objects into Scheme objects with a disjoint type. Specifically, this macro defines:

*   pred, a predicate for the new Scheme type;
*   wrap, a procedure that takes a pointer object and returns an object that satisfies pred;
*   unwrap, which does the reverse.

wrap preserves pointer identity, for two pointer objects p1 and p2 that are `equal?`, `(eq? (wrap p1) (wrap p2)) ⇒ #t`.

Finally, print should name a user-defined procedure to print such objects. The procedure is passed the wrapped object and a port to write to.

For example, assume we are wrapping a C library that defines a type, `bottle_t`, and functions that can be passed `bottle_t *` pointers to manipulate them. We could write:

(define-wrapped-pointer-type bottle
  bottle?
  wrap-bottle unwrap-bottle
  (lambda (b p)
    (format p "#<bottle of ~a ~x>"
            (bottle-contents b)
            (pointer-address (unwrap-bottle b)))))

(define grab-bottle
  ;; Wrapper for \`bottle\_t \*grab (void)'.
  (let ((grab (foreign-library-function libbottle "grab\_bottle"
                                        #:return-type '\*)))
    (lambda ()
      "Return a new bottle."
      (wrap-bottle (grab)))))

(define bottle-contents
  ;; Wrapper for \`const char \*bottle\_contents (bottle\_t \*)'.
  (let ((contents (foreign-library-function libbottle "bottle\_contents"
                                            #:return-type '\*
                                            #:arg-types  '(\*))))
    (lambda (b)
      "Return the contents of B."
      (pointer->string (contents (unwrap-bottle b))))))

(write (grab-bottle))
⇒ #<bottle of Château Haut-Brion 803d36>

In this example, `grab-bottle` is guaranteed to return a genuine `bottle` object satisfying `bottle?`. Likewise, `bottle-contents` errors out when its argument is not a genuine `bottle` object.

As another example, currently Guile has a variable, `scm_numptob`, as part of its API. It is declared as a C `long`. So, to read its value, we can do:

(use-modules (system foreign))
(use-modules (rnrs bytevectors))
(define numptob
  (foreign-library-pointer #f "scm\_numptob"))
numptob
(bytevector-uint-ref (pointer->bytevector numptob (sizeof long))
                     0 (native-endianness)
                     (sizeof long))
⇒ 8

If we wanted to corrupt Guile’s internal state, we could set `scm_numptob` to another value; but we shouldn’t, because that variable is not meant to be set. Indeed this point applies more widely: the C API is a dangerous place to be. Not only might setting a value crash your program, simply accessing the data pointed to by a dangling pointer or similar can prove equally disastrous.

* * *

Next: [More Foreign Functions](06_19_foreign_function_interface.md#6198-more-foreign-functions), Previous: [Void Pointers and Byte Access](06_19_foreign_function_interface.md#6196-void-pointers-and-byte-access), Up: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.19.7 Foreign Structs [¶](06_19_foreign_function_interface.md#6197-foreign-structs)

Finally, one last note on foreign values before moving on to actually calling foreign functions. Sometimes you need to deal with C structs, which requires interpreting each element of the struct according to the its type, offset, and alignment. The `(system foreign)` module has some primitives to support this.

(use-modules (system foreign))

Scheme Procedure: **sizeof** type [¶](06_19_foreign_function_interface.md)

C Function: **scm\_sizeof** (type) [¶](06_19_foreign_function_interface.md)

Return the size of type, in bytes.

type should be a valid C type, like `int`. Alternately type may be the symbol `*`, in which case the size of a pointer is returned. type may also be a list of types, in which case the size of a `struct` with ABI-conventional packing is returned.

Scheme Procedure: **alignof** type [¶](06_19_foreign_function_interface.md)

C Function: **scm\_alignof** (type) [¶](06_19_foreign_function_interface.md)

Return the alignment of type, in bytes.

type should be a valid C type, like `int`. Alternately type may be the symbol `*`, in which case the alignment of a pointer is returned. type may also be a list of types, in which case the alignment of a `struct` with ABI-conventional packing is returned.

Guile also provides some convenience syntax to efficiently read and write C structs to and from bytevectors.

Scheme Syntax: **read-c-struct** bv offset  
((field type) …) k [¶](06_19_foreign_function_interface.md)

Read a C struct with fields of type type... from the bytevector bv, at offset offset. Bind the fields to the identifiers field..., and return `(k field ...)`.

Unless cross-compiling, the field types are evaluated at macro-expansion time. This allows the resulting bytevector accessors and size/alignment computations to be completely inlined.

Scheme Syntax: **write-c-struct** bv offset  
((field type) …) [¶](06_19_foreign_function_interface.md)

Write a C struct with fields field... of type type... to the bytevector bv, at offset offset. Return zero values.

Like `write-c-struct` above, unless cross-compiling, the field types are evaluated at macro-expansion time.

For example, to define a parser and serializer for the equivalent of a `struct { int64_t a; uint8_t b; }`, one might do this:

(use-modules (system foreign) (rnrs bytevectors))

(define-syntax-rule
    (define-serialization (reader writer) (field type) ...)
  (begin
    (define (reader bv offset)
      (read-c-struct bv offset ((field type) ...) values))
    (define (writer bv offset field ...)
      (write-c-struct bv offset ((field type) ...)))))

(define-serialization (read-struct write-struct)
  (a int64) (b uint8))

(define bv (make-bytevector (sizeof (list int64 uint8))))

(write-struct bv 0 300 43)
(call-with-values (lambda () (read-struct bv 0))
  list)
⇒ (300 43)

There is also an older interface that is mostly equivalent to `read-c-struct` and `write-c-struct`, but which uses run-time dispatch, and operates on foreign pointers instead of bytevectors.

Scheme Procedure: **parse-c-struct** foreign types [¶](06_19_foreign_function_interface.md)

Parse a foreign pointer to a C struct, returning a list of values.

`types` should be a list of C types.

Our parser and serializer example for `struct { int64_t a; uint8_t b; }` looks more like this:

(parse-c-struct (make-c-struct (list int64 uint8)
                               (list 300 43))
                (list int64 uint8))
⇒ (300 43)

As yet, Guile only has convenience routines to support conventionally-packed structs. But given the `bytevector->pointer` and `pointer->bytevector` routines, one can create and parse tightly packed structs and unions by hand. See the code for `(system foreign)` for details.

* * *

Previous: [Foreign Structs](06_19_foreign_function_interface.md#6197-foreign-structs), Up: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.19.8 More Foreign Functions [¶](06_19_foreign_function_interface.md#6198-more-foreign-functions)

It is possible to pass pointers to foreign functions, and to return them as well. In that case the type of the argument or return value should be the symbol `*`, indicating a pointer. For example, the following code makes `memcpy` available to Scheme:

(use-modules (system foreign))
(define memcpy
  (foreign-library-function #f "memcpy"
                            #:return-type '\*
                            #:arg-types (list '\* '\* size\_t)))

To invoke `memcpy`, one must pass it foreign pointers:

(use-modules (rnrs bytevectors))

(define src-bits
  (u8-list->bytevector '(0 1 2 3 4 5 6 7)))
(define src
  (bytevector->pointer src-bits))
(define dest
  (bytevector->pointer (make-bytevector 16 0)))

(memcpy dest src (bytevector-length src-bits))

(bytevector->u8-list (pointer->bytevector dest 16))
⇒ (0 1 2 3 4 5 6 7 0 0 0 0 0 0 0 0)

One may also pass structs as values, passing structs as foreign pointers. See [Foreign Structs](06_19_foreign_function_interface.md#6197-foreign-structs), for more information on how to express struct types and struct values.

“Out” arguments are passed as foreign pointers. The memory pointed to by the foreign pointer is mutated in place.

;; struct timeval {
;;      time\_t      tv\_sec;     /\* seconds \*/
;;      suseconds\_t tv\_usec;    /\* microseconds \*/
;; };
;; assuming fields are of type "long"

(define gettimeofday
  (let ((f (foreign\-library\-function #f "gettimeofday"
                                     #:return-type int
                                     #:arg-types (list '\* '\*)))
        (tv-type (list long long)))
    (lambda ()
      (let\* ((timeval (make-c-struct tv-type (list 0 0)))
             (ret (f timeval %null-pointer)))
        (if (zero? ret)
            (apply values (parse-c-struct timeval tv-type))
            (error "gettimeofday returned an error" ret))))))

(gettimeofday)    
⇒ 1270587589
⇒ 499553

As you can see, this interface to foreign functions is at a very low, somewhat dangerous level[22](99_footnotes.md).

The FFI can also work in the opposite direction: making Scheme procedures callable from C. This makes it possible to use Scheme procedures as “callbacks” expected by C function.

Scheme Procedure: **procedure->pointer** return-type proc arg-types [¶](06_19_foreign_function_interface.md)

C Function: **scm\_procedure\_to\_pointer** (return\_type, proc, arg\_types) [¶](06_19_foreign_function_interface.md)

Return a pointer to a C function of type return-type taking arguments of types arg-types (a list) and behaving as a proxy to procedure proc. Thus proc’s arity, supported argument types, and return type should match return-type and arg-types.

As an example, here’s how the C library’s `qsort` array sorting function can be made accessible to Scheme (see [`qsort`](https://doc.guix.gnu.org/libc/latest/en/libc.html#Array-Sort-Function) in The GNU C Library Reference Manual):

(define qsort!
  (let ((qsort (foreign-library-function
                #f "qsort" #:arg-types (list '\* size\_t size\_t '\*))))
    (lambda (bv compare)
      ;; Sort bytevector BV in-place according to comparison
      ;; procedure COMPARE.
      (let ((ptr (procedure->pointer int
                                     (lambda (x y)
                                       ;; X and Y are pointers so,
                                       ;; for convenience, dereference
                                       ;; them before calling COMPARE.
                                       (compare (dereference-uint8\* x)
                                                (dereference-uint8\* y)))
                                     (list '\* '\*))))
        (qsort (bytevector->pointer bv)
               (bytevector-length bv) 1 ;; we're sorting bytes
               ptr)))))

(define (dereference-uint8\* ptr)
  ;; Helper function: dereference the byte pointed to by PTR.
  (let ((b (pointer->bytevector ptr 1)))
    (bytevector-u8-ref b 0)))

(define bv
  ;; An unsorted array of bytes.
  (u8-list->bytevector '(7 1 127 3 5 4 77 2 9 0)))

;; Sort BV.
(qsort! bv (lambda (x y) (- x y)))

;; Let's see what the sorted array looks like:
(bytevector->u8-list bv)
⇒ (0 1 2 3 4 5 7 9 77 127)

And voilà!

Note that `procedure->pointer` is not supported (and not defined) on a few exotic architectures. Thus, user code may need to check `(defined? 'procedure->pointer)`. Nevertheless, it is available on many architectures, including (as of libffi 3.0.9) x86, ia64, SPARC, PowerPC, ARM, and MIPS, to name a few.

* * *

Next: [Smobs](06_21_smobs.md#621-smobs), Previous: [Foreign Function Interface](06_19_foreign_function_interface.md#619-foreign-function-interface), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
