[]{#I18N_002eIncompleteSequenceError}

::: header
Next:
[I18N.InvalidCharsetError](I18N_002eInvalidCharsetError.html#I18N_002eInvalidCharsetError){accesskey="n"
rel="next"}, Previous:
[I18N.FileStreamSegment](I18N_002eFileStreamSegment.html#I18N_002eFileStreamSegment){accesskey="p"
rel="prev"}, Up: [Iconv/I18N
packages](Iconv_002fI18N-packages.html#Iconv_002fI18N-packages){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eIncompleteSequenceError-1}

### 5.7 I18N.IncompleteSequenceError {#i18n.incompletesequenceerror .section}

[]{#index-I18N_002eIncompleteSequenceError}

**Defined in namespace I18N**\
**Superclass: Error**\
**Category: i18n-Character sets**

:   I am raised if an invalid sequence is found while converting a
    string from a charset to another. In particular, I am raised if the
    input stream ends abruptly in the middle of a multi-byte sequence.

  ---------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [I18N.IncompleteSequenceError-accessing](I18N_002eIncompleteSequenceError_002daccessing.html#I18N_002eIncompleteSequenceError_002daccessing){accesskey="1"}:        (instance)
  ---------------------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
