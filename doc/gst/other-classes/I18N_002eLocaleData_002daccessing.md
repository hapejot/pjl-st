[]{#I18N_002eLocaleData_002daccessing}

::: header
Next:
[I18N.LocaleData-initialization](I18N_002eLocaleData_002dinitialization.html#I18N_002eLocaleData_002dinitialization){accesskey="n"
rel="next"}, Previous: [I18N.LocaleData
class-database](I18N_002eLocaleData-class_002ddatabase.html#I18N_002eLocaleData-class_002ddatabase){accesskey="p"
rel="prev"}, Up:
[I18N.LocaleData](I18N_002eLocaleData.html#I18N_002eLocaleData){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLocaleData_003a-accessing}

#### 5.23.3 I18N.LocaleData: accessing {#i18n.localedata-accessing .subsection}

[]{#index-charset}

**charset**

Return the charset supported by the receiver.

[]{#index-id}

**id**

Return the identifier of the locale supported by the receiver.

[]{#index-isPosixLocale}

**isPosixLocale**

Answer whether the receiver implements the default POSIX behavior for a
locale.

[]{#index-language}

**language**

Return the language supported by the receiver.

[]{#index-languageDirectory-1}

**languageDirectory**

Answer the directory where data files for the current language reside.

[]{#index-languageDirectory_003a-1}

**languageDirectory: rootDirectory**

Answer the directory where data files for the current language reside,
given the root directory of the locale data.

[]{#index-territory}

**territory**

Return the territory supported by the receiver.

[]{#index-territoryDirectory-1}

**territoryDirectory**

Answer the directory where data files for the current language, specific
to the territory, reside.

[]{#index-territoryDirectory_003a-1}

**territoryDirectory: rootDirectory**

Answer the directory where data files for the current language, specific
to the territory, reside, given the root directory of the locale data.
