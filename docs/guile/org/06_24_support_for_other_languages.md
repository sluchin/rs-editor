### 6.24 Support for Other Languages [¶](06_24_support_for_other_languages.md#624-support-for-other-languages)

In addition to Scheme, a user may write a Guile program in an increasing number of other languages. Currently supported languages include Emacs Lisp and ECMAScript.

Guile is still fundamentally a Scheme, but it tries to support a wide variety of language building-blocks, so that other languages can be implemented on top of Guile. This allows users to write or extend applications in languages other than Scheme, too. This section describes the languages that have been implemented.

(For details on how to implement a language, See [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine).)

*   [Using Other Languages](06_24_support_for_other_languages.md#6241-using-other-languages)
*   [Emacs Lisp](06_24_support_for_other_languages.md#6242-emacs-lisp)
*   [ECMAScript](06_24_support_for_other_languages.md#6243-ecmascript)

* * *

Next: [Emacs Lisp](06_24_support_for_other_languages.md#6242-emacs-lisp), Up: [Support for Other Languages](06_24_support_for_other_languages.md#624-support-for-other-languages)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.24.1 Using Other Languages [¶](06_24_support_for_other_languages.md#6241-using-other-languages)

There are currently only two ways to access other languages from within Guile: at the REPL, and programmatically, via `compile`, `read-and-compile`, and `compile-file`.

The REPL is Guile’s command prompt (see [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively)). The REPL has a concept of the “current language”, which defaults to Scheme. The user may change that language, via the meta-command `,language`.

For example, the following meta-command enables Emacs Lisp input:

scheme@(guile-user)> ,language elisp
Happy hacking with Emacs Lisp!  To switch back, type \`,L scheme'.
elisp@(guile-user)> (eq 1 2)
$1 = #nil

Each language has its short name: for example, `elisp`, for Elisp. The same short name may be used to compile source code programmatically, via `compile`:

elisp@(guile-user)> ,L scheme
Happy hacking with Guile Scheme!  To switch back, type \`,L elisp'.
scheme@(guile-user)> (compile '(eq 1 2) #:from 'elisp)
$2 = #nil

Granted, as the input to `compile` is a datum, this works best for Lispy languages, which have a straightforward datum representation. Other languages that need more parsing are better dealt with as strings.

The easiest way to deal with syntax-heavy language is with files, via `compile-file` and friends. However it is possible to invoke a language’s reader on a port, and then compile the resulting expression (which is a datum at that point). For more information, See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code).

For more details on introspecting aspects of different languages, See [Compiler Tower](09_04_compiling_to_the_virtual_machine.md#941-compiler-tower).

* * *

Next: [ECMAScript](06_24_support_for_other_languages.md#6243-ecmascript), Previous: [Using Other Languages](06_24_support_for_other_languages.md#6241-using-other-languages), Up: [Support for Other Languages](06_24_support_for_other_languages.md#624-support-for-other-languages)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.24.2 Emacs Lisp [¶](06_24_support_for_other_languages.md#6242-emacs-lisp)

Emacs Lisp (Elisp) is a dynamically-scoped Lisp dialect used in the Emacs editor. See [Overview](https://www.gnu.org/software/emacs/manual/html_mono/elisp.html#Top) in Emacs Lisp, for more information on Emacs Lisp.

We hope that eventually Guile’s implementation of Elisp will be good enough to replace Emacs’ own implementation of Elisp. For that reason, we have thought long and hard about how to support the various features of Elisp in a performant and compatible manner.

Readers familiar with Emacs Lisp might be curious about how exactly these various Elisp features are supported in Guile. The rest of this section focuses on addressing these concerns of the Elisp elect.

*   [Nil](06_24_support_for_other_languages.md#62421-nil)
*   [Dynamic Binding](06_24_support_for_other_languages.md#62422-dynamic-binding)
*   [Other Elisp Features](06_24_support_for_other_languages.md#62423-other-elisp-features)

* * *

Next: [Dynamic Binding](06_24_support_for_other_languages.md#62422-dynamic-binding), Up: [Emacs Lisp](06_24_support_for_other_languages.md#6242-emacs-lisp)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.24.2.1 Nil [¶](06_24_support_for_other_languages.md#62421-nil)

`nil` in ELisp is an amalgam of Scheme’s `#f` and `'()`. It is false, and it is the end-of-list; thus it is a boolean, and a list as well.

Guile has chosen to support `nil` as a separate value, distinct from `#f` and `'()`. This allows existing Scheme and Elisp code to maintain their current semantics. `nil`, which in Elisp would just be written and read as `nil`, in Scheme has the external representation `#nil`.

In Elisp code, `#nil`, `#f`, and `'()` behave like `nil`, in the sense that they are all interpreted as `nil` by Elisp `if`, `cond`, `when`, `not`, `null`, etc. To test whether Elisp would interpret an object as `nil` from within Scheme code, use `nil?`:

Scheme Procedure: **nil?** obj [¶](06_24_support_for_other_languages.md)

Return `#t` if obj would be interpreted as `nil` by Emacs Lisp code, else return `#f`.

([nil?](06_24_support_for_other_languages.md) #nil) ⇒ #t
([nil?](06_24_support_for_other_languages.md) #f)   ⇒ #t
([nil?](06_24_support_for_other_languages.md) '())  ⇒ #t
([nil?](06_24_support_for_other_languages.md) 3)    ⇒ #f

This decision to have `nil` as a low-level distinct value facilitates interoperability between the two languages. Guile has chosen to have Scheme deal with `nil` as follows:

(boolean? #nil) ⇒ #t
(not #nil) ⇒ #t
(null? #nil) ⇒ #t

And in C, one has:

scm\_is\_bool (SCM\_ELISP\_NIL) ⇒ 1
scm\_is\_false (SCM\_ELISP\_NIL) ⇒ 1
scm\_is\_null (SCM\_ELISP\_NIL) ⇒ 1

In this way, a version of `fold` written in Scheme can correctly fold a function written in Elisp (or in fact any other language) over a nil-terminated list, as Elisp makes. The converse holds as well; a version of `fold` written in Elisp can fold over a `'()`\-terminated list, as made by Scheme.

On a low level, the bit representations for `#f`, `#t`, `nil`, and `'()` are made in such a way that they differ by only one bit, and so a test for, for example, `#f`\-or-`nil` may be made very efficiently. See `libguile/boolean.h`, for more information.

#### Equality [¶](06_24_support_for_other_languages.md#equality)

Since Scheme’s `equal?` must be transitive, and `'()` is not `equal?` to `#f`, to Scheme `nil` is not `equal?` to `#f` or `'()`.

(eq? #f '()) ⇒ #f
(eq? #nil '()) ⇒ #f
(eq? #nil #f) ⇒ #f
(eqv? #f '()) ⇒ #f
(eqv? #nil '()) ⇒ #f
(eqv? #nil #f) ⇒ #f
(equal? #f '()) ⇒ #f
(equal? #nil '()) ⇒ #f
(equal? #nil #f) ⇒ #f

However, in Elisp, `'()`, `#f`, and `nil` are all `equal` (though not `eq`).

(defvar f (make-scheme-false))
(defvar eol (make-scheme-null))
(eq f eol) ⇒ nil
(eq nil eol) ⇒ nil
(eq nil f) ⇒ nil
(equal f eol) ⇒ t
(equal nil eol) ⇒ t
(equal nil f) ⇒ t

These choices facilitate interoperability between Elisp and Scheme code, but they are not perfect. Some code that is correct standard Scheme is not correct in the presence of a second false and null value. For example:

(define (truthiness x)
  (if (eq? x #f)
      #f
      #t))

This code seems to be meant to test a value for truth, but now that there are two false values, `#f` and `nil`, it is no longer correct.

Similarly, there is the loop:

(define (my-length l)
  (let lp ((l l) (len 0))
    (if (eq? l '())
        len
        (lp (cdr l) (1+ len)))))

Here, `my-length` will raise an error if l is a `nil`\-terminated list.

Both of these examples are correct standard Scheme, but, depending on what they really want to do, they are not correct Guile Scheme. Correctly written, they would test the _properties_ of falsehood or nullity, not the individual members of that set. That is to say, they should use `not` or `null?` to test for falsehood or nullity, not `eq?` or `memv` or the like.

Fortunately, using `not` and `null?` is in good style, so all well-written standard Scheme programs are correct, in Guile Scheme.

Here are correct versions of the above examples:

(define (truthiness\* x)
  (if (not x)
      #f
      #t))
;; or: (define (t\* x) (not (not x)))
;; or: (define (t\*\* x) x)

(define (my-length\* l)
  (let lp ((l l) (len 0))
    (if (null? l)
        len
        (lp (cdr l) (1+ len)))))

This problem has a mirror-image case in Elisp:

(defun my-falsep (x)
  (if (eq x nil)
      t
      nil))

Guile can warn when compiling code that has equality comparisons with `#f`, `'()`, or `nil`. See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), for details.

* * *

Next: [Other Elisp Features](06_24_support_for_other_languages.md#62423-other-elisp-features), Previous: [Nil](06_24_support_for_other_languages.md#62421-nil), Up: [Emacs Lisp](06_24_support_for_other_languages.md#6242-emacs-lisp)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.24.2.2 Dynamic Binding [¶](06_24_support_for_other_languages.md#62422-dynamic-binding)

In contrast to Scheme, which uses “lexical scoping”, Emacs Lisp scopes its variables dynamically. Guile supports dynamic scoping with its “fluids” facility. See [Fluids and Dynamic States](06_11_controlling_the_flow_of_program_execution.md#61111-fluids-and-dynamic-states), for more information.

* * *

Previous: [Dynamic Binding](06_24_support_for_other_languages.md#62422-dynamic-binding), Up: [Emacs Lisp](06_24_support_for_other_languages.md#6242-emacs-lisp)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.24.2.3 Other Elisp Features [¶](06_24_support_for_other_languages.md#62423-other-elisp-features)

Buffer-local and mode-local variables should be mentioned here, along with buckybits on characters, Emacs primitive data types, the Lisp-2-ness of Elisp, and other things. Contributions to the documentation are most welcome!

* * *

Previous: [Emacs Lisp](06_24_support_for_other_languages.md#6242-emacs-lisp), Up: [Support for Other Languages](06_24_support_for_other_languages.md#624-support-for-other-languages)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.24.3 ECMAScript [¶](06_24_support_for_other_languages.md#6243-ecmascript)

[ECMAScript](http://www.ecma-international.org/publications/files/ECMA-ST/Ecma-262.pdf) was not the first non-Schemey language implemented by Guile, but it was the first implemented for Guile’s bytecode compiler. The goal was to support ECMAScript version 3.1, a relatively small language, but the implementer was completely irresponsible and got distracted by other things before finishing the standard library, and even some bits of the syntax. So, ECMAScript does deserve a mention in the manual, but it doesn’t deserve an endorsement until its implementation is completed, perhaps by some more responsible hacker.

In the meantime, the charitable user might investigate such invocations as `,L ecmascript` and `cat test-suite/tests/ecmascript.test`.

* * *

Next: [Debugging Infrastructure](06_26_debugging_infrastructure.md#626-debugging-infrastructure), Previous: [Support for Other Languages](06_24_support_for_other_languages.md#624-support-for-other-languages), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
