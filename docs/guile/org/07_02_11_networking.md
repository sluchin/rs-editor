#### 7.2.11 Networking [¶](07_02_11_networking.md#7211-networking)

*   [Network Address Conversion](07_02_11_networking.md#72111-network-address-conversion)
*   [Network Databases](07_02_11_networking.md#72112-network-databases)
*   [Network Socket Address](07_02_11_networking.md#72113-network-socket-address)
*   [Network Sockets and Communication](07_02_11_networking.md#72114-network-sockets-and-communication)
*   [Network Socket Examples](07_02_11_networking.md#72115-network-socket-examples)

* * *

Next: [Network Databases](07_02_11_networking.md#72112-network-databases), Up: [Networking](07_02_11_networking.md#7211-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.2.11.1 Network Address Conversion [¶](07_02_11_networking.md#72111-network-address-conversion)

This section describes procedures which convert internet addresses between numeric and string formats.

#### IPv4 Address Conversion [¶](07_02_11_networking.md#ipv4-address-conversion)

An IPv4 Internet address is a 4-byte value, represented in Guile as an integer in host byte order, so that say “0.0.0.1” is 1, or “1.0.0.0” is 16777216.

Some underlying C functions use network byte order for addresses, Guile converts as necessary so that at the Scheme level its host byte order everywhere.

Variable: **INADDR\_ANY** [¶](07_02_11_networking.md)

For a server, this can be used with `bind` (see [Network Sockets and Communication](07_02_11_networking.md#72114-network-sockets-and-communication)) to allow connections from any interface on the machine.

Variable: **INADDR\_BROADCAST** [¶](07_02_11_networking.md)

The broadcast address on the local network.

Variable: **INADDR\_LOOPBACK** [¶](07_02_11_networking.md)

The address of the local host using the loopback device, ie. ‘127.0.0.1’.

Scheme Procedure: **inet-netof** address [¶](07_02_11_networking.md)

C Function: **scm\_inet\_netof** (address) [¶](07_02_11_networking.md)

Return the network number part of the given IPv4 Internet address. E.g.,

([inet-netof](07_02_11_networking.md) 2130706433) ⇒ 127

Scheme Procedure: **inet-lnaof** address [¶](07_02_11_networking.md)

C Function: **scm\_lnaof** (address) [¶](07_02_11_networking.md)

Return the local-address-with-network part of the given IPv4 Internet address, using the obsolete class A/B/C system. E.g.,

([inet-lnaof](07_02_11_networking.md) 2130706433) ⇒ 1

Scheme Procedure: **inet-makeaddr** net lna [¶](07_02_11_networking.md)

C Function: **scm\_inet\_makeaddr** (net, lna) [¶](07_02_11_networking.md)

Make an IPv4 Internet address by combining the network number net with the local-address-within-network number lna. E.g.,

([inet-makeaddr](07_02_11_networking.md) 127 1) ⇒ 2130706433

#### IPv6 Address Conversion [¶](07_02_11_networking.md#ipv6-address-conversion)

An IPv6 Internet address is a 16-byte value, represented in Guile as an integer in host byte order, so that say “::1” is 1. The following constants are defined for convenience.

Variable: **IN6ADDR\_ANY** [¶](07_02_11_networking.md)

For a server, this can be used with `bind` (see [Network Sockets and Communication](07_02_11_networking.md#72114-network-sockets-and-communication)) to allow connections from any IPv6 interface on the machine.

Variable: **IN6ADDR\_LOOPBACK** [¶](07_02_11_networking.md)

The address of the local host using the loopback device, ie. ‘::1’.

The procedures below convert an IPv6 _or_ an IPv4 address to and from its textual representation.

Scheme Procedure: **inet-ntop** family address [¶](07_02_11_networking.md)

C Function: **scm\_inet\_ntop** (family, address) [¶](07_02_11_networking.md)

Convert a network address from an integer to a printable string. family can be `AF_INET` or `AF_INET6`. E.g.,

([inet-ntop](07_02_11_networking.md) AF\_INET 2130706433) ⇒ "127.0.0.1"
([inet-ntop](07_02_11_networking.md) AF\_INET6 ([\-](06_06_02_numerical_data_types.md) ([expt](06_06_02_numerical_data_types.md) 2 128) 1))
  ⇒ "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"

Scheme Procedure: **inet-pton** family address [¶](07_02_11_networking.md)

C Function: **scm\_inet\_pton** (family, address) [¶](07_02_11_networking.md)

Convert a string containing a printable network address to an integer address. family can be `AF_INET` or `AF_INET6`. E.g.,

([inet-pton](07_02_11_networking.md) AF\_INET "127.0.0.1") ⇒ 2130706433
([inet-pton](07_02_11_networking.md) AF\_INET6 "::1") ⇒ 1

* * *

Next: [Network Socket Address](07_02_11_networking.md#72113-network-socket-address), Previous: [Network Address Conversion](07_02_11_networking.md#72111-network-address-conversion), Up: [Networking](07_02_11_networking.md#7211-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.2.11.2 Network Databases [¶](07_02_11_networking.md#72112-network-databases)

This section describes procedures which query various network databases. Care should be taken when using the database routines since they are not reentrant.

#### `getaddrinfo` [¶](07_02_11_networking.md#getaddrinfo)

The `getaddrinfo` procedure maps host and service names to socket addresses and associated information in a protocol-independent way.

Scheme Procedure: **getaddrinfo** name service \[hint\_flags \[hint\_family \[hint\_socktype \[hint\_protocol\]\]\]\] [¶](07_02_11_networking.md)

C Function: **scm\_getaddrinfo** (name, service, hint\_flags, hint\_family, hint\_socktype, hint\_protocol) [¶](07_02_11_networking.md)

Return a list of `addrinfo` structures containing a socket address and associated information for host name and/or service to be used in creating a socket with which to address the specified service.

(let\* ((ai (car (getaddrinfo "www.gnu.org" "http")))
       (s  (socket (addrinfo:fam ai) (addrinfo:socktype ai)
                   (addrinfo:protocol ai))))
  (connect s (addrinfo:addr ai))
  s)

When service is omitted or is `#f`, return network-level addresses for name. When name is `#f` service must be provided and service locations local to the caller are returned.

Additional hints can be provided. When specified, hint\_flags should be a bitwise-or of zero or more constants among the following:

`AI_PASSIVE`

Socket address is intended for `bind`.

`AI_CANONNAME`

Request for canonical host name, available via `addrinfo:canonname`. This makes sense mainly when DNS lookups are involved.

`AI_NUMERICHOST`

Specifies that name is a numeric host address string (e.g., `"127.0.0.1"`), meaning that name resolution will not be used.

`AI_NUMERICSERV`

Likewise, specifies that service is a numeric port string (e.g., `"80"`).

`AI_ADDRCONFIG`

Return only addresses configured on the local system It is highly recommended to provide this flag when the returned socket addresses are to be used to make connections; otherwise, some of the returned addresses could be unreachable or use a protocol that is not supported.

`AI_V4MAPPED`

When looking up IPv6 addresses, return mapped IPv4 addresses if there is no IPv6 address available at all.

`AI_ALL`

If this flag is set along with `AI_V4MAPPED` when looking up IPv6 addresses, return all IPv6 addresses as well as all IPv4 addresses, the latter mapped to IPv6 format.

When given, hint\_family should specify the requested address family, e.g., `AF_INET6`. Similarly, hint\_socktype should specify the requested socket type (e.g., `SOCK_DGRAM`), and hint\_protocol should specify the requested protocol (its value is interpreted as in calls to `socket`).

On error, an exception with key `getaddrinfo-error` is thrown, with an error code (an integer) as its argument:

(catch 'getaddrinfo-error
  (lambda ()
    (getaddrinfo "www.gnu.org" "gopher"))
  (lambda (key errcode)
    (cond ((= errcode EAI\_SERVICE)
	   (display "doesn't know about Gopher!\\n"))
	  ((= errcode EAI\_NONAME)
	   (display "www.gnu.org not found\\\\n"))
	  (else
	   (format #t "something wrong: ~a\\n"
		   (gai-strerror errcode))))))

Error codes are:

`EAI_AGAIN`

The name or service could not be resolved at this time. Future attempts may succeed.

`EAI_BADFLAGS`

hint\_flags contains an invalid value.

`EAI_FAIL`

A non-recoverable error occurred when attempting to resolve the name.

`EAI_FAMILY`

hint\_family was not recognized.

`EAI_NONAME`

Either name does not resolve for the supplied parameters, or neither name nor service were supplied.

`EAI_NODATA`

This non-POSIX error code can be returned on some systems (GNU and Darwin, at least), for example when name is known but requests that were made turned out no data. Error handling code should be prepared to handle it when it is defined.

`EAI_SERVICE`

service was not recognized for the specified socket type.

`EAI_SOCKTYPE`

hint\_socktype was not recognized.

`EAI_SYSTEM`

A system error occurred. In C, the error code can be found in `errno`; this value is not accessible from Scheme, but in practice it provides little information about the actual error cause.

Users are encouraged to read the ["POSIX specification](http://www.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html) for more details.

The following procedures take an `addrinfo` object as returned by `getaddrinfo`:

Scheme Procedure: **addrinfo:flags** ai [¶](07_02_11_networking.md)

Return flags for ai as a bitwise or of `AI_` values (see above).

Scheme Procedure: **addrinfo:fam** ai [¶](07_02_11_networking.md)

Return the address family of ai (a `AF_` value).

Scheme Procedure: **addrinfo:socktype** ai [¶](07_02_11_networking.md)

Return the socket type for ai (a `SOCK_` value).

Scheme Procedure: **addrinfo:protocol** ai [¶](07_02_11_networking.md)

Return the protocol of ai.

Scheme Procedure: **addrinfo:addr** ai [¶](07_02_11_networking.md)

Return the socket address associated with ai as a `sockaddr` object (see [Network Socket Address](07_02_11_networking.md#72113-network-socket-address)).

Scheme Procedure: **addrinfo:canonname** ai [¶](07_02_11_networking.md)

Return a string for the canonical name associated with ai if the `AI_CANONNAME` flag was supplied.

#### The Host Database [¶](07_02_11_networking.md#the-host-database)

A _host object_ is a structure that represents what is known about a network host, and is the usual way of representing a system’s network identity inside software.

The following functions accept a host object and return a selected component:

Scheme Procedure: **hostent:name** host [¶](07_02_11_networking.md)

The “official” hostname for host.

Scheme Procedure: **hostent:aliases** host [¶](07_02_11_networking.md)

A list of aliases for host.

Scheme Procedure: **hostent:addrtype** host [¶](07_02_11_networking.md)

The host address type, one of the `AF` constants, such as `AF_INET` or `AF_INET6`.

Scheme Procedure: **hostent:length** host [¶](07_02_11_networking.md)

The length of each address for host, in bytes.

Scheme Procedure: **hostent:addr-list** host [¶](07_02_11_networking.md)

The list of network addresses associated with host. For `AF_INET` these are integer IPv4 address (see [Network Address Conversion](07_02_11_networking.md#72111-network-address-conversion)).

The following procedures can be used to search the host database. However, `getaddrinfo` should be preferred over them since it’s more generic and thread-safe.

Scheme Procedure: **gethost** \[host\] [¶](07_02_11_networking.md)

Scheme Procedure: **gethostbyname** hostname [¶](07_02_11_networking.md)

Scheme Procedure: **gethostbyaddr** address [¶](07_02_11_networking.md)

C Function: **scm\_gethost** (host) [¶](07_02_11_networking.md)

Look up a host by name or address, returning a host object. The `gethost` procedure will accept either a string name or an integer address; if given no arguments, it behaves like `gethostent` (see below). If a name or address is supplied but the address can not be found, an error will be thrown to one of the keys: `host-not-found`, `try-again`, `no-recovery` or `no-data`, corresponding to the equivalent `h_error` values. Unusual conditions may result in errors thrown to the `system-error` or `misc_error` keys.

([gethost](07_02_11_networking.md) "www.gnu.org")
⇒ #("www.gnu.org" () 2 4 (3353880842))

([gethostbyname](07_02_11_networking.md) "www.emacs.org")
⇒ #("emacs.org" ("www.emacs.org") 2 4 (1073448978))

The following procedures may be used to step through the host database from beginning to end.

Scheme Procedure: **sethostent** \[stayopen\] [¶](07_02_11_networking.md)

Initialize an internal stream from which host objects may be read. This procedure must be called before any calls to `gethostent`, and may also be called afterward to reset the host entry stream. If stayopen is supplied and is not `#f`, the database is not closed by subsequent `gethostbyname` or `gethostbyaddr` calls, possibly giving an efficiency gain.

Scheme Procedure: **gethostent** [¶](07_02_11_networking.md)

Return the next host object from the host database, or `#f` if there are no more hosts to be found (or an error has been encountered). This procedure may not be used before `sethostent` has been called.

Scheme Procedure: **endhostent** [¶](07_02_11_networking.md)

Close the stream used by `gethostent`. The return value is unspecified.

Scheme Procedure: **sethost** \[stayopen\] [¶](07_02_11_networking.md)

C Function: **scm\_sethost** (stayopen) [¶](07_02_11_networking.md)

If stayopen is omitted, this is equivalent to `endhostent`. Otherwise it is equivalent to `sethostent stayopen`.

#### The Network Database [¶](07_02_11_networking.md#the-network-database)

The following functions accept an object representing a network and return a selected component:

Scheme Procedure: **netent:name** net [¶](07_02_11_networking.md)

The “official” network name.

Scheme Procedure: **netent:aliases** net [¶](07_02_11_networking.md)

A list of aliases for the network.

Scheme Procedure: **netent:addrtype** net [¶](07_02_11_networking.md)

The type of the network number. Currently, this returns only `AF_INET`.

Scheme Procedure: **netent:net** net [¶](07_02_11_networking.md)

The network number.

The following procedures are used to search the network database:

Scheme Procedure: **getnet** \[net\] [¶](07_02_11_networking.md)

Scheme Procedure: **getnetbyname** net-name [¶](07_02_11_networking.md)

Scheme Procedure: **getnetbyaddr** net-number [¶](07_02_11_networking.md)

C Function: **scm\_getnet** (net) [¶](07_02_11_networking.md)

Look up a network by name or net number in the network database. The net-name argument must be a string, and the net-number argument must be an integer. `getnet` will accept either type of argument, behaving like `getnetent` (see below) if no arguments are given.

The following procedures may be used to step through the network database from beginning to end.

Scheme Procedure: **setnetent** \[stayopen\] [¶](07_02_11_networking.md)

Initialize an internal stream from which network objects may be read. This procedure must be called before any calls to `getnetent`, and may also be called afterward to reset the net entry stream. If stayopen is supplied and is not `#f`, the database is not closed by subsequent `getnetbyname` or `getnetbyaddr` calls, possibly giving an efficiency gain.

Scheme Procedure: **getnetent** [¶](07_02_11_networking.md)

Return the next entry from the network database.

Scheme Procedure: **endnetent** [¶](07_02_11_networking.md)

Close the stream used by `getnetent`. The return value is unspecified.

Scheme Procedure: **setnet** \[stayopen\] [¶](07_02_11_networking.md)

C Function: **scm\_setnet** (stayopen) [¶](07_02_11_networking.md)

If stayopen is omitted, this is equivalent to `endnetent`. Otherwise it is equivalent to `setnetent stayopen`.

#### The Protocol Database [¶](07_02_11_networking.md#the-protocol-database)

The following functions accept an object representing a protocol and return a selected component:

Scheme Procedure: **protoent:name** protocol [¶](07_02_11_networking.md)

The “official” protocol name.

Scheme Procedure: **protoent:aliases** protocol [¶](07_02_11_networking.md)

A list of aliases for the protocol.

Scheme Procedure: **protoent:proto** protocol [¶](07_02_11_networking.md)

The protocol number.

The following procedures are used to search the protocol database:

Scheme Procedure: **getproto** \[protocol\] [¶](07_02_11_networking.md)

Scheme Procedure: **getprotobyname** name [¶](07_02_11_networking.md)

Scheme Procedure: **getprotobynumber** number [¶](07_02_11_networking.md)

C Function: **scm\_getproto** (protocol) [¶](07_02_11_networking.md)

Look up a network protocol by name or by number. `getprotobyname` takes a string argument, and `getprotobynumber` takes an integer argument. `getproto` will accept either type, behaving like `getprotoent` (see below) if no arguments are supplied.

The following procedures may be used to step through the protocol database from beginning to end.

Scheme Procedure: **setprotoent** \[stayopen\] [¶](07_02_11_networking.md)

Initialize an internal stream from which protocol objects may be read. This procedure must be called before any calls to `getprotoent`, and may also be called afterward to reset the protocol entry stream. If stayopen is supplied and is not `#f`, the database is not closed by subsequent `getprotobyname` or `getprotobynumber` calls, possibly giving an efficiency gain.

Scheme Procedure: **getprotoent** [¶](07_02_11_networking.md)

Return the next entry from the protocol database.

Scheme Procedure: **endprotoent** [¶](07_02_11_networking.md)

Close the stream used by `getprotoent`. The return value is unspecified.

Scheme Procedure: **setproto** \[stayopen\] [¶](07_02_11_networking.md)

C Function: **scm\_setproto** (stayopen) [¶](07_02_11_networking.md)

If stayopen is omitted, this is equivalent to `endprotoent`. Otherwise it is equivalent to `setprotoent stayopen`.

#### The Service Database [¶](07_02_11_networking.md#the-service-database)

The following functions accept an object representing a service and return a selected component:

Scheme Procedure: **servent:name** serv [¶](07_02_11_networking.md)

The “official” name of the network service.

Scheme Procedure: **servent:aliases** serv [¶](07_02_11_networking.md)

A list of aliases for the network service.

Scheme Procedure: **servent:port** serv [¶](07_02_11_networking.md)

The Internet port used by the service.

Scheme Procedure: **servent:proto** serv [¶](07_02_11_networking.md)

The protocol used by the service. A service may be listed many times in the database under different protocol names.

The following procedures are used to search the service database:

Scheme Procedure: **getserv** \[name \[protocol\]\] [¶](07_02_11_networking.md)

Scheme Procedure: **getservbyname** name protocol [¶](07_02_11_networking.md)

Scheme Procedure: **getservbyport** port protocol [¶](07_02_11_networking.md)

C Function: **scm\_getserv** (name, protocol) [¶](07_02_11_networking.md)

Look up a network service by name or by service number, and return a network service object. The protocol argument specifies the name of the desired protocol; if the protocol found in the network service database does not match this name, a system error is signaled.

The `getserv` procedure will take either a service name or number as its first argument; if given no arguments, it behaves like `getservent` (see below).

([getserv](07_02_11_networking.md) "imap" "tcp")
⇒ #("imap2" ("imap") 143 "tcp")

([getservbyport](07_02_11_networking.md) 88 "udp")
⇒ #("kerberos" ("kerberos5" "krb5") 88 "udp")

The following procedures may be used to step through the service database from beginning to end.

Scheme Procedure: **setservent** \[stayopen\] [¶](07_02_11_networking.md)

Initialize an internal stream from which service objects may be read. This procedure must be called before any calls to `getservent`, and may also be called afterward to reset the service entry stream. If stayopen is supplied and is not `#f`, the database is not closed by subsequent `getservbyname` or `getservbyport` calls, possibly giving an efficiency gain.

Scheme Procedure: **getservent** [¶](07_02_11_networking.md)

Return the next entry from the services database.

Scheme Procedure: **endservent** [¶](07_02_11_networking.md)

Close the stream used by `getservent`. The return value is unspecified.

Scheme Procedure: **setserv** \[stayopen\] [¶](07_02_11_networking.md)

C Function: **scm\_setserv** (stayopen) [¶](07_02_11_networking.md)

If stayopen is omitted, this is equivalent to `endservent`. Otherwise it is equivalent to `setservent stayopen`.

* * *

Next: [Network Sockets and Communication](07_02_11_networking.md#72114-network-sockets-and-communication), Previous: [Network Databases](07_02_11_networking.md#72112-network-databases), Up: [Networking](07_02_11_networking.md#7211-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.2.11.3 Network Socket Address [¶](07_02_11_networking.md#72113-network-socket-address)

A _socket address_ object identifies a socket endpoint for communication. In the case of `AF_INET` for instance, the socket address object comprises the host address (or interface on the host) and a port number which specifies a particular open socket in a running client or server process. A socket address object can be created with,

Scheme Procedure: **make-socket-address** AF\_INET ipv4addr port [¶](07_02_11_networking.md)

Scheme Procedure: **make-socket-address** AF\_INET6 ipv6addr port \[flowinfo \[scopeid\]\] [¶](07_02_11_networking.md)

Scheme Procedure: **make-socket-address** AF\_UNIX path [¶](07_02_11_networking.md)

C Function: **scm\_make\_socket\_address** (family, address, arglist) [¶](07_02_11_networking.md)

Return a new socket address object. The first argument is the address family, one of the `AF` constants, then the arguments vary according to the family.

For `AF_INET` the arguments are an IPv4 network address number (see [Network Address Conversion](07_02_11_networking.md#72111-network-address-conversion)), and a port number.

For `AF_INET6` the arguments are an IPv6 network address number and a port number. Optional flowinfo and scopeid arguments may be given (both integers, default 0).

For `AF_UNIX` the argument is a filename (a string).

The C function `scm_make_socket_address` takes the family and address arguments directly, then arglist is a list of further arguments, being the port for IPv4, port and optional flowinfo and scopeid for IPv6, or the empty list `SCM_EOL` for Unix domain.

The following functions access the fields of a socket address object,

Scheme Procedure: **sockaddr:fam** sa [¶](07_02_11_networking.md)

Return the address family from socket address object sa. This is one of the `AF` constants (e.g. `AF_INET`).

Scheme Procedure: **sockaddr:path** sa [¶](07_02_11_networking.md)

For an `AF_UNIX` socket address object sa, return the filename.

Scheme Procedure: **sockaddr:addr** sa [¶](07_02_11_networking.md)

For an `AF_INET` or `AF_INET6` socket address object sa, return the network address number.

Scheme Procedure: **sockaddr:port** sa [¶](07_02_11_networking.md)

For an `AF_INET` or `AF_INET6` socket address object sa, return the port number.

Scheme Procedure: **sockaddr:flowinfo** sa [¶](07_02_11_networking.md)

For an `AF_INET6` socket address object sa, return the flowinfo value.

Scheme Procedure: **sockaddr:scopeid** sa [¶](07_02_11_networking.md)

For an `AF_INET6` socket address object sa, return the scope ID value.

The functions below convert to and from the C `struct sockaddr` (see [Address Formats](https://doc.guix.gnu.org/libc/latest/en/libc.html#Address-Formats) in The GNU C Library Reference Manual). That structure is a generic type, an application can cast to or from `struct sockaddr_in`, `struct sockaddr_in6` or `struct sockaddr_un` according to the address family.

In a `struct sockaddr` taken or returned, the byte ordering in the fields follows the C conventions (see [Byte Order Conversion](https://doc.guix.gnu.org/libc/latest/en/libc.html#Byte-Order) in The GNU C Library Reference Manual). This means network byte order for `AF_INET` host address (`sin_addr.s_addr`) and port number (`sin_port`), and `AF_INET6` port number (`sin6_port`). But at the Scheme level these values are taken or returned in host byte order, so the port is an ordinary integer, and the host address likewise is an ordinary integer (as described in [Network Address Conversion](07_02_11_networking.md#72111-network-address-conversion)).

C Function: `struct sockaddr *` **scm\_c\_make\_socket\_address** `(SCM family, SCM address, SCM args, size_t *outsize)` [¶](07_02_11_networking.md)

Return a newly-`malloc`ed `struct sockaddr` created from arguments like those taken by `scm_make_socket_address` above.

The size (in bytes) of the `struct sockaddr` return is stored into `*outsize`. An application must call `free` to release the returned structure when no longer required.

C Function: `SCM` **scm\_from\_sockaddr** `(const struct sockaddr *address, unsigned address_size)` [¶](07_02_11_networking.md)

Return a Scheme socket address object from the C address structure. address\_size is the size in bytes of address.

C Function: `struct sockaddr *` **scm\_to\_sockaddr** `(SCM address, size_t *address_size)` [¶](07_02_11_networking.md)

Return a newly-`malloc`ed `struct sockaddr` from a Scheme level socket address object.

The size (in bytes) of the `struct sockaddr` return is stored into `*outsize`. An application must call `free` to release the returned structure when no longer required.

* * *

Next: [Network Socket Examples](07_02_11_networking.md#72115-network-socket-examples), Previous: [Network Socket Address](07_02_11_networking.md#72113-network-socket-address), Up: [Networking](07_02_11_networking.md#7211-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.2.11.4 Network Sockets and Communication [¶](07_02_11_networking.md#72114-network-sockets-and-communication)

Socket ports can be created using `socket` and `socketpair`. The ports are initially unbuffered, to make reading and writing to the same port more reliable. A buffer can be added to the port using `setvbuf` (see [Buffering](06_12_input_and_output.md#6126-buffering)).

Most systems have limits on how many files and sockets can be open, so it’s strongly recommended that socket ports be closed explicitly when no longer required (see [Ports](06_12_input_and_output.md#6121-ports)).

Some of the underlying C functions take values in network byte order, but the convention in Guile is that at the Scheme level everything is ordinary host byte order and conversions are made automatically where necessary.

Scheme Procedure: **socket** family style proto [¶](07_02_11_networking.md)

C Function: **scm\_socket** (family, style, proto) [¶](07_02_11_networking.md)

Return a new socket port of the type specified by family, style and proto. All three parameters are integers. The possible values for family are as follows, where supported by the system,

Variable: **PF\_UNIX** [¶](07_02_11_networking.md)

Variable: **PF\_INET** [¶](07_02_11_networking.md)

Variable: **PF\_INET6** [¶](07_02_11_networking.md)

The possible values for style are as follows, again where supported by the system,

Variable: **SOCK\_STREAM** [¶](07_02_11_networking.md)

Variable: **SOCK\_DGRAM** [¶](07_02_11_networking.md)

Variable: **SOCK\_RAW** [¶](07_02_11_networking.md)

Variable: **SOCK\_RDM** [¶](07_02_11_networking.md)

Variable: **SOCK\_SEQPACKET** [¶](07_02_11_networking.md)

proto can be obtained from a protocol name using `getprotobyname` (see [Network Databases](07_02_11_networking.md#72112-network-databases)). A value of zero means the default protocol, which is usually right.

A socket cannot by used for communication until it has been connected somewhere, usually with either `connect` or `accept` below.

Scheme Procedure: **socketpair** family style proto [¶](07_02_11_networking.md)

C Function: **scm\_socketpair** (family, style, proto) [¶](07_02_11_networking.md)

Return a pair, the `car` and `cdr` of which are two unnamed socket ports connected to each other. The connection is full-duplex, so data can be transferred in either direction between the two.

family, style and proto are as per `socket` above. But many systems only support socket pairs in the `PF_UNIX` family. Zero is likely to be the only meaningful value for proto.

Scheme Procedure: **getsockopt** sock level optname [¶](07_02_11_networking.md)

Scheme Procedure: **setsockopt** sock level optname value [¶](07_02_11_networking.md)

C Function: **scm\_getsockopt** (sock, level, optname) [¶](07_02_11_networking.md)

C Function: **scm\_setsockopt** (sock, level, optname, value) [¶](07_02_11_networking.md)

Get or set an option on socket port sock. `getsockopt` returns the current value. `setsockopt` sets a value and the return is unspecified.

level is an integer specifying a protocol layer, either `SOL_SOCKET` for socket level options, or a protocol number from the `IPPROTO` constants or `getprotoent` (see [Network Databases](07_02_11_networking.md#72112-network-databases)).

Variable: **SOL\_SOCKET** [¶](07_02_11_networking.md)

Variable: **IPPROTO\_IP** [¶](07_02_11_networking.md)

Variable: **IPPROTO\_IPV6** [¶](07_02_11_networking.md)

Variable: **IPPROTO\_TCP** [¶](07_02_11_networking.md)

Variable: **IPPROTO\_UDP** [¶](07_02_11_networking.md)

optname is an integer specifying an option within the protocol layer.

For `SOL_SOCKET` level the following optnames are defined (when provided by the system). For their meaning see [Socket-Level Options](https://doc.guix.gnu.org/libc/latest/en/libc.html#Socket_002dLevel-Options) in The GNU C Library Reference Manual, or `man 7 socket`.

Variable: **SO\_DEBUG** [¶](07_02_11_networking.md)

Variable: **SO\_REUSEADDR** [¶](07_02_11_networking.md)

Variable: **SO\_STYLE** [¶](07_02_11_networking.md)

Variable: **SO\_TYPE** [¶](07_02_11_networking.md)

Variable: **SO\_ERROR** [¶](07_02_11_networking.md)

Variable: **SO\_DONTROUTE** [¶](07_02_11_networking.md)

Variable: **SO\_BROADCAST** [¶](07_02_11_networking.md)

Variable: **SO\_SNDBUF** [¶](07_02_11_networking.md)

Variable: **SO\_RCVBUF** [¶](07_02_11_networking.md)

Variable: **SO\_KEEPALIVE** [¶](07_02_11_networking.md)

Variable: **SO\_OOBINLINE** [¶](07_02_11_networking.md)

Variable: **SO\_NO\_CHECK** [¶](07_02_11_networking.md)

Variable: **SO\_PRIORITY** [¶](07_02_11_networking.md)

Variable: **SO\_REUSEPORT** [¶](07_02_11_networking.md)

Variable: **SO\_RCVTIMEO** [¶](07_02_11_networking.md)

Variable: **SO\_SNDTIMEO** [¶](07_02_11_networking.md)

The value taken or returned is an integer.

Variable: **SO\_LINGER** [¶](07_02_11_networking.md)

The value taken or returned is a pair of integers `(ENABLE . TIMEOUT)`. On old systems without timeout support (ie. without `struct linger`), only ENABLE has an effect but the value in Guile is always a pair.

For IP level (`IPPROTO_IP`) the following optnames are defined (when provided by the system). See `man ip` for what they mean.

Variable: **IP\_MULTICAST\_IF** [¶](07_02_11_networking.md)

This sets the source interface used by multicast traffic.

Variable: **IP\_MULTICAST\_TTL** [¶](07_02_11_networking.md)

This sets the default TTL for multicast traffic. This defaults to 1 and should be increased to allow traffic to pass beyond the local network.

Variable: **IP\_ADD\_MEMBERSHIP** [¶](07_02_11_networking.md)

Variable: **IP\_DROP\_MEMBERSHIP** [¶](07_02_11_networking.md)

These can be used only with `setsockopt`, not `getsockopt`. value is a pair `(MULTIADDR . INTERFACEADDR)` of integer IPv4 addresses (see [Network Address Conversion](07_02_11_networking.md#72111-network-address-conversion)). MULTIADDR is a multicast address to be added to or dropped from the interface INTERFACEADDR. INTERFACEADDR can be `INADDR_ANY` to have the system select the interface. INTERFACEADDR can also be an interface index number, on systems supporting that.

Last, for IPv6 level (`IPPROTO_IPV6`), the following optnames are defined. See `man 7 ipv6` for details.

Variable: **IPV6\_V6ONLY** [¶](07_02_11_networking.md)

Determines whether an `AF_INET6` socket is restricted to transmitting IPv6 packets only, or whether it can also transmit packets for an IPv4-mapped IPv6 address.

For `IPPROTO_TCP` level the following optnames are defined (when provided by the system). For their meaning see `man 7 tcp`.

Variable: **TCP\_NODELAY** [¶](07_02_11_networking.md)

Variable: **TCP\_CORK** [¶](07_02_11_networking.md)

The value taken or returned is an integer.

Scheme Procedure: **shutdown** sock how [¶](07_02_11_networking.md)

C Function: **scm\_shutdown** (sock, how) [¶](07_02_11_networking.md)

Sockets can be closed simply by using `close-port`. The `shutdown` procedure allows reception or transmission on a connection to be shut down individually, according to the parameter how:

0

Stop receiving data for this socket. If further data arrives, reject it.

1

Stop trying to transmit data from this socket. Discard any data waiting to be sent. Stop looking for acknowledgement of data already sent; don’t retransmit it if it is lost.

2

Stop both reception and transmission.

The return value is unspecified.

Scheme Procedure: **connect** sock sockaddr [¶](07_02_11_networking.md)

Scheme Procedure: **connect** sock AF\_INET ipv4addr port [¶](07_02_11_networking.md)

Scheme Procedure: **connect** sock AF\_INET6 ipv6addr port \[flowinfo \[scopeid\]\] [¶](07_02_11_networking.md)

Scheme Procedure: **connect** sock AF\_UNIX path [¶](07_02_11_networking.md)

C Function: **scm\_connect** (sock, fam, address, args) [¶](07_02_11_networking.md)

Initiate a connection on socket port sock to a given address. The destination is either a socket address object, or arguments the same as `make-socket-address` would take to make such an object (see [Network Socket Address](07_02_11_networking.md#72113-network-socket-address)). Return true unless the socket was configured as non-blocking and the connection could not be made immediately.

(connect sock AF\_INET INADDR\_LOOPBACK 23)
(connect sock (make-socket-address AF\_INET INADDR\_LOOPBACK 23))

Scheme Procedure: **bind** sock sockaddr [¶](07_02_11_networking.md)

Scheme Procedure: **bind** sock AF\_INET ipv4addr port [¶](07_02_11_networking.md)

Scheme Procedure: **bind** sock AF\_INET6 ipv6addr port \[flowinfo \[scopeid\]\] [¶](07_02_11_networking.md)

Scheme Procedure: **bind** sock AF\_UNIX path [¶](07_02_11_networking.md)

C Function: **scm\_bind** (sock, fam, address, args) [¶](07_02_11_networking.md)

Bind socket port sock to the given address. The address is either a socket address object, or arguments the same as `make-socket-address` would take to make such an object (see [Network Socket Address](07_02_11_networking.md#72113-network-socket-address)). The return value is unspecified.

Generally a socket is only explicitly bound to a particular address when making a server, i.e. to listen on a particular port. For an outgoing connection the system will assign a local address automatically, if not already bound.

(bind sock AF\_INET INADDR\_ANY 12345)
(bind sock (make-socket-address AF\_INET INADDR\_ANY 12345))

Scheme Procedure: **listen** sock backlog [¶](07_02_11_networking.md)

C Function: **scm\_listen** (sock, backlog) [¶](07_02_11_networking.md)

Enable sock to accept connection requests. backlog is an integer specifying the maximum length of the queue for pending connections. If the queue fills, new clients will fail to connect until the server calls `accept` to accept a connection from the queue.

The return value is unspecified.

Scheme Procedure: **accept** sock \[flags\] [¶](07_02_11_networking.md)

C Function: **scm\_accept** (sock) [¶](07_02_11_networking.md)

Accept a connection from socket port sock which has been enabled for listening with `listen` above.

If there are no incoming connections in the queue, there are two possible behaviors, depending on whether sock has been configured for non-blocking operation or not:

*   If there is no connection waiting and the socket was set to non-blocking mode with the `O_NONBLOCK` port option (see [`fcntl`](07_02_02_ports_and_file_descriptors.md#722-ports-and-file-descriptors)), return `#f` directly.
*   Otherwise wait until a connection is available.

The return value is a pair. The `car` is a new socket port, connected and ready to communicate. The `cdr` is a socket address object (see [Network Socket Address](07_02_11_networking.md#72113-network-socket-address)) which is where the remote connection is from (like `getpeername` below).

flags, if given, may include `SOCK_CLOEXEC` or `SOCK_NONBLOCK`, which like `O_CLOEXEC` and `O_NONBLOCK` apply to the newly accepted socket.

All communication takes place using the new socket returned. The given sock remains bound and listening, and `accept` may be called on it again to get another incoming connection when desired.

Scheme Procedure: **getsockname** sock [¶](07_02_11_networking.md)

C Function: **scm\_getsockname** (sock) [¶](07_02_11_networking.md)

Return a socket address object which is the where sock is bound locally. sock may have obtained its local address from `bind` (above), or if a `connect` is done with an otherwise unbound socket (which is usual) then the system will have assigned an address.

Note that on many systems the address of a socket in the `AF_UNIX` namespace cannot be read.

Scheme Procedure: **getpeername** sock [¶](07_02_11_networking.md)

C Function: **scm\_getpeername** (sock) [¶](07_02_11_networking.md)

Return a socket address object which is where sock is connected to, i.e. the remote endpoint.

Note that on many systems the address of a socket in the `AF_UNIX` namespace cannot be read.

Scheme Procedure: **recv!** sock buf \[flags\] [¶](07_02_11_networking.md)

C Function: **scm\_recv** (sock, buf, flags) [¶](07_02_11_networking.md)

Receive data from a socket port. sock must already be bound to the address from which data is to be received. buf is a bytevector into which the data will be written. The size of buf limits the amount of data which can be received: in the case of packet protocols, if a packet larger than this limit is encountered then some data will be irrevocably lost.

The optional flags argument is a value or bitwise OR of `MSG_OOB`, `MSG_PEEK`, `MSG_DONTROUTE` etc.

The value returned is the number of bytes read from the socket.

Note that the data is read directly from the socket file descriptor: any unread buffered port data is ignored.

Scheme Procedure: **send** sock message \[flags\] [¶](07_02_11_networking.md)

C Function: **scm\_send** (sock, message, flags) [¶](07_02_11_networking.md)

Transmit bytevector message on socket port sock. sock must already be bound to a destination address. The value returned is the number of bytes transmitted—it’s possible for this to be less than the length of message if the socket is set to be non-blocking. The optional flags argument is a value or bitwise OR of `MSG_OOB`, `MSG_PEEK`, `MSG_DONTROUTE` etc.

Note that the data is written directly to the socket file descriptor: any unflushed buffered port data is ignored.

Scheme Procedure: **recvfrom!** sock buf \[flags \[start \[end\]\]\] [¶](07_02_11_networking.md)

C Function: **scm\_recvfrom** (sock, buf, flags, start, end) [¶](07_02_11_networking.md)

Receive data from socket port sock, returning the originating address as well as the data. This function is usually for datagram sockets, but can be used on stream-oriented sockets too.

The data received is stored in bytevector buf, using either the whole bytevector or just the region between the optional start and end positions. The size of buf limits the amount of data that can be received. For datagram protocols if a packet larger than this is received then excess bytes are irrevocably lost.

The return value is a pair. The `car` is the number of bytes read. The `cdr` is a socket address object (see [Network Socket Address](07_02_11_networking.md#72113-network-socket-address)) which is where the data came from, or `#f` if the origin is unknown.

The optional flags argument is a or bitwise-OR (`logior`) of `MSG_OOB`, `MSG_PEEK`, `MSG_DONTROUTE` etc.

Data is read directly from the socket file descriptor, any buffered port data is ignored.

On a GNU/Linux system `recvfrom!` is not multi-threading, all threads stop while a `recvfrom!` call is in progress. An application may need to use `select`, `O_NONBLOCK` or `MSG_DONTWAIT` to avoid this.

Scheme Procedure: **sendto** sock message sockaddr \[flags\] [¶](07_02_11_networking.md)

Scheme Procedure: **sendto** sock message AF\_INET ipv4addr port \[flags\] [¶](07_02_11_networking.md)

Scheme Procedure: **sendto** sock message AF\_INET6 ipv6addr port \[flowinfo \[scopeid \[flags\]\]\] [¶](07_02_11_networking.md)

Scheme Procedure: **sendto** sock message AF\_UNIX path \[flags\] [¶](07_02_11_networking.md)

C Function: **scm\_sendto** (sock, message, fam, address, args\_and\_flags) [¶](07_02_11_networking.md)

Transmit bytevector message as a datagram socket port sock. The destination is specified either as a socket address object, or as arguments the same as would be taken by `make-socket-address` to create such an object (see [Network Socket Address](07_02_11_networking.md#72113-network-socket-address)).

The destination address may be followed by an optional flags argument which is a `logior` (see [Bitwise Operations](06_06_02_numerical_data_types.md#66213-bitwise-operations)) of `MSG_OOB`, `MSG_PEEK`, `MSG_DONTROUTE` etc.

The value returned is the number of bytes transmitted – it’s possible for this to be less than the length of message if the socket is set to be non-blocking. Note that the data is written directly to the socket file descriptor: any unflushed buffered port data is ignored.

* * *

Previous: [Network Sockets and Communication](07_02_11_networking.md#72114-network-sockets-and-communication), Up: [Networking](07_02_11_networking.md#7211-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]

#### 7.2.11.5 Network Socket Examples [¶](07_02_11_networking.md#72115-network-socket-examples)

The following give examples of how to use network sockets.

#### Internet Socket Client Example [¶](07_02_11_networking.md#internet-socket-client-example)

The following example demonstrates an Internet socket client. It connects to the HTTP daemon running on the local machine and returns the contents of the root index URL.

(let ((s (socket PF\_INET SOCK\_STREAM 0)))
  (connect s AF\_INET (inet-pton AF\_INET "127.0.0.1") 80)
  (display "GET / HTTP/1.0\\r\\n\\r\\n" s)

  (do ((line (read-line s) (read-line s)))
      ((eof-object? line))
    (display line)
    (newline)))

#### Internet Socket Server Example [¶](07_02_11_networking.md#internet-socket-server-example)

The following example shows a simple Internet server which listens on port 2904 for incoming connections and sends a greeting back to the client.

(let ((s (socket PF\_INET SOCK\_STREAM 0)))
  (setsockopt s SOL\_SOCKET SO\_REUSEADDR 1)
  ;; Specific address?
  ;; (bind s AF\_INET (inet-pton AF\_INET "127.0.0.1") 2904)
  (bind s AF\_INET INADDR\_ANY 2904)
  (listen s 5)

  (simple-format #t "Listening for clients in pid: ~S" (getpid))
  (newline)

  (while #t
    (let\* ((client-connection (accept s))
           (client-details (cdr client-connection))
           (client (car client-connection)))
      (simple-format #t "Got new client connection: ~S"
                     client-details)
      (newline)
      (simple-format #t "Client address: ~S"
                     (gethostbyaddr
                      (sockaddr:addr client-details)))
      (newline)
      ;; Send back the greeting to the client port
      (display "Hello client\\r\\n" client)
      (close client))))

* * *

Next: [Locales](07_02_13_locales.md#7213-locales), Previous: [Networking](07_02_11_networking.md#7211-networking), Up: [POSIX System Calls and Networking](07_02_00_posix_system_calls_and_networking.md#72-posix-system-calls-and-networking)   \[[Contents](00_contents.md "Table of contents")\]\[[Index](index_r5rs.md#r5rs-index "Index")\]
