[]{#I18N_002eLcMessagesDomain}

::: header
Next:
[I18N.LcMessagesDummyDomain](I18N_002eLcMessagesDummyDomain.html#I18N_002eLcMessagesDummyDomain){accesskey="n"
rel="next"}, Previous:
[I18N.LcMessagesCatalog](I18N_002eLcMessagesCatalog.html#I18N_002eLcMessagesCatalog){accesskey="p"
rel="prev"}, Up: [Iconv/I18N
packages](Iconv_002fI18N-packages.html#Iconv_002fI18N-packages){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLcMessagesDomain-1}

### 5.12 I18N.LcMessagesDomain {#i18n.lcmessagesdomain .section}

[]{#index-I18N_002eLcMessagesDomain}

**Defined in namespace I18N**\
**Superclass: I18N.LocaleData**\
**Category: i18n-Messages**

:   This object is an abstract superclass for message domains
    (catalogs). It contains methods to create instances of its
    subclasses, but they are commonly used only by LcMessages.

    Translations are accessed using either #at: or the shortcut binary
    messages '?'. This way, common idioms to access translated strings
    will be

    string := NLS? 'abc'. string := self? 'abc'.

    (in the first case NLS is a class variable, in the second the
    receiver implements #? through delegation) which is only five or six
    characters longer than the traditional

    string := 'abc'.

    (cfr. the \_(\"abc\") idiom used by GNU gettext)

  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [I18N.LcMessagesDomain class-opening MO files](I18N_002eLcMessagesDomain-class_002dopening-MO-files.html#I18N_002eLcMessagesDomain-class_002dopening-MO-files){accesskey="1"}:        (class)
  • [I18N.LcMessagesDomain-handling the cache](I18N_002eLcMessagesDomain_002dhandling-the-cache.html#I18N_002eLcMessagesDomain_002dhandling-the-cache){accesskey="2"}:                    (instance)
  • [I18N.LcMessagesDomain-querying](I18N_002eLcMessagesDomain_002dquerying.html#I18N_002eLcMessagesDomain_002dquerying){accesskey="3"}:                                                  (instance)
  ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
