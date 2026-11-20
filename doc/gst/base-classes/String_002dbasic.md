[]{#String_002dbasic}

::: header
Next: [String-built
ins](String_002dbuilt-ins.html#String_002dbuilt-ins){accesskey="n"
rel="next"}, Previous:
[String-accessing](String_002daccessing.html#String_002daccessing){accesskey="p"
rel="prev"}, Up: [String](String.html#String){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#String_003a-basic}

#### 1.158.4 String: basic {#string-basic .subsection}

[]{#index-_002c-7}

**, aString**

Answer a new instance of an ArrayedCollection containing all the
elements in the receiver, followed by all the elements in
aSequenceableCollection

[]{#index-_003d-40}

**= aCollection**

Answer whether the receiver's items match those in aCollection

[]{#index-indexOf_003astartingAt_003a-2}

**indexOf: anElement startingAt: anIndex**

Answer the first index \> anIndex which contains anElement. Invoke
exceptionBlock and answer its result if no item is found

[]{#index-indexOf_003astartingAt_003aifAbsent_003a-4}

**indexOf: anElement startingAt: anIndex ifAbsent: exceptionBlock**

Answer the first index \> anIndex which contains anElement. Invoke
exceptionBlock and answer its result if no item is found
