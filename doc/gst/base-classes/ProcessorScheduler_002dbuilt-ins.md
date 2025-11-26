[]{#ProcessorScheduler_002dbuilt-ins}

::: header
Next: [ProcessorScheduler-idle
tasks](ProcessorScheduler_002didle-tasks.html#ProcessorScheduler_002didle-tasks){accesskey="n"
rel="next"}, Previous:
[ProcessorScheduler-basic](ProcessorScheduler_002dbasic.html#ProcessorScheduler_002dbasic){accesskey="p"
rel="prev"}, Up:
[ProcessorScheduler](ProcessorScheduler.html#ProcessorScheduler){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ProcessorScheduler_003a-built-ins}

#### 1.136.3 ProcessorScheduler: built ins {#processorscheduler-built-ins .subsection}

[]{#index-disableInterrupts} []{#index-disableInterrupts-1}
[]{#index-enableInterrupts-1}

**disableInterrupts**

Disable interrupts caused by external events while the current process
is executing. Note that interrupts are disabled on a per-process basis,
and that calling #disableInterrupts twice requires calling
#enableInterrupts twice as well to re-enable interrupts.

[]{#index-enableInterrupts}

**enableInterrupts**

Re-enable interrupts caused by external events while the current process
is executing. By default, interrupts are enabled.
