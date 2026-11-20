[]{#Sockets_002eMulticastSocket}

::: header
Next:
[Sockets.MulticastSocketImpl](Sockets_002eMulticastSocketImpl.html#Sockets_002eMulticastSocketImpl){accesskey="n"
rel="next"}, Previous:
[Sockets.IPAddress](Sockets_002eIPAddress.html#Sockets_002eIPAddress){accesskey="p"
rel="prev"}, Up: [Sockets
package](Sockets-package.html#Sockets-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eMulticastSocket-1}

### 6.13 Sockets.MulticastSocket {#sockets.multicastsocket .section}

[]{#index-Sockets_002eMulticastSocket}

**Defined in namespace Sockets**\
**Superclass: Sockets.DatagramSocket**\
**Category: Sockets-Streams**

:   This class models a multicast socket that sends packets to a
    multicast group. All members of the group listening on that address
    and port will receive all the messages sent to the group.

    In the TCP/IP world, these sockets are UDP-based and a multicast
    group consists of a multicast address (a class D internet address,
    i.e. one whose most significant bits are 1110), and a well known
    port number.

  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Sockets.MulticastSocket-instance creation](Sockets_002eMulticastSocket_002dinstance-creation.html#Sockets_002eMulticastSocket_002dinstance-creation){accesskey="1"}:        (instance)
  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
