[]{#Behavior_002devaluating}

::: header
Next: [Behavior-instance
creation](Behavior_002dinstance-creation.html#Behavior_002dinstance-creation){accesskey="n"
rel="next"}, Previous:
[Behavior-enumerating](Behavior_002denumerating.html#Behavior_002denumerating){accesskey="p"
rel="prev"}, Up: [Behavior](Behavior.html#Behavior){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Behavior_003a-evaluating}

#### 1.9.12 Behavior: evaluating {#behavior-evaluating .subsection}

[]{#index-evalString_003ato_003a}

**evalString: aString to: anObject**

Answer the stack top at the end of the evaluation of the code in
aString. The code is executed as part of anObject

[]{#index-evalString_003ato_003aifError_003a}

**evalString: aString to: anObject ifError: aBlock**

Answer the stack top at the end of the evaluation of the code in
aString. If aString cannot be parsed, evaluate aBlock (see
compile:ifError:). The code is executed as part of anObject

[]{#index-evaluate_003a}

**evaluate: code**

Evaluate Smalltalk expression in 'code' and return result.

[]{#index-evaluate_003aifError_003a}

**evaluate: code ifError: block**

Evaluate 'code'. If a parsing error is detected, invoke 'block'

[]{#index-evaluate_003anotifying_003a} []{#index-error_003a-1}

**evaluate: code notifying: requestor**

Evaluate Smalltalk expression in 'code'. If a parsing error is
encountered, send #error: to requestor

[]{#index-evaluate_003ato_003a}

**evaluate: code to: anObject**

Evaluate Smalltalk expression as part of anObject's method definition

[]{#index-evaluate_003ato_003aifError_003a}

**evaluate: code to: anObject ifError: block**

Evaluate Smalltalk expression as part of anObject's method definition.
This method is used to support Inspector expression evaluation. If a
parsing error is encountered, invoke error block, 'block'
