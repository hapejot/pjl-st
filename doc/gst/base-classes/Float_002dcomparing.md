[]{#Float_002dcomparing}

::: header
Next:
[Float-compiler](Float_002dcompiler.html#Float_002dcompiler){accesskey="n"
rel="next"}, Previous:
[Float-coercion](Float_002dcoercion.html#Float_002dcoercion){accesskey="p"
rel="prev"}, Up: [Float](Float.html#Float){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Float_003a-comparing}

#### 1.80.8 Float: comparing {#float-comparing .subsection}

[]{#index-max_003a}

**max: aNumber**

Answer the maximum between the receiver and aNumber. Redefine in
subclasses if necessary to ensure that if either self or aNumber is a
NaN, it is always answered.

[]{#index-min_003a}

**min: aNumber**

Answer the minimum between the receiver and aNumber. Redefine in
subclasses if necessary to ensure that if either self or aNumber is a
NaN, it is always answered.

[]{#index-withSignOf_003a}

**withSignOf: aNumber**

Answer the receiver, with its sign possibly changed to match that of
aNumber.
