[]{#Sockets_002eSocketAddress-class_002dinitialization}

::: header
Next:
[Sockets.SocketAddress-accessing](Sockets_002eSocketAddress_002daccessing.html#Sockets_002eSocketAddress_002daccessing){accesskey="n"
rel="next"}, Previous: [Sockets.SocketAddress class-host name
lookup](Sockets_002eSocketAddress-class_002dhost-name-lookup.html#Sockets_002eSocketAddress-class_002dhost-name-lookup){accesskey="p"
rel="prev"}, Up:
[Sockets.SocketAddress](Sockets_002eSocketAddress.html#Sockets_002eSocketAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eSocketAddress-class_003a-initialization}

#### 6.20.7 Sockets.SocketAddress class: initialization {#sockets.socketaddress-class-initialization .subsection}

[]{#index-anyLocalAddress_003a}

**anyLocalAddress: anObject**

Private - Store an object representing a local address in the address
family for the receiver

[]{#index-createLoopbackHost-2}

**createLoopbackHost**

Answer an object representing the loopback host in the address family
for the receiver.

[]{#index-createUnknownAddress-2}

**createUnknownAddress**

Answer an object representing an unkown address in the address family
for the receiver

[]{#index-flush-6}

**flush**

Flush the cached IP addresses.

[]{#index-initLocalAddresses}

**initLocalAddresses**

Private - Initialize the anyLocalAddress class-instance variable for the
entire hierarchy.

[]{#index-update_003a-2}

**update: aspect**

Flush all the caches for IPAddress subclasses
