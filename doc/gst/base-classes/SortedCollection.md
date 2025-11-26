[]{#SortedCollection}

::: header
Next: [Stream](Stream.html#Stream){accesskey="n" rel="next"}, Previous:
[SmallInteger](SmallInteger.html#SmallInteger){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#SortedCollection-1}

### 1.156 SortedCollection {#sortedcollection .section}

[]{#index-SortedCollection}

**Defined in namespace Smalltalk**\
**Superclass: OrderedCollection**\
**Category: Collections-Sequenceable**

:   I am a collection of objects, stored and accessed according to some
    sorting criteria. I store things using heap sort and quick sort. My
    instances have a comparison block associated with them; this block
    takes two arguments and is a predicate which returns true if the
    first argument should be sorted earlier than the second. The default
    block is \[ :a :b \| a \<= b \], but I will accept any block that
    conforms to the above criteria -- actually any object which responds
    to #value:value:.

  -------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [SortedCollection class-hacking](SortedCollection-class_002dhacking.html#SortedCollection-class_002dhacking){accesskey="1"}:                                      (class)
  • [SortedCollection class-instance creation](SortedCollection-class_002dinstance-creation.html#SortedCollection-class_002dinstance-creation){accesskey="2"}:        (class)
  • [SortedCollection-basic](SortedCollection_002dbasic.html#SortedCollection_002dbasic){accesskey="3"}:                                                              (instance)
  • [SortedCollection-copying](SortedCollection_002dcopying.html#SortedCollection_002dcopying){accesskey="4"}:                                                        (instance)
  • [SortedCollection-disabled](SortedCollection_002ddisabled.html#SortedCollection_002ddisabled){accesskey="5"}:                                                     (instance)
  • [SortedCollection-enumerating](SortedCollection_002denumerating.html#SortedCollection_002denumerating){accesskey="6"}:                                            (instance)
  • [SortedCollection-saving and loading](SortedCollection_002dsaving-and-loading.html#SortedCollection_002dsaving-and-loading){accesskey="7"}:                       (instance)
  • [SortedCollection-searching](SortedCollection_002dsearching.html#SortedCollection_002dsearching){accesskey="8"}:                                                  (instance)
  • [SortedCollection-sorting](SortedCollection_002dsorting.html#SortedCollection_002dsorting){accesskey="9"}:                                                        (instance)
  -------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
