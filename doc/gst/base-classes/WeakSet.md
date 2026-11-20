[]{#WeakSet}

::: header
Next:
[WeakValueIdentityDictionary](WeakValueIdentityDictionary.html#WeakValueIdentityDictionary){accesskey="n"
rel="next"}, Previous:
[WeakKeyIdentityDictionary](WeakKeyIdentityDictionary.html#WeakKeyIdentityDictionary){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WeakSet-1}

### 1.219 WeakSet {#weakset .section}

[]{#index-WeakSet}

**Defined in namespace Smalltalk**\
**Superclass: Set**\
**Category: Collections-Weak**

:   I am similar to a plain set, but my items are stored in a weak
    array; I track which of them are garbage collected and, as soon as I
    encounter one of them, I swiftly remove all.

  ----------------------------------------------------------------------------------------- ---- ------------
  • [WeakSet-accessing](WeakSet_002daccessing.html#WeakSet_002daccessing){accesskey="1"}:        (instance)
  • [WeakSet-copying](WeakSet_002dcopying.html#WeakSet_002dcopying){accesskey="2"}:              (instance)
  • [WeakSet-loading](WeakSet_002dloading.html#WeakSet_002dloading){accesskey="3"}:              (instance)
  ----------------------------------------------------------------------------------------- ---- ------------
