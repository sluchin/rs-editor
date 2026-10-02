### 7.20 Statprof [¶](07_20_statprof.md#720-statprof)

Statprof is a statistical profiler for Guile.

A simple use of statprof would look like this:

(use-modules (statprof))
(statprof (lambda ()
            (map 1+ (iota 1000000))
            #f))

This would run the thunk with statistical profiling, finally displaying a flat table of statistics which could look something like this:

%     cumulative   self
time   seconds     seconds  procedure
 57.14  39769.73      0.07  ice-9/boot-9.scm:249:5:map1
 28.57      0.04      0.04  ice-9/boot-9.scm:1165:0:iota
 14.29      0.02      0.02  1+
  0.00      0.12      0.00  <current input>:2:10
---
Sample count: 7
Total time: 0.123490713 seconds (0.201983993 seconds in GC)

All of the numerical data with the exception of the calls column is statistically approximate. In the following column descriptions, and in all of statprof, “time” refers to execution time (both user and system), not wall clock time.

The `% time` column indicates the percentage of the run-time time spent inside the procedure itself (not counting children). It is calculated as `self seconds`, measuring the amount of time spent in the procedure, divided by the total run-time.

`cumulative seconds` also counts time spent in children of a function. For recursive functions, this can exceed the total time, as in our example above, because each activation on the stack adds to the cumulative time.

Finally, the GC time measures the time spent in the garbage collector. On systems with multiple cores, this time can be larger than the run time, because it counts time spent in all threads, and will run the “marking” phase of GC in parallel. If GC time is a significant fraction of the run time, that means that most time in your program is spent allocating objects and cleaning up after those allocations. To speed up your program, one good place to start would be to look at how to reduce the allocation rate.

Statprof’s main mode of operation is as a statistical profiler. However statprof can also run in a “precise” mode as well. Pass the `#:count-calls? #t` keyword argument to `statprof` to record all calls:

(use-modules (statprof))
(statprof (lambda ()
            (map 1+ (iota 1000000))
            #f)
          #:count-calls? #t)

The result has an additional `calls` column:

%     cumulative   self             
time   seconds    seconds   calls   procedure
 82.26      0.73      0.73 1000000  1+
 11.29 420925.80      0.10 1000001  ice-9/boot-9.scm:249:5:map1
  4.84      0.06      0.04       1  ice-9/boot-9.scm:1165:0:iota
\[...\]
---
Sample count: 62
Total time: 0.893098065 seconds (1.222796536 seconds in GC)

As you can see, the profile is perturbed: `1+` ends up on top, whereas it was not marked as hot in the earlier profile. This is because the overhead of call-counting unfairly penalizes calls. Still, this precise mode can be useful at times to do algorithmic optimizations based on the precise call counts.

### Implementation notes [¶](07_20_statprof.md#implementation-notes)

The profiler works by setting the unix profiling signal `ITIMER_PROF` to go off after the interval you define in the call to `statprof-reset`. When the signal fires, a sampling routine runs which crawls up the stack, recording all instruction pointers into a buffer. After the sample is complete, the profiler resets profiling timer to fire again after the appropriate interval.

Later, when profiling stops, that log buffer is analyzed to produce the “self seconds” and “cumulative seconds” statistics. A procedure at the top of the stack counts toward “self” samples, and everything on the stack counts towards “cumulative” samples.

While the profiler is running it measures how much CPU time (system and user – which is also what `ITIMER_PROF` tracks) has elapsed while code has been executing within the profiler. Only run time counts towards the profile, not wall-clock time. For example, sleeping and waiting for input or output do not cause the timer clock to advance.

### Usage [¶](07_20_statprof.md#usage)

Scheme Procedure: **statprof** thunk \[#:loop loop=1\] \[#:hz hz=100\] \[#:port port=(current-output-port)\] \[#:count-calls? count-calls?=#f\] \[#:display-style display-style=’flat\] [¶](07_20_statprof.md)

Profile the execution of thunk, and return its return values.

The stack will be sampled hz times per second, and the thunk itself will be called loop times.

If count-calls? is true, all procedure calls will be recorded. This operation is somewhat expensive.

After the thunk has been profiled, print out a profile to port. If display-style is `flat`, the results will be printed as a flat profile. Otherwise if display-style is `tree`, print the results as a tree profile.

Note that `statprof` requires a working profiling timer. Some platforms do not support profiling timers. `(provided? 'ITIMER_PROF)` can be used to check for support of profiling timers.

Profiling can also be enabled and disabled manually.

Scheme Procedure: **statprof-active?** [¶](07_20_statprof.md)

Returns `#t` if `statprof-start` has been called more times than `statprof-stop`, `#f` otherwise.

Scheme Procedure: **statprof-start** [¶](07_20_statprof.md)

Scheme Procedure: **statprof-stop** [¶](07_20_statprof.md)

Start or stop the profiler.

Scheme Procedure: **statprof-reset** sample-seconds sample-microseconds count-calls? [¶](07_20_statprof.md)

Reset the profiling sample interval to sample-seconds and sample-microseconds. If count-calls? is true, arrange to instrument procedure calls as well as collecting statistical profiling data.

If you use the manual `statprof-start`/`statprof-stop` interface, an implicit statprof state will persist starting from the last call to `statprof-reset`, or the first call to `statprof-start`. There are a number of accessors to fetch statistics from this implicit state.

Scheme Procedure: **statprof-accumulated-time** [¶](07_20_statprof.md)

Returns the time accumulated during the last statprof run.

Scheme Procedure: **statprof-sample-count** [¶](07_20_statprof.md)

Returns the number of samples taken during the last statprof run.

Scheme Procedure: **statprof-fold-call-data** proc init [¶](07_20_statprof.md)

Fold proc over the call-data accumulated by statprof. This procedure cannot be called while statprof is active.

proc will be called with arguments, call-data and prior-result.

Scheme Procedure: **statprof-proc-call-data** proc [¶](07_20_statprof.md)

Returns the call-data associated with proc, or `#f` if none is available.

Scheme Procedure: **statprof-call-data-name** cd [¶](07_20_statprof.md)

Scheme Procedure: **statprof-call-data-calls** cd [¶](07_20_statprof.md)

Scheme Procedure: **statprof-call-data-cum-samples** cd [¶](07_20_statprof.md)

Scheme Procedure: **statprof-call-data-self-samples** cd [¶](07_20_statprof.md)

Accessors for the fields in a statprof call-data object.

Scheme Procedure: **statprof-call-data->stats** call-data [¶](07_20_statprof.md)

Returns an object of type `statprof-stats`.

Scheme Procedure: **statprof-stats-proc-name** stats [¶](07_20_statprof.md)

Scheme Procedure: **statprof-stats-%-time-in-proc** stats [¶](07_20_statprof.md)

Scheme Procedure: **statprof-stats-cum-secs-in-proc** stats [¶](07_20_statprof.md)

Scheme Procedure: **statprof-stats-self-secs-in-proc** stats [¶](07_20_statprof.md)

Scheme Procedure: **statprof-stats-calls** stats [¶](07_20_statprof.md)

Scheme Procedure: **statprof-stats-self-secs-per-call** stats [¶](07_20_statprof.md)

Scheme Procedure: **statprof-stats-cum-secs-per-call** stats [¶](07_20_statprof.md)

Accessors for the fields in a `statprof-stats` object.

Scheme Procedure: **statprof-display** \[port=(current-output-port)\] \[#:style style=flat\] [¶](07_20_statprof.md)

Displays a summary of the statistics collected. Possible values for style include:

`flat`

Display a traditional gprof-style flat profile.

`anomalies`

Find statistical anomalies in the data.

`tree`

Display a tree profile.

Scheme Procedure: **statprof-fetch-stacks** [¶](07_20_statprof.md)

Returns a list of stacks, as they were captured since the last call to `statprof-reset`.

Scheme Procedure: **statprof-fetch-call-tree** \[#:precise precise?=#f\] [¶](07_20_statprof.md)

Return a call tree for the previous statprof run.

The return value is a list of nodes.  A node is a list of the form:
@code
 node ::= (@var{proc} @var{count} . @var{nodes})
@end code

The @var{proc} is a printable representation of a procedure, as a
string.  If @var{precise?} is false, which is the default, then a node
corresponds to a procedure invocation.  If it is true, then a node
corresponds to a return point in a procedure.  Passing @code{#:precise?
#t} allows a user to distinguish different source lines in a procedure,
but usually it is too much detail, so it is off by default.

Scheme Procedure: **gcprof** thunk \[#:loop\] [¶](07_20_statprof.md)

Like the `statprof` procedure, but instead of profiling CPU time, we profile garbage collection.

The stack will be sampled soon after every garbage collection during the evaluation of thunk, yielding an approximate idea of what is causing allocation in your program.

Since GC does not occur very frequently, you may need to use the loop parameter, to cause thunk to be called loop times.

* * *

Next: [Texinfo Processing](07_22_texinfo_processing.md#722-texinfo-processing), Previous: [Statprof](07_20_statprof.md#720-statprof), Up: [Guile Modules](07_00_guile_modules.md#7-guile-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
