[]{#BlockClosure_002daccessing}

::: header
Next: [BlockClosure-built
ins](BlockClosure_002dbuilt-ins.html#BlockClosure_002dbuilt-ins){accesskey="n"
rel="next"}, Previous: [BlockClosure
class-testing](BlockClosure-class_002dtesting.html#BlockClosure-class_002dtesting){accesskey="p"
rel="prev"}, Up:
[BlockClosure](BlockClosure.html#BlockClosure){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockClosure_003a-accessing}

#### 1.11.3 BlockClosure: accessing {#blockclosure-accessing .subsection}

[]{#index-argumentCount}

**argumentCount**

Answer the number of arguments passed to the receiver

[]{#index-block}

**block**

Answer the CompiledBlock which contains the receiver's bytecodes

[]{#index-block_003a-1}

**block: aCompiledBlock**

Set the CompiledBlock which contains the receiver's bytecodes

[]{#index-finalIP}

**finalIP**

Answer the last instruction that can be executed by the receiver

[]{#index-fixTemps}

**fixTemps**

This should fix the values of the temporary variables used in the block
that are ordinarily shared with the method in which the block is
defined. Not defined yet, but it is not harmful that it isn't. Answer
the receiver.

[]{#index-initialIP}

**initialIP**

Answer the initial instruction pointer into the receiver.

[]{#index-method}

**method**

Answer the CompiledMethod in which the receiver lies

[]{#index-numArgs}

**numArgs**

Answer the number of arguments passed to the receiver

[]{#index-numTemps}

**numTemps**

Answer the number of temporary variables used by the receiver

[]{#index-outerContext}

**outerContext**

Answer the method/block context which is the immediate outer of the
receiver

[]{#index-outerContext_003a}

**outerContext: containingContext**

Set the method/block context which is the immediate outer of the
receiver

[]{#index-receiver}

**receiver**

Answer the object that is used as 'self' when executing the receiver (if
nil, it might mean that the receiver is not valid though\...)

[]{#index-receiver_003a}

**receiver: anObject**

Set the object that is used as 'self' when executing the receiver

[]{#index-stackDepth}

**stackDepth**

Answer the number of stack slots needed for the receiver
