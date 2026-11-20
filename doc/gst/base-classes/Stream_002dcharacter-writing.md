[]{#Stream_002dcharacter-writing}

::: header
Next:
[Stream-compiling](Stream_002dcompiling.html#Stream_002dcompiling){accesskey="n"
rel="next"}, Previous: [Stream-built
ins](Stream_002dbuilt-ins.html#Stream_002dbuilt-ins){accesskey="p"
rel="prev"}, Up: [Stream](Stream.html#Stream){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Stream_003a-character-writing}

#### 1.157.6 Stream: character writing {#stream-character-writing .subsection}

[]{#index-cr-1}

**cr**

Store a cr on the receiver

[]{#index-crTab}

**crTab**

Store a cr and a tab on the receiver

[]{#index-encoding-1}

**encoding**

Answer the encoding to be used when storing Unicode characters.

[]{#index-isUnicode-4}

**isUnicode**

Answer whether the receiver is able to store Unicode characters. Note
that if this method returns true, the stream may or may not be able to
store Characters (as opposed to UnicodeCharacters) whose value is above
127.

[]{#index-nl-1}

**nl**

Store a new line on the receiver

[]{#index-nlTab}

**nlTab**

Store a new line and a tab on the receiver

[]{#index-space-1}

**space**

Store a space on the receiver

[]{#index-space_003a}

**space: n**

Store n spaces on the receiver

[]{#index-tab-1}

**tab**

Store a tab on the receiver

[]{#index-tab_003a}

**tab: n**

Store n tabs on the receiver
