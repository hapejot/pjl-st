[]{#I18N_002eLocaleData-class_002daccessing}

::: header
Next: [I18N.LocaleData
class-database](I18N_002eLocaleData-class_002ddatabase.html#I18N_002eLocaleData-class_002ddatabase){accesskey="n"
rel="next"}, Up:
[I18N.LocaleData](I18N_002eLocaleData.html#I18N_002eLocaleData){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLocaleData-class_003a-accessing}

#### 5.23.1 I18N.LocaleData class: accessing {#i18n.localedata-class-accessing .subsection}

[]{#index-category-4}

**category**

Answer 'nil'.

[]{#index-default-2}

**default**

This method's functionality should be implemented by subclasses of
LocaleData

[]{#index-flush-3}

**flush**

Flush the contents of the instances of each subclass of LocaleData.

[]{#index-fromString_003a-4}

**fromString: lang**

This method's functionality should be implemented by subclasses of
LocaleData

[]{#index-language_003a}

**language: lang**

Answer the local object for the given language.

[]{#index-language_003aterritory_003a}

**language: lang territory: territory**

Answer the local object for the given language and territory.

[]{#index-language_003aterritory_003acharset_003a}

**language: lang territory: territory charset: charset**

Answer the local object for the given language, territory and charset.

[]{#index-new-11}

**new**

This method should not be called for instances of this class.

[]{#index-posix-2}

**posix**

This method's functionality should be implemented by subclasses of
LocaleData

[]{#index-update_003a-1}

**update: aspect**

Flush instances of the receiver when an image is loaded.
