#### 7.2.8 Signals [¶](07_02_08_signals.md#728-signals)

The following procedures raise, handle and wait for signals.

Scheme code signal handlers are run via an async (see [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts)), so they’re called in the handler’s thread at the next safe opportunity. Generally this is after any currently executing primitive procedure finishes (which could be a long time for primitives that wait for an external event).

Scheme Procedure: **kill** pid sig [¶](07_02_08_signals.md)

C Function: **scm\_kill** (pid, sig) [¶](07_02_08_signals.md)

Sends a signal to the specified process or group of processes.

pid specifies the processes to which the signal is sent:

pid greater than 0

The process whose identifier is pid.

pid equal to 0

All processes in the current process group.

pid less than -1

The process group whose identifier is -pid

pid equal to -1

If the process is privileged, all processes except for some special system processes. Otherwise, all processes with the current effective user ID.

sig should be specified using a variable corresponding to the Unix symbolic name, e.g.,

Variable: **SIGHUP** [¶](07_02_08_signals.md)

Hang-up signal.

Variable: **SIGINT** [¶](07_02_08_signals.md)

Interrupt signal.

A full list of signals on the GNU system may be found in [Standard Signals](https://doc.guix.gnu.org/libc/latest/en/libc.html#Standard-Signals) in The GNU C Library Reference Manual.

Scheme Procedure: **raise** sig [¶](07_02_08_signals.md)

C Function: **scm\_raise** (sig) [¶](07_02_08_signals.md)

Sends a specified signal sig to the current process, where sig is as described for the `kill` procedure.

Scheme Procedure: **sigaction** signum \[handler \[flags \[thread\]\]\] [¶](07_02_08_signals.md)

C Function: **scm\_sigaction** (signum, handler, flags) [¶](07_02_08_signals.md)

C Function: **scm\_sigaction\_for\_thread** (signum, handler, flags, thread) [¶](07_02_08_signals.md)

Install or report the signal handler for a specified signal.

signum is the signal number, which can be specified using the value of variables such as `SIGINT`.

If handler is omitted, `sigaction` returns a pair: the CAR is the current signal handler, which will be either an integer with the value `SIG_DFL` (default action) or `SIG_IGN` (ignore), or the Scheme procedure which handles the signal, or `#f` if a non-Scheme procedure handles the signal. The CDR contains the current `sigaction` flags for the handler.

If handler is provided, it is installed as the new handler for signum. handler can be a Scheme procedure taking one argument, or the value of `SIG_DFL` (default action) or `SIG_IGN` (ignore), or `#f` to restore whatever signal handler was installed before `sigaction` was first used. When a scheme procedure has been specified, that procedure will run in the given thread. When no thread has been given, the thread that made this call to `sigaction` is used.

flags is a `logior` (see [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations)) of the following (where provided by the system), or `0` for none.

Variable: **SA\_NOCLDSTOP** [¶](07_02_08_signals.md)

By default, `SIGCHLD` is signaled when a child process stops (ie. receives `SIGSTOP`), and when a child process terminates. With the `SA_NOCLDSTOP` flag, `SIGCHLD` is only signaled for termination, not stopping.

`SA_NOCLDSTOP` has no effect on signals other than `SIGCHLD`.

Variable: **SA\_RESTART** [¶](07_02_08_signals.md)

If a signal occurs while in a system call, deliver the signal then restart the system call (as opposed to returning an `EINTR` error from that call).

Guile handles signals asynchronously. When it receives a signal, the synchronous signal handler just records the fact that a signal was received and sets a flag to tell the relevant Guile thread that it has a pending signal. When the Guile thread checks the pending-interrupt flag, it will arrange to run the asynchronous part of the signal handler, which is the handler attached by `sigaction`.

This strategy has some perhaps-unexpected interactions with the `SA_RESTART` flag, though: because the synchronous handler doesn’t do very much, and notably it doesn’t run the Guile handler, it’s impossible to interrupt a thread stuck in a long-running system call via a signal handler that is installed with `SA_RESTART`: the synchronous handler just records the pending interrupt, but then the system call resumes and Guile doesn’t have a chance to actually check the flag and run the asynchronous handler. That’s just how it is.

The return value is a pair with information about the old handler as described above.

This interface does not provide access to the “signal blocking” facility. Maybe this is not needed, since the thread support may provide solutions to the problem of consistent access to data structures.

Scheme Procedure: **restore-signals** [¶](07_02_08_signals.md)

C Function: **scm\_restore\_signals** () [¶](07_02_08_signals.md)

Return all signal handlers to the values they had before any call to `sigaction` was made. The return value is unspecified.

Scheme Procedure: **alarm** i [¶](07_02_08_signals.md)

C Function: **scm\_alarm** (i) [¶](07_02_08_signals.md)

Set a timer to raise a `SIGALRM` signal after the specified number of seconds (an integer). It’s advisable to install a signal handler for `SIGALRM` beforehand, since the default action is to terminate the process.

The return value indicates the time remaining for the previous alarm, if any. The new value replaces the previous alarm. If there was no previous alarm, the return value is zero.

Scheme Procedure: **pause** [¶](07_02_08_signals.md)

C Function: **scm\_pause** () [¶](07_02_08_signals.md)

Pause the current process (thread?) until a signal arrives whose action is to either terminate the current process or invoke a handler procedure. The return value is unspecified.

Scheme Procedure: **sleep** secs [¶](07_02_08_signals.md)

Scheme Procedure: **usleep** usecs [¶](07_02_08_signals.md)

C Function: **scm\_sleep** (secs) [¶](07_02_08_signals.md)

C Function: **scm\_usleep** (usecs) [¶](07_02_08_signals.md)

Wait the given period secs seconds or usecs microseconds (both integers). If a signal arrives the wait stops and the return value is the time remaining, in seconds or microseconds respectively. If the period elapses with no signal the return is zero.

On most systems the process scheduler is not microsecond accurate and the actual period slept by `usleep` might be rounded to a system clock tick boundary, which might be 10 milliseconds for instance.

See `scm_std_sleep` and `scm_std_usleep` for equivalents at the C level (see [Blocking in Guile Mode](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6226-blocking-in-guile-mode)).

Scheme Procedure: **getitimer** which\_timer [¶](07_02_08_signals.md)

Scheme Procedure: **setitimer** which\_timer interval\_seconds interval\_microseconds value\_seconds value\_microseconds [¶](07_02_08_signals.md)

C Function: **scm\_getitimer** (which\_timer) [¶](07_02_08_signals.md)

C Function: **scm\_setitimer** (which\_timer, interval\_seconds, interval\_microseconds, value\_seconds, value\_microseconds) [¶](07_02_08_signals.md)

Get or set the periods programmed in certain system timers.

These timers have two settings. The first setting, the interval, is the value at which the timer will be reset when the current timer expires. The second is the current value of the timer, indicating when the next expiry will be signaled.

which\_timer is one of the following values:

Variable: **ITIMER\_REAL** [¶](07_02_08_signals.md)

A real-time timer, counting down elapsed real time. At zero it raises `SIGALRM`. This is like `alarm` above, but with a higher resolution period.

Variable: **ITIMER\_VIRTUAL** [¶](07_02_08_signals.md)

A virtual-time timer, counting down while the current process is actually using CPU. At zero it raises `SIGVTALRM`.

Variable: **ITIMER\_PROF** [¶](07_02_08_signals.md)

A profiling timer, counting down while the process is running (like `ITIMER_VIRTUAL`) and also while system calls are running on the process’s behalf. At zero it raises a `SIGPROF`.

This timer is intended for profiling where a program is spending its time (by looking where it is when the timer goes off).

`getitimer` returns the restart timer value and its current value, as a list containing two pairs. Each pair is a time in seconds and microseconds: `((interval_secs . interval_usecs) (value_secs . value_usecs))`.

`setitimer` sets the timer values similarly, in seconds and microseconds (which must be integers). The interval value can be zero to have the timer run down just once. The return value is the timer’s previous setting, in the same form as `getitimer` returns.

(setitimer ITIMER\_REAL
           5 500000     ;; Raise SIGALRM every 5.5 seconds
           2 0)         ;; with the first SIGALRM in 2 seconds

Although the timers are programmed in microseconds, the actual accuracy might not be that high.

Note that `ITIMER_PROF` and `ITIMER_VIRTUAL` are not functional on all platforms and may always error when called. `(provided? 'ITIMER_PROF)` and `(provided? 'ITIMER_VIRTUAL)` can be used to test if the those itimers are supported on the given host. `ITIMER_REAL` is supported on all platforms that support `setitimer`.

* * *

Next: [Pipes](07_02_10_pipes.md#7210-pipes), Previous: [Signals](07_02_08_signals.md#728-signals), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
