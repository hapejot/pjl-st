[]{#I18N_002eLocale-class_002dinstance-creation}

::: header
Next: [I18N.Locale-C
call-outs](I18N_002eLocale_002dC-call_002douts.html#I18N_002eLocale_002dC-call_002douts){accesskey="n"
rel="next"}, Previous: [I18N.Locale
class-initialization](I18N_002eLocale-class_002dinitialization.html#I18N_002eLocale-class_002dinitialization){accesskey="p"
rel="prev"}, Up:
[I18N.Locale](I18N_002eLocale.html#I18N_002eLocale){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLocale-class_003a-instance-creation}

#### 5.21.3 I18N.Locale class: instance creation {#i18n.locale-class-instance-creation .subsection}

[]{#index-default}

**default**

Answer an instance of the receiver that accesses the default locale.

[]{#index-flush-2}

**flush**

Flush the information on locales that are not valid across an image
save/load.

[]{#index-fromString_003a-2}

**fromString: aString**

Answer an instance of the receiver that accesses the given locale (in
the form language\[\_territory\]\[.charset\]).

[]{#index-posix}

**posix**

Answer an instance of the receiver that accesses the POSIX locale.
