[]{#BLOX_002eBLabel_002daccessing}

::: header
Previous: [BLOX.BLabel class-instance
creation](BLOX_002eBLabel-class_002dinstance-creation.html#BLOX_002eBLabel-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BLabel](BLOX_002eBLabel.html#BLOX_002eBLabel){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBLabel_003a-accessing}

#### 1.23.3 BLOX.BLabel: accessing {#blox.blabel-accessing .subsection}

[]{#index-alignment} []{#index-topLeft} []{#index-topCenter}
[]{#index-topRight} []{#index-leftCenter} []{#index-center-6}
[]{#index-rightCenter} []{#index-bottomLeft} []{#index-bottomCenter}
[]{#index-bottomRight} []{#index-topLeft-1}

**alignment**

Answer the value of the anchor option for the widget.

Specifies how the information in a widget (e.g. text or a bitmap) is to
be displayed in the widget. Must be one of the symbols #topLeft,
#topCenter, #topRight, #leftCenter, #center, #rightCenter, #bottomLeft,
#bottomCenter, #bottomRight. For example, #topLeft means display the
information such that its top-left corner is at the top-left corner of
the widget.

[]{#index-alignment_003a} []{#index-topLeft-2} []{#index-topCenter-1}
[]{#index-topRight-1} []{#index-leftCenter-1} []{#index-center-7}
[]{#index-rightCenter-1} []{#index-bottomLeft-1}
[]{#index-bottomCenter-1} []{#index-bottomRight-1} []{#index-topLeft-3}

**alignment: aSymbol**

Set the value of the anchor option for the widget.

Specifies how the information in a widget (e.g. text or a bitmap) is to
be displayed in the widget. Must be one of the symbols #topLeft,
#topCenter, #topRight, #leftCenter, #center, #rightCenter, #bottomLeft,
#bottomCenter, #bottomRight. For example, #topLeft means display the
information such that its top-left corner is at the top-left corner of
the widget.

[]{#index-backgroundColor-6}

**backgroundColor**

Answer the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-backgroundColor_003a-8}

**backgroundColor: value**

Set the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-font-4}

**font**

Answer the value of the font option for the widget.

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

[]{#index-font_003a-6}

**font: value**

Set the value of the font option for the widget.

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

[]{#index-foregroundColor-5}

**foregroundColor**

Answer the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-foregroundColor_003a-7}

**foregroundColor: value**

Set the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-label-1}

**label**

Answer the value of the label option for the widget.

Specifies a string to be displayed inside the widget. The way in which
the string is displayed depends on the particular widget and may be
determined by other options, such as anchor. For windows, this is the
title of the window.

[]{#index-label_003a-1}

**label: value**

Set the value of the label option for the widget.

Specifies a string to be displayed inside the widget. The way in which
the string is displayed depends on the particular widget and may be
determined by other options, such as anchor. For windows, this is the
title of the window.

------------------------------------------------------------------------

::: header
Previous: [BLOX.BLabel class-instance
creation](BLOX_002eBLabel-class_002dinstance-creation.html#BLOX_002eBLabel-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BLabel](BLOX_002eBLabel.html#BLOX_002eBLabel){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
