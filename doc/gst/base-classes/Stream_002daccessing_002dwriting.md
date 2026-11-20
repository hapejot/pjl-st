[]{#Stream_002daccessing_002dwriting}

::: header
Next:
[Stream-basic](Stream_002dbasic.html#Stream_002dbasic){accesskey="n"
rel="next"}, Previous:
[Stream-accessing-reading](Stream_002daccessing_002dreading.html#Stream_002daccessing_002dreading){accesskey="p"
rel="prev"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-accessing_002dwriting}

#### 1.157.2 Stream: accessing-writing {#stream-accessing-writing .subsection}

[]{#index-next_003aput_003a}

**next: anInteger put: anObject**

Write anInteger copies of anObject to the receiver

[]{#index-next_003aputAll_003astartingAt_003a-2}

**next: n putAll: aCollection startingAt: start**

Write n objects to the stream, reading them from aCollection and
starting at the start-th item.

[]{#index-nextPut_003a-5}

**nextPut: anObject**

Write anObject to the receiver

[]{#index-nextPutAll_003a}

**nextPutAll: aCollection**

Write all the objects in aCollection to the receiver

[]{#index-nextPutAllFlush_003a}

**nextPutAllFlush: aCollection**

Put all the elements of aCollection in the stream, then flush the
buffers if supported by the stream.
