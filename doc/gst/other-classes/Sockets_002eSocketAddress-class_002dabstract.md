[]{#Sockets_002eSocketAddress-class_002dabstract}

::: header
Next: [Sockets.SocketAddress
class-accessing](Sockets_002eSocketAddress-class_002daccessing.html#Sockets_002eSocketAddress-class_002daccessing){accesskey="n"
rel="next"}, Up:
[Sockets.SocketAddress](Sockets_002eSocketAddress.html#Sockets_002eSocketAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eSocketAddress-class_003a-abstract}

#### 6.20.1 Sockets.SocketAddress class: abstract {#sockets.socketaddress-class-abstract .subsection}

[]{#index-extractFromSockAddr_003aport_003a}

**extractFromSockAddr: aByteArray port: portAdaptor**

Private - Answer a new SocketAddress from a ByteArray containing a C
sockaddr structure. The portAdaptor's value is changed to contain the
port that the structure refers to.

[]{#index-fromSockAddr_003aport_003a-2}

**fromSockAddr: aByteArray port: portAdaptor**

Private - Answer a new IPAddress from a ByteArray containing a C
sockaddr structure. The portAdaptor's value is changed to contain the
port that the structure refers to. Raise an error if the address family
is unknown.
