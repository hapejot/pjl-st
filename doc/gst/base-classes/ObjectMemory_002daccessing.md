[]{#ObjectMemory_002daccessing}

::: header
Next:
[ObjectMemory-builtins](ObjectMemory_002dbuiltins.html#ObjectMemory_002dbuiltins){accesskey="n"
rel="next"}, Previous: [ObjectMemory class-saving the
image](ObjectMemory-class_002dsaving-the-image.html#ObjectMemory-class_002dsaving-the-image){accesskey="p"
rel="prev"}, Up:
[ObjectMemory](ObjectMemory.html#ObjectMemory){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ObjectMemory_003a-accessing}

#### 1.125.5 ObjectMemory: accessing {#objectmemory-accessing .subsection}

[]{#index-allocFailures}

**allocFailures**

Answer the number of times that the old-space allocator found no block
that was at least as big as requested, and had to ask the operating
system for more memory.

[]{#index-allocMatches}

**allocMatches**

Answer the number of times that the old-space allocator found a block
that was exactly as big as requested.

[]{#index-allocProbes}

**allocProbes**

Answer the number of free blocks that the old-space allocator had to
examine so far to allocate all the objects that are in old-space

[]{#index-allocSplits}

**allocSplits**

Answer the number of times that the old-space allocator could not find a
block that was exactly as big as requested, and had to split a larger
free block in two parts.

[]{#index-bytesPerOOP}

**bytesPerOOP**

Answer the number of bytes that is taken by an ordinary object pointer
(in practice, a field such as a named instance variable).

[]{#index-bytesPerOTE}

**bytesPerOTE**

Answer the number of bytes that is taken by an object table entry (in
practice, the overhead incurred by every object in the system, with the
sole exception of SmallIntegers).

[]{#index-edenSize}

**edenSize**

Answer the number of bytes in the 'eden' area of the young generation
(in practice, the number of allocated bytes between two scavenges).

[]{#index-edenUsedBytes}

**edenUsedBytes**

Answer the number of bytes that are currently filled in the 'eden' area
of the young generation.

[]{#index-fixedSpaceSize}

**fixedSpaceSize**

Answer the number of bytes in the special heap devoted to objects that
the garbage collector cannot move around in memory.

[]{#index-fixedSpaceUsedBytes}

**fixedSpaceUsedBytes**

Answer the number of bytes that are currently filled in the special heap
devoted to objects that the garbage collector cannot move around in
memory.

[]{#index-numCompactions}

**numCompactions**

Answer the number of oldspace compactions that happened since the VM was
started.

[]{#index-numFixedOOPs}

**numFixedOOPs**

Answer the number of objects that the garbage collector cannot move
around in memory.

[]{#index-numFreeOTEs}

**numFreeOTEs**

Answer the number of entries that are currently free in the object
table.

[]{#index-numGlobalGCs}

**numGlobalGCs**

Answer the number of global garbage collections (collection of the
entire heap) that happened since the VM was started.

[]{#index-numGrowths}

**numGrowths**

Answer the number of times that oldspace was grown since the VM was
started.

[]{#index-numOTEs}

**numOTEs**

Answer the number of entries that are currently allocated for the object
table.

[]{#index-numOldOOPs}

**numOldOOPs**

Answer the number of objects that reside in the old generation.

[]{#index-numScavenges}

**numScavenges**

Answer the number of scavenges (fast collections of the young
generation) that happened since the VM was started.

[]{#index-numWeakOOPs}

**numWeakOOPs**

Answer the number of weak objects that the garbage collector is
currently tracking.

[]{#index-oldSpaceSize}

**oldSpaceSize**

Answer the number of bytes in the old generation.

[]{#index-oldSpaceUsedBytes}

**oldSpaceUsedBytes**

Answer the number of bytes that are currently filled in the old
generation.

[]{#index-reclaimedBytesPerGlobalGC}

**reclaimedBytesPerGlobalGC**

Answer the average number of bytes that are found to be garbage during a
global garbage collections.

[]{#index-reclaimedBytesPerScavenge}

**reclaimedBytesPerScavenge**

Answer the average number of bytes that are found to be garbage during a
scavenge.

[]{#index-reclaimedPercentPerScavenge}

**reclaimedPercentPerScavenge**

Answer the average percentage of allocated bytes that are found to be
garbage during a scavenge. If this number falls below 60-70 you should
definitely increment the size of the eden, because you risk that
scavenging is eating a considerable fraction of your execution time; do
the measurement on a restarted image, so that the extra tenuring
incurred when creating long-lived objects such as classes or methods is
not considered.

[]{#index-survSpaceSize}

**survSpaceSize**

Answer the number of bytes in the 'survivor' area of the young
generation (the area to which young objects are relocated during
scavenges).

[]{#index-survSpaceUsedBytes}

**survSpaceUsedBytes**

Answer the number of bytes that are currently filled in the 'survivor'
area of the young generation.

[]{#index-tenuredBytesPerScavenge}

**tenuredBytesPerScavenge**

Answer the average number of bytes that are promoted to oldspace during
a scavenge.

[]{#index-timeBetweenGlobalGCs}

**timeBetweenGlobalGCs**

Answer the average number of milliseconds between two global garbage
collections.

[]{#index-timeBetweenGrowths}

**timeBetweenGrowths**

Answer the average number of milliseconds between decisions to grow the
heap.

[]{#index-timeBetweenScavenges}

**timeBetweenScavenges**

Answer the average number of milliseconds between two scavenges (fast
collections of the young generation).

[]{#index-timeToCollect}

**timeToCollect**

Answer the average number of milliseconds that a global garbage
collection takes.

[]{#index-timeToCompact}

**timeToCompact**

Answer the average number of milliseconds that compacting the heap
takes. This the same time that is taken by growing the heap.

[]{#index-timeToScavenge}

**timeToScavenge**

Answer the average number of milliseconds that a scavenge takes (fast
collections of the young generation).

------------------------------------------------------------------------

::: header
Next:
[ObjectMemory-builtins](ObjectMemory_002dbuiltins.html#ObjectMemory_002dbuiltins){accesskey="n"
rel="next"}, Previous: [ObjectMemory class-saving the
image](ObjectMemory-class_002dsaving-the-image.html#ObjectMemory-class_002dsaving-the-image){accesskey="p"
rel="prev"}, Up:
[ObjectMemory](ObjectMemory.html#ObjectMemory){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
