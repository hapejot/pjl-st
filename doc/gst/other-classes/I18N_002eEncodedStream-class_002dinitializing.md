[]{#I18N_002eEncodedStream-class_002dinitializing}

::: header
Next: [I18N.EncodedStream class-instance
creation](I18N_002eEncodedStream-class_002dinstance-creation.html#I18N_002eEncodedStream-class_002dinstance-creation){accesskey="n"
rel="next"}, Up:
[I18N.EncodedStream](I18N_002eEncodedStream.html#I18N_002eEncodedStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eEncodedStream-class_003a-initializing}

#### 5.2.1 I18N.EncodedStream class: initializing {#i18n.encodedstream-class-initializing .subsection}

[]{#index-initialize-2}

**initialize**

Initialize the registry of the encoders to include the standard encoders
contained in the library.

[]{#index-registerEncoderFor_003atoUTF32_003afromUTF32_003a}
[]{#index-next-8}

**registerEncoderFor: arrayOfAliases toUTF32: toUTF32Class fromUTF32:
fromUTF32Class**

Register the two classes that will respectively convert from the
charsets in arrayOfAliases to UTF-32 and vice versa.

The former class is a stream that accepts characters and returns (via
#next) integers representing UTF-32 character codes, while the latter
accepts UTF-32 character codes and converts them to characters. For an
example see respectively FromUTF7 and ToUTF7 (I admit it is not a
trivial example).
