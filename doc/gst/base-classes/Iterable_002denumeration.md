[]{#Iterable_002denumeration}

::: header
Next:
[Iterable-iteration](Iterable_002diteration.html#Iterable_002diteration){accesskey="n"
rel="next"}, Previous: [Iterable class-multibyte
encodings](Iterable-class_002dmultibyte-encodings.html#Iterable-class_002dmultibyte-encodings){accesskey="p"
rel="prev"}, Up: [Iterable](Iterable.html#Iterable){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Iterable_003a-enumeration}

#### 1.94.2 Iterable: enumeration {#iterable-enumeration .subsection}

[]{#index-_002c-5}

**, anIterable**

Answer an iterable that enumerates first the elements of the receiver
and then the elements of anIterable.

[]{#index-allSatisfy_003a}

**allSatisfy: aBlock**

Search the receiver for an element for which aBlock returns false.
Answer true if none does, false otherwise.

[]{#index-anySatisfy_003a}

**anySatisfy: aBlock**

Search the receiver for an element for which aBlock returns true. Answer
true if some does, false otherwise.

[]{#index-collect_003a-4}

**collect: aBlock**

Answer a new instance of a Collection containing all the results of
evaluating aBlock passing each of the receiver's elements

[]{#index-conform_003a}

**conform: aBlock**

Search the receiver for an element for which aBlock returns false.
Answer true if none does, false otherwise.

[]{#index-contains_003a}

**contains: aBlock**

Search the receiver for an element for which aBlock returns true. Answer
true if some does, false otherwise.

[]{#index-count_003a}

**count: aBlock**

Count the elements of the receiver for which aBlock returns true, and
return their number.

[]{#index-detect_003a}

**detect: aBlock**

Search the receiver for an element for which aBlock returns true. If
some does, answer it. If none does, fail

[]{#index-detect_003aifNone_003a}

**detect: aBlock ifNone: exceptionBlock**

Search the receiver for an element for which aBlock returns true. If
some does, answer it. If none does, answer the result of evaluating
aBlock

[]{#index-do_003a-5}

**do: aBlock**

Enumerate each object of the receiver, passing them to aBlock

[]{#index-do_003aseparatedBy_003a}

**do: aBlock separatedBy: separatorBlock**

Enumerate each object of the receiver, passing them to aBlock. Between
every two invocations of aBlock, invoke separatorBlock

[]{#index-fold_003a}

**fold: binaryBlock**

First, pass to binaryBlock the first and second elements of the
receiver; for each subsequent element, pass the result of the previous
evaluation and an element. Answer the result of the last invocation, or
the first element if the collection has size 1. Fail if the collection
is empty.

[]{#index-inject_003ainto_003a-1}

**inject: thisValue into: binaryBlock**

First, pass to binaryBlock thisValue and the first element of the
receiver; for each subsequent element, pass the result of the previous
evaluation and an element. Answer the result of the last invocation.

[]{#index-noneSatisfy_003a}

**noneSatisfy: aBlock**

Search the receiver for an element for which aBlock returns true. Answer
true if none does, false otherwise.

[]{#index-reject_003a-4}

**reject: aBlock**

Answer a new instance of a Collection containing all the elements in the
receiver which, when passed to aBlock, don't answer true

[]{#index-select_003a-4}

**select: aBlock**

Answer a new instance of a Collection containing all the elements in the
receiver which, when passed to aBlock, answer true

------------------------------------------------------------------------

::: header
Next:
[Iterable-iteration](Iterable_002diteration.html#Iterable_002diteration){accesskey="n"
rel="next"}, Previous: [Iterable class-multibyte
encodings](Iterable-class_002dmultibyte-encodings.html#Iterable-class_002dmultibyte-encodings){accesskey="p"
rel="prev"}, Up: [Iterable](Iterable.html#Iterable){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
