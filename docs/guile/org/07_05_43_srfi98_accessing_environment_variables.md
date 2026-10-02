#### 7.5.43 SRFI-98 Accessing environment variables. [¶](07_05_43_srfi98_accessing_environment_variables.md#7543-srfi-98-accessing-environment-variables)

This is a portable wrapper around Guile’s built-in support for interacting with the current environment, See [Runtime Environment](07_02_06_runtime_environment.md#726-runtime-environment).

Scheme Procedure: **get-environment-variable** name [¶](07_05_43_srfi98_accessing_environment_variables.md)

Returns a string containing the value of the environment variable given by the string `name`, or `#f` if the named environment variable is not found. This is equivalent to `(getenv name)`.

Scheme Procedure: **get-environment-variables** [¶](07_05_43_srfi98_accessing_environment_variables.md)

Returns the names and values of all the environment variables as an association list in which both the keys and the values are strings.

* * *

Next: [SRFI-111 Boxes.](07_05_45_srfi111_boxes.md#7545-srfi-111-boxes), Previous: [SRFI-98 Accessing environment variables.](07_05_43_srfi98_accessing_environment_variables.md#7543-srfi-98-accessing-environment-variables), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
