### 6.23 Configuration, Features and Runtime Options [¶](06_23_configuration_features_and_runtime_options.md#623-configuration-features-and-runtime-options)

Why is my Guile different from your Guile? There are three kinds of possible variation:

*   build differences — different versions of the Guile source code, installation directories, configuration flags that control pieces of functionality being included or left out, etc.
*   differences in dynamically loaded code — behavior and features provided by modules that can be dynamically loaded into a running Guile
*   different runtime options — some of the options that are provided for controlling Guile’s behavior may be set differently.

Guile provides “introspective” variables and procedures to query all of these possible variations at runtime. For runtime options, it also provides procedures to change the settings of options and to obtain documentation on what the options mean.

*   [Configuration, Build and Installation](06_23_configuration_features_and_runtime_options.md#6231-configuration-build-and-installation)
*   [Feature Tracking](06_23_configuration_features_and_runtime_options.md#6232-feature-tracking)
*   [Runtime Options](06_23_configuration_features_and_runtime_options.md#6233-runtime-options)

* * *

Next: [Feature Tracking](06_23_configuration_features_and_runtime_options.md#6232-feature-tracking), Up: [Configuration, Features and Runtime Options](06_23_configuration_features_and_runtime_options.md#623-configuration-features-and-runtime-options)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.23.1 Configuration, Build and Installation [¶](06_23_configuration_features_and_runtime_options.md#6231-configuration-build-and-installation)

The following procedures and variables provide information about how Guile was configured, built and installed on your system.

Scheme Procedure: **version** [¶](06_23_configuration_features_and_runtime_options.md)

Scheme Procedure: **effective-version** [¶](06_23_configuration_features_and_runtime_options.md)

Scheme Procedure: **major-version** [¶](06_23_configuration_features_and_runtime_options.md)

Scheme Procedure: **minor-version** [¶](06_23_configuration_features_and_runtime_options.md)

Scheme Procedure: **micro-version** [¶](06_23_configuration_features_and_runtime_options.md)

C Function: **scm\_version** () [¶](06_23_configuration_features_and_runtime_options.md)

C Function: **scm\_effective\_version** () [¶](06_23_configuration_features_and_runtime_options.md)

C Function: **scm\_major\_version** () [¶](06_23_configuration_features_and_runtime_options.md)

C Function: **scm\_minor\_version** () [¶](06_23_configuration_features_and_runtime_options.md)

C Function: **scm\_micro\_version** () [¶](06_23_configuration_features_and_runtime_options.md)

Return a string describing Guile’s full version number, effective version number, major, minor or micro version number, respectively. The `effective-version` function returns the version name that should remain unchanged during a stable series. Currently that means that it omits the micro version. The effective version should be used for items like the versioned share directory name i.e. /usr/share/guile/3.0/

([version](06_23_configuration_features_and_runtime_options.md)) ⇒ "3.0.0"
([effective-version](06_23_configuration_features_and_runtime_options.md)) ⇒ "3.0"
([major-version](06_23_configuration_features_and_runtime_options.md)) ⇒ "3"
([minor-version](06_23_configuration_features_and_runtime_options.md)) ⇒ "0"
([micro-version](06_23_configuration_features_and_runtime_options.md)) ⇒ "0"

Scheme Procedure: **%package-data-dir** [¶](06_23_configuration_features_and_runtime_options.md)

C Function: **scm\_sys\_package\_data\_dir** () [¶](06_23_configuration_features_and_runtime_options.md)

Return the name of the directory under which Guile Scheme files in general are stored. On Unix-like systems, this is usually /usr/local/share/guile or /usr/share/guile.

Scheme Procedure: **%library-dir** [¶](06_23_configuration_features_and_runtime_options.md)

C Function: **scm\_sys\_library\_dir** () [¶](06_23_configuration_features_and_runtime_options.md)

Return the name of the directory where the Guile Scheme files that belong to the core Guile installation (as opposed to files from a 3rd party package) are installed. On Unix-like systems this is usually /usr/local/share/guile/GUILE\_EFFECTIVE\_VERSION or /usr/share/guile/GUILE\_EFFECTIVE\_VERSION;

for example /usr/local/share/guile/3.0.

Scheme Procedure: **%site-dir** [¶](06_23_configuration_features_and_runtime_options.md)

C Function: **scm\_sys\_site\_dir** () [¶](06_23_configuration_features_and_runtime_options.md)

Return the name of the directory where Guile Scheme files specific to your site should be installed. On Unix-like systems, this is usually /usr/local/share/guile/site or /usr/share/guile/site.

Scheme Procedure: **%site-ccache-dir** [¶](06_23_configuration_features_and_runtime_options.md)

C Function: **scm\_sys\_site\_ccache\_dir** () [¶](06_23_configuration_features_and_runtime_options.md)

Return the directory where users should install compiled `.go` files for use with this version of Guile. Might look something like /usr/lib/guile/3.0/site-ccache.

Variable: **%guile-build-info** [¶](06_23_configuration_features_and_runtime_options.md)

Alist of information collected during the building of a particular Guile. Entries can be grouped into one of several categories: directories, env vars, and versioning info.

Briefly, here are the keys in `%guile-build-info`, by group:

directories

srcdir, top\_srcdir, prefix, exec\_prefix, bindir, sbindir, libexecdir, datadir, sysconfdir, sharedstatedir, localstatedir, libdir, infodir, mandir, includedir, pkgdatadir, pkglibdir, pkgincludedir

env vars

LIBS

versioning info

guileversion, libguileinterface, buildstamp

Values are all strings. The value for `LIBS` is typically found also as a part of `pkg-config --libs guile-3.0` output. The value for `guileversion` has form X.Y.Z, and should be the same as returned by `(version)`. The value for `libguileinterface` is libtool compatible and has form CURRENT:REVISION:AGE (see [Library interface versions](https://www.gnu.org/software/libtool/manual/libtool.html#Versioning) in GNU Libtool). The value for `buildstamp` is the output of the command ‘date -u +'%Y-%m-%d %T'’ (UTC).

In the source, `%guile-build-info` is initialized from libguile/libpath.h, which is completely generated, so deleting this file before a build guarantees up-to-date values for that build.

Variable: **%host-type** [¶](06_23_configuration_features_and_runtime_options.md)

The canonical host type (GNU triplet) of the host Guile was configured for, e.g., `"x86_64-unknown-linux-gnu"` (see [Canonicalizing](https://www.gnu.org/software/autoconf/manual/autoconf.html#Canonicalizing) in The GNU Autoconf Manual).

* * *

Next: [Runtime Options](06_23_configuration_features_and_runtime_options.md#6233-runtime-options), Previous: [Configuration, Build and Installation](06_23_configuration_features_and_runtime_options.md#6231-configuration-build-and-installation), Up: [Configuration, Features and Runtime Options](06_23_configuration_features_and_runtime_options.md#623-configuration-features-and-runtime-options)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.23.2 Feature Tracking [¶](06_23_configuration_features_and_runtime_options.md#6232-feature-tracking)

Guile has a Scheme level variable `*features*` that keeps track to some extent of the features that are available in a running Guile. `*features*` is a list of symbols, for example `threads`, each of which describes a feature of the running Guile process.

Variable: **\*features\*** [¶](06_23_configuration_features_and_runtime_options.md)

A list of symbols describing available features of the Guile process.

You shouldn’t modify the `*features*` variable directly using `set!`. Instead, see the procedures that are provided for this purpose in the following subsection.

*   [Feature Manipulation](06_23_configuration_features_and_runtime_options.md#62321-feature-manipulation)
*   [Common Feature Symbols](06_23_configuration_features_and_runtime_options.md#62322-common-feature-symbols)

* * *

Next: [Common Feature Symbols](06_23_configuration_features_and_runtime_options.md#62322-common-feature-symbols), Up: [Feature Tracking](06_23_configuration_features_and_runtime_options.md#6232-feature-tracking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.23.2.1 Feature Manipulation [¶](06_23_configuration_features_and_runtime_options.md#62321-feature-manipulation)

To check whether a particular feature is available, use the `provided?` procedure:

Scheme Procedure: **provided?** feature [¶](06_23_configuration_features_and_runtime_options.md)

Deprecated Scheme Procedure: **feature?** feature [¶](06_23_configuration_features_and_runtime_options.md)

Return `#t` if the specified feature is available, otherwise `#f`.

To advertise a feature from your own Scheme code, you can use the `provide` procedure:

Scheme Procedure: **provide** feature [¶](06_23_configuration_features_and_runtime_options.md)

Add feature to the list of available features in this Guile process.

For C code, the equivalent function takes its feature name as a `char *` argument for convenience:

C Function: `void` **scm\_add\_feature** `(const char *str)` [¶](06_23_configuration_features_and_runtime_options.md)

Add a symbol with name str to the list of available features in this Guile process.

* * *

Previous: [Feature Manipulation](06_23_configuration_features_and_runtime_options.md#62321-feature-manipulation), Up: [Feature Tracking](06_23_configuration_features_and_runtime_options.md#6232-feature-tracking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.23.2.2 Common Feature Symbols [¶](06_23_configuration_features_and_runtime_options.md#62322-common-feature-symbols)

In general, a particular feature may be available for one of two reasons. Either because the Guile library was configured and compiled with that feature enabled — i.e. the feature is built into the library on your system. Or because some C or Scheme code that was dynamically loaded by Guile has added that feature to the list.

In the first category, here are the features that the current version of Guile may define (depending on how it is built), and what they mean.

`array`

Indicates support for arrays (see [Arrays](06_06_13_arrays.md#6613-arrays)).

`array-for-each`

Indicates availability of `array-for-each` and other array mapping procedures (see [Arrays](06_06_13_arrays.md#6613-arrays)).

`char-ready?`

Indicates that the `char-ready?` function is available (see [Venerable Port Interfaces](06_12_input_and_output.md#61211-venerable-port-interfaces)).

`complex`

Indicates support for complex numbers.

`current-time`

Indicates availability of time-related functions: `times`, `get-internal-run-time` and so on (see [Time](07_02_05_time.md#725-time)).

`debug-extensions`

Indicates that the debugging evaluator is available, together with the options for controlling it.

`delay`

Indicates support for promises (see [Delayed Evaluation](06_16_reading_and_evaluating_scheme_code.md#61610-delayed-evaluation)).

`EIDs`

Indicates that the `geteuid` and `getegid` really return effective user and group IDs (see [Processes](07_02_07_processes.md#727-processes)).

`inexact`

Indicates support for inexact numbers.

`i/o-extensions`

Indicates availability of the following extended I/O procedures: `ftell`, `redirect-port`, `dup->fdes`, `dup2`, `fileno`, `isatty?`, `fdopen`, `primitive-move->fdes` and `fdes->ports` (see [Ports and File Descriptors](07_02_02_ports_and_file_descriptors.md#722-ports-and-file-descriptors)).

`net-db`

Indicates availability of network database functions: `scm_gethost`, `scm_getnet`, `scm_getproto`, `scm_getserv`, `scm_sethost`, `scm_setnet`, `scm_setproto`, `scm_setserv`, and their ‘byXXX’ variants (see [Network Databases](07_02_11_networking.md#72112-network-databases)).

`posix`

Indicates support for POSIX functions: `pipe`, `getgroups`, `kill`, `execl` and so on (see [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)).

`fork`

Indicates support for the POSIX `fork` function (see [`primitive-fork`](07_02_07_processes.md#727-processes)).

`popen`

Indicates support for `open-pipe` in the `(ice-9 popen)` module (see [Pipes](07_02_10_pipes.md#7210-pipes)).

`random`

Indicates availability of random number generation functions: `random`, `copy-random-state`, `random-uniform` and so on (see [Random Number Generation](06_06_02_numerical_data_types.md#66214-random-number-generation)).

`reckless`

Indicates that Guile was built with important checks omitted — you should never see this!

`regex`

Indicates support for POSIX regular expressions using `make-regexp`, `regexp-exec` and friends (see [Regexp Functions](06_13_regular_expressions.md#6131-regexp-functions)).

`socket`

Indicates availability of socket-related functions: `socket`, `bind`, `connect` and so on (see [Network Sockets and Communication](07_02_11_networking.md#72114-network-sockets-and-communication)).

`sort`

Indicates availability of sorting and merging functions (see [Sorting](06_09_general_utility_functions.md#693-sorting)).

`system`

Indicates that the `system` function is available (see [Processes](07_02_07_processes.md#727-processes)).

`threads`

Indicates support for multithreading (see [Threads](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads)).

`values`

Indicates support for multiple return values using `values` and `call-with-values` (see [Returning and Accepting Multiple Values](06_11_controlling_the_flow_of_program_execution.md#6117-returning-and-accepting-multiple-values)).

Available features in the second category depend, by definition, on what additional code your Guile process has loaded in. The following table lists features that you might encounter for this reason.

`defmacro`

Indicates that the `defmacro` macro is available (see [Macros](06_08_macros.md#68-macros)).

`describe`

Indicates that the `(oop goops describe)` module has been loaded, which provides a procedure for describing the contents of GOOPS instances.

`readline`

Indicates that Guile has loaded in Readline support, for command line editing (see [Readline Support](07_09_readline_support.md#79-readline-support)).

`record`

Indicates support for record definition using `make-record-type` and friends (see [Records](06_06_17_records.md#6617-records)).

Although these tables may seem exhaustive, it is probably unwise in practice to rely on them, as the correspondences between feature symbols and available procedures/behavior are not strictly defined. If you are writing code that needs to check for the existence of some procedure, it is probably safer to do so directly using the `defined?` procedure than to test for the corresponding feature using `provided?`.

* * *

Previous: [Feature Tracking](06_23_configuration_features_and_runtime_options.md#6232-feature-tracking), Up: [Configuration, Features and Runtime Options](06_23_configuration_features_and_runtime_options.md#623-configuration-features-and-runtime-options)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.23.3 Runtime Options [¶](06_23_configuration_features_and_runtime_options.md#6233-runtime-options)

There are a number of runtime options available for parameterizing built-in procedures, like `read`, and built-in behavior, like what happens on an uncaught error.

For more information on reader options, See [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code).

For more information on print options, See [Writing Scheme Values](06_16_reading_and_evaluating_scheme_code.md#6164-writing-scheme-values).

Finally, for more information on debugger options, See [Debug options](06_26_debugging_infrastructure.md#62645-debug-options).

*   [Examples of option use](06_23_configuration_features_and_runtime_options.md#62331-examples-of-option-use)

#### 6.23.3.1 Examples of option use [¶](06_23_configuration_features_and_runtime_options.md#62331-examples-of-option-use)

Here is an example of a session in which some read and debug option handling procedures are used. In this example, the user

1.  Notices that the symbols `abc` and `aBc` are not the same
2.  Examines the `read-options`, and sees that `case-insensitive` is set to “no”.
3.  Enables `case-insensitive`
4.  Quits the recursive prompt
5.  Verifies that now `aBc` and `abc` are the same

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) (define abc "hello")
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) abc
$1 [\=](06_06_02_numerical_data_types.md) "hello"
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) aBc
<unknown-location>: warning: possibly unbound variable \`aBc'
ERROR: In [procedure](06_07_procedures.md) module-lookup:
ERROR: Unbound variable: aBc
Entering a new prompt.  Type \`,bt' for a [backtrace](04_programming_in_scheme.md) or \`,q' to continue.
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ([read-options](06_16_reading_and_evaluating_scheme_code.md) 'help)
copy              no    Copy source code expressions.
positions         yes   Record positions of source code expressions.
case-insensitive  no    Convert symbols to lower case.
keywords          #f    Style of keyword recognition: #f, 'prefix or 'postfix.
r6rs-hex-escapes  no    Use R6RS variable-length character and [string](06_06_05_strings.md) hex escapes.
square-brackets   yes   Treat \`\[' and \`\]' as parentheses, for R6RS compatibility.
hungry-eol-escapes no   In strings, consume leading whitespace after an
                        escaped end-of-line.
curly-infix       no    Support SRFI-105 curly infix expressions.
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ([read-enable](06_16_reading_and_evaluating_scheme_code.md) 'case-insensitive)
$2 [\=](06_06_02_numerical_data_types.md) (square-brackets keywords #f case-insensitive positions)
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ,q
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) aBc
$3 [\=](06_06_02_numerical_data_types.md) "hello"

* * *

Next: [Support for Internationalization](06_25_support_for_internationalization.md#625-support-for-internationalization), Previous: [Configuration, Features and Runtime Options](06_23_configuration_features_and_runtime_options.md#623-configuration-features-and-runtime-options), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
