8 GOOPS [¶](08_00_goops.md#8-goops)
------------------------------------------------------------------------

GOOPS is the “Guile Object Oriented Programming System”. Its implementation is derived from STk-3.99.3 by Erick Gallesio and version 1.3 of Gregor Kiczales’ Tiny-Clos. It is very close in spirit to CLOS, the Common Lisp Object System, but is adapted for the Scheme language.

GOOPS is a full object oriented system, with classes, objects, multiple inheritance, and generic functions with multi-method dispatch. Furthermore its implementation relies on a meta object protocol — which means that GOOPS’s core operations are themselves defined as methods on relevant classes, and can be customised by overriding or redefining those methods.

To start using GOOPS you first need to import the `(oop goops)` module. You can do this at the Guile REPL by evaluating:

([use-modules](06_18_modules.md) (oop goops))

*   [Copyright Notice](08_01_copyright_notice.md#81-copyright-notice)
*   [Class Definition](08_02_class_definition.md#82-class-definition)
*   [Instance Creation and Slot Access](08_03_instance_creation_and_slot_access.md#83-instance-creation-and-slot-access)
*   [Slot Options](08_04_slot_options.md#84-slot-options)
*   [Illustrating Slot Description](08_05_illustrating_slot_description.md#85-illustrating-slot-description)
*   [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)
*   [Inheritance](08_07_inheritance.md#87-inheritance)
*   [Introspection](08_08_introspection.md#88-introspection)
*   [Error Handling](08_09_error_handling.md#89-error-handling)
*   [GOOPS Object Miscellany](08_10_goops_object_miscellany.md#810-goops-object-miscellany)
*   [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)
*   [Redefining a Class](08_12_redefining_a_class.md#812-redefining-a-class)
*   [Changing the Class of an Instance](08_13_changing_the_class_of_an_instance.md#813-changing-the-class-of-an-instance)

* * *

Next: [Class Definition](08_02_class_definition.md#82-class-definition), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
