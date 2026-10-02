### 8.3 Instance Creation and Slot Access [¶](08_03_instance_creation_and_slot_access.md#83-instance-creation-and-slot-access)

An instance (or object) of a defined class can be created with `make`. `make` takes one mandatory parameter, which is the class of the instance to create, and a list of optional arguments that will be used to initialize the slots of the new instance. For instance the following form

(define c ([make](08_03_instance_creation_and_slot_access.md) <my-complex>))

creates a new `<my-complex>` object and binds it to the Scheme variable `c`.

generic: **make** [¶](08_03_instance_creation_and_slot_access.md)

method: **make** (class <class>) initarg … [¶](08_03_instance_creation_and_slot_access.md)

Create and return a new instance of class class, initialized using initarg ....

In theory, initarg … can have any structure that is understood by whatever methods get applied when the `initialize` generic function is applied to the newly allocated instance.

In practice, specialized `initialize` methods would normally call `(next-method)` (see [Next-method](08_06_methods_and_generic_functions.md#864-next-method)), and so eventually the standard GOOPS `initialize` methods are applied. These methods expect initargs to be a list with an even number of elements, where even-numbered elements (counting from zero) are keywords and odd-numbered elements are the corresponding values.

GOOPS processes initialization argument keywords automatically for slots whose definition includes the `#:init-keyword` option (see [init-keyword](08_04_slot_options.md#84-slot-options)). Other keyword value pairs can only be processed by an `initialize` method that is specialized for the new instance’s class. Any unprocessed keyword value pairs are ignored.

generic: **make-instance** [¶](08_03_instance_creation_and_slot_access.md)

method: **make-instance** (class <class>) initarg … [¶](08_03_instance_creation_and_slot_access.md)

`make-instance` is an alias for `make`.

The slots of the new complex number can be accessed using `slot-ref` and `slot-set!`. `slot-set!` sets the value of an object slot and `slot-ref` retrieves it.

([slot-set!](08_08_introspection.md) c 'r 10)
([slot-set!](08_08_introspection.md) c 'i 3)
([slot-ref](08_08_introspection.md) c 'r) ⇒ 10
([slot-ref](08_08_introspection.md) c 'i) ⇒ 3

The `(oop goops describe)` module provides a `describe` function that is useful for seeing all the slots of an object; it prints the slots and their values to standard output.

([describe](04_programming_in_scheme.md) c)
⊣
#<<my-complex> 401d8638> is an instance of [class](08_11_the_metaobject_protocol.md) <my-complex>
Slots are: 
     r [\=](06_06_02_numerical_data_types.md) 10
     i [\=](06_06_02_numerical_data_types.md) 3

* * *

Next: [Illustrating Slot Description](08_05_illustrating_slot_description.md#85-illustrating-slot-description), Previous: [Instance Creation and Slot Access](08_03_instance_creation_and_slot_access.md#83-instance-creation-and-slot-access), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
