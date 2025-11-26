[]{#BLOX_002eBText_002dposition-_0026-lines}

::: header
Previous: [BLOX.BText-inserting
text](BLOX_002eBText_002dinserting-text.html#BLOX_002eBText_002dinserting-text){accesskey="p"
rel="prev"}, Up:
[BLOX.BText](BLOX_002eBText.html#BLOX_002eBText){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBText_003a-position-_0026-lines}

#### 1.42.8 BLOX.BText: position & lines {#blox.btext-position-lines .subsection}

[]{#index-charsInLine_003a}

**charsInLine: number**

Answer how many characters are there in the number-th line

[]{#index-currentColumn}

**currentColumn**

Answer the 1-based column number where the insertion point currently
lies.

[]{#index-currentLine}

**currentLine**

Answer the 1-based line number where the insertion point currently lies.

[]{#index-currentPosition}

**currentPosition**

Answer a Point representing where the insertion point currently lies.
Both coordinates in the answer are 1-based: the first line is line 1,
and the first character in the first line is character 1.

[]{#index-currentPosition_003a}

**currentPosition: aPoint**

Move the insertion point to the position given by aPoint. Both
coordinates in aPoint are interpreted as 1-based: the first line is line
1, and the first character in the first line is character 1.

[]{#index-gotoLine_003aend_003a}

**gotoLine: line end: aBoolean**

If aBoolean is true, move the insertion point to the last character of
the line-th line (1 being the first line in the widget); if aBoolean is
false, move it to the start of the line-th line.

[]{#index-indexAt_003a-1}

**indexAt: point**

Answer the position of the character that covers the pixel whose
coordinates within the text's window are given by the supplied Point
object.

[]{#index-lineAt_003a}

**lineAt: number**

Answer the number-th line of text in the widget

[]{#index-numberOfLines}

**numberOfLines**

Answer the number of lines in the widget

[]{#index-selectFrom_003ato_003a-2}

**selectFrom: first to: last**

Select the text between the given endpoints. The two endpoints are Point
objects in which both coordinates are 1-based: the first line is line 1,
and the first character in the first line is character 1.

[]{#index-setToEnd}

**setToEnd**

Move the insertion point to the end of the widget

------------------------------------------------------------------------

::: header
Previous: [BLOX.BText-inserting
text](BLOX_002eBText_002dinserting-text.html#BLOX_002eBText_002dinserting-text){accesskey="p"
rel="prev"}, Up:
[BLOX.BText](BLOX_002eBText.html#BLOX_002eBText){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
