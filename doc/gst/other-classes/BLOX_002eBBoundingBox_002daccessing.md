[]{#BLOX_002eBBoundingBox_002daccessing}

::: header
Up:
[BLOX.BBoundingBox](BLOX_002eBBoundingBox.html#BLOX_002eBBoundingBox){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBBoundingBox_003a-accessing}

#### 1.3.1 BLOX.BBoundingBox: accessing {#blox.bboundingbox-accessing .subsection}

[]{#index-boundingBox}

**boundingBox**

Answer a Rectangle enclosing all of the receiver

[]{#index-center}

**center**

Answer the center point of the receiver

[]{#index-center_003aextent_003a} []{#index-create-3}
[]{#index-redraw-3}

**center: center extent: extent**

Move the object so that it is centered around the center Point and its
size is given by the extent Point. No changes take place until you
invoke the #create (if the object has not been inserted in the canvas
yet) or the #redraw method.

[]{#index-corner}

**corner**

Answer the Point specifying the lower-right corner of the receiver

[]{#index-corner_003a} []{#index-create-4} []{#index-redraw-4}

**corner: pointOrArray**

Set the Point specifying the lower-right corner of the receiver;
pointOrArray can be a Point or a two-item Array. No changes take place
until you invoke the #create (if the object has not been inserted in the
canvas yet) or the #redraw method.

[]{#index-extent}

**extent**

Answer a Point specifying the size of the receiver

[]{#index-extent_003a} []{#index-create-5} []{#index-redraw-5}

**extent: pointOrArray**

Set the Point specifying the size of the receiver; pointOrArray can be a
Point or a two-item Array. No changes take place until you invoke the
#create (if the object has not been inserted in the canvas yet) or the
#redraw method.

[]{#index-moveBy_003a} []{#index-create-6} []{#index-redraw-6}

**moveBy: pointOrArray**

Move the object by the amount indicated by pointOrArray: that is, its
whole bounding box is shifted by that amount. No changes take place
until you invoke the #create (if the object has not been inserted in the
canvas yet) or the #redraw method.

[]{#index-origin}

**origin**

Answer the Point specifying the top-left corner of the receiver

[]{#index-origin_003a} []{#index-create-7} []{#index-redraw-7}

**origin: pointOrArray**

Set the Point specifying the top-left corner of the receiver;
pointOrArray can be a Point or a two-item Array. No changes take place
until you invoke the #create (if the object has not been inserted in the
canvas yet) or the #redraw method.

[]{#index-origin_003acorner_003a} []{#index-create-8}
[]{#index-redraw-8}

**origin: originPointOrArray corner: cornerPointOrArray**

Set the bounding box of the object, based on a Point specifying the
top-left corner of the receiver and another specifying the bottom-right
corner; the two parameters can both be Points or two-item Arrays. No
changes take place until you invoke the #create (if the object has not
been inserted in the canvas yet) or the #redraw method.

[]{#index-origin_003aextent_003a} []{#index-create-9}
[]{#index-redraw-9}

**origin: originPointOrArray extent: extentPointOrArray**

Set the bounding box of the object, based on a Point specifying the
top-left corner of the receiver and another specifying its size; the two
parameters can both be Points or two-item Arrays. No changes take place
until you invoke the #create (if the object has not been inserted in the
canvas yet) or the #redraw method.

------------------------------------------------------------------------

::: header
Up:
[BLOX.BBoundingBox](BLOX_002eBBoundingBox.html#BLOX_002eBBoundingBox){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
