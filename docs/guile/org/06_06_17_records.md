#### 6.6.17 Records [¶](06_06_17_records.md#6617-records)

A _record type_ is a first class object representing a user-defined data type. A _record_ is an instance of a record type.

Note that in many ways, this interface is too low-level for every-day use. Most uses of records are better served by SRFI-9 records. See [SRFI-9 Records](06_06_16_srfi9_records.md#6616-srfi-9-records).

Scheme Procedure: **record?** obj [¶](06_06_17_records.md)

Return `#t` if obj is a record of any type and `#f` otherwise.

Note that `record?` may be true of any Scheme value; there is no promise that records are disjoint with other Scheme types.

Scheme Procedure: **make-record-type** type-name field-names \[print\] \[#:parent=`#f`\] \[#:uid=`#f`\] \[#:extensible?=`#f`\] \[#:opaque?=`#f`\] \[#:allow-duplicate-field-names?=`#t`\] [¶](06_06_17_records.md)

Create and return a new _record-type descriptor_.

type-name is a string naming the type. Currently it’s only used in the printed representation of records, and in diagnostics. field-names is a list of elements of the form `(immutable name)`, `(mutable name)`, or name, where name are symbols naming the fields of a record of the type. Duplicates are not allowed among these symbols, unless allow-duplicate-field-names? is true.

(make-record-type "employee" '(name age salary))

The optional print argument is a function used by `display`, `write`, etc, for printing a record of the new type. It’s called as `(print record port)` and should look at record and write to port.

Pass the `#:parent` keyword to derive a record type from a supertype. A derived record type has the fields from its parent type, followed by fields declared in the `make-record-type` call. Record predicates and field accessors for instance of a parent type will also work on any instance of a subtype.

Allowing record subtyping has a small amount of overhead. To avoid this overhead, prevent extensibility by passing `#:extensible? #f`. By default, record types in Guile are not extensible.

Generally speaking, calling `make-record-type` returns a fresh record type; it _generates_ new record types. However sometimes you only want to define a record type if one hasn’t been defined already. For a _nongenerative_ record type definition, pass a symbol as the `#:uid` keyword parameter. If a record with the given uid was already defined, it will be returned instead. The type name, fields, parent (if any), and so on for the previously-defined type must be compatible.

R6RS defines a notion of “opaque” record types. Given an instance of an opaque record type, one cannot obtain a run-time representation of the record type. See [rnrs records procedural](07_06_r6rs_support.md#76210-rnrs-records-procedural), for full details. The `#:opaque?` flag is used by Guile’s R6RS layer to record this information. The default is determined by whether the parent type, if any, was opaque.

Fields are mutable by default, meaning that `record-modifier` will return a procedure that can update a record in place. Specifying a field using the form `(immutable name)` instead marks a field as immutable.

Scheme Procedure: **record-constructor** rtd [¶](06_06_17_records.md)

Return a procedure for constructing new members of the type represented by rtd. The result will be a procedure accepting exactly as many arguments as there are fields in the record type.

Scheme Procedure: **record-predicate** rtd [¶](06_06_17_records.md)

Return a procedure for testing membership in the type represented by rtd. The returned procedure accepts exactly one argument and returns a true value if the argument is a member of the indicated record type; it returns a false value otherwise.

Scheme Procedure: **record-accessor** rtd field-name [¶](06_06_17_records.md)

Return a procedure for reading the value of a particular field of a member of the type represented by rtd. The returned procedure accepts exactly one argument which must be a record of the appropriate type; it returns the current value of the field named by the symbol field-name in that record.

If field-name is a symbol, it must be a member of the list of field-names in the call to `make-record-type` that created the type represented by rtd. If multiple fields in rtd have the same name, `record-accessor` returns the first one.

If field-name is an integer, it should be an index into `(record-type-fields rtd)`. This allows accessing fields with duplicate names.

Scheme Procedure: **record-modifier** rtd field-name [¶](06_06_17_records.md)

Return a procedure for writing the value of a particular field of a member of the type represented by rtd. The returned procedure accepts exactly two arguments: first, a record of the appropriate type, and second, an arbitrary Scheme value; it modifies the field named by the symbol field-name in that record to contain the given value. The returned value of the modifier procedure is unspecified. The symbol field-name is a field name or a field index, as in `record-modifier`.

Scheme Procedure: **record-type-descriptor** record [¶](06_06_17_records.md)

Return a record-type descriptor representing the type of the given record. That is, for example, if the returned descriptor were passed to `record-predicate`, the resulting predicate would return a true value when passed the given record. Note that it is not necessarily the case that the returned descriptor is the one that was passed to `record-constructor` in the call that created the constructor procedure that created the given record.

Scheme Procedure: **record-type-name** rtd [¶](06_06_17_records.md)

Return the type-name associated with the type represented by rtd. The returned value is `eqv?` to the type-name argument given in the call to `make-record-type` that created the type represented by rtd.

Scheme Procedure: **record-type-fields** rtd [¶](06_06_17_records.md)

Return a list of the symbols naming the fields in members of the type represented by rtd. The returned value is `equal?` to the field-names argument given in the call to `make-record-type` that created the type represented by rtd.

* * *

Next: [Dictionary Types](06_06_19_dictionary_types.md#6619-dictionary-types), Previous: [Records](06_06_17_records.md#6617-records), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
