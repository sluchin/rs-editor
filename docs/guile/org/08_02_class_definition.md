### 8.2 Class Definition [¶](08_02_class_definition.md#82-class-definition)

A new class is defined with the `define-class` syntax:

(define-class [class](08_11_the_metaobject_protocol.md) (superclass [...](06_08_macros.md))
   slot-description [...](06_08_macros.md)
   class-option [...](06_08_macros.md))

class is the class being defined. The list of superclasses specifies which existing classes, if any, to inherit slots and properties from. _Slots_ hold per-instance[30](99_footnotes.md) data, for instances of that class — like “fields” or “member variables” in other object oriented systems. Each slot-description gives the name of a slot and optionally some “properties” of this slot; for example its initial value, the name of a function which will access its value, and so on. Class options, slot descriptions and inheritance are discussed more below.

syntax: **define-class** name (super …) slot-definition … class-option … [¶](08_02_class_definition.md)

Define a class called name that inherits from supers, with direct slots defined by slot-definitions and class-options. The newly created class is bound to the variable name name in the current environment.

Each slot-definition is either a symbol that names the slot or a list,

(slot-name-symbol . slot-options)

where slot-name-symbol is a symbol and slot-options is a list with an even number of elements. The even-numbered elements of slot-options (counting from zero) are slot option keywords; the odd-numbered elements are the corresponding values for those keywords.

Each class-option is an option keyword and corresponding value.

As an example, let us define a type for representing a complex number in terms of two real numbers.[31](99_footnotes.md) This can be done with the following class definition:

(define-class <my-complex> (<number>)
   r i)

This binds the variable `<my-complex>` to a new class whose instances will contain two slots. These slots are called `r` and `i` and will hold the real and imaginary parts of a complex number. Note that this class inherits from `<number>`, which is a predefined class.[32](99_footnotes.md)

Slot options are described in the next section. The possible class options are as follows.

class option: **#:metaclass** metaclass [¶](08_02_class_definition.md)

The `#:metaclass` class option specifies the metaclass of the class being defined. metaclass must be a class that inherits from `<class>`. For the use of metaclasses, see [Metaobjects and the Metaobject Protocol](08_11_the_metaobject_protocol.md#8111-metaobjects-and-the-metaobject-protocol) and [Metaclasses](08_11_the_metaobject_protocol.md#8112-metaclasses).

If the `#:metaclass` option is absent, GOOPS reuses or constructs a metaclass for the new class by calling `ensure-metaclass` (see [ensure-metaclass](08_11_the_metaobject_protocol.md#8115-class-definition-protocol)).

class option: **#:name** name [¶](08_02_class_definition.md)

The `#:name` class option specifies the new class’s name. This name is used to identify the class whenever related objects - the class itself, its instances and its subclasses - are printed.

If the `#:name` option is absent, GOOPS uses the first argument to `define-class` as the class name.

* * *

Next: [Slot Options](08_04_slot_options.md#84-slot-options), Previous: [Class Definition](08_02_class_definition.md#82-class-definition), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
