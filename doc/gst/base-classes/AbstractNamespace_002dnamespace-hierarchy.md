[]{#AbstractNamespace_002dnamespace-hierarchy}

::: header
Next: [AbstractNamespace-overrides for
superspaces](AbstractNamespace_002doverrides-for-superspaces.html#AbstractNamespace_002doverrides-for-superspaces){accesskey="n"
rel="next"}, Previous:
[AbstractNamespace-copying](AbstractNamespace_002dcopying.html#AbstractNamespace_002dcopying){accesskey="p"
rel="prev"}, Up:
[AbstractNamespace](AbstractNamespace.html#AbstractNamespace){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#AbstractNamespace_003a-namespace-hierarchy}

#### 1.1.5 AbstractNamespace: namespace hierarchy {#abstractnamespace-namespace-hierarchy .subsection}

[]{#index-addSubspace_003a}

**addSubspace: aSymbol**

Create a namespace named aSymbol, add it to the receiver's subspaces,
and answer it.

[]{#index-allSubassociationsDo_003a}

**allSubassociationsDo: aBlock**

Invokes aBlock once for every association in each of the receiver's
subspaces.

[]{#index-allSubspaces}

**allSubspaces**

Answer the direct and indirect subspaces of the receiver in a Set

[]{#index-allSubspacesDo_003a}

**allSubspacesDo: aBlock**

Invokes aBlock for all subspaces, both direct and indirect.

[]{#index-allSuperspacesDo_003a}

**allSuperspacesDo: aBlock**

Evaluate aBlock once for each of the receiver's superspaces

[]{#index-includesClassNamed_003a} []{#index-includesKey_003a-4}
[]{#index-includesGlobalNamed_003a-1}

**includesClassNamed: aString**

Answer whether the receiver or any of its superspaces include the given
class -- note that this method (unlike #includesKey:) does not require
aString to be interned and (unlike #includesGlobalNamed:) only returns
true if the global is a class object.

[]{#index-includesGlobalNamed_003a} []{#index-includesKey_003a-5}
[]{#index-includesClassNamed_003a-1}

**includesGlobalNamed: aString**

Answer whether the receiver or any of its superspaces include the given
key -- note that this method (unlike #includesKey:) does not require
aString to be interned but (unlike #includesClassNamed:) returns true
even if the global is not a class object.

[]{#index-removeSubspace_003a}

**removeSubspace: aSymbol**

Remove my subspace named aSymbol from the hierarchy.

[]{#index-selectSubspaces_003a}

**selectSubspaces: aBlock**

Return a Set of subspaces of the receiver satisfying aBlock.

[]{#index-selectSuperspaces_003a}

**selectSuperspaces: aBlock**

Return a Set of superspaces of the receiver satisfying aBlock.

[]{#index-siblings}

**siblings**

Answer all the other children of the same namespace as the receiver.

[]{#index-siblingsDo_003a}

**siblingsDo: aBlock**

Evaluate aBlock once for each of the other root namespaces, passing the
namespace as a parameter.

[]{#index-subspaces}

**subspaces**

Answer the receiver's direct subspaces

[]{#index-subspacesDo_003a}

**subspacesDo: aBlock**

Invokes aBlock for all direct subspaces.

[]{#index-superspace}

**superspace**

Answer the receiver's superspace.

[]{#index-superspace_003a}

**superspace: aNamespace**

Set the superspace of the receiver to be 'aNamespace'. Also adds the
receiver as a subspace of it.

[]{#index-withAllSubspaces}

**withAllSubspaces**

Answer a Set containing the receiver together with its direct and
indirect subspaces

[]{#index-withAllSubspacesDo_003a}

**withAllSubspacesDo: aBlock**

Invokes aBlock for the receiver and all subclasses, both direct and
indirect.

------------------------------------------------------------------------

::: header
Next: [AbstractNamespace-overrides for
superspaces](AbstractNamespace_002doverrides-for-superspaces.html#AbstractNamespace_002doverrides-for-superspaces){accesskey="n"
rel="next"}, Previous:
[AbstractNamespace-copying](AbstractNamespace_002dcopying.html#AbstractNamespace_002dcopying){accesskey="p"
rel="prev"}, Up:
[AbstractNamespace](AbstractNamespace.html#AbstractNamespace){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
