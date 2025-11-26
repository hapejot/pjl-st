[]{#Sockets_002eAbstractSocketImpl_002dsocket-options}

::: header
Previous: [Sockets.AbstractSocketImpl-socket
operations](Sockets_002eAbstractSocketImpl_002dsocket-operations.html#Sockets_002eAbstractSocketImpl_002dsocket-operations){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocketImpl](Sockets_002eAbstractSocketImpl.html#Sockets_002eAbstractSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocketImpl_003a-socket-options}

#### 6.2.10 Sockets.AbstractSocketImpl: socket options {#sockets.abstractsocketimpl-socket-options .subsection}

[]{#index-optionAt_003alevel_003aput_003a}

**optionAt: opt level: level put: anObject**

Modify the value of a socket option. The option identifier is in 'opt'
and the level is in 'level'. anObject can be a boolean, integer, socket
address or ByteArray. A layer over this method is provided for the most
common socket options, so this will be rarely used.

[]{#index-optionAt_003alevel_003asize_003a}

**optionAt: opt level: level size: size**

Answer in a ByteArray of the given size the value of a socket option.
The option identifier is in 'opt' and the level is in 'level'. A layer
over this method is provided for the most common socket options, so this
will be rarely used.

[]{#index-soLinger-2}

**soLinger**

Answer the number of seconds by which a 'close' operation can block to
ensure that all the packets have reliably reached the destination, or
nil if those packets are left to their destiny.

[]{#index-soLinger_003a-1}

**soLinger: linger**

Set the number of seconds by which a 'close' operation can block to
ensure that all the packets have reliably reached the destination. If
linger is nil, those packets are left to their destiny.

[]{#index-soReuseAddr-1}

**soReuseAddr**

Answer whether another socket can be bound the same local address as
this one. If you enable this option, you can actually have two sockets
with the same Internet port number; but the system won't allow you to
use the two identically-named sockets in a way that would confuse the
Internet. The reason for this option is that some higher-level Internet
protocols, including FTP, require you to keep reusing the same socket
number.

[]{#index-soReuseAddr_003a}

**soReuseAddr: aBoolean**

Set whether another socket can be bound the same local address as this
one.

[]{#index-valueWithoutBuffering_003a}

**valueWithoutBuffering: aBlock**

Evaluate aBlock, ensuring that any data that it writes to the socket is
sent immediately to the network.

------------------------------------------------------------------------

::: header
Previous: [Sockets.AbstractSocketImpl-socket
operations](Sockets_002eAbstractSocketImpl_002dsocket-operations.html#Sockets_002eAbstractSocketImpl_002dsocket-operations){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocketImpl](Sockets_002eAbstractSocketImpl.html#Sockets_002eAbstractSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
