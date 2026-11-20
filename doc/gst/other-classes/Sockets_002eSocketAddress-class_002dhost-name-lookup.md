[]{#Sockets_002eSocketAddress-class_002dhost-name-lookup}

::: header
Next: [Sockets.SocketAddress
class-initialization](Sockets_002eSocketAddress-class_002dinitialization.html#Sockets_002eSocketAddress-class_002dinitialization){accesskey="n"
rel="next"}, Previous: [Sockets.SocketAddress class-creating
sockets](Sockets_002eSocketAddress-class_002dcreating-sockets.html#Sockets_002eSocketAddress-class_002dcreating-sockets){accesskey="p"
rel="prev"}, Up:
[Sockets.SocketAddress](Sockets_002eSocketAddress.html#Sockets_002eSocketAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eSocketAddress-class_003a-host-name-lookup}

#### 6.20.6 Sockets.SocketAddress class: host name lookup {#sockets.socketaddress-class-host-name-lookup .subsection}

[]{#index-allByName_003a}

**allByName: aString**

Answer all the IP addresses that refer to the the given host. If a digit
address is passed in aString, the result is an array containing the
single passed address. If the host could not be resolved to an IP
address, answer nil.

[]{#index-byName_003a} []{#index-fromString_003a-8}

**byName: aString**

Answer a single IP address that refer to the the given host. If a digit
address is passed in aString, the result is the same as using
#fromString:. If the host could not be resolved to an IP address, answer
nil.
