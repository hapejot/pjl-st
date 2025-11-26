[]{#MappedCollection}

::: header
Next: [Memory](Memory.html#Memory){accesskey="n" rel="next"}, Previous:
[Magnitude](Magnitude.html#Magnitude){accesskey="p" rel="prev"}, Up:
[Base classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#MappedCollection-1}

### 1.108 MappedCollection {#mappedcollection .section}

[]{#index-MappedCollection}

**Defined in namespace Smalltalk**\
**Superclass: Collection**\
**Category: Collections-Keyed**

:   I represent collections of objects that are indirectly indexed by
    names. There are really two collections involved: domain and a map.
    The map maps between external names and indices into domain, which
    contains the real association. In order to work properly, the domain
    must be an instance of a subclass of SequenceableCollection, and the
    map must be an instance of Dictionary, or of a subclass of
    SequenceableCollection.

    As an example of using me, consider implenting a Dictionary whose
    elements are indexed. The domain would be a SequenceableCollection
    with n elements, the map a Dictionary associating each key to an
    index in the domain. To access by key, to perform enumeration, etc.
    you would ask an instance of me; to access by index, you would
    access the domain directly.

    Another idea could be to implement row access or column access to a
    matrix implemented as a single n\*m Array: the Array would be the
    domain, while the map would be an Interval.

  -------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [MappedCollection class-instance creation](MappedCollection-class_002dinstance-creation.html#MappedCollection-class_002dinstance-creation){accesskey="1"}:        (class)
  • [MappedCollection-basic](MappedCollection_002dbasic.html#MappedCollection_002dbasic){accesskey="2"}:                                                              (instance)
  -------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
