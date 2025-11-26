[]{#WriteStream_002daccessing_002dwriting}

::: header
Next:
[WriteStream-positioning](WriteStream_002dpositioning.html#WriteStream_002dpositioning){accesskey="n"
rel="next"}, Previous: [WriteStream class-instance
creation](WriteStream-class_002dinstance-creation.html#WriteStream-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[WriteStream](WriteStream.html#WriteStream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WriteStream_003a-accessing_002dwriting}

#### 1.223.2 WriteStream: accessing-writing {#writestream-accessing-writing .subsection}

[]{#index-contents-7}

**contents**

Returns a collection of the same type that the stream accesses, up to
and including the final element.

[]{#index-next_003aputAll_003astartingAt_003a-4}

**next: n putAll: aCollection startingAt: pos**

Put n characters or bytes of aCollection, starting at the pos-th, in the
collection buffer.

[]{#index-nextPut_003a-7}

**nextPut: anObject**

Store anObject as the next item in the receiver. Grow the collection if
necessary

[]{#index-readStream-8}

**readStream**

Answer a ReadStream on the same contents as the receiver

[]{#index-reverseContents-2}

**reverseContents**

Returns a collection of the same type that the stream accesses, up to
and including the final element, but in reverse order.
