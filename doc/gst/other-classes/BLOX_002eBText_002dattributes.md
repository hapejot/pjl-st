[]{#BLOX_002eBText_002dattributes}

::: header
Next: [BLOX.BText-geometry
management](BLOX_002eBText_002dgeometry-management.html#BLOX_002eBText_002dgeometry-management){accesskey="n"
rel="next"}, Previous:
[BLOX.BText-accessing](BLOX_002eBText_002daccessing.html#BLOX_002eBText_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BText](BLOX_002eBText.html#BLOX_002eBText){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBText_003a-attributes}

#### 1.42.4 BLOX.BText: attributes {#blox.btext-attributes .subsection}

[]{#index-insertAtEnd_003aattribute_003a}

**insertAtEnd: aString attribute: attr**

Clear the selection and append aString at the end of the widget. Use the
given attributes to format the text.

[]{#index-insertText_003aattribute_003a}

**insertText: aString attribute: attr**

Insert aString in the widget at the current insertion point, replacing
the currently selected text (if any). Use the given attributes to format
the text.

[]{#index-removeAttributes}

**removeAttributes**

Remove any kind of formatting from the text in the widget

[]{#index-removeAttributesFrom_003ato_003a}

**removeAttributesFrom: aPoint to: endPoint**

Remove any kind of formatting from the text in the widget between the
given endpoints. The two endpoints are Point objects in which both
coordinates are 1-based: the first line is line 1, and the first
character in the first line is character 1.

[]{#index-setAttributes_003afrom_003ato_003a}

**setAttributes: attr from: aPoint to: endPoint**

Add the formatting given by attr to the text in the widget between the
given endpoints. The two endpoints are Point objects in which both
coordinates are 1-based: the first line is line 1, and the first
character in the first line is character 1.
