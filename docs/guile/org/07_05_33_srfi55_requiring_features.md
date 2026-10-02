#### 7.5.33 SRFI-55 - Requiring Features [¶](07_05_33_srfi55_requiring_features.md#7533-srfi-55---requiring-features)

SRFI-55 provides `require-extension` which is a portable mechanism to load selected SRFI modules. This is implemented in the Guile core, there’s no module needed to get SRFI-55 itself.

library syntax: **require-extension** clause1 clause2 … [¶](07_05_33_srfi55_requiring_features.md)

Require the features of clause1 clause2 … , throwing an error if any are unavailable.

A clause is of the form `(identifier arg...)`. The only identifier currently supported is `srfi` and the arguments are SRFI numbers. For example to get SRFI-1 and SRFI-6,

(require-extension (srfi 1 6))

`require-extension` can only be used at the top-level.

A Guile-specific program can simply `use-modules` to load SRFIs not already in the core, `require-extension` is for programs designed to be portable to other Scheme implementations.

* * *

Next: [SRFI-61 - A more general `cond` clause](07_05_35_srfi61_a_more_general_cond_clause.md#7535-srfi-61---a-more-general-cond-clause), Previous: [SRFI-55 - Requiring Features](07_05_33_srfi55_requiring_features.md#7533-srfi-55---requiring-features), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
