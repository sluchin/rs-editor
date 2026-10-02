### 7.19 Curried Definitions [¶](07_19_curried_definitions.md#719-curried-definitions)

The macros in this section are provided by

([use-modules](06_18_modules.md) (ice-9 curried-definitions))

and replace those provided by default.

Prior to Guile 2.0, Guile provided a type of definition known colloquially as a “curried definition”. The idea is to extend the syntax of `define` so that you can conveniently define procedures that return procedures, up to any desired depth.

For example,

(define ((foo x) y)
  (list x y))

is a convenience form of

(define foo
  (lambda (x)
    (lambda (y)
      (list x y))))

Scheme Syntax: **define** (… (name args …) …) body … [¶](07_19_curried_definitions.md)

Scheme Syntax: **define\*** (… (name args …) …) body … [¶](07_19_curried_definitions.md)

Scheme Syntax: **define-public** (… (name args …) …) body … [¶](07_19_curried_definitions.md)

Create a top level variable name bound to the procedure with parameter list args. If name is itself a formal parameter list, then a higher order procedure is created using that formal-parameter list, and returning a procedure that has parameter list args. This nesting may occur to arbitrary depth.

`define*` is similar but the formal parameter lists take additional options as described in [lambda\* and define\*.](06_07_procedures.md#6741-lambda-and-define). For example,

(define\* ((foo #:keys (bar 'baz) (quux 'zot)) frotz #:rest rest)
  (list bar quux frotz rest))

((foo #:quux 'foo) 1 2 3 4 5)
⇒ (baz foo 1 (2 3 4 5))

`define-public` is similar to `define` but it also adds name to the list of exported bindings of the current module.

* * *

Next: [SXML](07_21_sxml.md#721-sxml), Previous: [Curried Definitions](07_19_curried_definitions.md#719-curried-definitions), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
