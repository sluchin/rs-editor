### 6.4 Initializing Guile [¶](06_04_initializing_guile.md#64-initializing-guile)

Each thread that wants to use functions from the Guile API needs to put itself into guile mode with either `scm_with_guile` or `scm_init_guile`. The global state of Guile is initialized automatically when the first thread enters guile mode.

When a thread wants to block outside of a Guile API function, it should leave guile mode temporarily with `scm_without_guile`, See [Blocking in Guile Mode](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6226-blocking-in-guile-mode).

Threads that are created by `call-with-new-thread` or `scm_spawn_thread` start out in guile mode so you don’t need to initialize them.

C Function: `void *` **scm\_with\_guile** `(void *(*func)(void *), void *data)` [¶](06_04_initializing_guile.md)

Call func, passing it data and return what func returns. While func is running, the current thread is in guile mode and can thus use the Guile API.

When `scm_with_guile` is called from guile mode, the thread remains in guile mode when `scm_with_guile` returns.

Otherwise, it puts the current thread into guile mode and, if needed, gives it a Scheme representation that is contained in the list returned by `all-threads`, for example. This Scheme representation is not removed when `scm_with_guile` returns so that a given thread is always represented by the same Scheme value during its lifetime, if at all.

When this is the first thread that enters guile mode, the global state of Guile is initialized before calling `func`.

The function func is called via `scm_with_continuation_barrier`; thus, `scm_with_guile` returns exactly once.

When `scm_with_guile` returns, the thread is no longer in guile mode (except when `scm_with_guile` was called from guile mode, see above). Thus, only `func` can store `SCM` variables on the stack and be sure that they are protected from the garbage collector. See `scm_init_guile` for another approach at initializing Guile that does not have this restriction.

It is OK to call `scm_with_guile` while a thread has temporarily left guile mode via `scm_without_guile`. It will then simply temporarily enter guile mode again.

C Function: `void` **scm\_init\_guile** `()` [¶](06_04_initializing_guile.md)

Arrange things so that all of the code in the current thread executes as if from within a call to `scm_with_guile`. That is, all functions called by the current thread can assume that `SCM` values on their stack frames are protected from the garbage collector (except when the thread has explicitly left guile mode, of course).

When `scm_init_guile` is called from a thread that already has been in guile mode once, nothing happens. This behavior matters when you call `scm_init_guile` while the thread has only temporarily left guile mode: in that case the thread will not be in guile mode after `scm_init_guile` returns. Thus, you should not use `scm_init_guile` in such a scenario.

When a uncaught throw happens in a thread that has been put into guile mode via `scm_init_guile`, a short message is printed to the current error port and the thread is exited via `scm_pthread_exit (NULL)`. No restrictions are placed on continuations.

The function `scm_init_guile` might not be available on all platforms since it requires some stack-bounds-finding magic that might not have been ported to all platforms that Guile runs on. Thus, if you can, it is better to use `scm_with_guile` or its variation `scm_boot_guile` instead of this function.

C Function: `void` **scm\_boot\_guile** `(int argc, char **argv, void (*main_func) (void *data, int argc, char **argv), void *data)` [¶](06_04_initializing_guile.md)

Enter guile mode as with `scm_with_guile` and call main\_func, passing it data, argc, and argv as indicated. When main\_func returns, `scm_boot_guile` calls `exit (0)`; `scm_boot_guile` never returns. If you want some other exit value, have main\_func call `exit` itself. If you don’t want to exit at all, use `scm_with_guile` instead of `scm_boot_guile`.

The function `scm_boot_guile` arranges for the Scheme `command-line` function to return the strings given by argc and argv. If main\_func modifies argc or argv, it should call `scm_set_program_arguments` with the final list, so Scheme code will know which arguments have been processed (see [Runtime Environment](07_02_06_runtime_environment.md#726-runtime-environment)).

C Function: `void` **scm\_shell** `(int argc, char **argv)` [¶](06_04_initializing_guile.md)

Process command-line arguments in the manner of the `guile` executable. This includes loading the normal Guile initialization files, interacting with the user or running any scripts or expressions specified by `-s` or `-e` options, and then exiting. See [Invoking Guile](04_programming_in_scheme.md#42-invoking-guile), for more details.

Since this function does not return, you must do all application-specific initialization before calling this function.

* * *

Next: [Data Types](06_06_00_data_types.md#66-data-types), Previous: [Initializing Guile](06_04_initializing_guile.md#64-initializing-guile), Up: [API Reference](06_00_api_reference.md#6-api-reference)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
