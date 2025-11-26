[]{#PositionableStream_002dcompiling}

::: header
Next:
[PositionableStream-positioning](PositionableStream_002dpositioning.html#PositionableStream_002dpositioning){accesskey="n"
rel="next"}, Previous: [PositionableStream-class type
methods](PositionableStream_002dclass-type-methods.html#PositionableStream_002dclass-type-methods){accesskey="p"
rel="prev"}, Up:
[PositionableStream](PositionableStream.html#PositionableStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PositionableStream_003a-compiling}

#### 1.133.4 PositionableStream: compiling {#positionablestream-compiling .subsection}

[]{#index-name-10}

**name**

Answer a string that represents what the receiver is streaming on

[]{#index-segmentFrom_003ato_003a-1} []{#index-asString-17}

**segmentFrom: startPos to: endPos**

Answer an object that, when sent #asString, will yield the result of
sending 'copyFrom: startPos to: endPos' to the receiver
