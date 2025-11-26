[]{#ClassDescription_002dprinting}

::: header
Next: [ClassDescription-still
unclassified](ClassDescription_002dstill-unclassified.html#ClassDescription_002dstill-unclassified){accesskey="n"
rel="next"}, Previous: [ClassDescription-parsing class
declarations](ClassDescription_002dparsing-class-declarations.html#ClassDescription_002dparsing-class-declarations){accesskey="p"
rel="prev"}, Up:
[ClassDescription](ClassDescription.html#ClassDescription){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#ClassDescription_003a-printing}

#### 1.32.7 ClassDescription: printing {#classdescription-printing .subsection}

[]{#index-classVariableString}

**classVariableString**

This method's functionality should be implemented by subclasses of
ClassDescription

[]{#index-instanceVariableString}

**instanceVariableString**

Answer a string containing the name of the receiver's instance
variables.

[]{#index-nameIn_003a-3}

**nameIn: aNamespace**

Answer the class name when the class is referenced from aNamespace

[]{#index-printOn_003ain_003a-2}

**printOn: aStream in: aNamespace**

Print on aStream the class name when the class is referenced from
aNamespace

[]{#index-sharedVariableString}

**sharedVariableString**

This method's functionality should be implemented by subclasses of
ClassDescription
