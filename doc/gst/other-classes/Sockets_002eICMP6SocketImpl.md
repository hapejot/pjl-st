[]{#Sockets_002eICMP6SocketImpl}

::: header
Next:
[Sockets.ICMPSocketImpl](Sockets_002eICMPSocketImpl.html#Sockets_002eICMPSocketImpl){accesskey="n"
rel="next"}, Previous:
[Sockets.DummyStream](Sockets_002eDummyStream.html#Sockets_002eDummyStream){accesskey="p"
rel="prev"}, Up: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eICMP6SocketImpl-1}

### 6.9 Sockets.ICMP6SocketImpl {#sockets.icmp6socketimpl .section}

[]{#index-Sockets_002eICMP6SocketImpl}

**Defined in namespace Sockets**\
**Superclass: Sockets.RawSocketImpl**\
**Category: Sockets-Protocols**

:   Unless the application installs its own implementation, this is the
    default socket implementation that will be used for IPv6 raw
    sockets. It uses C call-outs to implement standard BSD style sockets
    of family AF_INET, type SOCK_RAW, protocol IPPROTO_ICMPV6.

  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ---------
  • [Sockets.ICMP6SocketImpl class-C constants](Sockets_002eICMP6SocketImpl-class_002dC-constants.html#Sockets_002eICMP6SocketImpl-class_002dC-constants){accesskey="1"}:        (class)
  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ---------
