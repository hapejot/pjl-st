[]{#WeakValueIdentityDictionary}

::: header
Next:
[WeakValueLookupTable](WeakValueLookupTable.html#WeakValueLookupTable){accesskey="n"
rel="next"}, Previous: [WeakSet](WeakSet.html#WeakSet){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WeakValueIdentityDictionary-1}

### 1.220 WeakValueIdentityDictionary {#weakvalueidentitydictionary .section}

[]{#index-WeakValueIdentityDictionary}

**Defined in namespace Smalltalk**\
**Superclass: WeakValueLookupTable**\
**Category: Collections-Weak**

:   I am similar to a plain identity dictionary, but my values are
    stored in a weak array; I track which of the values are garbage
    collected and, as soon as one of them is accessed, I swiftly remove
    the associations for the garbage collected values
