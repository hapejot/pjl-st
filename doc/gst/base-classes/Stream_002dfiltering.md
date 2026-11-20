[]{#Stream_002dfiltering}

::: header
Next:
[Stream-polymorphism](Stream_002dpolymorphism.html#Stream_002dpolymorphism){accesskey="n"
rel="next"}, Previous: [Stream-filing
out](Stream_002dfiling-out.html#Stream_002dfiling-out){accesskey="p"
rel="prev"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-filtering}

#### 1.157.11 Stream: filtering {#stream-filtering .subsection}

[]{#index-_002c-6}

**, anIterable**

Answer a new stream that concatenates the data in the receiver with the
data in aStream. Both the receiver and aStream should be readable.

[]{#index-collect_003a-6}

**collect: aBlock**

Answer a new stream that will pass the returned objects through aBlock,
and return whatever object is returned by aBlock instead. Note that when
peeking in the returned stream, the block will be invoked multiple
times, with possibly surprising results.

[]{#index-lines-1}

**lines**

Answer a new stream that answers lines from the receiver.

[]{#index-peek-5}

**peek**

Returns the next element of the stream without moving the pointer.
Returns nil when at end of stream. Lookahead is implemented
automatically for streams that are not positionable but can be copied.

[]{#index-peekFor_003a-3}

**peekFor: aCharacter**

Returns true and gobbles the next element from the stream of it is equal
to anObject, returns false and doesn't gobble the next element if the
next element is not equal to anObject. Lookahead is implemented
automatically for streams that are not positionable but can be copied.

[]{#index-reject_003a-6}

**reject: aBlock**

Answer a new stream that only returns those objects for which aBlock
returns false. Note that the returned stream will not be positionable.

[]{#index-select_003a-6}

**select: aBlock**

Answer a new stream that only returns those objects for which aBlock
returns true. Note that the returned stream will not be positionable.
