[]{#BLOX_002eBTextAttributes_002dsetting-attributes}

::: header
Previous:
[BLOX.BTextAttributes-colors](BLOX_002eBTextAttributes_002dcolors.html#BLOX_002eBTextAttributes_002dcolors){accesskey="p"
rel="prev"}, Up:
[BLOX.BTextAttributes](BLOX_002eBTextAttributes.html#BLOX_002eBTextAttributes){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBTextAttributes_003a-setting-attributes}

#### 1.43.3 BLOX.BTextAttributes: setting attributes {#blox.btextattributes-setting-attributes .subsection}

[]{#index-backgroundColor-11}

**backgroundColor**

Answer the value of the backgroundColor option for the text.

Specifies the background color to use when displaying text with these
attributes. nil indicates that the default value is not overridden.

[]{#index-backgroundColor_003a-14}

**backgroundColor: color**

Set the value of the backgroundColor option for the text.

Specifies the background color to use when displaying text with these
attributes. nil indicates that the default value is not overridden.

[]{#index-center-3}

**center**

Center the text to which these attributes are applied

[]{#index-events}

**events**

Answer the event bindings which apply to text subject to these
attributes

[]{#index-events_003a-1}

**events: aBTextBindings**

Set the event bindings which apply to text subject to these attributes

[]{#index-font-7}

**font**

Answer the value of the font option for the text. The font can be given
as either an X font name or a Blox font description string, or nil if
you want the widget's default font to apply.

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

[]{#index-font_003a-10}

**font: fontName**

Set the value of the font option for the text. The font can be given as
either an X font name or a Blox font description string, or nil if you
want the widget's default font to apply.

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

[]{#index-foregroundColor-10}

**foregroundColor**

Answer the value of the foregroundColor option for the text.

Specifies the foreground color to use when displaying text with these
attributes. nil indicates that the default value is not overridden.

[]{#index-foregroundColor_003a-13}

**foregroundColor: color**

Set the value of the foregroundColor option for the text.

Specifies the foreground color to use when displaying text with these
attributes. nil indicates that the default value is not overridden.

[]{#index-isCentered}

**isCentered**

Answer whether the text to which these attributes are applied is
centered

[]{#index-isStruckout}

**isStruckout**

Answer whether the text to which these attributes are applied is
struckout

[]{#index-isUnderlined}

**isUnderlined**

Answer whether the text to which these attributes are applied is
underlined

[]{#index-strikeout-1}

**strikeout**

Strike out the text to which these attributes are applied

[]{#index-underline-1}

**underline**

Underline the text to which these attributes are applied

------------------------------------------------------------------------

::: header
Previous:
[BLOX.BTextAttributes-colors](BLOX_002eBTextAttributes_002dcolors.html#BLOX_002eBTextAttributes_002dcolors){accesskey="p"
rel="prev"}, Up:
[BLOX.BTextAttributes](BLOX_002eBTextAttributes.html#BLOX_002eBTextAttributes){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
