[]{#Object_002dclass-type-methods}

::: header
Next:
[Object-compiler](Object_002dcompiler.html#Object_002dcompiler){accesskey="n"
rel="next"}, Previous: [Object-change and
update](Object_002dchange-and-update.html#Object_002dchange-and-update){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-class-type-methods}

#### 1.123.4 Object: class type methods {#object-class-type-methods .subsection}

[]{#index-species-2} []{#index-class-2} []{#index-species-8}
[]{#index-copyEmpty_003a-3}

**species**

This method has no unique definition. Generally speaking, methods which
always return the same type usually don't use #class, but #species. For
example, a PositionableStream's species is the class of the collection
on which it is streaming (used by upTo:, upToAll:, upToEnd). Stream uses
species for obtaining the class of next:'s return value, Collection uses
it in its #copyEmpty: message, which in turn is used by all
collection-returning methods. An Interval's species is Array (used by
collect:, select:, reject:, etc.).

[]{#index-yourself}

**yourself**

Answer the receiver
