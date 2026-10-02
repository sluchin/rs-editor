### 8.8 Introspection [¶](08_08_introspection.md#88-introspection)

_Introspection_, or _reflection_, means being able to obtain information dynamically about GOOPS objects. It is perhaps best illustrated by considering an object oriented language that does not provide any introspection, namely C++.

Nothing in C++ allows a running program to obtain answers to the following types of question:

*   What are the data members of this object or class?
*   What classes does this class inherit from?
*   Is this method call virtual or non-virtual?
*   If I invoke `Employee::adjustHoliday()`, what class contains the `adjustHoliday()` method that will be applied?

In C++, answers to such questions can only be determined by looking at the source code, if you have access to it. GOOPS, on the other hand, includes procedures that allow answers to these questions — or their GOOPS equivalents — to be obtained dynamically, at run time.

*   [Classes](08_08_introspection.md#881-classes)
*   [Instances](08_08_introspection.md#882-instances)
*   [Slots](08_08_introspection.md#883-slots)
*   [Generic Functions](08_08_introspection.md#884-generic-functions)
*   [Accessing Slots](08_08_introspection.md#885-accessing-slots)

* * *

Next: [Instances](08_08_introspection.md#882-instances), Up: [Introspection](08_08_introspection.md#88-introspection)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.8.1 Classes [¶](08_08_introspection.md#881-classes)

A GOOPS class is itself an instance of the `<class>` class, or of a subclass of `<class>`. The definition of the `<class>` class has slots that are used to describe the properties of a class, including the following.

primitive procedure: **class-name** class [¶](08_08_introspection.md)

Return the name of class class. This is the value of class’s `name` slot.

primitive procedure: **class-direct-supers** class [¶](08_08_introspection.md)

Return a list containing the direct superclasses of class. This is the value of class’s `direct-supers` slot.

primitive procedure: **class-direct-slots** class [¶](08_08_introspection.md)

Return a list containing the slot definitions of the direct slots of class. This is the value of class’s `direct-slots` slot.

primitive procedure: **class-direct-subclasses** class [¶](08_08_introspection.md)

Return a list containing the direct subclasses of class. This is the value of class’s `direct-subclasses` slot.

primitive procedure: **class-direct-methods** class [¶](08_08_introspection.md)

Return a list of all the generic function methods that use class as a formal parameter specializer. This is the value of class’s `direct-methods` slot.

primitive procedure: **class-precedence-list** class [¶](08_08_introspection.md)

Return the class precedence list for class class (see [Class Precedence List](08_07_inheritance.md#871-class-precedence-list)). This is the value of class’s `cpl` slot.

primitive procedure: **class-slots** class [¶](08_08_introspection.md)

Return a list containing the slot definitions for all class’s slots, including any slots that are inherited from superclasses. This is the value of class’s `slots` slot.

procedure: **class-subclasses** class [¶](08_08_introspection.md)

Return a list of all subclasses of class.

procedure: **class-methods** class [¶](08_08_introspection.md)

Return a list of all methods that use class or a subclass of class as one of its formal parameter specializers.

* * *

Next: [Slots](08_08_introspection.md#883-slots), Previous: [Classes](08_08_introspection.md#881-classes), Up: [Introspection](08_08_introspection.md#88-introspection)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.8.2 Instances [¶](08_08_introspection.md#882-instances)

primitive procedure: **class-of** value [¶](08_08_introspection.md)

Return the GOOPS class of any Scheme value.

primitive procedure: **instance?** object [¶](08_08_introspection.md)

Return `#t` if object is any GOOPS instance, otherwise `#f`.

procedure: **is-a?** object class [¶](08_08_introspection.md)

Return `#t` if object is an instance of class or one of its subclasses.

You can use the `is-a?` predicate to ask whether any given value belongs to a given class, or `class-of` to discover the class of a given value. Note that when GOOPS is loaded (by code using the `(oop goops)` module) built-in classes like `<string>`, `<list>` and `<number>` are automatically set up, corresponding to all Guile Scheme types.

([is-a?](08_08_introspection.md) 2.3 <number>) ⇒ #t
([is-a?](08_08_introspection.md) 2.3 <real>) ⇒ #t
([is-a?](08_08_introspection.md) 2.3 <string>) ⇒ #f
([is-a?](08_08_introspection.md) '("a" "b") <string>) ⇒ #f
([is-a?](08_08_introspection.md) '("a" "b") <list>) ⇒ #t
([is-a?](08_08_introspection.md) ([car](06_06_08_pairs.md) '("a" "b")) <string>) ⇒ #t
([is-a?](08_08_introspection.md) <string> <class>) ⇒ #t
([is-a?](08_08_introspection.md) <class> <string>) ⇒ #f

([class-of](08_08_introspection.md) 2.3) ⇒ #<<class> <real> 908c708>
([class-of](08_08_introspection.md) #(1 2 3)) ⇒ #<<class> <vector> 908cd20>
([class-of](08_08_introspection.md) <string>) ⇒ #<<class> <class> 8bd3e10>
([class-of](08_08_introspection.md) <class>) ⇒ #<<class> <class> 8bd3e10>

* * *

Next: [Generic Functions](08_08_introspection.md#884-generic-functions), Previous: [Instances](08_08_introspection.md#882-instances), Up: [Introspection](08_08_introspection.md#88-introspection)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.8.3 Slots [¶](08_08_introspection.md#883-slots)

procedure: **class-slot-definition** class slot-name [¶](08_08_introspection.md)

Return the slot definition for the slot named slot-name in class class. slot-name should be a symbol.

procedure: **slot-definition-name** slot-def [¶](08_08_introspection.md)

Extract and return the slot name from slot-def.

procedure: **slot-definition-options** slot-def [¶](08_08_introspection.md)

Extract and return the slot options from slot-def.

procedure: **slot-definition-allocation** slot-def [¶](08_08_introspection.md)

Extract and return the slot allocation option from slot-def. This is the value of the `#:allocation` keyword (see [allocation](08_04_slot_options.md#84-slot-options)), or `#:instance` if the `#:allocation` keyword is absent.

procedure: **slot-definition-getter** slot-def [¶](08_08_introspection.md)

Extract and return the slot getter option from slot-def. This is the value of the `#:getter` keyword (see [getter](08_04_slot_options.md#84-slot-options)), or `#f` if the `#:getter` keyword is absent.

procedure: **slot-definition-setter** slot-def [¶](08_08_introspection.md)

Extract and return the slot setter option from slot-def. This is the value of the `#:setter` keyword (see [setter](08_04_slot_options.md#84-slot-options)), or `#f` if the `#:setter` keyword is absent.

procedure: **slot-definition-accessor** slot-def [¶](08_08_introspection.md)

Extract and return the slot accessor option from slot-def. This is the value of the `#:accessor` keyword (see [accessor](08_04_slot_options.md#84-slot-options)), or `#f` if the `#:accessor` keyword is absent.

procedure: **slot-definition-init-value** slot-def [¶](08_08_introspection.md)

Extract and return the slot init-value option from slot-def. This is the value of the `#:init-value` keyword (see [init-value](08_04_slot_options.md#84-slot-options)), or the unbound value if the `#:init-value` keyword is absent.

procedure: **slot-definition-init-form** slot-def [¶](08_08_introspection.md)

Extract and return the slot init-form option from slot-def. This is the value of the `#:init-form` keyword (see [init-form](08_04_slot_options.md#84-slot-options)), or the unbound value if the `#:init-form` keyword is absent.

procedure: **slot-definition-init-thunk** slot-def [¶](08_08_introspection.md)

Extract and return the slot init-thunk option from slot-def. This is the value of the `#:init-thunk` keyword (see [init-thunk](08_04_slot_options.md#84-slot-options)), or `#f` if the `#:init-thunk` keyword is absent.

procedure: **slot-definition-init-keyword** slot-def [¶](08_08_introspection.md)

Extract and return the slot init-keyword option from slot-def. This is the value of the `#:init-keyword` keyword (see [init-keyword](08_04_slot_options.md#84-slot-options)), or `#f` if the `#:init-keyword` keyword is absent.

procedure: **slot-init-function** class slot-name [¶](08_08_introspection.md)

Return the initialization function for the slot named slot-name in class class. slot-name should be a symbol.

The returned initialization function incorporates the effects of the standard `#:init-thunk`, `#:init-form` and `#:init-value` slot options. These initializations can be overridden by the `#:init-keyword` slot option or by a specialized `initialize` method, so, in general, the function returned by `slot-init-function` may be irrelevant. For a fuller discussion, see [init-value](08_04_slot_options.md#84-slot-options).

* * *

Next: [Accessing Slots](08_08_introspection.md#885-accessing-slots), Previous: [Slots](08_08_introspection.md#883-slots), Up: [Introspection](08_08_introspection.md#88-introspection)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.8.4 Generic Functions [¶](08_08_introspection.md#884-generic-functions)

A generic function is an instance of the `<generic>` class, or of a subclass of `<generic>`. The definition of the `<generic>` class has slots that are used to describe the properties of a generic function.

primitive procedure: **generic-function-name** gf [¶](08_08_introspection.md)

Return the name of generic function gf.

primitive procedure: **generic-function-methods** gf [¶](08_08_introspection.md)

Return a list of the methods of generic function gf. This is the value of gf’s `methods` slot.

Similarly, a method is an instance of the `<method>` class, or of a subclass of `<method>`; and the definition of the `<method>` class has slots that are used to describe the properties of a method.

primitive procedure: **method-generic-function** method [¶](08_08_introspection.md)

Return the generic function that method belongs to. This is the value of method’s `generic-function` slot.

primitive procedure: **method-specializers** method [¶](08_08_introspection.md)

Return a list of method’s formal parameter specializers . This is the value of method’s `specializers` slot.

primitive procedure: **method-procedure** method [¶](08_08_introspection.md)

Return the procedure that implements method. This is the value of method’s `procedure` slot.

generic: **method-source** [¶](08_08_introspection.md)

method: **method-source** (m <method>) [¶](08_08_introspection.md)

Return an expression that prints to show the definition of method m.

(define-generic cube)

(define-method (cube (n <number>))
  (\* n n n))

(map method-source (generic-function-methods cube))
⇒
((method ((n <number>)) (\* n n n)))

* * *

Previous: [Generic Functions](08_08_introspection.md#884-generic-functions), Up: [Introspection](08_08_introspection.md#88-introspection)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.8.5 Accessing Slots [¶](08_08_introspection.md#885-accessing-slots)

Any slot, regardless of its allocation, can be queried, referenced and set using the following four primitive procedures.

primitive procedure: **slot-exists?** obj slot-name [¶](08_08_introspection.md)

Return `#t` if obj has a slot with name slot-name, otherwise `#f`.

primitive procedure: **slot-bound?** obj slot-name [¶](08_08_introspection.md)

Return `#t` if the slot named slot-name in obj has a value, otherwise `#f`.

`slot-bound?` calls the generic function `slot-missing` if obj does not have a slot called slot-name (see [slot-missing](08_08_introspection.md#885-accessing-slots)).

primitive procedure: **slot-ref** obj slot-name [¶](08_08_introspection.md)

Return the value of the slot named slot-name in obj.

`slot-ref` calls the generic function `slot-missing` if obj does not have a slot called slot-name (see [slot-missing](08_08_introspection.md#885-accessing-slots)).

`slot-ref` calls the generic function `slot-unbound` if the named slot in obj does not have a value (see [slot-unbound](08_08_introspection.md#885-accessing-slots)).

primitive procedure: **slot-set!** obj slot-name value [¶](08_08_introspection.md)

Set the value of the slot named slot-name in obj to value.

`slot-set!` calls the generic function `slot-missing` if obj does not have a slot called slot-name (see [slot-missing](08_08_introspection.md#885-accessing-slots)).

GOOPS stores information about slots in classes. Internally, all of these procedures work by looking up the slot definition for the slot named slot-name in the class `(class-of obj)`, and then using the slot definition’s “getter” and “setter” closures to get and set the slot value.

The next four procedures differ from the previous ones in that they take the class as an explicit argument, rather than assuming `(class-of obj)`. Therefore they allow you to apply the “getter” and “setter” closures of a slot definition in one class to an instance of a different class.

primitive procedure: **slot-exists-using-class?** class obj slot-name [¶](08_08_introspection.md)

Return `#t` if class has a slot definition for a slot with name slot-name, otherwise `#f`.

primitive procedure: **slot-bound-using-class?** class obj slot-name [¶](08_08_introspection.md)

Return `#t` if applying `slot-ref-using-class` to the same arguments would call the generic function `slot-unbound`, otherwise `#f`.

`slot-bound-using-class?` calls the generic function `slot-missing` if class does not have a slot definition for a slot called slot-name (see [slot-missing](08_08_introspection.md#885-accessing-slots)).

primitive procedure: **slot-ref-using-class** class obj slot-name [¶](08_08_introspection.md)

Apply the “getter” closure for the slot named slot-name in class to obj, and return its result.

`slot-ref-using-class` calls the generic function `slot-missing` if class does not have a slot definition for a slot called slot-name (see [slot-missing](08_08_introspection.md#885-accessing-slots)).

`slot-ref-using-class` calls the generic function `slot-unbound` if the application of the “getter” closure to obj returns an unbound value (see [slot-unbound](08_08_introspection.md#885-accessing-slots)).

primitive procedure: **slot-set-using-class!** class obj slot-name value [¶](08_08_introspection.md)

Apply the “setter” closure for the slot named slot-name in class to obj and value.

`slot-set-using-class!` calls the generic function `slot-missing` if class does not have a slot definition for a slot called slot-name (see [slot-missing](08_08_introspection.md#885-accessing-slots)).

Slots whose allocation is per-class rather than per-instance can be referenced and set without needing to specify any particular instance.

procedure: **class-slot-ref** class slot-name [¶](08_08_introspection.md)

Return the value of the slot named slot-name in class class. The named slot must have `#:class` or `#:each-subclass` allocation (see [allocation](08_04_slot_options.md#84-slot-options)).

If there is no such slot with `#:class` or `#:each-subclass` allocation, `class-slot-ref` calls the `slot-missing` generic function with arguments class and slot-name. Otherwise, if the slot value is unbound, `class-slot-ref` calls the `slot-unbound` generic function, with the same arguments.

procedure: **class-slot-set!** class slot-name value [¶](08_08_introspection.md)

Set the value of the slot named slot-name in class class to value. The named slot must have `#:class` or `#:each-subclass` allocation (see [allocation](08_04_slot_options.md#84-slot-options)).

If there is no such slot with `#:class` or `#:each-subclass` allocation, `class-slot-ref` calls the `slot-missing` generic function with arguments class and slot-name.

When a `slot-ref` or `slot-set!` call specifies a non-existent slot name, or tries to reference a slot whose value is unbound, GOOPS calls one of the following generic functions.

generic: **slot-missing** [¶](08_08_introspection.md)

method: **slot-missing** (class <class>) slot-name [¶](08_08_introspection.md)

method: **slot-missing** (class <class>) (object <object>) slot-name [¶](08_08_introspection.md)

method: **slot-missing** (class <class>) (object <object>) slot-name value [¶](08_08_introspection.md)

When an application attempts to reference or set a class or instance slot by name, and the slot name is invalid for the specified class or object, GOOPS calls the `slot-missing` generic function.

The default methods all call `goops-error` with an appropriate message.

generic: **slot-unbound** [¶](08_08_introspection.md)

method: **slot-unbound** (object <object>) [¶](08_08_introspection.md)

method: **slot-unbound** (class <class>) slot-name [¶](08_08_introspection.md)

method: **slot-unbound** (class <class>) (object <object>) slot-name [¶](08_08_introspection.md)

When an application attempts to reference a class or instance slot, and the slot’s value is unbound, GOOPS calls the `slot-unbound` generic function.

The default methods all call `goops-error` with an appropriate message.

* * *

Next: [GOOPS Object Miscellany](08_10_goops_object_miscellany.md#810-goops-object-miscellany), Previous: [Introspection](08_08_introspection.md#88-introspection), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
