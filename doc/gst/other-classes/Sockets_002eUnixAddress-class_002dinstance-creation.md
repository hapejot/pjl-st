[]{#Sockets_002eUnixAddress-class_002dinstance-creation}

::: header
Next:
[Sockets.UnixAddress-accessing](Sockets_002eUnixAddress_002daccessing.html#Sockets_002eUnixAddress_002daccessing){accesskey="n"
rel="next"}, Previous: [Sockets.UnixAddress
class-initialization](Sockets_002eUnixAddress-class_002dinitialization.html#Sockets_002eUnixAddress-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[Sockets.UnixAddress](Sockets_002eUnixAddress.html#Sockets_002eUnixAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eUnixAddress-class_003a-instance-creation}

#### 6.25.3 Sockets.UnixAddress class: instance creation {#sockets.unixaddress-class-instance-creation .subsection}

[]{#index-fromSockAddr_003aport_003a-3}

**fromSockAddr: aByteArray port: portAdaptor**

Private - Answer the unique UnixAddress instance, filling in the
portAdaptor's value from a ByteArray containing a C sockaddr_in
structure.

[]{#index-uniqueInstance-1}

**uniqueInstance**

Not commented.
