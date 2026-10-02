### 8.13 Changing the Class of an Instance [¶](08_13_changing_the_class_of_an_instance.md#813-changing-the-class-of-an-instance)

When a redefinable class is redefined, any existing instance of the redefined class will be modified for the new class definition before the next time that any of the instance’s slots is referenced or set. GOOPS modifies each instance by calling the generic function `change-class`.

More generally, you can change the class of an existing instance at any time by invoking the generic function `change-class` with two arguments: the instance and the new class.

The default method for `change-class` decides how to implement the change of class by looking at the slot definitions for the instance’s existing class and for the new class. If the new class has slots with the same name as slots in the existing class, the values for those slots are preserved. Slots that are present only in the existing class are discarded. Slots that are present only in the new class are initialized using the corresponding slot definition’s init function (see [slot-init-function](08_08_introspection.md#881-classes)).

generic: **change-class** instance new-class [¶](08_13_changing_the_class_of_an_instance.md)

method: **change-class** (obj <object>) (new <redefinable-class>) [¶](08_13_changing_the_class_of_an_instance.md)

Modify instance obj to make it an instance of class new. obj itself must already be an instance of a redefinable class.

The value of each of obj’s slots is preserved only if a similarly named slot exists in new; any other slot values are discarded.

The slots in new that do not correspond to any of obj’s pre-existing slots are initialized according to new’s slot definitions’ init functions.

The default `change-class` method also invokes another generic function, `update-instance-for-different-class`, as the last thing that it does before returning. The applied `update-instance-for-different-class` method can make any further adjustments to new-instance that are required to complete or modify the change of class. The return value from the applied method is ignored.

generic: **update-instance-for-different-class** old-instance new-instance [¶](08_13_changing_the_class_of_an_instance.md)

A generic function that can be customized to put finishing touches to an instance whose class has just been changed. The default `update-instance-for-different-class` method does nothing.

Customized change of class behavior can be implemented by defining `change-class` methods that are specialized either by the class of the instances to be modified or by the metaclass of the new class.

* * *

Next: [GNU Free Documentation License](a_gnu_free_documentation_license.md#appendix-a-gnu-free-documentation-license), Previous: [GOOPS](08_00_goops.md#8-goops), Up: [The Guile Reference Manual](00_contents.md)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
