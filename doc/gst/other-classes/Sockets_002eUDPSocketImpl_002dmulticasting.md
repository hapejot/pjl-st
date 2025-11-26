[]{#Sockets_002eUDPSocketImpl_002dmulticasting}

::: header
Previous: [Sockets.UDPSocketImpl class-C
constants](Sockets_002eUDPSocketImpl-class_002dC-constants.html#Sockets_002eUDPSocketImpl-class_002dC-constants){accesskey="p"
rel="prev"}, Up:
[Sockets.UDPSocketImpl](Sockets_002eUDPSocketImpl.html#Sockets_002eUDPSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eUDPSocketImpl_003a-multicasting}

#### 6.24.2 Sockets.UDPSocketImpl: multicasting {#sockets.udpsocketimpl-multicasting .subsection}

[]{#index-ipMulticastIf-2}

**ipMulticastIf**

Answer the local device for a multicast socket (in the form of an
address)

[]{#index-ipMulticastIf_003a-1}

**ipMulticastIf: interface**

Set the local device for a multicast socket (in the form of an address,
usually anyLocalAddress)

[]{#index-join_003a-3}

**join: ipAddress**

Join the multicast socket at the given address

[]{#index-leave_003a-2}

**leave: ipAddress**

Leave the multicast socket at the given address

[]{#index-primJoinLeave_003aoption_003a}

**primJoinLeave: ipAddress option: opt**

Private - Used to join or leave a multicast service.

[]{#index-timeToLive-2}

**timeToLive**

Answer the time to live of the datagrams sent through the receiver to a
multicast socket.

[]{#index-timeToLive_003a-2}

**timeToLive: ttl**

Set the time to live of the datagrams sent through the receiver to a
multicast socket.
