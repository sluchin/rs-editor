2 Hello Guile! [¶](02_hello_guile.md#2-hello-guile)
------------------------------------------------------------------------------------------

This chapter presents a quick tour of all the ways that Guile can be used. There are additional examples in the examples/ directory in the Guile source distribution. It also explains how best to report any problems that you find.

The following examples assume that Guile has been installed in `/usr/local/`.

*   [Running Guile Interactively](02_hello_guile.md#21-running-guile-interactively)
*   [Running Guile Scripts](02_hello_guile.md#22-running-guile-scripts)
*   [Linking Guile into Programs](02_hello_guile.md#23-linking-guile-into-programs)
*   [Writing Guile Extensions](02_hello_guile.md#24-writing-guile-extensions)
*   [Using the Guile Module System](02_hello_guile.md#25-using-the-guile-module-system)
*   [Reporting Bugs](02_hello_guile.md#26-reporting-bugs)

* * *

Next: [Running Guile Scripts](02_hello_guile.md#22-running-guile-scripts), Up: [Hello Guile!](02_hello_guile.md#2-hello-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 2.1 Running Guile Interactively [¶](02_hello_guile.md#21-running-guile-interactively)

In its simplest form, Guile acts as an interactive interpreter for the Scheme programming language, reading and evaluating Scheme expressions the user enters from the terminal. Here is a sample interaction between Guile and a user; the user’s input appears after the `$` and `scheme@(guile-user)>` prompts:

$ guile
scheme@(guile-user)> (+ 1 2 3)                ; add some numbers
$1 = 6
scheme@(guile-user)> (define (factorial n)    ; define a function
                       (if (zero? n) 1 (\* n (factorial (- n 1)))))
scheme@(guile-user)> (factorial 20)
$2 = 2432902008176640000
scheme@(guile-user)> (getpwnam "root")        ; look in /etc/passwd
$3 = #("root" "x" 0 0 "root" "/root" "/bin/bash")
scheme@(guile-user)> C-d
$

* * *

Next: [Linking Guile into Programs](02_hello_guile.md#23-linking-guile-into-programs), Previous: [Running Guile Interactively](02_hello_guile.md#21-running-guile-interactively), Up: [Hello Guile!](02_hello_guile.md#2-hello-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 2.2 Running Guile Scripts [¶](02_hello_guile.md#22-running-guile-scripts)

Like AWK, Perl, or any shell, Guile can interpret script files. A Guile script is simply a file of Scheme code with some extra information at the beginning which tells the operating system how to invoke Guile, and then tells Guile how to handle the Scheme code.

Here is a trivial Guile script. See [Guile Scripting](04_programming_in_scheme.md#43-guile-scripting), for more details.

#!/usr/local/bin/guile -s
!#
(display "Hello, world!")
(newline)

* * *

Next: [Writing Guile Extensions](02_hello_guile.md#24-writing-guile-extensions), Previous: [Running Guile Scripts](02_hello_guile.md#22-running-guile-scripts), Up: [Hello Guile!](02_hello_guile.md#2-hello-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 2.3 Linking Guile into Programs [¶](02_hello_guile.md#23-linking-guile-into-programs)

The Guile interpreter is available as an object library, to be linked into applications using Scheme as a configuration or extension language.

Here is simple-guile.c, source code for a program that will produce a complete Guile interpreter. In addition to all usual functions provided by Guile, it will also offer the function `my-hostname`.

#include <stdlib.h>
#include <libguile.h>

static SCM
my\_hostname (void)
{
  char \*s = getenv ("HOSTNAME");
  if (s == NULL)
    return SCM\_BOOL\_F;
  else
    return scm\_from\_locale\_string (s);
}

static void
inner\_main (void \*data, int argc, char \*\*argv)
{
  scm\_c\_define\_gsubr ("my-hostname", 0, 0, 0, my\_hostname);
  scm\_shell (argc, argv);
}

int
main (int argc, char \*\*argv)
{
  scm\_boot\_guile (argc, argv, inner\_main, 0);
  return 0; /\* never reached \*/
}

When Guile is correctly installed on your system, the above program can be compiled and linked like this:

$ gcc -o simple-guile simple-guile.c \\
    \`pkg-config --cflags --libs guile-3.0\`

When it is run, it behaves just like the `guile` program except that you can also call the new `my-hostname` function.

$ ./simple-guile
scheme@(guile-user)> (+ 1 2 3)
$1 = 6
scheme@(guile-user)> (my-hostname)
"burns"

* * *

Next: [Using the Guile Module System](02_hello_guile.md#25-using-the-guile-module-system), Previous: [Linking Guile into Programs](02_hello_guile.md#23-linking-guile-into-programs), Up: [Hello Guile!](02_hello_guile.md#2-hello-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 2.4 Writing Guile Extensions [¶](02_hello_guile.md#24-writing-guile-extensions)

You can link Guile into your program and make Scheme available to the users of your program. You can also link your library into Guile and make its functionality available to all users of Guile.

A library that is linked into Guile is called an _extension_, but it really just is an ordinary object library.

The following example shows how to write a simple extension for Guile that makes the `j0` function available to Scheme code.

#include <math.h>
#include <libguile.h>

SCM
j0\_wrapper (SCM x)
{
  return scm\_from\_double (j0 (scm\_to\_double (x)));
}

void
init\_bessel ()
{
  scm\_c\_define\_gsubr ("j0", 1, 0, 0, j0\_wrapper);
}

This C source file needs to be compiled into a shared library. Here is how to do it on GNU/Linux:

gcc \`pkg-config --cflags guile-3.0\` \\
  -shared -o libguile-bessel.so -fPIC bessel.c

For creating shared libraries portably, we recommend the use of GNU Libtool (see [Introduction](https://www.gnu.org/software/libtool/manual/libtool.html#Top) in GNU Libtool).

A shared library can be loaded into a running Guile process with the function `load-extension`. The `j0` is then immediately available:

$ guile
scheme@(guile-user)> (load-extension "./libguile-bessel" "init\_bessel")
scheme@(guile-user)> (j0 2)
$1 = 0.223890779141236

For more on how to install your extension, see [Installing Site Packages](04_programming_in_scheme.md#47-installing-site-packages).

* * *

Next: [Reporting Bugs](02_hello_guile.md#26-reporting-bugs), Previous: [Writing Guile Extensions](02_hello_guile.md#24-writing-guile-extensions), Up: [Hello Guile!](02_hello_guile.md#2-hello-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 2.5 Using the Guile Module System [¶](02_hello_guile.md#25-using-the-guile-module-system)

Guile has support for dividing a program into _modules_. By using modules, you can group related code together and manage the composition of complete programs from largely independent parts.

For more details on the module system beyond this introductory material, See [Modules](06_18_modules.md#618-modules).

*   [Using Modules](02_hello_guile.md#251-using-modules)
*   [Writing new Modules](02_hello_guile.md#252-writing-new-modules)
*   [Putting Extensions into Modules](02_hello_guile.md#253-putting-extensions-into-modules)

* * *

Next: [Writing new Modules](02_hello_guile.md#252-writing-new-modules), Up: [Using the Guile Module System](02_hello_guile.md#25-using-the-guile-module-system)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 2.5.1 Using Modules [¶](02_hello_guile.md#251-using-modules)

Guile comes with a lot of useful modules, for example for string processing or command line parsing. Additionally, there exist many Guile modules written by other Guile hackers, but which have to be installed manually.

Here is a sample interactive session that shows how to use the `(ice-9 popen)` module which provides the means for communicating with other processes over pipes together with the `(ice-9 rdelim)` module that provides the function `read-line`.[1](99_footnotes.md)

$ guile
scheme@(guile-user)> (use-modules (ice-9 popen))
scheme@(guile-user)> (use-modules (ice-9 rdelim))
scheme@(guile-user)> (define p (open-input-pipe "ls -l"))
scheme@(guile-user)> (read-line p)
$1 = "total 30"
scheme@(guile-user)> (read-line p)
$2 = "drwxr-sr-x    2 mgrabmue mgrabmue     1024 Mar 29 19:57 CVS"

* * *

Next: [Putting Extensions into Modules](02_hello_guile.md#253-putting-extensions-into-modules), Previous: [Using Modules](02_hello_guile.md#251-using-modules), Up: [Using the Guile Module System](02_hello_guile.md#25-using-the-guile-module-system)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 2.5.2 Writing new Modules [¶](02_hello_guile.md#252-writing-new-modules)

You can create new modules using the syntactic form `define-module`. All definitions following this form until the next `define-module` are placed into the new module.

One module is usually placed into one file, and that file is installed in a location where Guile can automatically find it. The following session shows a simple example.

$ cat /usr/local/share/guile/site/foo/bar.scm

(define-module (foo bar)
  #:export (frob))

(define (frob x) (\* 2 x))

$ guile
scheme@(guile-user)> (use-modules (foo bar))
scheme@(guile-user)> (frob 12)
$1 = 24

For more on how to install your module, see [Installing Site Packages](04_programming_in_scheme.md#47-installing-site-packages).

* * *

Previous: [Writing new Modules](02_hello_guile.md#252-writing-new-modules), Up: [Using the Guile Module System](02_hello_guile.md#25-using-the-guile-module-system)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 2.5.3 Putting Extensions into Modules [¶](02_hello_guile.md#253-putting-extensions-into-modules)

In addition to Scheme code you can also put things that are defined in C into a module.

You do this by writing a small Scheme file that defines the module and call `load-extension` directly in the body of the module.

$ cat /usr/local/share/guile/site/math/bessel.scm

(define-module (math bessel)
  #:export (j0))

(load-extension "libguile-bessel" "init\_bessel")

$ file /usr/local/lib/guile/3.0/extensions/libguile-bessel.so
... ELF 32-bit LSB shared object ...
$ guile
scheme@(guile-user)> (use-modules (math bessel))
scheme@(guile-user)> (j0 2)
$1 = 0.223890779141236

See [Foreign Extensions](06_19_foreign_function_interface.md#6192-foreign-extensions), for more information.

* * *

Previous: [Using the Guile Module System](02_hello_guile.md#25-using-the-guile-module-system), Up: [Hello Guile!](02_hello_guile.md#2-hello-guile)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

### 2.6 Reporting Bugs [¶](02_hello_guile.md#26-reporting-bugs)

Any problems with the installation should be reported to [bug-guile@gnu.org](mailto:bug-guile@gnu.org).

If you find a bug in Guile, please report it to the Guile developers, so they can fix it. They may also be able to suggest workarounds when it is not possible for you to apply the bug-fix or install a new version of Guile yourself.

Before sending in bug reports, please check with the following list that you really have found a bug.

*   Whenever documentation and actual behavior differ, you have certainly found a bug, either in the documentation or in the program.
*   When Guile crashes, it is a bug.
*   When Guile hangs or takes forever to complete a task, it is a bug.
*   When calculations produce wrong results, it is a bug.
*   When Guile signals an error for valid Scheme programs, it is a bug.
*   When Guile does not signal an error for invalid Scheme programs, it may be a bug, unless this is explicitly documented.
*   When some part of the documentation is not clear and does not make sense to you even after re-reading the section, it is a bug.

Before reporting the bug, check whether any programs you have loaded into Guile, including your .guile file, set any variables that may affect the functioning of Guile. Also, see whether the problem happens in a freshly started Guile without loading your .guile file (start Guile with the `-q` switch to prevent loading the init file). If the problem does _not_ occur then, you must report the precise contents of any programs that you must load into Guile in order to cause the problem to occur.

When you write a bug report, please make sure to include as much of the information described below in the report. If you can’t figure out some of the items, it is not a problem, but the more information we get, the more likely we can diagnose and fix the bug.

*   The version number of Guile. You can get this information from invoking ‘guile --version’ at your shell, or calling `(version)` from within Guile.
*   Your machine type, as determined by the `config.guess` shell script. If you have a Guile checkout, this file is located in `build-aux`; otherwise you can fetch the latest version from [http://git.savannah.gnu.org/gitweb/?p=config.git;a=blob\_plain;f=config.guess;hb=HEAD](http://git.savannah.gnu.org/gitweb/?p=config.git;a=blob_plain;f=config.guess;hb=HEAD).
    
    $ build-aux/config.guess
    x86\_64-unknown-linux-gnu
    
*   If you installed Guile from a binary package, the version of that package. On systems that use RPM, use `rpm -qa | grep guile`. On systems that use DPKG, `dpkg -l | grep guile`.
*   If you built Guile yourself, the build configuration that you used:
    
    $ ./config.status --config
    '--enable-error-on-warning' '--disable-deprecated'...
    
*   A complete description of how to reproduce the bug.
    
    If you have a Scheme program that produces the bug, please include it in the bug report. If your program is too big to include, please try to reduce your code to a minimal test case.
    
    If you can reproduce your problem at the REPL, that is best. Give a transcript of the expressions you typed at the REPL.
    
*   A description of the incorrect behavior. For example, "The Guile process gets a fatal signal," or, "The resulting output is as follows, which I think is wrong."
    
    If the manifestation of the bug is a Guile error message, it is important to report the precise text of the error message, and a backtrace showing how the Scheme program arrived at the error. This can be done using the `,backtrace` command in Guile’s debugger.
    

If your bug causes Guile to crash, additional information from a low-level debugger such as GDB might be helpful. If you have built Guile yourself, you can run Guile under GDB via the `meta/gdb-uninstalled-guile` script. Instead of invoking Guile as usual, invoke the wrapper script, type `run` to start the process, then `backtrace` when the crash comes. Include that backtrace in your report.

* * *

Next: [Programming in Scheme](04_programming_in_scheme.md#4-programming-in-scheme), Previous: [Hello Guile!](02_hello_guile.md#2-hello-guile), Up: [The Guile Reference Manual](00_contents.md)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
