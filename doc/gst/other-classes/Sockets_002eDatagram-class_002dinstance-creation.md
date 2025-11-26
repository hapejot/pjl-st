[]{#Sockets_002eDatagram-class_002dinstance-creation}

::: header
Next:
[Sockets.Datagram-accessing](Sockets_002eDatagram_002daccessing.html#Sockets_002eDatagram_002daccessing){accesskey="n"
rel="next"}, Up:
[Sockets.Datagram](Sockets_002eDatagram.html#Sockets_002eDatagram){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eDatagram-class_003a-instance-creation}

#### 6.5.1 Sockets.Datagram class: instance creation {#sockets.datagram-class-instance-creation .subsection}

[]{#index-data_003a-2}

**data: aByteArray**

Answer a new datagram with the specified data.

[]{#index-data_003aaddress_003aport_003a}

**data: aByteArray address: ipAddress port: port**

Answer a new datagram with the specified target socket, and aByteArray
as its data.

[]{#index-object_003aaddress_003aport_003a}
[]{#index-object_003aobjectDumper_003aaddress_003aport_003a-1}

**object: object address: ipAddress port: port**

Serialize the object onto a ByteArray, and create a Datagram with the
object as its contents, and the specified receiver. Note that each
invocation of this method creates a separate ObjectDumper; if different
objects that you're sending are likely to contain references to the same
objects, you should use #object:objectDumper:address:port:.

[]{#index-object_003aobjectDumper_003aaddress_003aport_003a}

**object: object objectDumper: od address: ipAddress port: port**

Serialize the object onto a ByteArray, and create a Datagram with the
object as its contents, and the specified receiver. Serialization takes
place through ObjectDumper passed as 'od', and the stream attached to
the ObjectDumper is resetted every time. Using this method is indicated
if different objects that you're sending are likely to contain
references to the same objects.
