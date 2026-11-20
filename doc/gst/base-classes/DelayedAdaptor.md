[]{#DelayedAdaptor}

::: header
Next: [Dictionary](Dictionary.html#Dictionary){accesskey="n"
rel="next"}, Previous: [Delay](Delay.html#Delay){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#DelayedAdaptor-1}

### 1.63 DelayedAdaptor {#delayedadaptor .section}

[]{#index-DelayedAdaptor}

**Defined in namespace Smalltalk**\
**Superclass: PluggableAdaptor**\
**Category: Language-Data types**

:   I can be used where many expensive updates must be performed. My
    instances buffer the last value that was set, and only actually set
    the value when the #trigger message is sent. Apart from this, I'm
    equivalent to PluggableAdaptor.

  -------------------------------------------------------------------------------------------------------------- ---- ------------
  • [DelayedAdaptor-accessing](DelayedAdaptor_002daccessing.html#DelayedAdaptor_002daccessing){accesskey="1"}:        (instance)
  -------------------------------------------------------------------------------------------------------------- ---- ------------
