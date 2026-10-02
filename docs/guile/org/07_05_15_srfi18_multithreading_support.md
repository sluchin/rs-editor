#### 7.5.15 SRFI-18 - Multithreading support [¶](07_05_15_srfi18_multithreading_support.md#7515-srfi-18---multithreading-support)

This is an implementation of the SRFI-18 threading and synchronization library. The functions and variables described here are provided by

(use-modules (srfi srfi-18))

SRFI-18 defines facilities for threads, mutexes, condition variables, time, and exception handling. Because these facilities are at a higher level than Guile’s primitives, they are implemented as a layer on top of what Guile provides. In particular this means that a Guile mutex is not a SRFI-18 mutex, and a Guile thread is not a SRFI-18 thread, and so on. Guile provides a set of primitives and SRFI-18 is one of the systems built in terms of those primitives.

*   [SRFI-18 Threads](07_05_15_srfi18_multithreading_support.md#75151-srfi-18-threads)
*   [SRFI-18 Mutexes](07_05_15_srfi18_multithreading_support.md#75152-srfi-18-mutexes)
*   [SRFI-18 Condition variables](07_05_15_srfi18_multithreading_support.md#75153-srfi-18-condition-variables)
*   [SRFI-18 Time](07_05_15_srfi18_multithreading_support.md#75154-srfi-18-time)
*   [SRFI-18 Exceptions](07_05_15_srfi18_multithreading_support.md#75155-srfi-18-exceptions)

* * *

Next: [SRFI-18 Mutexes](07_05_15_srfi18_multithreading_support.md#75152-srfi-18-mutexes), Up: [SRFI-18 - Multithreading support](07_05_15_srfi18_multithreading_support.md#7515-srfi-18---multithreading-support)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.15.1 SRFI-18 Threads [¶](07_05_15_srfi18_multithreading_support.md#75151-srfi-18-threads)

Threads created by SRFI-18 differ in two ways from threads created by Guile’s built-in thread functions. First, a thread created by SRFI-18 `make-thread` begins in a blocked state and will not start execution until `thread-start!` is called on it. Second, SRFI-18 threads are constructed with a top-level exception handler that captures any exceptions that are thrown on thread exit.

SRFI-18 threads are disjoint from Guile’s primitive threads. See [Threads](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads), for more on Guile’s primitive facility.

Function: **current-thread** [¶](07_05_15_srfi18_multithreading_support.md)

Returns the thread that called this function. This is the same procedure as the same-named built-in procedure `current-thread` (see [Threads](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads)).

Function: **thread?** obj [¶](07_05_15_srfi18_multithreading_support.md)

Returns `#t` if obj is a thread, `#f` otherwise. This is the same procedure as the same-named built-in procedure `thread?` (see [Threads](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads)).

Function: **make-thread** thunk \[name\] [¶](07_05_15_srfi18_multithreading_support.md)

Call `thunk` in a new thread and with a new dynamic state, returning the new thread and optionally assigning it the object name name, which may be any Scheme object.

Note that the name `make-thread` conflicts with the `(ice-9 threads)` function `make-thread`. Applications wanting to use both of these functions will need to refer to them by different names.

Function: **thread-name** thread [¶](07_05_15_srfi18_multithreading_support.md)

Returns the name assigned to thread at the time of its creation, or `#f` if it was not given a name.

Function: **thread-specific** thread [¶](07_05_15_srfi18_multithreading_support.md)

Function: **thread-specific-set!** thread obj [¶](07_05_15_srfi18_multithreading_support.md)

Get or set the “object-specific” property of thread. In Guile’s implementation of SRFI-18, this value is stored as an object property, and will be `#f` if not set.

Function: **thread-start!** thread [¶](07_05_15_srfi18_multithreading_support.md)

Unblocks thread and allows it to begin execution if it has not done so already.

Function: **thread-yield!** [¶](07_05_15_srfi18_multithreading_support.md)

If one or more threads are waiting to execute, calling `thread-yield!` forces an immediate context switch to one of them. Otherwise, `thread-yield!` has no effect. `thread-yield!` behaves identically to the Guile built-in function `yield`.

Function: **thread-sleep!** timeout [¶](07_05_15_srfi18_multithreading_support.md)

The current thread waits until the point specified by the time object timeout is reached (see [SRFI-18 Time](07_05_15_srfi18_multithreading_support.md#75154-srfi-18-time)). This blocks the thread only if timeout represents a point in the future. it is an error for timeout to be `#f`.

Function: **thread-terminate!** thread [¶](07_05_15_srfi18_multithreading_support.md)

Causes an abnormal termination of thread. If thread is not already terminated, all mutexes owned by thread become unlocked/abandoned. If thread is the current thread, `thread-terminate!` does not return. Otherwise `thread-terminate!` returns an unspecified value; the termination of thread will occur before `thread-terminate!` returns. Subsequent attempts to join on thread will cause a “terminated thread exception” to be raised.

`thread-terminate!` is compatible with the thread cancellation procedures in the core threads API (see [Threads](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads)) in that if a cleanup handler has been installed for the target thread, it will be called before the thread exits and its return value (or exception, if any) will be stored for later retrieval via a call to `thread-join!`.

Function: **thread-join!** thread \[timeout \[timeout-val\]\] [¶](07_05_15_srfi18_multithreading_support.md)

Wait for thread to terminate and return its exit value. When a time value timeout is given, it specifies a point in time where the waiting should be aborted. When the waiting is aborted, timeout-val is returned if it is specified; otherwise, a `join-timeout-exception` exception is raised (see [SRFI-18 Exceptions](07_05_15_srfi18_multithreading_support.md#75155-srfi-18-exceptions)). Exceptions may also be raised if the thread was terminated by a call to `thread-terminate!` (`terminated-thread-exception` will be raised) or if the thread exited by raising an exception that was handled by the top-level exception handler (`uncaught-exception` will be raised; the original exception can be retrieved using `uncaught-exception-reason`).

* * *

Next: [SRFI-18 Condition variables](07_05_15_srfi18_multithreading_support.md#75153-srfi-18-condition-variables), Previous: [SRFI-18 Threads](07_05_15_srfi18_multithreading_support.md#75151-srfi-18-threads), Up: [SRFI-18 - Multithreading support](07_05_15_srfi18_multithreading_support.md#7515-srfi-18---multithreading-support)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.15.2 SRFI-18 Mutexes [¶](07_05_15_srfi18_multithreading_support.md#75152-srfi-18-mutexes)

SRFI-18 mutexes are disjoint from Guile’s primitive mutexes. See [Mutexes and Condition Variables](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6225-mutexes-and-condition-variables), for more on Guile’s primitive facility.

Function: **make-mutex** \[name\] [¶](07_05_15_srfi18_multithreading_support.md)

Returns a new mutex, optionally assigning it the object name name, which may be any Scheme object. The returned mutex will be created with the configuration described above.

Function: **mutex-name** mutex [¶](07_05_15_srfi18_multithreading_support.md)

Returns the name assigned to mutex at the time of its creation, or `#f` if it was not given a name.

Function: **mutex-specific** mutex [¶](07_05_15_srfi18_multithreading_support.md)

Return the “object-specific” property of mutex, or `#f` if none is set.

Function: **mutex-specific-set!** mutex obj [¶](07_05_15_srfi18_multithreading_support.md)

Set the “object-specific” property of mutex.

Function: **mutex-state** mutex [¶](07_05_15_srfi18_multithreading_support.md)

Returns information about the state of mutex. Possible values are:

*   thread t: the mutex is in the locked/owned state and thread t is the owner of the mutex
*   symbol `not-owned`: the mutex is in the locked/not-owned state
*   symbol `abandoned`: the mutex is in the unlocked/abandoned state
*   symbol `not-abandoned`: the mutex is in the unlocked/not-abandoned state

Function: **mutex-lock!** mutex \[timeout \[thread\]\] [¶](07_05_15_srfi18_multithreading_support.md)

Lock mutex, optionally specifying a time object timeout after which to abort the lock attempt and a thread thread giving a new owner for mutex different than the current thread.

Function: **mutex-unlock!** mutex \[condition-variable \[timeout\]\] [¶](07_05_15_srfi18_multithreading_support.md)

Unlock mutex, optionally specifying a condition variable condition-variable on which to wait, either indefinitely or, optionally, until the time object timeout has passed, to be signaled.

* * *

Next: [SRFI-18 Time](07_05_15_srfi18_multithreading_support.md#75154-srfi-18-time), Previous: [SRFI-18 Mutexes](07_05_15_srfi18_multithreading_support.md#75152-srfi-18-mutexes), Up: [SRFI-18 - Multithreading support](07_05_15_srfi18_multithreading_support.md#7515-srfi-18---multithreading-support)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.15.3 SRFI-18 Condition variables [¶](07_05_15_srfi18_multithreading_support.md#75153-srfi-18-condition-variables)

SRFI-18 does not specify a “wait” function for condition variables. Waiting on a condition variable can be simulated using the SRFI-18 `mutex-unlock!` function described in the previous section.

SRFI-18 condition variables are disjoint from Guile’s primitive condition variables. See [Mutexes and Condition Variables](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6225-mutexes-and-condition-variables), for more on Guile’s primitive facility.

Function: **condition-variable?** obj [¶](07_05_15_srfi18_multithreading_support.md)

Returns `#t` if obj is a condition variable, `#f` otherwise.

Function: **make-condition-variable** \[name\] [¶](07_05_15_srfi18_multithreading_support.md)

Returns a new condition variable, optionally assigning it the object name name, which may be any Scheme object.

Function: **condition-variable-name** condition-variable [¶](07_05_15_srfi18_multithreading_support.md)

Returns the name assigned to condition-variable at the time of its creation, or `#f` if it was not given a name.

Function: **condition-variable-specific** condition-variable [¶](07_05_15_srfi18_multithreading_support.md)

Return the “object-specific” property of condition-variable, or `#f` if none is set.

Function: **condition-variable-specific-set!** condition-variable obj [¶](07_05_15_srfi18_multithreading_support.md)

Set the “object-specific” property of condition-variable.

Function: **condition-variable-signal!** condition-variable [¶](07_05_15_srfi18_multithreading_support.md)

Function: **condition-variable-broadcast!** condition-variable [¶](07_05_15_srfi18_multithreading_support.md)

Wake up one thread that is waiting for condition-variable, in the case of `condition-variable-signal!`, or all threads waiting for it, in the case of `condition-variable-broadcast!`.

* * *

Next: [SRFI-18 Exceptions](07_05_15_srfi18_multithreading_support.md#75155-srfi-18-exceptions), Previous: [SRFI-18 Condition variables](07_05_15_srfi18_multithreading_support.md#75153-srfi-18-condition-variables), Up: [SRFI-18 - Multithreading support](07_05_15_srfi18_multithreading_support.md#7515-srfi-18---multithreading-support)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.15.4 SRFI-18 Time [¶](07_05_15_srfi18_multithreading_support.md#75154-srfi-18-time)

The SRFI-18 time functions manipulate time in two formats: a “time object” type that represents an absolute point in time in some implementation-specific way; and the number of seconds since some unspecified “epoch”. In Guile’s implementation, the epoch is the Unix epoch, 00:00:00 UTC, January 1, 1970.

Function: **current-time** [¶](07_05_15_srfi18_multithreading_support.md)

Return the current time as a time object. This procedure replaces the procedure of the same name in the core library, which returns the current time in seconds since the epoch.

Function: **time?** obj [¶](07_05_15_srfi18_multithreading_support.md)

Returns `#t` if obj is a time object, `#f` otherwise.

Function: **time->seconds** time [¶](07_05_15_srfi18_multithreading_support.md)

Function: **seconds->time** seconds [¶](07_05_15_srfi18_multithreading_support.md)

Convert between time objects and numerical values representing the number of seconds since the epoch. When converting from a time object to seconds, the return value is the number of seconds between time and the epoch. When converting from seconds to a time object, the return value is a time object that represents a time seconds seconds after the epoch.

* * *

Previous: [SRFI-18 Time](07_05_15_srfi18_multithreading_support.md#75154-srfi-18-time), Up: [SRFI-18 - Multithreading support](07_05_15_srfi18_multithreading_support.md#7515-srfi-18---multithreading-support)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.15.5 SRFI-18 Exceptions [¶](07_05_15_srfi18_multithreading_support.md#75155-srfi-18-exceptions)

SRFI-18 exceptions are identical to the exceptions provided by Guile’s implementation of SRFI-34. The behavior of exception handlers invoked to handle exceptions thrown from SRFI-18 functions, however, differs from the conventional behavior of SRFI-34 in that the continuation of the handler is the same as that of the call to the function. Handlers are called in a tail-recursive manner; the exceptions do not “bubble up”.

Function: **current-exception-handler** [¶](07_05_15_srfi18_multithreading_support.md)

Returns the current exception handler.

Function: **with-exception-handler** handler thunk [¶](07_05_15_srfi18_multithreading_support.md)

Installs handler as the current exception handler and calls the procedure thunk with no arguments, returning its value as the value of the exception. handler must be a procedure that accepts a single argument. The current exception handler at the time this procedure is called will be restored after the call returns.

Function: **raise** obj [¶](07_05_15_srfi18_multithreading_support.md)

Raise obj as an exception. This is the same procedure as the same-named procedure defined in SRFI 34.

Function: **join-timeout-exception?** obj [¶](07_05_15_srfi18_multithreading_support.md)

Returns `#t` if obj is an exception raised as the result of performing a timed join on a thread that does not exit within the specified timeout, `#f` otherwise.

Function: **abandoned-mutex-exception?** obj [¶](07_05_15_srfi18_multithreading_support.md)

Returns `#t` if obj is an exception raised as the result of attempting to lock a mutex that has been abandoned by its owner thread, `#f` otherwise.

Function: **terminated-thread-exception?** obj [¶](07_05_15_srfi18_multithreading_support.md)

Returns `#t` if obj is an exception raised as the result of joining on a thread that exited as the result of a call to `thread-terminate!`.

Function: **uncaught-exception?** obj [¶](07_05_15_srfi18_multithreading_support.md)

Function: **uncaught-exception-reason** exc [¶](07_05_15_srfi18_multithreading_support.md)

`uncaught-exception?` returns `#t` if obj is an exception thrown as the result of joining a thread that exited by raising an exception that was handled by the top-level exception handler installed by `make-thread`. When this occurs, the original exception is preserved as part of the exception thrown by `thread-join!` and can be accessed by calling `uncaught-exception-reason` on that exception. Note that because this exception-preservation mechanism is a side-effect of `make-thread`, joining on threads that exited as described above but were created by other means will not raise this `uncaught-exception` error.

* * *

Next: [SRFI-23 - Error Reporting](07_05_17_srfi23_error_reporting.md#7517-srfi-23---error-reporting), Previous: [SRFI-18 - Multithreading support](07_05_15_srfi18_multithreading_support.md#7515-srfi-18---multithreading-support), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
