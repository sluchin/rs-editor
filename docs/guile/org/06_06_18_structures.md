#### 6.6.18 Structures [¶](06_06_18_structures.md#6618-structures)

A _structure_ is a first class data type which holds Scheme values or C words in fields numbered 0 upwards. A _vtable_ is a structure that represents a structure type, giving field types and permissions, and an optional print function for `write` etc.

Structures are lower level than records (see [Records](06_06_17_records.md#6617-records)). Usually, when you need to represent structured data, you just want to use records. But sometimes you need to implement new kinds of structured data abstractions, and for that purpose structures are useful. Indeed, records in Guile are implemented with structures.

*   [Vtables](06_06_18_structures.md#66181-vtables)
*   [Structure Basics](06_06_18_structures.md#66182-structure-basics)
*   [Vtable Contents](06_06_18_structures.md#66183-vtable-contents)
*   [Meta-Vtables](06_06_18_structures.md#66184-meta-vtables)
*   [Vtable Example](06_06_18_structures.md#66185-vtable-example)

* * *

Next: [Structure Basics](06_06_18_structures.md#66182-structure-basics), Up: [Structures](06_06_18_structures.md#6618-structures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.18.1 Vtables [¶](06_06_18_structures.md#66181-vtables)

A vtable is a structure type, specifying its layout, and other information. A vtable is actually itself a structure, but there’s no need to worry about that initially (see [Vtable Contents](06_06_18_structures.md#66183-vtable-contents).)

Scheme Procedure: **make-vtable** fields \[print\] [¶](06_06_18_structures.md)

Create a new vtable.

fields is a string describing the fields in the structures to be created. Each field is represented by two characters, a type letter and a permissions letter, for example `"pw"`. The types are as follows.

*   `p` – a Scheme value. “p” stands for “protected” meaning it’s protected against garbage collection.
*   `u` – an arbitrary word of data (an `scm_t_bits`). At the Scheme level it’s read and written as an unsigned integer. “u” stands for “unboxed”, as it’s stored as a raw value without additional type annotations.

It used to be that the second letter for each field was a permission code, such as `w` for writable or `r` for read-only. However over time structs have become more of a raw low-level facility; access control is better implemented as a layer on top. After all, `struct-set!` is a cross-cutting operator that can bypass abstractions made by higher-level record facilities; it’s not generally safe (in the sense of abstraction-preserving) to expose `struct-set!` to “untrusted” code, even if the fields happen to be writable. Additionally, permission checks added overhead to every structure access in a way that couldn’t be optimized out, hampering the ability of structs to act as a low-level building block. For all of these reasons, all fields in Guile structs are now writable; attempting to make a read-only field will now issue a deprecation warning, and the field will be writable regardless.

(make-vtable "pw")      ;; one scheme field
(make-vtable "pwuwuw")  ;; one scheme and two unboxed fields

The optional print argument is a function called by `display` and `write` (etc) to give a printed representation of a structure created from this vtable. It’s called `(print struct port)` and should look at struct and write to port. The default print merely gives a form like ‘#<struct ADDR:ADDR>’ with a pair of machine addresses.

The following print function for example shows the two fields of its structure.

(make-vtable "pwpw"
             (lambda (struct port)
               (format port "#<~a and ~a>"
                       (struct-ref struct 0)
                       (struct-ref struct 1))))

* * *

Next: [Vtable Contents](06_06_18_structures.md#66183-vtable-contents), Previous: [Vtables](06_06_18_structures.md#66181-vtables), Up: [Structures](06_06_18_structures.md#6618-structures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.18.2 Structure Basics [¶](06_06_18_structures.md#66182-structure-basics)

This section describes the basic procedures for working with structures. `make-struct/no-tail` creates a structure, and `struct-ref` and `struct-set!` access its fields.

Scheme Procedure: **make-struct/no-tail** vtable init … [¶](06_06_18_structures.md)

Create a new structure, with layout per the given vtable (see [Vtables](06_06_18_structures.md#66181-vtables)).

The optional init… arguments are initial values for the fields of the structure. This is the only way to put values in read-only fields. If there are fewer init arguments than fields then the defaults are `#f` for a Scheme field (type `p`) or 0 for an unboxed field (type `u`).

The name is a bit strange, we admit. The reason for it is that Guile used to have a `make-struct` that took an additional argument; while we deprecate that old interface, `make-struct/no-tail` is the new name for this functionality.

For example,

(define v (make-vtable "pwpwpw"))
(define s (make-struct/no-tail v 123 "abc" 456))
(struct-ref s 0) ⇒ 123
(struct-ref s 1) ⇒ "abc"

C Function: `SCM` **scm\_make\_struct** `(SCM vtable, SCM tail_size, SCM init_list)` [¶](06_06_18_structures.md)

C Function: `SCM` **scm\_c\_make\_struct** `(SCM vtable, SCM tail_size, SCM init, ...)` [¶](06_06_18_structures.md)

C Function: `SCM` **scm\_c\_make\_structv** `(SCM vtable, SCM tail_size, size_t n_inits, scm_t_bits init[])` [¶](06_06_18_structures.md)

There are a few ways to make structures from C. `scm_make_struct` takes a list, `scm_c_make_struct` takes variable arguments terminated with SCM\_UNDEFINED, and `scm_c_make_structv` takes a packed array.

For all of these, tail\_size should be zero (as a SCM value).

Scheme Procedure: **struct?** obj [¶](06_06_18_structures.md)

C Function: **scm\_struct\_p** (obj) [¶](06_06_18_structures.md)

Return `#t` if obj is a structure, or `#f` if not.

Scheme Procedure: **struct-ref** struct n [¶](06_06_18_structures.md)

C Function: **scm\_struct\_ref** (struct, n) [¶](06_06_18_structures.md)

Return the contents of field number n in struct. The first field is number 0.

An error is thrown if n is out of range.

Scheme Procedure: **struct-set!** struct n value [¶](06_06_18_structures.md)

C Function: **scm\_struct\_set\_x** (struct, n, value) [¶](06_06_18_structures.md)

Set field number n in struct to value. The first field is number 0.

An error is thrown if n is out of range, or if the field cannot be written because it’s `r` read-only.

Unboxed fields (those with type `u`) need to be accessed with special procedures.

Scheme Procedure: **struct-ref/unboxed** struct n [¶](06_06_18_structures.md)

Scheme Procedure: **struct-set!/unboxed** struct n value [¶](06_06_18_structures.md)

C Function: **scm\_struct\_ref\_unboxed** (struct, n) [¶](06_06_18_structures.md)

C Function: **scm\_struct\_set\_x\_unboxed** (struct, n, value) [¶](06_06_18_structures.md)

Like `struct-ref` and `struct-set!`, except that these may only be used on unboxed fields. `struct-ref/unboxed` will always return a positive integer. Likewise, `struct-set!/unboxed` takes an unsigned integer as the value argument, and will signal an error otherwise.

Scheme Procedure: **struct-vtable** struct [¶](06_06_18_structures.md)

C Function: **scm\_struct\_vtable** (struct) [¶](06_06_18_structures.md)

Return the vtable that describes struct.

The vtable is effectively the type of the structure. See [Vtable Contents](06_06_18_structures.md#66183-vtable-contents), for more on vtables.

* * *

Next: [Meta-Vtables](06_06_18_structures.md#66184-meta-vtables), Previous: [Structure Basics](06_06_18_structures.md#66182-structure-basics), Up: [Structures](06_06_18_structures.md#6618-structures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.18.3 Vtable Contents [¶](06_06_18_structures.md#66183-vtable-contents)

A vtable is itself a structure. It has a specific set of fields describing various aspects of its _instances_: the structures created from a vtable. Some of the fields are internal to Guile, some of them are part of the public interface, and there may be additional fields added on by the user.

Every vtable has a field for the layout of their instances, a field for the procedure used to print its instances, and a field for the name of the vtable itself. Access to the layout and printer is exposed directly via field indexes. Access to the vtable name is exposed via accessor procedures.

Scheme Variable: **vtable-index-layout** [¶](06_06_18_structures.md)

C Macro: **scm\_vtable\_index\_layout** [¶](06_06_18_structures.md)

The field number of the layout specification in a vtable. The layout specification is a symbol like `pwpw` formed from the fields string passed to `make-vtable`, or created by `make-struct-layout` (see [Meta-Vtables](06_06_18_structures.md#66184-meta-vtables)).

(define v (make-vtable "pwpw" 0))
(struct-ref v vtable-index-layout) ⇒ pwpw

This field is read-only, since the layout of structures using a vtable cannot be changed.

Scheme Variable: **vtable-index-printer** [¶](06_06_18_structures.md)

C Macro: **scm\_vtable\_index\_printer** [¶](06_06_18_structures.md)

The field number of the printer function. This field contains `#f` if the default print function should be used.

(define (my-print-func struct port)
  ...)
(define v (make-vtable "pwpw" my-print-func))
(struct-ref v vtable-index-printer) ⇒ my-print-func

This field is writable, allowing the print function to be changed dynamically.

Scheme Procedure: **struct-vtable-name** vtable [¶](06_06_18_structures.md)

Scheme Procedure: **set-struct-vtable-name!** vtable name [¶](06_06_18_structures.md)

C Function: **scm\_struct\_vtable\_name** (vtable) [¶](06_06_18_structures.md)

C Function: **scm\_set\_struct\_vtable\_name\_x** (vtable, name) [¶](06_06_18_structures.md)

Get or set the name of vtable. name is a symbol and is used in the default print function when printing structures created from vtable.

(define v (make-vtable "pw"))
(set-struct-vtable-name! v 'my-name)

(define s (make-struct v 0))
(display s) ⊣ #<my-name b7ab3ae0:b7ab3730>

* * *

Next: [Vtable Example](06_06_18_structures.md#66185-vtable-example), Previous: [Vtable Contents](06_06_18_structures.md#66183-vtable-contents), Up: [Structures](06_06_18_structures.md#6618-structures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.18.4 Meta-Vtables [¶](06_06_18_structures.md#66184-meta-vtables)

As a structure, a vtable also has a vtable, which is also a structure. Structures, their vtables, the vtables of the vtables, and so on form a tree of structures. Making a new structure adds a leaf to the tree, and if that structure is a vtable, it may be used to create other leaves.

If you traverse up the tree of vtables, via calling `struct-vtable`, eventually you reach a root which is the vtable of itself:

scheme@(guile-user)> (current-module)
$1 = #<directory (guile-user) 221b090>
scheme@(guile-user)> (struct-vtable $1)
$2 = #<record-type module>
scheme@(guile-user)> (struct-vtable $2)
$3 = #<<standard-vtable> 12c30a0>
scheme@(guile-user)> (struct-vtable $3)
$4 = #<<standard-vtable> 12c3fa0>
scheme@(guile-user)> (struct-vtable $4)
$5 = #<<standard-vtable> 12c3fa0>
scheme@(guile-user)> <standard-vtable>
$6 = #<<standard-vtable> 12c3fa0>

In this example, we can say that `$1` is an instance of `$2`, `$2` is an instance of `$3`, `$3` is an instance of `$4`, and `$4`, strangely enough, is an instance of itself. The value bound to `$4` in this console session also bound to `<standard-vtable>` in the default environment.

Scheme Variable: **<standard-vtable>** [¶](06_06_18_structures.md)

A meta-vtable, useful for making new vtables.

All of these values are structures. All but `$1` are vtables. As `$2` is an instance of `$3`, and `$3` is a vtable, we can say that `$3` is a _meta-vtable_: a vtable that can create vtables.

With this definition, we can specify more precisely what a vtable is: a vtable is a structure made from a meta-vtable. Making a structure from a meta-vtable runs some special checks to ensure that the first field of the structure is a valid layout. Additionally, if these checks see that the layout of the child vtable contains all the required fields of a vtable, in the correct order, then the child vtable will also be a meta-table, inheriting a magical bit from the parent.

Scheme Procedure: **struct-vtable?** obj [¶](06_06_18_structures.md)

C Function: **scm\_struct\_vtable\_p** (obj) [¶](06_06_18_structures.md)

Return `#t` if obj is a vtable structure: an instance of a meta-vtable.

`<standard-vtable>` is a root of the vtable tree. (Normally there is only one root in a given Guile process, but due to some legacy interfaces there may be more than one.)

The set of required fields of a vtable is the set of fields in the `<standard-vtable>`, and is bound to `standard-vtable-fields` in the default environment. It is possible to create a meta-vtable that with additional fields in its layout, which can be used to create vtables with additional data:

scheme@(guile-user)> (struct-ref $3 vtable-index-layout)
$6 = pwuhuhpwphuhuhpwpwpw
scheme@(guile-user)> (struct-ref $4 vtable-index-layout)
$7 = pwuhuhpwphuhuh
scheme@(guile-user)> standard-vtable-fields
$8 = "pwuhuhpwphuhuh"
scheme@(guile-user)> (struct-ref $2 vtable-offset-user)
$9 = module

In this continuation of our earlier example, `$2` is a vtable that has extra fields, because its vtable, `$3`, was made from a meta-vtable with an extended layout. `vtable-offset-user` is a convenient definition that indicates the number of fields in `standard-vtable-fields`.

Scheme Variable: **standard-vtable-fields** [¶](06_06_18_structures.md)

A string containing the ordered set of fields that a vtable must have.

Scheme Variable: **vtable-offset-user** [¶](06_06_18_structures.md)

The first index in a vtable that is available for a user.

Scheme Procedure: **make-struct-layout** fields [¶](06_06_18_structures.md)

C Function: **scm\_make\_struct\_layout** (fields) [¶](06_06_18_structures.md)

Return a structure layout symbol, from a fields string. fields is as described under `make-vtable` (see [Vtables](06_06_18_structures.md#66181-vtables)). An invalid fields string is an error.

With these definitions, one can define `make-vtable` in this way:

(define\* (make-vtable fields #:optional printer)
  (make-struct/no-tail <standard-vtable>
    (make-struct-layout fields)
    printer))

* * *

Previous: [Meta-Vtables](06_06_18_structures.md#66184-meta-vtables), Up: [Structures](06_06_18_structures.md#6618-structures)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.6.18.5 Vtable Example [¶](06_06_18_structures.md#66185-vtable-example)

Let us bring these points together with an example. Consider a simple object system with single inheritance. Objects will be normal structures, and classes will be vtables with three extra class fields: the name of the class, the parent class, and the list of fields.

So, first we need a meta-vtable that allocates instances with these extra class fields.

(define <class>
  (make-vtable
   (string-append standard-vtable-fields "pwpwpw")
   (lambda (x port)
     (format port "<<class> ~a>" (class-name x)))))

(define (class? x)
  (and (struct? x)
       (eq? (struct-vtable x) <class>)))

To make a structure with a specific meta-vtable, we will use `make-struct/no-tail`, passing it the computed instance layout and printer, as with `make-vtable`, and additionally the extra three class fields.

(define (make-class name parent fields)
  (let\* ((fields (compute-fields parent fields))
         (layout (compute-layout fields)))
    (make-struct/no-tail <class>
      layout
      (lambda (x port)
        (print-instance x port))
      name
      parent
      fields)))

Instances will store their associated data in slots in the structure: as many slots as there are fields. The `compute-layout` procedure below can compute a layout, and `field-index` returns the slot corresponding to a field.

(define-syntax-rule (define-accessor name n)
  (define (name obj)
    (struct-ref obj n)))

;; Accessors for classes
(define-accessor class-name (+ vtable-offset-user 0))
(define-accessor class-parent (+ vtable-offset-user 1))
(define-accessor class-fields (+ vtable-offset-user 2))

(define (compute-fields parent fields)
  (if parent
      (append (class-fields parent) fields)
      fields))

(define (compute-layout fields)
  (make-struct-layout
   (string-concatenate (make-list (length fields) "pw"))))

(define (field-index class field)
  (list-index (class-fields class) field))

(define (print-instance x port)
  (format port "<~a" (class-name (struct-vtable x)))
  (for-each (lambda (field idx)
              (format port " ~a: ~a" field (struct-ref x idx)))
            (class-fields (struct-vtable x))
            (iota (length (class-fields (struct-vtable x)))))
  (format port ">"))

So, at this point we can actually make a few classes:

(define-syntax-rule (define-class name parent field ...)
  (define name (make-class 'name parent '(field ...))))

(define-class <surface> #f
  width height)

(define-class <window> <surface>
  x y)

And finally, make an instance:

(make-struct/no-tail <window> 400 300 10 20)
⇒ <<window> width: 400 height: 300 x: 10 y: 20>

And that’s that. Note that there are many possible optimizations and feature enhancements that can be made to this object system, and the included GOOPS system does make most of them. For more simple use cases, the records facility is usually sufficient. But sometimes you need to make new kinds of data abstractions, and for that purpose, structs are here.

* * *

Next: [Association Lists](06_06_20_association_lists.md#6620-association-lists), Previous: [Structures](06_06_18_structures.md#6618-structures), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
