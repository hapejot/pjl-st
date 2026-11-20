[]{#Sockets_002eReadBuffer_002daccessing_002dreading}

::: header
Next: [Sockets.ReadBuffer-buffer
handling](Sockets_002eReadBuffer_002dbuffer-handling.html#Sockets_002eReadBuffer_002dbuffer-handling){accesskey="n"
rel="next"}, Previous: [Sockets.ReadBuffer class-instance
creation](Sockets_002eReadBuffer-class_002dinstance-creation.html#Sockets_002eReadBuffer-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[Sockets.ReadBuffer](Sockets_002eReadBuffer.html#Sockets_002eReadBuffer){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Sockets_002eReadBuffer_003a-accessing_002dreading}

#### 6.17.2 Sockets.ReadBuffer: accessing-reading {#sockets.readbuffer-accessing-reading .subsection}

[]{#index-nextAvailable_003ainto_003astartingAt_003a}

**nextAvailable: anInteger into: aCollection startingAt: pos**

Place the next anInteger objects from the receiver into aCollection,
starting at position pos. Return the number of items stored.

[]{#index-nextAvailable_003aputAllOn_003a}

**nextAvailable: anInteger putAllOn: aStream**

Copy the next anInteger objects from the receiver to aStream. Return the
number of items stored.

[]{#index-upTo_003a}

**upTo: anObject**

Returns a collection of the same type that the stream accesses, up to
but not including the object anObject. Returns the entire rest of the
stream's contents if anObject is not present.

[]{#index-upToEnd}

**upToEnd**

Returns a collection of the same type that the stream accesses, up to
but not including the object anObject. Returns the entire rest of the
stream's contents if anObject is not present.
