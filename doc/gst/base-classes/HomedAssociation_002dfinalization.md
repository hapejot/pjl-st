[]{#HomedAssociation_002dfinalization}

::: header
Next:
[HomedAssociation-storing](HomedAssociation_002dstoring.html#HomedAssociation_002dstoring){accesskey="n"
rel="next"}, Previous:
[HomedAssociation-accessing](HomedAssociation_002daccessing.html#HomedAssociation_002daccessing){accesskey="p"
rel="prev"}, Up:
[HomedAssociation](HomedAssociation.html#HomedAssociation){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#HomedAssociation_003a-finalization}

#### 1.89.3 HomedAssociation: finalization {#homedassociation-finalization .subsection}

[]{#index-mourn-1}

**mourn**

This message is sent to the receiver when the object is made ephemeron
(which is common when HomedAssociations are used by a WeakKeyDictionary
or a WeakSet). The mourning of the object's key is first of all demanded
to the environment (which will likely remove the object from itself),
and then performed as usual by clearing the key and value fields.
