#### 7.2.3 File System [¶](07_02_03_file_system.md#723-file-system)

These procedures allow querying and setting file system attributes (such as owner, permissions, sizes and types of files); deleting, copying, renaming and linking files; creating and removing directories and querying their contents; syncing the file system and creating special files.

Scheme Procedure: **access?** path how [¶](07_02_03_file_system.md)

C Function: **scm\_access** (path, how) [¶](07_02_03_file_system.md)

Test accessibility of a file under the real UID and GID of the calling process. The return is `#t` if path exists and the permissions requested by how are all allowed, or `#f` if not.

how is an integer which is one of the following values, or a bitwise-OR (`logior`) of multiple values.

Variable: **R\_OK** [¶](07_02_03_file_system.md)

Test for read permission.

Variable: **W\_OK** [¶](07_02_03_file_system.md)

Test for write permission.

Variable: **X\_OK** [¶](07_02_03_file_system.md)

Test for execute permission.

Variable: **F\_OK** [¶](07_02_03_file_system.md)

Test for existence of the file. This is implied by each of the other tests, so there’s no need to combine it with them.

It’s important to note that `access?` does not simply indicate what will happen on attempting to read or write a file. In normal circumstances it does, but in a set-UID or set-GID program it doesn’t because `access?` tests the real ID, whereas an open or execute attempt uses the effective ID.

A program which will never run set-UID/GID can ignore the difference between real and effective IDs, but for maximum generality, especially in library functions, it’s best not to use `access?` to predict the result of an open or execute, instead simply attempt that and catch any exception.

The main use for `access?` is to let a set-UID/GID program determine what the invoking user would have been allowed to do, without the greater (or perhaps lesser) privileges afforded by the effective ID. For more on this, see [Testing File Access](https://doc.guix.gnu.org/libc/latest/en/libc.html#Testing-File-Access) in The GNU C Library Reference Manual.

Scheme Procedure: **stat** object \[exception-on-error?\] [¶](07_02_03_file_system.md)

C Function: **scm\_stat** (object, exception\_on\_error) [¶](07_02_03_file_system.md)

Return an object containing various information about the file determined by object. object can be a string containing a file name or a port or integer file descriptor which is open on a file (in which case `fstat` is used as the underlying system call).

If the optional exception\_on\_error argument is true, which is the default, an exception will be raised if the underlying system call returns an error, for example if the file is not found or is not readable. Otherwise, an error will cause `stat` to return `#f`.

The object returned by `stat` can be passed as a single parameter to the following procedures, all of which return integers:

Scheme Procedure: **stat:dev** st [¶](07_02_03_file_system.md)

The device number containing the file.

Scheme Procedure: **stat:ino** st [¶](07_02_03_file_system.md)

The file serial number, which distinguishes this file from all other files on the same device.

Scheme Procedure: **stat:mode** st [¶](07_02_03_file_system.md)

The mode of the file. This is an integer which incorporates file type information and file permission bits. See also `stat:type` and `stat:perms` below.

Scheme Procedure: **stat:nlink** st [¶](07_02_03_file_system.md)

The number of hard links to the file.

Scheme Procedure: **stat:uid** st [¶](07_02_03_file_system.md)

The user ID of the file’s owner.

Scheme Procedure: **stat:gid** st [¶](07_02_03_file_system.md)

The group ID of the file.

Scheme Procedure: **stat:rdev** st [¶](07_02_03_file_system.md)

Device ID; this entry is defined only for character or block special files. On some systems this field is not available at all, in which case `stat:rdev` returns `#f`.

Scheme Procedure: **stat:size** st [¶](07_02_03_file_system.md)

The size of a regular file in bytes.

Scheme Procedure: **stat:atime** st [¶](07_02_03_file_system.md)

The last access time for the file, in seconds.

Scheme Procedure: **stat:mtime** st [¶](07_02_03_file_system.md)

The last modification time for the file, in seconds.

Scheme Procedure: **stat:ctime** st [¶](07_02_03_file_system.md)

The last modification time for the attributes of the file, in seconds.

Scheme Procedure: **stat:atimensec** st [¶](07_02_03_file_system.md)

Scheme Procedure: **stat:mtimensec** st [¶](07_02_03_file_system.md)

Scheme Procedure: **stat:ctimensec** st [¶](07_02_03_file_system.md)

The fractional part of a file’s access, modification, or attribute modification time, in nanoseconds. Nanosecond timestamps are only available on some operating systems and file systems. If Guile cannot retrieve nanosecond-level timestamps for a file, these fields will be set to 0.

Scheme Procedure: **stat:blksize** st [¶](07_02_03_file_system.md)

The optimal block size for reading or writing the file, in bytes. On some systems this field is not available, in which case `stat:blksize` returns a sensible suggested block size.

Scheme Procedure: **stat:blocks** st [¶](07_02_03_file_system.md)

The amount of disk space that the file occupies measured in units of 512 byte blocks. On some systems this field is not available, in which case `stat:blocks` returns `#f`.

In addition, the following procedures return the information from `stat:mode` in a more convenient form:

Scheme Procedure: **stat:type** st [¶](07_02_03_file_system.md)

A symbol representing the type of file. Possible values are ‘regular’, ‘directory’, ‘symlink’, ‘block-special’, ‘char-special’, ‘fifo’, ‘socket’, and ‘unknown’.

Scheme Procedure: **stat:perms** st [¶](07_02_03_file_system.md)

An integer representing the access permission bits.

Scheme Procedure: **lstat** path [¶](07_02_03_file_system.md)

C Function: **scm\_lstat** (path) [¶](07_02_03_file_system.md)

Similar to `stat`, but does not follow symbolic links, i.e., it will return information about a symbolic link itself, not the file it points to. path must be a string.

Scheme Procedure: **statat** dir filename \[flags\] [¶](07_02_03_file_system.md)

C Function: **scm\_statat** dir filename flags [¶](07_02_03_file_system.md)

Like `stat`, but resolve filename relative to the directory referred to by the file port dir instead. The optional argument flags argument can be `AT_SYMLINK_NOFOLLOW`, in which case filename will not be dereferenced even if it is a symbolic link.

Scheme Procedure: **readlink** path [¶](07_02_03_file_system.md)

C Function: **scm\_readlink** (path) [¶](07_02_03_file_system.md)

Return the value of the symbolic link named by path (a string, or a port if supported by the system), i.e., the file that the link points to.

To read a symbolic link represented by a port, the symbolic link must have been opened with the `O_NOFOLLOW` and `O_PATH` flags. `(provided? 'readlink-port)` reports whether ports are supported.

Scheme Procedure: **chown** object owner group [¶](07_02_03_file_system.md)

C Function: **scm\_chown** (object, owner, group) [¶](07_02_03_file_system.md)

Change the ownership and group of the file referred to by object to the integer values owner and group. object can be a string containing a file name or, if the platform supports `fchown` (see [File Owner](https://doc.guix.gnu.org/libc/latest/en/libc.html#File-Owner) in The GNU C Library Reference Manual), a port or integer file descriptor which is open on the file. The return value is unspecified.

If object is a symbolic link, either the ownership of the link or the ownership of the referenced file will be changed depending on the operating system (lchown is unsupported at present). If owner or group is specified as `-1`, then that ID is not changed.

Scheme Procedure: **chownat** dir name owner group \[flags\] [¶](07_02_03_file_system.md)

C Function: **scm\_chownat** (dir, name, owner, group, flags) [¶](07_02_03_file_system.md)

Like `chown`, but modify the owner and/or group of the file named name in the directory referred to by the file port dir instead. The optional argument flags is a bitmask. If `AT_SYMLINK_NOFOLLOW` is present, then name will not be dereferenced if it is a symbolic link.

Scheme Procedure: **chmod** object mode [¶](07_02_03_file_system.md)

C Function: **scm\_chmod** (object, mode) [¶](07_02_03_file_system.md)

Changes the permissions of the file referred to by object. object can be a string containing a file name or a port or integer file descriptor which is open on a file (in which case `fchmod` is used as the underlying system call). mode specifies the new permissions as a decimal number, e.g., `(chmod "foo" #o755)`. The return value is unspecified.

Scheme Procedure: **utime** object \[actime \[modtime \[actimens \[modtimens \[flags\]\]\]\]\] [¶](07_02_03_file_system.md)

C Function: **scm\_utime** (object, actime, modtime, actimens, modtimens, flags) [¶](07_02_03_file_system.md)

`utime` sets the access and modification times for the file named by object. If actime or modtime is not supplied, then the current time is used. actime and modtime must be integer time values as returned by the `current-time` procedure.

object must be a file name or a port (if supported by the system).

The optional actimens and modtimens are nanoseconds to add actime and modtime. Nanosecond precision is only supported on some combinations of file systems and operating systems.

([utime](07_02_03_file_system.md) "foo" ([\-](06_06_02_numerical_data_types.md) ([current-time](07_02_05_time.md)) 3600))

will set the access time to one hour in the past and the modification time to the current time.

Last, flags may be either `0` or the `AT_SYMLINK_NOFOLLOW` constant, to set the time of object even if it is a symbolic link.

On GNU/Linux systems, at least when using the Linux kernel 5.10.46, if object is a port, it may not be a symbolic link, even if `AT_SYMLINK_NOFOLLOW` is set. This is either a bug in Linux or Guile’s wrappers. The exact cause is unclear.

Scheme Procedure: **delete-file** str [¶](07_02_03_file_system.md)

C Function: **scm\_delete\_file** (str) [¶](07_02_03_file_system.md)

Deletes (or “unlinks”) the file whose path is specified by str.

Scheme Procedure: **delete-file-at** dir str \[flags\] [¶](07_02_03_file_system.md)

C Function: **scm\_delete\_file\_at** (dir, str, flags) [¶](07_02_03_file_system.md)

Like `unlink`, but resolve str relative to the directory referred to by the file port dir instead.

The optional flags argument can be `AT_REMOVEDIR`, in which case `delete-file-at` will act like `rmdir` instead of `delete-file`. Why doesn’t POSIX have a `rmdirat` function for this instead? No idea!

Scheme Procedure: **copy-file** oldfile newfile \[#:copy-on-write=’auto\] [¶](07_02_03_file_system.md)

C Function: **scm\_copy\_file** (oldfile, newfile) [¶](07_02_03_file_system.md)

Copy the file specified by oldfile to newfile. The return value is unspecified.

`#:copy-on-write` keyword argument determines whether copy-on-write copy should be attempted and the behavior in case of failure. Possible values are `'always` (attempt the copy-on-write, return error if it fails), `'auto` (attempt the copy-on-write, fallback to regular copy if it fails) and `'never` (perform the regular copy).

Scheme Procedure: **sendfile** out in count \[offset\] [¶](07_02_03_file_system.md)

C Function: **scm\_sendfile** (out, in, count, offset) [¶](07_02_03_file_system.md)

Send count bytes from in to out, both of which must be either open file ports or file descriptors. When offset is omitted, start reading from in’s current position; otherwise, start reading at offset. Return the number of bytes actually sent.

When in is a port, it is often preferable to specify offset, because in’s offset as a port may be different from the offset of its underlying file descriptor.

On systems that support it, such as GNU/Linux, this procedure uses the `sendfile` libc function, which usually corresponds to a system call. This is faster than doing a series of `read` and `write` system calls. A typical application is to send a file over a socket.

In some cases, the `sendfile` libc function may return `EINVAL` or `ENOSYS`. In that case, Guile’s `sendfile` procedure automatically falls back to doing a series of `read` and `write` calls.

In other cases, the libc function may send fewer bytes than count—for instance because out is a slow or limited device, such as a pipe. When that happens, Guile’s `sendfile` automatically retries until exactly count bytes were sent or an error occurs.

Scheme Procedure: **rename-file** oldname newname [¶](07_02_03_file_system.md)

C Function: **scm\_rename** (oldname, newname) [¶](07_02_03_file_system.md)

Renames the file specified by oldname to newname. The return value is unspecified.

Scheme Procedure: **rename-file-at** olddir oldname newdir newname [¶](07_02_03_file_system.md)

C Function: **scm\_renameat** (olddir, oldname, newdir, newname) [¶](07_02_03_file_system.md)

Like `rename-file`, but when olddir or newdir is true, resolve oldname or newname relative to the directory specified by the file port olddir or newdir instead of the current working directory.

Scheme Procedure: **link** oldpath newpath [¶](07_02_03_file_system.md)

C Function: **scm\_link** (oldpath, newpath) [¶](07_02_03_file_system.md)

Creates a new name newpath in the file system for the file named by oldpath. If oldpath is a symbolic link, the link may or may not be followed depending on the system.

Scheme Procedure: **symlink** oldpath newpath [¶](07_02_03_file_system.md)

C Function: **scm\_symlink** (oldpath, newpath) [¶](07_02_03_file_system.md)

Create a symbolic link named newpath with the value (i.e., pointing to) oldpath. The return value is unspecified.

Scheme Procedure: **symlinkat** dir oldpath newpath [¶](07_02_03_file_system.md)

C Function: **scm\_symlinkat** (dir, oldpath, newpath) [¶](07_02_03_file_system.md)

Like `symlink`, but resolve newpath relative to the directory referred to by the file port dir.

Scheme Procedure: **mkdir** path \[mode\] [¶](07_02_03_file_system.md)

C Function: **scm\_mkdir** (path, mode) [¶](07_02_03_file_system.md)

Create a new directory named by path. If mode is omitted then the permissions of the directory are set to `#o777` masked with the current umask (see [`umask`](07_02_07_processes.md#727-processes)). Otherwise they are set to the value specified with mode masked with the current umask. The return value is unspecified.

Scheme Procedure: **mkdirat** dir path \[mode\] [¶](07_02_03_file_system.md)

C Function: **scm\_mkdirat** (dir, path, mode) [¶](07_02_03_file_system.md)

Like `mkdir`, but resolve path relative to the directory referred to by the file port dir instead.

Scheme Procedure: **rmdir** path [¶](07_02_03_file_system.md)

C Function: **scm\_rmdir** (path) [¶](07_02_03_file_system.md)

Remove the existing directory named by path. The directory must be empty for this to succeed. The return value is unspecified.

Scheme Procedure: **opendir** dirname [¶](07_02_03_file_system.md)

C Function: **scm\_opendir** (dirname) [¶](07_02_03_file_system.md)

Open the directory specified by dirname and return a directory stream.

Before using this and the procedures below, make sure to see the higher-level procedures for directory traversal that are available (see [File Tree Walk](07_12_file_tree_walk.md#712-file-tree-walk)).

Scheme Procedure: **directory-stream?** object [¶](07_02_03_file_system.md)

C Function: **scm\_directory\_stream\_p** (object) [¶](07_02_03_file_system.md)

Return a boolean indicating whether object is a directory stream as returned by `opendir`.

Scheme Procedure: **readdir** stream [¶](07_02_03_file_system.md)

C Function: **scm\_readdir** (stream) [¶](07_02_03_file_system.md)

Return (as a string) the next directory entry from the directory stream stream. If there is no remaining entry to be read then the end of file object is returned.

Scheme Procedure: **rewinddir** stream [¶](07_02_03_file_system.md)

C Function: **scm\_rewinddir** (stream) [¶](07_02_03_file_system.md)

Reset the directory port stream so that the next call to `readdir` will return the first directory entry.

Scheme Procedure: **closedir** stream [¶](07_02_03_file_system.md)

C Function: **scm\_closedir** (stream) [¶](07_02_03_file_system.md)

Close the directory stream stream. The return value is unspecified.

Here is an example showing how to display all the entries in a directory:

(define dir ([opendir](07_02_03_file_system.md) "/usr/lib"))
(do ((entry ([readdir](07_02_03_file_system.md) dir) ([readdir](07_02_03_file_system.md) dir)))
    (([eof-object?](06_12_input_and_output.md) entry))
  ([display](06_16_reading_and_evaluating_scheme_code.md) entry)([newline](06_12_input_and_output.md)))
([closedir](07_02_03_file_system.md) dir)

Scheme Procedure: **sync** [¶](07_02_03_file_system.md)

C Function: **scm\_sync** () [¶](07_02_03_file_system.md)

Flush the operating system disk buffers. The return value is unspecified.

Scheme Procedure: **mknod** path type perms dev [¶](07_02_03_file_system.md)

C Function: **scm\_mknod** (path, type, perms, dev) [¶](07_02_03_file_system.md)

Creates a new special file, such as a file corresponding to a device. path specifies the name of the file. type should be one of the following symbols: ‘regular’, ‘directory’, ‘symlink’, ‘block-special’, ‘char-special’, ‘fifo’, or ‘socket’. perms (an integer) specifies the file permissions. dev (an integer) specifies which device the special file refers to. Its exact interpretation depends on the kind of special file being created.

E.g.,

([mknod](07_02_03_file_system.md) "/dev/fd0" 'block-special #o660 ([+](06_06_02_numerical_data_types.md) ([\*](06_06_02_numerical_data_types.md) 2 256) 2))

The return value is unspecified.

Scheme Procedure: **tmpnam** [¶](07_02_03_file_system.md)

C Function: **scm\_tmpnam** () [¶](07_02_03_file_system.md)

Return an auto-generated name of a temporary file, a file which doesn’t already exist. The name includes a path, it’s usually in /tmp but that’s system dependent.

Care must be taken when using `tmpnam`. In between choosing the name and creating the file another program might use that name, or an attacker might even make it a symlink pointing at something important and causing you to overwrite that.

The safe way is to create the file using `open` with `O_EXCL` to avoid any overwriting. A loop can try again with another name if the file exists (error `EEXIST`). `mkstemp` below does that.

Scheme Procedure: **mkstemp** tmpl \[mode\] [¶](07_02_03_file_system.md)

Create a new unique file in the file system and return a new buffered port open for reading and writing to the file.

tmpl is a string specifying where the file should be created: it must end with ‘XXXXXX’. The name of the newly created file will be the same as tmpl, but with those ‘X’s changed, and can be determined by calling `port-filename` on the returned port.

Note that the newly created file is not deleted automatically by Guile; probably the caller should arrange to call `delete-file` when the file is no longer needed.

POSIX doesn’t specify the permissions mode of the file. On GNU and most systems it’s `#o600`; an application can use `chmod` to relax that if desired. For example `#o666` less `umask`, which is usual for ordinary file creation,

(let ((port (mkstemp "/tmp/myfile-XXXXXX")))
  (chmod port (logand #o666 (lognot (umask))))
  ...)

The optional mode argument specifies a mode with which to open the new file, as a string in the same format that `open-file` takes. It defaults to `"w+"`.

Scheme Procedure: **tmpfile** [¶](07_02_03_file_system.md)

C Function: **scm\_tmpfile** () [¶](07_02_03_file_system.md)

Return an input/output port to a unique temporary file named using the path prefix `P_tmpdir` defined in stdio.h. The file is automatically deleted when the port is closed or the program terminates.

Scheme Procedure: **mkdtemp** tmpl [¶](07_02_03_file_system.md)

C Function: **scm\_mkdtemp** (tmpl) [¶](07_02_03_file_system.md)

Create a new directory named in accordance with the template string tmpl.

tmpl is a string specifying the directory’s name. The last six characters of tmpl must be ‘XXXXXX’. Upon successful execution, the name of the new directory is returned which has the same form as tmpl but with the ‘XXXXXX’ characters modified to ensure the directory name is unique.

The permissions of the directory created are OS dependent, but, are usually `#o700`.

An error may be thrown if the template has the wrong format or if the directory cannot be created.

Scheme Procedure: **dirname** filename [¶](07_02_03_file_system.md)

C Function: **scm\_dirname** (filename) [¶](07_02_03_file_system.md)

Return the directory name component of the file name filename. If filename does not contain a directory component, `.` is returned.

Scheme Procedure: **basename** filename \[suffix\] [¶](07_02_03_file_system.md)

C Function: **scm\_basename** (filename, suffix) [¶](07_02_03_file_system.md)

Return the base name of the file name filename. The base name is the file name without any directory components. If suffix is provided, and is equal to the end of basename, it is removed also.

([basename](07_02_03_file_system.md) "/tmp/test.xml" ".xml")
⇒ "test"

Scheme Procedure: **canonicalize-path** path [¶](07_02_03_file_system.md)

C Function: **scm\_canonicalize\_path** (path) [¶](07_02_03_file_system.md)

Return the canonical (absolute) path of path. A canonical path has no `.` or `..` components, nor any repeated path separators (`/`) nor symlinks.

Raises an error if any component of path does not exist.

([canonicalize-path](07_02_03_file_system.md) "test.xml")
⇒ "/tmp/test.xml"

Scheme Procedure: **file-exists?** filename [¶](07_02_03_file_system.md)

Return `#t` if the file named filename exists, `#f` if not.

Many operating systems, such as GNU, use `/` (forward slash) to separate the components of a file name; any file name starting with `/` is considered an _absolute file name_. These conventions are specified by the POSIX Base Definitions, which refer to conforming file names as “pathnames”. Some operating systems use a different convention; in particular, Windows uses `\` (backslash) as the file name separator, and also has the notion of _volume names_ like `C:\` for absolute file names. The following procedures and variables provide support for portable file name manipulations.

Scheme Procedure: **system-file-name-convention** [¶](07_02_03_file_system.md)

Return either `posix` or `windows`, depending on what kind of system this Guile is running on.

Scheme Procedure: **file-name-separator?** c [¶](07_02_03_file_system.md)

Return true if character c is a file name separator on the host platform.

Scheme Procedure: **absolute-file-name?** file-name [¶](07_02_03_file_system.md)

Return true if file-name denotes an absolute file name on the host platform.

Scheme Variable: **file-name-separator-string** [¶](07_02_03_file_system.md)

The preferred file name separator.

Note that on MinGW builds for Windows, both `/` and `\` are valid separators. Thus, programs should not assume that `file-name-separator-string` is the _only_ file name separator—e.g., when extracting the components of a file name.

Scheme Procedure: **in-vicinity** directory file [¶](07_02_03_file_system.md)

Concatenate directory and file, adding `file-name-separator-string` (by default slash) in between if it is not already present. This helps create file names.

* * *

Next: [Time](07_02_05_time.md#725-time), Previous: [File System](07_02_03_file_system.md#723-file-system), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
