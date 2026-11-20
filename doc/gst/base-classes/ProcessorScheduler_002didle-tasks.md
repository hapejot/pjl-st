[]{#ProcessorScheduler_002didle-tasks}

::: header
Next:
[ProcessorScheduler-printing](ProcessorScheduler_002dprinting.html#ProcessorScheduler_002dprinting){accesskey="n"
rel="next"}, Previous: [ProcessorScheduler-built
ins](ProcessorScheduler_002dbuilt-ins.html#ProcessorScheduler_002dbuilt-ins){accesskey="p"
rel="prev"}, Up:
[ProcessorScheduler](ProcessorScheduler.html#ProcessorScheduler){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ProcessorScheduler_003a-idle-tasks}

#### 1.136.4 ProcessorScheduler: idle tasks {#processorscheduler-idle-tasks .subsection}

[]{#index-idle}

**idle**

Private - Call the next idle task. Return whether GNU Smalltalk should
pause until the next OS signal.

[]{#index-idleAdd_003a}

**idleAdd: aBlock**

Register aBlock to be executed when things are idle

[]{#index-initialize-19}

**initialize**

Private - Start the finalization process.

[]{#index-pause_003a}

**pause: aBoolean**

Private - Pause for some time if aBoolean is false, or until a signal if
it is true.

[]{#index-startFinalizers}

**startFinalizers**

Private - Fire a low-priority process to finalize the objects

[]{#index-update_003a-5}

**update: aSymbol**

If we left some work behind when the image was saved, do it now.
