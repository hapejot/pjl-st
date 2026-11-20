[]{#PluggableAdaptor}

::: header
Next: [PluggableProxy](PluggableProxy.html#PluggableProxy){accesskey="n"
rel="next"}, Previous:
[Permission](Permission.html#Permission){accesskey="p" rel="prev"}, Up:
[Base classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#PluggableAdaptor-1}

### 1.130 PluggableAdaptor {#pluggableadaptor .section}

[]{#index-PluggableAdaptor}

**Defined in namespace Smalltalk**\
**Superclass: ValueAdaptor**\
**Category: Language-Data types**

:   I mediate between complex get/set behavior and the #value/#value:
    protocol used by ValueAdaptors. The get/set behavior can be
    implemented by two blocks, or can be delegated to another object
    with messages such as #someProperty to get and #someProperty: to
    set.

  ----------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [PluggableAdaptor class-creating instances](PluggableAdaptor-class_002dcreating-instances.html#PluggableAdaptor-class_002dcreating-instances){accesskey="1"}:        (class)
  • [PluggableAdaptor-accessing](PluggableAdaptor_002daccessing.html#PluggableAdaptor_002daccessing){accesskey="2"}:                                                     (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
