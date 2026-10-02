#### 7.2.4 User Information [¶](07_02_04_user_information.md#724-user-information)

The facilities in this section provide an interface to the user and group database. They should be used with care since they are not reentrant.

The following functions accept an object representing user information and return a selected component:

Scheme Procedure: **passwd:name** pw [¶](07_02_04_user_information.md)

The name of the userid.

Scheme Procedure: **passwd:passwd** pw [¶](07_02_04_user_information.md)

The encrypted passwd.

Scheme Procedure: **passwd:uid** pw [¶](07_02_04_user_information.md)

The user id number.

Scheme Procedure: **passwd:gid** pw [¶](07_02_04_user_information.md)

The group id number.

Scheme Procedure: **passwd:gecos** pw [¶](07_02_04_user_information.md)

The full name.

Scheme Procedure: **passwd:dir** pw [¶](07_02_04_user_information.md)

The home directory.

Scheme Procedure: **passwd:shell** pw [¶](07_02_04_user_information.md)

The login shell.

  

Scheme Procedure: **getpwuid** uid [¶](07_02_04_user_information.md)

Look up an integer userid in the user database.

Scheme Procedure: **getpwnam** name [¶](07_02_04_user_information.md)

Look up a user name string in the user database.

Scheme Procedure: **setpwent** [¶](07_02_04_user_information.md)

Initializes a stream used by `getpwent` to read from the user database. The next use of `getpwent` will return the first entry. The return value is unspecified.

Scheme Procedure: **getpwent** [¶](07_02_04_user_information.md)

Read the next entry in the user database stream. The return is a passwd user object as above, or `#f` when no more entries.

Scheme Procedure: **endpwent** [¶](07_02_04_user_information.md)

Closes the stream used by `getpwent`. The return value is unspecified.

Scheme Procedure: **setpw** \[arg\] [¶](07_02_04_user_information.md)

C Function: **scm\_setpwent** (arg) [¶](07_02_04_user_information.md)

If called with a true argument, initialize or reset the password data stream. Otherwise, close the stream. The `setpwent` and `endpwent` procedures are implemented on top of this.

Scheme Procedure: **getpw** \[user\] [¶](07_02_04_user_information.md)

C Function: **scm\_getpwuid** (user) [¶](07_02_04_user_information.md)

Look up an entry in the user database. user can be an integer, a string, or omitted, giving the behavior of getpwuid, getpwnam or getpwent respectively.

The following functions accept an object representing group information and return a selected component:

Scheme Procedure: **group:name** gr [¶](07_02_04_user_information.md)

The group name.

Scheme Procedure: **group:passwd** gr [¶](07_02_04_user_information.md)

The encrypted group password.

Scheme Procedure: **group:gid** gr [¶](07_02_04_user_information.md)

The group id number.

Scheme Procedure: **group:mem** gr [¶](07_02_04_user_information.md)

A list of userids which have this group as a supplementary group.

  

Scheme Procedure: **getgrgid** gid [¶](07_02_04_user_information.md)

Look up an integer group id in the group database.

Scheme Procedure: **getgrnam** name [¶](07_02_04_user_information.md)

Look up a group name in the group database.

Scheme Procedure: **setgrent** [¶](07_02_04_user_information.md)

Initializes a stream used by `getgrent` to read from the group database. The next use of `getgrent` will return the first entry. The return value is unspecified.

Scheme Procedure: **getgrent** [¶](07_02_04_user_information.md)

Return the next entry in the group database, using the stream set by `setgrent`.

Scheme Procedure: **endgrent** [¶](07_02_04_user_information.md)

Closes the stream used by `getgrent`. The return value is unspecified.

Scheme Procedure: **setgr** \[arg\] [¶](07_02_04_user_information.md)

C Function: **scm\_setgrent** (arg) [¶](07_02_04_user_information.md)

If called with a true argument, initialize or reset the group data stream. Otherwise, close the stream. The `setgrent` and `endgrent` procedures are implemented on top of this.

Scheme Procedure: **getgr** \[group\] [¶](07_02_04_user_information.md)

C Function: **scm\_getgrgid** (group) [¶](07_02_04_user_information.md)

Look up an entry in the group database. group can be an integer, a string, or omitted, giving the behavior of getgrgid, getgrnam or getgrent respectively.

In addition to the accessor procedures for the user database, the following shortcut procedure is also available.

Scheme Procedure: **getlogin** [¶](07_02_04_user_information.md)

C Function: **scm\_getlogin** () [¶](07_02_04_user_information.md)

Return a string containing the name of the user logged in on the controlling terminal of the process, or `#f` if this information cannot be obtained.

* * *

Next: [Runtime Environment](07_02_06_runtime_environment.md#726-runtime-environment), Previous: [User Information](07_02_04_user_information.md#724-user-information), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
