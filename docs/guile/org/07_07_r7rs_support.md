### 7.7 R7RS Support [¶](07_07_r7rs_support.md#77-r7rs-support)

The [R7RS](https://small.r7rs.org/) standard is essentially R5RS (directly supported by Guile), plus a module facility, plus an organization of bindings into a standard set of modules.

Happily, the syntax for R7RS modules was chosen to be compatible with R6RS, and so Guile’s documentation there applies. See [R6RS Libraries](06_18_modules.md#6186-r6rs-libraries), for more information on how to define R6RS libraries, and their integration with Guile modules. See [Library Usage](07_06_r6rs_support.md#7621-library-usage), also.

*   [Incompatibilities with the R7RS](07_07_r7rs_support.md#771-incompatibilities-with-the-r7rs)
*   [R7RS Standard Libraries](07_07_r7rs_support.md#772-r7rs-standard-libraries)

* * *

Next: [R7RS Standard Libraries](07_07_r7rs_support.md#772-r7rs-standard-libraries), Up: [R7RS Support](07_07_r7rs_support.md#77-r7rs-support)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.7.1 Incompatibilities with the R7RS [¶](07_07_r7rs_support.md#771-incompatibilities-with-the-r7rs)

As the R7RS is a much less ambitious standard than the R6RS (see [Guile and Scheme](01_introduction.md#11-guile-and-scheme)), it is very easy for Guile to support. As such, Guile is a fully conforming implementation of R7RS, with the exception of the occasional bug and a couple of unimplemented features:

*   The R7RS specifies a syntax for reading circular data structures using _datum labels_, such as `#0=(1 2 3 . #0#)`. Guile’s reader does not support this syntax currently; [https://bugs.gnu.org/38236](https://bugs.gnu.org/38236).
*   As with R6RS, a number of lexical features of R7RS conflict with Guile’s historical syntax. In addition to `r6rs-hex-escapes` and `hungry-eol-escapes` (see [Incompatibilities with the R6RS](07_06_r6rs_support.md#761-incompatibilities-with-the-r6rs)), the `r7rs-symbols` reader feature needs to be explicitly enabled.

Guile exposes a procedure in the root module to choose R7RS defaults over Guile’s historical defaults.

Scheme Procedure: **install-r7rs!** [¶](07_07_r7rs_support.md)

Alter Guile’s default settings to better conform to the R7RS.

While Guile’s defaults may evolve over time, the current changes that this procedure imposes are to add `.sls` and `.guile.sls` to the set of supported `%load-extensions`, to better support R7RS conventions. See [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths). `install-r7rs!` will also enable the reader options mentioned above.

Finally, note that the `--r7rs` command-line argument will call `install-r7rs!` before calling user code. R7RS users probably want to pass this argument to their Guile.

* * *

Previous: [Incompatibilities with the R7RS](07_07_r7rs_support.md#771-incompatibilities-with-the-r7rs), Up: [R7RS Support](07_07_r7rs_support.md#77-r7rs-support)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.7.2 R7RS Standard Libraries [¶](07_07_r7rs_support.md#772-r7rs-standard-libraries)

The R7RS organizes the definitions from R5RS into modules, and also adds a few new definitions.

We do not attempt to document these libraries fully here, as unlike R6RS, there are few new definitions in R7RS relative to R5RS. Most of their functionality is already in Guile’s standard environment. Again, the expectation is that most Guile users will use the well-known and well-documented Guile modules; these R7RS libraries are mostly useful to users who want to port their code to other R7RS systems.

As a brief overview, we note that the libraries defined by the R7RS are as follows:

`(scheme base)`

The core functions, mostly corresponding to R5RS minus the elements listed separately below, but plus SRFI-34 error handling (see [SRFI-34 - Exception handling for programs](07_05_23_srfi34_exception_handling_for_programs.md#7523-srfi-34---exception-handling-for-programs)), bytevectors and bytevector ports (see [Bytevectors](06_06_12_bytevectors.md#6612-bytevectors)), and some miscellaneous other new procedures.

`(scheme case-lambda)`

`case-lambda`.

`(scheme char)`

Converting strings and characters to upper or lower case, predicates for if a characer is numeric, and so on.

`(scheme complex)`

Constructors and accessors for complex numbers.

`(scheme cxr)`

`cddr`, `cadadr`, and all that.

`(scheme eval)`

`eval`, but also an `environment` routine allowing a user to specify an environment using a module import set.

`(scheme file)`

`call-with-input-file` and so on.

`(scheme inexact)`

Routines that operate on inexact numbers: `sin`, `finite?`, and so on.

`(scheme lazy)`

Promises.

`(scheme load)`

The `load` procedure.

`(scheme process-context)`

Environment variables. See [SRFI-98 Accessing environment variables.](07_05_43_srfi98_accessing_environment_variables.md#7543-srfi-98-accessing-environment-variables). Also, `commmand-line`, `emergency-exit` (like Guile’s `primitive-_exit`), and `exit`.

`(scheme r5rs)`

The precise set of bindings exported by `r5rs`, but without `transcript-off` / `transcript-on`, and also with the auxiliary syntax definitions like `_` or `else`. See [Syntax-rules Macros](06_08_macros.md#682-syntax-rules-macros), for more on auxiliary syntax.

`(scheme read)`

The `read` procedure.

`(scheme repl)`

The `interaction-environment` procedure.

`(scheme time)`

`current-second`, as well as `current-jiffy` and `jiffies-per-second`. Guile uses the term “internal time unit” for what R7RS calls “jiffies”.

`(scheme write)`

`display`, `write`, as well as `write-shared` and `write-simple`.

For complete documentation, we advise the interested user to consult the R7RS directly (see [R7RS](https://doc.guix.gnu.org/guile/latest/en/r7rs.html#R7RS) in The Revised^7 Report on the Algorithmic Language Scheme).

* * *

Next: [Readline Support](07_09_readline_support.md#79-readline-support), Previous: [R7RS Support](07_07_r7rs_support.md#77-r7rs-support), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
