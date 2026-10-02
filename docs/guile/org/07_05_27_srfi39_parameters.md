#### 7.5.27 SRFI-39 - Parameters [¶](07_05_27_srfi39_parameters.md#7527-srfi-39---parameters)

This SRFI adds support for dynamically-scoped parameters. SRFI 39 is implemented in the Guile core; there’s no module needed to get SRFI-39 itself. Parameters are documented in [Parameters](06_11_controlling_the_flow_of_program_execution.md#61112-parameters).

This module does export one extra function: `with-parameters*`. This is a Guile-specific addition to the SRFI, similar to the core `with-fluids*` (see [Fluids and Dynamic States](06_11_controlling_the_flow_of_program_execution.md#61111-fluids-and-dynamic-states)).

Function: **with-parameters\*** param-list value-list thunk [¶](07_05_27_srfi39_parameters.md)

Establish a new dynamic scope, as per `parameterize` above, taking parameters from param-list and corresponding values from value-list. A call `(thunk)` is made in the new scope and the result from that thunk is the return from `with-parameters*`.

* * *

Next: [SRFI-42 - Eager Comprehensions](07_05_29_srfi42_eager_comprehensions.md#7529-srfi-42---eager-comprehensions), Previous: [SRFI-39 - Parameters](07_05_27_srfi39_parameters.md#7527-srfi-39---parameters), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
