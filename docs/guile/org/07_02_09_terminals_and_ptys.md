#### 7.2.9 Terminals and Ptys [¶](07_02_09_terminals_and_ptys.md#729-terminals-and-ptys)

Scheme Procedure: **isatty?** port [¶](07_02_09_terminals_and_ptys.md)

C Function: **scm\_isatty\_p** (port) [¶](07_02_09_terminals_and_ptys.md)

Return `#t` if port is using a serial non–file device, otherwise `#f`.

Scheme Procedure: **ttyname** port [¶](07_02_09_terminals_and_ptys.md)

C Function: **scm\_ttyname** (port) [¶](07_02_09_terminals_and_ptys.md)

Return a string with the name of the serial terminal device underlying port.

Scheme Procedure: **ctermid** [¶](07_02_09_terminals_and_ptys.md)

C Function: **scm\_ctermid** () [¶](07_02_09_terminals_and_ptys.md)

Return a string containing the file name of the controlling terminal for the current process.

Scheme Procedure: **tcgetpgrp** port [¶](07_02_09_terminals_and_ptys.md)

C Function: **scm\_tcgetpgrp** (port) [¶](07_02_09_terminals_and_ptys.md)

Return the process group ID of the foreground process group associated with the terminal open on the file descriptor underlying port.

If there is no foreground process group, the return value is a number greater than 1 that does not match the process group ID of any existing process group. This can happen if all of the processes in the job that was formerly the foreground job have terminated, and no other job has yet been moved into the foreground.

Scheme Procedure: **tcsetpgrp** port pgid [¶](07_02_09_terminals_and_ptys.md)

C Function: **scm\_tcsetpgrp** (port, pgid) [¶](07_02_09_terminals_and_ptys.md)

Set the foreground process group ID for the terminal used by the file descriptor underlying port to the integer pgid. The calling process must be a member of the same session as pgid and must have the same controlling terminal. The return value is unspecified.

* * *

Next: [Networking](07_02_11_networking.md#7211-networking), Previous: [Terminals and Ptys](07_02_09_terminals_and_ptys.md#729-terminals-and-ptys), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
