[]{#RunArray}

::: header
Next: [ScaledDecimal](ScaledDecimal.html#ScaledDecimal){accesskey="n"
rel="next"}, Previous:
[RootNamespace](RootNamespace.html#RootNamespace){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#RunArray-1}

### 1.147 RunArray {#runarray .section}

[]{#index-RunArray}

**Defined in namespace Smalltalk**\
**Superclass: OrderedCollection**\
**Category: Collections-Sequenceable**

:   My instances are OrderedCollections that automatically apply Run
    Length Encoding compression to the things they store. Be careful
    when using me: I can provide great space savings, but my instances
    don't grant linear access time. RunArray's behavior currently is
    similar to that of OrderedCollection (you can add elements to
    RunArrays); maybe it should behave like an ArrayedCollection.

  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [RunArray class-instance creation](RunArray-class_002dinstance-creation.html#RunArray-class_002dinstance-creation){accesskey="1"}:        (class)
  • [RunArray-accessing](RunArray_002daccessing.html#RunArray_002daccessing){accesskey="2"}:                                                  (instance)
  • [RunArray-adding](RunArray_002dadding.html#RunArray_002dadding){accesskey="3"}:                                                           (instance)
  • [RunArray-basic](RunArray_002dbasic.html#RunArray_002dbasic){accesskey="4"}:                                                              (instance)
  • [RunArray-copying](RunArray_002dcopying.html#RunArray_002dcopying){accesskey="5"}:                                                        (instance)
  • [RunArray-enumerating](RunArray_002denumerating.html#RunArray_002denumerating){accesskey="6"}:                                            (instance)
  • [RunArray-removing](RunArray_002dremoving.html#RunArray_002dremoving){accesskey="7"}:                                                     (instance)
  • [RunArray-searching](RunArray_002dsearching.html#RunArray_002dsearching){accesskey="8"}:                                                  (instance)
  • [RunArray-testing](RunArray_002dtesting.html#RunArray_002dtesting){accesskey="9"}:                                                        (instance)
  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
