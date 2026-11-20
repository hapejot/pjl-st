[]{#Number_002dshortcuts-and-iterators}

::: header
Next:
[Number-testing](Number_002dtesting.html#Number_002dtesting){accesskey="n"
rel="next"}, Previous:
[Number-retrying](Number_002dretrying.html#Number_002dretrying){accesskey="p"
rel="prev"}, Up: [Number](Number.html#Number){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Number_003a-shortcuts-and-iterators}

#### 1.122.12 Number: shortcuts and iterators {#number-shortcuts-and-iterators .subsection}

[]{#index-to_003a}

**to: stop**

Return an interval going from the receiver to stop by 1

[]{#index-to_003aby_003a}

**to: stop by: step**

Return an interval going from the receiver to stop with the given step

[]{#index-to_003aby_003acollect_003a}

**to: stop by: step collect: aBlock**

Evaluate aBlock for each value in the interval going from the receiver
to stop with the given step. The results are collected in an Array and
returned.

[]{#index-to_003aby_003ado_003a}

**to: stop by: step do: aBlock**

Evaluate aBlock for each value in the interval going from the receiver
to stop with the given step. Compiled in-line for integer literal steps,
and for one-argument aBlocks without temporaries, and therefore not
overridable.

[]{#index-to_003acollect_003a}

**to: stop collect: aBlock**

Evaluate aBlock for each value in the interval going from the receiver
to stop by 1. The results are collected in an Array and returned.

[]{#index-to_003ado_003a}

**to: stop do: aBlock**

Evaluate aBlock for each value in the interval going from the receiver
to stop by 1. Compiled in-line for one-argument aBlocks without
temporaries, and therefore not overridable.
