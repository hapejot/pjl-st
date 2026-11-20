[]{#BLOX_002eBText_002dimages}

::: header
Next: [BLOX.BText-inserting
text](BLOX_002eBText_002dinserting-text.html#BLOX_002eBText_002dinserting-text){accesskey="n"
rel="next"}, Previous: [BLOX.BText-geometry
management](BLOX_002eBText_002dgeometry-management.html#BLOX_002eBText_002dgeometry-management){accesskey="p"
rel="prev"}, Up:
[BLOX.BText](BLOX_002eBText.html#BLOX_002eBText){accesskey="u" rel="up"}
  \[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBText_003a-images}

#### 1.42.6 BLOX.BText: images {#blox.btext-images .subsection}

[]{#index-insertImage_003a} []{#index-registerImage_003a-1}

**insertImage: anObject**

Insert an image where the insertion point currently lies in the widget.
anObject can be a String containing image data (either Base-64 encoded
GIF data, XPM data, or PPM data), or the result or registering an image
with #registerImage:

[]{#index-insertImage_003aat_003a} []{#index-registerImage_003a-2}

**insertImage: anObject at: position**

Insert an image at the given position in the widget. The position is a
Point object in which both coordinates are 1-based: the first line is
line 1, and the first character in the first line is character 1.

anObject can be a String containing image data (either Base-64 encoded
GIF data, XPM data, or PPM data), or the result or registering an image
with #registerImage:

[]{#index-insertImageAtEnd_003a} []{#index-registerImage_003a-3}

**insertImageAtEnd: anObject**

Insert an image at the end of the widgets text. anObject can be a String
containing image data (either Base-64 encoded GIF data, XPM data, or PPM
data), or the result or registering an image with #registerImage:

[]{#index-registerImage_003a} []{#index-registerImage_003a-4}

**registerImage: anObject**

Register an image (whose data is in anObject, a String including Base-64
encoded GIF data, XPM data, or PPM data) to be used in the widget. If
the same image must be used a lot of times, it is better to register it
once and then pass the result of #registerImage: to the image insertion
methods.

Registered image are private within each BText widget. Registering an
image with a widget and using it with another could give unpredictable
results.
