#### 6.6.16 SRFI-9 Records [¶](06_06_16_srfi9_records.md#6616-srfi-9-records)

SRFI-9 standardizes a syntax for defining new record types and creating predicate, constructor, and field getter and setter functions. In Guile this is the recommended option to create new record types (see [Record Overview](06_06_15_record_overview.md#6615-record-overview)). It can be used with:

(use-modules (srfi srfi-9))

Scheme Syntax: **define-record-type** type  
(constructor fieldname …)  
predicate  
(fieldname accessor \[modifier\]) … [¶](06_06_16_srfi9_records.md)

  

Create a new record type, and make various `define`s for using it. This syntax can only occur at the top-level, not nested within some other form.

type is bound to the record type, which is as per the return from the core `make-record-type`. type also provides the name for the record, as per `record-type-name`.

constructor is bound to a function to be called as `(constructor fieldval …)` to create a new record of this type. The arguments are initial values for the fields, one argument for each field, in the order they appear in the `define-record-type` form.

The fieldnames provide the names for the record fields, as per the core `record-type-fields` etc, and are referred to in the subsequent accessor/modifier forms.

predicate is bound to a function to be called as `(predicate obj)`. It returns `#t` or `#f` according to whether obj is a record of this type.

Each accessor is bound to a function to be called `(accessor record)` to retrieve the respective field from a record. Similarly each modifier is bound to a function to be called `(modifier record val)` to set the respective field in a record.

An example will illustrate typical usage,

(define-record-type <employee>
  (make-employee name age salary)
  employee?
  (name    employee-name)
  (age     employee-age    set-employee-age!)
  (salary  employee-salary set-employee-salary!))

This creates a new employee data type, with name, age and salary fields. Accessor functions are created for each field, but no modifier function for the name (the intention in this example being that it’s established only when an employee object is created). These can all then be used as for example,

<employee> ⇒ #<record-type <employee>>

(define fred (make-employee "Fred" 45 20000.00))

(employee? fred)        ⇒ #t
(employee-age fred)     ⇒ 45
(set-employee-salary! fred 25000.00)  ;; pay rise

The functions created by `define-record-type` are ordinary top-level `define`s. They can be redefined or `set!` as desired, exported from a module, etc.

*   [Non-toplevel Record Definitions](06_06_16_srfi9_records.md#non-toplevel-record-definitions)
*   [Custom Printers](06_06_16_srfi9_records.md#custom-printers)
*   [Functional “Setters”](06_06_16_srfi9_records.md#functional-setters)

#### Non-toplevel Record Definitions [¶](06_06_16_srfi9_records.md#non-toplevel-record-definitions)

The SRFI-9 specification explicitly disallows record definitions in a non-toplevel context, such as inside `lambda` body or inside a let block. However, Guile’s implementation does not enforce that restriction.

#### Custom Printers [¶](06_06_16_srfi9_records.md#custom-printers)

You may use `set-record-type-printer!` to customize the default printing behavior of records. This is a Guile extension and is not part of SRFI-9. It is located in the `(srfi srfi-9 gnu)` module.

Scheme Syntax: **set-record-type-printer!** type proc [¶](06_06_16_srfi9_records.md)

Where type corresponds to the first argument of `define-record-type`, and proc is a procedure accepting two arguments, the record to print, and an output port.

This example prints the employee’s name in brackets, for instance `[Fred]`.

(set-record-type-printer! <employee>
  (lambda (record port)
    (write-char #\\\[ port)
    (display (employee-name record) port)
    (write-char #\\\] port)))

#### Functional “Setters” [¶](06_06_16_srfi9_records.md#functional-setters)

When writing code in a functional style, it is desirable to never alter the contents of records. For such code, a simple way to return new record instances based on existing ones is highly desirable.

The `(srfi srfi-9 gnu)` module extends SRFI-9 with facilities to return new record instances based on existing ones, only with one or more field values changed—_functional setters_. First, the `define-immutable-record-type` works like `define-record-type`, except that fields are immutable and setters are defined as functional setters.

Scheme Syntax: **define-immutable-record-type** type  
(constructor fieldname …)  
predicate  
(fieldname accessor \[modifier\]) … [¶](06_06_16_srfi9_records.md)

Define type as a new record type, like `define-record-type`. However, the record type is made _immutable_ (records may not be mutated, even with `struct-set!`), and any modifier is defined to be a functional setter—a procedure that returns a new record instance with the specified field changed, and leaves the original unchanged (see example below.)

In addition, the generic `set-field` and `set-fields` macros may be applied to any SRFI-9 record.

Scheme Syntax: **set-field** record (field sub-fields ...) value [¶](06_06_16_srfi9_records.md)

Return a new record of record’s type whose fields are equal to the corresponding fields of record except for the one specified by field.

field must be the name of the getter corresponding to the field of record being “set”. Subsequent sub-fields must be record getters designating sub-fields within that field value to be set (see example below.)

Scheme Syntax: **set-fields** record ((field sub-fields ...) value) ... [¶](06_06_16_srfi9_records.md)

Like `set-field`, but can be used to set more than one field at a time. This expands to code that is more efficient than a series of single `set-field` calls.

To illustrate the use of functional setters, let’s assume these two record type definitions:

(define-record-type <address>
  (address street city country)
  address?
  (street  address-street)
  (city    address-city)
  (country address-country))

(define-immutable-record-type <person>
  (person age email address)
  person?
  (age     person-age set-person-age)
  (email   person-email set-person-email)
  (address person-address set-person-address))

First, note that the `<person>` record type definition introduces named functional setters. These may be used like this:

(define fsf-address
  (address "Franklin Street" "Boston" "USA"))

(define rms
  (person 30 "rms@gnu.org" fsf-address))

(and (equal? (set-person-age rms 60)
             (person 60 "rms@gnu.org" fsf-address))
     (= (person-age rms) 30))
⇒ #t

Here, the original `<person>` record, to which rms is bound, is left unchanged.

Now, suppose we want to change both the street and age of rms. This can be achieved using `set-fields`:

(set-fields rms
  ((person-age) 60)
  ((person-address address-street) "Temple Place"))
⇒ #<<person> age: 60 email: "rms@gnu.org"
  address: #<<address> street: "Temple Place" city: "Boston" country: "USA">>

Notice how the above changed two fields of rms, including the `street` field of its `address` field, in a concise way. Also note that `set-fields` works equally well for types defined with just `define-record-type`.

* * *

Next: [Structures](06_06_18_structures.md#6618-structures), Previous: [SRFI-9 Records](06_06_16_srfi9_records.md#6616-srfi-9-records), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
