### 8.6 Methods and Generic Functions [¶](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)

A GOOPS method is like a Scheme procedure except that it is specialized for a particular set of argument classes, and will only be used when the actual arguments in a call match the classes in the method definition.

(define-method ([+](06_06_02_numerical_data_types.md) (x <string>) (y <string>))
  ([string-append](06_06_05_strings.md) x y))

([+](06_06_02_numerical_data_types.md) "abc" "de") ⇒ "abcde"

A method is not formally associated with any single class (as it is in many other object oriented languages), because a method can be specialized for a combination of several classes. If you’ve studied object orientation in non-Lispy languages, you may remember discussions such as whether a method to stretch a graphical image around a surface should be a method of the image class, with a surface as a parameter, or a method of the surface class, with an image as a parameter. In GOOPS you’d just write

(define-method (stretch (im <image>) (sf <surface>))
  [...](06_08_macros.md))

and the question of which class the method is more associated with does not need answering.

There can simultaneously be several methods with the same name but different sets of specializing argument classes; for example:

(define-method ([+](06_06_02_numerical_data_types.md) (x <string>) (y <string)) [...](06_08_macros.md))
(define-method ([+](06_06_02_numerical_data_types.md) (x <matrix>) (y <matrix>)) [...](06_08_macros.md))
(define-method ([+](06_06_02_numerical_data_types.md) (f <fish>) (b <bicycle>)) [...](06_08_macros.md))
(define-method ([+](06_06_02_numerical_data_types.md) (a <foo>) (b <bar>) (c <baz>)) [...](06_08_macros.md))

A generic function is a container for the set of such methods that a program intends to use.

If you look at a program’s source code, and see `(+ x y)` somewhere in it, conceptually what is happening is that the program at that point calls a generic function (in this case, the generic function bound to the identifier `+`). When that happens, Guile works out which of the generic function’s methods is the most appropriate for the arguments that the function is being called with; then it evaluates the method’s code with the arguments as formal parameters. This happens every time that a generic function call is evaluated — it isn’t assumed that a given source code call will end up invoking the same method every time.

Defining an identifier as a generic function is done with the `define-generic` macro. Definition of a new method is done with the `define-method` macro. Note that `define-method` automatically does a `define-generic` if the identifier concerned is not already a generic function, so often an explicit `define-generic` call is not needed.

syntax: **define-generic** symbol [¶](08_06_methods_and_generic_functions.md)

Create a generic function with name symbol and bind it to the variable symbol. If symbol was previously bound to a Scheme procedure (or procedure-with-setter), the old procedure (and setter) is incorporated into the new generic function as its default procedure (and setter). Any other previous value, including an existing generic function, is discarded and replaced by a new, empty generic function.

syntax: **define-method** (generic parameter …) body … [¶](08_06_methods_and_generic_functions.md)

Define a method for the generic function or accessor generic with parameters parameters and body body ....

generic is a generic function. If generic is a variable which is not yet bound to a generic function object, the expansion of `define-method` will include a call to `define-generic`. If generic is `(setter generic-with-setter)`, where generic-with-setter is a variable which is not yet bound to a generic-with-setter object, the expansion will include a call to `define-accessor`.

Each parameter must be either a symbol or a two-element list `(symbol class)`. The symbols refer to variables in the body forms that will be bound to the parameters supplied by the caller when calling this method. The classes, if present, specify the possible combinations of parameters to which this method can be applied.

body … are the bodies of the method definition.

`define-method` expressions look a little like Scheme procedure definitions of the form

(define (name formals ...) . body)

The important difference is that each formal parameter, apart from the possible “rest” argument, can be qualified by a class name: `formal` becomes `(formal class)`. The meaning of this qualification is that the method being defined will only be applicable in a particular generic function invocation if the corresponding argument is an instance of `class` (or one of its subclasses). If more than one of the formal parameters is qualified in this way, then the method will only be applicable if each of the corresponding arguments is an instance of its respective qualifying class.

Note that unqualified formal parameters act as though they are qualified by the class `<top>`, which GOOPS uses to mean the superclass of all valid Scheme types, including both primitive types and GOOPS classes.

For example, if a generic function method is defined with parameters `(s1 <square>)` and `(n <number>)`, that method is only applicable to invocations of its generic function that have two parameters where the first parameter is an instance of the `<square>` class and the second parameter is a number.

*   [Accessors](08_06_methods_and_generic_functions.md#861-accessors)
*   [Extending Primitives](08_06_methods_and_generic_functions.md#862-extending-primitives)
*   [Merging Generics](08_06_methods_and_generic_functions.md#863-merging-generics)
*   [Next-method](08_06_methods_and_generic_functions.md#864-next-method)
*   [method\* and define-method\*](08_06_methods_and_generic_functions.md#865-method-and-define-method)
*   [Generic Function and Method Examples](08_06_methods_and_generic_functions.md#866-generic-function-and-method-examples)
*   [Handling Invocation Errors](08_06_methods_and_generic_functions.md#867-handling-invocation-errors)

* * *

Next: [Extending Primitives](08_06_methods_and_generic_functions.md#862-extending-primitives), Up: [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.1 Accessors [¶](08_06_methods_and_generic_functions.md#861-accessors)

An accessor is a generic function that can also be used with the generalized `set!` syntax (see [Procedures with Setters](06_07_procedures.md#678-procedures-with-setters)). Guile will handle a call like

(set! (accessor args...) value)

by calling the most specialized method of `accessor` that matches the classes of `args` and `value`. `define-accessor` is used to bind an identifier to an accessor.

syntax: **define-accessor** symbol [¶](08_06_methods_and_generic_functions.md)

Create an accessor with name symbol and bind it to the variable symbol. If symbol was previously bound to a Scheme procedure (or procedure-with-setter), the old procedure (and setter) is incorporated into the new accessor as its default procedure (and setter). Any other previous value, including an existing generic function or accessor, is discarded and replaced by a new, empty accessor.

* * *

Next: [Merging Generics](08_06_methods_and_generic_functions.md#863-merging-generics), Previous: [Accessors](08_06_methods_and_generic_functions.md#861-accessors), Up: [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.2 Extending Primitives [¶](08_06_methods_and_generic_functions.md#862-extending-primitives)

Many of Guile’s primitive procedures can be extended by giving them a generic function definition that operates in conjunction with their normal C-coded implementation. When a primitive is extended in this way, it behaves like a generic function with the C-coded implementation as its default method.

This extension happens automatically if a method is defined (by a `define-method` call) for a variable whose current value is a primitive. But it can also be forced by calling `enable-primitive-generic!`.

primitive procedure: **enable-primitive-generic!** primitive [¶](08_06_methods_and_generic_functions.md)

Force the creation of a generic function definition for primitive.

Once the generic function definition for a primitive has been created, it can be retrieved using `primitive-generic-generic`.

primitive procedure: **primitive-generic-generic** primitive [¶](08_06_methods_and_generic_functions.md)

Return the generic function definition of primitive.

`primitive-generic-generic` raises an error if primitive is not a primitive with generic capability.

* * *

Next: [Next-method](08_06_methods_and_generic_functions.md#864-next-method), Previous: [Extending Primitives](08_06_methods_and_generic_functions.md#862-extending-primitives), Up: [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.3 Merging Generics [¶](08_06_methods_and_generic_functions.md#863-merging-generics)

GOOPS generic functions and accessors often have short, generic names. For example, if a vector package provides an accessor for the X coordinate of a vector, that accessor may just be called `x`. It doesn’t need to be called, for example, `vector:x`, because GOOPS will work out, when it sees code like `(x obj)`, that the vector-specific method of `x` should be called if obj is a vector.

That raises the question, though, of what happens when different packages define a generic function with the same name. Suppose we work with a graphical package which needs to use two independent vector packages for 2D and 3D vectors respectively. If both packages export `x`, what does the code using those packages end up with?

[duplicate binding handlers](06_18_modules.md#6183-creating-guile-modules) explains how this is resolved for conflicting bindings in general. For generics, there is a special duplicates handler, `merge-generics`, which tells the module system to merge generic functions with the same name. Here is an example:

(define-module (math 2D-vectors)
  #:use-module (oop goops)
  #:export (x y [...](06_08_macros.md)))
		  
(define-module (math 3D-vectors)
  #:use-module (oop goops)
  #:export (x y z [...](06_08_macros.md)))

(define-module (my-module)
  #:use-module (oop goops)
  #:use-module (math 2D-vectors)
  #:use-module (math 3D-vectors)
  #:duplicates (merge-generics))

The generic function `x` in `(my-module)` will now incorporate all of the methods of `x` from both imported modules.

To be precise, there will now be three distinct generic functions named `x`: `x` in `(math 2D-vectors)`, `x` in `(math 3D-vectors)`, and `x` in `(my-module)`; and these functions share their methods in an interesting and dynamic way.

To explain, let’s call the imported generic functions (in `(math 2D-vectors)` and `(math 3D-vectors)`) the _ancestors_, and the merged generic function (in `(my-module)`), the _descendant_. The general rule is that for any generic function G, the applicable methods are selected from the union of the methods of G’s descendant functions, the methods of G itself and the methods of G’s ancestor functions.

Thus ancestor functions effectively share methods with their descendants, and vice versa. In the example above, `x` in `(math 2D-vectors)` will share the methods of `x` in `(my-module)` and vice versa.[33](99_footnotes.md) Sharing is dynamic, so adding another new method to a descendant implies adding it to that descendant’s ancestors too.

* * *

Next: [method\* and define-method\*](08_06_methods_and_generic_functions.md#865-method-and-define-method), Previous: [Merging Generics](08_06_methods_and_generic_functions.md#863-merging-generics), Up: [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.4 Next-method [¶](08_06_methods_and_generic_functions.md#864-next-method)

When you call a generic function, with a particular set of arguments, GOOPS builds a list of all the methods that are applicable to those arguments and orders them by how closely the method definitions match the actual argument types. It then calls the method at the top of this list. If the selected method’s code wants to call on to the next method in this list, it can do so by using `next-method`.

(define-method (test (a <integer>)) ([cons](06_06_08_pairs.md) 'integer (next-method)))
(define-method (test (a <number>))  ([cons](06_06_08_pairs.md) 'number  (next-method)))
(define-method (test a)             ([list](06_06_09_lists.md) 'top))

With these definitions,

(test 1)   ⇒ (integer number top)
(test 1.0) ⇒ (number top)
(test #t)  ⇒ (top)

`next-method` can be called as just `(next-method)`. The arguments for the next method call are then implicit, and the same as for the original method call.

If you want to call on to a method with the same name but with a different set of arguments (as you might with overloaded methods in C++, for example), you can pass custom arguments to `next-method`:

(define-method (test (a <number>) [min](06_06_02_numerical_data_types.md) [max](06_06_02_numerical_data_types.md))
  (if (and ([\>=](06_06_02_numerical_data_types.md) a [min](06_06_02_numerical_data_types.md)) ([<=](06_06_02_numerical_data_types.md) a [max](06_06_02_numerical_data_types.md)))
      ([display](06_16_reading_and_evaluating_scheme_code.md) "Number is in range\\n"))
  (next-method a))

(test 2 1 10)
⊣
Number is [in](04_programming_in_scheme.md) range
⇒
(integer number top)

* * *

Next: [Generic Function and Method Examples](08_06_methods_and_generic_functions.md#866-generic-function-and-method-examples), Previous: [Next-method](08_06_methods_and_generic_functions.md#864-next-method), Up: [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.5 method\* and define-method\* [¶](08_06_methods_and_generic_functions.md#865-method-and-define-method)

`method*` and `define-method*` are the GOOPS versions of `lambda*` and `define*`.

syntax: **method\*** (\[param…\]  
\[#:optional vardef…\]  
\[#:key vardef… \[#:allow-other-keys\]\]  
\[#:rest var | . var\])  
body1 body2 … [¶](08_06_methods_and_generic_functions.md)

  

Create a method which takes optional and/or keyword arguments specified with `#:optional` and `#:key`. See [lambda\* and define\*.](06_07_procedures.md#6741-lambda-and-define). param… are ordinary method parameters as for `method` and `define-method`, i.e. either var or (var typespec).

`define-method*` is syntactic sugar for defining methods using `method*`. For example,

(define-method\* (foo (a <integer>) b #:optional c
                     #:key (d 2) e
                     #:rest f)
  ([list](06_06_09_lists.md) a b c d e f))

is a method with fixed arguments a of type `<integer>` and b of type `<top>`, optional argument c, keyword arguments d, with default value 2, and e, and rest argument f. A call

(foo 1 'x #:d 3 'y)

will return

(1 x #f 3 #f (#:d 3 y))

The values for c and e are `#f` since they have not been given in the call. The given keyword argument(s) are included in the rest argument, as for `define*`.

*   [Advanced argument handling in method and define-method](08_06_methods_and_generic_functions.md#8651-advanced-argument-handling-in-method-and-define-method)
*   [Type dispatch and redefinition for advanced argument handling](08_06_methods_and_generic_functions.md#8652-type-dispatch-and-redefinition-for-advanced-argument-handling)
*   [next-method call in method\*](08_06_methods_and_generic_functions.md#8653-next-method-call-in-method)
*   [Advanced argument handling design choices](08_06_methods_and_generic_functions.md#8654-advanced-argument-handling-design-choices)

* * *

Next: [Type dispatch and redefinition for advanced argument handling](08_06_methods_and_generic_functions.md#8652-type-dispatch-and-redefinition-for-advanced-argument-handling), Up: [method\* and define-method\*](08_06_methods_and_generic_functions.md#865-method-and-define-method)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.5.1 Advanced argument handling in method and define-method [¶](08_06_methods_and_generic_functions.md#8651-advanced-argument-handling-in-method-and-define-method)

Some users may find it more natural not to have to choose between `define-method` and `define-method*` in their code.

It turns out that `method*` and `define-method*` can do just fine also for ordinary methods without keyword formals. They take marginally longer to compile but result in the same code as `method` and `define-method` if keyword formals are absent.

For this reason, we provide a module (oop goops keyword-formals) which replaces the standard `method` and `define-method` bindings with their keyword formal counterparts. It can be used like this:

([use-modules](06_18_modules.md) (oop goops) (oop goops keyword-formals))

or

(define-module (foo)
  #:use-module (oop goops)
  #:use-module (oop goops keyword-formals))

* * *

Next: [next-method call in method\*](08_06_methods_and_generic_functions.md#8653-next-method-call-in-method), Previous: [Advanced argument handling in method and define-method](08_06_methods_and_generic_functions.md#8651-advanced-argument-handling-in-method-and-define-method), Up: [method\* and define-method\*](08_06_methods_and_generic_functions.md#865-method-and-define-method)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.5.2 Type dispatch and redefinition for advanced argument handling [¶](08_06_methods_and_generic_functions.md#8652-type-dispatch-and-redefinition-for-advanced-argument-handling)

As in CLOS, GOOPS only does type dispatch on required arguments, also for method\*. For a `method*` with keyword formals, the list of specializers becomes the same as for a corresponding `method` with the same number of required arguments and a tail/rest argument. For example, the list of specializers for

(define-method\* (foo (a <integer>) b #:optional c #:key d)
  [...](06_08_macros.md))

becomes

(<integer> <top> . <top>)

This means that `(method (a <integer>) b) ...)` is more specialized than `(method (a <integer>) b #:optional c #:key d) ...)` and will come before the latter in the list of applicable methods.

Two methods of the same generic function can’t have the same specializers list. This means that if we do

(define-method\* (foo (a <integer>) b #:optional c)
 [...](06_08_macros.md))
(define-method\* (foo (a <integer>) b . rest)
 [...](06_08_macros.md))

the second `define-method*` will cause a redefinition such that the second method will replace the first.

* * *

Next: [Advanced argument handling design choices](08_06_methods_and_generic_functions.md#8654-advanced-argument-handling-design-choices), Previous: [Type dispatch and redefinition for advanced argument handling](08_06_methods_and_generic_functions.md#8652-type-dispatch-and-redefinition-for-advanced-argument-handling), Up: [method\* and define-method\*](08_06_methods_and_generic_functions.md#865-method-and-define-method)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.5.3 next-method call in method\* [¶](08_06_methods_and_generic_functions.md#8653-next-method-call-in-method)

A `(next-method)` call in a `method*` will pass on all required, optional and rest arguments in the list of formals to the next less specialized method in the list of applicable methods, as well as any actual keyword arguments passed in the call to the `method*`.

This has the following consequences for default values:

1.  A an optional argument will shadow default values for optional arguments in the same position in less specialized methods. For example if <B> is a subclass of <A> and b an instance of <B>,
    
    (define-method\* (foo (obj <A>) #:optional (c 1)) c)
    (define-method\* (foo (obj <B>) #:optional c)) (next-method))
    (foo b) ⇒ #f
    
    The reason for this is that c will obtain the value `#f` already in the call to the second (most specialized, called first) method, and this value will be passed on by the `(next-method)` call.
    
2.  A keyword argument will not shadow a default value for the same keyword argument in less specialized methods. Example:
    
    (define-method\* (foo (obj <A>) #:key (c 1)) c)
    (define-method\* (foo (obj <B>) #:key c)) (next-method))
    (foo b) ⇒ 1
    
    The reason for this is that the first (less specialized, called last) method will not be passed any keyword arguments from the call. In particular, it won’t be passed `#:c VAL`, and it will therefore use its default value.
    

The user can pass arguments to `next-method` to customize the behavior. In particular, if the current method uses advanced argument handling in such a way that it has keywords in its list of formals and the next, less specialized, method is an ordinary `method`, it will be necessary to pass arguments explicitly to `next-method` to “filter out” any keywords.

* * *

Previous: [next-method call in method\*](08_06_methods_and_generic_functions.md#8653-next-method-call-in-method), Up: [method\* and define-method\*](08_06_methods_and_generic_functions.md#865-method-and-define-method)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.5.4 Advanced argument handling design choices [¶](08_06_methods_and_generic_functions.md#8654-advanced-argument-handling-design-choices)

There are two design choices with regard to advanced argument handling in GOOPS methods which are worth explaining.

First, there was the choice whether to add this functionality to `method` and `define-method` proper or to introduce new syntax `method*` and `define-method*`.

There are several reasons why it could be better not to introduce new syntax. It would give a simpler API, with less cognitive load for the user, and a slightly simpler implementation (compared to providing the new syntax as well). It would align with the CLOS design choice where `defmethod` does support advanced argument handling.

Eventually, we decided to introduce new syntax instead of extending the old for the following reasons:

1.  It aligns with `lambda*` and `define*`.
2.  It is somewhat better at protecting backward compatibility.
3.  It preserves the conceptual simplicity of `method` and `define-method`.
4.  It makes it easier for other implementations (like guile-hoot) to choose to only provide the simpler functionality (through `method` and `define-method`).

However, note that users who prefer to only use `define-method` can do so. See [Advanced argument handling in method and define-method](08_06_methods_and_generic_functions.md#8651-advanced-argument-handling-in-method-and-define-method).

Second, we have chosen not to do type dispatch on optional or keyword arguments. Reasons include:

1.  It strikes a reasonable balance with regards to the complexity of implementation (and cognitive load for the user).
2.  It aligns with the CLOS implementation which also likely had good reasons for this choice.
3.  Currently, we don’t have clear ideas about a conceptual framework of rules governing type dispatch for advanced argument handling or how this would be implemented.

* * *

Next: [Handling Invocation Errors](08_06_methods_and_generic_functions.md#867-handling-invocation-errors), Previous: [method\* and define-method\*](08_06_methods_and_generic_functions.md#865-method-and-define-method), Up: [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.6 Generic Function and Method Examples [¶](08_06_methods_and_generic_functions.md#866-generic-function-and-method-examples)

Consider the following definitions:

(define-generic G)
(define-method (G (a <integer>) b) 'integer)
(define-method (G (a <real>) b) 'real)
(define-method (G a b) 'top)

The `define-generic` call defines G as a generic function. The three next lines define methods for G. Each method uses a sequence of _parameter specializers_ that specify when the given method is applicable. A specializer permits to indicate the class a parameter must belong to (directly or indirectly) to be applicable. If no specializer is given, the system defaults it to `<top>`. Thus, the first method definition is equivalent to

(define-method (G (a <integer>) (b <top>)) 'integer)

Now, let’s look at some possible calls to the generic function G:

(G 2 3)    ⇒ integer
(G 2 #t)   ⇒ integer
(G 1.2 'a) ⇒ real
(G #t #f)  ⇒ top
(G 1 2 3)  ⇒ [error](04_programming_in_scheme.md) (since no [method](08_11_the_metaobject_protocol.md) [exists](07_06_r6rs_support.md) for 3 parameters)

The methods above use only one specializer per parameter list. But in general, any or all of a method’s parameters may be specialized. Suppose we define now:

(define-method (G (a <integer>) (b <number>))  'integer-number)
(define-method (G (a <integer>) (b <real>))    'integer-real)
(define-method (G (a <integer>) (b <integer>)) 'integer-integer)
(define-method (G a (b <number>))              'top-number)

With these definitions:

(G 1 2)   ⇒ integer-integer
(G 1 1.0) ⇒ integer-real
(G 1 #t)  ⇒ integer
(G 'a 1)  ⇒ top-number

As a further example we shall continue to define operations on the `<my-complex>` class. Suppose that we want to use it to implement complex numbers completely. For instance a definition for the addition of two complex numbers could be

(define-method (new-+ (a <my-complex>) (b <my-complex>))
  ([make-rectangular](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) ([real-part](06_06_02_numerical_data_types.md) a) ([real-part](06_06_02_numerical_data_types.md) b))
                    ([+](06_06_02_numerical_data_types.md) ([imag-part](06_06_02_numerical_data_types.md) a) ([imag-part](06_06_02_numerical_data_types.md) b))))

To be sure that the `+` used in the method `new-+` is the standard addition we can do:

(define-generic new-+)

(let (([+](06_06_02_numerical_data_types.md) [+](06_06_02_numerical_data_types.md)))
  (define-method (new-+ (a <my-complex>) (b <my-complex>))
    ([make-rectangular](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) ([real-part](06_06_02_numerical_data_types.md) a) ([real-part](06_06_02_numerical_data_types.md) b))
                      ([+](06_06_02_numerical_data_types.md) ([imag-part](06_06_02_numerical_data_types.md) a) ([imag-part](06_06_02_numerical_data_types.md) b)))))

The `define-generic` ensures here that `new-+` will be defined in the global environment. Once this is done, we can add methods to the generic function `new-+` which make a closure on the `+` symbol. A complete writing of the `new-+` methods is shown in [Figure 8.1](08_06_methods_and_generic_functions.md).

(define-generic new-+)

(let (([+](06_06_02_numerical_data_types.md) [+](06_06_02_numerical_data_types.md)))

  (define-method (new-+ (a <real>) (b <real>)) ([+](06_06_02_numerical_data_types.md) a b))

  (define-method (new-+ (a <real>) (b <my-complex>)) 
    ([make-rectangular](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) a ([real-part](06_06_02_numerical_data_types.md) b)) ([imag-part](06_06_02_numerical_data_types.md) b)))

  (define-method (new-+ (a <my-complex>) (b <real>))
    ([make-rectangular](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) ([real-part](06_06_02_numerical_data_types.md) a) b) ([imag-part](06_06_02_numerical_data_types.md) a)))

  (define-method (new-+ (a <my-complex>) (b <my-complex>))
    ([make-rectangular](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) ([real-part](06_06_02_numerical_data_types.md) a) ([real-part](06_06_02_numerical_data_types.md) b))
                      ([+](06_06_02_numerical_data_types.md) ([imag-part](06_06_02_numerical_data_types.md) a) ([imag-part](06_06_02_numerical_data_types.md) b))))

  (define-method (new-+ (a <number>))  a)
  
  (define-method (new-+) 0)

  (define-method (new-+ . args)
    (new-+ ([car](06_06_08_pairs.md) args) 
      ([apply](06_16_reading_and_evaluating_scheme_code.md) new-+ ([cdr](06_06_08_pairs.md) args)))))

([set!](07_06_r6rs_support.md) [+](06_06_02_numerical_data_types.md) new-+)

**Figure 8.1:** Extending `+` to handle complex numbers

We take advantage here of the fact that generic function are not obliged to have a fixed number of parameters. The four first methods implement dyadic addition. The fifth method says that the addition of a single element is this element itself. The sixth method says that using the addition with no parameter always return 0 (as is also true for the primitive `+`). The last method takes an arbitrary number of parameters[34](99_footnotes.md). This method acts as a kind of `reduce`: it calls the dyadic addition on the _car_ of the list and on the result of applying it on its rest. To finish, the `set!` permits to redefine the `+` symbol to our extended addition.

To conclude our implementation (integration?) of complex numbers, we could redefine standard Scheme predicates in the following manner:

(define-method ([complex?](06_06_02_numerical_data_types.md) c <my-complex>) #t)
(define-method ([complex?](06_06_02_numerical_data_types.md) c)           #f)

(define-method ([number?](06_06_02_numerical_data_types.md) n <number>) #t)
(define-method ([number?](06_06_02_numerical_data_types.md) n)          #f)
[...](06_08_macros.md)

Standard primitives in which complex numbers are involved could also be redefined in the same manner.

* * *

Previous: [Generic Function and Method Examples](08_06_methods_and_generic_functions.md#866-generic-function-and-method-examples), Up: [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 8.6.7 Handling Invocation Errors [¶](08_06_methods_and_generic_functions.md#867-handling-invocation-errors)

If a generic function is invoked with a combination of parameters for which there is no applicable method, GOOPS raises an error.

generic: **no-method** [¶](08_06_methods_and_generic_functions.md)

method: **no-method** (gf <generic>) args [¶](08_06_methods_and_generic_functions.md)

When an application invokes a generic function, and no methods at all have been defined for that generic function, GOOPS calls the `no-method` generic function. The default method calls `goops-error` with an appropriate message.

generic: **no-applicable-method** [¶](08_06_methods_and_generic_functions.md)

method: **no-applicable-method** (gf <generic>) args [¶](08_06_methods_and_generic_functions.md)

When an application applies a generic function to a set of arguments, and no methods have been defined for those argument types, GOOPS calls the `no-applicable-method` generic function. The default method calls `goops-error` with an appropriate message.

generic: **no-next-method** [¶](08_06_methods_and_generic_functions.md)

method: **no-next-method** (gf <generic>) args [¶](08_06_methods_and_generic_functions.md)

When a generic function method calls `(next-method)` to invoke the next less specialized method for that generic function, and no less specialized methods have been defined for the current generic function arguments, GOOPS calls the `no-next-method` generic function. The default method calls `goops-error` with an appropriate message.

* * *

Next: [Introspection](08_08_introspection.md#88-introspection), Previous: [Methods and Generic Functions](08_06_methods_and_generic_functions.md#86-methods-and-generic-functions), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
