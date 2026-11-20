[]{#I18N_002eLcMessagesDomain_002dquerying}

::: header
Previous: [I18N.LcMessagesDomain-handling the
cache](I18N_002eLcMessagesDomain_002dhandling-the-cache.html#I18N_002eLcMessagesDomain_002dhandling-the-cache){accesskey="p"
rel="prev"}, Up:
[I18N.LcMessagesDomain](I18N_002eLcMessagesDomain.html#I18N_002eLcMessagesDomain){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLcMessagesDomain_003a-querying}

#### 5.12.3 I18N.LcMessagesDomain: querying {#i18n.lcmessagesdomain-querying .subsection}

[]{#index-_003f-1}

**? aString**

Answer the translation of 'aString', or answer aString itself if none is
available.

[]{#index-at_003a-6}

**at: aString**

Answer the translation of 'aString', or answer aString itself if none is
available.

[]{#index-at_003aplural_003awith_003a}

**at: singularString plural: pluralString with: n**

Answer either the translation of pluralString with '%1' replaced by n if
n \~= 1, or the translation of singularString if n = 1.

[]{#index-at_003aput_003a-1}

**at: aString put: anotherString**

This method should not be called for instances of this class.

[]{#index-translatorInformation}

**translatorInformation**

Answer information on the translation, or nil if there is none. This
information is stored as the 'translation' of an empty string.

[]{#index-translatorInformationAt_003a}

**translatorInformationAt: key**

Answer information on the translation associated to a given key

[]{#index-translatorInformationAt_003aat_003a}

**translatorInformationAt: key at: subkey**

Answer information on the translation associated to a given key and to a
subkey of the key
