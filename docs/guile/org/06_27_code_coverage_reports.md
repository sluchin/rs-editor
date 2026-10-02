### 6.27 Code Coverage Reports [¶](06_27_code_coverage_reports.md#627-code-coverage-reports)

When writing a test suite for a program or library, it is desirable to know what part of the code is _covered_ by the test suite. The `(system vm coverage)` module provides tools to gather code coverage data and to present them, as detailed below.

Scheme Procedure: **with-code-coverage** thunk [¶](06_27_code_coverage_reports.md)

Run thunk, a zero-argument procedure, while instrumenting Guile’s virtual machine to collect code coverage data. Return code coverage data and the values returned by thunk.

Scheme Procedure: **coverage-data?** obj [¶](06_27_code_coverage_reports.md)

Return `#t` if obj is a _coverage data_ object as returned by `with-code-coverage`.

Scheme Procedure: **coverage-data->lcov** data port #:key modules [¶](06_27_code_coverage_reports.md)

Traverse code coverage information data, as obtained with `with-code-coverage`, and write coverage information to port in the `.info` format used by [LCOV](http://ltp.sourceforge.net/coverage/lcov.php). The report will include all of modules (or, by default, all the currently loaded modules) even if their code was not executed.

The generated data can be fed to LCOV’s `genhtml` command to produce an HTML report, which aids coverage data visualization.

Here’s an example use:

(use-modules (system vm coverage)
             (system vm vm))

(call-with-values (lambda ()
                    (with-code-coverage
                      (lambda ()
                        (do-something-tricky))))
  (lambda (data result)
    (let ((port (open-output-file "lcov.info")))
      (coverage-data->lcov data port)
      (close port))))

In addition, the module provides low-level procedures that would make it possible to write other user interfaces to the coverage data.

Scheme Procedures: **instrumented-source-files** data [¶](06_27_code_coverage_reports.md)

Return the list of “instrumented” source files, i.e., source files whose code was loaded at the time data was collected.

Scheme Procedures: **line-execution-counts** data file [¶](06_27_code_coverage_reports.md)

Return a list of line number/execution count pairs for file, or `#f` if file is not among the files covered by data. This includes lines with zero count.

Scheme Procedures: **instrumented/executed-lines** data file [¶](06_27_code_coverage_reports.md)

Return the number of instrumented and the number of executed source lines in file according to data.

Scheme Procedures: **procedure-execution-count** data proc [¶](06_27_code_coverage_reports.md)

Return the number of times proc’s code was executed, according to data, or `#f` if proc was not executed. When proc is a closure, the number of times its code was executed is returned, not the number of times this code associated with this particular closure was executed.

* * *

Next: [GOOPS](08_00_goops.md#8-goops), Previous: [API Reference](06_00_api_reference.md#6-api-reference), Up: [The Guile Reference Manual](00_contents.md)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
