[]{#Bag_002dadding}

::: header
Next: [Bag-enumerating the elements of a
collection](Bag_002denumerating-the-elements-of-a-collection.html#Bag_002denumerating-the-elements-of-a-collection){accesskey="n"
rel="next"}, Previous: [Bag
class-basic](Bag-class_002dbasic.html#Bag-class_002dbasic){accesskey="p"
rel="prev"}, Up: [Bag](Bag.html#Bag){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Bag_003a-adding}

#### 1.8.2 Bag: adding {#bag-adding .subsection}

[]{#index-add_003a-1}

**add: newObject**

Add an occurrence of newObject to the receiver. Answer newObject. Fail
if newObject is nil.

[]{#index-add_003awithOccurrences_003a}

**add: newObject withOccurrences: anInteger**

If anInteger \> 0, add anInteger occurrences of newObject to the
receiver. If anInteger \< 0, remove them. Answer newObject. Fail if
newObject is nil.
