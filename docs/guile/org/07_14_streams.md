### 7.14 Streams [¶](07_14_streams.md#714-streams)

This section documents Guile’s legacy stream module. For a more complete and portable stream library, see [SRFI-41 - Streams](07_05_28_srfi41_streams.md#7528-srfi-41---streams).

A stream represents a sequence of values, each of which is calculated only when required. This allows large or even infinite sequences to be represented and manipulated with familiar operations like “car”, “cdr”, “map” or “fold”. In such manipulations only as much as needed is actually held in memory at any one time. The functions in this section are available from

(use-modules (ice-9 streams))

Streams are implemented using promises (see [Delayed Evaluation](06_16_reading_and_evaluating_scheme_code.md#61610-delayed-evaluation)), which is how the underlying calculation of values is made only when needed, and the values then retained so the calculation is not repeated.

Here is a simple example producing a stream of all odd numbers,

(define odds (make-stream (lambda (state)
                            (cons state (+ state 2)))
                          1))
(stream-car odds)              ⇒ 1
(stream-car (stream-cdr odds)) ⇒ 3

`stream-map` could be used to derive a stream of odd squares,

(define (square n) (\* n n))
(define oddsquares (stream-map square odds))

These are infinite sequences, so it’s not possible to convert them to a list, but they could be printed (infinitely) with for example

(stream-for-each (lambda (n sq)
                   (format #t "~a squared is ~a\\n" n sq))
                 odds oddsquares)
⊣
1 squared is 1
3 squared is 9
5 squared is 25
7 squared is 49
...

  

Scheme Procedure: **make-stream** proc initial-state [¶](07_14_streams.md)

Return a new stream, formed by calling proc successively.

Each call is `(proc state)`, it should return a pair, the `car` being the value for the stream, and the `cdr` being the new state for the next call. For the first call state is the given initial-state. At the end of the stream, proc should return some non-pair object.

Scheme Procedure: **stream-car** stream [¶](07_14_streams.md)

Return the first element from stream. stream must not be empty.

Scheme Procedure: **stream-cdr** stream [¶](07_14_streams.md)

Return a stream which is the second and subsequent elements of stream. stream must not be empty.

Scheme Procedure: **stream-null?** stream [¶](07_14_streams.md)

Return true if stream is empty.

Scheme Procedure: **list->stream** list [¶](07_14_streams.md)

Scheme Procedure: **vector->stream** vector [¶](07_14_streams.md)

Return a stream with the contents of list or vector.

list or vector should not be modified subsequently, since it’s unspecified whether changes there will be reflected in the stream returned.

Scheme Procedure: **port->stream** port readproc [¶](07_14_streams.md)

Return a stream which is the values obtained by reading from port using readproc. Each read call is `(readproc port)`, and it should return an EOF object (see [Binary I/O](06_12_input_and_output.md#6122-binary-io)) at the end of input.

For example a stream of characters from a file,

(port->stream (open-input-file "/foo/bar.txt") read-char)

Scheme Procedure: **stream->list** stream [¶](07_14_streams.md)

Return a list which is the entire contents of stream.

Scheme Procedure: **stream->reversed-list** stream [¶](07_14_streams.md)

Return a list which is the entire contents of stream, but in reverse order.

Scheme Procedure: **stream->list&length** stream [¶](07_14_streams.md)

Return two values (see [Returning and Accepting Multiple Values](06_11_controlling_the_flow_of_program_execution.md#6117-returning-and-accepting-multiple-values)), being firstly a list which is the entire contents of stream, and secondly the number of elements in that list.

Scheme Procedure: **stream->reversed-list&length** stream [¶](07_14_streams.md)

Return two values (see [Returning and Accepting Multiple Values](06_11_controlling_the_flow_of_program_execution.md#6117-returning-and-accepting-multiple-values)) being firstly a list which is the entire contents of stream, but in reverse order, and secondly the number of elements in that list.

Scheme Procedure: **stream->vector** stream [¶](07_14_streams.md)

Return a vector which is the entire contents of stream.

Function: **stream-fold** proc init stream1 stream2 … [¶](07_14_streams.md)

Apply proc successively over the elements of the given streams, from first to last until the end of the shortest stream is reached. Return the result from the last proc call.

Each call is `(proc elem1 elem2 … prev)`, where each elem is from the corresponding stream. prev is the return from the previous proc call, or the given init for the first call.

Function: **stream-for-each** proc stream1 stream2 … [¶](07_14_streams.md)

Call proc on the elements from the given streams. The return value is unspecified.

Each call is `(proc elem1 elem2 …)`, where each elem is from the corresponding stream. `stream-for-each` stops when it reaches the end of the shortest stream.

Function: **stream-map** proc stream1 stream2 … [¶](07_14_streams.md)

Return a new stream which is the results of applying proc to the elements of the given streams.

Each call is `(proc elem1 elem2 …)`, where each elem is from the corresponding stream. The new stream ends when the end of the shortest given stream is reached.

* * *

Next: [Expect](07_16_expect.md#716-expect), Previous: [Streams](07_14_streams.md#714-streams), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
