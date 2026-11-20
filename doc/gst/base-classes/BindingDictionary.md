[]{#BindingDictionary}

::: header
Next: [BlockClosure](BlockClosure.html#BlockClosure){accesskey="n"
rel="next"}, Previous: [Behavior](Behavior.html#Behavior){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BindingDictionary-1}

### 1.10 BindingDictionary {#bindingdictionary .section}

[]{#index-BindingDictionary}

**Defined in namespace Smalltalk**\
**Superclass: Dictionary**\
**Category: Language-Implementation**

:   I am a special form of dictionary that provides special ways to
    access my keys, which typically begin with an uppercase letter;
    also, my associations are actually VariableBinding instances.

    My keys are (expected to be) symbols, so I use == to match searched
    keys to those in the dictionary -- this is done expecting that it
    brings a bit more speed.

  -------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [BindingDictionary-accessing](BindingDictionary_002daccessing.html#BindingDictionary_002daccessing){accesskey="1"}:                                         (instance)
  • [BindingDictionary-basic & copying](BindingDictionary_002dbasic-_0026-copying.html#BindingDictionary_002dbasic-_0026-copying){accesskey="2"}:               (instance)
  • [BindingDictionary-copying](BindingDictionary_002dcopying.html#BindingDictionary_002dcopying){accesskey="3"}:                                               (instance)
  • [BindingDictionary-forward declarations](BindingDictionary_002dforward-declarations.html#BindingDictionary_002dforward-declarations){accesskey="4"}:        (instance)
  • [BindingDictionary-printing](BindingDictionary_002dprinting.html#BindingDictionary_002dprinting){accesskey="5"}:                                            (instance)
  • [BindingDictionary-testing](BindingDictionary_002dtesting.html#BindingDictionary_002dtesting){accesskey="6"}:                                               (instance)
  -------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
