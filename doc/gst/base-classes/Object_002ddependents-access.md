[]{#Object_002ddependents-access}

::: header
Next: [Object-error
raising](Object_002derror-raising.html#Object_002derror-raising){accesskey="n"
rel="next"}, Previous:
[Object-debugging](Object_002ddebugging.html#Object_002ddebugging){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-dependents-access}

#### 1.123.9 Object: dependents access {#object-dependents-access .subsection}

[]{#index-addDependent_003a}

**addDependent: anObject**

Add anObject to the set of the receiver's dependents. Important: if an
object has dependents, it won't be garbage collected.

[]{#index-dependents}

**dependents**

Answer a collection of the receiver's dependents.

[]{#index-release}

**release**

Remove all of the receiver's dependents from the set and allow the
receiver to be garbage collected.

[]{#index-removeDependent_003a}

**removeDependent: anObject**

Remove anObject to the set of the receiver's dependents. No problem if
anObject is not in the set of the receiver's dependents.
