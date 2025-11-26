[]{#DeferredVariableBinding-class_002dbasic}

::: header
Next:
[DeferredVariableBinding-basic](DeferredVariableBinding_002dbasic.html#DeferredVariableBinding_002dbasic){accesskey="n"
rel="next"}, Up:
[DeferredVariableBinding](DeferredVariableBinding.html#DeferredVariableBinding){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DeferredVariableBinding-class_003a-basic}

#### 1.61.1 DeferredVariableBinding class: basic {#deferredvariablebinding-class-basic .subsection}

[]{#index-key_003aclass_003adefaultDictionary_003a}
[]{#index-resolveBinding}

**key: aSymbol class: aClass defaultDictionary: aDictionary**

Answer a binding that will look up aSymbol as a variable in aClass's
environment at first access. See #resolveBinding's comment for
aDictionary's meaning.

[]{#index-path_003aclass_003adefaultDictionary_003a}
[]{#index-key_003aclass_003adefaultDictionary_003a-1}

**path: anArray class: aClass defaultDictionary: aDictionary**

As with #key:class:defaultDictionary:, but accepting an array of
symbols, representing a namespace path, instead.
