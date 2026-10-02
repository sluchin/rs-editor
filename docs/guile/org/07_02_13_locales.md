#### 7.2.13 Locales [¶](07_02_13_locales.md#7213-locales)

Scheme Procedure: **setlocale** category \[locale\] [¶](07_02_13_locales.md)

C Function: **scm\_setlocale** (category, locale) [¶](07_02_13_locales.md)

Get or set the current locale, used for various internationalizations. Locales are strings, such as ‘sv\_SE’.

If locale is given then the locale for the given category is set and the new value returned. If locale is not given then the current value is returned. category should be one of the following values (see [Categories of Activities that Locales Affect](https://doc.guix.gnu.org/libc/latest/en/libc.html#Locale-Categories) in The GNU C Library Reference Manual):

Variable: **LC\_ALL** [¶](07_02_13_locales.md)

Variable: **LC\_COLLATE** [¶](07_02_13_locales.md)

Variable: **LC\_CTYPE** [¶](07_02_13_locales.md)

Variable: **LC\_MESSAGES** [¶](07_02_13_locales.md)

Variable: **LC\_MONETARY** [¶](07_02_13_locales.md)

Variable: **LC\_NUMERIC** [¶](07_02_13_locales.md)

Variable: **LC\_TIME** [¶](07_02_13_locales.md)

A common usage is ‘(setlocale LC\_ALL "")’, which initializes all categories based on standard environment variables (`LANG` etc). For full details on categories and locale names see [Locales and Internationalization](https://doc.guix.gnu.org/libc/latest/en/libc.html#Locales) in The GNU C Library Reference Manual.

Note that `setlocale` affects locale settings for the whole process. See [locale objects and `make-locale`](06_25_support_for_internationalization.md#6251-internationalization-with-guile), for a thread-safe alternative.

A `system-error` exception (see [How to Handle Errors](06_11_controlling_the_flow_of_program_execution.md#61113-how-to-handle-errors)) is raised by `setlocale` when locale-name does not match any of the locales compiled on the system.

* * *

Previous: [Locales](07_02_13_locales.md#7213-locales), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
