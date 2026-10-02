#### 7.2.1 POSIX Interface Conventions [¶](07_02_01_posix_interface_conventions.md#721-posix-interface-conventions)

These interfaces provide access to operating system facilities. They provide a simple wrapping around the underlying C interfaces to make usage from Scheme more convenient. They are also used to implement the Guile port of scsh (see [The Scheme shell (scsh)](07_18_the_scheme_shell_scsh.md#718-the-scheme-shell-scsh)).

Generally there is a single procedure for each corresponding Unix facility. There are some exceptions, such as procedures implemented for speed and convenience in Scheme with no primitive Unix equivalent, e.g. `copy-file`.

The interfaces are intended as far as possible to be portable across different versions of Unix. In some cases procedures which can’t be implemented on particular systems may become no-ops, or perform limited actions. In other cases they may throw errors.

General naming conventions are as follows:

*   The Scheme name is often identical to the name of the underlying Unix facility.
*   Underscores in Unix procedure names are converted to hyphens.
*   Procedures which destructively modify Scheme data have exclamation marks appended, e.g., `recv!`.
*   Predicates (returning only `#t` or `#f`) have question marks appended, e.g., `access?`.
*   Some names are changed to avoid conflict with dissimilar interfaces defined by scsh, e.g., `primitive-fork`.
*   Unix preprocessor names such as `EPERM` or `R_OK` are converted to Scheme variables of the same name (underscores are not replaced with hyphens).

Unexpected conditions are generally handled by raising exceptions. There are a few procedures which return a special value if they don’t succeed, e.g., `getenv` returns `#f` if it the requested string is not found in the environment. These cases are noted in the documentation.

For ways to deal with exceptions, see [Exceptions](06_11_controlling_the_flow_of_program_execution.md#6118-exceptions).

Errors which the C library would report by returning a null pointer or through some other means are reported by raising a `system-error` exception with `scm-error` (see [Procedures for Signaling Errors](06_11_controlling_the_flow_of_program_execution.md#6119-procedures-for-signaling-errors)). The data parameter is a list containing the Unix `errno` value (an integer). For example,

(define (my-handler key func fmt fmtargs data)
  (display key) (newline)
  (display func) (newline)
  (apply format #t fmt fmtargs) (newline)
  (display data) (newline))

(catch 'system-error
  (lambda () (dup2 -123 -456))
  my-handler)

⊣
system-error
dup2
Bad file descriptor
(9)

  

Function: **system-error-errno** arglist [¶](07_02_01_posix_interface_conventions.md)

Return the `errno` value from a list which is the arguments to an exception handler. If the exception is not a `system-error`, then the return is `#f`. For example,

(catch
 'system-error
 (lambda ()
   (mkdir "/this-ought-to-fail-if-I'm-not-root"))
 (lambda stuff
   (let ((errno (system-error-errno stuff)))
     (cond
      ((= errno EACCES)
       (display "You're not allowed to do that."))
      ((= errno EEXIST)
       (display "Already exists."))
      (#t
       (display (strerror errno))))
     (newline))))

* * *

Next: [File System](07_02_03_file_system.md#723-file-system), Previous: [POSIX Interface Conventions](07_02_01_posix_interface_conventions.md#721-posix-interface-conventions), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
