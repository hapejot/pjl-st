[]{#I18N_002eRTEBinaryNode_002dcomputing}

::: header
Previous:
[I18N.RTEBinaryNode-compiling](I18N_002eRTEBinaryNode_002dcompiling.html#I18N_002eRTEBinaryNode_002dcompiling){accesskey="p"
rel="prev"}, Up:
[I18N.RTEBinaryNode](I18N_002eRTEBinaryNode.html#I18N_002eRTEBinaryNode){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eRTEBinaryNode_003a-computing}

#### 5.25.3 I18N.RTEBinaryNode: computing {#i18n.rtebinarynode-computing .subsection}

[]{#index-lhs_003aop_003arhs_003a-1}

**lhs: lhsNode op: aSymbol rhs: rhsNode**

Initialize the children of the receiver and the operation to be done
between them

[]{#index-printOn_003a-6}

**printOn: aStream**

Print a representation of the receiver on aStream

[]{#index-send_003a-1}

**send: parameter**

Private - Evaluate the receiver by evaluating both children and
performing an arithmetic operation between them.
