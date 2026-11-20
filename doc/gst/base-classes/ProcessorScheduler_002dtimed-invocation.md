[]{#ProcessorScheduler_002dtimed-invocation}

::: header
Previous:
[ProcessorScheduler-storing](ProcessorScheduler_002dstoring.html#ProcessorScheduler_002dstoring){accesskey="p"
rel="prev"}, Up:
[ProcessorScheduler](ProcessorScheduler.html#ProcessorScheduler){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ProcessorScheduler_003a-timed-invocation}

#### 1.136.8 ProcessorScheduler: timed invocation {#processorscheduler-timed-invocation .subsection}

[]{#index-isTimeoutProgrammed} []{#index-signal_003aatMilliseconds_003a}

**isTimeoutProgrammed**

Private - Answer whether there is a pending call to
#signal:atMilliseconds:

[]{#index-signal_003aatNanosecondClockValue_003a}

**signal: aSemaphore atNanosecondClockValue: ns**

Private - signal 'aSemaphore' when the nanosecond clock reaches 'ns'
nanoseconds.

[]{#index-signal_003aonInterrupt_003a}

**signal: aSemaphore onInterrupt: anIntegerSignalNumber**

Signal 'aSemaphore' when the given C signal occurs.
