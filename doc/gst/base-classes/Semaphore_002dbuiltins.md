[]{#Semaphore_002dbuiltins}

::: header
Next: [Semaphore-mutual
exclusion](Semaphore_002dmutual-exclusion.html#Semaphore_002dmutual-exclusion){accesskey="n"
rel="next"}, Previous:
[Semaphore-accessing](Semaphore_002daccessing.html#Semaphore_002daccessing){accesskey="p"
rel="prev"}, Up: [Semaphore](Semaphore.html#Semaphore){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Semaphore_003a-builtins}

#### 1.150.3 Semaphore: builtins {#semaphore-builtins .subsection}

[]{#index-lock}

**lock**

Without putting the receiver to sleep, force processes that try to wait
on the semaphore to block. Answer whether this was the case even before.

[]{#index-notify}

**notify**

Resume one of the processes that were waiting on the semaphore if there
were any. Do not leave a signal on the semaphore if no process is
waiting.

[]{#index-notifyAll}

**notifyAll**

Resume all the processes that were waiting on the semaphore if there
were any. Do not leave a signal on the semaphore if no process is
waiting.

[]{#index-signal-2}

**signal**

Signal the receiver, resuming a waiting process' if there is one

[]{#index-wait-2}

**wait**

Wait for the receiver to be signalled, suspending the executing process
if it is not yet. Return nil if the wait was interrupted, the receiver
otherwise.

[]{#index-waitAfterSignalling_003a} []{#index-notify-1}
[]{#index-notifyAll-1}

**waitAfterSignalling: aSemaphore**

Signal aSemaphore then, atomically, wait for the receiver to be
signalled, suspending the executing process if it is not yet. This is
needed to avoid race conditions when the #notify and #notifyAll are used
before waiting on receiver: otherwise, if a process sends any of the two
between the time aSemaphore is signaled and the time the process starts
waiting on the receiver, the notification is lost.
