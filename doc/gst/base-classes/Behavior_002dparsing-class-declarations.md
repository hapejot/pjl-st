[]{#Behavior_002dparsing-class-declarations}

::: header
Next: [Behavior-pluggable behavior (not yet
implemented)](Behavior_002dpluggable-behavior-_0028not-yet-implemented_0029.html#Behavior_002dpluggable-behavior-_0028not-yet-implemented_0029){accesskey="n"
rel="next"}, Previous: [Behavior-method
dictionary](Behavior_002dmethod-dictionary.html#Behavior_002dmethod-dictionary){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-parsing-class-declarations}

#### 1.9.16 Behavior: parsing class declarations {#behavior-parsing-class-declarations .subsection}

[]{#index-parseInstanceVariableString_003a}
[]{#index-parseVariableString_003a-1}

**parseInstanceVariableString: variableString**

As with #parseVariableString:, but answer symbols that name the
variables instead of strings.

[]{#index-parseVariableString_003a}

**parseVariableString: aString**

Answer an array of instance variable names. aString should specify these
in traditional file-in 'instanceVariableNames' format. Signal an error
if aString contains something other than valid Smalltalk variables.
