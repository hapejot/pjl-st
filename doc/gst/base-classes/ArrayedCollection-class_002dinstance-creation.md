[]{#ArrayedCollection-class_002dinstance-creation}

::: header
Next:
[ArrayedCollection-basic](ArrayedCollection_002dbasic.html#ArrayedCollection_002dbasic){accesskey="n"
rel="next"}, Up:
[ArrayedCollection](ArrayedCollection.html#ArrayedCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ArrayedCollection-class_003a-instance-creation}

#### 1.5.1 ArrayedCollection class: instance creation {#arrayedcollection-class-instance-creation .subsection}

[]{#index-join_003a}

**join: aCollection**

Where aCollection is a collection of SequenceableCollections, answer a
new instance with all the elements therein, in order.

[]{#index-join_003aseparatedBy_003a}

**join: aCollection separatedBy: sepCollection**

Where aCollection is a collection of SequenceableCollections, answer a
new instance with all the elements therein, in order, each separated by
an occurrence of sepCollection.

[]{#index-new_003awithAll_003a}

**new: size withAll: anObject**

Answer a collection with the given size, whose elements are all set to
anObject

[]{#index-streamContents_003a}

**streamContents: aBlock**

Create a ReadWriteStream on an empty instance of the receiver; pass the
stream to aBlock, then retrieve its contents and answer them.

[]{#index-with_003a}

**with: element1**

Answer a collection whose only element is element1

[]{#index-with_003awith_003a}

**with: element1 with: element2**

Answer a collection whose only elements are the parameters in the order
they were passed

[]{#index-with_003awith_003awith_003a}

**with: element1 with: element2 with: element3**

Answer a collection whose only elements are the parameters in the order
they were passed

[]{#index-with_003awith_003awith_003awith_003a}

**with: element1 with: element2 with: element3 with: element4**

Answer a collection whose only elements are the parameters in the order
they were passed

[]{#index-with_003awith_003awith_003awith_003awith_003a}

**with: element1 with: element2 with: element3 with: element4 with:
element5**

Answer a collection whose only elements are the parameters in the order
they were passed

[]{#index-withAll_003a}

**withAll: aCollection**

Answer a collection whose elements are the same as those in aCollection
