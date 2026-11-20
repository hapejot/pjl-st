[]{#ProcessVariable_002daccessing}

::: header
Previous: [ProcessVariable
class-accessing](ProcessVariable-class_002daccessing.html#ProcessVariable-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[ProcessVariable](ProcessVariable.html#ProcessVariable){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ProcessVariable_003a-accessing}

#### 1.137.2 ProcessVariable: accessing {#processvariable-accessing .subsection}

[]{#index-environment-7}

**environment**

Return the environment in which this ProcessVariable lives. This is the
singleton instance of ProcessEnvironment for all variables.

[]{#index-use_003aduring_003a-1}

**use: anObject during: aBlock**

Set the value of this variable to anObject during the execution of
aBlock, then restore it.

[]{#index-value-15}

**value**

Return the value of this variable in the current process.

[]{#index-value_003a-16}

**value: anObject**

Set the value of the current process's copy of the variable to be
anObject.

[]{#index-valueIfAbsent_003a-1}

**valueIfAbsent: aBlock**

Return the value of this variable in the current process.
