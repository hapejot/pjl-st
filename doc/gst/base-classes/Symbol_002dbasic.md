[]{#Symbol_002dbasic}

::: header
Next: [Symbol-built
ins](Symbol_002dbuilt-ins.html#Symbol_002dbuilt-ins){accesskey="n"
rel="next"}, Previous: [Symbol-accessing the method
dictionary](Symbol_002daccessing-the-method-dictionary.html#Symbol_002daccessing-the-method-dictionary){accesskey="p"
rel="prev"}, Up: [Symbol](Symbol.html#Symbol){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Symbol_003a-basic}

#### 1.159.5 Symbol: basic {#symbol-basic .subsection}

[]{#index-deepCopy-9}

**deepCopy**

Returns a deep copy of the receiver. As Symbols are identity objects, we
actually return the receiver itself.

[]{#index-keywords} []{#index-_002b-18} []{#index-not-3}
[]{#index-printOn_003a-55} []{#index-ifTrue_003aifFalse_003a-3}

**keywords**

Answer an array of keywords that compose the receiver, which is supposed
to be a valid message name (#+, #not, #printOn:, #ifTrue:ifFalse:, etc.)

[]{#index-numArgs-5} []{#index-_002b-19} []{#index-not-4}
[]{#index-printOn_003a-56} []{#index-ifTrue_003aifFalse_003a-4}

**numArgs**

Answer the number of arguments supported by the receiver, which is
supposed to be a valid message name (#+, #not, #printOn:,
#ifTrue:ifFalse:, etc.)

[]{#index-shallowCopy-6}

**shallowCopy**

Returns a deep copy of the receiver. As Symbols are identity objects, we
actually return the receiver itself.
