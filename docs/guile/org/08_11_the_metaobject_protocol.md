### 8.11 The Metaobject Protocol [¶](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)

At this point, we’ve said about as much as can be said about GOOPS without having to confront the idea of the metaobject protocol. There are a couple more topics that could be discussed in isolation first — class redefinition, and changing the class of existing instances — but in practice developers using them will be advanced enough to want to understand the metaobject protocol too, and will probably be using the protocol to customize exactly what happens during these events.

So let’s plunge in. GOOPS is based on a “metaobject protocol” (aka “MOP”) derived from the ones used in CLOS (the Common Lisp Object System), tiny-clos (a small Scheme implementation of a subset of CLOS functionality) and STKlos.

The MOP underlies many possible GOOPS customizations — such as defining an `initialize` method to customize the initialization of instances of an application-defined class — and an understanding of the MOP makes it much easier to explain such customizations in a precise way. And at a deeper level, understanding the MOP is a key part of understanding GOOPS, and of taking full advantage of GOOPS’ power, by customizing the behavior of GOOPS itself.

*   [Metaobjects and the Metaobject Protocol](08_11_the_metaobject_protocol.md#8111-metaobjects-and-the-metaobject-protocol)
*   [Metaclasses](08_11_the_metaobject_protocol.md#8112-metaclasses)
*   [MOP Specification](08_11_the_metaobject_protocol.md#8113-mop-specification)
*   [Instance Creation Protocol](08_11_the_metaobject_protocol.md#8114-instance-creation-protocol)
*   [Class Definition Protocol](08_11_the_metaobject_protocol.md#8115-class-definition-protocol)
*   [Customizing Class Definition](08_11_the_metaobject_protocol.md#8116-customizing-class-definition)
*   [Method Definition](08_11_the_metaobject_protocol.md#8117-method-definition)
*   [Method Definition Internals](08_11_the_metaobject_protocol.md#8118-method-definition-internals)
*   [Generic Function Internals](08_11_the_metaobject_protocol.md#8119-generic-function-internals)
*   [Generic Function Invocation](08_11_the_metaobject_protocol.md#81110-generic-function-invocation)

* * *

Next: [Metaclasses](08_11_the_metaobject_protocol.md#8112-metaclasses), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.1 Metaobjects and the Metaobject Protocol [¶](08_11_the_metaobject_protocol.md#8111-metaobjects-and-the-metaobject-protocol)

The building blocks of GOOPS are classes, slot definitions, instances, generic functions and methods. A class is a grouping of inheritance relations and slot definitions. An instance is an object with slots that are allocated following the rules implied by its class’s superclasses and slot definitions. A generic function is a collection of methods and rules for determining which of those methods to apply when the generic function is invoked. A method is a procedure and a set of specializers that specify the type of arguments to which the procedure is applicable.

Of these entities, GOOPS represents classes, generic functions and methods as “metaobjects”. In other words, the values in a GOOPS program that describe classes, generic functions and methods, are themselves instances (or “objects”) of special GOOPS classes that encapsulate the behavior, respectively, of classes, generic functions, and methods.

(The other two entities are slot definitions and instances. Slot definitions are not strictly instances, but every slot definition is associated with a GOOPS class that specifies the behavior of the slot as regards accessibility and protection from garbage collection. Instances are of course objects in the usual sense, and there is no benefit from thinking of them as metaobjects.)

The “metaobject protocol” (or “MOP”) is the specification of the generic functions which determine the behavior of these metaobjects and the circumstances in which these generic functions are invoked.

For a concrete example of what this means, consider how GOOPS calculates the set of slots for a class that is being defined using `define-class`. The desired set of slots is the union of the new class’s direct slots and the slots of all its superclasses. But `define-class` itself does not perform this calculation. Instead, there is a method of the `initialize` generic function that is specialized for instances of type `<class>`, and it is this method that performs the slot calculation.

`initialize` is a generic function which GOOPS calls whenever a new instance is created, immediately after allocating memory for a new instance, in order to initialize the new instance’s slots. The sequence of steps is as follows.

*   `define-class` uses `make` to make a new instance of the `<class>` class, passing as initialization arguments the superclasses, slot definitions and class options that were specified in the `define-class` form.
*   `make` allocates memory for the new instance, and invokes the `initialize` generic function to initialize the new instance’s slots.
*   The `initialize` generic function applies the method that is specialized for instances of type `<class>`, and this method performs the slot calculation.

In other words, rather than being hardcoded in `define-class`, the default behavior of class definition is encapsulated by generic function methods that are specialized for the class `<class>`.

It is possible to create a new class that inherits from `<class>`, which is called a “metaclass”, and to write a new `initialize` method that is specialized for instances of the new metaclass. Then, if the `define-class` form includes a `#:metaclass` class option whose value is the new metaclass, the class that is defined by the `define-class` form will be an instance of the new metaclass rather than of the default `<class>`, and will be defined in accordance with the new `initialize` method. Thus the default slot calculation, as well as any other aspect of the new class’s relationship with its superclasses, can be modified or overridden.

In a similar way, the behavior of generic functions can be modified or overridden by creating a new class that inherits from the standard generic function class `<generic>`, writing appropriate methods that are specialized to the new class, and creating new generic functions that are instances of the new class.

The same is true for method metaobjects. And the same basic mechanism allows the application class author to write an `initialize` method that is specialized to their application class, to initialize instances of that class.

Such is the power of the MOP. Note that `initialize` is just one of a large number of generic functions that can be customized to modify the behavior of application objects and classes and of GOOPS itself. Each following section covers a particular area of GOOPS functionality, and describes the generic functions that are relevant for customization of that area.

* * *

Next: [MOP Specification](08_11_the_metaobject_protocol.md#8113-mop-specification), Previous: [Metaobjects and the Metaobject Protocol](08_11_the_metaobject_protocol.md#8111-metaobjects-and-the-metaobject-protocol), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.2 Metaclasses [¶](08_11_the_metaobject_protocol.md#8112-metaclasses)

A _metaclass_ is the class of an object which represents a GOOPS class. Put more succinctly, a metaclass is a class’s class.

Most GOOPS classes have the metaclass `<class>` and, by default, any new class that is created using `define-class` has the metaclass `<class>`.

But what does this really mean? To find out, let’s look in more detail at what happens when a new class is created using `define-class`:

(define-class <my-class> (<object>) . slots)

Guile expands this to something like:

(define <my-class> (class (<object>) . slots))

which in turn expands to:

(define <my-class>
  (make <class> #:dsupers (list <object>) #:slots slots))

As this expansion makes clear, the resulting value of `<my-class>` is an instance of the class `<class>` with slot values specifying the superclasses and slot definitions for the class `<my-class>`. (`#:dsupers` and `#:slots` are initialization keywords for the `dsupers` and `dslots` slots of the `<class>` class.)

Now suppose that you want to define a new class with a metaclass other than the default `<class>`. This is done by writing:

(define-class <my-class2> (<object>)
   slot ...
   #:metaclass <my-metaclass>)

and Guile expands _this_ to something like:

(define <my-class2>
  (make <my-metaclass> #:dsupers (list <object>) #:slots slots))

In this case, the value of `<my-class2>` is an instance of the more specialized class `<my-metaclass>`. Note that `<my-metaclass>` itself must previously have been defined as a subclass of `<class>`. For a full discussion of when and how it is useful to define new metaclasses, see [MOP Specification](08_11_the_metaobject_protocol.md#8113-mop-specification).

Now let’s make an instance of `<my-class2>`:

(define my-object (make <my-class2> ...))

All of the following statements are correct expressions of the relationships between `my-object`, `<my-class2>`, `<my-metaclass>` and `<class>`.

*   `my-object` is an instance of the class `<my-class2>`.
*   `<my-class2>` is an instance of the class `<my-metaclass>`.
*   `<my-metaclass>` is an instance of the class `<class>`.
*   The class of `my-object` is `<my-class2>`.
*   The class of `<my-class2>` is `<my-metaclass>`.
*   The class of `<my-metaclass>` is `<class>`.

* * *

Next: [Instance Creation Protocol](08_11_the_metaobject_protocol.md#8114-instance-creation-protocol), Previous: [Metaclasses](08_11_the_metaobject_protocol.md#8112-metaclasses), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.3 MOP Specification [¶](08_11_the_metaobject_protocol.md#8113-mop-specification)

The aim of the MOP specification in this chapter is to specify all the customizable generic function invocations that can be made by the standard GOOPS syntax, procedures and methods, and to explain the protocol for customizing such invocations.

A generic function invocation is customizable if the types of the arguments to which it is applied are not completely determined by the lexical context in which the invocation appears. For example, the `(initialize instance initargs)` invocation in the default `make-instance` method is customizable, because the type of the `instance` argument is determined by the class that was passed to `make-instance`.

(Whereas — to give a counter-example — the `(make <generic> #:name ',name)` invocation in `define-generic` is not customizable, because all of its arguments have lexically determined types.)

When using this rule to decide whether a given generic function invocation is customizable, we ignore arguments that are expected to be handled in method definitions as a single “rest” list argument.

For each customizable generic function invocation, the _invocation protocol_ is explained by specifying

*   what, conceptually, the applied method is intended to do
*   what assumptions, if any, the caller makes about the applied method’s side effects
*   what the caller expects to get as the applied method’s return value.

* * *

Next: [Class Definition Protocol](08_11_the_metaobject_protocol.md#8115-class-definition-protocol), Previous: [MOP Specification](08_11_the_metaobject_protocol.md#8113-mop-specification), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.4 Instance Creation Protocol [¶](08_11_the_metaobject_protocol.md#8114-instance-creation-protocol)

`make <class> . initargs` (method)

*   `allocate-instance class initargs` (generic)
    
    The applied `allocate-instance` method should allocate storage for a new instance of class class and return the uninitialized instance.
    
*   `initialize instance initargs` (generic)
    
    instance is the uninitialized instance returned by `allocate-instance`. The applied method should initialize the new instance in whatever sense is appropriate for its class. The method’s return value is ignored.
    

`make` itself is a generic function. Hence the `make` invocation itself can be customized in the case where the new instance’s metaclass is more specialized than the default `<class>`, by defining a `make` method that is specialized to that metaclass.

Normally, however, the method for classes with metaclass `<class>` will be applied. This method calls two generic functions:

*   (allocate-instance class . initargs)
*   (initialize instance . initargs)

`allocate-instance` allocates storage for and returns the new instance, uninitialized. You might customize `allocate-instance`, for example, if you wanted to provide a GOOPS wrapper around some other object programming system.

To do this, you would create a specialized metaclass, which would act as the metaclass for all classes and instances from the other system. Then define an `allocate-instance` method, specialized to that metaclass, which calls a Guile primitive C function (or FFI code), which in turn allocates the new instance using the interface of the other object system.

In this case, for a complete system, you would also need to customize a number of other generic functions like `make` and `initialize`, so that GOOPS knows how to make classes from the other system, access instance slots, and so on.

`initialize` initializes the instance that is returned by `allocate-instance`. The standard GOOPS methods perform initializations appropriate to the instance class.

*   At the least specialized level, the method for instances of type `<object>` performs internal GOOPS instance initialization, and initializes the instance’s slots according to the slot definitions and any slot initialization keywords that appear in initargs.
*   The method for instances of type `<class>` calls `(next-method)`, then performs the class initializations described in [Class Definition Protocol](08_11_the_metaobject_protocol.md#8115-class-definition-protocol).
*   and so on for generic functions, methods, operator classes …

Similarly, you can customize the initialization of instances of any application-defined class by defining an `initialize` method specialized to that class.

Imagine a class whose instances’ slots need to be initialized at instance creation time by querying a database. Although it might be possible to achieve this a combination of `#:init-thunk` keywords and closures in the slot definitions, it may be neater to write an `initialize` method for the class that queries the database once and initializes all the dependent slot values according to the results.

* * *

Next: [Customizing Class Definition](08_11_the_metaobject_protocol.md#8116-customizing-class-definition), Previous: [Instance Creation Protocol](08_11_the_metaobject_protocol.md#8114-instance-creation-protocol), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.5 Class Definition Protocol [¶](08_11_the_metaobject_protocol.md#8115-class-definition-protocol)

Here is a summary diagram of the syntax, procedures and generic functions that may be involved in class definition.

`define-class` (syntax)

*   `class` (syntax)
    *   `make-class` (procedure)
        *   `ensure-metaclass` (procedure)
        *   `make metaclass …` (generic)
            *   `allocate-instance` (generic)
            *   `initialize` (generic)
                *   `compute-cpl` (generic)
                    *   `compute-std-cpl` (procedure)
                *   `compute-slots` (generic)
                *   `compute-get-n-set` (generic)
                *   `compute-getter-method` (generic)
                *   `compute-setter-method` (generic)
*   `class-redefinition` (generic)
    *   `remove-class-accessors` (generic)
    *   `update-direct-method!` (generic)
    *   `update-direct-subclass!` (generic)

Wherever a step above is marked as “generic”, it can be customized, and the detail shown below it is only “correct” insofar as it describes what the default method of that generic function does. For example, if you write an `initialize` method, for some metaclass, that does not call `next-method` and does not call `compute-cpl`, then `compute-cpl` will not be called when a class is defined with that metaclass.

A `(define-class ...)` form (see [Class Definition](08_02_class_definition.md#82-class-definition)) expands to an expression which

*   checks that it is being evaluated only at top level
*   defines any accessors that are implied by the slot-definitions
*   uses `class` to create the new class
*   checks for a previous class definition for name and, if found, handles the redefinition by invoking `class-redefinition` (see [Redefining a Class](08_12_redefining_a_class.md#812-redefining-a-class)).

syntax: **class** name (super …) slot-definition … class-option … [¶](08_11_the_metaobject_protocol.md)

Return a newly created class that inherits from supers, with direct slots defined by slot-definitions and class-options. For the format of slot-definitions and class-options, see [define-class](08_02_class_definition.md#82-class-definition).

`class` expands to an expression which

*   processes the class and slot definition options to check that they are well-formed, to convert the `#:init-form` option to an `#:init-thunk` option, to supply a default environment parameter (the current top-level environment) and to evaluate all the bits that need to be evaluated
*   calls `make-class` to create the class with the processed and evaluated parameters.

procedure: **make-class** supers slots class-option … [¶](08_11_the_metaobject_protocol.md)

Return a newly created class that inherits from supers, with direct slots defined by slots and class-options. For the format of slots and class-options, see [define-class](08_02_class_definition.md#82-class-definition), except note that for `make-class`, slots is a separate list of slot definitions.

`make-class`

*   adds `<object>` to the supers list if supers is empty or if none of the classes in supers have `<object>` in their class precedence list
*   defaults the `#:environment`, `#:name` and `#:metaclass` options, if they are not specified by options, to the current top-level environment, the unbound value, and `(ensure-metaclass supers)` respectively
*   checks for duplicate classes in supers and duplicate slot names in slots, and signals an error if there are any duplicates
*   calls `make`, passing the metaclass as the first parameter and all other parameters as option keywords with values.

procedure: **ensure-metaclass** supers env [¶](08_11_the_metaobject_protocol.md)

Return a metaclass suitable for a class that inherits from the list of classes in supers. The returned metaclass is the union by inheritance of the metaclasses of the classes in supers.

In the simplest case, where all the supers are straightforward classes with metaclass `<class>`, the returned metaclass is just `<class>`.

For a more complex example, suppose that supers contained one class with metaclass `<operator-class>` and one with metaclass `<foreign-object-class>`. Then the returned metaclass would be a class that inherits from both `<operator-class>` and `<foreign-object-class>`.

If supers is the empty list, `ensure-metaclass` returns the default GOOPS metaclass `<class>`.

GOOPS keeps a list of the metaclasses created by `ensure-metaclass`, so that each required type of metaclass only has to be created once.

The `env` parameter is ignored.

generic: **make** metaclass initarg … [¶](08_11_the_metaobject_protocol.md)

metaclass is the metaclass of the class being defined, either taken from the `#:metaclass` class option or computed by `ensure-metaclass`. The applied method must create and return the fully initialized class metaobject for the new class definition.

The `(make metaclass initarg …)` invocation is a particular case of the instance creation protocol covered in the previous section. It will create an class metaobject with metaclass metaclass. By default, this metaobject will be initialized by the `initialize` method that is specialized for instances of type `<class>`.

The `initialize` method for classes (signature `(initialize <class> initargs)`) calls the following generic functions.

*   `compute-cpl class` (generic)
    
    The applied method should compute and return the class precedence list for class as a list of class metaobjects. When `compute-cpl` is called, the following class metaobject slots have all been initialized: `name`, `direct-supers`, `direct-slots`, `direct-subclasses` (empty), `direct-methods`. The value returned by `compute-cpl` will be stored in the `cpl` slot.
    
*   `compute-slots class` (generic)
    
    The applied method should compute and return the slots (union of direct and inherited) for class as a list of slot definitions. When `compute-slots` is called, all the class metaobject slots mentioned for `compute-cpl` have been initialized, plus the following: `cpl`, `redefined` (`#f`), `environment`. The value returned by `compute-slots` will be stored in the `slots` slot.
    
*   `compute-get-n-set class slot-def` (generic)
    
    `initialize` calls `compute-get-n-set` for each slot computed by `compute-slots`. The applied method should compute and return a pair of closures that, respectively, get and set the value of the specified slot. The get closure should have arity 1 and expect a single argument that is the instance whose slot value is to be retrieved. The set closure should have arity 2 and expect two arguments, where the first argument is the instance whose slot value is to be set and the second argument is the new value for that slot. The closures should be returned in a two element list: `(list get set)`.
    
    The closures returned by `compute-get-n-set` are stored as part of the value of the class metaobject’s `getters-n-setters` slot. Specifically, the value of this slot is a list with the same number of elements as there are slots in the class, and each element looks either like
    
    (slot-name-symbol init-function . index)
    
    or like
    
    (slot-name-symbol init-function get set)
    
    Where the get and set closures are replaced by index, the slot is an instance slot and index is the slot’s index in the underlying structure: GOOPS knows how to get and set the value of such slots and so does not need specially constructed get and set closures. Otherwise, get and set are the closures returned by `compute-get-n-set`.
    
    The structure of the `getters-n-setters` slot value is important when understanding the next customizable generic functions that `initialize` calls…
    
*   `compute-getter-method class gns` (generic)
    
    `initialize` calls `compute-getter-method` for each of the class’s slots (as determined by `compute-slots`) that includes a `#:getter` or `#:accessor` slot option. gns is the element of the class metaobject’s `getters-n-setters` slot that specifies how the slot in question is referenced and set, as described above under `compute-get-n-set`. The applied method should create and return a method that is specialized for instances of type class and uses the get closure to retrieve the slot’s value. `initialize` uses `add-method!` to add the returned method to the generic function named by the slot definition’s `#:getter` or `#:accessor` option.
    
*   `compute-setter-method class gns` (generic)
    
    `compute-setter-method` is invoked with the same arguments as `compute-getter-method`, for each of the class’s slots that includes a `#:setter` or `#:accessor` slot option. The applied method should create and return a method that is specialized for instances of type class and uses the set closure to set the slot’s value. `initialize` then uses `add-method!` to add the returned method to the generic function named by the slot definition’s `#:setter` or `#:accessor` option.
    

* * *

Next: [Method Definition](08_11_the_metaobject_protocol.md#8117-method-definition), Previous: [Class Definition Protocol](08_11_the_metaobject_protocol.md#8115-class-definition-protocol), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.6 Customizing Class Definition [¶](08_11_the_metaobject_protocol.md#8116-customizing-class-definition)

If the metaclass of the new class is something more specialized than the default `<class>`, then the type of class in the calls above is more specialized than `<class>`, and hence it becomes possible to define generic function methods, specialized for the new class’s metaclass, that can modify or override the default behavior of `initialize`, `compute-cpl` or `compute-get-n-set`.

`compute-cpl` computes the class precedence list (“CPL”) for the new class (see [Class Precedence List](08_07_inheritance.md#871-class-precedence-list)), and returns it as a list of class objects. The CPL is important because it defines a superclass ordering that is used, when a generic function is invoked upon an instance of the class, to decide which of the available generic function methods is the most specific. Hence `compute-cpl` could be customized in order to modify the CPL ordering algorithm for all classes with a special metaclass.

The default CPL algorithm is encapsulated by the `compute-std-cpl` procedure, which is called by the default `compute-cpl` method.

procedure: **compute-std-cpl** class [¶](08_11_the_metaobject_protocol.md)

Compute and return the class precedence list for class according to the algorithm described in [Class Precedence List](08_07_inheritance.md#871-class-precedence-list).

`compute-slots` computes and returns a list of all slot definitions for the new class. By default, this list includes the direct slot definitions from the `define-class` form, plus the slot definitions that are inherited from the new class’s superclasses. The default `compute-slots` method uses the CPL computed by `compute-cpl` to calculate this union of slot definitions, with the rule that slots inherited from superclasses are shadowed by direct slots with the same name. One possible reason for customizing `compute-slots` would be to implement an alternative resolution strategy for slot name conflicts.

`compute-get-n-set` computes the low-level closures that will be used to get and set the value of a particular slot, and returns them in a list with two elements.

The closures returned depend on how storage for that slot is allocated. The standard `compute-get-n-set` method, specialized for classes of type `<class>`, handles the standard GOOPS values for the `#:allocation` slot option (see [allocation](08_04_slot_options.md#84-slot-options)). By defining a new `compute-get-n-set` method for a more specialized metaclass, it is possible to support new types of slot allocation.

Suppose you wanted to create a large number of instances of some class with a slot that should be shared between some but not all instances of that class - say every 10 instances should share the same slot storage. The following example shows how to implement and use a new type of slot allocation to do this.

(define-class <batched-allocation-metaclass> (<class>))

(let ((batch-allocation-count 0)
      (batch-get-n-set #f))
  (define-method (compute-get-n-set
                     (class <batched-allocation-metaclass>) s)
    (case (slot-definition-allocation s)
      ((#:batched)
       ;; If we've already used the same slot storage for 10 instances,
       ;; reset variables.
       (if (= batch-allocation-count 10)
           (begin
             (set! batch-allocation-count 0)
             (set! batch-get-n-set #f)))
       ;; If we don't have a current pair of get and set closures,
       ;; create one.  make-closure-variable returns a pair of closures
       ;; around a single Scheme variable - see goops.scm for details.
       (or batch-get-n-set
           (set! batch-get-n-set (make-closure-variable)))
       ;; Increment the batch allocation count.
       (set! batch-allocation-count (+ batch-allocation-count 1))
       batch-get-n-set)

      ;; Call next-method to handle standard allocation types.
      (else (next-method)))))

(define-class <class-using-batched-slot> ()
  ...
  (c #:allocation #:batched)
  ...
  #:metaclass <batched-allocation-metaclass>)

The usage of `compute-getter-method` and `compute-setter-method` is described in [Class Definition Protocol](08_11_the_metaobject_protocol.md#8115-class-definition-protocol).

`compute-cpl` and `compute-get-n-set` are called by the standard `initialize` method for classes whose metaclass is `<class>`. But `initialize` itself can also be modified, by defining an `initialize` method specialized to the new class’s metaclass. Such a method could complete override the standard behavior, by not calling `(next-method)` at all, but more typically it would perform additional class initialization steps before and/or after calling `(next-method)` for the standard behavior.

* * *

Next: [Method Definition Internals](08_11_the_metaobject_protocol.md#8118-method-definition-internals), Previous: [Customizing Class Definition](08_11_the_metaobject_protocol.md#8116-customizing-class-definition), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.7 Method Definition [¶](08_11_the_metaobject_protocol.md#8117-method-definition)

`define-method` (syntax)

*   `add-method! target method` (generic)

`define-method` invokes the `add-method!` generic function to handle adding the new method to a variety of possible targets. GOOPS includes methods to handle target as

*   a generic function (the most common case)
*   a procedure
*   a primitive generic (see [Extending Primitives](08_06_methods_and_generic_functions.md#862-extending-primitives))

By defining further methods for `add-method!`, you can theoretically handle adding methods to further types of target.

* * *

Next: [Generic Function Internals](08_11_the_metaobject_protocol.md#8119-generic-function-internals), Previous: [Method Definition](08_11_the_metaobject_protocol.md#8117-method-definition), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.8 Method Definition Internals [¶](08_11_the_metaobject_protocol.md#8118-method-definition-internals)

`define-method`:

*   checks the form of the first parameter, and applies the following steps to the accessor’s setter if it has the `(setter …)` form
*   interpolates a call to `define-generic` or `define-accessor` if a generic function is not already defined with the supplied name
*   calls `method` with the parameters and body, to make a new method instance
*   calls `add-method!` to add this method to the relevant generic function.

syntax: **method** (parameter …) body … [¶](08_11_the_metaobject_protocol.md)

Make a method whose specializers are defined by the classes in parameters and whose procedure definition is constructed from the parameter symbols and body forms.

The parameter and body parameters should be as for `define-method` (see [define-method](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)).

`method`:

*   extracts formals and specializing classes from the parameters, defaulting the class for unspecialized parameters to `<top>`
*   creates a closure using the formals and the body forms
*   calls `make` with metaclass `<method>` and the specializers and closure using the `#:specializers` and `#:procedure` keywords.

procedure: **make-method** specializers procedure [¶](08_11_the_metaobject_protocol.md)

Make a method using specializers and procedure.

specializers should be a list of classes that specifies the parameter combinations to which this method will be applicable.

procedure should be the closure that will applied to the generic function parameters when this method is invoked.

`make-method` is a simple wrapper around `make` with metaclass `<method>`.

generic: **add-method!** target method [¶](08_11_the_metaobject_protocol.md)

Generic function for adding method method to target.

method: **add-method!** (generic <generic>) (method <method>) [¶](08_11_the_metaobject_protocol.md)

Add method method to the generic function generic.

method: **add-method!** (proc <procedure>) (method <method>) [¶](08_11_the_metaobject_protocol.md)

If proc is a procedure with generic capability (see [generic-capability?](08_06_methods_and_generic_functions.md#862-extending-primitives)), upgrade it to a primitive generic and add method to its generic function definition.

method: **add-method!** (pg <primitive-generic>) (method <method>) [¶](08_11_the_metaobject_protocol.md)

Add method method to the generic function definition of pg.

Implementation: `(add-method! (primitive-generic-generic pg) method)`.

method: **add-method!** (whatever <top>) (method <method>) [¶](08_11_the_metaobject_protocol.md)

Raise an error indicating that whatever is not a valid generic function.

* * *

Next: [Generic Function Invocation](08_11_the_metaobject_protocol.md#81110-generic-function-invocation), Previous: [Method Definition Internals](08_11_the_metaobject_protocol.md#8118-method-definition-internals), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.9 Generic Function Internals [¶](08_11_the_metaobject_protocol.md#8119-generic-function-internals)

`define-generic` calls `ensure-generic` to upgrade a pre-existing procedure value, or `make` with metaclass `<generic>` to create a new generic function.

`define-accessor` calls `ensure-accessor` to upgrade a pre-existing procedure value, or `make-accessor` to create a new accessor.

procedure: **ensure-generic** old-definition \[name\] [¶](08_11_the_metaobject_protocol.md)

Return a generic function with name name, if possible by using or upgrading old-definition. If unspecified, name defaults to `#f`.

If old-definition is already a generic function, it is returned unchanged.

If old-definition is a Scheme procedure or procedure-with-setter, `ensure-generic` returns a new generic function that uses old-definition for its default procedure and setter.

Otherwise `ensure-generic` returns a new generic function with no defaults and no methods.

procedure: **make-generic** \[name\] [¶](08_11_the_metaobject_protocol.md)

Return a new generic function with name `(car name)`. If unspecified, name defaults to `#f`.

`ensure-generic` calls `make` with metaclasses `<generic>` and `<generic-with-setter>`, depending on the previous value of the variable that it is trying to upgrade.

`make-generic` is a simple wrapper for `make` with metaclass `<generic>`.

procedure: **ensure-accessor** proc \[name\] [¶](08_11_the_metaobject_protocol.md)

Return an accessor with name name, if possible by using or upgrading proc. If unspecified, name defaults to `#f`.

If proc is already an accessor, it is returned unchanged.

If proc is a Scheme procedure, procedure-with-setter or generic function, `ensure-accessor` returns an accessor that reuses the reusable elements of proc.

Otherwise `ensure-accessor` returns a new accessor with no defaults and no methods.

procedure: **make-accessor** \[name\] [¶](08_11_the_metaobject_protocol.md)

Return a new accessor with name `(car name)`. If unspecified, name defaults to `#f`.

`ensure-accessor` calls `make` with metaclass `<generic-with-setter>`, as well as calls to `ensure-generic`, `make-accessor` and (tail recursively) `ensure-accessor`.

`make-accessor` calls `make` twice, first with metaclass `<generic>` to create a generic function for the setter, then with metaclass `<generic-with-setter>` to create the accessor, passing the setter generic function as the value of the `#:setter` keyword.

* * *

Previous: [Generic Function Internals](08_11_the_metaobject_protocol.md#8119-generic-function-internals), Up: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.11.10 Generic Function Invocation [¶](08_11_the_metaobject_protocol.md#81110-generic-function-invocation)

There is a detailed and customizable protocol involved in the process of invoking a generic function — i.e., in the process of deciding which of the generic function’s methods are applicable to the current arguments, and which one of those to apply. Here is a summary diagram of the generic functions involved.

`apply-generic` (generic)

*   `no-method` (generic)
*   `compute-applicable-methods` (generic)
*   `sort-applicable-methods` (generic)
    *   `method-more-specific?` (generic)
*   `apply-methods` (generic)
    *   `apply-method` (generic)
    *   `no-next-method` (generic)
*   `no-applicable-method`

We do not yet have full documentation for these. Please refer to the code (oop/goops.scm) for details.

* * *

Next: [Changing the Class of an Instance](08_13_changing_the_class_of_an_instance.md#813-changing-the-class-of-an-instance), Previous: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
