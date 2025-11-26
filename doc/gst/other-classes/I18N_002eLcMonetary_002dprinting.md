[]{#I18N_002eLcMonetary_002dprinting}

::: header
Previous: [I18N.LcMonetary
class-accessing](I18N_002eLcMonetary-class_002daccessing.html#I18N_002eLcMonetary-class_002daccessing){accesskey="p"
rel="prev"}, Up:
[I18N.LcMonetary](I18N_002eLcMonetary.html#I18N_002eLcMonetary){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLcMonetary_003a-printing}

#### 5.16.2 I18N.LcMonetary: printing {#i18n.lcmonetary-printing .subsection}

[]{#index-print_003aon_003a-2}

**print: aNumber on: aStream**

Print aNumber on aStream according to the receiver's formatting
conventions. Always print a currency sign and don't force to print
negative numbers by putting parentheses around them.

[]{#index-print_003aon_003acurrency_003aparentheses_003a}

**print: aNumber on: aStream currency: currency parentheses: p**

Print aNumber on aStream according to the receiver's formatting
conventions. If currency is true, print a currency sign, and if p is
true force to print negative numbers by putting parentheses around them.
If p is true, for positive numbers spaces are put around the number to
keep them aligned.
