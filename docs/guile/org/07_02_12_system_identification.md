#### 7.2.12 System Identification [¶](07_02_12_system_identification.md#7212-system-identification)

This section lists the various procedures Guile provides for accessing information about the system it runs on.

Scheme Procedure: **uname** [¶](07_02_12_system_identification.md)

C Function: **scm\_uname** () [¶](07_02_12_system_identification.md)

Return an object with some information about the computer system the program is running on.

The following procedures accept an object as returned by `uname` and return a selected component (all of which are strings).

Scheme Procedure: **utsname:sysname** un [¶](07_02_12_system_identification.md)

The name of the operating system.

Scheme Procedure: **utsname:nodename** un [¶](07_02_12_system_identification.md)

The network name of the computer.

Scheme Procedure: **utsname:release** un [¶](07_02_12_system_identification.md)

The current release level of the operating system implementation.

Scheme Procedure: **utsname:version** un [¶](07_02_12_system_identification.md)

The current version level within the release of the operating system.

Scheme Procedure: **utsname:machine** un [¶](07_02_12_system_identification.md)

A description of the hardware.

Scheme Procedure: **gethostname** [¶](07_02_12_system_identification.md)

C Function: **scm\_gethostname** () [¶](07_02_12_system_identification.md)

Return the host name of the current processor.

Scheme Procedure: **sethostname** name [¶](07_02_12_system_identification.md)

C Function: **scm\_sethostname** (name) [¶](07_02_12_system_identification.md)

Set the host name of the current processor to name. May only be used by the superuser. The return value is not specified.

* * *

Next: [Encryption](07_02_14_encryption.md#7214-encryption), Previous: [System Identification](07_02_12_system_identification.md#7212-system-identification), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
