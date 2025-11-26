[]{#Sockets_002eICMPSocketImpl}

::: header
Next:
[Sockets.IP6Address](Sockets_002eIP6Address.html#Sockets_002eIP6Address){accesskey="n"
rel="next"}, Previous:
[Sockets.ICMP6SocketImpl](Sockets_002eICMP6SocketImpl.html#Sockets_002eICMP6SocketImpl){accesskey="p"
rel="prev"}, Up: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eICMPSocketImpl-1}

### 6.10 Sockets.ICMPSocketImpl {#sockets.icmpsocketimpl .section}

[]{#index-Sockets_002eICMPSocketImpl}

**Defined in namespace Sockets**\
**Superclass: Sockets.RawSocketImpl**\
**Category: Sockets-Protocols**

:   Unless the application installs its own implementation, this is the
    default socket implementation that will be used for IPv4 raw
    sockets. It uses C call-outs to implement standard BSD style sockets
    of family AF_INET, type SOCK_RAW, protocol IPPROTO_ICMP.

  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ---------
  • [Sockets.ICMPSocketImpl class-C constants](Sockets_002eICMPSocketImpl-class_002dC-constants.html#Sockets_002eICMPSocketImpl-class_002dC-constants){accesskey="1"}:        (class)
  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ---------
