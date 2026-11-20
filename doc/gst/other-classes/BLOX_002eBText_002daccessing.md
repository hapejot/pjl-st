[]{#BLOX_002eBText_002daccessing}

::: header
Next:
[BLOX.BText-attributes](BLOX_002eBText_002dattributes.html#BLOX_002eBText_002dattributes){accesskey="n"
rel="next"}, Previous: [BLOX.BText class-instance
creation](BLOX_002eBText-class_002dinstance-creation.html#BLOX_002eBText-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BText](BLOX_002eBText.html#BLOX_002eBText){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBText_003a-accessing}

#### 1.42.3 BLOX.BText: accessing {#blox.btext-accessing .subsection}

[]{#index-backgroundColor-10}

**backgroundColor**

Answer the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-backgroundColor_003a-12}

**backgroundColor: value**

Set the value of the backgroundColor option for the widget.

Specifies the normal background color to use when displaying the widget.

[]{#index-callback-6}

**callback**

Answer a DirectedMessage that is sent when the receiver is modified, or
nil if none has been set up.

[]{#index-callback_003amessage_003a-8}

**callback: aReceiver message: aSymbol**

Set up so that aReceiver is sent the aSymbol message (the name of a
zero- or one-argument selector) when the receiver is modified. If the
method accepts an argument, the receiver is passed.

[]{#index-contents-2}

**contents**

Return the contents of the widget

[]{#index-contents_003a-4}

**contents: aString**

Set the contents of the widget

[]{#index-font-6}

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

[]{#index-font_003a-8}

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

[]{#index-foregroundColor-9}

**foregroundColor**

Answer the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-foregroundColor_003a-11}

**foregroundColor: value**

Set the value of the foregroundColor option for the widget.

Specifies the normal foreground color to use when displaying the widget.

[]{#index-getSelection}

**getSelection**

Answer an empty string if the widget has no selection, else answer the
currently selected text

[]{#index-selectBackground-1}

**selectBackground**

Answer the value of the selectBackground option for the widget.

Specifies the background color to use when displaying selected parts of
the widget.

[]{#index-selectBackground_003a-1}

**selectBackground: value**

Set the value of the selectBackground option for the widget.

Specifies the background color to use when displaying selected parts of
the widget.

[]{#index-selectForeground-1}

**selectForeground**

Answer the value of the selectForeground option for the widget.

Specifies the foreground color to use when displaying selected parts of
the widget.

[]{#index-selectForeground_003a-1}

**selectForeground: value**

Set the value of the selectForeground option for the widget.

Specifies the foreground color to use when displaying selected parts of
the widget.

[]{#index-wrap} []{#index-none} []{#index-char} []{#index-word}

**wrap**

Answer the value of the wrap option for the widget.

Specifies how to handle lines in the text that are too long to be
displayed in a single line of the text's window. The value must be #none
or #char or #word. A wrap mode of none means that each line of text
appears as exactly one line on the screen; extra characters that do not
fit on the screen are not displayed. In the other modes each line of
text will be broken up into several screen lines if necessary to keep
all the characters visible. In char mode a screen line break may occur
after any character; in word mode a line break will only be made at word
boundaries.

[]{#index-wrap_003a} []{#index-none-1} []{#index-char-1}
[]{#index-word-1}

**wrap: value**

Set the value of the wrap option for the widget.

Specifies how to handle lines in the text that are too long to be
displayed in a single line of the text's window. The value must be #none
or #char or #word. A wrap mode of none means that each line of text
appears as exactly one line on the screen; extra characters that do not
fit on the screen are not displayed. In the other modes each line of
text will be broken up into several screen lines if necessary to keep
all the characters visible. In char mode a screen line break may occur
after any character; in word mode a line break will only be made at word
boundaries.

------------------------------------------------------------------------

::: header
Next:
[BLOX.BText-attributes](BLOX_002eBText_002dattributes.html#BLOX_002eBText_002dattributes){accesskey="n"
rel="next"}, Previous: [BLOX.BText class-instance
creation](BLOX_002eBText-class_002dinstance-creation.html#BLOX_002eBText-class_002dinstance-creation){accesskey="p"
rel="prev"}, Up:
[BLOX.BText](BLOX_002eBText.html#BLOX_002eBText){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
