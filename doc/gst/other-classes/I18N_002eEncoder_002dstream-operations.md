[]{#I18N_002eEncoder_002dstream-operations}

::: header
Previous: [I18N.Encoder class-instance
creation](I18N_002eEncoder-class_002dinstance-creation.html#I18N_002eEncoder-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[I18N.Encoder](I18N_002eEncoder.html#I18N_002eEncoder){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#I18N_002eEncoder_003a-stream-operations}

#### 5.5.2 I18N.Encoder: stream operations {#i18n.encoder-stream-operations .subsection}

[]{#index-atEnd-1}

**atEnd**

Return whether the receiver can produce another character in the
receiver; by default, this is true if there is another character in the
origin.

[]{#index-atEndOfInput}

**atEndOfInput**

Return whether there is another character in the origin. This method is
for private use by encoders, calling it outside won't corrupt the
internal state of the encoder but the result probably won't be
meaningful (depending on the innards of the encoder).

[]{#index-next-2}

**next**

Return the next character in the receiver; by default, this is the next
character in the origin.

[]{#index-nextInput}

**nextInput**

Return the next character in the origin. This method is for private use
by encoders, calling it outside may corrupt the internal state of the
encoder.

[]{#index-nextInputAvailable_003ainto_003astartingAt_003a}

**nextInputAvailable: n into: aCollection startingAt: pos**

Place up to N characters from the origin in aCollection. This method is
for private use by encoders, calling it outside may corrupt the internal
state of the encoder.

[]{#index-peekInput}

**peekInput**

Return the next character in the origin without advancing it.

[]{#index-species-1}

**species**

We answer a string of Characters encoded in our destination encoding.
