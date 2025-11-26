[]{#ClassDescription_002dcompiling}

::: header
Next:
[ClassDescription-conversion](ClassDescription_002dconversion.html#ClassDescription_002dconversion){accesskey="n"
rel="next"}, Up:
[ClassDescription](ClassDescription.html#ClassDescription){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ClassDescription_003a-compiling}

#### 1.32.1 ClassDescription: compiling {#classdescription-compiling .subsection}

[]{#index-compile_003aclassified_003a}

**compile: code classified: categoryName**

Compile code in the receiver, assigning the method to the given
category. Answer the newly created CompiledMethod, or nil if an error
was found.

[]{#index-compile_003aclassified_003aifError_003a}

**compile: code classified: categoryName ifError: block**

Compile method source and install in method category, categoryName. If
there are parsing errors, invoke exception block, 'block' (see
compile:ifError:). Return the method

[]{#index-compile_003aclassified_003anotifying_003a}

**compile: code classified: categoryName notifying: requestor**

Compile method source and install in method category, categoryName. If
there are parsing errors, send an error message to requestor
