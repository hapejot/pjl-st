[]{#SystemDictionary}

::: header
Next:
[SystemExceptions.AlreadyDefined](SystemExceptions_002eAlreadyDefined.html#SystemExceptions_002eAlreadyDefined){accesskey="n"
rel="next"}, Previous: [SymLink](SymLink.html#SymLink){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SystemDictionary-1}

### 1.161 SystemDictionary {#systemdictionary .section}

[]{#index-SystemDictionary}

**Defined in namespace Smalltalk**\
**Superclass: RootNamespace**\
**Category: Language-Implementation**

:   I am a special namespace. I only have one instance, called
    \"Smalltalk\", which is known to the Smalltalk interpreter. I define
    several methods that are \"system\" related, such as #quitPrimitive.
    My instance also helps keep track of dependencies between objects.

  ----------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [SystemDictionary class-initialization](SystemDictionary-class_002dinitialization.html#SystemDictionary-class_002dinitialization){accesskey="1"}:        (class)
  • [SystemDictionary-basic](SystemDictionary_002dbasic.html#SystemDictionary_002dbasic){accesskey="2"}:                                                     (instance)
  • [SystemDictionary-builtins](SystemDictionary_002dbuiltins.html#SystemDictionary_002dbuiltins){accesskey="3"}:                                            (instance)
  • [SystemDictionary-c call-outs](SystemDictionary_002dc-call_002douts.html#SystemDictionary_002dc-call_002douts){accesskey="4"}:                           (instance)
  • [SystemDictionary-command-line](SystemDictionary_002dcommand_002dline.html#SystemDictionary_002dcommand_002dline){accesskey="5"}:                        (instance)
  • [SystemDictionary-miscellaneous](SystemDictionary_002dmiscellaneous.html#SystemDictionary_002dmiscellaneous){accesskey="6"}:                             (instance)
  • [SystemDictionary-printing](SystemDictionary_002dprinting.html#SystemDictionary_002dprinting){accesskey="7"}:                                            (instance)
  • [SystemDictionary-profiling](SystemDictionary_002dprofiling.html#SystemDictionary_002dprofiling){accesskey="8"}:                                         (instance)
  • [SystemDictionary-special accessing](SystemDictionary_002dspecial-accessing.html#SystemDictionary_002dspecial-accessing){accesskey="9"}:                 (instance)
  • [SystemDictionary-testing](SystemDictionary_002dtesting.html#SystemDictionary_002dtesting):                                                              (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
