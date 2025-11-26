[]{#Object_002dVM-callbacks}

::: header
Previous: [Object-testing
functionality](Object_002dtesting-functionality.html#Object_002dtesting-functionality){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-VM-callbacks}

#### 1.123.19 Object: VM callbacks {#object-vm-callbacks .subsection}

[]{#index-badReturnError}

**badReturnError**

Called back when a block performs a bad return.

[]{#index-mustBeBoolean}

**mustBeBoolean**

Called by the system when ifTrue:\*, ifFalse:\*, and: or or: are sent to
anything but a boolean

[]{#index-noRunnableProcess}

**noRunnableProcess**

Called back when all processes are suspended

[]{#index-userInterrupt}

**userInterrupt**

Called back when the user presses Ctrl-Break
