[]{#Float_002dtranscendental-operations}

::: header
Next: [Float-truncation and round
off](Float_002dtruncation-and-round-off.html#Float_002dtruncation-and-round-off){accesskey="n"
rel="next"}, Previous: [Float-testing
functionality](Float_002dtesting-functionality.html#Float_002dtesting-functionality){accesskey="p"
rel="prev"}, Up: [Float](Float.html#Float){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Float_003a-transcendental-operations}

#### 1.80.17 Float: transcendental operations {#float-transcendental-operations .subsection}

[]{#index-asFloat}

**asFloat**

Just defined for completeness. Return the receiver.

[]{#index-ceilingLog_003a}

**ceilingLog: radix**

Answer (self log: radix) ceiling. Use exact arithmetic if radix is not a
floating point value.

[]{#index-estimatedLog}

**estimatedLog**

Answer an estimate of (self abs floorLog: 10)

[]{#index-floorLog_003a}

**floorLog: radix**

Answer (self log: radix) floor. Use exact arithmetic if radix is not a
floating point value.

[]{#index-log}

**log**

Answer log base 10 of the receiver.
