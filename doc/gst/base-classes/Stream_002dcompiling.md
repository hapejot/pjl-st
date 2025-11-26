[]{#Stream_002dcompiling}

::: header
Next:
[Stream-concatenating](Stream_002dconcatenating.html#Stream_002dconcatenating){accesskey="n"
rel="next"}, Previous: [Stream-character
writing](Stream_002dcharacter-writing.html#Stream_002dcharacter-writing){accesskey="p"
rel="prev"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-compiling}

#### 1.157.7 Stream: compiling {#stream-compiling .subsection}

[]{#index-segmentFrom_003ato_003a-2} []{#index-asString-18}

**segmentFrom: startPos to: endPos**

Answer an object that, when sent #asString, will yield the result of
sending 'copyFrom: startPos to: endPos' to the receiver
