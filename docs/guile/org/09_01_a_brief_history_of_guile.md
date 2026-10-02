### 9.1 A Brief History of Guile [¶](09_01_a_brief_history_of_guile.md#91-a-brief-history-of-guile)

Guile is an artifact of historical processes, both as code and as a community of hackers. It is sometimes useful to know this history when hacking the source code, to know about past decisions and future directions.

Of course, the real history of Guile is written by the hackers hacking and not the writers writing, so we round up the section with a note on current status and future directions.

*   [The Emacs Thesis](09_01_a_brief_history_of_guile.md#911-the-emacs-thesis)
*   [Early Days](09_01_a_brief_history_of_guile.md#912-early-days)
*   [A Scheme of Many Maintainers](09_01_a_brief_history_of_guile.md#913-a-scheme-of-many-maintainers)
*   [A Timeline of Selected Guile Releases](09_01_a_brief_history_of_guile.md#914-a-timeline-of-selected-guile-releases)
*   [Status, or: Your Help Needed](09_01_a_brief_history_of_guile.md#915-status-or-your-help-needed)

* * *

Next: [Early Days](09_01_a_brief_history_of_guile.md#912-early-days), Up: [A Brief History of Guile](09_01_a_brief_history_of_guile.md#91-a-brief-history-of-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.1.1 The Emacs Thesis [¶](09_01_a_brief_history_of_guile.md#911-the-emacs-thesis)

The story of Guile is the story of bringing the development experience of Emacs to the mass of programs on a GNU system.

Emacs, when it was first created in its GNU form in 1984, was a new take on the problem of “how to make a program”. The Emacs thesis is that it is delightful to create composite programs based on an orthogonal kernel written in a low-level language together with a powerful, high-level extension language.

Extension languages foster extensible programs, programs which adapt readily to different users and to changing times. Proof of this can be seen in Emacs’ current and continued existence, spanning more than a quarter-century.

Besides providing for modification of a program by others, extension languages are good for _intension_ as well. Programs built in “the Emacs way” are pleasurable and easy for their authors to flesh out with the features that they need.

After the Emacs experience was appreciated more widely, a number of hackers started to consider how to spread this experience to the rest of the GNU system. It was clear that the easiest way to Emacsify a program would be to embed a shared language implementation into it.

* * *

Next: [A Scheme of Many Maintainers](09_01_a_brief_history_of_guile.md#913-a-scheme-of-many-maintainers), Previous: [The Emacs Thesis](09_01_a_brief_history_of_guile.md#911-the-emacs-thesis), Up: [A Brief History of Guile](09_01_a_brief_history_of_guile.md#91-a-brief-history-of-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.1.2 Early Days [¶](09_01_a_brief_history_of_guile.md#912-early-days)

Tom Lord was the first to fully concentrate his efforts on an embeddable language runtime, which he named “GEL”, the GNU Extension Language.

GEL was the product of converting SCM, Aubrey Jaffer’s implementation of Scheme, into something more appropriate to embedding as a library. (SCM was itself based on an implementation by George Carrette, SIOD.)

Lord managed to convince Richard Stallman to dub GEL the official extension language for the GNU project. It was a natural fit, given that Scheme was a cleaner, more modern Lisp than Emacs Lisp. Part of the argument was that eventually when GEL became more capable, it could gain the ability to execute other languages, especially Emacs Lisp.

Due to a naming conflict with another programming language, Lee Thomas suggested a new name for GEL: “Guile”. Besides being a recursive acronym, “Guile” craftily follows the naming of its ancestors, “Planner”, “Conniver”, and “Schemer”. (The latter was truncated to “Scheme” due to a 6-character file name limit on an old operating system.) Finally, “Guile” suggests “guy-ell”, or “Guy L. Steele”, who, together with Gerald Sussman, originally discovered Scheme.

Around the same time that Guile (then GEL) was readying itself for public release, another extension language was gaining in popularity, Tcl. Many developers found advantages in Tcl because of its shell-like syntax and its well-developed graphical widgets library, Tk. Also, at the time there was a large marketing push promoting Tcl as a “universal extension language”.

Richard Stallman, as the primary author of GNU Emacs, had a particular vision of what extension languages should be, and Tcl did not seem to him to be as capable as Emacs Lisp. He posted a criticism to the comp.lang.tcl newsgroup, sparking one of the internet’s legendary flamewars. As part of these discussions, retrospectively dubbed the “Tcl Wars”, he announced the Free Software Foundation’s intent to promote Guile as the extension language for the GNU project.

It is a common misconception that Guile was created as a reaction to Tcl. While it is true that the public announcement of Guile happened at the same time as the “Tcl wars”, Guile was created out of a condition that existed outside the polemic. Indeed, the need for a powerful language to bridge the gap between extension of existing applications and a more fully dynamic programming environment is still with us today.

* * *

Next: [A Timeline of Selected Guile Releases](09_01_a_brief_history_of_guile.md#914-a-timeline-of-selected-guile-releases), Previous: [Early Days](09_01_a_brief_history_of_guile.md#912-early-days), Up: [A Brief History of Guile](09_01_a_brief_history_of_guile.md#91-a-brief-history-of-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.1.3 A Scheme of Many Maintainers [¶](09_01_a_brief_history_of_guile.md#913-a-scheme-of-many-maintainers)

Surveying the field, it seems that Scheme implementations correspond with their maintainers on an N-to-1 relationship. That is to say, that those people that implement Schemes might do so on a number of occasions, but that the lifetime of a given Scheme is tied to the maintainership of one individual.

Guile is atypical in this regard.

Tom Lord maintained Guile for its first year and a half or so, corresponding to the end of 1994 through the middle of 1996. The releases made in this time constitute an arc from SCM as a standalone program to Guile as a reusable, embeddable library, but passing through a explosion of features: embedded Tcl and Tk, a toolchain for compiling and disassembling Java, addition of a C-like syntax, creation of a module system, and a start at a rich POSIX interface.

Only some of those features remain in Guile. There were ongoing tensions between providing a small, embeddable language, and one which had all of the features (e.g. a graphical toolkit) that a modern Emacs might need. In the end, as Guile gained in uptake, the development team decided to focus on depth, documentation and orthogonality rather than on breadth. This has been the focus of Guile ever since, although there is a wide range of third-party libraries for Guile.

Jim Blandy presided over that period of stabilization, in the three years until the end of 1999, when he too moved on to other projects. Since then, Guile has had a group maintainership. The first group was Maciej Stachowiak, Mikael Djurfeldt, and Marius Vollmer, with Vollmer staying on the longest. By late 2007, Marius had mostly moved on to other things, so Neil Jerram and Ludovic Courtès stepped up to take on the primary maintenance responsibility. Neil and Ludovic were joined by Andy Wingo in late 2009, allowing Neil to step away, and Mark Weaver joined shortly thereafter. After spending more than 5 years in the role, Mark stepped down as well, leaving Ludovic and Andy as the current co-maintainers of Guile as of January 2020.

Of course, a large part of the actual work on Guile has come from other contributors too numerous to mention, but without whom the world would be a poorer place.

* * *

Next: [Status, or: Your Help Needed](09_01_a_brief_history_of_guile.md#915-status-or-your-help-needed), Previous: [A Scheme of Many Maintainers](09_01_a_brief_history_of_guile.md#913-a-scheme-of-many-maintainers), Up: [A Brief History of Guile](09_01_a_brief_history_of_guile.md#91-a-brief-history-of-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.1.4 A Timeline of Selected Guile Releases [¶](09_01_a_brief_history_of_guile.md#914-a-timeline-of-selected-guile-releases)

guile-i — 4 February 1995

SCM, turned into a library.

guile-ii — 6 April 1995

A low-level module system was added. Tcl/Tk support was added, allowing extension of Scheme by Tcl or vice versa. POSIX support was improved, and there was an experimental stab at Java integration.

guile-iii — 18 August 1995

The C-like syntax, ctax, was improved, but mostly this release featured a start at the task of breaking Guile into pieces.

1.0 — 5 January 1997

`#f` was distinguished from `'()`. User-level, cooperative multi-threading was added. Source-level debugging became more useful, and programmer’s and user’s manuals were begun. The module system gained a high-level interface, which is still used today in more or less the same form.

1.1 — 16 May 1997

1.2 — 24 June 1997

Support for Tcl/Tk and ctax were split off as separate packages, and have remained there since. Guile became more compatible with SCSH, and more useful as a UNIX scripting language. Libguile could now be built as a shared library, and third-party extensions written in C became loadable via dynamic linking.

1.3.0 — 19 October 1998

Command-line editing became much more pleasant through the use of the readline library. The initial support for internationalization via multi-byte strings was removed; 10 years were to pass before proper internationalization would land again. Initial Emacs Lisp support landed, ports gained better support for file descriptors, and fluids were added.

1.3.2 — 20 August 1999

1.3.4 — 25 September 1999

1.4 — 21 June 2000

A long list of lispy features were added: hooks, Common Lisp’s `format`, optional and keyword procedure arguments, `getopt-long`, sorting, random numbers, and many other fixes and enhancements. Guile also gained an interactive debugger, interactive help, and better backtraces.

1.6 — 6 September 2002

Guile gained support for the R5RS standard, and added a number of SRFI modules. The module system was expanded with programmatic support for identifier selection and renaming. The GOOPS object system was merged into Guile core.

1.8 — 20 February 2006

Guile’s arbitrary-precision arithmetic switched to use the GMP library, and added support for exact rationals. Guile’s embedded user-space threading was removed in favor of POSIX pre-emptive threads, providing true multiprocessing. Gettext support was added, and Guile’s C API was cleaned up and orthogonalized in a massive way.

2.0 — 16 February 2010

A virtual machine was added to Guile, along with the associated compiler and toolchain. Support for internationalization was finally reimplemented, in terms of unicode, locales, and libunistring. Running Guile instances became controllable and debuggable from within Emacs, via Geiser. Guile caught up to features found in a number of other Schemes: SRFI-18 threads, module-hygienic macros, a profiler, tracer, and debugger, SSAX XML integration, bytevectors, a dynamic FFI, delimited continuations, module versions, and partial support for R6RS.

2.2 — 15 March 2017

The virtual machine and introduced in 2.0 was completely rewritten, along with much of the compiler and toolchain. This speeds up many Guile programs as well as reducing startup time and memory usage. Guile’s POSIX multithreading was improved, stacks became dynamically expandable, the ports facility gained support for non-blocking I/O.

3.0 – January 2020

Guile gained support for native code generation via a simple just-in-time (JIT) compiler, further improving the speed of its virtual machine. The compiler itself gained a number of new optimizations: inlining of top-level bindings, better closure optimization, and better unboxing of integer and floating-point values. R7RS support was added, and R6RS support improved. The exception facility (throw and catch) was rewritten in terms of SRFI-34 exception handlers.

* * *

Previous: [A Timeline of Selected Guile Releases](09_01_a_brief_history_of_guile.md#914-a-timeline-of-selected-guile-releases), Up: [A Brief History of Guile](09_01_a_brief_history_of_guile.md#91-a-brief-history-of-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 9.1.5 Status, or: Your Help Needed [¶](09_01_a_brief_history_of_guile.md#915-status-or-your-help-needed)

Guile has achieved much of what it set out to achieve, but there is much remaining to do.

There is still the old problem of bringing existing applications into a more Emacs-like experience. Guile has had some successes in this respect, but still most applications in the GNU system are without Guile integration.

Getting Guile to those applications takes an investment, the “hacktivation energy” needed to wire Guile into a program that only pays off once it is good enough to enable new kinds of behavior. This would be a great way for new hackers to contribute: take an application that you use and that you know well, think of something that it can’t yet do, and figure out a way to integrate Guile and implement that task in Guile.

With time, perhaps this exposure can reverse itself, whereby programs can run under Guile instead of vice versa, eventually resulting in the Emacsification of the entire GNU system. Indeed, this is the reason for the naming of the many Guile modules that live in the `ice-9` namespace, a nod to the fictional substance in Kurt Vonnegut’s novel, Cat’s Cradle, capable of acting as a seed crystal to crystallize the mass of software.

Implicit to this whole discussion is the idea that dynamic languages are somehow better than languages like C. While languages like C have their place, Guile’s take on this question is that yes, Scheme is more expressive than C, and more fun to write. This realization carries an imperative with it to write as much code in Scheme as possible rather than in other languages.

These days it is possible to write extensible applications almost entirely from high-level languages, through byte-code and native compilation, speed gains in the underlying hardware, and foreign call interfaces in the high-level language. Smalltalk systems are like this, as are Common Lisp-based systems. While there already are a number of pure-Guile applications out there, in the past users have still needed to drop down to C for some tasks: interfacing to system libraries that don’t have prebuilt Guile interfaces, and for some tasks requiring high performance. With the arrival of native code generation via a JIT compiler in Guile 3.0, most of these older applications can now be updated to move more C code to Scheme.

Still, even with an all-Guile application, sometimes you want to provide an opportunity for users to extend your program from a language with a syntax that is closer to C, or to Python. Another interesting idea to consider is compiling e.g. Python to Guile. It’s not that far-fetched of an idea: see for example IronPython or JRuby.

Also, there’s Emacs itself. Guile’s Emacs Lisp support has reached an excellent level of correctness, robustness, and speed. However there is still work to do to finish its integration into Emacs itself. This will give lots of exciting things to Emacs: native threads, a real object system, more sophisticated types, cleaner syntax, and access to all of the Guile extensions.

Finally, so much of the world’s computation is performed in web browsers that it makes sense to ask ourselves what the Guile-on-the-web-client story is. With the advent of WebAssembly, there may finally be a reasonable compilation target that’s present on almost all user-exposed devices. Especially with the upcoming proposals to allow for tail calls, delimited continuations, and GC-managed objects, Scheme might once again have a place in the web browser. Get to it!

* * *

Next: [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile), Previous: [A Brief History of Guile](09_01_a_brief_history_of_guile.md#91-a-brief-history-of-guile), Up: [Guile Implementation](09_00_guile_implementation.md#9-guile-implementation)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
