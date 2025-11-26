[]{#BLOX_002eBPolyline_002daccessing}

::: header
Up:
[BLOX.BPolyline](BLOX_002eBPolyline.html#BLOX_002eBPolyline){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBPolyline_003a-accessing}

#### 1.32.1 BLOX.BPolyline: accessing {#blox.bpolyline-accessing .subsection}

[]{#index-boundingBox-3}

**boundingBox**

Answer 'boundingBox'.

[]{#index-cap-1}

**cap**

Answer the way in which caps are to be drawn at the endpoints of the
line.

This option is only available for open polylines. If you want to set it
for a closed polylines, draw an open one on top of it.

[]{#index-cap_003a-1} []{#index-butt-2} []{#index-projecting-2}
[]{#index-round-2}

**cap: aSymbol**

Set the way in which caps are to be drawn at the endpoints of the line.
aSymbol may be #butt (the default), #projecting, or #round).

This option is only available for open polylines. If you want to set it
for a closed polylines, draw an open one on top of it.

[]{#index-closed}

**closed**

Answer whether the polyline is an open or a closed one.

[]{#index-closed_003a}

**closed: aBoolean**

Set whether the polyline is an open or a closed one. This option may be
set only once.

[]{#index-join}

**join**

Answer the way in which joints are to be drawn at the vertices of the
line.

This option is only available for open polylines. If you want to set it
for a closed polylines, draw an open one on top of it.

[]{#index-join_003a} []{#index-bevel} []{#index-miter}
[]{#index-round-3}

**join: aSymbol**

Answer the way in which joints are to be drawn at the vertices of the
line. aSymbol can be #bevel, #miter (the default) or #round.

This option is only available for open polylines. If you want to set it
for a closed polylines, draw an open one on top of it.

[]{#index-outlineColor}

**outlineColor**

Answer the color with which the outline of the polyline is drawn. This
option is only available for closed polylines.

[]{#index-outlineColor_003a}

**outlineColor: color**

Set the color with which the outline of the polyline is drawn. This
option is only available for closed polylines.

[]{#index-points}

**points**

Answer the points that are vertices of the polyline.

[]{#index-points_003a} []{#index-create-11} []{#index-redraw-10}

**points: arrayOfPointsOrArrays**

Set the points that are vertices of the polyline. Each of the items of
arrayOfPointsOrArrays can be a Point or a two-element Array. Note that
no changes take place until you invoke the #create (if the object has
not been inserted in the canvas yet) or the #redraw method.

[]{#index-width-1}

**width**

Answer the width with which the polyline (or its outline if it is a
closed one) is drawn.

[]{#index-width_003a-1}

**width: pixels**

Set the width with which the polyline (or its outline if it is a closed
one) is drawn.

------------------------------------------------------------------------

::: header
Up:
[BLOX.BPolyline](BLOX_002eBPolyline.html#BLOX_002eBPolyline){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
