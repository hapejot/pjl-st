[]{#CompiledBlock_002dsaving-and-loading}

::: header
Previous:
[CompiledBlock-printing](CompiledBlock_002dprinting.html#CompiledBlock_002dprinting){accesskey="p"
rel="prev"}, Up:
[CompiledBlock](CompiledBlock.html#CompiledBlock){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledBlock_003a-saving-and-loading}

#### 1.38.5 CompiledBlock: saving and loading {#compiledblock-saving-and-loading .subsection}

[]{#index-binaryRepresentationObject} []{#index-blockAt_003a-1}

**binaryRepresentationObject**

This method is implemented to allow for a PluggableProxy to be used with
CompiledBlocks. Answer a DirectedMessage which sends #blockAt: to the
CompiledMethod containing the receiver.
