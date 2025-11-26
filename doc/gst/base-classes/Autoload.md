[]{#Autoload}

::: header
Next: [Bag](Bag.html#Bag){accesskey="n" rel="next"}, Previous:
[Association](Association.html#Association){accesskey="p" rel="prev"},
Up: [Base classes](Base-classes.html#Base-classes){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Autoload-1}

### 1.7 Autoload {#autoload .section}

[]{#index-Autoload}

**Defined in namespace Smalltalk**\
**Superclass: none**\
**Category: Examples-Useful tools**

:   I am not a part of the normal Smalltalk kernel class system. I
    provide the ability to do late (\"on-demand\") loading of class
    definitions. Through me, you can define any class to be loaded when
    any message is sent to the class itself (such as to create an
    instance) or to its metaclass (such as #methodsFor: to extend it
    with class-side methods).

  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [Autoload class-instance creation](Autoload-class_002dinstance-creation.html#Autoload-class_002dinstance-creation){accesskey="1"}:        (class)
  • [Autoload-accessing](Autoload_002daccessing.html#Autoload_002daccessing){accesskey="2"}:                                                  (instance)
  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
