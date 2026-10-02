#### 7.2.2 Ports and File Descriptors [¶](07_02_02_ports_and_file_descriptors.md#722-ports-and-file-descriptors)

Conventions generally follow those of scsh, [The Scheme shell (scsh)](07_18_the_scheme_shell_scsh.md#718-the-scheme-shell-scsh).

Each open file port has an associated operating system file descriptor. File descriptors are generally not useful in Scheme programs; however they may be needed when interfacing with foreign code and the Unix environment.

A file descriptor can be extracted from a port and a new port can be created from a file descriptor. However a file descriptor is just an integer and the garbage collector doesn’t recognize it as a reference to the port. If all other references to the port were dropped, then it’s likely that the garbage collector would free the port, with the side-effect of closing the file descriptor prematurely.

To assist the programmer in avoiding this problem, each port has an associated _revealed count_ which can be used to keep track of how many times the underlying file descriptor has been stored in other places. If a port’s revealed count is greater than zero, the file descriptor will not be closed when the port is garbage collected. A programmer can therefore ensure that the revealed count will be greater than zero if the file descriptor is needed elsewhere.

For the simple case where a file descriptor is “imported” once to become a port, it does not matter if the file descriptor is closed when the port is garbage collected. There is no need to maintain a revealed count. Likewise when “exporting” a file descriptor to the external environment, setting the revealed count is not required provided the port is kept open (i.e., is pointed to by a live Scheme binding) while the file descriptor is in use.

To correspond with traditional Unix behavior, three file descriptors (0, 1, and 2) are automatically imported when a program starts up and assigned to the initial values of the current/standard input, output, and error ports, respectively. The revealed count for each is initially set to one, so that dropping references to one of these ports will not result in its garbage collection: it could be retrieved with `fdopen` or `fdes->ports`.

Guile’s ports can be buffered. This means that writing a byte to a file port goes to the internal buffer first, and only when the buffer is full (or the user invokes `force-output` on the port) is the data actually written to the file descriptor. Likewise on input, bytes are read in from the file descriptor in blocks and placed in a buffer. Reading a character via `read-char` first goes to the buffer, filling it as needed. Usually read buffering is more or less transparent, but write buffering can sometimes cause writes to be delayed unexpectedly, if you forget to call `force-output`. See [Buffering](06_12_input_and_output.md#6126-buffering), for more on how to control port buffers.

Note however that some procedures (e.g., `recv!`) will accept ports as arguments, but will actually operate directly on the file descriptor underlying the port. Any port buffering is ignored, including the buffer which implements `peek-char` and `unread-char`.

Scheme Procedure: **port-revealed** port [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_port\_revealed** (port) [¶](07_02_02_ports_and_file_descriptors.md)

Return the revealed count for port.

Scheme Procedure: **set-port-revealed!** port rcount [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_set\_port\_revealed\_x** (port, rcount) [¶](07_02_02_ports_and_file_descriptors.md)

Sets the revealed count for a port to rcount. The return value is unspecified.

Scheme Procedure: **fileno** port [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_fileno** (port) [¶](07_02_02_ports_and_file_descriptors.md)

Return the integer file descriptor underlying port. Does not change its revealed count.

Scheme Procedure: **port->fdes** port [¶](07_02_02_ports_and_file_descriptors.md)

Returns the integer file descriptor underlying port. As a side effect the revealed count of port is incremented.

Scheme Procedure: **fdopen** fdes modes [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_fdopen** (fdes, modes) [¶](07_02_02_ports_and_file_descriptors.md)

Return a new port based on the file descriptor fdes. Modes are given by the string modes. The revealed count of the port is initialized to zero. The modes string is the same as that accepted by `open-file` (see [open-file](06_12_input_and_output.md#612101-file-ports)).

Scheme Procedure: **fdes->ports** fdes [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_fdes\_to\_ports** (fdes) [¶](07_02_02_ports_and_file_descriptors.md)

Return a list of existing ports which have fdes as an underlying file descriptor, without changing their revealed counts.

Scheme Procedure: **fdes->inport** fdes [¶](07_02_02_ports_and_file_descriptors.md)

Returns an existing input port which has fdes as its underlying file descriptor, if one exists, and increments its revealed count. Otherwise, returns a new input port with a revealed count of 1.

Scheme Procedure: **fdes->outport** fdes [¶](07_02_02_ports_and_file_descriptors.md)

Returns an existing output port which has fdes as its underlying file descriptor, if one exists, and increments its revealed count. Otherwise, returns a new output port with a revealed count of 1.

Scheme Procedure: **primitive-move->fdes** port fdes [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_primitive\_move\_to\_fdes** (port, fdes) [¶](07_02_02_ports_and_file_descriptors.md)

Moves the underlying file descriptor for port to the integer value fdes without changing the revealed count of port. Any other ports already using this descriptor will be automatically shifted to new descriptors and their revealed counts reset to zero. The return value is `#f` if the file descriptor already had the required value or `#t` if it was moved.

Scheme Procedure: **move->fdes** port fdes [¶](07_02_02_ports_and_file_descriptors.md)

Moves the underlying file descriptor for port to the integer value fdes and sets its revealed count to one. Any other ports already using this descriptor will be automatically shifted to new descriptors and their revealed counts reset to zero. The return value is unspecified.

Scheme Procedure: **release-port-handle** port [¶](07_02_02_ports_and_file_descriptors.md)

Decrements the revealed count for a port.

Scheme Procedure: **fsync** port\_or\_fd [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_fsync** (port\_or\_fd) [¶](07_02_02_ports_and_file_descriptors.md)

Copies any unwritten data for the specified output file descriptor to disk. If port\_or\_fd is a port, its buffer is flushed before the underlying file descriptor is fsync’d. The return value is unspecified.

Scheme Procedure: **open** path flags \[mode\] [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_open** (path, flags, mode) [¶](07_02_02_ports_and_file_descriptors.md)

Open the file named by path for reading and/or writing. flags is an integer specifying how the file should be opened. mode is an integer specifying the permission bits of the file, if it needs to be created, before the umask (see [Processes](07_02_07_processes.md#727-processes)) is applied. The default is 666 (Unix itself has no default).

flags can be constructed by combining variables using `logior`. Basic flags are:

Variable: **O\_RDONLY** [¶](07_02_02_ports_and_file_descriptors.md)

Open the file read-only.

Variable: **O\_WRONLY** [¶](07_02_02_ports_and_file_descriptors.md)

Open the file write-only.

Variable: **O\_RDWR** [¶](07_02_02_ports_and_file_descriptors.md)

Open the file read/write.

Variable: **O\_APPEND** [¶](07_02_02_ports_and_file_descriptors.md)

Append to the file instead of truncating.

Variable: **O\_CREAT** [¶](07_02_02_ports_and_file_descriptors.md)

Create the file if it does not already exist.

See [File Status Flags](https://doc.guix.gnu.org/libc/latest/en/libc.html#File-Status-Flags) in The GNU C Library Reference Manual, for additional flags.

Scheme Procedure: **openat** dir path flags \[mode\] [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_openat** (dir, path, flags, mode) [¶](07_02_02_ports_and_file_descriptors.md)

Similar to `open`, but resolve the file name path relative to the directory referred to by the file port dir instead.

Scheme Procedure: **open-fdes** path flags \[mode\] [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_open\_fdes** (path, flags, mode) [¶](07_02_02_ports_and_file_descriptors.md)

Similar to `open` but return a file descriptor instead of a port.

Scheme Procedure: **open-fdes-at** dir path flags \[mode\] [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_open\_fdes\_at** (dir, path, flags, mode) [¶](07_02_02_ports_and_file_descriptors.md)

Similar to `openat`, but return a file descriptor instead of a port.

Scheme Procedure: **close** fd\_or\_port [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_close** (fd\_or\_port) [¶](07_02_02_ports_and_file_descriptors.md)

Similar to `close-port` (see [close-port](06_12_input_and_output.md#6121-ports)), but also works on file descriptors. A side effect of closing a file descriptor is that any ports using that file descriptor are moved to a different file descriptor and have their revealed counts set to zero.

Scheme Procedure: **close-fdes** fd [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_close\_fdes** (fd) [¶](07_02_02_ports_and_file_descriptors.md)

A simple wrapper for the `close` system call. Close file descriptor fd, which must be an integer. Unlike `close`, the file descriptor will be closed even if a port is using it. The return value is unspecified.

Scheme Procedure: **pipe** \[flags\] [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_pipe** () [¶](07_02_02_ports_and_file_descriptors.md)

Return a newly created pipe: a pair of ports which are linked together on the local machine. The CAR is the input port and the CDR is the output port. Data written (and flushed) to the output port can be read from the input port. Pipes are commonly used for communication with a newly forked child process. The need to flush the output port can be avoided by making it unbuffered using `setvbuf` (see [Buffering](06_12_input_and_output.md#6126-buffering)).

Optionally, on systems that support it such as GNU/Linux and GNU/Hurd, flags can specify a bitwise-or of the following constants:

`O_CLOEXEC`

Mark the returned file descriptors as close-on-exec;

`O_DIRECT`

Create a pipe that performs input/output in “packet" mode—see `man 2 pipe` for details;

`O_NONBLOCK`

Set the `O_NONBLOCK` status flag (non-blocking input and output) on the file descriptors.

On systems that do _not_ support it, passing a non-zero flags value triggers a `system-error` exception.

Variable: **PIPE\_BUF** [¶](07_02_02_ports_and_file_descriptors.md)

A write of up to `PIPE_BUF` many bytes to a pipe is atomic, meaning when done it goes into the pipe instantaneously and as a contiguous block (see [Atomicity of Pipe I/O](https://doc.guix.gnu.org/libc/latest/en/libc.html#Pipe-Atomicity) in The GNU C Library Reference Manual).

Note that the output port is likely to block if too much data has been written but not yet read from the input port. Typically the capacity is `PIPE_BUF` bytes.

The next group of procedures perform a `dup2` system call, if newfd (an integer) is supplied, otherwise a `dup`. The file descriptor to be duplicated can be supplied as an integer or contained in a port. The type of value returned varies depending on which procedure is used.

All procedures also have the side effect when performing `dup2` that any ports using newfd are moved to a different file descriptor and have their revealed counts set to zero.

Scheme Procedure: **dup->fdes** fd\_or\_port \[fd\] [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_dup\_to\_fdes** (fd\_or\_port, fd) [¶](07_02_02_ports_and_file_descriptors.md)

Return a new integer file descriptor referring to the open file designated by fd\_or\_port, which must be either an open file port or a file descriptor.

Scheme Procedure: **dup->inport** port/fd \[newfd\] [¶](07_02_02_ports_and_file_descriptors.md)

Returns a new input port using the new file descriptor.

Scheme Procedure: **dup->outport** port/fd \[newfd\] [¶](07_02_02_ports_and_file_descriptors.md)

Returns a new output port using the new file descriptor.

Scheme Procedure: **dup** port/fd \[newfd\] [¶](07_02_02_ports_and_file_descriptors.md)

Returns a new port if port/fd is a port, with the same mode as the supplied port, otherwise returns an integer file descriptor.

Scheme Procedure: **dup->port** port/fd mode \[newfd\] [¶](07_02_02_ports_and_file_descriptors.md)

Returns a new port using the new file descriptor. mode supplies a mode string for the port (see [open-file](06_12_input_and_output.md#612101-file-ports)).

Scheme Procedure: **duplicate-port** port modes [¶](07_02_02_ports_and_file_descriptors.md)

Returns a new port which is opened on a duplicate of the file descriptor underlying port, with mode string modes as for [open-file](06_12_input_and_output.md#612101-file-ports). The two ports will share a file position and file status flags.

Unexpected behavior can result if both ports are subsequently used and the original and/or duplicate ports are buffered. The mode string can include `0` to obtain an unbuffered duplicate port.

This procedure is equivalent to `(dup->port port modes)`.

Scheme Procedure: **redirect-port** old\_port new\_port [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_redirect\_port** (old\_port, new\_port) [¶](07_02_02_ports_and_file_descriptors.md)

This procedure takes two ports and duplicates the underlying file descriptor from old\_port into new\_port. The current file descriptor in new\_port will be closed. After the redirection the two ports will share a file position and file status flags.

The return value is unspecified.

Unexpected behavior can result if both ports are subsequently used and the original and/or duplicate ports are buffered.

This procedure does not have any side effects on other ports or revealed counts.

Scheme Procedure: **dup2** oldfd newfd [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_dup2** (oldfd, newfd) [¶](07_02_02_ports_and_file_descriptors.md)

A simple wrapper for the `dup2` system call. Copies the file descriptor oldfd to descriptor number newfd, replacing the previous meaning of newfd. Both oldfd and newfd must be integers. Unlike for `dup->fdes` or `primitive-move->fdes`, no attempt is made to move away ports which are using newfd. The return value is unspecified.

Scheme Procedure: **port-for-each** proc [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_port\_for\_each** (SCM proc) [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_c\_port\_for\_each** (void (\*proc)(void \*, SCM), void \*data) [¶](07_02_02_ports_and_file_descriptors.md)

Apply proc to each port in the Guile port table (FIXME: what is the Guile port table?) in turn. The return value is unspecified. More specifically, proc is applied exactly once to every port that exists in the system at the time `port-for-each` is invoked. Changes to the port table while `port-for-each` is running have no effect as far as `port-for-each` is concerned.

The C function `scm_port_for_each` takes a Scheme procedure encoded as a `SCM` value, while `scm_c_port_for_each` takes a pointer to a C function and passes along a arbitrary data cookie.

Scheme Procedure: **fcntl** port/fd cmd \[value\] [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_fcntl** (object, cmd, value) [¶](07_02_02_ports_and_file_descriptors.md)

Apply cmd on port/fd, either a port or file descriptor. The value argument is used by the `SET` commands described below, it’s an integer value.

Values for cmd are:

Variable: **F\_DUPFD** [¶](07_02_02_ports_and_file_descriptors.md)

Duplicate the file descriptor, the same as `dup->fdes` above does.

Variable: **F\_GETFD** [¶](07_02_02_ports_and_file_descriptors.md)

Variable: **F\_SETFD** [¶](07_02_02_ports_and_file_descriptors.md)

Get or set flags associated with the file descriptor. The only flag is the following,

Variable: **FD\_CLOEXEC** [¶](07_02_02_ports_and_file_descriptors.md)

“Close on exec”, meaning the file descriptor will be closed on an `exec` call (a successful such call). For example to set that flag,

(fcntl port F\_SETFD FD\_CLOEXEC)

Or better, set it but leave any other possible future flags unchanged,

(fcntl port F\_SETFD (logior FD\_CLOEXEC
                            (fcntl port F\_GETFD)))

Variable: **F\_GETFL** [¶](07_02_02_ports_and_file_descriptors.md)

Variable: **F\_SETFL** [¶](07_02_02_ports_and_file_descriptors.md)

Get or set flags associated with the open file. These flags are `O_RDONLY` etc described under `open` above.

A common use is to set `O_NONBLOCK` on a network socket. The following sets that flag, and leaves other flags unchanged.

(fcntl sock F\_SETFL (logior O\_NONBLOCK
                            (fcntl sock F\_GETFL)))

Variable: **F\_GETOWN** [¶](07_02_02_ports_and_file_descriptors.md)

Variable: **F\_SETOWN** [¶](07_02_02_ports_and_file_descriptors.md)

Get or set the process ID of a socket’s owner, for `SIGIO` signals.

Scheme Procedure: **flock** file operation [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_flock** (file, operation) [¶](07_02_02_ports_and_file_descriptors.md)

Apply or remove an advisory lock on an open file. operation specifies the action to be done:

Variable: **LOCK\_SH** [¶](07_02_02_ports_and_file_descriptors.md)

Shared lock. More than one process may hold a shared lock for a given file at a given time.

Variable: **LOCK\_EX** [¶](07_02_02_ports_and_file_descriptors.md)

Exclusive lock. Only one process may hold an exclusive lock for a given file at a given time.

Variable: **LOCK\_UN** [¶](07_02_02_ports_and_file_descriptors.md)

Unlock the file.

Variable: **LOCK\_NB** [¶](07_02_02_ports_and_file_descriptors.md)

Don’t block when locking. This is combined with one of the other operations using `logior` (see [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations)). If `flock` would block an `EWOULDBLOCK` error is thrown (see [POSIX Interface Conventions](07_02_01_posix_interface_conventions.md#721-posix-interface-conventions)).

The return value is not specified. file may be an open file descriptor or an open file descriptor port.

Note that `flock` does not lock files across NFS.

Scheme Procedure: **select** reads writes excepts \[secs \[usecs\]\] [¶](07_02_02_ports_and_file_descriptors.md)

C Function: **scm\_select** (reads, writes, excepts, secs, usecs) [¶](07_02_02_ports_and_file_descriptors.md)

This procedure has a variety of uses: waiting for the ability to provide input, accept output, or the existence of exceptional conditions on a collection of ports or file descriptors, or waiting for a timeout to occur.

When an error occurs, this procedure throws a `system-error` exception (see [`system-error`](07_02_01_posix_interface_conventions.md#721-posix-interface-conventions)). Note that `select` may return early for other reasons, for example due to pending interrupts. See [Asynchronous Interrupts](06_22_threads_mutexes_asyncs_and_dynamic_roots.md#6223-asynchronous-interrupts), for more on interrupts.

reads, writes and excepts can be lists or vectors, with each member a port or a file descriptor. The value returned is a list of three corresponding lists or vectors containing only the members which meet the specified requirement. The ability of port buffers to provide input or accept output is taken into account. Ordering of the input lists or vectors is not preserved.

The optional arguments secs and usecs specify the timeout. Either secs can be specified alone, as either an integer or a real number, or both secs and usecs can be specified as integers, in which case usecs is an additional timeout expressed in microseconds. If secs is omitted or is `#f` then select will wait for as long as it takes for one of the other conditions to be satisfied.

The scsh version of `select` differs as follows: Only vectors are accepted for the first three arguments. The usecs argument is not supported. Multiple values are returned instead of a list. Duplicates in the input vectors appear only once in output. An additional `select!` interface is provided.

While it is sometimes necessary to operate at the level of file descriptors, this is an operation whose correctness can only be considered as part of a whole program. So for example while the effects of `(string-set! x 34 #\y)` are limited to the bits of code that can access x, `(close-fdes 34)` mutates the state of the entire process. In particular if another thread is using file descriptor 34 then their state might be corrupted; and another thread which opens a file might cause file descriptor 34 to be re-used, so that corruption could manifest itself in a strange way.

However when working with file descriptors, it’s common to want to associate information with the file descriptor, perhaps in a side table. To support this use case and to allow user code to remove an association when a file descriptor is closed, Guile offers _fdes finalizers_.

As the name indicates, fdes finalizers are finalizers – they can run in response to garbage collection, and they can also run in response to explicit calls to `close-port`, `close-fdes`, or the like. As such they inherit many of the pitfalls of finalizers: they may be invoked from concurrent threads, or not at all. See [Foreign Object Memory Management](05_programming_in_c.md#554-foreign-object-memory-management), for more on finalizers.

To use fdes finalizers, import their module;

(use-modules (ice-9 fdes-finalizers))

Scheme Procedure: **add-fdes-finalizer!** fdes finalizer [¶](07_02_02_ports_and_file_descriptors.md)

Scheme Procedure: **remove-fdes-finalizer!** fdes finalizer [¶](07_02_02_ports_and_file_descriptors.md)

Add or remove a finalizer for fdes. A finalizer is a procedure that is called by Guile when a file descriptor is closed. The file descriptor being closed is passed as the one argument to the finalizer. If a finalizer has been added multiple times to a file descriptor, to remove it would require that number of calls to `remove-fdes-finalizer!`.

The finalizers added to a file descriptor are called by Guile in an unspecified order, and their return values are ignored.

* * *

Next: [User Information](07_02_04_user_information.md#724-user-information), Previous: [Ports and File Descriptors](07_02_02_ports_and_file_descriptors.md#722-ports-and-file-descriptors), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
