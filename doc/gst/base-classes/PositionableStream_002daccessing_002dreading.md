[]{#PositionableStream_002daccessing_002dreading}

::: header
Next: [PositionableStream-class type
methods](PositionableStream_002dclass-type-methods.html#PositionableStream_002dclass-type-methods){accesskey="n"
rel="next"}, Previous: [PositionableStream class-instance
creation](PositionableStream-class_002dinstance-creation.html#PositionableStream-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[PositionableStream](PositionableStream.html#PositionableStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PositionableStream_003a-accessing_002dreading}

#### 1.133.2 PositionableStream: accessing-reading {#positionablestream-accessing-reading .subsection}

[]{#index-close-1}

**close**

Disassociate a stream from its backing store.

[]{#index-contents-4}

**contents**

Returns a collection of the same type that the stream accesses, up to
and including the final element.

[]{#index-copyFrom_003ato_003a-6} []{#index-position-3}

**copyFrom: start to: end**

Answer the data on which the receiver is streaming, from the start-th
item to the end-th. Note that this method is 0-based, unlike the one in
Collection, because a Stream's #position method returns 0-based values.

[]{#index-next-4}

**next**

Answer the next item of the receiver. Returns nil when at end of stream.

[]{#index-nextAvailable_003ainto_003astartingAt_003a-2}

**nextAvailable: anInteger into: aCollection startingAt: pos**

Place up to anInteger objects from the receiver into aCollection,
starting from position pos in the collection and stopping if no more
data is available.

[]{#index-nextAvailable_003aputAllOn_003a-1}

**nextAvailable: anInteger putAllOn: aStream**

Copy up to anInteger objects from the receiver into aStream, stopping if
no more data is available.

[]{#index-peek-3}

**peek**

Returns the next element of the stream without moving the pointer.
Returns nil when at end of stream.

[]{#index-peekFor_003a-2}

**peekFor: anObject**

Returns true and gobbles the next element from the stream of it is equal
to anObject, returns false and doesn't gobble the next element if the
next element is not equal to anObject.

[]{#index-readStream-5}

**readStream**

Answer a ReadStream on the same contents as the receiver

[]{#index-reverseContents-1}

**reverseContents**

Returns a collection of the same type that the stream accesses, up to
and including the final element, but in reverse order.

[]{#index-upTo_003a-1}

**upTo: anObject**

Returns a collection of the same type that the stream accesses, up to
but not including the object anObject. Returns the entire rest of the
stream's contents if anObject is not present.

[]{#index-upToEnd}

**upToEnd**

Returns a collection of the same type that the stream accesses,
containing the entire rest of the stream's contents.

------------------------------------------------------------------------

::: header
Next: [PositionableStream-class type
methods](PositionableStream_002dclass-type-methods.html#PositionableStream_002dclass-type-methods){accesskey="n"
rel="next"}, Previous: [PositionableStream class-instance
creation](PositionableStream-class_002dinstance-creation.html#PositionableStream-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[PositionableStream](PositionableStream.html#PositionableStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
