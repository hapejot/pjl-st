[]{#Behavior}

::: header
Next:
[BindingDictionary](BindingDictionary.html#BindingDictionary){accesskey="n"
rel="next"}, Previous: [Bag](Bag.html#Bag){accesskey="p" rel="prev"},
Up: [Base classes](Base-classes.html#Base-classes){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior-1}

### 1.9 Behavior {#behavior .section}

[]{#index-Behavior}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Implementation**

:   I am the parent class of all \"class\" type methods. My instances
    know about the subclass/superclass relationships between classes,
    contain the description that instances are created from, and hold
    the method dictionary that's associated with each class. I provide
    methods for compiling methods, modifying the class inheritance
    hierarchy, examining the method dictionary, and iterating over the
    class hierarchy.

  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ ---- ------------
  • [Behavior-accessing class hierarchy](Behavior_002daccessing-class-hierarchy.html#Behavior_002daccessing-class-hierarchy){accesskey="1"}:                                                      (instance)
  • [Behavior-accessing instances and variables](Behavior_002daccessing-instances-and-variables.html#Behavior_002daccessing-instances-and-variables){accesskey="2"}:                              (instance)
  • [Behavior-accessing the method dictionary](Behavior_002daccessing-the-method-dictionary.html#Behavior_002daccessing-the-method-dictionary){accesskey="3"}:                                    (instance)
  • [Behavior-built ins](Behavior_002dbuilt-ins.html#Behavior_002dbuilt-ins){accesskey="4"}:                                                                                                      (instance)
  • [Behavior-builtin](Behavior_002dbuiltin.html#Behavior_002dbuiltin){accesskey="5"}:                                                                                                            (instance)
  • [Behavior-compilation](Behavior_002dcompilation.html#Behavior_002dcompilation){accesskey="6"}:                                                                                                (instance)
  • [Behavior-compilation (alternative)](Behavior_002dcompilation-_0028alternative_0029.html#Behavior_002dcompilation-_0028alternative_0029){accesskey="7"}:                                      (instance)
  • [Behavior-compiling](Behavior_002dcompiling.html#Behavior_002dcompiling){accesskey="8"}:                                                                                                      (instance)
  • [Behavior-compiling methods](Behavior_002dcompiling-methods.html#Behavior_002dcompiling-methods){accesskey="9"}:                                                                              (instance)
  • [Behavior-creating a class hierarchy](Behavior_002dcreating-a-class-hierarchy.html#Behavior_002dcreating-a-class-hierarchy):                                                                  (instance)
  • [Behavior-enumerating](Behavior_002denumerating.html#Behavior_002denumerating):                                                                                                               (instance)
  • [Behavior-evaluating](Behavior_002devaluating.html#Behavior_002devaluating):                                                                                                                  (instance)
  • [Behavior-instance creation](Behavior_002dinstance-creation.html#Behavior_002dinstance-creation):                                                                                             (instance)
  • [Behavior-instance variables](Behavior_002dinstance-variables.html#Behavior_002dinstance-variables):                                                                                          (instance)
  • [Behavior-method dictionary](Behavior_002dmethod-dictionary.html#Behavior_002dmethod-dictionary):                                                                                             (instance)
  • [Behavior-parsing class declarations](Behavior_002dparsing-class-declarations.html#Behavior_002dparsing-class-declarations):                                                                  (instance)
  • [Behavior-pluggable behavior (not yet implemented)](Behavior_002dpluggable-behavior-_0028not-yet-implemented_0029.html#Behavior_002dpluggable-behavior-_0028not-yet-implemented_0029):        (instance)
  • [Behavior-printing hierarchy](Behavior_002dprinting-hierarchy.html#Behavior_002dprinting-hierarchy):                                                                                          (instance)
  • [Behavior-source code](Behavior_002dsource-code.html#Behavior_002dsource-code):                                                                                                               (instance)
  • [Behavior-still unclassified](Behavior_002dstill-unclassified.html#Behavior_002dstill-unclassified):                                                                                          (instance)
  • [Behavior-support for lightweight classes](Behavior_002dsupport-for-lightweight-classes.html#Behavior_002dsupport-for-lightweight-classes):                                                   (instance)
  • [Behavior-testing functionality](Behavior_002dtesting-functionality.html#Behavior_002dtesting-functionality):                                                                                 (instance)
  • [Behavior-testing the class hierarchy](Behavior_002dtesting-the-class-hierarchy.html#Behavior_002dtesting-the-class-hierarchy):                                                               (instance)
  • [Behavior-testing the form of the instances](Behavior_002dtesting-the-form-of-the-instances.html#Behavior_002dtesting-the-form-of-the-instances):                                             (instance)
  • [Behavior-testing the method dictionary](Behavior_002dtesting-the-method-dictionary.html#Behavior_002dtesting-the-method-dictionary):                                                         (instance)
  ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ ---- ------------

------------------------------------------------------------------------

::: header
Next:
[BindingDictionary](BindingDictionary.html#BindingDictionary){accesskey="n"
rel="next"}, Previous: [Bag](Bag.html#Bag){accesskey="p" rel="prev"},
Up: [Base classes](Base-classes.html#Base-classes){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
