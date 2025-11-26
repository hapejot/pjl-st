[]{#CompiledMethod_002daccessing}

::: header
Next:
[CompiledMethod-attributes](CompiledMethod_002dattributes.html#CompiledMethod_002dattributes){accesskey="n"
rel="next"}, Previous: [CompiledMethod class-lean
images](CompiledMethod-class_002dlean-images.html#CompiledMethod-class_002dlean-images){accesskey="p"
rel="prev"}, Up:
[CompiledMethod](CompiledMethod.html#CompiledMethod){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledMethod_003a-accessing}

#### 1.40.4 CompiledMethod: accessing {#compiledmethod-accessing .subsection}

[]{#index-allBlocksDo_003a}

**allBlocksDo: aBlock**

Evaluate aBlock, passing to it all the CompiledBlocks it holds

[]{#index-allLiterals}

**allLiterals**

Answer the literals referred to by the receiver and all the blocks in it

[]{#index-flags-2}

**flags**

Private - Answer the optimization flags for the receiver

[]{#index-isOldSyntax}

**isOldSyntax**

Answer whether the method was written with the old (chunk-format) syntax

[]{#index-method-3}

**method**

Answer the receiver, since it is already a method.

[]{#index-methodCategory-2}

**methodCategory**

Answer the method category

[]{#index-methodCategory_003a-2}

**methodCategory: aCategory**

Set the method category to the given string

[]{#index-methodClass-2}

**methodClass**

Answer the class in which the receiver is installed.

[]{#index-methodClass_003a-2}

**methodClass: methodClass**

Set the receiver's class instance variable

[]{#index-noteOldSyntax}

**noteOldSyntax**

Remember that the method is written with the old (chunk-format) syntax

[]{#index-numArgs-3}

**numArgs**

Answer the number of arguments for the receiver

[]{#index-numTemps-3}

**numTemps**

Answer the number of temporaries for the receiver

[]{#index-primitive-1}

**primitive**

Answer the primitive called by the receiver

[]{#index-selector-2}

**selector**

Answer the selector through which the method is called

[]{#index-selector_003a-2}

**selector: aSymbol**

Set the selector through which the method is called

[]{#index-sourceCodeLinesDelta-2}

**sourceCodeLinesDelta**

Answer the delta from the numbers in LINE_NUMBER bytecodes to source
code line numbers.

[]{#index-stackDepth-3}

**stackDepth**

Answer the number of stack slots needed for the receiver

[]{#index-withAllBlocksDo_003a}

**withAllBlocksDo: aBlock**

Evaluate aBlock, passing the receiver and all the CompiledBlocks it
holds

[]{#index-withNewMethodClass_003a}

**withNewMethodClass: class**

Answer either the receiver or a copy of it, with the method class set to
class

[]{#index-withNewMethodClass_003aselector_003a}

**withNewMethodClass: class selector: selector**

Answer either the receiver or a copy of it, with the method class set to
class

------------------------------------------------------------------------

::: header
Next:
[CompiledMethod-attributes](CompiledMethod_002dattributes.html#CompiledMethod_002dattributes){accesskey="n"
rel="next"}, Previous: [CompiledMethod class-lean
images](CompiledMethod-class_002dlean-images.html#CompiledMethod-class_002dlean-images){accesskey="p"
rel="prev"}, Up:
[CompiledMethod](CompiledMethod.html#CompiledMethod){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
