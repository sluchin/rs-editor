### 8.12 Redefining a Class [¶](08_12_redefining_a_class.md#812-redefining-a-class)

Suppose that a class `<my-class>` is defined using `define-class` (see [define-class](08_02_class_definition.md#82-class-definition)), with slots that have accessor functions, and that an application has created several instances of `<my-class>` using `make` (see [make](08_03_instance_creation_and_slot_access.md#83-instance-creation-and-slot-access)). What then happens if `<my-class>` is redefined by calling `define-class` again?

*   [Redefinable Classes](08_12_redefining_a_class.md#8121-redefinable-classes)
*   [Default Class Redefinition Behavior](08_12_redefining_a_class.md#8122-default-class-redefinition-behavior)
*   [Customizing Class Redefinition](08_12_redefining_a_class.md#8123-customizing-class-redefinition)

* * *

Next: [Default Class Redefinition Behavior](08_12_redefining_a_class.md#8122-default-class-redefinition-behavior), Up: [Redefining a Class](08_12_redefining_a_class.md#812-redefining-a-class)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.12.1 Redefinable Classes [¶](08_12_redefining_a_class.md#8121-redefinable-classes)

The ability for a class to be redefined is a choice for a class author to make. By default, classes in GOOPS are _not_ redefinable. A redefinable class is an instance of `<redefinable-class>`; that is to say, a class with `<redefinable-class>` as its metaclass. Accordingly, to define a redefinable class, add `#:metaclass <redefinable-class>` to its class definition:

(define-class <foo> ()
  #:metaclass <redefinable-class>)

Note that any subclass of `<foo>` is also redefinable, without the need to explicitly pass the `#:metaclass` argument, so you only need to specify `#:metaclass` for the roots of your application’s class hierarchy.

(define-class <bar> (<foo>))
(class-of <bar>) ⇒ <redefinable-class>

Note that prior to Guile 3.0, all GOOPS classes were redefinable in theory. In practice, attempting to, for example, redefine `<class>` itself would almost certainly not do what you want. Still, redefinition is an interesting capability when building long-lived resilient systems, so GOOPS does offer this facility.

* * *

Next: [Customizing Class Redefinition](08_12_redefining_a_class.md#8123-customizing-class-redefinition), Previous: [Redefinable Classes](08_12_redefining_a_class.md#8121-redefinable-classes), Up: [Redefining a Class](08_12_redefining_a_class.md#812-redefining-a-class)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.12.2 Default Class Redefinition Behavior [¶](08_12_redefining_a_class.md#8122-default-class-redefinition-behavior)

When a class is defined using `define-class` and the class name was previously defined, by default the new binding just replaces the old binding. This is the normal behavior for `define`. However if both the old and new bindings are redefinable classes (instances of `<redefinable-class>`), then the class will be updated in place, and its instances lazily migrated over.

The way that the class is updated and the way that the instances migrate over are of course part of the meta-object protocol. However the default behavior usually suffices, and it goes as follows.

*   All existing direct instances of `<my-class>` are converted to be instances of the new class. This is achieved by preserving the values of slots that exist in both the old and new definitions, and initializing the values of new slots in the usual way (see [make](08_03_instance_creation_and_slot_access.md#83-instance-creation-and-slot-access)).
*   All existing subclasses of `<my-class>` are redefined, as though the `define-class` expressions that defined them were re-evaluated following the redefinition of `<my-class>`, and the class redefinition process described here is applied recursively to the redefined subclasses.
*   Once all of its instances and subclasses have been updated, the class metaobject previously bound to the variable `<my-class>` is no longer needed and so can be allowed to be garbage collected.

To keep things tidy, GOOPS also needs to do a little housekeeping on methods that are associated with the redefined class.

*   Slot accessor methods for slots in the old definition should be removed from their generic functions. They will be replaced by accessor methods for the slots of the new class definition.
*   Any generic function method that uses the old `<my-class>` metaobject as one of its formal parameter specializers must be updated to refer to the new `<my-class>` metaobject. (Whenever a new generic function method is defined, `define-method` adds the method to a list stored in the class metaobject for each class used as a formal parameter specializer, so it is easy to identify all the methods that must be updated when a class is redefined.)

If this class redefinition strategy strikes you as rather counter-intuitive, bear in mind that it is derived from similar behavior in other object systems such as CLOS, and that experience in those systems has shown it to be very useful in practice.

Also bear in mind that, like most of GOOPS’ default behavior, it can be customized…

* * *

Previous: [Default Class Redefinition Behavior](08_12_redefining_a_class.md#8122-default-class-redefinition-behavior), Up: [Redefining a Class](08_12_redefining_a_class.md#812-redefining-a-class)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.12.3 Customizing Class Redefinition [¶](08_12_redefining_a_class.md#8123-customizing-class-redefinition)

When `define-class` notices that a class is being redefined, it constructs the new class metaobject as usual, then invokes the `class-redefinition` generic function with the old and new classes as arguments. Therefore, if the old or new classes have metaclasses other than the default `<redefinable-class>`, class redefinition behavior can be customized by defining a `class-redefinition` method that is specialized for the relevant metaclasses.

generic: **class-redefinition** [¶](08_12_redefining_a_class.md)

Handle the class redefinition from old to new, and return the new class metaobject that should be bound to the variable specified by `define-class`’s first argument.

method: **class-redefinition** (old <top>) (new <class>) [¶](08_12_redefining_a_class.md)

Not all classes are redefinable, and not all previous bindings are classes. See [Redefinable Classes](08_12_redefining_a_class.md#8121-redefinable-classes). This default method just returns new.

method: **class-redefinition** (old <redefinable-class>) (new <redefinable-class>) [¶](08_12_redefining_a_class.md)

This method implements GOOPS’ default class redefinition behavior, as described in [Default Class Redefinition Behavior](08_12_redefining_a_class.md#8122-default-class-redefinition-behavior). Returns the metaobject for the new class definition.

The `class-redefinition` method for classes with metaclass `<redefinable-class>` calls the following generic functions, which could of course be individually customized.

generic: **remove-class-accessors!** old [¶](08_12_redefining_a_class.md)

The default `remove-class-accessors!` method removes the accessor methods of the old class from all classes which they specialize.

generic: **update-direct-method!** method old new [¶](08_12_redefining_a_class.md)

The default `update-direct-method!` method substitutes the new class for the old in all methods specialized to the old class.

generic: **update-direct-subclass!** subclass old new [¶](08_12_redefining_a_class.md)

The default `update-direct-subclass!` method invokes `class-redefinition` recursively to handle the redefinition of subclasses.

An alternative class redefinition strategy could be to leave all existing instances as instances of the old class, but accepting that the old class is now “nameless”, since its name has been taken over by the new definition. In this strategy, any existing subclasses could also be left as they are, on the understanding that they inherit from a nameless superclass.

This strategy is easily implemented in GOOPS, by defining a new metaclass, that will be used as the metaclass for all classes to which the strategy should apply, and then defining a `class-redefinition` method that is specialized for this metaclass:

(define-class <can-be-nameless> (<redefinable-class>))

(define-method (class-redefinition (old <can-be-nameless>)
                                   (new <class>))
  new)

When customization can be as easy as this, aren’t you glad that GOOPS implements the far more difficult strategy as its default!

* * *

Previous: [Redefining a Class](08_12_redefining_a_class.md#812-redefining-a-class), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
