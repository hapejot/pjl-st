[]{#Sockets_002eDatagram_002daccessing}

::: header
Previous: [Sockets.Datagram class-instance
creation](Sockets_002eDatagram-class_002dinstance-creation.html#Sockets_002eDatagram-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[Sockets.Datagram](Sockets_002eDatagram.html#Sockets_002eDatagram){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eDatagram_003a-accessing}

#### 6.5.2 Sockets.Datagram: accessing {#sockets.datagram-accessing .subsection}

[]{#index-address-1}

**address**

Answer the address of the target socket

[]{#index-address_003a}

**address: ipAddress**

Set the address of the target socket

[]{#index-data-1}

**data**

Answer the data attached to the datagram

[]{#index-data_003a-3}

**data: aByteArray**

Set the data attached to the datagram

[]{#index-dataSize}

**dataSize**

Answer the size of the message.

[]{#index-dataSize_003a}

**dataSize: aSize**

I am called to update the size\...

[]{#index-get} []{#index-object_003aaddress_003aport_003a-1}

**get**

Parse the data attached to the datagram through a newly created
ObjectDumper, and answer the resulting object. This method is
complementary to #object:address:port:.

[]{#index-getThrough_003a}
[]{#index-object_003aobjectDumper_003aaddress_003aport_003a-2}

**getThrough: objectDumper**

Parse the data attached to the datagram through the given ObjectDumper
without touching the stream to which it is attached, and answer the
resulting object. The state of the ObjectDumper, though, is updated.
This method is complementary to #object:objectDumper:address:port:.

[]{#index-port-1}

**port**

Answer the IP port of the target socket

[]{#index-port_003a}

**port: thePort**

Set the IP port of the target socket

[]{#index-size-6}

**size**

I determine the size of the datagram. It is either an explicitly
specified dataSize, or the size of the whole collection.
