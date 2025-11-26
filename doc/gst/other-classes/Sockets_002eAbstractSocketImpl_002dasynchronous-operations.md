[]{#Sockets_002eAbstractSocketImpl_002dasynchronous-operations}

::: header
Next: [Sockets.AbstractSocketImpl-C
call-outs](Sockets_002eAbstractSocketImpl_002dC-call_002douts.html#Sockets_002eAbstractSocketImpl_002dC-call_002douts){accesskey="n"
rel="next"}, Previous:
[Sockets.AbstractSocketImpl-accessing](Sockets_002eAbstractSocketImpl_002daccessing.html#Sockets_002eAbstractSocketImpl_002daccessing){accesskey="p"
rel="prev"}, Up:
[Sockets.AbstractSocketImpl](Sockets_002eAbstractSocketImpl.html#Sockets_002eAbstractSocketImpl){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eAbstractSocketImpl_003a-asynchronous-operations}

#### 6.2.6 Sockets.AbstractSocketImpl: asynchronous operations {#sockets.abstractsocketimpl-asynchronous-operations .subsection}

[]{#index-ensureReadable}

**ensureReadable**

If the file is open, wait until data can be read from it. The wait
allows other Processes to run.

[]{#index-ensureWriteable}

**ensureWriteable**

If the file is open, wait until we can write to it. The wait allows
other Processes to run.

[]{#index-waitForException}

**waitForException**

If the file is open, wait until an exceptional condition (such as
presence of out of band data) has occurred on it. The wait allows other
Processes to run.
