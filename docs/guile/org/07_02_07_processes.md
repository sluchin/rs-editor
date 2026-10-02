#### 7.2.7 Processes [¶](07_02_07_processes.md#727-processes)

Scheme Procedure: **chdir** str [¶](07_02_07_processes.md)

C Function: **scm\_chdir** (str) [¶](07_02_07_processes.md)

Change the current working directory to str. str can be a string containing a file name, or a port if supported by the system. `(provided? 'chdir-port)` reports whether ports are supported. The return value is unspecified.

Scheme Procedure: **getcwd** [¶](07_02_07_processes.md)

C Function: **scm\_getcwd** () [¶](07_02_07_processes.md)

Return the name of the current working directory.

Scheme Procedure: **umask** \[mode\] [¶](07_02_07_processes.md)

C Function: **scm\_umask** (mode) [¶](07_02_07_processes.md)

If mode is omitted, returns a decimal number representing the current file creation mask. Otherwise the file creation mask is set to mode and the previous value is returned. See [Assigning File Permissions](https://doc.guix.gnu.org/libc/latest/en/libc.html#Setting-Permissions) in The GNU C Library Reference Manual, for more on how to use umasks.

E.g., `(umask #o022)` sets the mask to octal 22/decimal 18.

Scheme Procedure: **chroot** path [¶](07_02_07_processes.md)

C Function: **scm\_chroot** (path) [¶](07_02_07_processes.md)

Change the root directory to that specified in path. This directory will be used for path names beginning with /. The root directory is inherited by all children of the current process. Only the superuser may change the root directory.

Scheme Procedure: **getpid** [¶](07_02_07_processes.md)

C Function: **scm\_getpid** () [¶](07_02_07_processes.md)

Return an integer representing the current process ID.

Scheme Procedure: **getgroups** [¶](07_02_07_processes.md)

C Function: **scm\_getgroups** () [¶](07_02_07_processes.md)

Return a vector of integers representing the current supplementary group IDs.

Scheme Procedure: **getppid** [¶](07_02_07_processes.md)

C Function: **scm\_getppid** () [¶](07_02_07_processes.md)

Return an integer representing the process ID of the parent process.

Scheme Procedure: **getuid** [¶](07_02_07_processes.md)

C Function: **scm\_getuid** () [¶](07_02_07_processes.md)

Return an integer representing the current real user ID.

Scheme Procedure: **getgid** [¶](07_02_07_processes.md)

C Function: **scm\_getgid** () [¶](07_02_07_processes.md)

Return an integer representing the current real group ID.

Scheme Procedure: **geteuid** [¶](07_02_07_processes.md)

C Function: **scm\_geteuid** () [¶](07_02_07_processes.md)

Return an integer representing the current effective user ID. If the system does not support effective IDs, then the real ID is returned. `(provided? 'EIDs)` reports whether the system supports effective IDs.

Scheme Procedure: **getegid** [¶](07_02_07_processes.md)

C Function: **scm\_getegid** () [¶](07_02_07_processes.md)

Return an integer representing the current effective group ID. If the system does not support effective IDs, then the real ID is returned. `(provided? 'EIDs)` reports whether the system supports effective IDs.

Scheme Procedure: **setgroups** vec [¶](07_02_07_processes.md)

C Function: **scm\_setgroups** (vec) [¶](07_02_07_processes.md)

Set the current set of supplementary group IDs to the integers in the given vector vec. The return value is unspecified.

Generally only the superuser can set the process group IDs (see [Setting the Group IDs](https://doc.guix.gnu.org/libc/latest/en/libc.html#Setting-Groups) in The GNU C Library Reference Manual).

Scheme Procedure: **setuid** id [¶](07_02_07_processes.md)

C Function: **scm\_setuid** (id) [¶](07_02_07_processes.md)

Sets both the real and effective user IDs to the integer id, provided the process has appropriate privileges. The return value is unspecified.

Scheme Procedure: **setgid** id [¶](07_02_07_processes.md)

C Function: **scm\_setgid** (id) [¶](07_02_07_processes.md)

Sets both the real and effective group IDs to the integer id, provided the process has appropriate privileges. The return value is unspecified.

Scheme Procedure: **seteuid** id [¶](07_02_07_processes.md)

C Function: **scm\_seteuid** (id) [¶](07_02_07_processes.md)

Sets the effective user ID to the integer id, provided the process has appropriate privileges. If effective IDs are not supported, the real ID is set instead—`(provided? 'EIDs)` reports whether the system supports effective IDs. The return value is unspecified.

Scheme Procedure: **setegid** id [¶](07_02_07_processes.md)

C Function: **scm\_setegid** (id) [¶](07_02_07_processes.md)

Sets the effective group ID to the integer id, provided the process has appropriate privileges. If effective IDs are not supported, the real ID is set instead—`(provided? 'EIDs)` reports whether the system supports effective IDs. The return value is unspecified.

Scheme Procedure: **getpgrp** [¶](07_02_07_processes.md)

C Function: **scm\_getpgrp** () [¶](07_02_07_processes.md)

Return an integer representing the current process group ID. This is the POSIX definition, not BSD.

Scheme Procedure: **setpgid** pid pgid [¶](07_02_07_processes.md)

C Function: **scm\_setpgid** (pid, pgid) [¶](07_02_07_processes.md)

Move the process pid into the process group pgid. pid or pgid must be integers: they can be zero to indicate the ID of the current process. Fails on systems that do not support job control. The return value is unspecified.

Scheme Procedure: **setsid** [¶](07_02_07_processes.md)

C Function: **scm\_setsid** () [¶](07_02_07_processes.md)

Creates a new session. The current process becomes the session leader and is put in a new process group. The process will be detached from its controlling terminal if it has one. The return value is an integer representing the new process group ID.

Scheme Procedure: **getsid** pid [¶](07_02_07_processes.md)

C Function: **scm\_getsid** (pid) [¶](07_02_07_processes.md)

Returns the session ID of process pid. (The session ID of a process is the process group ID of its session leader.)

Scheme Procedure: **waitpid** pid \[options\] [¶](07_02_07_processes.md)

C Function: **scm\_waitpid** (pid, options) [¶](07_02_07_processes.md)

This procedure collects status information from a child process which has terminated or (optionally) stopped. Normally it will suspend the calling process until this can be done. If more than one child process is eligible then one will be chosen by the operating system.

The value of pid determines the behavior:

pid greater than 0

Request status information from the specified child process.

pid equal to -1 or `WAIT_ANY` [¶](07_02_07_processes.md)

Request status information for any child process.

pid equal to 0 or `WAIT_MYPGRP` [¶](07_02_07_processes.md)

Request status information for any child process in the current process group.

pid less than -1

Request status information for any child process whose process group ID is −pid.

The options argument, if supplied, should be the bitwise OR of the values of zero or more of the following variables:

Variable: **WNOHANG** [¶](07_02_07_processes.md)

Return immediately even if there are no child processes to be collected.

Variable: **WUNTRACED** [¶](07_02_07_processes.md)

Report status information for stopped processes as well as terminated processes.

The return value is a pair containing:

1.  The process ID of the child process, or 0 if `WNOHANG` was specified and no process was collected.
2.  The integer status value (see [Process Completion Status](https://doc.guix.gnu.org/libc/latest/en/libc.html#Process-Completion-Status) in The GNU C Library Reference Manual).

The following three functions can be used to decode the integer status value returned by `waitpid`.

Scheme Procedure: **status:exit-val** status [¶](07_02_07_processes.md)

C Function: **scm\_status\_exit\_val** (status) [¶](07_02_07_processes.md)

Return the exit status value, as would be set if a process ended normally through a call to `exit` or `_exit`, if any, otherwise `#f`.

Scheme Procedure: **status:term-sig** status [¶](07_02_07_processes.md)

C Function: **scm\_status\_term\_sig** (status) [¶](07_02_07_processes.md)

Return the signal number which terminated the process, if any, otherwise `#f`.

Scheme Procedure: **status:stop-sig** status [¶](07_02_07_processes.md)

C Function: **scm\_status\_stop\_sig** (status) [¶](07_02_07_processes.md)

Return the signal number which stopped the process, if any, otherwise `#f`.

Scheme Procedure: **system** \[cmd\] [¶](07_02_07_processes.md)

C Function: **scm\_system** (cmd) [¶](07_02_07_processes.md)

Execute cmd using the operating system’s “command processor”. Under Unix this is usually the default shell `sh`. The value returned is cmd’s exit status as returned by `waitpid`, which can be interpreted using the functions above.

If `system` is called without arguments, return a boolean indicating whether the command processor is available.

Scheme Procedure: **system\*** arg1 arg2 … [¶](07_02_07_processes.md)

C Function: **scm\_system\_star** (args) [¶](07_02_07_processes.md)

Execute the command indicated by arg1 arg2 .... The first element must be a string indicating the command to be executed, and the remaining items must be strings representing each of the arguments to that command.

This function returns the exit status of the command as provided by `waitpid`. This value can be handled with `status:exit-val` and the related functions.

`system*` is similar to `system`, but accepts only one string per-argument, and performs no shell interpretation. The command is executed using fork and execlp. Accordingly this function may be safer than `system` in situations where shell interpretation is not required.

Example: (system\* "echo" "foo" "bar")

Scheme Procedure: **quit** \[status\] [¶](07_02_07_processes.md)

Scheme Procedure: **exit** \[status\] [¶](07_02_07_processes.md)

Terminate the current process with proper unwinding of the Scheme stack. The exit status zero if status is not supplied. If status is supplied, and it is an integer, that integer is used as the exit status. If status is `#t` or `#f`, the exit status is EXIT\_SUCCESS or EXIT\_FAILURE, respectively.

The procedure `exit` is an alias of `quit`. They have the same functionality.

Scheme Variable: **EXIT\_SUCCESS** [¶](07_02_07_processes.md)

Scheme Variable: **EXIT\_FAILURE** [¶](07_02_07_processes.md)

These constants represent the standard exit codes for success (zero) or failure (one.)

Scheme Procedure: **primitive-exit** \[status\] [¶](07_02_07_processes.md)

Scheme Procedure: **primitive-\_exit** \[status\] [¶](07_02_07_processes.md)

C Function: **scm\_primitive\_exit** (status) [¶](07_02_07_processes.md)

C Function: **scm\_primitive\_\_exit** (status) [¶](07_02_07_processes.md)

Terminate the current process without unwinding the Scheme stack. The exit status is status if supplied, otherwise zero.

`primitive-exit` uses the C `exit` function and hence runs usual C level cleanups (flush output streams, call `atexit` functions, etc, see [Normal Termination](https://doc.guix.gnu.org/libc/latest/en/libc.html#Normal-Termination) in The GNU C Library Reference Manual)).

`primitive-_exit` is the `_exit` system call (see [Termination Internals](https://doc.guix.gnu.org/libc/latest/en/libc.html#Termination-Internals) in The GNU C Library Reference Manual). This terminates the program immediately, with neither Scheme-level nor C-level cleanups.

The typical use for `primitive-_exit` is from a child process created with `primitive-fork`. For example in a Gdk program the child process inherits the X server connection and a C-level `atexit` cleanup which will close that connection. But closing in the child would upset the protocol in the parent, so `primitive-_exit` should be used to exit without that.

Scheme Procedure: **execl** filename arg … [¶](07_02_07_processes.md)

C Function: **scm\_execl** (filename, args) [¶](07_02_07_processes.md)

Executes the file named by filename as a new process image. The remaining arguments are supplied to the process; from a C program they are accessible as the `argv` argument to `main`. Conventionally the first arg is the same as filename. All arguments must be strings.

If arg is missing, filename is executed with a null argument list, which may have system-dependent side-effects.

This procedure is currently implemented using the `execv` system call, but we call it `execl` because of its Scheme calling interface.

Scheme Procedure: **execlp** filename arg … [¶](07_02_07_processes.md)

C Function: **scm\_execlp** (filename, args) [¶](07_02_07_processes.md)

Similar to `execl`, however if filename does not contain a slash then the file to execute will be located by searching the directories listed in the `PATH` environment variable.

This procedure is currently implemented using the `execvp` system call, but we call it `execlp` because of its Scheme calling interface.

Scheme Procedure: **execle** filename env arg … [¶](07_02_07_processes.md)

C Function: **scm\_execle** (filename, env, args) [¶](07_02_07_processes.md)

Similar to `execl`, but the environment of the new process is specified by env, which must be a list of strings as returned by the `environ` procedure.

This procedure is currently implemented using the `execve` system call, but we call it `execle` because of its Scheme calling interface.

Scheme Procedure: **primitive-fork** [¶](07_02_07_processes.md)

C Function: **scm\_fork** () [¶](07_02_07_processes.md)

Creates a new “child” process by duplicating the current “parent” process. In the child the return value is 0. In the parent the return value is the integer process ID of the child.

Note that it is unsafe to fork a process that has multiple threads running, as only the thread that calls `primitive-fork` will persist in the child. Any resources that other threads held, such as locked mutexes or open file descriptors, are lost. Indeed, POSIX specifies that only async-signal-safe procedures are safe to call after a multithreaded fork, which is a very limited set. Guile issues a warning if it detects a fork from a multi-threaded program.

> **Note:** If you are looking to spawn a process with some pipes set up, using the `spawn` procedure described below will be more robust (in particular in multi-threaded contexts), more portable, and usually more efficient than the combination of `primitive-fork` and `execl`.

This procedure has been renamed from `fork` to avoid a naming conflict with the scsh fork.

Scheme Procedure: **spawn** program arguments \[#:environment=(environ)\] \[#:input=(current-input-port)\] \[#:output=(current-output-port)\] \[#:error=(current-error-port)\] \[#:search-path?=#t\] [¶](07_02_07_processes.md)

Spawn a new child process executing program with the given arguments, a list of one or more strings (by convention, the first argument is typically program), and return its PID. Raise a `system-error` exception if program could not be found or could not be executed.

If the keyword argument `#:search-path?` is true, it selects whether the `PATH` environment variable should be inspected to find program. It is true by default.

The `#:environment` keyword parameter specifies the list of environment variables of the child process. It defaults to `(environ)`.

The keyword arguments `#:input`, `#:output`, and `#:error` specify the port or file descriptor for the child process to use as standard input, standard output, and standard error. No other file descriptors are inherited from the parent process.

The example below shows how to spawn the `uname` program with the \-o option (see [uname invocation](https://www.gnu.org/software/coreutils/manual/coreutils.html#uname-invocation) in GNU Coreutils), redirect its standard output to a pipe, and read from it:

([use-modules](06_18_modules.md) (rnrs io ports))

(let\* ((input+output ([pipe](07_02_02_ports_and_file_descriptors.md)))
       (pid ([spawn](07_02_07_processes.md) "uname" '("uname" "-o")
                    #:output ([cdr](06_06_08_pairs.md) input+output))))
  ([close-port](06_12_input_and_output.md) ([cdr](06_06_08_pairs.md) input+output))
  ([format](07_05_20_srfi28_basic_format_strings.md) #t "read ~s~%" ([get-string-all](06_12_input_and_output.md) ([car](06_06_08_pairs.md) input+output)))
  ([close-port](06_12_input_and_output.md) ([car](06_06_08_pairs.md) input+output))
  ([waitpid](07_02_07_processes.md) pid))

⊣ [read](06_16_reading_and_evaluating_scheme_code.md) "GNU/Linux\\n"
⇒ (1234 . 0)

Scheme Procedure: **nice** incr [¶](07_02_07_processes.md)

C Function: **scm\_nice** (incr) [¶](07_02_07_processes.md)

Increment the priority of the current process by incr. A higher priority value means that the process runs less often. The return value is unspecified.

Scheme Procedure: **setpriority** which who prio [¶](07_02_07_processes.md)

C Function: **scm\_setpriority** (which, who, prio) [¶](07_02_07_processes.md)

Set the scheduling priority of the process, process group or user, as indicated by which and who. which is one of the variables `PRIO_PROCESS`, `PRIO_PGRP` or `PRIO_USER`, and who is interpreted relative to which (a process identifier for `PRIO_PROCESS`, process group identifier for `PRIO_PGRP`, and a user identifier for `PRIO_USER`. A zero value of who denotes the current process, process group, or user. prio is a value in the range \[−20,20\]. The default priority is 0; lower priorities (in numerical terms) cause more favorable scheduling. Sets the priority of all of the specified processes. Only the super-user may lower priorities. The return value is not specified.

Scheme Procedure: **getpriority** which who [¶](07_02_07_processes.md)

C Function: **scm\_getpriority** (which, who) [¶](07_02_07_processes.md)

Return the scheduling priority of the process, process group or user, as indicated by which and who. which is one of the variables `PRIO_PROCESS`, `PRIO_PGRP` or `PRIO_USER`, and who should be interpreted depending on which (a process identifier for `PRIO_PROCESS`, process group identifier for `PRIO_PGRP`, and a user identifier for `PRIO_USER`). A zero value of who denotes the current process, process group, or user. Return the highest priority (lowest numerical value) of any of the specified processes.

Scheme Procedure: **getaffinity** pid [¶](07_02_07_processes.md)

C Function: **scm\_getaffinity** (pid) [¶](07_02_07_processes.md)

Return a bitvector representing the CPU affinity mask for process pid. Each CPU the process has affinity with has its corresponding bit set in the returned bitvector. The number of bits set is a good estimate of how many CPUs Guile can use without stepping on other processes’ toes.

Currently this procedure is only defined on GNU variants (see [`sched_getaffinity`](https://doc.guix.gnu.org/libc/latest/en/libc.html#CPU-Affinity) in The GNU C Library Reference Manual).

Scheme Procedure: **setaffinity** pid mask [¶](07_02_07_processes.md)

C Function: **scm\_setaffinity** (pid, mask) [¶](07_02_07_processes.md)

Install the CPU affinity mask mask, a bitvector, for the process or thread with ID pid. The return value is unspecified.

Currently this procedure is only defined on GNU variants (see [`sched_setaffinity`](https://doc.guix.gnu.org/libc/latest/en/libc.html#CPU-Affinity) in The GNU C Library Reference Manual).

See [Threads](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads), for information on how get the number of processors available on a system.

* * *

Next: [Terminals and Ptys](07_02_09_terminals_and_ptys.md#729-terminals-and-ptys), Previous: [Processes](07_02_07_processes.md#727-processes), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
