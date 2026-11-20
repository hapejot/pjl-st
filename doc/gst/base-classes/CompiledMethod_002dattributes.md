[]{#CompiledMethod_002dattributes}

::: header
Next:
[CompiledMethod-basic](CompiledMethod_002dbasic.html#CompiledMethod_002dbasic){accesskey="n"
rel="next"}, Previous:
[CompiledMethod-accessing](CompiledMethod_002daccessing.html#CompiledMethod_002daccessing){accesskey="p"
rel="prev"}, Up:
[CompiledMethod](CompiledMethod.html#CompiledMethod){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledMethod_003a-attributes}

#### 1.40.5 CompiledMethod: attributes {#compiledmethod-attributes .subsection}

[]{#index-attributeAt_003a}

**attributeAt: aSymbol**

Return a Message for the first attribute named aSymbol defined by the
receiver, or answer an error if none was found.

[]{#index-attributeAt_003aifAbsent_003a}

**attributeAt: aSymbol ifAbsent: aBlock**

Return a Message for the first attribute named aSymbol defined by the
receiver, or evaluate aBlock is none was found.

[]{#index-attributes}

**attributes**

Return an Array of Messages, one for each attribute defined by the
receiver.

[]{#index-attributesDo_003a}

**attributesDo: aBlock**

Evaluate aBlock once for each attribute defined by the receiver, passing
a Message each time.

[]{#index-isAnnotated-1}

**isAnnotated**

If the receiver has any attributes, answer true.

[]{#index-primitiveAttribute}

**primitiveAttribute**

If the receiver defines a primitive, return a Message resembling the
attribute that was used to define it.
