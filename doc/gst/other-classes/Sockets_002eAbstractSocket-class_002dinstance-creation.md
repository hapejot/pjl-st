[]{#Sockets_002eAbstractSocket-class_002dinstance-creation}

::: header
Next: [Sockets.AbstractSocket class-timed-out
operations](Sockets_002eAbstractSocket-class_002dtimed_002dout-operations.html#Sockets_002eAbstractSocket-class_002dtimed_002dout-operations){accesskey="n"
rel="next"}, Previous: [Sockets.AbstractSocket
class-defaults](Sockets_002eAbstractSocket-class_002ddefaults.html#Sockets_002eAbstractSocket-class_002ddefaults){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocket](Sockets_002eAbstractSocket.html#Sockets_002eAbstractSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocket-class_003a-instance-creation}

#### 6.1.2 Sockets.AbstractSocket class: instance creation {#sockets.abstractsocket-class-instance-creation .subsection}

[]{#index-new-12}

**new**

This method should not be called for instances of this class.

[]{#index-new_003a-10}

**new: implementation**

Answer a new instance of the receiver, using as the underlying layer the
object passed as the 'implementation' parameter; the object is probably
going to be some kind of AbstractSocketImpl.

[]{#index-new_003aaddressClass_003a}

**new: implClass addressClass: addressClass**

Answer a new instance of the receiver, using as the underlying layer a
new instance of 'implementationClass' and using the protocol family of
'addressClass'.
