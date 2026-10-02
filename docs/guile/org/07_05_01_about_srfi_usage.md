#### 7.5.1 About SRFI Usage [¶](07_05_01_about_srfi_usage.md#751-about-srfi-usage)

SRFI support in Guile is currently implemented partly in the core library, and partly as add-on modules. That means that some SRFIs are automatically available when the interpreter is started, whereas the other SRFIs require you to use the appropriate support module explicitly.

There are several reasons for this inconsistency. First, the feature checking syntactic form `cond-expand` (see [SRFI-0 - cond-expand](07_05_02_subsection_2.md#752-srfi-0---cond-expand)) must be available immediately, because it must be there when the user wants to check for the Scheme implementation, that is, before she can know that it is safe to use `use-modules` to load SRFI support modules. The second reason is that some features defined in SRFIs had been implemented in Guile before the developers started to add SRFI implementations as modules (for example SRFI-13 (see [SRFI-13 - String Library](07_05_11_srfi13_string_library.md#7511-srfi-13---string-library))). In the future, it is possible that SRFIs in the core library might be factored out into separate modules, requiring explicit module loading when they are needed. So you should be prepared to have to use `use-modules` someday in the future to access SRFI-13 bindings. If you want, you can do that already. We have included the module `(srfi srfi-13)` in the distribution, which currently does nothing, but ensures that you can write future-safe code.

Generally, support for a specific SRFI is made available by using modules named `(srfi srfi-number)`, where number is the number of the SRFI needed. Another possibility is to use the command line option `--use-srfi`, which will load the necessary modules automatically (see [Invoking Guile](04_programming_in_scheme.md#42-invoking-guile)).

* * *

Next: [SRFI-1 - List library](07_05_03_srfi1_list_library.md#753-srfi-1---list-library), Previous: [About SRFI Usage](07_05_01_about_srfi_usage.md#751-about-srfi-usage), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
