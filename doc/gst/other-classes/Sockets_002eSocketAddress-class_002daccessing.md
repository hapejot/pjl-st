[]{#Sockets_002eSocketAddress-class_002daccessing}

::: header
Next: [Sockets.SocketAddress class-C
call-outs](Sockets_002eSocketAddress-class_002dC-call_002douts.html#Sockets_002eSocketAddress-class_002dC-call_002douts){accesskey="n"
rel="next"}, Previous: [Sockets.SocketAddress
class-abstract](Sockets_002eSocketAddress-class_002dabstract.html#Sockets_002eSocketAddress-class_002dabstract){accesskey="p"
rel="prev"}, Up:
[Sockets.SocketAddress](Sockets_002eSocketAddress.html#Sockets_002eSocketAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eSocketAddress-class_003a-accessing}

#### 6.20.2 Sockets.SocketAddress class: accessing {#sockets.socketaddress-class-accessing .subsection}

[]{#index-anyLocalAddress}

**anyLocalAddress**

Answer an IPAddress representing a local address.

[]{#index-at_003acache_003a}

**at: host cache: aBlock**

Private - Answer the list of addresses associated to the given host in
the cache. If the host is not cached yet, evaluate aBlock and cache and
answer the result.

[]{#index-defaultDatagramSocketImplClass}

**defaultDatagramSocketImplClass**

Answer the class that, by default, is used to map between the Socket's
protocol and a low-level C interface.

[]{#index-defaultDatagramSocketImplClass_003a}

**defaultDatagramSocketImplClass: aClass**

Set which class will be used by default to map between the receiver's
protocol and a low-level C interface.

[]{#index-defaultRawSocketImplClass}

**defaultRawSocketImplClass**

Answer the class that, by default, is used to map between the Socket's
protocol and a low-level C interface.

[]{#index-defaultRawSocketImplClass_003a}

**defaultRawSocketImplClass: aClass**

Set which class will be used by default to map between the receiver's
protocol and a low-level C interface.

[]{#index-defaultStreamSocketImplClass}

**defaultStreamSocketImplClass**

Answer the class that, by default, is used to map between the Socket's
protocol and a low-level C interface.

[]{#index-defaultStreamSocketImplClass_003a}

**defaultStreamSocketImplClass: aClass**

Set which class will be used by default to map between the receiver's
protocol and a low-level C interface.

[]{#index-isDigitAddress_003a}

**isDigitAddress: aString**

Answer whether the receiver can interpret aString as a valid address
without going through a resolver.

[]{#index-localHostName}

**localHostName**

Answer the name of the local machine.

[]{#index-loopbackHost}

**loopbackHost**

Answer an instance of the receiver representing the local machine
(127.0.0.1 in the IPv4 family).

[]{#index-unknownAddress}

**unknownAddress**

Answer an instance of the receiver representing an unknown machine
(0.0.0.0 in the IPv4 family).

------------------------------------------------------------------------

::: header
Next: [Sockets.SocketAddress class-C
call-outs](Sockets_002eSocketAddress-class_002dC-call_002douts.html#Sockets_002eSocketAddress-class_002dC-call_002douts){accesskey="n"
rel="next"}, Previous: [Sockets.SocketAddress
class-abstract](Sockets_002eSocketAddress-class_002dabstract.html#Sockets_002eSocketAddress-class_002dabstract){accesskey="p"
rel="prev"}, Up:
[Sockets.SocketAddress](Sockets_002eSocketAddress.html#Sockets_002eSocketAddress){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
