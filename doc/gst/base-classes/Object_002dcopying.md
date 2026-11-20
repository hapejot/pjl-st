[]{#Object_002dcopying}

::: header
Next:
[Object-debugging](Object_002ddebugging.html#Object_002ddebugging){accesskey="n"
rel="next"}, Previous:
[Object-conversion](Object_002dconversion.html#Object_002dconversion){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-copying}

#### 1.123.7 Object: copying {#object-copying .subsection}

[]{#index-copy-2}

**copy**

Returns a shallow copy of the receiver (the instance variables are not
copied). The shallow copy receives the message postCopy and the result
of postCopy is passed back.

[]{#index-deepCopy-7}

**deepCopy**

Returns a deep copy of the receiver (the instance variables are copies
of the receiver's instance variables)

[]{#index-postCopy-3}

**postCopy**

Performs any changes required to do on a copied object. This is the
place where one could, for example, put code to replace objects with
copies of the objects
