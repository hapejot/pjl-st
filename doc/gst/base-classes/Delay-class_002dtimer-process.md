[]{#Delay-class_002dtimer-process}

::: header
Next:
[Delay-accessing](Delay_002daccessing.html#Delay_002daccessing){accesskey="n"
rel="next"}, Previous: [Delay class-still
unclassified](Delay-class_002dstill-unclassified.html#Delay-class_002dstill-unclassified){accesskey="p"
rel="prev"}, Up: [Delay](Delay.html#Delay){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Delay-class_003a-timer-process}

#### 1.62.3 Delay class: timer process {#delay-class-timer-process .subsection}

[]{#index-activeDelay}

**activeDelay**

Return the delay at the head of the queue.

[]{#index-handleDelayRequestor}

**handleDelayRequestor**

Handle a timer event; which can be either: - a schedule or unschedule
request (DelayRequestor notNil) - a timer signal (not explicitly
specified) We check for timer expiry every time we get a signal.

[]{#index-runDelayProcess}

**runDelayProcess**

Run the timer event loop.

[]{#index-scheduleDelay_003a}

**scheduleDelay: aDelay**

Private - Schedule this Delay. Run in the timer process, which is the
only one that manipulates Queue.

[]{#index-startDelayLoop}

**startDelayLoop**

Start the timer event loop.

[]{#index-unscheduleDelay_003a}

**unscheduleDelay: aDelay**

Private - Unschedule this Delay. Run in the timer process, which is the
only one that manipulates Queue.
