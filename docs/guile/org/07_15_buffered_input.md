### 7.15 Buffered Input [¶](07_15_buffered_input.md#715-buffered-input)

The following functions are provided by

(use-modules (ice-9 buffered-input))

A buffered input port allows a reader function to return chunks of characters which are to be handed out on reading the port. A notion of further input for an application level logical expression is maintained too, and passed through to the reader.

Scheme Procedure: **make-buffered-input-port** reader [¶](07_15_buffered_input.md)

Create an input port which returns characters obtained from the given reader function. reader is called (reader cont), and should return a string or an EOF object.

The new port gives precisely the characters returned by reader, nothing is added, so if any newline characters or other separators are desired they must come from the reader function.

The cont parameter to reader is `#f` for initial input, or `#t` when continuing an expression. This is an application level notion, set with `set-buffered-input-continuation?!` below. If the user has entered a partial expression then it allows reader for instance to give a different prompt to show more is required.

Scheme Procedure: **make-line-buffered-input-port** reader [¶](07_15_buffered_input.md)

Create an input port which returns characters obtained from the specified reader function, similar to `make-buffered-input-port` above, but where reader is expected to be a line-oriented.

reader is called (reader cont), and should return a string or an EOF object as above. Each string is a line of input without a newline character, the port code inserts a newline after each string.

Scheme Procedure: **set-buffered-input-continuation?!** port cont [¶](07_15_buffered_input.md)

Set the input continuation flag for a given buffered input port.

An application uses this by calling with a cont flag of `#f` when beginning to read a new logical expression. For example with the Scheme `read` function (see [Reading Scheme Code](06_16_reading_and_evaluating_scheme_code.md#6162-reading-scheme-code)),

(define my-port (make-buffered-input-port my-reader))

(set-buffered-input-continuation?! my-port #f)
(let ((obj (read my-port)))
  ...

* * *

Next: [`sxml-match`: Pattern Matching of SXML](07_17_sxmlmatch_pattern_matching_of_sxml.md#717-sxml-match-pattern-matching-of-sxml), Previous: [Buffered Input](07_15_buffered_input.md#715-buffered-input), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
