[]{#I18N_002eLocaleConventions-class_002daccessing}

::: header
Next:
[I18N.LocaleConventions-accessing](I18N_002eLocaleConventions_002daccessing.html#I18N_002eLocaleConventions_002daccessing){accesskey="n"
rel="next"}, Up:
[I18N.LocaleConventions](I18N_002eLocaleConventions.html#I18N_002eLocaleConventions){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLocaleConventions-class_003a-accessing}

#### 5.22.1 I18N.LocaleConventions class: accessing {#i18n.localeconventions-class-accessing .subsection}

[]{#index-_003f-3}

**? anObject**

Query the default object, forwarding the message to it.

[]{#index-default-1}

**default**

Answer an instance of the receiver that accesses the default locale.

[]{#index-fromString_003a-3}

**fromString: aString**

Answer an instance of the receiver that accesses the given locale (in
the form language\[\_territory\]\[.charset\]).

[]{#index-posix-1}

**posix**

Answer an instance of the receiver that accesses the POSIX locale.

[]{#index-selector-5}

**selector**

This method's functionality should be implemented by subclasses of
LocaleConventions
