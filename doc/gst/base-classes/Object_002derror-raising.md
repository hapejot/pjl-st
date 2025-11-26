[]{#Object_002derror-raising}

::: header
Next:
[Object-finalization](Object_002dfinalization.html#Object_002dfinalization){accesskey="n"
rel="next"}, Previous: [Object-dependents
access](Object_002ddependents-access.html#Object_002ddependents-access){accesskey="p"
rel="prev"}, Up: [Object](Object.html#Object){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Object_003a-error-raising}

#### 1.123.10 Object: error raising {#object-error-raising .subsection}

[]{#index-doesNotUnderstand_003a-2}

**doesNotUnderstand: aMessage**

Called by the system when a selector was not found. message is a Message
containing information on the receiver

[]{#index-error_003a}

**error: message**

Display a walkback for the receiver, with the given error message.
Signal an 'Error' exception.

[]{#index-halt_003a}

**halt: message**

Display a walkback for the receiver, with the given error message.
Signal an 'Halt' exception.
