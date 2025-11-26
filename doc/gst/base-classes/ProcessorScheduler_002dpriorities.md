[]{#ProcessorScheduler_002dpriorities}

::: header
Next:
[ProcessorScheduler-storing](ProcessorScheduler_002dstoring.html#ProcessorScheduler_002dstoring){accesskey="n"
rel="next"}, Previous:
[ProcessorScheduler-printing](ProcessorScheduler_002dprinting.html#ProcessorScheduler_002dprinting){accesskey="p"
rel="prev"}, Up:
[ProcessorScheduler](ProcessorScheduler.html#ProcessorScheduler){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ProcessorScheduler_003a-priorities}

#### 1.136.6 ProcessorScheduler: priorities {#processorscheduler-priorities .subsection}

[]{#index-highIOPriority}

**highIOPriority**

Answer the priority for system high-priority I/O processes, such as a
process handling input from a network.

[]{#index-highestPriority}

**highestPriority**

Answer the highest valid priority

[]{#index-idlePriority}

**idlePriority**

Answer the priority of idle processes.

[]{#index-lowIOPriority}

**lowIOPriority**

Answer the priority for system low-priority I/O processes. Examples are
the process handling input from the user (keyboard, pointing device,
etc.) and the process distributing input from a network.

[]{#index-lowestPriority}

**lowestPriority**

Answer the lowest valid priority

[]{#index-priorityName_003a}

**priorityName: priority**

Private - Answer a name for the given process priority

[]{#index-systemBackgroundPriority}

**systemBackgroundPriority**

Answer the priority for system background-priority processes. An
incremental garbage collector could run at this level but now it runs at
idlePriority instead.

[]{#index-timingPriority}

**timingPriority**

Answer the priority for system real-time processes.

[]{#index-unpreemptedPriority} []{#index-valueWithoutPreemption-1}

**unpreemptedPriority**

Answer the highest priority avilable in the system; never create a
process with this priority, instead use
BlockClosure\>\>#valueWithoutPreemption.

[]{#index-userBackgroundPriority}

**userBackgroundPriority**

Answer the priority for user background-priority processes

[]{#index-userInterruptPriority}

**userInterruptPriority**

Answer the priority for user interrupt-priority processes. Processes run
at this level will preempt the window scheduler and should, therefore,
not consume the processor forever.

[]{#index-userSchedulingPriority}

**userSchedulingPriority**

Answer the priority for user standard-priority processes
