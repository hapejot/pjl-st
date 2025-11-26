[]{#WeakValueLookupTable}

::: header
Next: [WordArray](WordArray.html#WordArray){accesskey="n" rel="next"},
Previous:
[WeakValueIdentityDictionary](WeakValueIdentityDictionary.html#WeakValueIdentityDictionary){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WeakValueLookupTable-1}

### 1.221 WeakValueLookupTable {#weakvaluelookuptable .section}

[]{#index-WeakValueLookupTable}

**Defined in namespace Smalltalk**\
**Superclass: LookupTable**\
**Category: Collections-Weak**

:   I am similar to a plain LookupTable, but my values are stored in a
    weak array; I track which of the values are garbage collected and,
    as soon as one of them is accessed, I swiftly remove the
    associations for the garbage collected values

  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [WeakValueLookupTable class-hacks](WeakValueLookupTable-class_002dhacks.html#WeakValueLookupTable-class_002dhacks){accesskey="1"}:        (class)
  • [WeakValueLookupTable-hacks](WeakValueLookupTable_002dhacks.html#WeakValueLookupTable_002dhacks){accesskey="2"}:                          (instance)
  • [WeakValueLookupTable-rehashing](WeakValueLookupTable_002drehashing.html#WeakValueLookupTable_002drehashing){accesskey="3"}:              (instance)
  -------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
