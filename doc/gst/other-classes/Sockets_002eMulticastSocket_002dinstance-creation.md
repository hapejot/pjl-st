[]{#Sockets_002eMulticastSocket_002dinstance-creation}

::: header
Up:
[Sockets.MulticastSocket](Sockets_002eMulticastSocket.html#Sockets_002eMulticastSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eMulticastSocket_003a-instance-creation}

#### 6.13.1 Sockets.MulticastSocket: instance creation {#sockets.multicastsocket-instance-creation .subsection}

[]{#index-interface}

**interface**

Answer the local device supporting the multicast socket. This is usually
set to any local address.

[]{#index-interface_003a}

**interface: ipAddress**

Set the local device supporting the multicast socket. This is usually
set to any local address.

[]{#index-join_003a-1}

**join: ipAddress**

Join the multicast socket at the given IP address

[]{#index-leave_003a}

**leave: ipAddress**

Leave the multicast socket at the given IP address

[]{#index-nextPut_003atimeToLive_003a}

**nextPut: packet timeToLive: timeToLive**

Send the datagram with a specific TTL (time-to-live)

[]{#index-timeToLive}

**timeToLive**

Answer the socket's datagrams' default time-to-live

[]{#index-timeToLive_003a}

**timeToLive: newTTL**

Set the default time-to-live for the socket's datagrams
