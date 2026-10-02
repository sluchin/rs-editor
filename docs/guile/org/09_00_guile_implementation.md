9 Guile Implementation [¶](09_00_guile_implementation.md#9-guile-implementation)
------------------------------------------------------------------------------------------------------

At some point, after one has been programming in Scheme for some time, another level of Scheme comes into view: its implementation. Knowledge of how Scheme can be implemented turns out to be necessary to become an expert hacker. As Peter Norvig notes in his retrospective on PAIP[36](99_footnotes.md), “The expert Lisp programmer eventually develops a good ‘efficiency model’.”

By this Norvig means that over time, the Lisp hacker eventually develops an understanding of how much her code “costs” in terms of space and time.

This chapter describes Guile as an implementation of Scheme: its history, how it represents and evaluates its data, and its compiler. This knowledge can help you to make that step from being one who is merely familiar with Scheme to being a real hacker.

*   [A Brief History of Guile](09_01_a_brief_history_of_guile.md#91-a-brief-history-of-guile)
*   [Data Representation](09_02_data_representation.md#92-data-representation)
*   [A Virtual Machine for Guile](09_03_a_virtual_machine_for_guile.md#93-a-virtual-machine-for-guile)
*   [Compiling to the Virtual Machine](09_04_compiling_to_the_virtual_machine.md#94-compiling-to-the-virtual-machine)

* * *

Next: [Data Representation](09_02_data_representation.md#92-data-representation), Up: [Guile Implementation](09_00_guile_implementation.md#9-guile-implementation)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
