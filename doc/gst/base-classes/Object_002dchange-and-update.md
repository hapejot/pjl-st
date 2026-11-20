[]{#Object_002dchange-and-update}

::: header
Next: [Object-class type
methods](Object_002dclass-type-methods.html#Object_002dclass-type-methods){accesskey="n"
rel="next"}, Previous: [Object-built
ins](Object_002dbuilt-ins.html#Object_002dbuilt-ins){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-change-and-update}

#### 1.123.3 Object: change and update {#object-change-and-update .subsection}

[]{#index-broadcast_003a}

**broadcast: aSymbol**

Send the unary message aSymbol to each of the receiver's dependents

[]{#index-broadcast_003awith_003a}

**broadcast: aSymbol with: anObject**

Send the message aSymbol to each of the receiver's dependents, passing
anObject

[]{#index-broadcast_003awith_003awith_003a}

**broadcast: aSymbol with: arg1 with: arg2**

Send the message aSymbol to each of the receiver's dependents, passing
arg1 and arg2 as parameters

[]{#index-broadcast_003awithArguments_003a}

**broadcast: aSymbol withArguments: anArray**

Send the message aSymbol to each of the receiver's dependents, passing
the parameters in anArray

[]{#index-broadcast_003awithBlock_003a}

**broadcast: aSymbol withBlock: aBlock**

Send the message aSymbol to each of the receiver's dependents, passing
the result of evaluating aBlock with each dependent as the parameter

[]{#index-changed}

**changed**

Send update: for each of the receiver's dependents, passing them the
receiver

[]{#index-changed_003a}

**changed: aParameter**

Send update: for each of the receiver's dependents, passing them
aParameter

[]{#index-update_003a-4} []{#index-changed-1} []{#index-changed_003a-2}

**update: aParameter**

Default behavior is to do nothing. Called by #changed and #changed:
