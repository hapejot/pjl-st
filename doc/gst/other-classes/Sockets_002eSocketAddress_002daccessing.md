[]{#Sockets_002eSocketAddress_002daccessing}

::: header
Next:
[Sockets.SocketAddress-testing](Sockets_002eSocketAddress_002dtesting.html#Sockets_002eSocketAddress_002dtesting){accesskey="n"
rel="next"}, Previous: [Sockets.SocketAddress
class-initialization](Sockets_002eSocketAddress-class_002dinitialization.html#Sockets_002eSocketAddress-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[Sockets.SocketAddress](Sockets_002eSocketAddress.html#Sockets_002eSocketAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eSocketAddress_003a-accessing}

#### 6.20.8 Sockets.SocketAddress: accessing {#sockets.socketaddress-accessing .subsection}

[]{#index-_003d-1}

**= aSocketAddress**

Answer whether the receiver and aSocketAddress represent the same
machine. The host name is not checked because an IPAddress created
before a DNS is activated is named after its numbers-and-dots notation,
while the same IPAddress, created when a DNS is active, is named after
its resolved name.

[]{#index-asByteArray-2}

**asByteArray**

Convert the receiver to a ByteArray passed to the operating system's
socket functions)

[]{#index-hash-2}

**hash**

Answer an hash value for the receiver

[]{#index-name-1}

**name**

Answer the host name (or the digit notation if the DNS could not resolve
the address). If the DNS answers a different IP address for the same
name, the second response is not cached and the digit notation is also
returned (somebody's likely playing strange jokes with your DNS).
