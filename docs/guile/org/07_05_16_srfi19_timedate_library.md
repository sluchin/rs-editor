#### 7.5.16 SRFI-19 - Time/Date Library [¶](07_05_16_srfi19_timedate_library.md#7516-srfi-19---timedate-library)

This is an implementation of the SRFI-19 time/date library. The functions and variables described here are provided by

(use-modules (srfi srfi-19))

*   [SRFI-19 Introduction](07_05_16_srfi19_timedate_library.md#75161-srfi-19-introduction)
*   [SRFI-19 Time](07_05_16_srfi19_timedate_library.md#75162-srfi-19-time)
*   [SRFI-19 Date](07_05_16_srfi19_timedate_library.md#75163-srfi-19-date)
*   [SRFI-19 Time/Date conversions](07_05_16_srfi19_timedate_library.md#75164-srfi-19-timedate-conversions)
*   [SRFI-19 Date to string](07_05_16_srfi19_timedate_library.md#75165-srfi-19-date-to-string)
*   [SRFI-19 String to date](07_05_16_srfi19_timedate_library.md#75166-srfi-19-string-to-date)

* * *

Next: [SRFI-19 Time](07_05_16_srfi19_timedate_library.md#75162-srfi-19-time), Up: [SRFI-19 - Time/Date Library](07_05_16_srfi19_timedate_library.md#7516-srfi-19---timedate-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.16.1 SRFI-19 Introduction [¶](07_05_16_srfi19_timedate_library.md#75161-srfi-19-introduction)

This module implements time and date representations and calculations, in various time systems, including Coordinated Universal Time (UTC) and International Atomic Time (TAI).

For those not familiar with these time systems, TAI is based on a fixed length second derived from oscillations of certain atoms. UTC differs from TAI by an integral number of seconds, which is increased or decreased at announced times to keep UTC aligned to a mean solar day (the orbit and rotation of the earth are not quite constant).

So far, only increases in the TAI <-> UTC difference have been needed. Such an increase is a “leap second”, an extra second of TAI introduced at the end of a UTC day. When working entirely within UTC this is never seen, every day simply has 86400 seconds. But when converting from TAI to a UTC date, an extra 23:59:60 is present, where normally a day would end at 23:59:59. Effectively the UTC second from 23:59:59 to 00:00:00 has taken two TAI seconds.

In the current implementation, the system clock is assumed to be UTC, and a table of leap seconds in the code converts to TAI. See comments in srfi-19.scm for how to update this table.

Also, for those not familiar with the terminology, a _Julian Day_ represents a point in time as a real number of days since -4713-11-24T12:00:00Z, i.e. midday UT on 24 November 4714 BC in the proleptic Gregorian calendar (1 January 4713 BC in the proleptic Julian calendar).

A _Modified Julian Day_ represents a point in time as a real number of days since 1858-11-17T00:00:00Z, i.e. midnight UT on Wednesday 17 November AD 1858. That time is julian day 2400000.5.

* * *

Next: [SRFI-19 Date](07_05_16_srfi19_timedate_library.md#75163-srfi-19-date), Previous: [SRFI-19 Introduction](07_05_16_srfi19_timedate_library.md#75161-srfi-19-introduction), Up: [SRFI-19 - Time/Date Library](07_05_16_srfi19_timedate_library.md#7516-srfi-19---timedate-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.16.2 SRFI-19 Time [¶](07_05_16_srfi19_timedate_library.md#75162-srfi-19-time)

A _time_ object has type, seconds and nanoseconds fields representing a point in time starting from some epoch. This is an arbitrary point in time, not just a time of day. Although times are represented in nanoseconds, the actual resolution may be lower.

The following variables hold the possible time types. For instance `(current-time time-process)` would give the current CPU process time.

Variable: **time-utc** [¶](07_05_16_srfi19_timedate_library.md)

Universal Coordinated Time (UTC).

Variable: **time-tai** [¶](07_05_16_srfi19_timedate_library.md)

International Atomic Time (TAI).

Variable: **time-monotonic** [¶](07_05_16_srfi19_timedate_library.md)

Monotonic time, meaning a monotonically increasing time starting from an unspecified epoch.

Note that in the current implementation `time-monotonic` is the same as `time-tai`, and unfortunately is therefore affected by adjustments to the system clock. Perhaps this will change in the future.

Variable: **time-duration** [¶](07_05_16_srfi19_timedate_library.md)

A duration, meaning simply a difference between two times.

Variable: **time-process** [¶](07_05_16_srfi19_timedate_library.md)

CPU time spent in the current process, starting from when the process began.

Variable: **time-thread** [¶](07_05_16_srfi19_timedate_library.md)

CPU time spent in the current thread. Not currently implemented.

  

Function: **time?** obj [¶](07_05_16_srfi19_timedate_library.md)

Return `#t` if obj is a time object, or `#f` if not.

Function: **make-time** type nanoseconds seconds [¶](07_05_16_srfi19_timedate_library.md)

Create a time object with the given type, seconds and nanoseconds.

Function: **time-type** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-nanosecond** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-second** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **set-time-type!** time type [¶](07_05_16_srfi19_timedate_library.md)

Function: **set-time-nanosecond!** time nsec [¶](07_05_16_srfi19_timedate_library.md)

Function: **set-time-second!** time sec [¶](07_05_16_srfi19_timedate_library.md)

Get or set the type, seconds or nanoseconds fields of a time object.

`set-time-type!` merely changes the field, it doesn’t convert the time value. For conversions, see [SRFI-19 Time/Date conversions](07_05_16_srfi19_timedate_library.md#75164-srfi-19-timedate-conversions).

Function: **copy-time** time [¶](07_05_16_srfi19_timedate_library.md)

Return a new time object, which is a copy of the given time.

Function: **current-time** \[type\] [¶](07_05_16_srfi19_timedate_library.md)

Return the current time of the given type. The default type is `time-utc`.

Note that the name `current-time` conflicts with the Guile core `current-time` function (see [Time](07_02_05_time.md#725-time)) as well as the SRFI-18 `current-time` function (see [SRFI-18 Time](07_05_15_srfi18_multithreading_support.md#75154-srfi-18-time)). Applications wanting to use more than one of these functions will need to refer to them by different names.

Function: **time-resolution** \[type\] [¶](07_05_16_srfi19_timedate_library.md)

Return the resolution, in nanoseconds, of the given time type. The default type is `time-utc`.

Function: **time<=?** t1 t2 [¶](07_05_16_srfi19_timedate_library.md)

Function: **time<?** t1 t2 [¶](07_05_16_srfi19_timedate_library.md)

Function: **time=?** t1 t2 [¶](07_05_16_srfi19_timedate_library.md)

Function: **time>=?** t1 t2 [¶](07_05_16_srfi19_timedate_library.md)

Function: **time>?** t1 t2 [¶](07_05_16_srfi19_timedate_library.md)

Return `#t` or `#f` according to the respective relation between time objects t1 and t2. t1 and t2 must be the same time type.

Function: **time-difference** t1 t2 [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-difference!** t1 t2 [¶](07_05_16_srfi19_timedate_library.md)

Return a time object of type `time-duration` representing the period between t1 and t2. t1 and t2 must be the same time type.

`time-difference` returns a new time object, `time-difference!` may modify t1 to form its return.

Function: **add-duration** time duration [¶](07_05_16_srfi19_timedate_library.md)

Function: **add-duration!** time duration [¶](07_05_16_srfi19_timedate_library.md)

Function: **subtract-duration** time duration [¶](07_05_16_srfi19_timedate_library.md)

Function: **subtract-duration!** time duration [¶](07_05_16_srfi19_timedate_library.md)

Return a time object which is time with the given duration added or subtracted. duration must be a time object of type `time-duration`.

`add-duration` and `subtract-duration` return a new time object. `add-duration!` and `subtract-duration!` may modify the given time to form their return.

* * *

Next: [SRFI-19 Time/Date conversions](07_05_16_srfi19_timedate_library.md#75164-srfi-19-timedate-conversions), Previous: [SRFI-19 Time](07_05_16_srfi19_timedate_library.md#75162-srfi-19-time), Up: [SRFI-19 - Time/Date Library](07_05_16_srfi19_timedate_library.md#7516-srfi-19---timedate-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.16.3 SRFI-19 Date [¶](07_05_16_srfi19_timedate_library.md#75163-srfi-19-date)

A _date_ object represents a date in the Gregorian calendar and a time of day on that date in some timezone.

The fields are year, month, day, hour, minute, second, nanoseconds and timezone. A date object is immutable, its fields can be read but they cannot be modified once the object is created.

Historically, the Gregorian calendar was only used from the latter part of the year 1582 onwards, and not until even later in many countries. Prior to that most countries used the Julian calendar. SRFI-19 does not deal with the Julian calendar at all, and so does not reflect this historical calendar reform. Instead it projects the Gregorian calendar back proleptically as far as necessary. When dealing with historical data, especially prior to the British Empire’s adoption of the Gregorian calendar in 1752, one should be mindful of which calendar is used in each context, and apply non-SRFI-19 facilities to convert where necessary.

Function: **date?** obj [¶](07_05_16_srfi19_timedate_library.md)

Return `#t` if obj is a date object, or `#f` if not.

Function: **make-date** nsecs seconds minutes hours day month year zone-offset [¶](07_05_16_srfi19_timedate_library.md)

Create a new date object.

Function: **date-nanosecond** date [¶](07_05_16_srfi19_timedate_library.md)

Nanoseconds, 0 to 999999999.

Function: **date-second** date [¶](07_05_16_srfi19_timedate_library.md)

Seconds, 0 to 59, or 60 for a leap second. 60 is never seen when working entirely within UTC, it’s only when converting to or from TAI.

Function: **date-minute** date [¶](07_05_16_srfi19_timedate_library.md)

Minutes, 0 to 59.

Function: **date-hour** date [¶](07_05_16_srfi19_timedate_library.md)

Hour, 0 to 23.

Function: **date-day** date [¶](07_05_16_srfi19_timedate_library.md)

Day of the month, 1 to 31 (or less, according to the month).

Function: **date-month** date [¶](07_05_16_srfi19_timedate_library.md)

Month, 1 to 12.

Function: **date-year** date [¶](07_05_16_srfi19_timedate_library.md)

Year, eg. 2003. Dates B.C. are negative, eg. _\-46_ is 46 B.C. There is no year 0, year _\-1_ is followed by year 1.

Function: **date-zone-offset** date [¶](07_05_16_srfi19_timedate_library.md)

Time zone, an integer number of seconds east of Greenwich.

Function: **date-year-day** date [¶](07_05_16_srfi19_timedate_library.md)

Day of the year, starting from 1 for 1st January.

Function: **date-week-day** date [¶](07_05_16_srfi19_timedate_library.md)

Day of the week, starting from 0 for Sunday.

Function: **date-week-number** date dstartw [¶](07_05_16_srfi19_timedate_library.md)

Week of the year, ignoring a first partial week. dstartw is the day of the week which is taken to start a week, 0 for Sunday, 1 for Monday, etc.

Function: **current-date** \[tz-offset\] [¶](07_05_16_srfi19_timedate_library.md)

Return a date object representing the current date/time, in UTC offset by tz-offset. tz-offset is seconds east of Greenwich and defaults to the local timezone.

Function: **current-julian-day** [¶](07_05_16_srfi19_timedate_library.md)

Return the current Julian Day.

Function: **current-modified-julian-day** [¶](07_05_16_srfi19_timedate_library.md)

Return the current Modified Julian Day.

* * *

Next: [SRFI-19 Date to string](07_05_16_srfi19_timedate_library.md#75165-srfi-19-date-to-string), Previous: [SRFI-19 Date](07_05_16_srfi19_timedate_library.md#75163-srfi-19-date), Up: [SRFI-19 - Time/Date Library](07_05_16_srfi19_timedate_library.md#7516-srfi-19---timedate-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.16.4 SRFI-19 Time/Date conversions [¶](07_05_16_srfi19_timedate_library.md#75164-srfi-19-timedate-conversions)

Function: **date->julian-day** date [¶](07_05_16_srfi19_timedate_library.md)

Function: **date->modified-julian-day** date [¶](07_05_16_srfi19_timedate_library.md)

Function: **date->time-monotonic** date [¶](07_05_16_srfi19_timedate_library.md)

Function: **date->time-tai** date [¶](07_05_16_srfi19_timedate_library.md)

Function: **date->time-utc** date [¶](07_05_16_srfi19_timedate_library.md)

Function: **julian-day->date** jdn \[tz-offset\] [¶](07_05_16_srfi19_timedate_library.md)

Function: **julian-day->time-monotonic** jdn [¶](07_05_16_srfi19_timedate_library.md)

Function: **julian-day->time-tai** jdn [¶](07_05_16_srfi19_timedate_library.md)

Function: **julian-day->time-utc** jdn [¶](07_05_16_srfi19_timedate_library.md)

Function: **modified-julian-day->date** jdn \[tz-offset\] [¶](07_05_16_srfi19_timedate_library.md)

Function: **modified-julian-day->time-monotonic** jdn [¶](07_05_16_srfi19_timedate_library.md)

Function: **modified-julian-day->time-tai** jdn [¶](07_05_16_srfi19_timedate_library.md)

Function: **modified-julian-day->time-utc** jdn [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-monotonic->date** time \[tz-offset\] [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-monotonic->time-tai** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-monotonic->time-tai!** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-monotonic->time-utc** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-monotonic->time-utc!** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-tai->date** time \[tz-offset\] [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-tai->julian-day** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-tai->modified-julian-day** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-tai->time-monotonic** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-tai->time-monotonic!** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-tai->time-utc** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-tai->time-utc!** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-utc->date** time \[tz-offset\] [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-utc->julian-day** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-utc->modified-julian-day** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-utc->time-monotonic** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-utc->time-monotonic!** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-utc->time-tai** time [¶](07_05_16_srfi19_timedate_library.md)

Function: **time-utc->time-tai!** time [¶](07_05_16_srfi19_timedate_library.md)

  

Convert between dates, times and days of the respective types. For instance `time-tai->time-utc` accepts a time object of type `time-tai` and returns an object of type `time-utc`.

The `!` variants may modify their time argument to form their return. The plain functions create a new object.

For conversions to dates, tz-offset is seconds east of Greenwich. The default is the local timezone, at the given time, as provided by the system, using `localtime` (see [Time](07_02_05_time.md#725-time)).

On 32-bit systems, `localtime` is limited to a 32-bit `time_t`, so a default tz-offset is only available for times between Dec 1901 and Jan 2038. For prior dates an application might like to use the value in 1902, though some locations have zone changes prior to that. For future dates an application might like to assume today’s rules extend indefinitely. But for correct daylight savings transitions it will be necessary to take an offset for the same day and time but a year in range and which has the same starting weekday and same leap/non-leap (to support rules like last Sunday in October).

* * *

Next: [SRFI-19 String to date](07_05_16_srfi19_timedate_library.md#75166-srfi-19-string-to-date), Previous: [SRFI-19 Time/Date conversions](07_05_16_srfi19_timedate_library.md#75164-srfi-19-timedate-conversions), Up: [SRFI-19 - Time/Date Library](07_05_16_srfi19_timedate_library.md#7516-srfi-19---timedate-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.16.5 SRFI-19 Date to string [¶](07_05_16_srfi19_timedate_library.md#75165-srfi-19-date-to-string)

Function: **date->string** date \[format\] [¶](07_05_16_srfi19_timedate_library.md)

Convert a date to a string under the control of a format. format should be a string containing ‘~’ escapes, which will be expanded as per the following conversion table. The default format is ‘~c’, a locale-dependent date and time.

Many of these conversion characters are the same as POSIX `strftime` (see [Time](07_02_05_time.md#725-time)), but there are some extras and some variations.

`~~`

literal ~

`~a`

locale abbreviated weekday, eg. ‘Sun’

`~A`

locale full weekday, eg. ‘Sunday’

`~b`

locale abbreviated month, eg. ‘Jan’

`~B`

locale full month, eg. ‘January’

`~c`

locale date and time, eg.  
‘Fri Jul 14 20:28:42-0400 2000’

`~d`

day of month, zero padded, ‘01’ to ‘31’

`~e`

day of month, blank padded, ‘ 1’ to ‘31’

`~f`

seconds and fractional seconds, with locale decimal point, eg. ‘5.2’

`~h`

same as `~b`

`~H`

hour, 24-hour clock, zero padded, ‘00’ to ‘23’

`~I`

hour, 12-hour clock, zero padded, ‘01’ to ‘12’

`~j`

day of year, zero padded, ‘001’ to ‘366’

`~k`

hour, 24-hour clock, blank padded, ‘ 0’ to ‘23’

`~l`

hour, 12-hour clock, blank padded, ‘ 1’ to ‘12’

`~m`

month, zero padded, ‘01’ to ‘12’

`~M`

minute, zero padded, ‘00’ to ‘59’

`~n`

newline

`~N`

nanosecond, zero padded, ‘000000000’ to ‘999999999’

`~p`

locale AM or PM

`~r`

time, 12 hour clock, ‘~I:~M:~S ~p’

`~s`

number of full seconds since “the epoch” in UTC

`~S`

second, zero padded ‘00’ to ‘60’  
(usual limit is 59, 60 is a leap second)

`~t`

horizontal tab character

`~T`

time, 24 hour clock, ‘~H:~M:~S’

`~U`

week of year, Sunday first day of week, ‘00’ to ‘52’

`~V`

ISO 8601 week number of the year, Monday first day of week, ‘01’ to ‘53’

`~w`

day of week, 0 for Sunday, ‘0’ to ‘6’

`~W`

week of year, Monday first day of week, ‘00’ to ‘52’

`~y`

year, two digits, ‘00’ to ‘99’

`~Y`

year, full, eg. ‘2003’

`~z`

time zone, RFC-822 style

`~Z`

time zone symbol (not currently implemented)

`~1`

ISO-8601 date, ‘~Y-~m-~d’

`~2`

ISO-8601 time+zone, ‘~H:~M:~S~z’

`~3`

ISO-8601 time, ‘~H:~M:~S’

`~4`

ISO-8601 date/time+zone, ‘~Y-~m-~dT~H:~M:~S~z’

`~5`

ISO-8601 date/time, ‘~Y-~m-~dT~H:~M:~S’

Conversions ‘~D’, ‘~x’ and ‘~X’ are not currently described here, since the specification and reference implementation differ.

Conversion is locale-dependent on systems that support it (see [Accessing Locale Information](06_25_support_for_internationalization.md#6255-accessing-locale-information)). See [`setlocale`](07_02_13_locales.md#7213-locales), for information on how to change the current locale.

* * *

Previous: [SRFI-19 Date to string](07_05_16_srfi19_timedate_library.md#75165-srfi-19-date-to-string), Up: [SRFI-19 - Time/Date Library](07_05_16_srfi19_timedate_library.md#7516-srfi-19---timedate-library)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.5.16.6 SRFI-19 String to date [¶](07_05_16_srfi19_timedate_library.md#75166-srfi-19-string-to-date)

Function: **string->date** input template [¶](07_05_16_srfi19_timedate_library.md)

Convert an input string to a date under the control of a template string. Return a newly created date object.

Literal characters in template must match characters in input and ‘~’ escapes must match the input forms described in the table below. “Skip to” means characters up to one of the given type are ignored, or “no skip” for no skipping. “Read” is what’s then read, and “Set” is the field affected in the date object.

For example ‘~Y’ skips input characters until a digit is reached, at which point it expects a year and stores that to the year field of the date.

Skip to

Read

Set

`~~`

no skip

literal ~

nothing

`~a`

`char-alphabetic?`

locale abbreviated weekday name

nothing

`~A`

`char-alphabetic?`

locale full weekday name

nothing

`~b`

`char-alphabetic?`

locale abbreviated month name

`date-month`

`~B`

`char-alphabetic?`

locale full month name

`date-month`

`~d`

`char-numeric?`

day of month

`date-day`

`~e`

no skip

day of month, blank padded

`date-day`

`~h`

same as ‘~b’

`~H`

`char-numeric?`

hour

`date-hour`

`~k`

no skip

hour, blank padded

`date-hour`

`~m`

`char-numeric?`

month

`date-month`

`~M`

`char-numeric?`

minute

`date-minute`

`~N`

`char-numeric?`

nanosecond

`date-nanosecond`

`~S`

`char-numeric?`

second

`date-second`

`~y`

no skip

2-digit year

`date-year` within 50 years

`~Y`

`char-numeric?`

year

`date-year`

`~z`

no skip

time zone

date-zone-offset

Notice that the weekday matching forms don’t affect the date object returned, instead the weekday will be derived from the day, month and year.

Conversion is locale-dependent on systems that support it (see [Accessing Locale Information](06_25_support_for_internationalization.md#6255-accessing-locale-information)). See [`setlocale`](07_02_13_locales.md#7213-locales), for information on how to change the current locale.

* * *

Next: [SRFI-26 - specializing parameters](07_05_18_srfi26_specializing_parameters.md#7518-srfi-26---specializing-parameters), Previous: [SRFI-19 - Time/Date Library](07_05_16_srfi19_timedate_library.md#7516-srfi-19---timedate-library), Up: [SRFI Support Modules](07_05_00_srfi_support_modules.md#75-srfi-support-modules)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
