[]{#WeakKeyDictionary}

::: header
Next:
[WeakKeyIdentityDictionary](WeakKeyIdentityDictionary.html#WeakKeyIdentityDictionary){accesskey="n"
rel="next"}, Previous:
[WeakIdentitySet](WeakIdentitySet.html#WeakIdentitySet){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WeakKeyDictionary-1}

### 1.217 WeakKeyDictionary {#weakkeydictionary .section}

[]{#index-WeakKeyDictionary}

**Defined in namespace Smalltalk**\
**Superclass: Dictionary**\
**Category: Collections-Weak**

:   I am similar to a plain Dictionary, but my keys are stored in a weak
    array; I track which of them are garbage collected and, as soon as I
    encounter one of them, I swiftly remove all the associations for the
    garbage collected keys

  ----------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [WeakKeyDictionary class-hacks](WeakKeyDictionary-class_002dhacks.html#WeakKeyDictionary-class_002dhacks){accesskey="1"}:        (class)
  • [WeakKeyDictionary-accessing](WeakKeyDictionary_002daccessing.html#WeakKeyDictionary_002daccessing){accesskey="2"}:              (instance)
  ----------------------------------------------------------------------------------------------------------------------------- ---- ------------
