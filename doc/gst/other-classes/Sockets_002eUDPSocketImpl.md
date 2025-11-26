[]{#Sockets_002eUDPSocketImpl}

::: header
Next:
[Sockets.UnixAddress](Sockets_002eUnixAddress.html#Sockets_002eUnixAddress){accesskey="n"
rel="next"}, Previous:
[Sockets.TCPSocketImpl](Sockets_002eTCPSocketImpl.html#Sockets_002eTCPSocketImpl){accesskey="p"
rel="prev"}, Up: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eUDPSocketImpl-1}

### 6.24 Sockets.UDPSocketImpl {#sockets.udpsocketimpl .section}

[]{#index-Sockets_002eUDPSocketImpl}

**Defined in namespace Sockets**\
**Superclass: Sockets.MulticastSocketImpl**\
**Category: Sockets-Protocols**

:   Unless the application installs its own implementation, this is the
    default socket implementation that will be used for IPv4 datagram
    sockets. It uses C call-outs to implement standard BSD style sockets
    of family AF_INET and type SOCK_DGRAM.

  ------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Sockets.UDPSocketImpl class-C constants](Sockets_002eUDPSocketImpl-class_002dC-constants.html#Sockets_002eUDPSocketImpl-class_002dC-constants){accesskey="1"}:        (class)
  • [Sockets.UDPSocketImpl-multicasting](Sockets_002eUDPSocketImpl_002dmulticasting.html#Sockets_002eUDPSocketImpl_002dmulticasting){accesskey="2"}:                       (instance)
  ------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
