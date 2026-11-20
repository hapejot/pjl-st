[]{#Sockets_002eUnixAddress}

::: header
Next:
[Sockets.UnixDatagramSocketImpl](Sockets_002eUnixDatagramSocketImpl.html#Sockets_002eUnixDatagramSocketImpl){accesskey="n"
rel="next"}, Previous:
[Sockets.UDPSocketImpl](Sockets_002eUDPSocketImpl.html#Sockets_002eUDPSocketImpl){accesskey="p"
rel="prev"}, Up: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eUnixAddress-1}

### 6.25 Sockets.UnixAddress {#sockets.unixaddress .section}

[]{#index-Sockets_002eUnixAddress}

**Defined in namespace Sockets**\
**Superclass: Sockets.SocketAddress**\
**Category: Sockets-Protocols**

:   This class represents an address for a machine using the AF_UNIX
    address family. Since this address family is only used for local
    sockets, the class is a singleton; the filesystem path to the socket
    is represented using the port argument to socket functions, as
    either a String or a File object.

  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Sockets.UnixAddress class-C constants](Sockets_002eUnixAddress-class_002dC-constants.html#Sockets_002eUnixAddress-class_002dC-constants){accesskey="1"}:                          (class)
  • [Sockets.UnixAddress class-initialization](Sockets_002eUnixAddress-class_002dinitialization.html#Sockets_002eUnixAddress-class_002dinitialization){accesskey="2"}:                 (class)
  • [Sockets.UnixAddress class-instance creation](Sockets_002eUnixAddress-class_002dinstance-creation.html#Sockets_002eUnixAddress-class_002dinstance-creation){accesskey="3"}:        (class)
  • [Sockets.UnixAddress-accessing](Sockets_002eUnixAddress_002daccessing.html#Sockets_002eUnixAddress_002daccessing){accesskey="4"}:                                                  (instance)
  • [Sockets.UnixAddress-printing](Sockets_002eUnixAddress_002dprinting.html#Sockets_002eUnixAddress_002dprinting){accesskey="5"}:                                                     (instance)
  • [Sockets.UnixAddress-testing](Sockets_002eUnixAddress_002dtesting.html#Sockets_002eUnixAddress_002dtesting){accesskey="6"}:                                                        (instance)
  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
