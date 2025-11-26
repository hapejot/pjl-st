[]{#Process_002daccessing}

::: header
Next:
[Process-basic](Process_002dbasic.html#Process_002dbasic){accesskey="n"
rel="next"}, Up: [Process](Process.html#Process){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Process_003a-accessing}

#### 1.134.1 Process: accessing {#process-accessing .subsection}

[]{#index-externalInterruptsEnabled}

**externalInterruptsEnabled**

Answer whether the receiver is executed with interrupts enabled

[]{#index-name-11}

**name**

Answer the user-friendly name of the process.

[]{#index-name_003a-4}

**name: aString**

Give the name aString to the process

[]{#index-priority}

**priority**

Answer the receiver's priority

[]{#index-priority_003a}

**priority: anInteger**

Change the receiver's priority to anInteger

[]{#index-queueInterrupt_003a}

**queueInterrupt: aBlock**

Force the receiver to be interrupted and to evaluate aBlock as soon as
it becomes the active process (this could mean NOW if the receiver is
active). If the process is temporarily suspended or waiting on a
semaphore, it is temporarily woken up so that the interrupt is processed
as soon as the process priority allows to do. Answer the receiver.

[]{#index-suspendedContext}

**suspendedContext**

Answer the context that the process was executing at the time it was
suspended.

[]{#index-suspendedContext_003a}

**suspendedContext: aContext**

Modify the context that the process was executing at the time it was
suspended.

[]{#index-valueWithoutInterrupts_003a}

**valueWithoutInterrupts: aBlock**

Evaluate aBlock and delay all interrupts that are requested during its
execution to after aBlock returns.
