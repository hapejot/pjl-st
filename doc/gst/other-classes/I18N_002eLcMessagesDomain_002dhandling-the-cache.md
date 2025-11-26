[]{#I18N_002eLcMessagesDomain_002dhandling-the-cache}

::: header
Next:
[I18N.LcMessagesDomain-querying](I18N_002eLcMessagesDomain_002dquerying.html#I18N_002eLcMessagesDomain_002dquerying){accesskey="n"
rel="next"}, Previous: [I18N.LcMessagesDomain class-opening MO
files](I18N_002eLcMessagesDomain-class_002dopening-MO-files.html#I18N_002eLcMessagesDomain-class_002dopening-MO-files){accesskey="p"
rel="prev"}, Up:
[I18N.LcMessagesDomain](I18N_002eLcMessagesDomain.html#I18N_002eLcMessagesDomain){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eLcMessagesDomain_003a-handling-the-cache}

#### 5.12.2 I18N.LcMessagesDomain: handling the cache {#i18n.lcmessagesdomain-handling-the-cache .subsection}

[]{#index-flush}

**flush**

Flush the receiver's cache of translations

[]{#index-shouldCache}

**shouldCache**

Answer whether translations should be cached. Never override this method
to always answer false, because that would cause bugs when
transliteration is being used.
