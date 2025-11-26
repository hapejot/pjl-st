[]{#Generator-class_002dinstance-creation}

::: header
Next: [Generator-stream
protocol](Generator_002dstream-protocol.html#Generator_002dstream-protocol){accesskey="n"
rel="next"}, Up: [Generator](Generator.html#Generator){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Generator-class_003a-instance-creation}

#### 1.85.1 Generator class: instance creation {#generator-class-instance-creation .subsection}

[]{#index-inject_003ainto_003a}

**inject: aValue into: aBlock**

Return an infinite generator; the first item is aValue, the following
items are obtained by passing the previous value to aBlock.

[]{#index-on_003a-3} []{#index-next-9} []{#index-yield_003a-1}

**on: aBlock**

Return a generator and pass it to aBlock. When #next is sent to the
generator, the block will start execution, and will be suspended again
as soon as #yield: is sent from the block to the generator.

[]{#index-on_003ado_003a-1}

**on: aCollection do: aBlock**

Return a generator; for each item of aCollection, evaluate aBlock
passing the generator and the item.
