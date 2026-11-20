[]{#BlockClosure_002dmultiple-process}

::: header
Next:
[BlockClosure-overriding](BlockClosure_002doverriding.html#BlockClosure_002doverriding){accesskey="n"
rel="next"}, Previous: [BlockClosure-exception
handling](BlockClosure_002dexception-handling.html#BlockClosure_002dexception-handling){accesskey="p"
rel="prev"}, Up:
[BlockClosure](BlockClosure.html#BlockClosure){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BlockClosure_003a-multiple-process}

#### 1.11.7 BlockClosure: multiple process {#blockclosure-multiple-process .subsection}

[]{#index-fork}

**fork**

Create a new process executing the receiver and start it

[]{#index-forkAt_003a}

**forkAt: priority**

Create a new process executing the receiver with given priority and
start it

[]{#index-forkWithoutPreemption}

**forkWithoutPreemption**

Evaluate the receiver in a process that cannot be preempted. If the
receiver expect a parameter, pass the current process.

[]{#index-newProcess}

**newProcess**

Create a new process executing the receiver in suspended state. The
priority is the same as for the calling process. The receiver must not
contain returns

[]{#index-newProcessWith_003a}

**newProcessWith: anArray**

Create a new process executing the receiver with the passed arguments,
and leave it in suspended state. The priority is the same as for the
calling process. The receiver must not contain returns

[]{#index-valueWithoutInterrupts}

**valueWithoutInterrupts**

Evaluate aBlock and delay all interrupts that are requested to the
active process during its execution to after aBlock returns.

[]{#index-valueWithoutPreemption}

**valueWithoutPreemption**

Evaluate the receiver with external interrupts disabled. This
effectively disables preemption as long as the block does not explicitly
yield control, wait on semaphores, and the like.
