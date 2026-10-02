#### 7.5.20 SRFI-28 - Basic Format Strings [¶](07_05_20_srfi28_basic_format_strings.md#7520-srfi-28---basic-format-strings)

SRFI-28 provides a basic `format` procedure that provides only the `~a`, `~s`, `~%`, and `~~` format specifiers. You can import this procedure by using:

([use-modules](06_18_modules.md) (srfi srfi-28))

Scheme Procedure: **format** message arg … [¶](07_05_20_srfi28_basic_format_strings.md)

Returns a formatted message, using message as the format string, which can contain the following format specifiers:

`~a`

Insert the textual representation of the next arg, as if printed by `display`.

`~s`

Insert the textual representation of the next arg, as if printed by `write`.

`~%`

Insert a newline.

`~~`

Insert a tilde.

This procedure is the same as calling `simple-format` (see [Simple Textual Output](06_12_input_and_output.md#6125-simple-textual-output)) with `#f` as the destination.

* * *

Next: [SRFI-31 - A special form ‘rec’ for recursive evaluation](07_05_22_srfi31_a_special_form_rec_for_recursive_evaluation.md#7522-srfi-31---a-special-form-rec-for-recursive-evaluation), Previous: [SRFI-28 - Basic Format Strings](07_05_20_srfi28_basic_format_strings.md#7520-srfi-28---basic-format-strings), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
