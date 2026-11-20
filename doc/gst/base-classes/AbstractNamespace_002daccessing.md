[]{#AbstractNamespace_002daccessing}

::: header
Next:
[AbstractNamespace-compiling](AbstractNamespace_002dcompiling.html#AbstractNamespace_002dcompiling){accesskey="n"
rel="next"}, Previous: [AbstractNamespace class-instance
creation](AbstractNamespace-class_002dinstance-creation.html#AbstractNamespace-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[AbstractNamespace](AbstractNamespace.html#AbstractNamespace){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#AbstractNamespace_003a-accessing}

#### 1.1.2 AbstractNamespace: accessing {#abstractnamespace-accessing .subsection}

[]{#index-allAssociations}

**allAssociations**

Answer a Dictionary with all of the associations in the receiver and
each of its superspaces (duplicate keys are associated to the
associations that are deeper in the namespace hierarchy)

[]{#index-allBehaviorsDo_003a}

**allBehaviorsDo: aBlock**

Evaluate aBlock once for each class and metaclass in the namespace.

[]{#index-allClassObjectsDo_003a}

**allClassObjectsDo: aBlock**

Evaluate aBlock once for each class and metaclass in the namespace.

[]{#index-allClassesDo_003a}

**allClassesDo: aBlock**

Evaluate aBlock once for each class in the namespace.

[]{#index-allMetaclassesDo_003a}

**allMetaclassesDo: aBlock**

Evaluate aBlock once for each metaclass in the namespace.

[]{#index-classAt_003a}

**classAt: aKey**

Answer the value corrisponding to aKey if it is a class. Fail if either
aKey is not found or it is associated to something different from a
class.

[]{#index-classAt_003aifAbsent_003a}

**classAt: aKey ifAbsent: aBlock**

Answer the value corrisponding to aKey if it is a class. Evaluate aBlock
and answer its result if either aKey is not found or it is associated to
something different from a class.
