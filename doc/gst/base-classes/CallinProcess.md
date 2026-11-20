[]{#CallinProcess}

::: header
Next: [CArray](CArray.html#CArray){accesskey="n" rel="next"}, Previous:
[CAggregate](CAggregate.html#CAggregate){accesskey="p" rel="prev"}, Up:
[Base classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CallinProcess-1}

### 1.16 CallinProcess {#callinprocess .section}

[]{#index-CallinProcess}

**Defined in namespace Smalltalk**\
**Superclass: Process**\
**Category: Language-Processes**

:   I represent a unit of computation for which external C code
    requested execution, so I must store the returned value once my
    computation terminates and I must not survive across image saves
    (since those who invoked me no longer exist). I am otherwise
    equivalent to a Process.

  ----------------------------------------------------------------------------------------------------------- ---- ------------
  • [CallinProcess-debugging](CallinProcess_002ddebugging.html#CallinProcess_002ddebugging){accesskey="1"}:        (instance)
  ----------------------------------------------------------------------------------------------------------- ---- ------------
