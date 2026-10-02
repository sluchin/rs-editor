### 8.5 Illustrating Slot Description [¶](08_05_illustrating_slot_description.md#85-illustrating-slot-description)

To illustrate slot description, we can redefine the `<my-complex>` class seen before. A definition could be:

(define-class <my-complex> (<number>) 
   (r #:init-value 0 #:getter get-r #:setter set-r! #:init-keyword #:r)
   (i #:init-value 0 #:getter get-i #:setter set-i! #:init-keyword #:i))

With this definition, the `r` and `i` slots are set to 0 by default, and can be initialised to other values by calling `make` with the `#:r` and `#:i` keywords. Also the generic functions `get-r`, `set-r!`, `get-i` and `set-i!` are automatically defined to read and write the slots.

(define c1 ([make](08_03_instance_creation_and_slot_access.md) <my-complex> #:r 1 #:i 2))
(get-r c1) ⇒ 1
(set-r! c1 12)
(get-r c1) ⇒ 12
(define c2 ([make](08_03_instance_creation_and_slot_access.md) <my-complex> #:r 2))
(get-r c2) ⇒ 2
(get-i c2) ⇒ 0

Accessors can both read and write a slot. So, another definition of the `<my-complex>` class, using the `#:accessor` option, could be:

(define-class <my-complex> (<number>) 
   (r #:init-value 0 #:accessor [real-part](06_06_02_numerical_data_types.md) #:init-keyword #:r)
   (i #:init-value 0 #:accessor [imag-part](06_06_02_numerical_data_types.md) #:init-keyword #:i))

With this definition, the `r` slot can be read with:

([real-part](06_06_02_numerical_data_types.md) c)

and set with:

([set!](07_06_r6rs_support.md) ([real-part](06_06_02_numerical_data_types.md) c) new-value)

Suppose now that we want to manipulate complex numbers with both rectangular and polar coordinates. One solution could be to have a definition of complex numbers which uses one particular representation and some conversion functions to pass from one representation to the other. A better solution is to use virtual slots, like this:

(define-class <my-complex> (<number>)
   ;; True slots use rectangular coordinates
   (r #:init-value 0 #:accessor [real-part](06_06_02_numerical_data_types.md) #:init-keyword #:r)
   (i #:init-value 0 #:accessor [imag-part](06_06_02_numerical_data_types.md) #:init-keyword #:i)
   ;; Virtual slots access do the conversion
   (m #:accessor [magnitude](06_06_02_numerical_data_types.md) #:init-keyword #:magn  
      #:allocation #:virtual
      #:slot-ref (lambda (o)
                  (let ((r ([slot-ref](08_08_introspection.md) o 'r)) (i ([slot-ref](08_08_introspection.md) o 'i)))
                    ([sqrt](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) ([\*](06_06_02_numerical_data_types.md) r r) ([\*](06_06_02_numerical_data_types.md) i i)))))
      #:slot-set! (lambda (o m)
                    (let ((a ([slot-ref](08_08_introspection.md) o 'a)))
                      ([slot-set!](08_08_introspection.md) o 'r ([\*](06_06_02_numerical_data_types.md) m ([cos](06_06_02_numerical_data_types.md) a)))
                      ([slot-set!](08_08_introspection.md) o 'i ([\*](06_06_02_numerical_data_types.md) m ([sin](06_06_02_numerical_data_types.md) a))))))
   (a #:accessor [angle](06_06_02_numerical_data_types.md) #:init-keyword #:angle
      #:allocation #:virtual
      #:slot-ref (lambda (o)
                  ([atan](06_06_02_numerical_data_types.md) ([slot-ref](08_08_introspection.md) o 'i) ([slot-ref](08_08_introspection.md) o 'r)))
      #:slot-set! (lambda(o a)
                   (let ((m ([slot-ref](08_08_introspection.md) o 'm)))
                      ([slot-set!](08_08_introspection.md) o 'r ([\*](06_06_02_numerical_data_types.md) m ([cos](06_06_02_numerical_data_types.md) a)))
                      ([slot-set!](08_08_introspection.md) o 'i ([\*](06_06_02_numerical_data_types.md) m ([sin](06_06_02_numerical_data_types.md) a)))))))

In this class definition, the magnitude `m` and angle `a` slots are virtual, and are calculated, when referenced, from the normal (i.e. `#:allocation #:instance`) slots `r` and `i`, by calling the function defined in the relevant `#:slot-ref` option. Correspondingly, writing `m` or `a` leads to calling the function defined in the `#:slot-set!` option. Thus the following expression

([slot-set!](08_08_introspection.md) c 'a 3)

permits to set the angle of the `c` complex number.

(define c ([make](08_03_instance_creation_and_slot_access.md) <my-complex> #:r 12 #:i 20))
([real-part](06_06_02_numerical_data_types.md) c) ⇒ 12
([angle](06_06_02_numerical_data_types.md) c) ⇒ 1.03037682652431
([slot-set!](08_08_introspection.md) c 'i 10)
([set!](07_06_r6rs_support.md) ([real-part](06_06_02_numerical_data_types.md) c) 1)
([describe](04_programming_in_scheme.md) c)
⊣
#<<my-complex> 401e9b58> is an instance of [class](08_11_the_metaobject_protocol.md) <my-complex>
Slots are: 
     r [\=](06_06_02_numerical_data_types.md) 1
     i [\=](06_06_02_numerical_data_types.md) 10
     m [\=](06_06_02_numerical_data_types.md) 10.0498756211209
     a [\=](06_06_02_numerical_data_types.md) 1.47112767430373

Since initialization keywords have been defined for the four slots, we can now define the standard Scheme primitives `make-rectangular` and `make-polar`.

(define [make-rectangular](06_06_02_numerical_data_types.md) 
   (lambda (x y) ([make](08_03_instance_creation_and_slot_access.md) <my-complex> #:r x #:i y)))

(define [make-polar](06_06_02_numerical_data_types.md)
   (lambda (x y) ([make](08_03_instance_creation_and_slot_access.md) <my-complex> #:magn x #:angle y)))

* * *

Next: [Inheritance](08_07_inheritance.md#87-inheritance), Previous: [Illustrating Slot Description](08_05_illustrating_slot_description.md#85-illustrating-slot-description), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
