### 7.13 Queues [¶](07_13_queues.md#713-queues)

The functions in this section are provided by

(use-modules (ice-9 q))

This module implements queues holding arbitrary scheme objects and designed for efficient first-in / first-out operations.

`make-q` creates a queue, and objects are entered and removed with `enq!` and `deq!`. `q-push!` and `q-pop!` can be used too, treating the front of the queue like a stack.

  

Scheme Procedure: **make-q** [¶](07_13_queues.md)

Return a new queue.

Scheme Procedure: **q?** obj [¶](07_13_queues.md)

Return `#t` if obj is a queue, or `#f` if not.

Note that queues are not a distinct class of objects but are implemented with cons cells. For that reason certain list structures can get `#t` from `q?`.

Scheme Procedure: **enq!** q obj [¶](07_13_queues.md)

Add obj to the rear of q, and return q.

Scheme Procedure: **deq!** q [¶](07_13_queues.md)

Scheme Procedure: **q-pop!** q [¶](07_13_queues.md)

Remove and return the front element from q. If q is empty, a `q-empty` exception is thrown.

`deq!` and `q-pop!` are the same operation, the two names just let an application match `enq!` with `deq!`, or `q-push!` with `q-pop!`.

Scheme Procedure: **q-push!** q obj [¶](07_13_queues.md)

Add obj to the front of q, and return q.

Scheme Procedure: **q-length** q [¶](07_13_queues.md)

Return the number of elements in q.

Scheme Procedure: **q-empty?** q [¶](07_13_queues.md)

Return true if q is empty.

Scheme Procedure: **q-empty-check** q [¶](07_13_queues.md)

Throw a `q-empty` exception if q is empty.

Scheme Procedure: **q-front** q [¶](07_13_queues.md)

Return the first element of q (without removing it). If q is empty, a `q-empty` exception is thrown.

Scheme Procedure: **q-rear** q [¶](07_13_queues.md)

Return the last element of q (without removing it). If q is empty, a `q-empty` exception is thrown.

Scheme Procedure: **q-remove!** q obj [¶](07_13_queues.md)

Remove all occurrences of obj from q, and return q. obj is compared to queue elements using `eq?`.

  

The `q-empty` exceptions described above are thrown just as `(throw 'q-empty)`, there’s no message etc like an error throw.

A queue is implemented as a cons cell, the `car` containing a list of queued elements, and the `cdr` being the last cell in that list (for ease of enqueuing).

(list . last-cell)

If the queue is empty, list is the empty list and last-cell is `#f`.

An application can directly access the queue list if desired, for instance to search the elements or to insert at a specific point.

Scheme Procedure: **sync-q!** q [¶](07_13_queues.md)

Recompute the last-cell field in q.

All the operations above maintain last-cell as described, so normally there’s no need for `sync-q!`. But if an application modifies the queue list then it must either maintain last-cell similarly, or call `sync-q!` to recompute it.

* * *

Next: [Buffered Input](07_15_buffered_input.md#715-buffered-input), Previous: [Queues](07_13_queues.md#713-queues), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
