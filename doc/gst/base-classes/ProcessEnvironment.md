[]{#ProcessEnvironment}

::: header
Next:
[ProcessorScheduler](ProcessorScheduler.html#ProcessorScheduler){accesskey="n"
rel="next"}, Previous: [Process](Process.html#Process){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ProcessEnvironment-1}

### 1.135 ProcessEnvironment {#processenvironment .section}

[]{#index-ProcessEnvironment}

**Defined in namespace Smalltalk**\
**Superclass: Object**\
**Category: Language-Processes**

:   I represent a proxy for thread-local variables defined for Smalltalk
    processes. Associations requested to me retrieve the thread-local
    value for the current process. For now, I don't provide the full
    protocol of a Dictionary; in particular the iteration protocol is
    absent.

  -------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [ProcessEnvironment class-disabled](ProcessEnvironment-class_002ddisabled.html#ProcessEnvironment-class_002ddisabled){accesskey="1"}:                       (class)
  • [ProcessEnvironment class-singleton](ProcessEnvironment-class_002dsingleton.html#ProcessEnvironment-class_002dsingleton){accesskey="2"}:                    (class)
  • [ProcessEnvironment-accessing](ProcessEnvironment_002daccessing.html#ProcessEnvironment_002daccessing){accesskey="3"}:                                      (instance)
  • [ProcessEnvironment-dictionary removing](ProcessEnvironment_002ddictionary-removing.html#ProcessEnvironment_002ddictionary-removing){accesskey="4"}:        (instance)
  • [ProcessEnvironment-dictionary testing](ProcessEnvironment_002ddictionary-testing.html#ProcessEnvironment_002ddictionary-testing){accesskey="5"}:           (instance)
  -------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
