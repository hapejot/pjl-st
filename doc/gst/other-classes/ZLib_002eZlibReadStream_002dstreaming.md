[]{#ZLib_002eZlibReadStream_002dstreaming}

::: header
Previous:
[ZLib.ZlibReadStream-accessing-reading](ZLib_002eZlibReadStream_002daccessing_002dreading.html#ZLib_002eZlibReadStream_002daccessing_002dreading){accesskey="p"
rel="prev"}, Up:
[ZLib.ZlibReadStream](ZLib_002eZlibReadStream.html#ZLib_002eZlibReadStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ZLib_002eZlibReadStream_003a-streaming}

#### 7.11.2 ZLib.ZlibReadStream: streaming {#zlib.zlibreadstream-streaming .subsection}

[]{#index-atEnd-5}

**atEnd**

Answer whether the stream has got to an end

[]{#index-next-7}

**next**

Return the next object (character or byte) in the receiver.

[]{#index-peek-3}

**peek**

Returns the next element of the stream without moving the pointer.
Returns nil when at end of stream.

[]{#index-peekFor_003a-1}

**peekFor: anObject**

Returns true and gobbles the next element from the stream of it is equal
to anObject, returns false and doesn't gobble the next element if the
next element is not equal to anObject.

[]{#index-position-1}

**position**

Answer the current value of the stream pointer. Note that only inflating
streams support random access to the stream data.
