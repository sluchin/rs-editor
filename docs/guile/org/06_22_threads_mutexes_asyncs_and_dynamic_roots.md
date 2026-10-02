### 6.22 Threads, Mutexes, Asyncs and Dynamic Roots [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots)

*   [Threads](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads)
*   [Thread-Local Variables](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6222-thread-local-variables)
*   [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts)
*   [Atomics](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6224-atomics)
*   [Mutexes and Condition Variables](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6225-mutexes-and-condition-variables)
*   [Blocking in Guile Mode](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6226-blocking-in-guile-mode)
*   [Futures](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6227-futures)
*   [Parallel forms](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6228-parallel-forms)

* * *

Next: [Thread-Local Variables](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6222-thread-local-variables), Up: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.22.1 Threads [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads)

Guile supports POSIX threads, unless it was configured with `--without-threads` or the host lacks POSIX thread support. When thread support is available, the `threads` feature is provided (see [`provided?`](06_23_configuration_features_and_runtime_options.md#62321-feature-manipulation)).

The procedures below manipulate Guile threads, which are wrappers around the system’s POSIX threads. For application-level parallelism, using higher-level constructs, such as futures, is recommended (see [Futures](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6227-futures)).

To use these facilities, load the `(ice-9 threads)` module.

(use-modules (ice-9 threads))

Scheme Procedure: **all-threads** [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_all\_threads** () [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return a list of all threads.

Scheme Procedure: **current-thread** [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_current\_thread** () [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return the thread that called this function.

Scheme Procedure: **call-with-new-thread** thunk \[handler\] [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Call `thunk` in a new thread and with a new dynamic state, returning the new thread. The procedure thunk is called via `with-continuation-barrier`.

When handler is specified, then thunk is called from within a `catch` with tag `#t` that has handler as its handler. This catch is established inside the continuation barrier.

Once thunk or handler returns, the return value is made the _exit value_ of the thread and the thread is terminated.

C Function: `SCM` **scm\_spawn\_thread** `(scm_t_catch_body body, void *body_data, scm_t_catch_handler handler, void *handler_data)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Call body in a new thread, passing it body\_data, returning the new thread. The function body is called via `scm_c_with_continuation_barrier`.

When handler is non-`NULL`, body is called via `scm_internal_catch` with tag `SCM_BOOL_T` that has handler and handler\_data as the handler and its data. This catch is established inside the continuation barrier.

Once body or handler returns, the return value is made the _exit value_ of the thread and the thread is terminated.

Scheme Procedure: **thread?** obj [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_thread\_p** (obj) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return `#t` ff obj is a thread; otherwise, return `#f`.

Scheme Procedure: **join-thread** thread \[timeout \[timeoutval\]\] [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_join\_thread** (thread) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_join\_thread\_timed** (thread, timeout, timeoutval) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Wait for thread to terminate and return its exit value. Only threads that were created with `call-with-new-thread` or `scm_spawn_thread` can be joinable; attempting to join a foreign thread will raise an error.

When timeout is given, it specifies a point in time where the waiting should be aborted. It can be either an integer as returned by `current-time` or a pair as returned by `gettimeofday`. When the waiting is aborted, timeoutval is returned (if it is specified; `#f` is returned otherwise).

Scheme Procedure: **thread-exited?** thread [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_thread\_exited\_p** (thread) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return `#t` if thread has exited, or `#f` otherwise.

Scheme Procedure: **yield** [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_yield** (thread) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

If one or more threads are waiting to execute, calling yield forces an immediate context switch to one of them. Otherwise, yield has no effect.

Scheme Procedure: **cancel-thread** thread . values [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_cancel\_thread** (thread) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Asynchronously interrupt thread and ask it to terminate. `dynamic-wind` post thunks will run, but throw handlers will not. If thread has already terminated or been signaled to terminate, this function is a no-op. Calling `join-thread` on the thread will return the given values, if the cancel succeeded.

Under the hood, thread cancellation uses `system-async-mark` and `abort-to-prompt`. See [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts) for more on asynchronous interrupts.

macro: **make-thread** proc arg … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Apply proc to arg … in a new thread formed by `call-with-new-thread` using a default error handler that displays the error to the current error port. The arg … expressions are evaluated in the new thread.

macro: **begin-thread** expr1 expr2 … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Evaluate forms expr1 expr2 … in a new thread formed by `call-with-new-thread` using a default error handler that displays the error to the current error port.

One often wants to limit the number of threads running to be proportional to the number of available processors. These interfaces are therefore exported by (ice-9 threads) as well.

Scheme Procedure: **total-processor-count** [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_total\_processor\_count** () [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return the total number of processors of the machine, which is guaranteed to be at least 1. A “processor” here is a thread execution unit, which can be either:

*   an execution core in a (possibly multi-core) chip, in a (possibly multi- chip) module, in a single computer, or
*   a thread execution unit inside a core in the case of _hyper-threaded_ CPUs.

Which of the two definitions is used, is unspecified.

Scheme Procedure: **current-processor-count** [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_current\_processor\_count** () [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Like `total-processor-count`, but return the number of processors available to the current process. See `setaffinity` and `getaffinity` for more information.

* * *

Next: [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts), Previous: [Threads](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads), Up: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.22.2 Thread-Local Variables [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6222-thread-local-variables)

Sometimes you want to establish a variable binding that is only valid for a given thread: a “thread-local variable”.

You would think that fluids or parameters would be Guile’s answer for thread-local variables, since establishing a new fluid binding doesn’t affect bindings in other threads. See [Fluids and Dynamic States](06_11_controlling_the_flow_of_program_execution.md#61111-fluids-and-dynamic-states), or See [Parameters](06_11_controlling_the_flow_of_program_execution.md#61112-parameters). However, new threads inherit the fluid bindings that were in place in their creator threads. In this way, a binding established using a fluid (or a parameter) in a thread can escape to other threads, which might not be what you want. Or, it might escape via explicit reification via `current-dynamic-state`.

Of course, this dynamic scoping might be exactly what you want; that’s why fluids and parameters work this way, and is what you want for many common parameters such as the current input and output ports, the current locale conversion parameters, and the like. Perhaps this is the case for most parameters, even. If your use case for thread-local bindings comes from a desire to isolate a binding from its setting in unrelated threads, then fluids and parameters apply nicely.

On the other hand, if your use case is to prevent concurrent access to a value from multiple threads, then using vanilla fluids or parameters is not appropriate. For this purpose, Guile has _thread-local fluids_. A fluid created with `make-thread-local-fluid` won’t be captured by `current-dynamic-state` and won’t be propagated to new threads.

Scheme Procedure: **make-thread-local-fluid** \[dflt\] [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_make\_thread\_local\_fluid** (dflt) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return a newly created fluid, whose initial value is dflt, or `#f` if dflt is not given. Unlike fluids made with `make-fluid`, thread local fluids are not captured by `make-dynamic-state`. Similarly, a newly spawned child thread does not inherit thread-local fluid values from the parent thread.

Scheme Procedure: **fluid-thread-local?** fluid [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_fluid\_thread\_local\_p** (fluid) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return `#t` if the fluid fluid is thread-local, or `#f` otherwise.

For example:

(define %thread-local (make-thread-local-fluid))

(with-fluids ((%thread-local (compute-data)))
  ... (fluid-ref %thread-local) ...)

You can also make a thread-local parameter out of a thread-local fluid using the normal `fluid->parameter`:

(define param (fluid->parameter (make-thread-local-fluid)))

(parameterize ((param (compute-data)))
  ... (param) ...)

* * *

Next: [Atomics](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6224-atomics), Previous: [Thread-Local Variables](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6222-thread-local-variables), Up: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.22.3 Asynchronous Interrupts [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts)

Every Guile thread can be interrupted. Threads running Guile code will periodically check if there are pending interrupts and run them if necessary. To interrupt a thread, call `system-async-mark` on that thread.

Scheme Procedure: **system-async-mark** proc \[thread\] [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_system\_async\_mark** (proc) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_system\_async\_mark\_for\_thread** (proc, thread) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Enqueue proc (a procedure with zero arguments) for future execution in thread. When proc has already been enqueued for thread but has not been executed yet, this call has no effect. When thread is omitted, the thread that called `system-async-mark` is used.

Note that `scm_system_async_mark_for_thread` is not “async-signal-safe” and so cannot be called from a C signal handler. (Indeed in general, `libguile` functions are not safe to call from C signal handlers.)

Though an interrupt procedure can have any side effect permitted to Guile code, asynchronous interrupts are generally used either for profiling or for prematurely canceling a computation. The former case is mostly transparent to the program being run, by design, but the latter case can introduce bugs. Like finalizers (see [Foreign Object Memory Management](05_programming_in_c.md#554-foreign-object-memory-management)), asynchronous interrupts introduce concurrency in a program. An asynchronous interrupt can run in the middle of some mutex-protected operation, for example, and potentially corrupt the program’s state.

If some bit of Guile code needs to temporarily inhibit interrupts, it can use `call-with-blocked-asyncs`. This function works by temporarily increasing the _async blocking level_ of the current thread while a given procedure is running. The blocking level starts out at zero, and whenever a safe point is reached, a blocking level greater than zero will prevent the execution of queued asyncs.

Analogously, the procedure `call-with-unblocked-asyncs` will temporarily decrease the blocking level of the current thread. You can use it when you want to disable asyncs by default and only allow them temporarily.

In addition to the C versions of `call-with-blocked-asyncs` and `call-with-unblocked-asyncs`, C code can use `scm_dynwind_block_asyncs` and `scm_dynwind_unblock_asyncs` inside a _dynamic context_ (see [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind)) to block or unblock asyncs temporarily.

Scheme Procedure: **call-with-blocked-asyncs** proc [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_call\_with\_blocked\_asyncs** (proc) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Call proc and block the execution of asyncs by one level for the current thread while it is running. Return the value returned by proc. For the first two variants, call proc with no arguments; for the third, call it with data.

C Function: `void *` **scm\_c\_call\_with\_blocked\_asyncs** `(void * (*proc) (void *data), void *data)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

The same but with a C function proc instead of a Scheme thunk.

Scheme Procedure: **call-with-unblocked-asyncs** proc [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_call\_with\_unblocked\_asyncs** (proc) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Call proc and unblock the execution of asyncs by one level for the current thread while it is running. Return the value returned by proc. For the first two variants, call proc with no arguments; for the third, call it with data.

C Function: `void *` **scm\_c\_call\_with\_unblocked\_asyncs** `(void *(*proc) (void *data), void *data)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

The same but with a C function proc instead of a Scheme thunk.

C Function: `void` **scm\_dynwind\_block\_asyncs** `()` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

During the current dynwind context, increase the blocking of asyncs by one level. This function must be used inside a pair of calls to `scm_dynwind_begin` and `scm_dynwind_end` (see [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind)).

C Function: `void` **scm\_dynwind\_unblock\_asyncs** `()` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

During the current dynwind context, decrease the blocking of asyncs by one level. This function must be used inside a pair of calls to `scm_dynwind_begin` and `scm_dynwind_end` (see [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind)).

Sometimes you want to interrupt a thread that might be waiting for something to happen, for example on a file descriptor or a condition variable. In that case you can inform Guile of how to interrupt that wait using the following procedures:

C Function: `int` **scm\_c\_prepare\_to\_wait\_on\_fd** `(int fd)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Inform Guile that the current thread is about to sleep, and that if an asynchronous interrupt is signaled on this thread, Guile should wake up the thread by writing a zero byte to fd. Returns zero if the prepare succeeded, or nonzero if the thread already has a pending async and that it should avoid waiting.

C Function: `int` **scm\_c\_prepare\_to\_wait\_on\_cond** `(scm_i_pthread_mutex_t *mutex, scm_i_pthread_cond_t *cond)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Inform Guile that the current thread is about to sleep, and that if an asynchronous interrupt is signaled on this thread, Guile should wake up the thread by acquiring mutex and signaling cond. The caller must already hold mutex and only drop it as part of the `pthread_cond_wait` call. Returns zero if the prepare succeeded, or nonzero if the thread already has a pending async and that it should avoid waiting.

C Function: `void` **scm\_c\_wait\_finished** `(void)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Inform Guile that the current thread has finished waiting, and that asynchronous interrupts no longer need any special wakeup action; the current thread will periodically poll its internal queue instead.

Guile’s own interface to `sleep`, `wait-condition-variable`, `select`, and so on all call the above routines as appropriate.

Finally, note that threads can also be interrupted via POSIX signals. See [Signals](07_02_08_signals.md#728-signals). As an implementation detail, signal handlers will effectively call `system-async-mark` in a signal-safe way, eventually running the signal handler using the same async mechanism. In this way you can temporarily inhibit signal handlers from running using the above interfaces.

* * *

Next: [Mutexes and Condition Variables](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6225-mutexes-and-condition-variables), Previous: [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts), Up: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.22.4 Atomics [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6224-atomics)

When accessing data in parallel from multiple threads, updates made by one thread are not generally guaranteed to be visible by another thread. It could be that your hardware requires special instructions to be emitted to propagate a change from one CPU core to another. Or, it could be that your hardware updates values with a sequence of instructions, and a parallel thread could see a value that is in the process of being updated but not fully updated.

Atomic references solve this problem. Atomics are a standard, primitive facility to allow for concurrent access and update of mutable variables from multiple threads with guaranteed forward-progress and well-defined intermediate states.

Atomic references serve not only as a hardware memory barrier but also as a compiler barrier. Normally a compiler might choose to reorder or elide certain memory accesses due to optimizations like common subexpression elimination. Atomic accesses however will not be reordered relative to each other, and normal memory accesses will not be reordered across atomic accesses.

As an implementation detail, currently all atomic accesses and updates use the sequential consistency memory model from C11. We may relax this in the future to the acquire/release semantics, which still issues a memory barrier so that non-atomic updates are not reordered across atomic accesses or updates.

To use Guile’s atomic operations, load the `(ice-9 atomic)` module:

(use-modules (ice-9 atomic))

Scheme Procedure: **make-atomic-box** init [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return an atomic box initialized to value init.

Scheme Procedure: **atomic-box?** obj [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return `#t` if obj is an atomic-box object, else return `#f`.

Scheme Procedure: **atomic-box-ref** box [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Fetch the value stored in the atomic box box and return it.

Scheme Procedure: **atomic-box-set!** box val [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Store val into the atomic box box.

Scheme Procedure: **atomic-box-swap!** box val [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Store val into the atomic box box, and return the value that was previously stored in the box.

Scheme Procedure: **atomic-box-compare-and-swap!** box expected desired [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

If the value of the atomic box box is the same as, expected (in the sense of `eq?`), replace the contents of the box with desired. Otherwise does not update the box. Returns the previous value of the box in either case, so you can know if the swap worked by checking if the return value is `eq?` to expected.

* * *

Next: [Blocking in Guile Mode](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6226-blocking-in-guile-mode), Previous: [Atomics](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6224-atomics), Up: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.22.5 Mutexes and Condition Variables [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6225-mutexes-and-condition-variables)

Mutexes are low-level primitives used to coordinate concurrent access to mutable data. Short for “mutual exclusion”, the name “mutex” indicates that only one thread at a time can acquire access to data that is protected by a mutex – threads are excluded from accessing data at the same time. If one thread has locked a mutex, then another thread attempting to lock that same mutex will wait until the first thread is done.

Mutexes can be used to build robust multi-threaded programs that take advantage of multiple cores. However, they provide very low-level functionality and are somewhat dangerous; usually you end up wanting to acquire multiple mutexes at the same time to perform a multi-object access, but this can easily lead to deadlocks if the program is not carefully written. For example, if objects A and B are protected by associated mutexes M and N, respectively, then to access both of them then you need to acquire both mutexes. But what if one thread acquires M first and then N, at the same time that another thread acquires N them M? You can easily end up in a situation where one is waiting for the other.

There’s no easy way around this problem on the language level. A function A that uses mutexes does not necessarily compose nicely with a function B that uses mutexes. For this reason we suggest using atomic variables when you can (see [Atomics](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6224-atomics)), as they do not have this problem.

Still, if you as a programmer are responsible for a whole system, then you can use mutexes as a primitive to provide safe concurrent abstractions to your users. (For example, given all locks in a system, if you establish an order such that M is consistently acquired before N, you can avoid the “deadly-embrace” deadlock described above. The problem is enumerating all mutexes and establishing this order from a system perspective.) Guile gives you the low-level facilities to build such systems.

In Guile there are additional considerations beyond the usual ones in other programming languages: non-local control flow and asynchronous interrupts. What happens if you hold a mutex, but somehow you cause an exception to be thrown? There is no one right answer. You might want to keep the mutex locked to prevent any other code from ever entering that critical section again. Or, your critical section might be fine if you unlock the mutex “on the way out”, via an exception handler or `dynamic-wind`. See [Exceptions](06_11_controlling_the_flow_of_program_execution.md#6118-exceptions), and See [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind).

But if you arrange to unlock the mutex when leaving a dynamic extent via `dynamic-wind`, what to do if control re-enters that dynamic extent via a continuation invocation? Surely re-entering the dynamic extent without the lock is a bad idea, so there are two options on the table: either prevent re-entry via `with-continuation-barrier` or similar, or reacquire the lock in the entry thunk of a `dynamic-wind`.

You might think that because you don’t use continuations, that you don’t have to think about this, and you might be right. If you control the whole system, you can reason about continuation use globally. Or, if you know all code that can be called in a dynamic extent, and none of that code can call continuations, then you don’t have to worry about re-entry, and you might not have to worry about early exit either.

However, do consider the possibility of asynchronous interrupts (see [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts)). If the user interrupts your code interactively, that can cause an exception; or your thread might be canceled, which does the same; or the user could be running your code under some pre-emptive system that periodically causes lightweight task switching. (Guile does not currently include such a system, but it’s possible to implement as a library.) Probably you also want to defer asynchronous interrupt processing while you hold the mutex, and probably that also means that you should not hold the mutex for very long.

All of these additional Guile-specific considerations mean that from a system perspective, you would do well to avoid these hazards if you can by not requiring mutexes. Instead, work with immutable data that can be shared between threads without hazards, or use persistent data structures with atomic updates based on the atomic variable library (see [Atomics](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6224-atomics)).

There are three types of mutexes in Guile: “standard”, “recursive”, and “unowned”.

Calling `make-mutex` with no arguments makes a standard mutex. A standard mutex can only be locked once. If you try to lock it again from the thread that locked it to begin with (the "owner" thread), it throws an error. It can only be unlocked from the thread that locked it in the first place.

Calling `make-mutex` with the symbol `recursive` as the argument, or calling `make-recursive-mutex`, will give you a recursive mutex. A recursive mutex can be locked multiple times by its owner. It then has to be unlocked the corresponding number of times, and like standard mutexes can only be unlocked by the owner thread.

Finally, calling `make-mutex` with the symbol `allow-external-unlock` creates an unowned mutex. An unowned mutex is like a standard mutex, except that it can be unlocked by any thread. A corollary of this behavior is that a thread’s attempt to lock a mutex that it already owns will block instead of signaling an error, as it could be that some other thread unlocks the mutex, allowing the owner thread to proceed. This kind of mutex is a bit strange and is here for use by SRFI-18.

The mutex procedures in Guile can operate on all three kinds of mutexes.

To use these facilities, load the `(ice-9 threads)` module.

(use-modules (ice-9 threads))

  

Scheme Procedure: **make-mutex** \[kind\] [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_make\_mutex** () [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_make\_mutex\_with\_kind** (SCM kind) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return a new mutex. It will be a standard non-recursive mutex, unless the `recursive` symbol is passed as the optional kind argument, in which case it will be recursive. It’s also possible to pass `unowned` for semantics tailored to SRFI-18’s use case; see above for details.

Scheme Procedure: **mutex?** obj [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_mutex\_p** (obj) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return `#t` if obj is a mutex; otherwise, return `#f`.

Scheme Procedure: **make-recursive-mutex** [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_make\_recursive\_mutex** () [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Create a new recursive mutex. It is initially unlocked. Calling this function is equivalent to calling `make-mutex` with the `recursive` kind.

Scheme Procedure: **lock-mutex** mutex \[timeout\] [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_lock\_mutex** (mutex) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_timed\_lock\_mutex** (mutex, timeout) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Lock mutex and return `#t`. If the mutex is already locked, then block and return only when mutex has been acquired.

When timeout is given, it specifies a point in time where the waiting should be aborted. It can be either an integer as returned by `current-time` or a pair as returned by `gettimeofday`. When the waiting is aborted, `#f` is returned.

For standard mutexes (`make-mutex`), an error is signaled if the thread has itself already locked mutex.

For a recursive mutex (`make-recursive-mutex`), if the thread has itself already locked mutex, then a further `lock-mutex` call increments the lock count. An additional `unlock-mutex` will be required to finally release.

When an asynchronous interrupt (see [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts)) is scheduled for a thread blocked in `lock-mutex`, Guile will interrupt the wait, run the interrupts, and then resume the wait.

C Function: `void` **scm\_dynwind\_lock\_mutex** `(SCM mutex)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Arrange for mutex to be locked whenever the current dynwind context is entered and to be unlocked when it is exited.

Scheme Procedure: **try-mutex** mx [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_try\_mutex** (mx) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Try to lock mutex and return `#t` if successful, or `#f` otherwise. This is like calling `lock-mutex` with an expired timeout.

Scheme Procedure: **unlock-mutex** mutex [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_unlock\_mutex** (mutex) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Unlock mutex. An error is signaled if mutex is not locked.

“Standard” and “recursive” mutexes can only be unlocked by the thread that locked them; Guile detects this situation and signals an error. “Unowned” mutexes can be unlocked by any thread.

Scheme Procedure: **mutex-owner** mutex [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_mutex\_owner** (mutex) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return the current owner of mutex, in the form of a thread or `#f` (indicating no owner). Note that a mutex may be unowned but still locked.

Scheme Procedure: **mutex-level** mutex [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_mutex\_level** (mutex) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return the current lock level of mutex. If mutex is currently unlocked, this value will be 0; otherwise, it will be the number of times mutex has been recursively locked by its current owner.

Scheme Procedure: **mutex-locked?** mutex [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_mutex\_locked\_p** (mutex) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return `#t` if mutex is locked, regardless of ownership; otherwise, return `#f`.

Scheme Procedure: **make-condition-variable** [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_make\_condition\_variable** () [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return a new condition variable.

Scheme Procedure: **condition-variable?** obj [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_condition\_variable\_p** (obj) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return `#t` if obj is a condition variable; otherwise, return `#f`.

Scheme Procedure: **wait-condition-variable** condvar mutex \[time\] [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_wait\_condition\_variable** (condvar, mutex, time) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Wait until condvar has been signaled. While waiting, mutex is atomically unlocked (as with `unlock-mutex`) and is locked again when this function returns. When time is given, it specifies a point in time where the waiting should be aborted. It can be either a integer as returned by `current-time` or a pair as returned by `gettimeofday`. When the waiting is aborted, `#f` is returned. When the condition variable has in fact been signaled, `#t` is returned. The mutex is re-locked in any case before `wait-condition-variable` returns.

When an async is activated for a thread that is blocked in a call to `wait-condition-variable`, the waiting is interrupted, the mutex is locked, and the async is executed. When the async returns, the mutex is unlocked again and the waiting is resumed. When the thread block while re-acquiring the mutex, execution of asyncs is blocked.

Scheme Procedure: **signal-condition-variable** condvar [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_signal\_condition\_variable** (condvar) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Wake up one thread that is waiting for condvar.

Scheme Procedure: **broadcast-condition-variable** condvar [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: **scm\_broadcast\_condition\_variable** (condvar) [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Wake up all threads that are waiting for condvar.

Guile also includes some higher-level abstractions for working with mutexes.

macro: **with-mutex** mutex body1 body2 … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Lock mutex, evaluate the body body1 body2 …, then unlock mutex. The return value is that returned by the last body form.

The lock, body and unlock form the branches of a `dynamic-wind` (see [Dynamic Wind](06_11_controlling_the_flow_of_program_execution.md#61110-dynamic-wind)), so mutex is automatically unlocked if an error or new continuation exits the body, and is re-locked if the body is re-entered by a captured continuation.

macro: **monitor** body1 body2 … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Evaluate the body form body1 body2 … with a mutex locked so only one thread can execute that code at any one time. The return value is the return from the last body form.

Each `monitor` form has its own private mutex and the locking and evaluation is as per `with-mutex` above. A standard mutex (`make-mutex`) is used, which means the body must not recursively re-enter the `monitor` form.

The term “monitor” comes from operating system theory, where it means a particular bit of code managing access to some resource and which only ever executes on behalf of one process at any one time.

* * *

Next: [Futures](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6227-futures), Previous: [Mutexes and Condition Variables](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6225-mutexes-and-condition-variables), Up: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.22.6 Blocking in Guile Mode [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6226-blocking-in-guile-mode)

Up to Guile version 1.8, a thread blocked in guile mode would prevent the garbage collector from running. Thus threads had to explicitly leave guile mode with `scm_without_guile ()` before making a potentially blocking call such as a mutex lock, a `select ()` system call, etc. The following functions could be used to temporarily leave guile mode or to perform some common blocking operations in a supported way.

Starting from Guile 2.0, blocked threads no longer hinder garbage collection. Thus, the functions below are not needed anymore. They can still be used to inform the GC that a thread is about to block, giving it a (small) optimization opportunity for “stop the world” garbage collections, should they occur while the thread is blocked.

C Function: `void *` **scm\_without\_guile** `(void *(*func) (void *), void *data)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Leave guile mode, call func on data, enter guile mode and return the result of calling func.

While a thread has left guile mode, it must not call any libguile functions except `scm_with_guile` or `scm_without_guile` and must not use any libguile macros. Also, local variables of type `SCM` that are allocated while not in guile mode are not protected from the garbage collector.

When used from non-guile mode, calling `scm_without_guile` is still allowed: it simply calls func. In that way, you can leave guile mode without having to know whether the current thread is in guile mode or not.

C Function: `int` **scm\_pthread\_mutex\_lock** `(pthread_mutex_t *mutex)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Like `pthread_mutex_lock`, but leaves guile mode while waiting for the mutex.

C Function: `int` **scm\_pthread\_cond\_wait** `(pthread_cond_t *cond, pthread_mutex_t *mutex)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

C Function: `int` **scm\_pthread\_cond\_timedwait** `(pthread_cond_t *cond, pthread_mutex_t *mutex, struct timespec *abstime)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Like `pthread_cond_wait` and `pthread_cond_timedwait`, but leaves guile mode while waiting for the condition variable.

C Function: `int` **scm\_std\_select** `(int nfds, fd_set *readfds, fd_set *writefds, fd_set *exceptfds, struct timeval *timeout)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Like `select` but leaves guile mode while waiting. Also, the delivery of an async causes this function to be interrupted with error code `EINTR`.

C Function: `unsigned int` **scm\_std\_sleep** `(unsigned int seconds)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Like `sleep`, but leaves guile mode while sleeping. Also, the delivery of an async causes this function to be interrupted.

C Function: `unsigned long` **scm\_std\_usleep** `(unsigned long usecs)` [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Like `usleep`, but leaves guile mode while sleeping. Also, the delivery of an async causes this function to be interrupted.

* * *

Next: [Parallel forms](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6228-parallel-forms), Previous: [Blocking in Guile Mode](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6226-blocking-in-guile-mode), Up: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.22.7 Futures [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6227-futures)

The `(ice-9 futures)` module provides _futures_, a construct for fine-grain parallelism. A future is a wrapper around an expression whose computation may occur in parallel with the code of the calling thread, and possibly in parallel with other futures. Like promises, futures are essentially proxies that can be queried to obtain the value of the enclosed expression:

([touch](06_22_threads_mutexes_asyncs_and_dynamic_roots.md) ([future](06_22_threads_mutexes_asyncs_and_dynamic_roots.md) ([+](06_06_02_numerical_data_types.md) 2 3)))
⇒ 5

However, unlike promises, the expression associated with a future may be evaluated on another CPU core, should one be available. This supports _fine-grain parallelism_, because even relatively small computations can be embedded in futures. Consider this sequential code:

(define (find-prime lst1 lst2)
  (or ([find](07_05_03_srfi1_list_library.md) prime? lst1)
      ([find](07_05_03_srfi1_list_library.md) prime? lst2)))

The two arms of `or` are potentially computation-intensive. They are independent of one another, yet, they are evaluated sequentially when the first one returns `#f`. Using futures, one could rewrite it like this:

(define (find-prime lst1 lst2)
  (let ((f ([future](06_22_threads_mutexes_asyncs_and_dynamic_roots.md) ([find](07_05_03_srfi1_list_library.md) prime? lst2))))
    (or ([find](07_05_03_srfi1_list_library.md) prime? lst1)
        ([touch](06_22_threads_mutexes_asyncs_and_dynamic_roots.md) f))))

This preserves the semantics of `find-prime`. On a multi-core machine, though, the computation of `(find prime? lst2)` may be done in parallel with that of the other `find` call, which can reduce the execution time of `find-prime`.

Futures may be nested: a future can itself spawn and then `touch` other futures, leading to a directed acyclic graph of futures. Using this facility, a parallel `map` procedure can be defined along these lines:

([use-modules](06_18_modules.md) (ice-9 futures) (ice-9 [match](07_08_pattern_matching.md)))

(define ([par-map](06_22_threads_mutexes_asyncs_and_dynamic_roots.md) proc lst)
  ([match](07_08_pattern_matching.md) lst
    (()
     '())
    ((head tail [...](06_08_macros.md))
     (let ((tail ([future](06_22_threads_mutexes_asyncs_and_dynamic_roots.md) ([par-map](06_22_threads_mutexes_asyncs_and_dynamic_roots.md) proc tail)))
           (head (proc head)))
       ([cons](06_06_08_pairs.md) head ([touch](06_22_threads_mutexes_asyncs_and_dynamic_roots.md) tail))))))

Note that futures are intended for the evaluation of purely functional expressions. Expressions that have side-effects or rely on I/O may require additional care, such as explicit synchronization (see [Mutexes and Condition Variables](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6225-mutexes-and-condition-variables)).

Guile’s futures are implemented on top of POSIX threads (see [Threads](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6221-threads)). Internally, a fixed-size pool of threads is used to evaluate futures, such that offloading the evaluation of an expression to another thread doesn’t incur thread creation costs. By default, the pool contains one thread per available CPU core, minus one, to account for the main thread. The number of available CPU cores is determined using `current-processor-count` (see [Processes](07_02_07_processes.md#727-processes)).

When a thread touches a future that has not completed yet, it processes any pending future while waiting for it to complete, or just waits if there are no pending futures. When `touch` is called from within a future, the execution of the calling future is suspended, allowing its host thread to process other futures, and resumed when the touched future has completed. This suspend/resume is achieved by capturing the calling future’s continuation, and later reinstating it (see [delimited continuations](06_11_controlling_the_flow_of_program_execution.md#6115-prompts)).

Scheme Syntax: **future** exp [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return a future for expression exp. This is equivalent to:

([make-future](06_22_threads_mutexes_asyncs_and_dynamic_roots.md) (lambda () [exp](06_06_02_numerical_data_types.md)))

Scheme Procedure: **make-future** thunk [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return a future for thunk, a zero-argument procedure.

This procedure returns immediately. Execution of thunk may begin in parallel with the calling thread’s computations, if idle CPU cores are available, or it may start when `touch` is invoked on the returned future.

If the execution of thunk throws an exception, that exception will be re-thrown when `touch` is invoked on the returned future.

Scheme Procedure: **future?** obj [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return `#t` if obj is a future.

Scheme Procedure: **touch** f [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Return the result of the expression embedded in future f.

If the result was already computed in parallel, `touch` returns instantaneously. Otherwise, it waits for the computation to complete, if it already started, or initiates it. In the former case, the calling thread may process other futures in the meantime.

* * *

Previous: [Futures](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6227-futures), Up: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 6.22.8 Parallel forms [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6228-parallel-forms)

The functions described in this section are available from

(use-modules (ice-9 threads))

They provide high-level parallel constructs. The following functions are implemented in terms of futures (see [Futures](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6227-futures)). Thus they are relatively cheap as they re-use existing threads, and portable, since they automatically use one thread per available CPU core.

syntax: **parallel** expr … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Evaluate each expr expression in parallel, each in its own thread. Return the results of n expressions as a set of n multiple values (see [Returning and Accepting Multiple Values](06_11_controlling_the_flow_of_program_execution.md#6117-returning-and-accepting-multiple-values)).

syntax: **letpar** ((var expr) …) body1 body2 … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Evaluate each expr in parallel, each in its own thread, then bind the results to the corresponding var variables, and then evaluate body1 body2 ...

`letpar` is like `let` (see [Local Variable Bindings](06_10_definitions_and_variable_bindings.md#6102-local-variable-bindings)), but all the expressions for the bindings are evaluated in parallel.

Scheme Procedure: **par-map** proc lst1 lst2 … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Scheme Procedure: **par-for-each** proc lst1 lst2 … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Call proc on the elements of the given lists. `par-map` returns a list comprising the return values from proc. `par-for-each` returns an unspecified value, but waits for all calls to complete.

The proc calls are `(proc elem1 elem2 …)`, where each elem is from the corresponding lst . Each lst must be the same length. The calls are potentially made in parallel, depending on the number of CPU cores available.

These functions are like `map` and `for-each` (see [List Mapping](06_06_09_lists.md#6698-list-mapping)), but make their proc calls in parallel.

Unlike those above, the functions described below take a number of threads as an argument. This makes them inherently non-portable since the specified number of threads may differ from the number of available CPU cores as returned by `current-processor-count` (see [Processes](07_02_07_processes.md#727-processes)). In addition, these functions create the specified number of threads when they are called and terminate them upon completion, which makes them quite expensive.

Therefore, they should be avoided.

Scheme Procedure: **n-par-map** n proc lst1 lst2 … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Scheme Procedure: **n-par-for-each** n proc lst1 lst2 … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Call proc on the elements of the given lists, in the same way as `par-map` and `par-for-each` above, but use no more than n threads at any one time. The order in which calls are initiated within that threads limit is unspecified.

These functions are good for controlling resource consumption if proc calls might be costly, or if there are many to be made. On a dual-CPU system for instance _n\=4_ might be enough to keep the CPUs utilized, and not consume too much memory.

Scheme Procedure: **n-for-each-par-map** n sproc pproc lst1 lst2 … [¶](06_22_threads_mutexes_asyncs_and_dynamic_roots.md)

Apply pproc to the elements of the given lists, and apply sproc to each result returned by pproc. The final return value is unspecified, but all calls will have been completed before returning.

The calls made are `(sproc (pproc elem1 … elemN))`, where each elem is from the corresponding lst. Each lst must have the same number of elements.

The pproc calls are made in parallel, in separate threads. No more than n threads are used at any one time. The order in which pproc calls are initiated within that limit is unspecified.

The sproc calls are made serially, in list element order, one at a time. pproc calls on later elements may execute in parallel with the sproc calls. Exactly which thread makes each sproc call is unspecified.

This function is designed for individual calculations that can be done in parallel, but with results needing to be handled serially, for instance to write them to a file. The n limit on threads controls system resource usage when there are many calculations or when they might be costly.

It will be seen that `n-for-each-par-map` is like a combination of `n-par-map` and `for-each`,

(for-each sproc (n-par-map n pproc lst1 ... lstN))

But the actual implementation is more efficient since each sproc call, in turn, can be initiated once the relevant pproc call has completed, it doesn’t need to wait for all to finish.

* * *

Next: [Support for Other Languages](06_24_support_for_other_languages.md#624-support-for-other-languages), Previous: [Threads, Mutexes, Asyncs and Dynamic Roots](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#622-threads-mutexes-asyncs-and-dynamic-roots), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
