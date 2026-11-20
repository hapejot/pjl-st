[]{#ObjectMemory-class_002dbuiltins}

::: header
Next: [ObjectMemory
class-initialization](ObjectMemory-class_002dinitialization.html#ObjectMemory-class_002dinitialization){accesskey="n"
rel="next"}, Previous: [ObjectMemory
class-accessing](ObjectMemory-class_002daccessing.html#ObjectMemory-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[ObjectMemory](ObjectMemory.html#ObjectMemory){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ObjectMemory-class_003a-builtins}

#### 1.125.2 ObjectMemory class: builtins {#objectmemory-class-builtins .subsection}

[]{#index-abort}

**abort**

Quit the Smalltalk environment, dumping core.

[]{#index-addressOf_003a-1}

**addressOf: anObject**

Returns the address of the actual object that anObject references. Note
that, with the exception of fixed objects this address is only valid
until the next garbage collection; thus it's pretty risky to count on
the address returned by this method for very long.

[]{#index-addressOfOOP_003a}

**addressOfOOP: anObject**

Returns the address of the OOP (object table slot) for anObject. The
address is an Integer and will not change over time (i.e. is immune from
garbage collector action) except if the virtual machine is stopped and
restarted.

[]{#index-bigObjectThreshold}

**bigObjectThreshold**

Answer the smallest size for objects that are allocated outside the main
heap in the hope of providing more locality of reference between small
objects.

[]{#index-bigObjectThreshold_003a}

**bigObjectThreshold: bytes**

Set the smallest size for objects that are allocated outside the main
heap in the hope of providing more locality of reference between small
objects. bytes must be a positive SmallInteger.

[]{#index-compact}

**compact**

Force a full garbage collection, including compaction of oldspace

[]{#index-finishIncrementalGC}

**finishIncrementalGC**

Do a step in the incremental garbage collection.

[]{#index-gcMessage}

**gcMessage**

Answer whether messages indicating that garbage collection is taking
place are printed on stdout

[]{#index-gcMessage_003a}

**gcMessage: aBoolean**

Set whether messages indicating that garbage collection is taking place
are printed on stdout

[]{#index-globalGarbageCollect}

**globalGarbageCollect**

Force a full garbage collection

[]{#index-growThresholdPercent}

**growThresholdPercent**

Answer the percentage of the amount of memory used by the system grows
which has to be full for the system to allocate more memory

[]{#index-growThresholdPercent_003a}

**growThresholdPercent: growPercent**

Set the percentage of the amount of memory used by the system grows
which has to be full for the system to allocate more memory

[]{#index-growTo_003a}

**growTo: numBytes**

Grow the amount of memory used by the system grows to numBytes.

[]{#index-incrementalGCStep}

**incrementalGCStep**

Do a step in the incremental garbage collection.

[]{#index-quit}

**quit**

Quit the Smalltalk environment. Whether files are closed and other
similar cleanup occurs depends on the platform

[]{#index-quit_003a}

**quit: exitStatus**

Quit the Smalltalk environment, passing the exitStatus integer to the
OS. Files are closed and other similar cleanups occur.

[]{#index-scavenge}

**scavenge**

Force a minor garbage collection

[]{#index-smoothingFactor}

**smoothingFactor**

Answer the factor (between 0 and 1) used to smooth the statistics
provided by the virtual machine about memory handling. 0 disables
updating the averages, 1 disables the smoothing (the statistics return
the last value).

[]{#index-smoothingFactor_003a}

**smoothingFactor: rate**

Set the factor (between 0 and 1) used to smooth the statistics provided
by the virtual machine about memory handling. 0 disables updating the
averages, 1 disables the smoothing (the statistics return the last
value).

[]{#index-spaceGrowRate}

**spaceGrowRate**

Answer the rate with which the amount of memory used by the system grows

[]{#index-spaceGrowRate_003a}

**spaceGrowRate: rate**

Set the rate with which the amount of memory used by the system grows

------------------------------------------------------------------------

::: header
Next: [ObjectMemory
class-initialization](ObjectMemory-class_002dinitialization.html#ObjectMemory-class_002dinitialization){accesskey="n"
rel="next"}, Previous: [ObjectMemory
class-accessing](ObjectMemory-class_002daccessing.html#ObjectMemory-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[ObjectMemory](ObjectMemory.html#ObjectMemory){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
