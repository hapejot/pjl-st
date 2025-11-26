[]{#SequenceableCollection_002dreplacing-items}

::: header
Next:
[SequenceableCollection-sorting](SequenceableCollection_002dsorting.html#SequenceableCollection_002dsorting){accesskey="n"
rel="next"}, Previous:
[SequenceableCollection-manipulation](SequenceableCollection_002dmanipulation.html#SequenceableCollection_002dmanipulation){accesskey="p"
rel="prev"}, Up:
[SequenceableCollection](SequenceableCollection.html#SequenceableCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SequenceableCollection_003a-replacing-items}

#### 1.151.8 SequenceableCollection: replacing items {#sequenceablecollection-replacing-items .subsection}

[]{#index-replaceAll_003awith_003a}

**replaceAll: anObject with: anotherObject**

In the receiver, replace every occurrence of anObject with
anotherObject.

[]{#index-replaceFrom_003ato_003awith_003a}

**replaceFrom: start to: stop with: replacementCollection**

Replace the items from start to stop with replacementCollection's items
from 1 to stop-start+1 (in unexpected order if the collection is not
sequenceable).

[]{#index-replaceFrom_003ato_003awith_003astartingAt_003a-2}

**replaceFrom: start to: stop with: replacementCollection startingAt:
repStart**

Replace the items from start to stop with replacementCollection's items
from repStart to repStart+stop-start

[]{#index-replaceFrom_003ato_003awithObject_003a}

**replaceFrom: anIndex to: stopIndex withObject: replacementObject**

Replace every item from start to stop with replacementObject.
