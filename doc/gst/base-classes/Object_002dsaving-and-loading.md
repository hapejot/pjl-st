[]{#Object_002dsaving-and-loading}

::: header
Next:
[Object-storing](Object_002dstoring.html#Object_002dstoring){accesskey="n"
rel="next"}, Previous: [Object-relational
operators](Object_002drelational-operators.html#Object_002drelational-operators){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-saving-and-loading}

#### 1.123.15 Object: saving and loading {#object-saving-and-loading .subsection}

[]{#index-binaryRepresentationObject-2}

**binaryRepresentationObject**

This method must be implemented if PluggableProxies are used with the
receiver's class. The default implementation raises an exception.

[]{#index-postLoad-1}

**postLoad**

Called after loading an object; must restore it to the state before
'preStore' was called. Do nothing by default

[]{#index-postStore-1} []{#index-postLoad-7}

**postStore**

Called after an object is dumped; must restore it to the state before
'preStore' was called. Call #postLoad by default

[]{#index-preStore}

**preStore**

Called before dumping an object; it must \*change\* it (it must not
answer a new object) if necessary. Do nothing by default

[]{#index-reconstructOriginalObject-1}
[]{#index-binaryRepresentationObject-4}

**reconstructOriginalObject**

Used if an instance of the receiver's class is returned as the
#binaryRepresentationObject of another object. The default
implementation raises an exception.
