[]{#CompiledCode_002dbasic}

::: header
Next:
[CompiledCode-copying](CompiledCode_002dcopying.html#CompiledCode_002dcopying){accesskey="n"
rel="next"}, Previous:
[CompiledCode-accessing](CompiledCode_002daccessing.html#CompiledCode_002daccessing){accesskey="p"
rel="prev"}, Up:
[CompiledCode](CompiledCode.html#CompiledCode){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CompiledCode_003a-basic}

#### 1.39.5 CompiledCode: basic {#compiledcode-basic .subsection}

[]{#index-_003d-10} []{#index-_0023}

**= aMethod**

Answer whether the receiver is the same object as arg. Testing for
equality could break the browser, since it's possible to put arbitrary
objects via ##(\...), so this is safer.

[]{#index-hash-6}

**hash**

Answer an hash value for the receiver

[]{#index-methodCategory-1}

**methodCategory**

Answer the method category

[]{#index-methodCategory_003a-1}

**methodCategory: aCategory**

Set the method category to the given string

[]{#index-methodSourceCode-1}

**methodSourceCode**

Answer the method source code (a FileSegment or String or nil)

[]{#index-methodSourceFile-1}

**methodSourceFile**

Answer the file where the method source code is stored

[]{#index-methodSourcePos-1}

**methodSourcePos**

Answer the location where the method source code is stored in the
methodSourceFile

[]{#index-methodSourceString-1}

**methodSourceString**

Answer the method source code as a string
