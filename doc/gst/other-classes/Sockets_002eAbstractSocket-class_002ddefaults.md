[]{#Sockets_002eAbstractSocket-class_002ddefaults}

::: header
Next: [Sockets.AbstractSocket class-instance
creation](Sockets_002eAbstractSocket-class_002dinstance-creation.html#Sockets_002eAbstractSocket-class_002dinstance-creation){accesskey="n"
rel="next"}, Up:
[Sockets.AbstractSocket](Sockets_002eAbstractSocket.html#Sockets_002eAbstractSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocket-class_003a-defaults}

#### 6.1.1 Sockets.AbstractSocket class: defaults {#sockets.abstractsocket-class-defaults .subsection}

[]{#index-defaultAddressClass}

**defaultAddressClass**

Answer the default address family to be used. In the library, the
address family is represented by a subclass of SocketAddress which is by
default IPAddress.

[]{#index-defaultAddressClass_003a}

**defaultAddressClass: class**

Set the default address family to be used. In the library, the address
family is represented by a subclass of SocketAddress which is by default
IPAddress.

[]{#index-defaultImplementationClassFor_003a}

**defaultImplementationClassFor: aSocketAddressClass**

Answer the default implementation class. Depending on the subclass, this
might be the default stream socket implementation class of the given
address class, or rather its default datagram socket implementation
class.
