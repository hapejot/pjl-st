[]{#SequenceableCollection_002dbasic}

::: header
Next:
[SequenceableCollection-comparing](SequenceableCollection_002dcomparing.html#SequenceableCollection_002dcomparing){accesskey="n"
rel="next"}, Previous: [SequenceableCollection class-instance
creation](SequenceableCollection-class_002dinstance-creation.html#SequenceableCollection-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[SequenceableCollection](SequenceableCollection.html#SequenceableCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SequenceableCollection_003a-basic}

#### 1.151.2 SequenceableCollection: basic {#sequenceablecollection-basic .subsection}

[]{#index-after_003a}

**after: oldObject**

Return the element after oldObject. Error if oldObject not found or if
no following object is available

[]{#index-allButFirst}

**allButFirst**

Answer a copy of the receiver without the first object.

[]{#index-allButFirst_003a}

**allButFirst: n**

Answer a copy of the receiver without the first n objects.

[]{#index-allButLast}

**allButLast**

Answer a copy of the receiver without the last object.

[]{#index-allButLast_003a}

**allButLast: n**

Answer a copy of the receiver without the last n objects.

[]{#index-at_003aifAbsent_003a-6}

**at: anIndex ifAbsent: aBlock**

Answer the anIndex-th item of the collection, or evaluate aBlock and
answer the result if the index is out of range

[]{#index-atAll_003a-3} []{#index-collect_003a-10}

**atAll: keyCollection**

Answer a collection of the same kind returned by #collect:, that only
includes the values at the given indices. Fail if any of the values in
keyCollection is out of bounds for the receiver.

[]{#index-atAll_003aput_003a}

**atAll: aCollection put: anObject**

Put anObject at every index contained in aCollection

[]{#index-atAllPut_003a}

**atAllPut: anObject**

Put anObject at every index in the receiver

[]{#index-atRandom}

**atRandom**

Return a random item of the receiver.

[]{#index-before_003a}

**before: oldObject**

Return the element before oldObject. Error if oldObject not found or if
no preceding object is available

[]{#index-first-4}

**first**

Answer the first item in the receiver

[]{#index-first_003a}

**first: n**

Answer the first n items in the receiver

[]{#index-fourth}

**fourth**

Answer the fourth item in the receiver

[]{#index-identityIncludes_003a-3}

**identityIncludes: anObject**

Answer whether we include the anObject object

[]{#index-identityIndexOf_003a}

**identityIndexOf: anElement**

Answer the index of the first occurrence of an object identical to
anElement in the receiver. Answer 0 if no item is found

[]{#index-identityIndexOf_003aifAbsent_003a}

**identityIndexOf: anElement ifAbsent: exceptionBlock**

Answer the index of the first occurrence of an object identical to
anElement in the receiver. Invoke exceptionBlock and answer its result
if no item is found

[]{#index-identityIndexOf_003astartingAt_003a}

**identityIndexOf: anElement startingAt: anIndex**

Answer the first index \> anIndex which contains an object identical to
anElement. Answer 0 if no item is found

[]{#index-identityIndexOf_003astartingAt_003aifAbsent_003a}

**identityIndexOf: anObject startingAt: anIndex ifAbsent:
exceptionBlock**

Answer the first index \> anIndex which contains an object exactly
identical to anObject. Invoke exceptionBlock and answer its result if no
item is found

[]{#index-identityIndexOfLast_003aifAbsent_003a}

**identityIndexOfLast: anElement ifAbsent: exceptionBlock**

Answer the last index which contains an object identical to anElement.
Invoke exceptionBlock and answer its result if no item is found

[]{#index-includes_003a-6}

**includes: anObject**

Answer whether we include anObject

[]{#index-indexOf_003a}

**indexOf: anElement**

Answer the index of the first occurrence of anElement in the receiver.
Answer 0 if no item is found

[]{#index-indexOf_003aifAbsent_003a}

**indexOf: anElement ifAbsent: exceptionBlock**

Answer the index of the first occurrence of anElement in the receiver.
Invoke exceptionBlock and answer its result if no item is found

[]{#index-indexOf_003astartingAt_003a-1}

**indexOf: anElement startingAt: anIndex**

Answer the first index \> anIndex which contains anElement. Answer 0 if
no item is found

[]{#index-indexOf_003astartingAt_003aifAbsent_003a-2}

**indexOf: anElement startingAt: anIndex ifAbsent: exceptionBlock**

Answer the first index \> anIndex which contains anElement. Invoke
exceptionBlock and answer its result if no item is found

[]{#index-indexOfLast_003aifAbsent_003a}

**indexOfLast: anElement ifAbsent: exceptionBlock**

Answer the last index which contains anElement. Invoke exceptionBlock
and answer its result if no item is found

[]{#index-indexOfSubCollection_003a}

**indexOfSubCollection: aSubCollection**

Answer the first index \> anIndex at which starts a sequence of items
matching aSubCollection. Answer 0 if no such sequence is found.

[]{#index-indexOfSubCollection_003aifAbsent_003a}

**indexOfSubCollection: aSubCollection ifAbsent: exceptionBlock**

Answer the first index \> anIndex at which starts a sequence of items
matching aSubCollection. Answer 0 if no such sequence is found.

[]{#index-indexOfSubCollection_003astartingAt_003a}

**indexOfSubCollection: aSubCollection startingAt: anIndex**

Answer the first index \> anIndex at which starts a sequence of items
matching aSubCollection. Answer 0 if no such sequence is found.

[]{#index-indexOfSubCollection_003astartingAt_003aifAbsent_003a}

**indexOfSubCollection: aSubCollection startingAt: anIndex ifAbsent:
exceptionBlock**

Answer the first index \> anIndex at which starts a sequence of items
matching aSubCollection. Invoke exceptionBlock and answer its result if
no such sequence is found

[]{#index-last-4}

**last**

Answer the last item in the receiver

[]{#index-last_003a}

**last: n**

Answer the last n items in the receiver

[]{#index-second-1}

**second**

Answer the second item in the receiver

[]{#index-third}

**third**

Answer the third item in the receiver

------------------------------------------------------------------------

::: header
Next:
[SequenceableCollection-comparing](SequenceableCollection_002dcomparing.html#SequenceableCollection_002dcomparing){accesskey="n"
rel="next"}, Previous: [SequenceableCollection class-instance
creation](SequenceableCollection-class_002dinstance-creation.html#SequenceableCollection-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[SequenceableCollection](SequenceableCollection.html#SequenceableCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
