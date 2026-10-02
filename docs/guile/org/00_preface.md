Preface [¶](00_preface.md#preface)
--------------------------------------------------------------------------

This manual describes how to use Guile, GNU’s Ubiquitous Intelligent Language for Extensions. It relates particularly to Guile version 3.0.11.

*   [Contributors to this Manual](00_preface.md#contributors-to-this-manual)
*   [The Guile License](00_preface.md#the-guile-license)

* * *

Next: [The Guile License](00_preface.md#the-guile-license), Up: [Preface](00_preface.md#preface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### Contributors to this Manual [¶](00_preface.md#contributors-to-this-manual)

Like Guile itself, the Guile reference manual is a living entity, cared for by many people over a long period of time. As such, it is hard to identify individuals of whom to say “yes, this single person wrote the manual.”

Still, among the many contributions, some caretakers stand out. First among them is Neil Jerram, who has worked on this document for over ten years. Neil’s attention both to detail and to the big picture have made a real difference in the understanding of a generation of Guile hackers.

Next we should note Marius Vollmer’s effect on this document. Marius maintained Guile during a period in which Guile’s API was clarified—put to the fire, so to speak—and he had the good sense to effect the same change on the manual.

Martin Grabmueller made substantial contributions throughout the manual in preparation for the Guile 1.6 release, including filling out a lot of the documentation of Scheme data types, control mechanisms and procedures. In addition, he wrote the documentation for Guile’s SRFI modules and modules associated with the Guile REPL.

Ludovic Courtès and Andy Wingo, who co-maintain Guile since 2010, along with Mark Weaver, have also made their dent in the manual, writing documentation for new modules and subsystems that arrived with Guile 2.0. Ludovic, Andy, and Mark are also responsible for ensuring that the existing text retains its relevance as Guile evolves. See [Reporting Bugs](02_hello_guile.md#26-reporting-bugs), for more information on reporting problems in this manual.

The content for the first versions of this manual incorporated and was inspired by documents from Aubrey Jaffer, author of the SCM system on which Guile was based, and from Tom Lord, Guile’s first maintainer. Although most of this text has been rewritten, all of it was important, and some of the structure remains.

The manual for the first versions of Guile were largely written, edited, and compiled by Mark Galassi and Jim Blandy. In particular, Jim wrote the original tutorial on Guile’s data representation and the C API for accessing Guile objects.

Significant portions were also contributed by Thien-Thi Nguyen, Kevin Ryde, Mikael Djurfeldt, Christian Lynbech, Julian Graham, Gary Houston, Tim Pierce, and a few dozen more. You, reader, are most welcome to join their esteemed ranks. Visit Guile’s web site at [http://www.gnu.org/software/guile/](http://www.gnu.org/software/guile/) to find out how to get involved.

* * *

Previous: [Contributors to this Manual](00_preface.md#contributors-to-this-manual), Up: [Preface](00_preface.md#preface)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### The Guile License [¶](00_preface.md#the-guile-license)

Guile is Free Software. Guile is copyrighted, not public domain, and there are restrictions on its distribution or redistribution, but these restrictions are designed to permit everything a cooperating person would want to do.

*   The Guile library (libguile) and supporting files are published under the terms of the GNU Lesser General Public License version 3 or later. See the files COPYING.LESSER and COPYING.
*   The Guile readline module is published under the terms of the GNU General Public License version 3 or later. See the file COPYING.
*   The manual you’re now reading is published under the terms of the GNU Free Documentation License (see [GNU Free Documentation License](a_gnu_free_documentation_license.md#appendix-a-gnu-free-documentation-license)).

C code linking to the Guile library is subject to terms of that library. Basically such code may be published on any terms, provided users can re-link against a new or modified version of Guile.

C code linking to the Guile readline module is subject to the terms of that module. Basically such code must be published on Free terms.

Scheme level code written to be run by Guile (but not derived from Guile itself) is not restricted in any way, and may be published on any terms. We encourage authors to publish on Free terms.

You must be aware there is no warranty whatsoever for Guile. This is described in full in the licenses.

* * *

Next: [Hello Guile!](02_hello_guile.md#2-hello-guile), Previous: [Preface](00_preface.md#preface), Up: [The Guile Reference Manual](00_contents.md)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
