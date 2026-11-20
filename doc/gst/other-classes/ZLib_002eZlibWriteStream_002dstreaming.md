[]{#ZLib_002eZlibWriteStream_002dstreaming}

::: header
Up:
[ZLib.ZlibWriteStream](ZLib_002eZlibWriteStream.html#ZLib_002eZlibWriteStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ZLib_002eZlibWriteStream_003a-streaming}

#### 7.13.1 ZLib.ZlibWriteStream: streaming {#zlib.zlibwritestream-streaming .subsection}

[]{#index-close-7}

**close**

Finish the deflated output to the destination stream using Z_FINISH. The
destination stream is closed, which implies flushing.

[]{#index-contents-3} []{#index-contents-4}

**contents**

Finish the deflated output to the destination stream using Z_FINISH and
return the deflated data (requires the destination stream to support
#contents).

[]{#index-finish-1}

**finish**

Finish the deflated output to the destination stream using Z_FINISH. The
destination stream is not flushed.

[]{#index-flush-8}

**flush**

Flush the deflated output to the destination stream, and flush the
destination stream.

[]{#index-flushBuffer}

**flushBuffer**

Flush the deflated output to the destination stream.

[]{#index-flushDictionary}

**flushDictionary**

Flush the deflated output to the destination stream using Z_FULL_FLUSH,
and flush the destination stream.

[]{#index-next_003aputAll_003astartingAt_003a-3}

**next: n putAll: aCollection startingAt: pos**

Put n characters or bytes of aCollection, starting at the pos-th, in the
deflation buffer.

[]{#index-nextPut_003a-6}

**nextPut: aByte**

Append a character or byte (depending on whether the destination stream
works on a ByteArray or String) to the deflation buffer.

[]{#index-partialFlush}

**partialFlush**

Flush the deflated output to the destination stream using
Z_PARTIAL_FLUSH, and flush the destination stream.

[]{#index-position-2}

**position**

Answer the number of compressed bytes written.

[]{#index-readStream} []{#index-readStream-1}

**readStream**

Finish the deflated output to the destination stream using Z_FINISH and
return a ReadStream on the deflated data (requires the destination
stream to support #readStream).

[]{#index-syncFlush}

**syncFlush**

Flush the deflated output to the destination stream using Z_SYNC_FLUSH,
and flush the destination stream. Note that this includes the four bytes
0/0/255/255 at the end of the flush.

------------------------------------------------------------------------

::: header
Up:
[ZLib.ZlibWriteStream](ZLib_002eZlibWriteStream.html#ZLib_002eZlibWriteStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
