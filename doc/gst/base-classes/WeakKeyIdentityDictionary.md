[]{#WeakKeyIdentityDictionary}

::: header
Next: [WeakSet](WeakSet.html#WeakSet){accesskey="n" rel="next"},
Previous:
[WeakKeyDictionary](WeakKeyDictionary.html#WeakKeyDictionary){accesskey="p"
rel="prev"}, Up: [Base
classes](Base-classes.html#Base-classes){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#WeakKeyIdentityDictionary-1}

### 1.218 WeakKeyIdentityDictionary {#weakkeyidentitydictionary .section}

[]{#index-WeakKeyIdentityDictionary}

**Defined in namespace Smalltalk**\
**Superclass: WeakKeyDictionary**\
**Category: Collections-Weak**

:   I am similar to a plain identity dictionary, but my keys are stored
    in a weak array; I track which of them are garbage collected and, as
    soon as I encounter one of them, I swiftly remove all the
    associations for the garbage collected keys
