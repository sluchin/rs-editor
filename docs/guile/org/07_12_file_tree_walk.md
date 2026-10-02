### 7.12 File Tree Walk [¶](07_12_file_tree_walk.md#712-file-tree-walk)

The functions in this section traverse a tree of files and directories. They come in two flavors: the first one is a high-level functional interface, and the second one is similar to the C `ftw` and `nftw` routines (see [Working with Directory Trees](https://doc.guix.gnu.org/libc/latest/en/libc.html#Working-with-Directory-Trees) in GNU C Library Reference Manual).

(use-modules (ice-9 ftw))

  

Scheme Procedure: **file-system-tree** file-name \[enter? \[stat\]\] [¶](07_12_file_tree_walk.md)

Return a tree of the form `(file-name stat children ...)` where stat is the result of `(stat file-name)` and children are similar structures for each file contained in file-name when it designates a directory.

The optional enter? predicate is invoked as `(enter? name stat)` and should return true to allow recursion into directory name; the default value is a procedure that always returns `#t`. When a directory does not match enter?, it nonetheless appears in the resulting tree, only with zero children.

The stat argument is optional and defaults to `lstat`, as for `file-system-fold` (see below.)

The example below shows how to obtain a hierarchical listing of the files under the module/language directory in the Guile source tree, discarding their `stat` info:

(use-modules (ice-9 match))

(define remove-stat
  ;; Remove the \`stat' object the \`file-system-tree' provides
  ;; for each file in the tree.
  (match-lambda
    ((name stat)              ; flat file
     name)
    ((name stat children ...) ; directory
     (list name (map remove-stat children)))))

(let ((dir (string-append (assq-ref %guile-build-info 'top\_srcdir)
                          "/module/language")))
  (remove-stat (file-system-tree dir)))

⇒
("language"
 (("value" ("spec.go" "spec.scm"))
  ("scheme"
   ("spec.go"
    "spec.scm"
    "compile-tree-il.scm"
    "decompile-tree-il.scm"
    "decompile-tree-il.go"
    "compile-tree-il.go"))
  ("tree-il"
   ("spec.go"
    "fix-letrec.go"
    "inline.go"
    "fix-letrec.scm"
    "compile-glil.go"
    "spec.scm"
    "optimize.scm"
    "primitives.scm"
    ...))
  ...))

It is often desirable to process directories entries directly, rather than building up a tree of entries in memory, like `file-system-tree` does. The following procedure, a _combinator_, is designed to allow directory entries to be processed directly as a directory tree is traversed; in fact, `file-system-tree` is implemented in terms of it.

Scheme Procedure: **file-system-fold** enter? leaf down up skip error init file-name \[stat\] [¶](07_12_file_tree_walk.md)

Traverse the directory at file-name, recursively, and return the result of the successive applications of the leaf, down, up, and skip procedures as described below.

Enter sub-directories only when `(enter? path stat result)` returns true. When a sub-directory is entered, call `(down path stat result)`, where path is the path of the sub-directory and stat the result of `(false-if-exception (stat path))`; when it is left, call `(up path stat result)`.

For each file in a directory, call `(leaf path stat result)`.

When enter? returns `#f`, or when an unreadable directory is encountered, call `(skip path stat result)`.

When file-name names a flat file, `(leaf path stat init)` is returned.

When an `opendir` or stat call fails, call `(error path stat errno result)`, with errno being the operating system error number that was raised—e.g., `EACCES`—and stat either `#f` or the result of the stat call for that entry, when available.

The special . and .. entries are not passed to these procedures. The path argument to the procedures is a full file name—e.g., `"../foo/bar/gnu"`; if file-name is an absolute file name, then path is also an absolute file name. Files and directories, as identified by their device/inode number pair, are traversed only once.

The optional stat argument defaults to `lstat`, which means that symbolic links are not followed; the `stat` procedure can be used instead when symbolic links are to be followed (see [stat](07_02_03_file_system.md#723-file-system)).

The example below illustrates the use of `file-system-fold`:

(define (total-file-size file-name)
  "Return the size in bytes of the files under FILE-NAME (similar
to \`du --apparent-size' with GNU Coreutils.)"

  (define (enter? name stat result)
    ;; Skip version control directories.
    (not (member (basename name) '(".git" ".svn" "CVS"))))
  (define (leaf name stat result)
    ;; Return RESULT plus the size of the file at NAME.
    (+ result (stat:size stat)))

  ;; Count zero bytes for directories.
  (define (down name stat result) result)
  (define (up name stat result) result)

  ;; Likewise for skipped directories.
  (define (skip name stat result) result)

  ;; Ignore unreadable files/directories but warn the user.
  (define (error name stat errno result)
    (format (current-error-port) "warning: ~a: ~a~%"
            name (strerror errno))
    result)

  (file-system-fold enter? leaf down up skip error
                           0  ; initial counter is zero bytes
                           file-name))

(total-file-size ".")
⇒ 8217554

(total-file-size "/dev/null")
⇒ 0

The alternative C-like functions are described below.

Scheme Procedure: **scandir** name \[select? \[entry<?\]\] [¶](07_12_file_tree_walk.md)

Return the list of the names of files contained in directory name that match predicate select? (by default, all files). The returned list of file names is sorted according to entry<?, which defaults to `string-locale<?` such that file names are sorted in the locale’s alphabetical order (see [Text Collation](06_25_support_for_internationalization.md#6252-text-collation)). Return `#f` when name is unreadable or is not a directory.

This procedure is modeled after the C library function of the same name (see [Scanning Directory Content](https://doc.guix.gnu.org/libc/latest/en/libc.html#Scanning-Directory-Content) in GNU C Library Reference Manual).

Scheme Procedure: **ftw** startname proc \[’hash-size n\] [¶](07_12_file_tree_walk.md)

Walk the file system tree descending from startname, calling proc for each file and directory.

Hard links and symbolic links are followed. A file or directory is reported to proc only once, and skipped if seen again in another place. One consequence of this is that `ftw` is safe against circularly linked directory structures.

Each proc call is `(proc filename statinfo flag)` and it should return `#t` to continue, or any other value to stop.

filename is the item visited, being startname plus a further path and the name of the item. statinfo is the return from `stat` (see [File System](07_02_03_file_system.md#723-file-system)) on filename. flag is one of the following symbols,

`regular`

filename is a file, this includes special files like devices, named pipes, etc.

`directory`

filename is a directory.

`invalid-stat`

An error occurred when calling `stat`, so nothing is known. statinfo is `#f` in this case.

`directory-not-readable`

filename is a directory, but one which cannot be read and hence won’t be recursed into.

`symlink`

filename is a dangling symbolic link. Symbolic links are normally followed and their target reported, the link itself is reported if the target does not exist.

The return value from `ftw` is `#t` if it ran to completion, or otherwise the non-`#t` value from proc which caused the stop.

Optional argument symbol `hash-size` and an integer can be given to set the size of the hash table used to track items already visited. (see [Hash Table Reference](06_06_22_hash_tables.md#66222-hash-table-reference))

In the current implementation, returning non-`#t` from proc is the only valid way to terminate `ftw`. proc must not use `throw` or similar to escape.

Scheme Procedure: **nftw** startname proc \[’chdir\] \[’depth\] \[’hash-size n\] \[’mount\] \[’physical\] [¶](07_12_file_tree_walk.md)

Walk the file system tree starting at startname, calling proc for each file and directory. `nftw` has extra features over the basic `ftw` described above.

Like `ftw`, hard links and symbolic links are followed. A file or directory is reported to proc only once, and skipped if seen again in another place. One consequence of this is that `nftw` is safe against circular linked directory structures.

Each proc call is `(proc filename statinfo flag base level)` and it should return `#t` to continue, or any other value to stop.

filename is the item visited, being startname plus a further path and the name of the item. statinfo is the return from `stat` on filename (see [File System](07_02_03_file_system.md#723-file-system)). base is an integer offset into filename which is where the basename for this item begins. level is an integer giving the directory nesting level, starting from 0 for the contents of startname (or that item itself if it’s a file). flag is one of the following symbols,

`regular`

filename is a file, including special files like devices, named pipes, etc.

`directory`

filename is a directory.

`directory-processed`

filename is a directory, and its contents have all been visited. This flag is given instead of `directory` when the `depth` option below is used.

`invalid-stat`

An error occurred when applying `stat` to filename, so nothing is known about it. statinfo is `#f` in this case.

`directory-not-readable`

filename is a directory, but one which cannot be read and hence won’t be recursed into.

`stale-symlink`

filename is a dangling symbolic link. Links are normally followed and their target reported, the link itself is reported if its target does not exist.

`symlink`

When the `physical` option described below is used, this indicates filename is a symbolic link whose target exists (and is not being followed).

The following optional arguments can be given to modify the way `nftw` works. Each is passed as a symbol (and `hash-size` takes a following integer value).

`chdir`

Change to the directory containing the item before calling proc. When `nftw` returns the original current directory is restored.

Under this option, generally the base parameter to each proc call should be used to pick out the base part of the filename. The filename is still a path but with a changed directory it won’t be valid (unless the startname directory was absolute).

`depth`

Visit files “depth first”, meaning proc is called for the contents of each directory before it’s called for the directory itself. Normally a directory is reported first, then its contents.

Under this option, the flag to proc for a directory is `directory-processed` instead of `directory`.

`hash-size n`

Set the size of the hash table used to track items already visited. (see [Hash Table Reference](06_06_22_hash_tables.md#66222-hash-table-reference))

`mount`

Don’t cross a mount point, meaning only visit items on the same file system as startname (ie. the same `stat:dev`).

`physical`

Don’t follow symbolic links, instead report them to proc as `symlink`. Dangling links (those whose target doesn’t exist) are still reported as `stale-symlink`.

The return value from `nftw` is `#t` if it ran to completion, or otherwise the non-`#t` value from proc which caused the stop.

In the current implementation, returning non-`#t` from proc is the only valid way to terminate `ftw`. proc must not use `throw` or similar to escape.

* * *

Next: [Streams](07_14_streams.md#714-streams), Previous: [File Tree Walk](07_12_file_tree_walk.md#712-file-tree-walk), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
