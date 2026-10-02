4 Programming in Scheme [¶](04_programming_in_scheme.md#4-programming-in-scheme)
--------------------------------------------------------------------------------------------------------

Guile’s core language is Scheme, and a lot can be achieved simply by using Guile to write and run Scheme programs — as opposed to having to dive into C code. In this part of the manual, we explain how to use Guile in this mode, and describe the tools that Guile provides to help you with script writing, debugging, and packaging your programs for distribution.

For detailed reference information on the variables, functions, and so on that make up Guile’s application programming interface (API), see [API Reference](06_00_api_reference.md#6-api-reference).

*   [Guile’s Implementation of Scheme](04_programming_in_scheme.md#41-guiles-implementation-of-scheme)
*   [Invoking Guile](04_programming_in_scheme.md#42-invoking-guile)
*   [Guile Scripting](04_programming_in_scheme.md#43-guile-scripting)
*   [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively)
*   [Using Guile in Emacs](04_programming_in_scheme.md#45-using-guile-in-emacs)
*   [Using Guile Tools](04_programming_in_scheme.md#46-using-guile-tools)
*   [Installing Site Packages](04_programming_in_scheme.md#47-installing-site-packages)
*   [Distributing Guile Code](04_programming_in_scheme.md#48-distributing-guile-code)

* * *

Next: [Invoking Guile](04_programming_in_scheme.md#42-invoking-guile), Up: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 4.1 Guile’s Implementation of Scheme [¶](04_programming_in_scheme.md#41-guiles-implementation-of-scheme)

Guile’s core language is Scheme, which is specified and described in the series of reports known as _RnRS_. _RnRS_ is shorthand for the _Revised^n Report on the Algorithmic Language Scheme_. Guile complies fully with R5RS (see [Introduction](https://doc.guix.gnu.org/r5rs/latest/en/r5rs.html#Top) in R5RS), and is largely compliant with R6RS and R7RS.

Guile also has many extensions that go beyond these reports. Some of the areas where Guile extends standard Scheme are:

*   Guile’s interactive documentation system
*   Guile’s support for POSIX-compliant network programming
*   GOOPS – Guile’s framework for object oriented programming.

* * *

Next: [Guile Scripting](04_programming_in_scheme.md#43-guile-scripting), Previous: [Guile’s Implementation of Scheme](04_programming_in_scheme.md#41-guiles-implementation-of-scheme), Up: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 4.2 Invoking Guile [¶](04_programming_in_scheme.md#42-invoking-guile)

Many features of Guile depend on and can be changed by information that the user provides either before or when Guile is started. Below is a description of what information to provide and how to provide it.

*   [Command-line Options](04_programming_in_scheme.md#421-command-line-options)
*   [Environment Variables](04_programming_in_scheme.md#422-environment-variables)

* * *

Next: [Environment Variables](04_programming_in_scheme.md#422-environment-variables), Up: [Invoking Guile](04_programming_in_scheme.md#42-invoking-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.2.1 Command-line Options [¶](04_programming_in_scheme.md#421-command-line-options)

Here we describe Guile’s command-line processing in detail. Guile processes its arguments from left to right, recognizing the switches described below. For examples, see [Scripting Examples](04_programming_in_scheme.md#434-scripting-examples).

`script arg...` [¶](04_programming_in_scheme.md)

`-s script arg...`

By default, Guile will read a file named on the command line as a script. Any command-line arguments arg... following script become the script’s arguments; the `command-line` function returns a list of strings of the form `(script arg...)`.

It is possible to name a file using a leading hyphen, for example, \-myfile.scm. In this case, the file name must be preceded by \-s to tell Guile that a (script) file is being named.

Scripts are read and evaluated as Scheme source code just as the `load` function would. After loading script, Guile exits.

`-c expr arg...` [¶](04_programming_in_scheme.md)

Evaluate expr as Scheme code, and then exit. Any command-line arguments arg... following expr become command-line arguments; the `command-line` function returns a list of strings of the form `(guile arg...)`, where guile is the path of the Guile executable.

`-- arg...`

Run interactively, prompting the user for expressions and evaluating them. Any command-line arguments arg... following the \-- become command-line arguments for the interactive session; the `command-line` function returns a list of strings of the form `(guile arg...)`, where guile is the path of the Guile executable.

`-L directory`

Add directory to the front of Guile’s module load path. The given directories are searched in the order given on the command line and before any directories in the `GUILE_LOAD_PATH` environment variable. Paths added here are _not_ in effect during execution of the user’s .guile file.

`-C directory`

Like \-L, but adjusts the load path for _compiled_ files.

`-x extension`

Add extension to the front of Guile’s load extension list (see [`%load-extensions`](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths)). The specified extensions are tried in the order given on the command line, and before the default load extensions. Extensions added here are _not_ in effect during execution of the user’s .guile file.

`-l file`

Load Scheme source code from file, and continue processing the command line.

`-e function`

Make function the _entry point_ of the script. After loading the script file (with \-s) or evaluating the expression (with \-c), apply function to a list containing the program name and the command-line arguments—the list provided by the `command-line` function.

A \-e switch can appear anywhere in the argument list, but Guile always invokes the function as the _last_ action it performs. This is weird, but because of the way script invocation works under POSIX, the \-s option must always come last in the list.

The function is most often a simple symbol that names a function that is defined in the script. It can also be of the form `(@ module-name symbol)`, and in that case, the symbol is looked up in the module named module-name.

As a shorthand you can use the form `(symbol ...)`, that is, a list of only symbols that doesn’t start with `@`. It is equivalent to `(@ module-name main)`, where module-name is `(symbol ...)` form. See [Using Guile Modules](06_18_modules.md#6182-using-guile-modules) and [Scripting Examples](04_programming_in_scheme.md#434-scripting-examples).

`-ds`

Treat a final \-s option as if it occurred at this point in the command line; load the script here.

This switch is necessary because, although the POSIX script invocation mechanism effectively requires the \-s option to appear last, the programmer may well want to run the script before other actions requested on the command line. For examples, see [Scripting Examples](04_programming_in_scheme.md#434-scripting-examples).

`\`

Read more command-line arguments, starting from the second line of the script file. See [The Meta Switch](04_programming_in_scheme.md#432-the-meta-switch).

`--use-srfi=list` [¶](04_programming_in_scheme.md)

The option \--use-srfi expects a comma-separated list of numbers, each representing a SRFI module to be loaded into the interpreter before evaluating a script file or starting the REPL. Additionally, the feature identifier for the loaded SRFIs is recognized by the procedure `cond-expand` when this option is used.

Here is an example that loads the modules SRFI-8 (’receive’) and SRFI-13 (’string library’) before the GUILE interpreter is started:

guile --use-srfi=8,13

`--r6rs` [¶](04_programming_in_scheme.md)

Adapt Guile’s initial environment to better support R6RS. See [Incompatibilities with the R6RS](07_06_r6rs_support.md#761-incompatibilities-with-the-r6rs), for some caveats.

`--r7rs` [¶](04_programming_in_scheme.md)

Adapt Guile’s initial environment to better support R7RS. See [Incompatibilities with the R7RS](07_07_r7rs_support.md#771-incompatibilities-with-the-r7rs), for some caveats.

`--debug` [¶](04_programming_in_scheme.md)

Start with the debugging virtual machine (VM) engine. Using the debugging VM will enable support for VM hooks, which are needed for tracing, breakpoints, and accurate call counts when profiling. The debugging VM is slower than the regular VM, though, by about ten percent. See [VM Hooks](06_26_debugging_infrastructure.md#62651-vm-hooks), for more information.

By default, the debugging VM engine is only used when entering an interactive session. When executing a script with \-s or \-c, the normal, faster VM is used by default.

`--no-debug` [¶](04_programming_in_scheme.md)

Do not use the debugging VM engine, even when entering an interactive session.

Note that, despite the name, Guile running with \--no-debug _does_ support the usual debugging facilities, such as printing a detailed backtrace upon error. The only difference with \--debug is lack of support for VM hooks and the facilities that build upon it (see above).

`-q` [¶](04_programming_in_scheme.md)

Do not load the initialization file, .guile. This option only has an effect when running interactively; running scripts does not load the .guile file. See [The Init File, ~/.guile](04_programming_in_scheme.md#441-the-init-file-guile).

`--listen[=p]`

While this program runs, listen on a local port or a path for REPL clients. If p starts with a number, it is assumed to be a local port on which to listen. If it starts with a forward slash, it is assumed to be the file name of a UNIX domain socket on which to listen.

If p is not given, the default is local port 37146. If you look at it upside down, it almost spells “Guile”. If you have netcat installed, you should be able to nc localhost 37146 and get a Guile prompt. Alternately you can fire up Emacs and connect to the process; see [Using Guile in Emacs](04_programming_in_scheme.md#45-using-guile-in-emacs) for more details.

> **Note:** Opening a port allows anyone who can connect to that port to do anything Guile can do, as the user that the Guile process is running as. Do not use \--listen on multi-user machines. Of course, if you do not pass \--listen to Guile, no port will be opened.
> 
> Guile protects against the [_HTTP inter-protocol exploitation attack_](https://en.wikipedia.org/wiki/Inter-protocol_exploitation), a scenario whereby an attacker can, _via_ an HTML page, cause a web browser to send data to TCP servers listening on a loopback interface or private network. Nevertheless, you are advised to use UNIX domain sockets, as in `--listen=/some/local/file`, whenever possible.

That said, \--listen is great for interactive debugging and development.

`--statprof=style`

Run any scripts or expressions inside the statprof profiler. Show the results in style, See [Statprof](07_20_statprof.md#720-statprof) for the possible values of STYLE. Inside the REPL, use [Profile Commands](04_programming_in_scheme.md#4445-profile-commands) instead.

`--auto-compile`

Compile source files automatically (default behavior).

`--fresh-auto-compile`

Treat the auto-compilation cache as invalid, forcing recompilation.

`--no-auto-compile`

Disable automatic source file compilation.

`--language=lang`

For the remainder of the command line arguments, assume that files mentioned with `-l` and expressions passed with `-c` are written in lang. lang must be the name of one of the languages supported by the compiler (see [Compiler Tower](09_04_compiling_to_the_virtual_machine.md#941-compiler-tower)). When run interactively, set the REPL’s language to lang (see [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively)).

The default language is `scheme`; other interesting values include `elisp` (for Emacs Lisp), and `ecmascript`.

The example below shows the evaluation of expressions in Scheme, Emacs Lisp, and ECMAScript:

guile -c "(apply + '(1 2))"
guile --language\=elisp -c "(= (funcall (symbol-function '+) 1 2) 3)"
guile --language\=ecmascript -c '(function (x) { return x \* x; })(2);'

To load a file written in Scheme and one written in Emacs Lisp, and then start a Scheme REPL, type:

guile -l foo.scm --language=elisp -l foo.el --language=scheme

`-h, --help`

Display help on invoking Guile, and then exit.

`-v, --version`

Display the current version of Guile, and then exit.

* * *

Previous: [Command-line Options](04_programming_in_scheme.md#421-command-line-options), Up: [Invoking Guile](04_programming_in_scheme.md#42-invoking-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.2.2 Environment Variables [¶](04_programming_in_scheme.md#422-environment-variables)

The _environment_ is a feature of the operating system; it consists of a collection of variables with names and values. Each variable is called an _environment variable_ (or, sometimes, a “shell variable”); environment variable names are case-sensitive, and it is conventional to use upper-case letters only. The values are all text strings, even those that are written as numerals. (Note that here we are referring to names and values that are defined in the operating system shell from which Guile is invoked. This is not the same as a Scheme environment that is defined within a running instance of Guile. For a description of Scheme environments, see [Names, Locations, Values and Environments](03_hello_scheme.md#341-names-locations-values-and-environments).)

How to set environment variables before starting Guile depends on the operating system and, especially, the shell that you are using. For example, here is how to tell Guile to provide detailed warning messages about deprecated features by setting `GUILE_WARN_DEPRECATED` using Bash:

$ export GUILE\_WARN\_DEPRECATED="detailed"
$ guile

Or, detailed warnings can be turned on for a single invocation using:

$ env GUILE\_WARN\_DEPRECATED="detailed" guile

If you wish to retrieve or change the value of the shell environment variables that affect the run-time behavior of Guile from within a running instance of Guile, see [Runtime Environment](07_02_06_runtime_environment.md#726-runtime-environment).

Here are the environment variables that affect the run-time behavior of Guile:

`GUILE_AUTO_COMPILE` [¶](04_programming_in_scheme.md)

This is a flag that can be used to tell Guile whether or not to compile Scheme source files automatically. Starting with Guile 2.0, Scheme source files will be compiled automatically, by default.

If a compiled (.go) file corresponding to a .scm file is not found or is not newer than the .scm file, the .scm file will be compiled on the fly, and the resulting .go file stored away. An advisory note will be printed on the console.

Compiled files will be stored in the directory $XDG\_CACHE\_HOME/guile/ccache, where `XDG_CACHE_HOME` defaults to the directory $HOME/.cache. This directory will be created if it does not already exist.

Note that this mechanism depends on the timestamp of the .go file being newer than that of the .scm file; if the .scm or .go files are moved after installation, care should be taken to preserve their original timestamps.

Set `GUILE_AUTO_COMPILE` to zero (0), to prevent Scheme files from being compiled automatically. Set this variable to “fresh” to tell Guile to compile Scheme files whether they are newer than the compiled files or not.

See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code).

`GUILE_HISTORY` [¶](04_programming_in_scheme.md)

This variable names the file that holds the Guile REPL command history. You can specify a different history file by setting this environment variable. By default, the history file is $HOME/.guile\_history.

`GUILE_INSTALL_LOCALE` [¶](04_programming_in_scheme.md)

This is a flag that can be used to tell Guile whether or not to install the current locale at startup, via a call to `(setlocale LC_ALL "")`[3](99_footnotes.md). See [Locales](07_02_13_locales.md#7213-locales), for more information on locales.

You may explicitly indicate that you do not want to install the locale by setting `GUILE_INSTALL_LOCALE` to `0`, or explicitly enable it by setting the variable to `1`.

Usually, installing the current locale is the right thing to do. It allows Guile to correctly parse and print strings with non-ASCII characters. Therefore, this option is on by default.

`GUILE_LOAD_COMPILED_PATH` [¶](04_programming_in_scheme.md)

This variable may be used to augment the path that is searched for compiled Scheme files (.go files) when loading. Its value should be a colon-separated list of directories. If it contains the special path component `...` (ellipsis), then the default path is put in place of the ellipsis, otherwise the default path is placed at the end. The result is stored in `%load-compiled-path` (see [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths)).

Here is an example using the Bash shell that adds the current directory, ., and the relative directory ../my-library to `%load-compiled-path`:

$ export GUILE\_LOAD\_COMPILED\_PATH=".:../my-library"
$ guile -c '(display %load-compiled-path) (newline)'
(. ../my-library /usr/local/lib/guile/3.0/ccache)

`GUILE_LOAD_PATH` [¶](04_programming_in_scheme.md)

This variable may be used to augment the path that is searched for Scheme files when loading. Its value should be a colon-separated list of directories. If it contains the special path component `...` (ellipsis), then the default path is put in place of the ellipsis, otherwise the default path is placed at the end. The result is stored in `%load-path` (see [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths)).

Here is an example using the Bash shell that prepends the current directory to `%load-path`, and adds the relative directory ../srfi to the end:

$ env GUILE\_LOAD\_PATH=".:...:../srfi" \\
guile -c '(display %load-path) (newline)'
(. /usr/local/share/guile/3.0 \\
/usr/local/share/guile/site/3.0 \\
/usr/local/share/guile/site \\
/usr/local/share/guile \\
../srfi)

(Note: The line breaks, above, are for documentation purposes only, and not required in the actual example.)

`GUILE_EXTENSIONS_PATH` [¶](04_programming_in_scheme.md)

This variable may be used to augment the path that is searched for foreign libraries via `load-extension`, `dynamic-link`, `load-foreign-library`, or the like. Its value should be a colon-separated (semicolon on Windows) list of directories. See [Foreign Libraries](06_19_foreign_function_interface.md#6191-foreign-libraries).

`GUILE_WARN_DEPRECATED` [¶](04_programming_in_scheme.md)

As Guile evolves, some features will be eliminated or replaced by newer features. To help users migrate their code as this evolution occurs, Guile will issue warning messages about code that uses features that have been marked for eventual elimination. `GUILE_WARN_DEPRECATED` can be set to “no” to tell Guile not to display these warning messages, or set to “detailed” to tell Guile to display more lengthy messages describing the warning. See [Deprecation](06_02_deprecation.md#62-deprecation).

`HOME` [¶](04_programming_in_scheme.md)

Guile uses the environment variable `HOME`, the name of your home directory, to locate various files, such as .guile or .guile\_history.

`GUILE_JIT_THRESHOLD` [¶](04_programming_in_scheme.md)

Guile has a just-in-time (JIT) code generator that makes running Guile code fast. See [Just-In-Time Native Code](09_03_a_virtual_machine_for_guile.md#938-just-in-time-native-code), for more. The unit of code generation is the function. Each function has its own counter that gets incremented when the function is called and at each loop iteration in the function. When the counter exceeds the `GUILE_JIT_THRESHOLD`, the function will get JIT-compiled. Set `GUILE_JIT_THRESHOLD` to `-1` to disable JIT compilation, or `0` to eagerly JIT-compile each function as it’s first seen.

`GUILE_JIT_LOG` [¶](04_programming_in_scheme.md)

Set to `1`, `2`, or `3` to give increasing amounts of logging for JIT compilation events. Used for debugging.

`GUILE_JIT_STOP_AFTER` [¶](04_programming_in_scheme.md)

Though we have tested the JIT compiler as well as we can, it’s possible that it has bugs. If you suspect that Guile’s JIT compiler is causing your program to fail, set `GUILE_JIT_STOP_AFTER` to a positive integer indicating the maximum number of functions to JIT-compile. By bisecting over the value of `GUILE_JIT_STOP_AFTER`, you can pinpoint the precise function that is being miscompiled.

`GUILE_JIT_PAUSE_WHEN_STOPPING` [¶](04_programming_in_scheme.md)

Debugging the JIT compiler sometimes requires analysing the running process. Setting `GUILE_JIT_PAUSE_WHEN_STOPPING` will pause the process when the JIT stops to let you connect a debugger to it and will print something along these lines:

stopping automatic JIT compilation, as requested
sleeping for 30s; to debug:
   gdb -p 133646

* * *

Next: [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively), Previous: [Invoking Guile](04_programming_in_scheme.md#42-invoking-guile), Up: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 4.3 Guile Scripting [¶](04_programming_in_scheme.md#43-guile-scripting)

Like AWK, Perl, or any shell, Guile can interpret script files. A Guile script is simply a file of Scheme code with some extra information at the beginning which tells the operating system how to invoke Guile, and then tells Guile how to handle the Scheme code.

*   [The Top of a Script File](04_programming_in_scheme.md#431-the-top-of-a-script-file)
*   [The Meta Switch](04_programming_in_scheme.md#432-the-meta-switch)
*   [Command Line Handling](04_programming_in_scheme.md#433-command-line-handling)
*   [Scripting Examples](04_programming_in_scheme.md#434-scripting-examples)

* * *

Next: [The Meta Switch](04_programming_in_scheme.md#432-the-meta-switch), Up: [Guile Scripting](04_programming_in_scheme.md#43-guile-scripting)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.3.1 The Top of a Script File [¶](04_programming_in_scheme.md#431-the-top-of-a-script-file)

The first line of a Guile script must tell the operating system to use Guile to evaluate the script, and then tell Guile how to go about doing that. Here is the simplest case:

*   The first two characters of the file must be ‘#!’.
    
    The operating system interprets this to mean that the rest of the line is the name of an executable that can interpret the script. Guile, however, interprets these characters as the beginning of a multi-line comment, terminated by the characters ‘!#’ on a line by themselves. (This is an extension to the syntax described in R5RS, added to support shell scripts.)
    
*   Immediately after those two characters must come the full pathname to the Guile interpreter. On most systems, this would be ‘/usr/local/bin/guile’.
*   Then must come a space, followed by a command-line argument to pass to Guile; this should be ‘\-s’. This switch tells Guile to run a script, instead of soliciting the user for input from the terminal. There are more elaborate things one can do here; see [The Meta Switch](04_programming_in_scheme.md#432-the-meta-switch).
*   Follow this with a newline.
*   The second line of the script should contain only the characters ‘!#’ — just like the top of the file, but reversed. The operating system never reads this far, but Guile treats this as the end of the comment begun on the first line by the ‘#!’ characters.
*   If this source code file is not ASCII or ISO-8859-1 encoded, a coding declaration such as `coding: utf-8` should appear in a comment somewhere in the first five lines of the file: see [Character Encoding of Source Files](06_16_reading_and_evaluating_scheme_code.md#6169-character-encoding-of-source-files).
*   The rest of the file should be a Scheme program.

Guile reads the program, evaluating expressions in the order that they appear. Upon reaching the end of the file, Guile exits.

* * *

Next: [Command Line Handling](04_programming_in_scheme.md#433-command-line-handling), Previous: [The Top of a Script File](04_programming_in_scheme.md#431-the-top-of-a-script-file), Up: [Guile Scripting](04_programming_in_scheme.md#43-guile-scripting)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.3.2 The Meta Switch [¶](04_programming_in_scheme.md#432-the-meta-switch)

Guile’s command-line switches allow the programmer to describe reasonably complicated actions in scripts. Unfortunately, the POSIX script invocation mechanism only allows one argument to appear on the ‘#!’ line after the path to the Guile executable, and imposes arbitrary limits on that argument’s length. Suppose you wrote a script starting like this:

#!/usr/local/bin/guile -e main -s
!#
(define (main args)
  (map (lambda (arg) (display arg) (display " "))
       (cdr args))
  (newline))

The intended meaning is clear: load the file, and then call `main` on the command-line arguments. However, the system will treat everything after the Guile path as a single argument — the string `"-e main -s"` — which is not what we want.

As a workaround, the meta switch `\` allows the Guile programmer to specify an arbitrary number of options without patching the kernel. If the first argument to Guile is `\`, Guile will open the script file whose name follows the `\`, parse arguments starting from the file’s second line (according to rules described below), and substitute them for the `\` switch.

Working in concert with the meta switch, Guile treats the characters ‘#!’ as the beginning of a comment which extends through the next line containing only the characters ‘!#’. This sort of comment may appear anywhere in a Guile program, but it is most useful at the top of a file, meshing magically with the POSIX script invocation mechanism.

Thus, consider a script named /u/jimb/ekko which starts like this:

#!/usr/local/bin/guile \\
-e main -s
!#
(define (main args)
        (map (lambda (arg) (display arg) (display " "))
             (cdr args))
        (newline))

Suppose a user invokes this script as follows:

$ /u/jimb/ekko a b c

Here’s what happens:

*   the operating system recognizes the ‘#!’ token at the top of the file, and rewrites the command line to:
    
    /usr/local/bin/guile \\ /u/jimb/ekko a b c
    
    This is the usual behavior, prescribed by POSIX.
    
*   When Guile sees the first two arguments, `\ /u/jimb/ekko`, it opens /u/jimb/ekko, parses the three arguments `-e`, `main`, and `-s` from it, and substitutes them for the `\` switch. Thus, Guile’s command line now reads:
    
    /usr/local/bin/guile -e main -s /u/jimb/ekko a b c
    
*   Guile then processes these switches: it loads /u/jimb/ekko as a file of Scheme code (treating the first three lines as a comment), and then performs the application `(main "/u/jimb/ekko" "a" "b" "c")`.

When Guile sees the meta switch `\`, it parses command-line argument from the script file according to the following rules:

*   Each space character terminates an argument. This means that two spaces in a row introduce an argument `""`.
*   The tab character is not permitted (unless you quote it with the backslash character, as described below), to avoid confusion.
*   The newline character terminates the sequence of arguments, and will also terminate a final non-empty argument. (However, a newline following a space will not introduce a final empty-string argument; it only terminates the argument list.)
*   The backslash character is the escape character. It escapes backslash, space, tab, and newline. The ANSI C escape sequences like `\n` and `\t` are also supported. These produce argument constituents; the two-character combination `\n` doesn’t act like a terminating newline. The escape sequence `\NNN` for exactly three octal digits reads as the character whose ASCII code is NNN. As above, characters produced this way are argument constituents. Backslash followed by other characters is not allowed.

* * *

Next: [Scripting Examples](04_programming_in_scheme.md#434-scripting-examples), Previous: [The Meta Switch](04_programming_in_scheme.md#432-the-meta-switch), Up: [Guile Scripting](04_programming_in_scheme.md#43-guile-scripting)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.3.3 Command Line Handling [¶](04_programming_in_scheme.md#433-command-line-handling)

The ability to accept and handle command line arguments is very important when writing Guile scripts to solve particular problems, such as extracting information from text files or interfacing with existing command line applications. This chapter describes how Guile makes command line arguments available to a Guile script, and the utilities that Guile provides to help with the processing of command line arguments.

When a Guile script is invoked, Guile makes the command line arguments accessible via the procedure `command-line`, which returns the arguments as a list of strings.

For example, if the script

#! /usr/local/bin/guile -s
!#
(write (command-line))
(newline)

is saved in a file cmdline-test.scm and invoked using the command line `./cmdline-test.scm bar.txt -o foo -frumple grob`, the output is

("./cmdline-test.scm" "bar.txt" "-o" "foo" "-frumple" "grob")

If the script invocation includes a `-e` option, specifying a procedure to call after loading the script, Guile will call that procedure with `(command-line)` as its argument. So a script that uses `-e` doesn’t need to refer explicitly to `command-line` in its code. For example, the script above would have identical behavior if it was written instead like this:

#! /usr/local/bin/guile \\
-e main -s
!#
(define (main args)
  (write args)
  (newline))

(Note the use of the meta switch `\` so that the script invocation can include more than one Guile option: See [The Meta Switch](04_programming_in_scheme.md#432-the-meta-switch).)

These scripts use the `#!` POSIX convention so that they can be executed using their own file names directly, as in the example command line `./cmdline-test.scm bar.txt -o foo -frumple grob`. But they can also be executed by typing out the implied Guile command line in full, as in:

$ guile -s ./cmdline-test.scm bar.txt -o foo -frumple grob

or

$ guile -e main -s ./cmdline-test2.scm bar.txt -o foo -frumple grob

Even when a script is invoked using this longer form, the arguments that the script receives are the same as if it had been invoked using the short form. Guile ensures that the `(command-line)` or `-e` arguments are independent of how the script is invoked, by stripping off the arguments that Guile itself processes.

A script is free to parse and handle its command line arguments in any way that it chooses. Where the set of possible options and arguments is complex, however, it can get tricky to extract all the options, check the validity of given arguments, and so on. This task can be greatly simplified by taking advantage of the module `(ice-9 getopt-long)`, which is distributed with Guile, See [The (ice-9 getopt-long) Module](07_04_the_ice9_getoptlong_module.md#74-the-ice-9-getopt-long-module).

* * *

Previous: [Command Line Handling](04_programming_in_scheme.md#433-command-line-handling), Up: [Guile Scripting](04_programming_in_scheme.md#43-guile-scripting)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.3.4 Scripting Examples [¶](04_programming_in_scheme.md#434-scripting-examples)

To start with, here are some examples of invoking Guile directly:

`guile -- a b c`

Run Guile interactively; `(command-line)` will return  
`("/usr/local/bin/guile" "a" "b" "c")`.

`guile -s /u/jimb/ex2 a b c`

Load the file /u/jimb/ex2; `(command-line)` will return  
`("/u/jimb/ex2" "a" "b" "c")`.

`guile -c '(write %load-path) (newline)'`

Write the value of the variable `%load-path`, print a newline, and exit.

`guile -e main -s /u/jimb/ex4 foo`

Load the file /u/jimb/ex4, and then call the function `main`, passing it the list `("/u/jimb/ex4" "foo")`.

`guile -e '(ex4)' -s /u/jimb/ex4.scm foo`

Load the file /u/jimb/ex4.scm, and then call the function `main` from the module ’(ex4)’, passing it the list `("/u/jimb/ex4" "foo")`.

`guile -l first -ds -l last -s script`

Load the files first, script, and last, in that order. The `-ds` switch says when to process the `-s` switch. For a more motivated example, see the scripts below.

Here is a very simple Guile script:

#!/usr/local/bin/guile -s
!#
(display "Hello, world!")
(newline)

The first line marks the file as a Guile script. When the user invokes it, the system runs /usr/local/bin/guile to interpret the script, passing `-s`, the script’s filename, and any arguments given to the script as command-line arguments. When Guile sees `-s script`, it loads script. Thus, running this program produces the output:

Hello, world!

Here is a script which prints the factorial of its argument:

#!/usr/local/bin/guile -s
!#
(define (fact n)
  (if (zero? n) 1
    (\* n (fact (- n 1)))))

(display (fact (string->number (cadr (command-line)))))
(newline)

In action:

$ ./fact 5
120
$

However, suppose we want to use the definition of `fact` in this file from another script. We can’t simply `load` the script file, and then use `fact`’s definition, because the script will try to compute and display a factorial when we load it. To avoid this problem, we might write the script this way:

#!/usr/local/bin/guile \\
-e main -s
!#
(define (fact n)
  (if (zero? n) 1
    (\* n (fact (- n 1)))))

(define (main args)
  (display (fact (string->number (cadr args))))
  (newline))

This version packages the actions the script should perform in a function, `main`. This allows us to load the file purely for its definitions, without any extraneous computation taking place. Then we used the meta switch `\` and the entry point switch `-e` to tell Guile to call `main` after loading the script.

$ ./fact 50
30414093201713378043612608166064768844377641568960512000000000000

Suppose that we now want to write a script which computes the `choose` function: given a set of m distinct objects, `(choose n m)` is the number of distinct subsets containing n objects each. It’s easy to write `choose` given `fact`, so we might write the script this way:

#!/usr/local/bin/guile \\
-l fact -e main -s
!#
(define (choose n m)
  (/ (fact m) (\* (fact (- m n)) (fact n))))

(define (main args)
  (let ((n (string->number (cadr args)))
        (m (string->number (caddr args))))
    (display (choose n m))
    (newline)))

The command-line arguments here tell Guile to first load the file fact, and then run the script, with `main` as the entry point. In other words, the `choose` script can use definitions made in the `fact` script. Here are some sample runs:

$ ./choose 0 4
1
$ ./choose 1 4
4
$ ./choose 2 4
6
$ ./choose 3 4
4
$ ./choose 4 4
1
$ ./choose 50 100
100891344545564193334812497256

To call a specific procedure from a given module, we can use the special form `(@ (module) procedure)`:

#!/usr/local/bin/guile \\
-l fact -e (@ (fac) main) -s
!#
(define-module (fac)
  #:export (main))

(define (choose n m)
  (/ (fact m) (\* (fact (- m n)) (fact n))))

(define (main args)
  (let ((n (string->number (cadr args)))
        (m (string->number (caddr args))))
    (display (choose n m))
    (newline)))

We can use `@@` to invoke non-exported procedures. For exported procedures, we can simplify this call with the shorthand `(module)`:

#!/usr/local/bin/guile \\
-l fact -e (fac) -s
!#
(define-module (fac)
  #:export (main))

(define (choose n m)
  (/ (fact m) (\* (fact (- m n)) (fact n))))

(define (main args)
  (let ((n (string->number (cadr args)))
        (m (string->number (caddr args))))
    (display (choose n m))
    (newline)))

For maximum portability, we can instead use the shell to execute `guile` with specified command line arguments. Here we need to take care to quote the command arguments correctly:

#!/usr/bin/env sh
exec guile -l fact -e '(@ (fac) main)' -s "$0" "$@"
!#
(define-module (fac)
  #:export (main))

(define (choose n m)
  (/ (fact m) (\* (fact (- m n)) (fact n))))

(define (main args)
  (let ((n (string->number (cadr args)))
        (m (string->number (caddr args))))
    (display (choose n m))
    (newline)))

Finally, seasoned scripters are probably missing a mention of subprocesses. In Bash, for example, most shell scripts run other programs like `sed` or the like to do the actual work.

In Guile it’s often possible get everything done within Guile itself, so do give that a try first. But if you just need to run a program and wait for it to finish, use `system*`. If you need to run a sub-program and capture its output, or give it input, use `open-pipe`. See [Processes](07_02_07_processes.md#727-processes), and See [Pipes](07_02_10_pipes.md#7210-pipes), for more information.

* * *

Next: [Using Guile in Emacs](04_programming_in_scheme.md#45-using-guile-in-emacs), Previous: [Guile Scripting](04_programming_in_scheme.md#43-guile-scripting), Up: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 4.4 Using Guile Interactively [¶](04_programming_in_scheme.md#44-using-guile-interactively)

When you start up Guile by typing just `guile`, without a `-c` argument or the name of a script to execute, you get an interactive interpreter where you can enter Scheme expressions, and Guile will evaluate them and print the results for you. Here are some simple examples.

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([+](06_06_02_numerical_data_types.md) 3 4 5)
$1 [\=](06_06_02_numerical_data_types.md) 12
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([display](06_16_reading_and_evaluating_scheme_code.md) "Hello world!\\n")
Hello world!
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([values](06_11_controlling_the_flow_of_program_execution.md) 'a 'b)
$2 [\=](06_06_02_numerical_data_types.md) a
$3 [\=](06_06_02_numerical_data_types.md) b

This mode of use is called a _REPL_, which is short for “Read-Eval-Print Loop”, because the Guile interpreter first reads the expression that you have typed, then evaluates it, and then prints the result.

The prompt shows you what language and module you are in. In this case, the current language is `scheme`, and the current module is `(guile-user)`. See [Support for Other Languages](06_24_support_for_other_languages.md#624-support-for-other-languages), for more information on Guile’s support for languages other than Scheme.

*   [The Init File, ~/.guile](04_programming_in_scheme.md#441-the-init-file-guile)
*   [Readline](04_programming_in_scheme.md#442-readline)
*   [Value History](04_programming_in_scheme.md#443-value-history)
*   [REPL Commands](04_programming_in_scheme.md#444-repl-commands)
*   [Error Handling](04_programming_in_scheme.md#445-error-handling)
*   [Interactive Debugging](04_programming_in_scheme.md#446-interactive-debugging)

* * *

Next: [Readline](04_programming_in_scheme.md#442-readline), Up: [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.1 The Init File, ~/.guile [¶](04_programming_in_scheme.md#441-the-init-file-guile)

When run interactively, Guile will load a local initialization file from ~/.guile. This file should contain Scheme expressions for evaluation.

This facility lets the user customize their interactive Guile environment, pulling in extra modules or parameterizing the REPL implementation.

To run Guile without loading the init file, use the `-q` command-line option.

* * *

Next: [Value History](04_programming_in_scheme.md#443-value-history), Previous: [The Init File, ~/.guile](04_programming_in_scheme.md#441-the-init-file-guile), Up: [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.2 Readline [¶](04_programming_in_scheme.md#442-readline)

To make it easier for you to repeat and vary previously entered expressions, or to edit the expression that you’re typing in, Guile can use the GNU Readline library. This is not enabled by default because of licensing reasons, but all you need to activate Readline is the following pair of lines.

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([use-modules](06_18_modules.md) (ice-9 [readline](07_09_readline_support.md)))
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([activate-readline](07_09_readline_support.md))

It’s a good idea to put these two lines (without the `scheme@(guile-user)>` prompts) in your .guile file. See [The Init File, ~/.guile](04_programming_in_scheme.md#441-the-init-file-guile), for more on .guile.

* * *

Next: [REPL Commands](04_programming_in_scheme.md#444-repl-commands), Previous: [Readline](04_programming_in_scheme.md#442-readline), Up: [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.3 Value History [¶](04_programming_in_scheme.md#443-value-history)

Just as Readline helps you to reuse a previous input line, _value history_ allows you to use the _result_ of a previous evaluation in a new expression. When value history is enabled, each evaluation result is automatically assigned to the next in the sequence of variables `$1`, `$2`, …. You can then use these variables in subsequent expressions.

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([iota](07_05_03_srfi1_list_library.md) 10)
$1 [\=](06_06_02_numerical_data_types.md) (0 1 2 3 4 5 6 7 8 9)
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([apply](06_16_reading_and_evaluating_scheme_code.md) [\*](06_06_02_numerical_data_types.md) ([cdr](06_06_08_pairs.md) $1))
$2 [\=](06_06_02_numerical_data_types.md) 362880
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([sqrt](06_06_02_numerical_data_types.md) $2)
$3 [\=](06_06_02_numerical_data_types.md) 602.3952191045344
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ([cons](06_06_08_pairs.md) $2 $1)
$4 [\=](06_06_02_numerical_data_types.md) (362880 0 1 2 3 4 5 6 7 8 9)

Value history is enabled by default, because Guile’s REPL imports the `(ice-9 history)` module. Value history may be turned off or on within the repl, using the options interface:

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ,option value-history #f
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) 'foo
foo
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ,option value-history #t
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) 'bar
$5 [\=](06_06_02_numerical_data_types.md) bar

Note that previously recorded values are still accessible, even if value history is off. In rare cases, these references to past computations can cause Guile to use too much memory. One may clear these values, possibly enabling garbage collection, via the `clear-value-history!` procedure, described below.

The programmatic interface to value history is in a module:

([use-modules](06_18_modules.md) (ice-9 history))

Scheme Procedure: **value-history-enabled?** [¶](04_programming_in_scheme.md)

Return true if value history is enabled, or false otherwise.

Scheme Procedure: **enable-value-history!** [¶](04_programming_in_scheme.md)

Turn on value history, if it was off.

Scheme Procedure: **disable-value-history!** [¶](04_programming_in_scheme.md)

Turn off value history, if it was on.

Scheme Procedure: **clear-value-history!** [¶](04_programming_in_scheme.md)

Clear the value history. If the stored values are not captured by some other data structure or closure, they may then be reclaimed by the garbage collector.

* * *

Next: [Error Handling](04_programming_in_scheme.md#445-error-handling), Previous: [Value History](04_programming_in_scheme.md#443-value-history), Up: [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.4 REPL Commands [¶](04_programming_in_scheme.md#444-repl-commands)

The REPL exists to read expressions, evaluate them, and then print their results. But sometimes one wants to tell the REPL to evaluate an expression in a different way, or to do something else altogether. A user can affect the way the REPL works with a _REPL command_.

The previous section had an example of a command, in the form of `,option`.

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) ,option value-history #t

Commands are distinguished from expressions by their initial comma (‘,’). Since a comma cannot begin an expression in most languages, it is an effective indicator to the REPL that the following text forms a command, not an expression.

REPL commands are convenient because they are always there. Even if the current module doesn’t have a binding for `pretty-print`, one can always `,pretty-print`.

The following sections document the various commands, grouped together by functionality. Many of the commands have abbreviations; see the online help (`,help`) for more information.

*   [Help Commands](04_programming_in_scheme.md#4441-help-commands)
*   [Module Commands](04_programming_in_scheme.md#4442-module-commands)
*   [Language Commands](04_programming_in_scheme.md#4443-language-commands)
*   [Compile Commands](04_programming_in_scheme.md#4444-compile-commands)
*   [Profile Commands](04_programming_in_scheme.md#4445-profile-commands)
*   [Debug Commands](04_programming_in_scheme.md#4446-debug-commands)
*   [Inspect Commands](04_programming_in_scheme.md#4447-inspect-commands)
*   [System Commands](04_programming_in_scheme.md#4448-system-commands)

* * *

Next: [Module Commands](04_programming_in_scheme.md#4442-module-commands), Up: [REPL Commands](04_programming_in_scheme.md#444-repl-commands)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.4.1 Help Commands [¶](04_programming_in_scheme.md#4441-help-commands)

When Guile starts interactively, it notifies the user that help can be had by typing ‘,help’. Indeed, `help` is a command, and a particularly useful one, as it allows the user to discover the rest of the commands.

REPL Command: **help** \[`all` | group | `[-c]` command\] [¶](04_programming_in_scheme.md)

Show help.

With one argument, tries to look up the argument as a group name, giving help on that group if successful. Otherwise tries to look up the argument as a command, giving help on the command.

If there is a command whose name is also a group name, use the ‘\-c command’ form to give help on the command instead of the group.

Without any argument, a list of help commands and command groups are displayed.

REPL Command: **show** \[topic\] [¶](04_programming_in_scheme.md)

Gives information about Guile.

With one argument, tries to show a particular piece of information; currently supported topics are ‘warranty’ (or ‘w’), ‘copying’ (or ‘c’), and ‘version’ (or ‘v’).

Without any argument, a list of topics is displayed.

REPL Command: **apropos** regexp [¶](04_programming_in_scheme.md)

Find bindings/modules/packages.

REPL Command: **describe** obj [¶](04_programming_in_scheme.md)

Show description/documentation.

* * *

Next: [Language Commands](04_programming_in_scheme.md#4443-language-commands), Previous: [Help Commands](04_programming_in_scheme.md#4441-help-commands), Up: [REPL Commands](04_programming_in_scheme.md#444-repl-commands)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.4.2 Module Commands [¶](04_programming_in_scheme.md#4442-module-commands)

REPL Command: **module** \[module\] [¶](04_programming_in_scheme.md)

Change modules / Show current module.

REPL Command: **import** module … [¶](04_programming_in_scheme.md)

Import modules / List those imported.

REPL Command: **load** file [¶](04_programming_in_scheme.md)

Load a file in the current module.

REPL Command: **reload** \[module\] [¶](04_programming_in_scheme.md)

Reload the given module, or the current module if none was given.

REPL Command: **binding** [¶](04_programming_in_scheme.md)

List current bindings.

REPL Command: **in** module expression [¶](04_programming_in_scheme.md)

REPL Command: **in** module command arg … [¶](04_programming_in_scheme.md)

Evaluate an expression, or alternatively, execute another meta-command in the context of a module. For example, ‘,in (foo bar) ,binding’ will show the bindings in the module `(foo bar)`.

* * *

Next: [Compile Commands](04_programming_in_scheme.md#4444-compile-commands), Previous: [Module Commands](04_programming_in_scheme.md#4442-module-commands), Up: [REPL Commands](04_programming_in_scheme.md#444-repl-commands)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.4.3 Language Commands [¶](04_programming_in_scheme.md#4443-language-commands)

REPL Command: **language** language [¶](04_programming_in_scheme.md)

Change languages.

* * *

Next: [Profile Commands](04_programming_in_scheme.md#4445-profile-commands), Previous: [Language Commands](04_programming_in_scheme.md#4443-language-commands), Up: [REPL Commands](04_programming_in_scheme.md#444-repl-commands)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.4.4 Compile Commands [¶](04_programming_in_scheme.md#4444-compile-commands)

REPL Command: **compile** exp [¶](04_programming_in_scheme.md)

Generate compiled code.

REPL Command: **compile-file** file [¶](04_programming_in_scheme.md)

Compile a file.

REPL Command: **expand** exp [¶](04_programming_in_scheme.md)

Expand any macros in a form.

REPL Command: **optimize** exp [¶](04_programming_in_scheme.md)

Run the optimizer on a piece of code and print the result.

REPL Command: **disassemble** exp [¶](04_programming_in_scheme.md)

Disassemble a compiled procedure.

REPL Command: **disassemble-file** file [¶](04_programming_in_scheme.md)

Disassemble a file.

* * *

Next: [Debug Commands](04_programming_in_scheme.md#4446-debug-commands), Previous: [Compile Commands](04_programming_in_scheme.md#4444-compile-commands), Up: [REPL Commands](04_programming_in_scheme.md#444-repl-commands)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.4.5 Profile Commands [¶](04_programming_in_scheme.md#4445-profile-commands)

REPL Command: **time** exp [¶](04_programming_in_scheme.md)

Time execution.

REPL Command: **profile** exp \[#:hz hz=100\] \[#:count-calls? count-calls?=#f\] \[#:display-style display-style=list\] [¶](04_programming_in_scheme.md)

Profile execution of an expression. This command compiled exp and then runs it within the statprof profiler, passing all keyword options to the `statprof` procedure. For more on statprof and on the the options available to this command, See [Statprof](07_20_statprof.md#720-statprof).

REPL Command: **trace** exp \[#:width w\] \[#:max-indent i\] [¶](04_programming_in_scheme.md)

Trace execution.

By default, the trace will limit its width to the width of your terminal, or width if specified. Nested procedure invocations will be printed farther to the right, though if the width of the indentation passes the max-indent, the indentation is abbreviated.

These REPL commands can also be called as regular functions in scheme code on including the `(ice-9 time)` module.

* * *

Next: [Inspect Commands](04_programming_in_scheme.md#4447-inspect-commands), Previous: [Profile Commands](04_programming_in_scheme.md#4445-profile-commands), Up: [REPL Commands](04_programming_in_scheme.md#444-repl-commands)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.4.6 Debug Commands [¶](04_programming_in_scheme.md#4446-debug-commands)

These debugging commands are only available within a recursive REPL; they do not work at the top level.

REPL Command: **backtrace** \[count\] \[#:width w\] \[#:full? f\] [¶](04_programming_in_scheme.md)

Print a backtrace.

Print a backtrace of all stack frames, or innermost count frames. If count is negative, the last count frames will be shown.

REPL Command: **up** \[count\] [¶](04_programming_in_scheme.md)

Select a calling stack frame.

Select and print stack frames that called this one. An argument says how many frames up to go.

REPL Command: **down** \[count\] [¶](04_programming_in_scheme.md)

Select a called stack frame.

Select and print stack frames called by this one. An argument says how many frames down to go.

REPL Command: **frame** \[idx\] [¶](04_programming_in_scheme.md)

Show a frame.

Show the selected frame. With an argument, select a frame by index, then show it.

REPL Command: **locals** [¶](04_programming_in_scheme.md)

Show local variables.

Show locally-bound variables in the selected frame.

REPL Command: **error-message** [¶](04_programming_in_scheme.md)

REPL Command: **error** [¶](04_programming_in_scheme.md)

Show error message.

Display the message associated with the error that started the current debugging REPL.

REPL Command: **registers** [¶](04_programming_in_scheme.md)

Show the VM registers associated with the current frame.

See [Stack Layout](09_03_a_virtual_machine_for_guile.md#933-stack-layout), for more information on VM stack frames.

REPL Command: **width** \[cols\] [¶](04_programming_in_scheme.md)

Sets the number of display columns in the output of `,backtrace` and `,locals` to cols. If cols is not given, the width of the terminal is used.

The next 3 commands work at any REPL.

REPL Command: **break** proc [¶](04_programming_in_scheme.md)

Set a breakpoint at proc.

REPL Command: **break-at-source** file line [¶](04_programming_in_scheme.md)

Set a breakpoint at the given source location.

REPL Command: **tracepoint** proc [¶](04_programming_in_scheme.md)

Set a tracepoint on the given procedure. This will cause all calls to the procedure to print out a tracing message. See [Tracing Traps](06_26_debugging_infrastructure.md#62654-tracing-traps), for more information.

The rest of the commands in this subsection all apply only when the stack is _continuable_ — in other words when it makes sense for the program that the stack comes from to continue running. Usually this means that the program stopped because of a trap or a breakpoint.

REPL Command: **step** [¶](04_programming_in_scheme.md)

Tell the debugged program to step to the next source location.

REPL Command: **next** [¶](04_programming_in_scheme.md)

Tell the debugged program to step to the next source location in the same frame. (See [Traps](06_26_debugging_infrastructure.md#6265-traps) for the details of how this works.)

REPL Command: **finish** [¶](04_programming_in_scheme.md)

Tell the program being debugged to continue running until the completion of the current stack frame, and at that time to print the result and reenter the REPL.

* * *

Next: [System Commands](04_programming_in_scheme.md#4448-system-commands), Previous: [Debug Commands](04_programming_in_scheme.md#4446-debug-commands), Up: [REPL Commands](04_programming_in_scheme.md#444-repl-commands)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.4.7 Inspect Commands [¶](04_programming_in_scheme.md#4447-inspect-commands)

REPL Command: **inspect** exp [¶](04_programming_in_scheme.md)

Inspect the result(s) of evaluating exp.

REPL Command: **pretty-print** exp [¶](04_programming_in_scheme.md)

Pretty-print the result(s) of evaluating exp.

* * *

Previous: [Inspect Commands](04_programming_in_scheme.md#4447-inspect-commands), Up: [REPL Commands](04_programming_in_scheme.md#444-repl-commands)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.4.8 System Commands [¶](04_programming_in_scheme.md#4448-system-commands)

REPL Command: **gc** [¶](04_programming_in_scheme.md)

Garbage collection.

REPL Command: **statistics** [¶](04_programming_in_scheme.md)

Display statistics.

REPL Command: **option** \[name\] \[exp\] [¶](04_programming_in_scheme.md)

With no arguments, lists all options. With one argument, shows the current value of the name option. With two arguments, sets the name option to the result of evaluating the Scheme expression exp.

REPL Command: **quit** [¶](04_programming_in_scheme.md)

Quit this session.

Current REPL options include:

`compile-options`

The options used when compiling expressions entered at the REPL. See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), for more on compilation options.

`interp`

Whether to interpret or compile expressions given at the REPL, if such a choice is available. Off by default (indicating compilation).

`prompt`

A customized REPL prompt. `#f` by default, indicating the default prompt.

`print`

A procedure of two arguments used to print the result of evaluating each expression. The arguments are the current REPL and the value to print. By default, `#f`, to use the default procedure.

`value-history`

Whether value history is on or not. See [Value History](04_programming_in_scheme.md#443-value-history).

`on-error`

What to do when an error happens. By default, `debug`, meaning to enter the debugger. Other values include `backtrace`, to show a backtrace without entering the debugger, or `report`, to simply show a short error printout.

Default values for REPL options may be set using `repl-default-option-set!` from `(system repl common)`:

Scheme Procedure: **repl-default-option-set!** key value [¶](04_programming_in_scheme.md)

Set the default value of a REPL option. This function is particularly useful in a user’s init file. See [The Init File, ~/.guile](04_programming_in_scheme.md#441-the-init-file-guile).

* * *

Next: [Interactive Debugging](04_programming_in_scheme.md#446-interactive-debugging), Previous: [REPL Commands](04_programming_in_scheme.md#444-repl-commands), Up: [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.5 Error Handling [¶](04_programming_in_scheme.md#445-error-handling)

When code being evaluated from the REPL hits an error, Guile enters a new prompt, allowing you to inspect the context of the error.

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) (map [string-append](06_06_05_strings.md) '("a" "b") '("c" #\\d))
ERROR: In [procedure](06_07_procedures.md) string-append:
ERROR: Wrong type (expecting [string](06_06_05_strings.md)): #\\d
Entering a new prompt.  Type \`,bt' for a [backtrace](04_programming_in_scheme.md) or \`,q' to continue.
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md)

The new prompt runs inside the old one, in the dynamic context of the error. It is a recursive REPL, augmented with a reified representation of the stack, ready for debugging.

`,backtrace` (abbreviated `,bt`) displays the Scheme call stack at the point where the error occurred:

scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ,bt
           1 (map #<procedure [string-append](06_06_05_strings.md) \_> ("a" "b") ("c" #\\d))
           0 ([string-append](06_06_05_strings.md) "b" #\\d)

In the above example, the backtrace doesn’t have much source information, as `map` and `string-append` are both primitives. But in the general case, the space on the left of the backtrace indicates the line and column in which a given procedure calls another.

You can exit a recursive REPL in the same way that you exit any REPL: via ‘(quit)’, ‘,quit’ (abbreviated ‘,q’), or C-d, among other options.

* * *

Previous: [Error Handling](04_programming_in_scheme.md#445-error-handling), Up: [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 4.4.6 Interactive Debugging [¶](04_programming_in_scheme.md#446-interactive-debugging)

A recursive debugging REPL exposes a number of other meta-commands that inspect the state of the computation at the time of the error. These commands allow you to

*   display the Scheme call stack at the point where the error occurred;
*   move up and down the call stack, to see in detail the expression being evaluated, or the procedure being applied, in each _frame_; and
*   examine the values of variables and expressions in the context of each frame.

See [Debug Commands](04_programming_in_scheme.md#4446-debug-commands), for documentation of the individual commands. This section aims to give more of a walkthrough of a typical debugging session.

First, we’re going to need a good error. Let’s try to macroexpand the expression `(unquote foo)`, outside of a `quasiquote` form, and see how the macroexpander reports this error.

scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) (macroexpand '([unquote](07_06_r6rs_support.md) foo))
ERROR: In [procedure](06_07_procedures.md) macroexpand:
ERROR: unquote: expression [not](06_06_01_booleans.md) valid outside of [quasiquote](07_06_r6rs_support.md) [in](04_programming_in_scheme.md) ([unquote](07_06_r6rs_support.md) foo)
Entering a new prompt.  Type \`,bt' for a [backtrace](04_programming_in_scheme.md) or \`,q' to continue.
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md)

The `backtrace` command, which can also be invoked as `bt`, displays the call stack (aka backtrace) at the point where the debugger was entered:

scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ,bt
In ice-9/psyntax.scm:
  1130:21  3 (chi-top ([unquote](07_06_r6rs_support.md) foo) () ((top)) e ([eval](06_16_reading_and_evaluating_scheme_code.md)) (hygiene #))
  1071:30  2 (syntax-type ([unquote](07_06_r6rs_support.md) foo) () ((top)) #f #f (# #) #f)
  1368:28  1 (chi-macro #<procedure de9360 at ice-9/psyntax.scm...> [...](06_08_macros.md))
In unknown file:
           0 ([scm-error](06_11_controlling_the_flow_of_program_execution.md) syntax-error macroexpand "~a: ~a in ~a" # #f)

A call stack consists of a sequence of stack _frames_, with each frame describing one procedure which is waiting to do something with the values returned by another. Here we see that there are four frames on the stack.

Note that `macroexpand` is not on the stack – it must have made a tail call to `chi-top`, as indeed we would find if we searched `ice-9/psyntax.scm` for its definition.

When you enter the debugger, the innermost frame is selected, which means that the commands for getting information about the “current” frame, or for evaluating expressions in the context of the current frame, will do so by default with respect to the innermost frame. To select a different frame, so that these operations will apply to it instead, use the `up`, `down` and `frame` commands like this:

scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ,up
In ice-9/psyntax.scm:
  1368:28  1 (chi-macro #<procedure de9360 at ice-9/psyntax.scm...> [...](06_08_macros.md))
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ,frame 3
In ice-9/psyntax.scm:
  1130:21  3 (chi-top ([unquote](07_06_r6rs_support.md) foo) () ((top)) e ([eval](06_16_reading_and_evaluating_scheme_code.md)) (hygiene #))
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ,down
In ice-9/psyntax.scm:
  1071:30  2 (syntax-type ([unquote](07_06_r6rs_support.md) foo) () ((top)) #f #f (# #) #f)

Perhaps we’re interested in what’s going on in frame 2, so we take a look at its local variables:

scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ,locals
  Local variables:
  $1 [\=](06_06_02_numerical_data_types.md) e [\=](06_06_02_numerical_data_types.md) ([unquote](07_06_r6rs_support.md) foo)
  $2 [\=](06_06_02_numerical_data_types.md) r [\=](06_06_02_numerical_data_types.md) ()
  $3 [\=](06_06_02_numerical_data_types.md) w [\=](06_06_02_numerical_data_types.md) ((top))
  $4 [\=](06_06_02_numerical_data_types.md) s [\=](06_06_02_numerical_data_types.md) #f
  $5 [\=](06_06_02_numerical_data_types.md) rib [\=](06_06_02_numerical_data_types.md) #f
  $6 [\=](06_06_02_numerical_data_types.md) [mod](07_06_r6rs_support.md) [\=](06_06_02_numerical_data_types.md) (hygiene guile-user)
  $7 [\=](06_06_02_numerical_data_types.md) for-car? [\=](06_06_02_numerical_data_types.md) #f
  $8 [\=](06_06_02_numerical_data_types.md) [first](07_05_03_srfi1_list_library.md) [\=](06_06_02_numerical_data_types.md) [unquote](07_06_r6rs_support.md)
  $9 [\=](06_06_02_numerical_data_types.md) ftype [\=](06_06_02_numerical_data_types.md) macro
  $10 [\=](06_06_02_numerical_data_types.md) fval [\=](06_06_02_numerical_data_types.md) #<procedure de9360 at ice-9/psyntax.scm:2817:2 (x)[\>](06_06_02_numerical_data_types.md)
  $11 [\=](06_06_02_numerical_data_types.md) fe [\=](06_06_02_numerical_data_types.md) [unquote](07_06_r6rs_support.md)
  $12 [\=](06_06_02_numerical_data_types.md) fw [\=](06_06_02_numerical_data_types.md) ((top))
  $13 [\=](06_06_02_numerical_data_types.md) fs [\=](06_06_02_numerical_data_types.md) #f
  $14 [\=](06_06_02_numerical_data_types.md) fmod [\=](06_06_02_numerical_data_types.md) (hygiene guile-user)

All of the values are accessible by their value-history names (`$n`):

scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) $10
$15 [\=](06_06_02_numerical_data_types.md) #<procedure de9360 at ice-9/psyntax.scm:2817:2 (x)[\>](06_06_02_numerical_data_types.md)

We can even invoke the procedure at the REPL directly:

scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ($10 'not-going-to-work)
ERROR: In [procedure](06_07_procedures.md) macroexpand:
ERROR: source expression failed to [match](07_08_pattern_matching.md) [any](07_05_03_srfi1_list_library.md) pattern [in](04_programming_in_scheme.md) not-going-to-work
Entering a new prompt.  Type \`,bt' for a [backtrace](04_programming_in_scheme.md) or \`,q' to continue.

Well at this point we’ve caused an error within an error. Let’s just quit back to the top level:

scheme@(guile-user) \[2\][\>](06_06_02_numerical_data_types.md) ,q
scheme@(guile-user) \[1\][\>](06_06_02_numerical_data_types.md) ,q
scheme@(guile-user)[\>](06_06_02_numerical_data_types.md) 

Finally, as a word to the wise: hackers close their REPL prompts with C-d.

* * *

Next: [Using Guile Tools](04_programming_in_scheme.md#46-using-guile-tools), Previous: [Using Guile Interactively](04_programming_in_scheme.md#44-using-guile-interactively), Up: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 4.5 Using Guile in Emacs [¶](04_programming_in_scheme.md#45-using-guile-in-emacs)

Any text editor can edit Scheme, but some are better than others. Emacs is the best, of course, and not just because it is a fine text editor. Emacs has good support for Scheme out of the box, with sensible indentation rules, parenthesis-matching, syntax highlighting, and even a set of keybindings for structural editing, allowing navigation, cut-and-paste, and transposition operations that work on balanced S-expressions.

As good as it is, though, two things will vastly improve your experience with Emacs and Guile.

The first is Taylor Campbell’s [Paredit](http://www.emacswiki.org/emacs/ParEdit). You should not code in any dialect of Lisp without Paredit. (They say that unopinionated writing is boring—hence this tone—but it’s the truth, regardless.) Paredit is the bee’s knees.

The second is José Antonio Ortega Ruiz’s [Geiser](http://www.nongnu.org/geiser/). Geiser complements Emacs’ `scheme-mode` with tight integration to running Guile processes via a `comint-mode` REPL buffer.

Of course there are keybindings to switch to the REPL, and a good REPL environment, but Geiser goes beyond that, providing:

*   Form evaluation in the context of the current file’s module.
*   Macro expansion.
*   File/module loading and/or compilation.
*   Namespace-aware identifier completion (including local bindings, names visible in the current module, and module names).
*   Autodoc: the echo area shows information about the signature of the procedure/macro around point automatically.
*   Jump to definition of identifier at point.
*   Access to documentation (including docstrings when the implementation provides it).
*   Listings of identifiers exported by a given module.
*   Listings of callers/callees of procedures.
*   Rudimentary support for debugging and error navigation.
*   Support for multiple, simultaneous REPLs.

See Geiser’s web page at [http://www.nongnu.org/geiser/](http://www.nongnu.org/geiser/), for more information.

* * *

Next: [Installing Site Packages](04_programming_in_scheme.md#47-installing-site-packages), Previous: [Using Guile in Emacs](04_programming_in_scheme.md#45-using-guile-in-emacs), Up: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 4.6 Using Guile Tools [¶](04_programming_in_scheme.md#46-using-guile-tools)

Guile also comes with a growing number of command-line utilities: a compiler, a disassembler, some module inspectors, and in the future, a system to install Guile packages from the internet. These tools may be invoked using the `guild` program.

$ guild compile -o foo.go foo.scm
wrote \`foo.go'

This program used to be called `guile-tools` up to Guile version 2.0.1, and for backward compatibility it still may be called as such. However we changed the name to `guild`, not only because it is pleasantly shorter and easier to read, but also because this tool will serve to bind Guile wizards together, by allowing hackers to share code with each other using a CPAN-like system.

See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), for more on `guild compile`.

A complete list of guild scripts can be had by invoking `guild list`, or simply `guild`.

* * *

Next: [Distributing Guile Code](04_programming_in_scheme.md#48-distributing-guile-code), Previous: [Using Guile Tools](04_programming_in_scheme.md#46-using-guile-tools), Up: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 4.7 Installing Site Packages [¶](04_programming_in_scheme.md#47-installing-site-packages)

At some point, you will probably want to share your code with other people. To do so effectively, it is important to follow a set of common conventions, to make it easy for the user to install and use your package.

The first thing to do is to install your Scheme files where Guile can find them. When Guile goes to find a Scheme file, it will search a _load path_ to find the file: first in Guile’s own path, then in paths for _site packages_. A site package is any Scheme code that is installed and not part of Guile itself. See [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths), for more on load paths.

There are several site paths, for historical reasons, but the one that should generally be used can be obtained by invoking the `%site-dir` procedure. See [Configuration, Build and Installation](06_23_configuration_features_and_runtime_options.md#6231-configuration-build-and-installation). If Guile 3.0 is installed on your system in `/usr/`, then `(%site-dir)` will be `/usr/share/guile/site/3.0`. Scheme files should be installed there.

If you do not install compiled `.go` files, Guile will compile your modules and programs when they are first used, and cache them in the user’s home directory. See [Compiling Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6166-compiling-scheme-code), for more on auto-compilation. However, it is better to compile the files before they are installed, and to just copy the files to a place that Guile can find them.

As with Scheme files, Guile searches a path to find compiled `.go` files, the `%load-compiled-path`. By default, this path has two entries: a path for Guile’s files, and a path for site packages. You should install your `.go` files into the latter directory, whose value is returned by invoking the `%site-ccache-dir` procedure. As in the previous example, if Guile 3.0 is installed on your system in `/usr/`, then `(%site-ccache-dir)` site packages will be `/usr/lib/guile/3.0/site-ccache`.

Note that a `.go` file will only be loaded in preference to a `.scm` file if it is newer. For that reason, you should install your Scheme files first, and your compiled files second. See [Load Paths](06_16_reading_and_evaluating_scheme_code.md#6168-load-paths), for more on the loading process.

Finally, although this section is only about Scheme, sometimes you need to install C extensions too. Shared libraries should be installed in the _extensions dir_. This value can be had from the build config (see [Configuration, Build and Installation](06_23_configuration_features_and_runtime_options.md#6231-configuration-build-and-installation)). Again, if Guile 3.0 is installed on your system in `/usr/`, then the extensions dir will be `/usr/lib/guile/3.0/extensions`.

* * *

Previous: [Installing Site Packages](04_programming_in_scheme.md#47-installing-site-packages), Up: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 4.8 Distributing Guile Code [¶](04_programming_in_scheme.md#48-distributing-guile-code)

There’s a tool that doesn’t come bundled with Guile and yet can be very useful in your day to day experience with it. This tool is [Hall](https://gitlab.com/a-sassmannshausen/guile-hall).

Hall helps you create, manage, and package your Guile projects through a simple command-line interface. When you start a new project, Hall creates a folder containing a scaffold of your new project. It contains a directory for your tests, for your libraries, for your scripts and for your documentation. This means you immediately know where to put the files you are hacking on.

In addition, the scaffold will include your basic “Autotools” setup, so you don’t have to take care of that yourself (see [The GNU Build System](https://www.gnu.org/software/autoconf/manual/autoconf.html#The-GNU-Build-System) in Autoconf: Creating Automatic Configuration Scripts, for more information on the GNU “Autotools”). Having Autotools set up with your project means you can immediately start hacking on your project without worrying about whether your code will work on other people’s computers. Hall can also generate package definitions for the GNU Guix package manager, making it easy for Guix users to install it.

* * *

Next: [API Reference](06_00_api_reference.md#6-api-reference), Previous: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme), Up: [The Guile Reference Manual](00_contents.md)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
