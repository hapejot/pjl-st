[]{#WeakIdentitySet}

::: header
Next:
[WeakKeyDictionary](WeakKeyDictionary.html#WeakKeyDictionary){accesskey="n"
rel="next"}, Previous:
[WeakArray](WeakArray.html#WeakArray){accesskey="p" rel="prev"}, Up:
[Base classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WeakIdentitySet-1}

### 1.216 WeakIdentitySet {#weakidentityset .section}

[]{#index-WeakIdentitySet}

**Defined in namespace Smalltalk**\
**Superclass: WeakSet**\
**Category: Collections-Weak**

:   I am similar to a plain identity set, but my keys are stored in a
    weak array; I track which of them are garbage collected and, as soon
    as I encounter one of them, I swiftly remove all the garbage
    collected keys

  ----------------------------------------------------------------------------------------------------------------- ---- ------------
  • [WeakIdentitySet-accessing](WeakIdentitySet_002daccessing.html#WeakIdentitySet_002daccessing){accesskey="1"}:        (instance)
  ----------------------------------------------------------------------------------------------------------------- ---- ------------
