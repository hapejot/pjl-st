[]{#CCompound-class_002dsubclass-creation}

::: header
Next:
[CCompound-debugging](CCompound_002ddebugging.html#CCompound_002ddebugging){accesskey="n"
rel="next"}, Previous: [CCompound class-instance
creation](CCompound-class_002dinstance-creation.html#CCompound-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up: [CCompound](CCompound.html#CCompound){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#CCompound-class_003a-subclass-creation}

#### 1.24.2 CCompound class: subclass creation {#ccompound-class-subclass-creation .subsection}

[]{#index-alignof-5}

**alignof**

Answer 1, the alignment of an empty struct

[]{#index-classPragmas}

**classPragmas**

Return the pragmas that are written in the file-out of this class.

[]{#index-compileSize_003aalign_003a}

**compileSize: size align: alignment**

Private - Compile sizeof and alignof methods

[]{#index-declaration}

**declaration**

Return the description of the fields in the receiver class.

[]{#index-declaration_003a}

**declaration: array**

This method's functionality should be implemented by subclasses of
CCompound

[]{#index-declaration_003ainject_003ainto_003a}

**declaration: array inject: startOffset into: aBlock**

Compile methods that implement the declaration in array. To compute the
offset after each field, the value of the old offset plus the new
field's size is passed to aBlock, together with the new field's
alignment requirements.

[]{#index-emitFieldNameTo_003afor_003a} []{#index-examineOn_003a-6}

**emitFieldNameTo: str for: name**

Private - Emit onto the given stream the code for adding the given
selector to the CCompound's #examineOn: method.

[]{#index-newStruct_003adeclaration_003a}
[]{#index-subclass_003adeclaration_003a}

**newStruct: structName declaration: array**

The old way to create a CStruct. Superseded by
#subclass:declaration:\...

[]{#index-sizeof-5}

**sizeof**

Answer 0, the size of an empty struct

[]{#index-subclass_003adeclaration_003aclassVariableNames_003apoolDictionaries_003acategory_003a}

**subclass: structName declaration: array classVariableNames: cvn
poolDictionaries: pd category: category**

Create a new class with the given name that contains code to implement
the given C struct. All the parameters except 'array' are the same as
for a standard class creation message; see documentation for more
information
