### 6.2 Deprecation [¶](06_02_deprecation.md#62-deprecation)

From time to time functions and other features of Guile become obsolete. Guile’s _deprecation_ is a mechanism that can help you cope with this.

When you use a feature that is deprecated, you will likely get a warning message at run-time. Also, if you have a new enough toolchain, using a deprecated function from `libguile` will cause a link-time warning.

The primary source for information about just what interfaces are deprecated in a given release is the file NEWS. That file also documents what you should use instead of the obsoleted things.

The file README contains instructions on how to control the inclusion or removal of the deprecated features from the public API of Guile, and how to control the deprecation warning messages.

The idea behind this mechanism is that normally all deprecated interfaces are available, but you get feedback when compiling and running code that uses them, so that you can migrate to the newer APIs at your leisure.

* * *

Next: [Initializing Guile](06_04_initializing_guile.md#64-initializing-guile), Previous: [Deprecation](06_02_deprecation.md#62-deprecation), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
