### 8.10 GOOPS Object Miscellany [¶](08_10_goops_object_miscellany.md#810-goops-object-miscellany)

Here we cover some points about GOOPS objects that aren’t substantial enough to merit sections on their own.

#### Object Equality [¶](08_10_goops_object_miscellany.md#object-equality)

When GOOPS is loaded, `eqv?`, `equal?` and `=` become generic functions, and you can define methods for them, specialized for your own classes, so as to control what the various kinds of equality mean for your classes.

For example, the `assoc` procedure, for looking up an entry in an alist, is specified as using `equal?` to determine when the car of an entry in the alist is the same as the key parameter that `assoc` is called with. Hence, if you had defined a new class, and wanted to use instances of that class as the keys in an alist, you could define a method for `equal?`, for your class, to control `assoc`’s lookup precisely.

#### Cloning Objects [¶](08_10_goops_object_miscellany.md#cloning-objects)

generic: **shallow-clone** [¶](08_10_goops_object_miscellany.md)

method: **shallow-clone** (self <object>) [¶](08_10_goops_object_miscellany.md)

Return a “shallow” clone of self. The default method makes a shallow clone by allocating a new instance and copying slot values from self to the new instance. Each slot value is copied either as an immediate value or by reference.

generic: **deep-clone** [¶](08_10_goops_object_miscellany.md)

method: **deep-clone** (self <object>) [¶](08_10_goops_object_miscellany.md)

Return a “deep” clone of self. The default method makes a deep clone by allocating a new instance and copying or cloning slot values from self to the new instance. If a slot value is an instance (satisfies `instance?`), it is cloned by calling `deep-clone` on that value. Other slot values are copied either as immediate values or by reference.

#### Write and Display [¶](08_10_goops_object_miscellany.md#write-and-display)

primitive generic: **write** object port [¶](08_10_goops_object_miscellany.md)

primitive generic: **display** object port [¶](08_10_goops_object_miscellany.md)

When GOOPS is loaded, `write` and `display` become generic functions with special methods for printing

*   objects - instances of the class `<object>`
*   foreign objects - instances of the class `<foreign-object>`
*   classes - instances of the class `<class>`
*   generic functions - instances of the class `<generic>`
*   methods - instances of the class `<method>`.

`write` and `display` print non-GOOPS values in the same way as the Guile primitive `write` and `display` functions.

In addition to the cases mentioned, you can of course define `write` and `display` methods for your own classes, to customize how instances of those classes are printed.

* * *

Next: [Redefining a Class](08_12_redefining_a_class.md#812-redefining-a-class), Previous: [GOOPS Object Miscellany](08_10_goops_object_miscellany.md#810-goops-object-miscellany), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
