[]{#ProcessVariable-class_002daccessing}

::: header
Next:
[ProcessVariable-accessing](ProcessVariable_002daccessing.html#ProcessVariable_002daccessing){accesskey="n"
rel="next"}, Up:
[ProcessVariable](ProcessVariable.html#ProcessVariable){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ProcessVariable-class_003a-accessing}

#### 1.137.1 ProcessVariable class: accessing {#processvariable-class-accessing .subsection}

[]{#index-key_003a-2}

**key: anObject**

Return a new ProcessVariable with the given key. Not that the key need
not be a symbol or string, for example you could use an array #(#{class
name} 'name'). Setting the variable's value will automatically create it
in the current process, while removal must be done by hand through the
ProcessEnvironment singleton object.

[]{#index-new-20}

**new**

Return a new ProcessVariable with a new anonymous but unique key. It is
suggested to use a descriptive name instead to ease debugging. Setting
the variable's value will automatically create it in the current
process, while removal must be done by hand through the
ProcessEnvironment singleton object.
