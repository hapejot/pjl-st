[]{#I18N_002eLcMessagesMoFileVersion0-class_002dplurals}

::: header
Next: [I18N.LcMessagesMoFileVersion0-flushing the
cache](I18N_002eLcMessagesMoFileVersion0_002dflushing-the-cache.html#I18N_002eLcMessagesMoFileVersion0_002dflushing-the-cache){accesskey="n"
rel="next"}, Previous: [I18N.LcMessagesMoFileVersion0
class-documentation](I18N_002eLcMessagesMoFileVersion0-class_002ddocumentation.html#I18N_002eLcMessagesMoFileVersion0-class_002ddocumentation){accesskey="p"
rel="prev"}, Up:
[I18N.LcMessagesMoFileVersion0](I18N_002eLcMessagesMoFileVersion0.html#I18N_002eLcMessagesMoFileVersion0){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLcMessagesMoFileVersion0-class_003a-plurals}

#### 5.14.2 I18N.LcMessagesMoFileVersion0 class: plurals {#i18n.lcmessagesmofileversion0-class-plurals .subsection}

[]{#index-initialize-3}

**initialize**

Initialize a table with the expressions computing the plurals for the
most common languages

[]{#index-pluralExpressionFor_003aifAbsent_003a}

**pluralExpressionFor: locale ifAbsent: aBlock**

Answer a RunTimeExpression yielding the plural form for the given
language and territory, if one is known, else evaluate aBlock and answer
it.
