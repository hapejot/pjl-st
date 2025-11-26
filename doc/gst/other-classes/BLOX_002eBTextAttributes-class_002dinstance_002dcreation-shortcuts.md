[]{#BLOX_002eBTextAttributes-class_002dinstance_002dcreation-shortcuts}

::: header
Next:
[BLOX.BTextAttributes-colors](BLOX_002eBTextAttributes_002dcolors.html#BLOX_002eBTextAttributes_002dcolors){accesskey="n"
rel="next"}, Up:
[BLOX.BTextAttributes](BLOX_002eBTextAttributes.html#BLOX_002eBTextAttributes){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBTextAttributes-class_003a-instance_002dcreation-shortcuts}

#### 1.43.1 BLOX.BTextAttributes class: instance-creation shortcuts {#blox.btextattributes-class-instance-creation-shortcuts .subsection}

[]{#index-backgroundColor_003a-13}

**backgroundColor: color**

Create a new BTextAttributes object resulting in text with the given
background color.

[]{#index-black}

**black**

Create a new BTextAttributes object resulting in black text.

[]{#index-blue}

**blue**

Create a new BTextAttributes object resulting in blue text.

[]{#index-center-2}

**center**

Create a new BTextAttributes object resulting in centered paragraphs.

[]{#index-cyan}

**cyan**

Create a new BTextAttributes object resulting in cyan text.

[]{#index-darkCyan}

**darkCyan**

Create a new BTextAttributes object resulting in dark cyan text.

[]{#index-darkGreen}

**darkGreen**

Create a new BTextAttributes object resulting in dark green text.

[]{#index-darkMagenta}

**darkMagenta**

Create a new BTextAttributes object resulting in dark purple text.

[]{#index-events_003a}

**events: aBTextBindings**

Create a new BTextAttributes object for text that responds to events
according to the callbacks established in aBTextBindings.

[]{#index-font_003a-9}

**font: font**

Create a new BTextAttributes object resulting in text with the given
font. The font can be given as either an X font name or a Blox font
description string.

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

[]{#index-foregroundColor_003a-12}

**foregroundColor: color**

Create a new BTextAttributes object resulting in text with the given
foreground color.

[]{#index-green}

**green**

Create a new BTextAttributes object resulting in green text.

[]{#index-magenta}

**magenta**

Create a new BTextAttributes object resulting in magenta text.

[]{#index-red}

**red**

Create a new BTextAttributes object resulting in red text.

[]{#index-strikeout}

**strikeout**

Create a new BTextAttributes object resulting in struck-out text.

[]{#index-underline}

**underline**

Create a new BTextAttributes object resulting in underlined text.

[]{#index-white}

**white**

Create a new BTextAttributes object resulting in white text.

[]{#index-yellow}

**yellow**

Create a new BTextAttributes object resulting in yellow text.

------------------------------------------------------------------------

::: header
Next:
[BLOX.BTextAttributes-colors](BLOX_002eBTextAttributes_002dcolors.html#BLOX_002eBTextAttributes_002dcolors){accesskey="n"
rel="next"}, Up:
[BLOX.BTextAttributes](BLOX_002eBTextAttributes.html#BLOX_002eBTextAttributes){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
