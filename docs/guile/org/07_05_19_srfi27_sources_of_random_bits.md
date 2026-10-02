#### 7.5.19 SRFI-27 - Sources of Random Bits [¶](07_05_19_srfi27_sources_of_random_bits.md#7519-srfi-27---sources-of-random-bits)

This subsection is based on the [specification of SRFI-27](http://srfi.schemers.org/srfi-27/srfi-27.html) written by Sebastian Egner.

This SRFI provides access to a (pseudo) random number generator; for Guile’s built-in random number facilities, which SRFI-27 is implemented upon, See [Random Number Generation](06_06_02_numerical_data_types.md#66214-random-number-generation). With SRFI-27, random numbers are obtained from a _random source_, which encapsulates a random number generation algorithm and its state.

*   [The Default Random Source](07_05_19_srfi27_sources_of_random_bits.md#75191-the-default-random-source)
*   [Random Sources](07_05_19_srfi27_sources_of_random_bits.md#75192-random-sources)
*   [Obtaining random number generator procedures](07_05_19_srfi27_sources_of_random_bits.md#75193-obtaining-random-number-generator-procedures)

* * *

Next: [Random Sources](07_05_19_srfi27_sources_of_random_bits.md#75192-random-sources), Up: [SRFI-27 - Sources of Random Bits](07_05_19_srfi27_sources_of_random_bits.md#7519-srfi-27---sources-of-random-bits)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.19.1 The Default Random Source [¶](07_05_19_srfi27_sources_of_random_bits.md#75191-the-default-random-source)

Function: **random-integer** n [¶](07_05_19_srfi27_sources_of_random_bits.md)

Return a random number between zero (inclusive) and n (exclusive), using the default random source. The numbers returned have a uniform distribution.

Function: **random-real** [¶](07_05_19_srfi27_sources_of_random_bits.md)

Return a random number in (0,1), using the default random source. The numbers returned have a uniform distribution.

Function: **default-random-source** [¶](07_05_19_srfi27_sources_of_random_bits.md)

A random source from which `random-integer` and `random-real` have been derived using `random-source-make-integers` and `random-source-make-reals` (see [Obtaining random number generator procedures](07_05_19_srfi27_sources_of_random_bits.md#75193-obtaining-random-number-generator-procedures) for those procedures). Note that an assignment to `default-random-source` does not change `random-integer` or `random-real`; it is also strongly recommended not to assign a new value.

* * *

Next: [Obtaining random number generator procedures](07_05_19_srfi27_sources_of_random_bits.md#75193-obtaining-random-number-generator-procedures), Previous: [The Default Random Source](07_05_19_srfi27_sources_of_random_bits.md#75191-the-default-random-source), Up: [SRFI-27 - Sources of Random Bits](07_05_19_srfi27_sources_of_random_bits.md#7519-srfi-27---sources-of-random-bits)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.19.2 Random Sources [¶](07_05_19_srfi27_sources_of_random_bits.md#75192-random-sources)

Function: **make-random-source** [¶](07_05_19_srfi27_sources_of_random_bits.md)

Create a new random source. The stream of random numbers obtained from each random source created by this procedure will be identical, unless its state is changed by one of the procedures below.

Function: **random-source?** object [¶](07_05_19_srfi27_sources_of_random_bits.md)

Tests whether object is a random source. Random sources are a disjoint type.

Function: **random-source-randomize!** source [¶](07_05_19_srfi27_sources_of_random_bits.md)

Attempt to set the state of the random source to a truly random value. The current implementation uses a seed based on the current system time.

Function: **random-source-pseudo-randomize!** source i j [¶](07_05_19_srfi27_sources_of_random_bits.md)

Changes the state of the random source s into the initial state of the (i, j)-th independent random source, where i and j are non-negative integers. This procedure provides a mechanism to obtain a large number of independent random sources (usually all derived from the same backbone generator), indexed by two integers. In contrast to `random-source-randomize!`, this procedure is entirely deterministic.

The state associated with a random state can be obtained an reinstated with the following procedures:

Function: **random-source-state-ref** source [¶](07_05_19_srfi27_sources_of_random_bits.md)

Function: **random-source-state-set!** source state [¶](07_05_19_srfi27_sources_of_random_bits.md)

Get and set the state of a random source. No assumptions should be made about the nature of the state object, besides it having an external representation (i.e. it can be passed to `write` and subsequently `read` back).

* * *

Previous: [Random Sources](07_05_19_srfi27_sources_of_random_bits.md#75192-random-sources), Up: [SRFI-27 - Sources of Random Bits](07_05_19_srfi27_sources_of_random_bits.md#7519-srfi-27---sources-of-random-bits)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.19.3 Obtaining random number generator procedures [¶](07_05_19_srfi27_sources_of_random_bits.md#75193-obtaining-random-number-generator-procedures)

Function: **random-source-make-integers** source [¶](07_05_19_srfi27_sources_of_random_bits.md)

Obtains a procedure to generate random integers using the random source source. The returned procedure takes a single argument n, which must be a positive integer, and returns the next uniformly distributed random integer from the interval {0, ..., n\-1} by advancing the state of source.

If an application obtains and uses several generators for the same random source source, a call to any of these generators advances the state of source. Hence, the generators do not produce the same sequence of random integers each but rather share a state. This also holds for all other types of generators derived from a fixed random sources.

While the SRFI text specifies that “Implementations that support concurrency make sure that the state of a generator is properly advanced”, this is currently not the case in Guile’s implementation of SRFI-27, as it would cause a severe performance penalty. So in multi-threaded programs, you either must perform locking on random sources shared between threads yourself, or use different random sources for multiple threads.

Function: **random-source-make-reals** source [¶](07_05_19_srfi27_sources_of_random_bits.md)

Function: **random-source-make-reals** source unit [¶](07_05_19_srfi27_sources_of_random_bits.md)

Obtains a procedure to generate random real numbers _0 < x < 1_ using the random source source. The procedure rand is called without arguments.

The optional parameter unit determines the type of numbers being produced by the returned procedure and the quantization of the output. unit must be a number such that _0 < unit < 1_. The numbers created by the returned procedure are of the same numerical type as unit and the potential output values are spaced by at most unit. One can imagine rand to create numbers as x \* unit where x is a random integer in {1, ..., floor(1/unit)-1}. Note, however, that this need not be the way the values are actually created and that the actual resolution of rand can be much higher than unit. In case unit is absent it defaults to a reasonably small value (related to the width of the mantissa of an efficient number format).

* * *

Next: [SRFI-30 - Nested Multi-line Comments](07_05_21_srfi30_nested_multiline_comments.md#7521-srfi-30---nested-multi-line-comments), Previous: [SRFI-27 - Sources of Random Bits](07_05_19_srfi27_sources_of_random_bits.md#7519-srfi-27---sources-of-random-bits), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
