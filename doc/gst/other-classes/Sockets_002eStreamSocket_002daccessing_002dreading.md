[]{#Sockets_002eStreamSocket_002daccessing_002dreading}

::: header
Next: [Sockets.StreamSocket-out-of-band
data](Sockets_002eStreamSocket_002dout_002dof_002dband-data.html#Sockets_002eStreamSocket_002dout_002dof_002dband-data){accesskey="n"
rel="next"}, Previous:
[Sockets.StreamSocket-accessing](Sockets_002eStreamSocket_002daccessing.html#Sockets_002eStreamSocket_002daccessing){accesskey="p"
rel="prev"}, Up:
[Sockets.StreamSocket](Sockets_002eStreamSocket.html#Sockets_002eStreamSocket){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eStreamSocket_003a-accessing_002dreading}

#### 6.22.5 Sockets.StreamSocket: accessing-reading {#sockets.streamsocket-accessing-reading .subsection}

[]{#index-nextAvailable_003ainto_003astartingAt_003a-1}

**nextAvailable: anInteger into: aCollection startingAt: pos**

Place up to anInteger objects from the receiver into aCollection,
starting from position pos and stopping if no more data is available.

[]{#index-nextAvailable_003aputAllOn_003a-1}

**nextAvailable: anInteger putAllOn: aStream**

Copy up to anInteger objects from the receiver to aStream, stopping if
no more data is available.
