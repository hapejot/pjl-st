[]{#SequenceableCollection_002dconcatenating}

::: header
Next: [SequenceableCollection-copying
SequenceableCollections](SequenceableCollection_002dcopying-SequenceableCollections.html#SequenceableCollection_002dcopying-SequenceableCollections){accesskey="n"
rel="next"}, Previous:
[SequenceableCollection-comparing](SequenceableCollection_002dcomparing.html#SequenceableCollection_002dcomparing){accesskey="p"
rel="prev"}, Up:
[SequenceableCollection](SequenceableCollection.html#SequenceableCollection){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SequenceableCollection_003a-concatenating}

#### 1.151.4 SequenceableCollection: concatenating {#sequenceablecollection-concatenating .subsection}

[]{#index-join_003a-2}

**join: sepCollection**

Answer a new collection like my first element, with all the elements (in
order) of all my elements (which should be collections) separated by
sepCollection.

I use my first element instead of myself as a prototype because my
elements are more likely to share the desired properties than I am, such
as in:

#('hello,' 'world') join: ' ' =\> 'hello, world'

[]{#index-with_003a-3}

**with: aSequenceableCollection**

Return an Array with the same size as the receiver and
aSequenceableCollection, each element of which is a 2-element Arrays
including one element from the receiver and one from
aSequenceableCollection.

[]{#index-with_003awith_003a-2}

**with: seqColl1 with: seqColl2**

Return an Array with the same size as the receiver and the arguments,
each element of which is a 3-element Arrays including one element from
the receiver and one from each argument.

[]{#index-with_003awith_003awith_003a-2}

**with: seqColl1 with: seqColl2 with: seqColl3**

Return an Array with the same size as the receiver and the arguments,
each element of which is a 4-element Arrays including one element from
the receiver and one from each argument.
