[]{#Stream_002dpositioning}

::: header
Next:
[Stream-printing](Stream_002dprinting.html#Stream_002dprinting){accesskey="n"
rel="next"}, Previous:
[Stream-polymorphism](Stream_002dpolymorphism.html#Stream_002dpolymorphism){accesskey="p"
rel="prev"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-positioning}

#### 1.157.13 Stream: positioning {#stream-positioning .subsection}

[]{#index-isPositionable-2} []{#index-skip_003a-5}

**isPositionable**

Answer true if the stream supports moving backwards with #skip:.

[]{#index-skip_003a-2}

**skip: anInteger**

Move the position forwards by anInteger places

[]{#index-skipSeparators} []{#index-next-13} []{#index-skipSeparators-1}

**skipSeparators**

Advance the receiver until we find a character that is not a separator.
Answer false if we reach the end of the stream, else answer true; in
this case, sending #next will return the first non-separator character
(possibly the same to which the stream pointed before #skipSeparators
was sent).

[]{#index-skipTo_003a}

**skipTo: anObject**

Move the current position to after the next occurrence of anObject and
return true if anObject was found. If anObject doesn't exist, the
pointer is atEnd, and false is returned.

[]{#index-skipToAll_003a}

**skipToAll: aCollection**

If there is a sequence of objects remaining in the stream that is equal
to the sequence in aCollection, set the stream position just past that
sequence and answer true. Else, set the stream position to its end and
answer false.
