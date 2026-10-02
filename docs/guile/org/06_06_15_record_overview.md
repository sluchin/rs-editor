#### 6.6.15 Record Overview [¶](06_06_15_record_overview.md#6615-record-overview)

_Records_, also called _structures_, are Scheme’s primary mechanism to define new disjoint types. A _record type_ defines a list of _fields_ that instances of the type consist of. This is like C’s `struct`.

Historically, Guile has offered several different ways to define record types and to create records, offering different features, and making different trade-offs. Over the years, each “standard” has also come with its own new record interface, leading to a maze of record APIs.

At the highest level is SRFI-9, a high-level record interface implemented by most Scheme implementations (see [SRFI-9 Records](06_06_16_srfi9_records.md#6616-srfi-9-records)). It defines a simple and efficient syntactic abstraction of record types and their associated type predicate, fields, and field accessors. SRFI-9 is suitable for most uses, and this is the recommended way to create record types in Guile. Similar high-level record APIs include SRFI-35 (see [SRFI-35 - Conditions](07_05_24_srfi35_conditions.md#7524-srfi-35---conditions)) and R6RS records (see [rnrs records syntactic](07_06_r6rs_support.md#7629-rnrs-records-syntactic)).

Then comes Guile’s historical “records” API (see [Records](06_06_17_records.md#6617-records)). Record types defined this way are first-class objects. Introspection facilities are available, allowing users to query the list of fields or the value of a specific field at run-time, without prior knowledge of the type.

Finally, the common denominator of these interfaces is Guile’s _structure_ API (see [Structures](06_06_18_structures.md#6618-structures)). Guile’s structures are the low-level building block for all other record APIs. Application writers will normally not need to use it.

Records created with these APIs may all be pattern-matched using Guile’s standard pattern matcher (see [Pattern Matching](07_08_pattern_matching.md#78-pattern-matching)).

* * *

Next: [Records](06_06_17_records.md#6617-records), Previous: [Record Overview](06_06_15_record_overview.md#6615-record-overview), Up: [Data Types](06_06_00_data_types.md#66-data-types)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
