[]{#Dictionary_002ddictionary-removing}

::: header
Next: [Dictionary-dictionary
testing](Dictionary_002ddictionary-testing.html#Dictionary_002ddictionary-testing){accesskey="n"
rel="next"}, Previous: [Dictionary-dictionary
enumerating](Dictionary_002ddictionary-enumerating.html#Dictionary_002ddictionary-enumerating){accesskey="p"
rel="prev"}, Up: [Dictionary](Dictionary.html#Dictionary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Dictionary_003a-dictionary-removing}

#### 1.64.6 Dictionary: dictionary removing {#dictionary-dictionary-removing .subsection}

[]{#index-remove_003a-1}

**remove: anAssociation**

Remove anAssociation's key from the dictionary

[]{#index-remove_003aifAbsent_003a-2}

**remove: anAssociation ifAbsent: aBlock**

Remove anAssociation's key from the dictionary

[]{#index-removeAllKeys_003a}

**removeAllKeys: keys**

Remove all the keys in keys, without raising any errors

[]{#index-removeAllKeys_003aifAbsent_003a}

**removeAllKeys: keys ifAbsent: aBlock**

Remove all the keys in keys, passing the missing keys as parameters to
aBlock as they're encountered

[]{#index-removeKey_003a}

**removeKey: key**

Remove the passed key from the dictionary, fail if it is not found

[]{#index-removeKey_003aifAbsent_003a}

**removeKey: key ifAbsent: aBlock**

Remove the passed key from the dictionary, answer the result of
evaluating aBlock if it is not found
