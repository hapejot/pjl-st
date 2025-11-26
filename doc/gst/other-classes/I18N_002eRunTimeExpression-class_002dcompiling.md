[]{#I18N_002eRunTimeExpression-class_002dcompiling}

::: header
Next: [I18N.RunTimeExpression
class-initializing](I18N_002eRunTimeExpression-class_002dinitializing.html#I18N_002eRunTimeExpression-class_002dinitializing){accesskey="n"
rel="next"}, Up:
[I18N.RunTimeExpression](I18N_002eRunTimeExpression.html#I18N_002eRunTimeExpression){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eRunTimeExpression-class_003a-compiling}

#### 5.29.1 I18N.RunTimeExpression class: compiling {#i18n.runtimeexpression-class-compiling .subsection}

[]{#index-parseExpression_003a}

**parseExpression: stream**

Private - Compile the expression in the stream

[]{#index-parseOperand_003a}

**parseOperand: stream**

Parse an operand from the stream (i.e. an unary negation, a
parenthesized subexpression, 'n' or a number) and answer the
corresponding parse node.

[]{#index-parseOperator_003a}

**parseOperator: stream**

Answer a Symbol for an operator read from stream, or nil if something
else is found.
