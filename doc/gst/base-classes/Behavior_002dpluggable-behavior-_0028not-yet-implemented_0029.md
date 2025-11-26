[]{#Behavior_002dpluggable-behavior-_0028not-yet-implemented_0029}

::: header
Next: [Behavior-printing
hierarchy](Behavior_002dprinting-hierarchy.html#Behavior_002dprinting-hierarchy){accesskey="n"
rel="next"}, Previous: [Behavior-parsing class
declarations](Behavior_002dparsing-class-declarations.html#Behavior_002dparsing-class-declarations){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-pluggable-behavior-_0028not-yet-implemented_0029}

#### 1.9.17 Behavior: pluggable behavior (not yet implemented) {#behavior-pluggable-behavior-not-yet-implemented .subsection}

[]{#index-debuggerClass} []{#index-debuggingPriority}

**debuggerClass**

Answer which class is to be used to debug a chain of contexts which
includes the receiver. nil means 'do not debug'; other classes are sent
#debuggingPriority and the one with the highest priority is picked.

[]{#index-decompilerClass}

**decompilerClass**

Answer the class that can be used to decompile methods, or nil if there
is none (as is the case now).

[]{#index-evaluatorClass}

**evaluatorClass**

Answer the class that can be used to evaluate doits, or nil if there is
none (as is the case now).

[]{#index-parserClass}

**parserClass**

Answer the class that can be used to parse methods, or nil if there is
none (as is the case now).
