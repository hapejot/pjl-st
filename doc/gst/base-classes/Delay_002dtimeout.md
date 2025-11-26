[]{#Delay_002dtimeout}

::: header
Previous:
[Delay-testing](Delay_002dtesting.html#Delay_002dtesting){accesskey="p"
rel="prev"}, Up: [Delay](Delay.html#Delay){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Delay_003a-timeout}

#### 1.62.11 Delay: timeout {#delay-timeout .subsection}

[]{#index-value_003aonTimeoutDo_003a}

**value: aBlock onTimeoutDo: aTimeoutBlock**

Execute aBlock for up to the time of my own delay; in case the code did
not finish abort the execution, unwind the block and then evaluate
aTimeoutBlock.
