[]{#Process_002dbasic}

::: header
Next:
[Process-builtins](Process_002dbuiltins.html#Process_002dbuiltins){accesskey="n"
rel="next"}, Previous:
[Process-accessing](Process_002daccessing.html#Process_002daccessing){accesskey="p"
rel="prev"}, Up: [Process](Process.html#Process){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Process_003a-basic}

#### 1.134.2 Process: basic {#process-basic .subsection}

[]{#index-context-1}

**context**

Return the execution context of the receiver.

[]{#index-debugger-1}

**debugger**

Return the object in charge of debugging the receiver. This always
returns nil unless the DebugTools package is loaded.

[]{#index-finalize-3}

**finalize**

Terminate processes that are GCed while waiting on a dead semaphore.

[]{#index-lowerPriority} []{#index-lowerPriority-1}
[]{#index-raisePriority-1}

**lowerPriority**

Lower a bit the priority of the receiver. A #lowerPriority will cancel a
previous #raisePriority, and vice versa.

[]{#index-makeUntrusted_003a-1}

**makeUntrusted: aBoolean**

Set whether the receiver is trusted or not.

[]{#index-primTerminate}

**primTerminate**

Terminate the receiver - This is nothing more than prohibiting to resume
the process, then suspending it.

[]{#index-raisePriority} []{#index-lowerPriority-2}
[]{#index-raisePriority-2}

**raisePriority**

Raise a bit the priority of the receiver. A #lowerPriority will cancel a
previous #raisePriority, and vice versa.

[]{#index-singleStep} []{#index-singleStepWaitingOn_003a-1}

**singleStep**

Execute a limited amount of code (usually a bytecode, or up to the next
backward jump, or up to the next message send) of the receiver, which
must in a ready-to-run state (neither executing nor terminating nor
suspended), then restart running the current process. The current
process should have higher priority than the receiver. For better
performance, use the underlying primitive,
Process\>\>#singleStepWaitingOn:.

[]{#index-terminate} []{#index-ensure_003a-9}
[]{#index-ifCurtailed_003a-2}

**terminate**

Terminate the receiver after having evaluated all the #ensure: and
#ifCurtailed: blocks that are active in it. This is done by signalling a
ProcessBeingTerminated notification.

[]{#index-terminateOnQuit} []{#index-quit_003a-1}

**terminateOnQuit**

Mark the receiver so that it is terminated when ObjectMemory
class\>\>#quit: is sent.

------------------------------------------------------------------------

::: header
Next:
[Process-builtins](Process_002dbuiltins.html#Process_002dbuiltins){accesskey="n"
rel="next"}, Previous:
[Process-accessing](Process_002daccessing.html#Process_002daccessing){accesskey="p"
rel="prev"}, Up: [Process](Process.html#Process){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
