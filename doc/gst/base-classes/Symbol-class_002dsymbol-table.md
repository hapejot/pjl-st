[]{#Symbol-class_002dsymbol-table}

::: header
Next: [Symbol-accessing the method
dictionary](Symbol_002daccessing-the-method-dictionary.html#Symbol_002daccessing-the-method-dictionary){accesskey="n"
rel="next"}, Previous: [Symbol class-instance
creation](Symbol-class_002dinstance-creation.html#Symbol-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [Symbol](Symbol.html#Symbol){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Symbol-class_003a-symbol-table}

#### 1.159.3 Symbol class: symbol table {#symbol-class-symbol-table .subsection}

[]{#index-hasInterned_003aifTrue_003a} []{#index-hash-41}

**hasInterned: aString ifTrue: aBlock**

If aString has not been interned yet, answer false. Else, pass the
interned version to aBlock and answer true. Note that this works because
String\>\>#hash calculates the same hash value used by the VM when
interning strings into the SymbolTable. Changing one of the hashing
methods without changing the other will break this method.

[]{#index-isSymbolString_003a} []{#index-hash-42}

**isSymbolString: aString**

Answer whether aString has already been interned. Note that this works
because String\>\>#hash calculates the same hash value used by the VM
when interning strings into the SymbolTable. Changing one of the hashing
methods without changing the other will break this method.

[]{#index-rebuildTable} []{#index-hash-43}

**rebuildTable**

Rebuild the SymbolTable, thereby garbage-collecting unreferenced
Symbols. While this process is done, preemption is disabled because it
is not acceptable to leave the SymbolTable in a partially updated state.
Note that this works because String\>\>#hash calculates the same hash
value used by the VM when interning strings into the SymbolTable.
Changing one of the hashing methods without changing the other will
break this method.
