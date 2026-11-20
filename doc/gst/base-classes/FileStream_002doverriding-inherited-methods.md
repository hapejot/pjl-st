[]{#FileStream_002doverriding-inherited-methods}

::: header
Next:
[FileStream-testing](FileStream_002dtesting.html#FileStream_002dtesting){accesskey="n"
rel="next"}, Previous:
[FileStream-initialize-release](FileStream_002dinitialize_002drelease.html#FileStream_002dinitialize_002drelease){accesskey="p"
rel="prev"}, Up: [FileStream](FileStream.html#FileStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#FileStream_003a-overriding-inherited-methods}

#### 1.79.7 FileStream: overriding inherited methods {#filestream-overriding-inherited-methods .subsection}

[]{#index-next_003aputAll_003astartingAt_003a-1}

**next: n putAll: aCollection startingAt: pos**

Write n values from aCollection, the first being at pos.

[]{#index-nextLine}

**nextLine**

Returns a collection of the same type that the stream accesses,
containing the next line up to the next new-line character. Returns the
entire rest of the stream's contents if no new-line character is found.

[]{#index-nextPutAllOn_003a-1}

**nextPutAllOn: aStream**

Put all the characters of the receiver in aStream.

[]{#index-upTo_003a}

**upTo: aCharacter**

Returns a collection of the same type that the stream accesses,
containing data up to aCharacter. Returns the entire rest of the
stream's contents if no such character is found.
