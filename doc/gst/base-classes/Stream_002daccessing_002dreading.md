[]{#Stream_002daccessing_002dreading}

::: header
Next:
[Stream-accessing-writing](Stream_002daccessing_002dwriting.html#Stream_002daccessing_002dwriting){accesskey="n"
rel="next"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-accessing_002dreading}

#### 1.157.1 Stream: accessing-reading {#stream-accessing-reading .subsection}

[]{#index-contents-6}

**contents**

Answer the whole contents of the receiver, from the next object to the
last

[]{#index-file-2}

**file**

Return nil by default; not all streams have a file.

[]{#index-name-14}

**name**

Return nil by default; not all streams have a name.

[]{#index-next-8}

**next**

Return the next object in the receiver

[]{#index-next_003a}

**next: anInteger**

Return the next anInteger objects in the receiver

[]{#index-nextAvailable_003a}

**nextAvailable: anInteger**

Return up to anInteger objects in the receiver. Besides stopping if the
end of the stream is reached, this may return less than this number of
bytes for various reasons. For example, on files and sockets this
operation could be non-blocking, or could do at most one I/O operation.

[]{#index-nextAvailable_003ainto_003astartingAt_003a-3}

**nextAvailable: anInteger into: aCollection startingAt: pos**

Place the next anInteger objects from the receiver into aCollection,
starting at position pos. Return the number of items stored. Besides
stopping if the end of the stream is reached, this may return less than
this number of bytes for various reasons. For example, on files and
sockets this operation could be non-blocking, or could do at most one
I/O operation.

[]{#index-nextAvailable_003aputAllOn_003a-2}

**nextAvailable: anInteger putAllOn: aStream**

Copy up to anInteger objects in the receiver to aStream. Besides
stopping if the end of the stream is reached, this may return less than
this number of bytes for various reasons. For example, on files and
sockets this operation could be non-blocking, or could do at most one
I/O operation.

[]{#index-nextLine-1}

**nextLine**

Returns a collection of the same type that the stream accesses,
containing the next line up to the next new-line character. Returns the
entire rest of the stream's contents if no new-line character is found.

[]{#index-nextMatchFor_003a}

**nextMatchFor: anObject**

Answer whether the next object is equal to anObject. Even if it does
not, anObject is lost

[]{#index-splitAt_003a} []{#index-_003d-48}

**splitAt: anObject**

Answer an OrderedCollection of parts of the receiver. A new (possibly
empty) part starts at the start of the receiver, or after every
occurrence of an object which is equal to anObject (as compared by #=).

[]{#index-upTo_003a-2}

**upTo: anObject**

Returns a collection of the same type that the stream accesses, up to
but not including the object anObject. Returns the entire rest of the
stream's contents if anObject is not present.

[]{#index-upToAll_003a}

**upToAll: aCollection**

If there is a sequence of objects remaining in the stream that is equal
to the sequence in aCollection, set the stream position just past that
sequence and answer the elements up to, but not including, the sequence.
Else, set the stream position to its end and answer all the remaining
elements.

[]{#index-upToEnd-1}

**upToEnd**

Answer every item in the collection on which the receiver is streaming,
from the next one to the last

------------------------------------------------------------------------

::: header
Next:
[Stream-accessing-writing](Stream_002daccessing_002dwriting.html#Stream_002daccessing_002dwriting){accesskey="n"
rel="next"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
