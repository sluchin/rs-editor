#### 7.2.14 Encryption [¶](07_02_14_encryption.md#7214-encryption)

Please note that the procedures in this section are not suited for strong encryption, they are only interfaces to the well-known and common system library functions of the same name. They are just as good (or bad) as the underlying functions, so you should refer to your system documentation before using them (see [Encrypting Passwords](https://doc.guix.gnu.org/libc/latest/en/libc.html#crypt) in The GNU C Library Reference Manual).

Scheme Procedure: **crypt** key salt [¶](07_02_14_encryption.md)

C Function: **scm\_crypt** (key, salt) [¶](07_02_14_encryption.md)

Encrypt key, with the addition of salt (both strings), using the `crypt` C library call.

Although `getpass` is not an encryption procedure per se, it appears here because it is often used in combination with `crypt`:

Scheme Procedure: **getpass** prompt [¶](07_02_14_encryption.md)

C Function: **scm\_getpass** (prompt) [¶](07_02_14_encryption.md)

Display prompt to the standard error output and read a password from /dev/tty. If this file is not accessible, it reads from standard input. The password may be up to 127 characters in length. Additional characters and the terminating newline character are discarded. While reading the password, echoing and the generation of signals by special characters is disabled.

* * *

Next: [The (ice-9 getopt-long) Module](07_04_the_ice9_getoptlong_module.md#74-the-ice-9-getopt-long-module), Previous: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
