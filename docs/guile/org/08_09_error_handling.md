### 8.9 Error Handling [¶](08_09_error_handling.md#89-error-handling)

The procedure `goops-error` is called to raise an appropriate error by the default methods of the following generic functions:

*   `slot-missing` (see [slot-missing](08_08_introspection.md#885-accessing-slots))
*   `slot-unbound` (see [slot-unbound](08_08_introspection.md#885-accessing-slots))
*   `no-method` (see [no-method](08_06_methods_and_generic_functions.md#867-handling-invocation-errors))
*   `no-applicable-method` (see [no-applicable-method](08_06_methods_and_generic_functions.md#867-handling-invocation-errors))
*   `no-next-method` (see [no-next-method](08_06_methods_and_generic_functions.md#867-handling-invocation-errors))

If you customize these functions for particular classes or metaclasses, you may still want to use `goops-error` to signal any error conditions that you detect.

procedure: **goops-error** format-string arg … [¶](08_09_error_handling.md)

Raise an error with key `goops-error` and error message constructed from format-string and arg .... Error message formatting is as done by `scm-error`.

* * *

Next: [The Metaobject Protocol](08_11_the_metaobject_protocol.md#811-the-metaobject-protocol), Previous: [Error Handling](08_09_error_handling.md#89-error-handling), Up: [GOOPS](08_00_goops.md#8-goops)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
