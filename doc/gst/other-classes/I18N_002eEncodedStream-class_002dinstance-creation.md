[]{#I18N_002eEncodedStream-class_002dinstance-creation}

::: header
Previous: [I18N.EncodedStream
class-initializing](I18N_002eEncodedStream-class_002dinitializing.html#I18N_002eEncodedStream-class_002dinitializing){accesskey="p"
rel="prev"}, Up:
[I18N.EncodedStream](I18N_002eEncodedStream.html#I18N_002eEncodedStream){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eEncodedStream-class_003a-instance-creation}

#### 5.2.2 I18N.EncodedStream class: instance creation {#i18n.encodedstream-class-instance-creation .subsection}

[]{#index-encoding_003a}

**encoding: anUnicodeString**

Answer a pipe of encoders that converts anUnicodeString to default
encoding for strings (the current locale's default charset if none is
specified).

[]{#index-encoding_003aas_003a}

**encoding: aStringOrStream as: toEncoding**

Answer a pipe of encoders that converts anUnicodeString (which contains
to the supplied encoding (which can be an ASCII String or Symbol).

[]{#index-on_003afrom_003a}

**on: aStringOrStream from: fromEncoding**

Answer a pipe of encoders that converts aStringOrStream (which can be a
string or another stream) from the given encoding to the default
locale's default charset.

[]{#index-on_003afrom_003ato_003a}

**on: aStringOrStream from: fromEncoding to: toEncoding**

Answer a pipe of encoders that converts aStringOrStream (which can be a
string or another stream) between the two supplied encodings (which can
be ASCII Strings or Symbols)

[]{#index-on_003ato_003a}

**on: aStringOrStream to: toEncoding**

Answer a pipe of encoders that converts aStringOrStream (which can be a
string or another stream) from the default locale's default charset to
the given encoding.

[]{#index-unicodeOn_003a}

**unicodeOn: aStringOrStream**

Answer a pipe of encoders that converts aStringOrStream (which can be a
string or another stream) from its encoding (or the current locale's
default charset, if the encoding cannot be determined) to integers
representing Unicode character codes.

[]{#index-unicodeOn_003aencoding_003a}

**unicodeOn: aStringOrStream encoding: fromEncoding**

Answer a pipe of encoders that converts aStringOrStream (which can be a
string or another stream) from the supplied encoding (which can be an
ASCII String or Symbol) to integers representing Unicode character
codes.
