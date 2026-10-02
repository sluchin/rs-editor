### 8.4 Slot Options [¶](08_04_slot_options.md#84-slot-options)

When specifying a slot (in a `(define-class …)` form), various options can be specified in addition to the slot’s name. Each option is specified by a keyword. The list of possible keywords is as follows.

slot option: **#:init-value** init-value [¶](08_04_slot_options.md)

slot option: **#:init-form** init-form [¶](08_04_slot_options.md)

slot option: **#:init-thunk** init-thunk [¶](08_04_slot_options.md)

slot option: **#:init-keyword** init-keyword [¶](08_04_slot_options.md)

These options provide various ways to specify how to initialize the slot’s value at instance creation time.

init-value specifies a fixed initial slot value (shared across all new instances of the class).

init-thunk specifies a thunk that will provide a default value for the slot. The thunk is called when a new instance is created and should return the desired initial slot value.

init-form specifies a form that, when evaluated, will return an initial value for the slot. The form is evaluated each time that an instance of the class is created, in the lexical environment of the containing `define-class` expression.

init-keyword specifies a keyword that can be used to pass an initial slot value to `make` when creating a new instance.

Note that, since an `init-value` value is shared across all instances of a class, you should only use it when the initial value is an immutable value, like a constant. If you want to initialize a slot with a fresh, independently mutable value, you should use `init-thunk` or `init-form` instead. Consider the following example.

(define-class <chbouib> ()
  (hashtab #:init-value (make-hash-table)))

Here only one hash table is created and all instances of `<chbouib>` have their `hashtab` slot refer to it. In order to have each instance of `<chbouib>` refer to a new hash table, you should instead write:

(define-class <chbouib> ()
  (hashtab #:init-thunk make-hash-table))

or:

(define-class <chbouib> ()
  (hashtab #:init-form (make-hash-table)))

If more than one of these options is specified for the same slot, the order of precedence, highest first is

*   `#:init-keyword`, if init-keyword is present in the options passed to `make`
*   `#:init-thunk`, `#:init-form` or `#:init-value`.

If the slot definition contains more than one initialization option of the same precedence, the later ones are ignored. If a slot is not initialized at all, its value is unbound.

In general, slots that are shared between more than one instance are only initialized at new instance creation time if the slot value is unbound at that time. However, if the new instance creation specifies a valid init keyword and value for a shared slot, the slot is re-initialized regardless of its previous value.

Note, however, that the power of GOOPS’ metaobject protocol means that everything written here may be customized or overridden for particular classes! The slot initializations described here are performed by the least specialized method of the generic function `initialize`, whose signature is

(define-method (initialize (object <object>) initargs) ...)

The initialization of instances of any given class can be customized by defining a `initialize` method that is specialized for that class, and the author of the specialized method may decide to call `next-method` - which will result in a call to the next less specialized `initialize` method - at any point within the specialized code, or maybe not at all. In general, therefore, the initialization mechanisms described here may be modified or overridden by more specialized code, or may not be supported at all for particular classes.

slot option: **#:getter** getter [¶](08_04_slot_options.md)

slot option: **#:setter** setter [¶](08_04_slot_options.md)

slot option: **#:accessor** accessor [¶](08_04_slot_options.md)

Given an object obj with slots named `foo` and `bar`, it is always possible to read and write those slots by calling `slot-ref` and `slot-set!` with the relevant slot name; for example:

(slot-ref obj 'foo)
(slot-set! obj 'bar 25)

The `#:getter`, `#:setter` and `#:accessor` options, if present, tell GOOPS to create generic function and method definitions that can be used to get and set the slot value more conveniently. getter specifies a generic function to which GOOPS will add a method for getting the slot value. setter specifies a generic function to which GOOPS will add a method for setting the slot value. accessor specifies an accessor to which GOOPS will add methods for both getting and setting the slot value.

So if a class includes a slot definition like this:

(c #:getter get-count #:setter set-count #:accessor count)

GOOPS defines generic function methods such that the slot value can be referenced using either the getter or the accessor -

(let ((current-count (get-count obj))) ...)
(let ((current-count (count obj))) ...)

\- and set using either the setter or the accessor -

(set-count obj (+ 1 current-count))
(set! (count obj) (+ 1 current-count))

Note that

*   with an accessor, the slot value is set using the generalized `set!` syntax
*   in practice, it is unusual for a slot to use all three of these options: read-only, write-only and read-write slots would typically use only `#:getter`, `#:setter` and `#:accessor` options respectively.

The binding of the specified names is done in the environment of the `define-class` expression. If the names are already bound (in that environment) to values that cannot be upgraded to generic functions, those values are overwritten when the `define-class` expression is evaluated. For more detail, see [ensure-generic](08_11_the_metaobject_protocol.md#8119-generic-function-internals).

slot option: **#:allocation** allocation [¶](08_04_slot_options.md)

The `#:allocation` option tells GOOPS how to allocate storage for the slot. Possible values for allocation are

*   `#:instance`
    
    Indicates that GOOPS should create separate storage for this slot in each new instance of the containing class (and its subclasses). This is the default.
    
*   `#:class`
    
    Indicates that GOOPS should create storage for this slot that is shared by all instances of the containing class (and its subclasses). In other words, a slot in class C with allocation `#:class` is shared by all instances for which `(is-a? instance c)`. This permits defining a kind of global variable which can be accessed only by (in)direct instances of the class which defines the slot.
    
*   `#:each-subclass`
    
    Indicates that GOOPS should create storage for this slot that is shared by all _direct_ instances of the containing class, and that whenever a subclass of the containing class is defined, GOOPS should create a new storage for the slot that is shared by all _direct_ instances of the subclass. In other words, a slot with allocation `#:each-subclass` is shared by all instances with the same `class-of`.
    
*   `#:virtual`
    
    Indicates that GOOPS should not allocate storage for this slot. The slot definition must also include the `#:slot-ref` and `#:slot-set!` options to specify how to reference and set the value for this slot. See the example below.
    

Slot allocation options are processed when defining a new class by the generic function `compute-get-n-set`, which is specialized by the class’s metaclass. Hence new types of slot allocation can be implemented by defining a new metaclass and a method for `compute-get-n-set` that is specialized for the new metaclass. For an example of how to do this, see [Customizing Class Definition](08_11_the_metaobject_protocol.md#8116-customizing-class-definition).

slot option: **#:slot-ref** getter [¶](08_04_slot_options.md)

slot option: **#:slot-set!** setter [¶](08_04_slot_options.md)

The `#:slot-ref` and `#:slot-set!` options must be specified if the slot allocation is `#:virtual`, and are ignored otherwise.

getter should be a closure taking a single instance parameter that returns the current slot value. setter should be a closure taking two parameters - instance and new-val - that sets the slot value to new-val.

* * *

Next: [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions), Previous: [Slot Options](08_04_slot_options.md#84-slot-options), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
