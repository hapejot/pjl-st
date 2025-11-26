[]{#Behavior_002dtesting-the-method-dictionary}

::: header
Previous: [Behavior-testing the form of the
instances](Behavior_002dtesting-the-form-of-the-instances.html#Behavior_002dtesting-the-form-of-the-instances){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-testing-the-method-dictionary}

#### 1.9.25 Behavior: testing the method dictionary {#behavior-testing-the-method-dictionary .subsection}

[]{#index-canUnderstand_003a}

**canUnderstand: selector**

Returns true if the instances of the receiver understand the given
selector

[]{#index-hasMethods}

**hasMethods**

Return whether the receiver has any methods defined

[]{#index-includesSelector_003a}

**includesSelector: selector**

Returns true if the local method dictionary contains the given selector

[]{#index-scopeHas_003aifTrue_003a}

**scopeHas: name ifTrue: aBlock**

If methods understood by the receiver's instances have access to a
symbol named 'name', evaluate aBlock

[]{#index-whichClassIncludesSelector_003a}

**whichClassIncludesSelector: selector**

Answer which class in the receiver's hierarchy contains the
implementation of selector used by instances of the class (nil if none
does)

[]{#index-whichSelectorsAccess_003a}

**whichSelectorsAccess: instVarName**

Answer a Set of selectors which access the given instance variable

[]{#index-whichSelectorsAssign_003a}

**whichSelectorsAssign: instVarName**

Answer a Set of selectors which read the given instance variable

[]{#index-whichSelectorsRead_003a}

**whichSelectorsRead: instVarName**

Answer a Set of selectors which read the given instance variable

[]{#index-whichSelectorsReferTo_003a}

**whichSelectorsReferTo: anObject**

Returns a Set of selectors that refer to anObject

[]{#index-whichSelectorsReferToByteCode_003a}

**whichSelectorsReferToByteCode: aByteCode**

Return the collection of selectors in the class which reference the byte
code, aByteCode
