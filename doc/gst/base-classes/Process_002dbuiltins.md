[]{#Process_002dbuiltins}

::: header
Next:
[Process-debugging](Process_002ddebugging.html#Process_002ddebugging){accesskey="n"
rel="next"}, Previous:
[Process-basic](Process_002dbasic.html#Process_002dbasic){accesskey="p"
rel="prev"}, Up: [Process](Process.html#Process){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Process_003a-builtins}

#### 1.134.3 Process: builtins {#process-builtins .subsection}

[]{#index-resume-1}

**resume**

Resume the receiver's execution

[]{#index-singleStepWaitingOn_003a}

**singleStepWaitingOn: aSemaphore**

Execute a limited amount of code (usually a bytecode, or up to the next
backward jump, or up to the next message send) of the receiver, which
must in a ready-to-run state (neither executing nor terminating nor
suspended), then restart running the current process. aSemaphore is used
as a means to synchronize the execution of the current process and the
receiver and should have no signals on it. The current process should
have higher priority than the receiver.

[]{#index-suspend}

**suspend**

Do nothing if we're already suspended. Note that the blue book made
suspend a primitive - but the real primitive is yielding control to
another process. Suspending is nothing more than taking ourselves out of
every scheduling list and THEN yielding control to another process

[]{#index-yield}

**yield**

Yield control from the receiver to other processes
