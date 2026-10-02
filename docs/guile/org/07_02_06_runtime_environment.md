#### 7.2.6 Runtime Environment [¶](07_02_06_runtime_environment.md#726-runtime-environment)

Scheme Procedure: **program-arguments** [¶](07_02_06_runtime_environment.md)

Scheme Procedure: **command-line** [¶](07_02_06_runtime_environment.md)

Scheme Procedure: **set-program-arguments** [¶](07_02_06_runtime_environment.md)

C Function: **scm\_program\_arguments** () [¶](07_02_06_runtime_environment.md)

C Function: **scm\_set\_program\_arguments\_scm** (lst) [¶](07_02_06_runtime_environment.md)

Get the command line arguments passed to Guile, or set new arguments.

The arguments are a list of strings, the first of which is the invoked program name. This is just `"guile"` (or the executable path) when run interactively, or it’s the script name when running a script with \-s (see [Invoking Guile](04_programming_in_scheme.md#42-invoking-guile)).

guile -L /my/extra/dir -s foo.scm abc def

(program-arguments) ⇒ ("foo.scm" "abc" "def")

`set-program-arguments` allows a library module or similar to modify the arguments, for example to strip options it recognises, leaving the rest for the mainline.

The argument list is held in a fluid, which means it’s separate for each thread. Neither the list nor the strings within it are copied at any point and normally should not be mutated.

The two names `program-arguments` and `command-line` are an historical accident, they both do exactly the same thing. The name `scm_set_program_arguments_scm` has an extra `_scm` on the end to avoid clashing with the C function below.

C Function: `void` **scm\_set\_program\_arguments** `(int argc, char **argv, char *first)` [¶](07_02_06_runtime_environment.md)

Set the list of command line arguments for `program-arguments` and `command-line` above.

argv is an array of null-terminated strings, as in a C `main` function. argc is the number of strings in argv, or if it’s negative then a `NULL` in argv marks its end.

first is an extra string put at the start of the arguments, or `NULL` for no such extra. This is a convenient way to pass the program name after advancing argv to strip option arguments. Eg.

{
  char \*progname = argv\[0\];
  for (argv++; argv\[0\] != NULL && argv\[0\]\[0\] == '-'; argv++)
    {
      /\* munch option ... \*/
    }
  /\* remaining args for scheme level use \*/
  scm\_set\_program\_arguments (\-1, argv, progname);
}

This sort of thing is often done at startup under `scm_boot_guile` with options handled at the C level removed. The given strings are all copied, so the C data is not accessed again once `scm_set_program_arguments` returns.

Scheme Procedure: **getenv** name [¶](07_02_06_runtime_environment.md)

C Function: **scm\_getenv** (name) [¶](07_02_06_runtime_environment.md)

Looks up the string name in the current environment. The return value is `#f` unless a string of the form `NAME=VALUE` is found, in which case the string `VALUE` is returned.

Scheme Procedure: **setenv** name value [¶](07_02_06_runtime_environment.md)

Modifies the environment of the current process, which is also the default environment inherited by child processes.

If value is `#f`, then name is removed from the environment. Otherwise, the string name\=value is added to the environment, replacing any existing string with name matching name.

The return value is unspecified.

Scheme Procedure: **unsetenv** name [¶](07_02_06_runtime_environment.md)

Remove variable name from the environment. The name can not contain a ‘\=’ character.

Scheme Procedure: **environ** \[env\] [¶](07_02_06_runtime_environment.md)

C Function: **scm\_environ** (env) [¶](07_02_06_runtime_environment.md)

If env is omitted, return the current environment (in the Unix sense) as a list of strings. Otherwise set the current environment, which is also the default environment for child processes, to the supplied list of strings. Each member of env should be of the form name\=value and values of name should not be duplicated. If env is supplied then the return value is unspecified.

Scheme Procedure: **putenv** str [¶](07_02_06_runtime_environment.md)

C Function: **scm\_putenv** (str) [¶](07_02_06_runtime_environment.md)

Modifies the environment of the current process, which is also the default environment inherited by child processes.

If str is of the form `NAME=VALUE` then it will be written directly into the environment, replacing any existing environment string with name matching `NAME`. If str does not contain an equal sign, then any existing string with name matching str will be removed.

The return value is unspecified.

* * *

Next: [Signals](07_02_08_signals.md#728-signals), Previous: [Runtime Environment](07_02_06_runtime_environment.md#726-runtime-environment), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
