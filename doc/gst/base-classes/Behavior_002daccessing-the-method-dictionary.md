[]{#Behavior_002daccessing-the-method-dictionary}

::: header
Next: [Behavior-built
ins](Behavior_002dbuilt-ins.html#Behavior_002dbuilt-ins){accesskey="n"
rel="next"}, Previous: [Behavior-accessing instances and
variables](Behavior_002daccessing-instances-and-variables.html#Behavior_002daccessing-instances-and-variables){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-accessing-the-method-dictionary}

#### 1.9.3 Behavior: accessing the method dictionary {#behavior-accessing-the-method-dictionary .subsection}

[]{#index-_003e_003e}

**\>\> selector**

Return the compiled method associated with selector, from the local
method dictionary. Error if not found.

[]{#index-allSelectors}

**allSelectors**

Answer a Set of all the selectors understood by the receiver

[]{#index-compiledMethodAt_003a}

**compiledMethodAt: selector**

Return the compiled method associated with selector, from the local
method dictionary. Error if not found.

[]{#index-compiledMethodAt_003aifAbsent_003a}

**compiledMethodAt: selector ifAbsent: aBlock**

Return the compiled method associated with selector, from the local
method dictionary. Evaluate aBlock if not found.

[]{#index-formattedSourceStringAt_003a}

**formattedSourceStringAt: selector**

Answer the method source code as a formatted string (if available) for
the given selector. Requires package Parser.

[]{#index-lookupAllSelectors_003a}

**lookupAllSelectors: aSelector**

Answer a Set of all the compiled method associated with selector. from
the local method dictionary and all of the superclasses.

[]{#index-lookupSelector_003a}

**lookupSelector: aSelector**

Return the compiled method associated with selector, from the local
method dictionary or one of a superclass; return nil if not found.

[]{#index-parseTreeFor_003a}

**parseTreeFor: selector**

Answer the parse tree for the given selector, or nil if there was an
error. Requires the Parser package to be loaded.

[]{#index-selectorAt_003a}

**selectorAt: method**

Return selector for the given CompiledMethod

[]{#index-selectors}

**selectors**

Answer a Set of the receiver's selectors

[]{#index-sourceCodeAt_003a}

**sourceCodeAt: selector**

Answer source code (if available) for the given selector.

[]{#index-sourceCodeAt_003aifAbsent_003a}

**sourceCodeAt: selector ifAbsent: aBlock**

Answer source code (if available) for the given selector.

[]{#index-sourceMethodAt_003a}

**sourceMethodAt: selector**

This is too dependent on the original implementation

------------------------------------------------------------------------

::: header
Next: [Behavior-built
ins](Behavior_002dbuilt-ins.html#Behavior_002dbuilt-ins){accesskey="n"
rel="next"}, Previous: [Behavior-accessing instances and
variables](Behavior_002daccessing-instances-and-variables.html#Behavior_002daccessing-instances-and-variables){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
