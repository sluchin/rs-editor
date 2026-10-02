#### 7.5.24 SRFI-35 - Conditions [¶](07_05_24_srfi35_conditions.md#7524-srfi-35---conditions)

[SRFI-35](http://srfi.schemers.org/srfi-35/srfi-35.html) defines _conditions_, a data structure akin to records designed to convey information about exceptional conditions between parts of a program. It is normally used in conjunction with SRFI-34’s `raise`:

([raise](07_02_08_signals.md) ([condition](07_05_24_srfi35_conditions.md) ([&message](07_06_r6rs_support.md)
                    (message "An error occurred"))))

Users can define _condition types_ containing arbitrary information. Condition types may inherit from one another. This allows the part of the program that handles (or “catches”) conditions to get accurate information about the exceptional condition that arose.

SRFI-35 conditions are made available using:

([use-modules](06_18_modules.md) (srfi srfi-35))

The procedures available to manipulate condition types are the following:

Scheme Procedure: **make-condition-type** id parent field-names [¶](07_05_24_srfi35_conditions.md)

Return a new condition type named id, inheriting from parent, and with the fields whose names are listed in field-names. field-names must be a list of symbols and must not contain names already used by parent or one of its supertypes.

Scheme Procedure: **condition-type?** obj [¶](07_05_24_srfi35_conditions.md)

Return true if obj is a condition type.

Conditions can be created and accessed with the following procedures:

Scheme Procedure: **make-condition** type . field+value [¶](07_05_24_srfi35_conditions.md)

Return a new condition of type type with fields initialized as specified by field+value, a sequence of field names (symbols) and values as in the following example:

(let ((&ct ([make-condition-type](07_05_24_srfi35_conditions.md) 'foo [&condition](07_06_r6rs_support.md) '(a b c))))
  ([make-condition](07_05_24_srfi35_conditions.md) &ct 'a 1 'b 2 'c 3))

Note that all fields of type and its supertypes must be specified.

Scheme Procedure: **make-compound-condition** condition1 condition2 … [¶](07_05_24_srfi35_conditions.md)

Return a new compound condition composed of condition1 condition2 .... The returned condition has the type of each condition of condition1 condition2 … (per `condition-has-type?`).

Scheme Procedure: **condition-has-type?** c type [¶](07_05_24_srfi35_conditions.md)

Return true if condition c has type type.

Scheme Procedure: **condition-ref** c field-name [¶](07_05_24_srfi35_conditions.md)

Return the value of the field named field-name from condition c.

If c is a compound condition and several underlying condition types contain a field named field-name, then the value of the first such field is returned, using the order in which conditions were passed to `make-compound-condition`.

Scheme Procedure: **extract-condition** c type [¶](07_05_24_srfi35_conditions.md)

Return a condition of condition type type with the field values specified by c.

If c is a compound condition, extract the field values from the subcondition belonging to type that appeared first in the call to `make-compound-condition` that created the condition.

Convenience macros are also available to create condition types and conditions.

library syntax: **define-condition-type** type supertype predicate field-spec... [¶](07_05_24_srfi35_conditions.md)

Define a new condition type named type that inherits from supertype. In addition, bind predicate to a type predicate that returns true when passed a condition of type type or any of its subtypes. field-spec must have the form `(field accessor)` where field is the name of field of type and accessor is the name of a procedure to access field field in conditions of type type.

The example below defines condition type `&foo`, inheriting from `&condition` with fields `a`, `b` and `c`:

(define-condition-type &foo [&condition](07_06_r6rs_support.md)
  foo-condition?
  (a  foo-a)
  (b  foo-b)
  (c  foo-c))

library syntax: **condition** type-field-binding1 type-field-binding2 … [¶](07_05_24_srfi35_conditions.md)

Return a new condition or compound condition, initialized according to type-field-binding1 type-field-binding2 .... Each type-field-binding must have the form `(type field-specs...)`, where type is the name of a variable bound to a condition type; each field-spec must have the form `(field-name value)` where field-name is a symbol denoting the field being initialized to value. As for `make-condition`, all fields must be specified.

The following example returns a simple condition:

([condition](07_05_24_srfi35_conditions.md) ([&message](07_06_r6rs_support.md) (message "An error occurred")))

The one below returns a compound condition:

([condition](07_05_24_srfi35_conditions.md) ([&message](07_06_r6rs_support.md) (message "An error occurred"))
           ([&serious](07_06_r6rs_support.md)))

Finally, SRFI-35 defines a several standard condition types.

Variable: **&condition** [¶](07_05_24_srfi35_conditions.md)

This condition type is the root of all condition types. It has no fields.

Variable: **&message** [¶](07_05_24_srfi35_conditions.md)

A condition type that carries a message describing the nature of the condition to humans.

Scheme Procedure: **message-condition?** c [¶](07_05_24_srfi35_conditions.md)

Return true if c is of type `&message` or one of its subtypes.

Scheme Procedure: **condition-message** c [¶](07_05_24_srfi35_conditions.md)

Return the message associated with message condition c.

Variable: **&serious** [¶](07_05_24_srfi35_conditions.md)

This type describes conditions serious enough that they cannot safely be ignored. It has no fields.

Scheme Procedure: **serious-condition?** c [¶](07_05_24_srfi35_conditions.md)

Return true if c is of type `&serious` or one of its subtypes.

Variable: **&error** [¶](07_05_24_srfi35_conditions.md)

This condition describes errors, typically caused by something that has gone wrong in the interaction of the program with the external world or the user.

Scheme Procedure: **error?** c [¶](07_05_24_srfi35_conditions.md)

Return true if c is of type `&error` or one of its subtypes.

As an implementation note, condition objects in Guile are the same as “exception objects”. See [Exception Objects](06_11_controlling_the_flow_of_program_execution.md#61181-exception-objects). The `&condition`, `&serious`, and `&error` condition types are known in core Guile as `&exception`, `&error`, and `&external-error`, respectively.

* * *

Next: [SRFI-38 - External Representation for Data With Shared Structure](07_05_26_srfi38_external_representation_for_data_with_shared_structure.md#7526-srfi-38---external-representation-for-data-with-shared-structure), Previous: [SRFI-35 - Conditions](07_05_24_srfi35_conditions.md#7524-srfi-35---conditions), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
