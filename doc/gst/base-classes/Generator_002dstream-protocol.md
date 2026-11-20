[]{#Generator_002dstream-protocol}

::: header
Previous: [Generator class-instance
creation](Generator-class_002dinstance-creation.html#Generator-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [Generator](Generator.html#Generator){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Generator_003a-stream-protocol}

#### 1.85.2 Generator: stream protocol {#generator-stream-protocol .subsection}

[]{#index-atEnd-2}

**atEnd**

Answer whether more data can be generated.

[]{#index-next-2}

**next**

Evaluate the generator until it generates the next value or decides that
nothing else can be generated.

[]{#index-peek-2} []{#index-peek-6} []{#index-next-10}

**peek**

Evaluate the generator until it generates the next value or decides that
nothing else can be generated, and save the value so that #peek or #next
will return it again.

[]{#index-peekFor_003a-1} []{#index-peek-7} []{#index-next-11}

**peekFor: anObject**

Evaluate the generator until it generates the next value or decides that
nothing else can be generated, and if it is not equal to anObject, save
the value so that #peek or #next will return it again.

[]{#index-yield_003a}

**yield: anObject**

When entering from the generator the code in the block is executed and
control flow goes back to the consumer. When entering from the consumer,
the code after the continuation is executed, which resumes execution of
the generator block.
