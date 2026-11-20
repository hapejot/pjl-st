[]{#BLOX_002eBEmbeddedText_002daccessing}

::: header
Up:
[BLOX.BEmbeddedText](BLOX_002eBEmbeddedText.html#BLOX_002eBEmbeddedText){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBEmbeddedText_003a-accessing}

#### 1.17.1 BLOX.BEmbeddedText: accessing {#blox.bembeddedtext-accessing .subsection}

[]{#index-font-3}

**font**

Answer the value of the font option for the canvas object.

Specifies the font to use when drawing text inside the widget. The font
can be given as either an X font name or a Blox font description string.

X font names are given as many fields, each led by a minus, and each of
which can be replaced by an \* to indicate a default value is ok:
foundry, family, weight, slant, setwidth, addstyle, pixel size, point
size (the same as pixel size for historical reasons), horizontal
resolution, vertical resolution, spacing, width, charset and character
encoding.

Blox font description strings have three fields, which must be separated
by a space and of which only the first is mandatory: the font family,
the font size in points (or in pixels if a negative value is supplied),
and a number of styles separated by a space (valid styles are normal,
bold, italic, underline and overstrike). Examples of valid fonts are
"Helvetica 10 Bold", "Times -14", "Futura Bold Underline". You must
enclose the font family in braces if it is made of two or more words.

[]{#index-font_003a-5}

**font: font**

Set the value of the font option for the canvas object.

Specifies the font to use when drawing text inside the widget. The font
can be given as either an X font name or a Blox font description string.

X font names are given as many fields, each led by a minus, and each of
which can be replaced by an \* to indicate a default value is ok:
foundry, family, weight, slant, setwidth, addstyle, pixel size, point
size (the same as pixel size for historical reasons), horizontal
resolution, vertical resolution, spacing, width, charset and character
encoding.

Blox font description strings have three fields, which must be separated
by a space and of which only the first is mandatory: the font family,
the font size in points (or in pixels if a negative value is supplied),
and a number of styles separated by a space (valid styles are normal,
bold, italic, underline and overstrike). Examples of valid fonts are
"Helvetica 10 Bold", "Times -14", "Futura Bold Underline". You must
enclose the font family in braces if it is made of two or more words.

[]{#index-justify}

**justify**

Answer how to justify the text within its bounding region.

[]{#index-justify_003a} []{#index-left} []{#index-right}
[]{#index-center-5}

**justify: aSymbol**

Sets how to justify the text within its bounding region. Can be #left,
#right or #center (the default).

[]{#index-redraw-2}

**redraw**

Force the object to be displayed in the parent canvas, creating it if it
has not been inserted for real in the parent, and refresh its position.

[]{#index-text-4}

**text**

Answer the text that is printed by the object

[]{#index-text_003a-3}

**text: aString**

Set the text that is printed by the object

------------------------------------------------------------------------

::: header
Up:
[BLOX.BEmbeddedText](BLOX_002eBEmbeddedText.html#BLOX_002eBEmbeddedText){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
