[]{#Dictionary_002dnamespace-protocol}

::: header
Next:
[Dictionary-printing](Dictionary_002dprinting.html#Dictionary_002dprinting){accesskey="n"
rel="next"}, Previous: [Dictionary-dictionary
testing](Dictionary_002ddictionary-testing.html#Dictionary_002ddictionary-testing){accesskey="p"
rel="prev"}, Up: [Dictionary](Dictionary.html#Dictionary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Dictionary_003a-namespace-protocol}

#### 1.64.8 Dictionary: namespace protocol {#dictionary-namespace-protocol .subsection}

[]{#index-allSuperspaces}

**allSuperspaces**

Answer all the receiver's superspaces in a collection

[]{#index-allSuperspacesDo_003a-1}

**allSuperspacesDo: aBlock**

Evaluate aBlock once for each of the receiver's superspaces (which is
none for BindingDictionary).

[]{#index-definedKeys}

**definedKeys**

Answer a kind of Set containing the keys of the receiver

[]{#index-definesKey_003a}

**definesKey: key**

Answer whether the receiver defines the given key. 'Defines' means that
the receiver's superspaces, if any, are not considered.

[]{#index-hereAssociationAt_003a}

**hereAssociationAt: key**

Return the association for the variable named as specified by 'key' \*in
this namespace\*. If the key is not found search will \*not\* be carried
on in superspaces and the method will fail.

[]{#index-hereAssociationAt_003aifAbsent_003a}

**hereAssociationAt: key ifAbsent: aBlock**

Return the association for the variable named as specified by 'key' \*in
this namespace\*. If the key is not found search will \*not\* be carried
on in superspaces and aBlock will be immediately evaluated.

[]{#index-hereAt_003a}

**hereAt: key**

Return the value associated to the variable named as specified by 'key'
\*in this namespace\*. If the key is not found search will \*not\* be
carried on in superspaces and the method will fail.

[]{#index-hereAt_003aifAbsent_003a}

**hereAt: key ifAbsent: aBlock**

Return the value associated to the variable named as specified by 'key'
\*in this namespace\*. If the key is not found search will \*not\* be
carried on in superspaces and aBlock will be immediately evaluated.

[]{#index-inheritsFrom_003a-1}

**inheritsFrom: aNamespace**

Answer whether aNamespace is one of the receiver's direct and indirect
superspaces

[]{#index-superspace-1}

**superspace**

Answer the receiver's superspace, which is nil for BindingDictionary.

[]{#index-withAllSuperspaces}

**withAllSuperspaces**

Answer the receiver and all of its superspaces in a collection, which is
none for BindingDictionary

[]{#index-withAllSuperspacesDo_003a}

**withAllSuperspacesDo: aBlock**

Invokes aBlock for the receiver and all superspaces, both direct and
indirect (though a BindingDictionary does not have any).

------------------------------------------------------------------------

::: header
Next:
[Dictionary-printing](Dictionary_002dprinting.html#Dictionary_002dprinting){accesskey="n"
rel="next"}, Previous: [Dictionary-dictionary
testing](Dictionary_002ddictionary-testing.html#Dictionary_002ddictionary-testing){accesskey="p"
rel="prev"}, Up: [Dictionary](Dictionary.html#Dictionary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
