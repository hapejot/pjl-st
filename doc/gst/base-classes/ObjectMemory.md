[]{#ObjectMemory}

::: header
Next:
[OrderedCollection](OrderedCollection.html#OrderedCollection){accesskey="n"
rel="next"}, Previous:
[ObjectDumper](ObjectDumper.html#ObjectDumper){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ObjectMemory-1}

### 1.125 ObjectMemory {#objectmemory .section}

[]{#index-ObjectMemory}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Implementation**

:   I provide a few methods that enable one to tune the virtual
    machine's usage of memory. In addition, I can signal to my
    dependants some 'events' that can happen during the virtual
    machine's life.

    ObjectMemory has both class-side and instance-side methods. In
    general, class-side methods provide means to tune the parameters of
    the memory manager, while instance-side methods are used together
    with the #current class-side method to take a look at statistics on
    the memory manager's state.

  ----------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [ObjectMemory class-accessing](ObjectMemory-class_002daccessing.html#ObjectMemory-class_002daccessing){accesskey="1"}:                             (class)
  • [ObjectMemory class-builtins](ObjectMemory-class_002dbuiltins.html#ObjectMemory-class_002dbuiltins){accesskey="2"}:                                (class)
  • [ObjectMemory class-initialization](ObjectMemory-class_002dinitialization.html#ObjectMemory-class_002dinitialization){accesskey="3"}:              (class)
  • [ObjectMemory class-saving the image](ObjectMemory-class_002dsaving-the-image.html#ObjectMemory-class_002dsaving-the-image){accesskey="4"}:        (class)
  • [ObjectMemory-accessing](ObjectMemory_002daccessing.html#ObjectMemory_002daccessing){accesskey="5"}:                                               (instance)
  • [ObjectMemory-builtins](ObjectMemory_002dbuiltins.html#ObjectMemory_002dbuiltins){accesskey="6"}:                                                  (instance)
  • [ObjectMemory-derived information](ObjectMemory_002dderived-information.html#ObjectMemory_002dderived-information){accesskey="7"}:                 (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
