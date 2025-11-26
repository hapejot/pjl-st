[]{#DynamicVariable}

::: header
Next: [Error](Error.html#Error){accesskey="n" rel="next"}, Previous:
[Duration](Duration.html#Duration){accesskey="p" rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DynamicVariable-1}

### 1.70 DynamicVariable {#dynamicvariable .section}

[]{#index-DynamicVariable}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Utilities**

:   I am a variable that is visible only in the stackframes outgoing
    from this one. Do not use DynamicVariable directly, instead create a
    subclass for each variable you want to use.

    You can override the #value class method, and call #valueIfAbsent:
    from there if you want the default value to be something else than
    nil.

  -------------------------------------------------------------------------------------------------------------------------------------- ---- ---------
  • [DynamicVariable class-evaluating](DynamicVariable-class_002devaluating.html#DynamicVariable-class_002devaluating){accesskey="1"}:        (class)
  -------------------------------------------------------------------------------------------------------------------------------------- ---- ---------
