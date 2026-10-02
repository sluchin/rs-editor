### 8.7 Inheritance [¶](08_07_inheritance.md#87-inheritance)

Here are some class definitions to help illustrate inheritance:

(define-class A () a)
(define-class B () b)
(define-class C () c)
(define-class D (A B) d a)
(define-class E (A C) e c)
(define-class F (D E) f)

`A`, `B`, `C` have a null list of superclasses. In this case, the system will replace the null list by a list which only contains `<object>`, the root of all the classes defined by `define-class`. `D`, `E`, `F` use multiple inheritance: each class inherits from two previously defined classes. Those class definitions define a hierarchy which is shown in [Figure 8.2](08_07_inheritance.md). In this figure, the class `<top>` is also shown; this class is the superclass of all Scheme objects. In particular, `<top>` is the superclass of all standard Scheme types.

          <top>
          / \\\\\\\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_
         /   \\\\\_\_\_\_\_\_\_\_\_\_\_          \\
        /     \\           \\          \\
    <object>  <pair>  <procedure>  <number>
    /  |  \\                           |
   /   |   \\                          |
  A    B    C                      <complex>
  |\\\_\_/\_\_   |                         |
   \\ /   \\ /                          |
    D     E                         <real>
     \\   /                            |
       F                              |
                                   <integer>

**Figure 8.2:** A class hierarchy.

When a class has superclasses, its set of slots is calculated by taking the union of its own slots and those of all its superclasses. Thus each instance of D will have three slots, `a`, `b` and `d`). The slots of a class can be discovered using the `class-slots` primitive. For instance,

([class-slots](08_08_introspection.md) A) ⇒ ((a))
([class-slots](08_08_introspection.md) E) ⇒ ((a) (e) (c))
([class-slots](08_08_introspection.md) F) ⇒ ((e) (c) (b) (d) (a) (f))

The ordering of the returned slots is not significant.

*   [Class Precedence List](08_07_inheritance.md#871-class-precedence-list)
*   [Sorting Methods](08_07_inheritance.md#872-sorting-methods)
*   [Inheritance and accessors](08_07_inheritance.md#873-inheritance-and-accessors)

* * *

Next: [Sorting Methods](08_07_inheritance.md#872-sorting-methods), Up: [Inheritance](08_07_inheritance.md#87-inheritance)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.7.1 Class Precedence List [¶](08_07_inheritance.md#871-class-precedence-list)

What happens when a class inherits from two or more superclasses that have a slot with the same name but incompatible definitions — for example, different init values or slot allocations? We need a rule for deciding which slot definition the derived class ends up with, and this rule is provided by the class’s _Class Precedence List_.[35](99_footnotes.md)

Another problem arises when invoking a generic function, and there is more than one method that could apply to the call arguments. Here we need a way of ordering the applicable methods, so that Guile knows which method to use first, which to use next if that method calls `next-method`, and so on. One of the ingredients for this ordering is determining, for each given call argument, which of the specializing classes, from each applicable method’s definition, is the most specific for that argument; and here again the class precedence list helps.

If inheritance was restricted such that each class could only have one superclass — which is known as _single_ inheritance — class ordering would be easy. The rule would be simply that a subclass is considered more specific than its superclass.

With multiple inheritance, ordering is less obvious, and we have to impose an arbitrary rule to determine precedence. Suppose we have

(define-class X ()
   (x #:init-value 1))

(define-class Y ()
   (x #:init-value 2))

(define-class Z (X Y)
   ([...](06_08_macros.md)))

Clearly the `Z` class is more specific than `X` or `Y`, for instances of `Z`. But which is more specific out of `X` and `Y` — and hence, for the definitions above, which `#:init-value` will take effect when creating an instance of `Z`? The rule in GOOPS is that the superclasses listed earlier are more specific than those listed later. Hence `X` is more specific than `Y`, and the `#:init-value` for slot `x` in instances of `Z` will be 1.

Hence there is a linear ordering for a class and all its superclasses, from most specific to least specific, and this ordering is called the Class Precedence List of the class.

In fact the rules above are not quite enough to always determine a unique order, but they give an idea of how things work. For example, for the `F` class shown in [Figure 8.2](08_07_inheritance.md), the class precedence list is

(f d e a c b <object> <top>)

In cases where there is any ambiguity (like this one), it is a bad idea for programmers to rely on exactly what the order is. If the order for some superclasses is important, it can be expressed directly in the class definition.

The precedence list of a class can be obtained by calling `class-precedence-list`. This function returns a ordered list whose first element is the most specific class. For instance:

([class-precedence-list](08_08_introspection.md) B) ⇒ (#<<class> B 401b97c8> 
                                     #<<class> <object> 401e4a10> 
                                     #<<class> <top> 4026a9d8>)

Or for a more immediately readable result:

(map [class-name](08_08_introspection.md) ([class-precedence-list](08_08_introspection.md) B)) ⇒ (B <object> <top>) 

* * *

Next: [Inheritance and accessors](08_07_inheritance.md#873-inheritance-and-accessors), Previous: [Class Precedence List](08_07_inheritance.md#871-class-precedence-list), Up: [Inheritance](08_07_inheritance.md#87-inheritance)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.7.2 Sorting Methods [¶](08_07_inheritance.md#872-sorting-methods)

Now, with the idea of the class precedence list, we can state precisely how the possible methods are sorted when more than one of the methods of a generic function are applicable to the call arguments.

The rules are that

*   the applicable methods are sorted in order of specificity, and the most specific method is used first, then the next if that method calls `next-method`, and so on
*   a method M1 is more specific than another method M2 if the first specializing class that differs, between the definitions of M1 and M2, is more specific, in M1’s definition, for the corresponding actual call argument, than the specializing class in M2’s definition
*   a class C1 is more specific than another class C2, for an object of actual class C, if C1 comes before C2 in C’s class precedence list.

* * *

Previous: [Sorting Methods](08_07_inheritance.md#872-sorting-methods), Up: [Inheritance](08_07_inheritance.md#87-inheritance)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.7.3 Inheritance and accessors [¶](08_07_inheritance.md#873-inheritance-and-accessors)

When a class <A> defines a getter, setter or accessor for one of its slots x, an _accessor method_ specialized to class <A> is created. Accessor methods are special in that they always refer to a concrete class. When you subclass <A>, a new accessor method for x is created automatically, specialized to the subclass:

(define-class <A> () (x #:accessor x))
(define-class <B> (<A>))
([generic-function-methods](08_08_introspection.md) x)
⇒
(#<<accessor-method> (<B>) 7faa66b5b1c0>
 #<<accessor-method> (<A>) 7faa66b5b240>)

Note, in particular, that the x accessor method specialized to <A> is _not_ applicable to objects of class B:

(define o ([make](08_03_instance_creation_and_slot_access.md) <B>))
(compute-applicable-methods x ([list](06_06_09_lists.md) o))
⇒
(#<<accessor-method> (<B>) 7faa66b5b1c0>)

As a consequence, an accessor doesn’t have a next-method. The fact that accessor methods always apply to concrete classes allows for extensive optimization.

* * *

Next: [Error Handling](08_09_error_handling.md#89-error-handling), Previous: [Inheritance](08_07_inheritance.md#87-inheritance), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
