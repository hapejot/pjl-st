[]{#LookupTable}

::: header
Next: [Magnitude](Magnitude.html#Magnitude){accesskey="n" rel="next"},
Previous: [LookupKey](LookupKey.html#LookupKey){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#LookupTable-1}

### 1.106 LookupTable {#lookuptable .section}

[]{#index-LookupTable}

**Defined in namespace Smalltalk**\
**Superclass: Dictionary**\
**Category: Collections-Keyed**

:   I am a more efficient variant of Dictionary that cannot be used as a
    pool dictionary of variables, as I don't use Associations to store
    key-value pairs. I also cannot have nil as a key; if you need to be
    able to store nil as a key, use Dictionary instead. I use the object
    equality comparison message #= to determine equivalence of indices.

  ----------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [LookupTable class-instance creation](LookupTable-class_002dinstance-creation.html#LookupTable-class_002dinstance-creation){accesskey="1"}:        (class)
  • [LookupTable-accessing](LookupTable_002daccessing.html#LookupTable_002daccessing){accesskey="2"}:                                                  (instance)
  • [LookupTable-enumerating](LookupTable_002denumerating.html#LookupTable_002denumerating){accesskey="3"}:                                            (instance)
  • [LookupTable-hashing](LookupTable_002dhashing.html#LookupTable_002dhashing){accesskey="4"}:                                                        (instance)
  • [LookupTable-rehashing](LookupTable_002drehashing.html#LookupTable_002drehashing){accesskey="5"}:                                                  (instance)
  • [LookupTable-removing](LookupTable_002dremoving.html#LookupTable_002dremoving){accesskey="6"}:                                                     (instance)
  • [LookupTable-storing](LookupTable_002dstoring.html#LookupTable_002dstoring){accesskey="7"}:                                                        (instance)
  ----------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
