[]{#Point_002dpoint-functions}

::: header
Next:
[Point-printing](Point_002dprinting.html#Point_002dprinting){accesskey="n"
rel="next"}, Previous:
[Point-converting](Point_002dconverting.html#Point_002dconverting){accesskey="p"
rel="prev"}, Up: [Point](Point.html#Point){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#Point_003a-point-functions}

#### 1.132.6 Point: point functions {#point-point-functions .subsection}

[]{#index-arcTan-2}

**arcTan**

Answer the angle (measured counterclockwise) between the receiver and a
ray starting in (0, 0) and moving towards (1, 0) - i.e. 3 o'clock

[]{#index-dist_003a}

**dist: aPoint**

Answer the distance between the receiver and aPoint

[]{#index-dotProduct_003a}

**dotProduct: aPoint**

Answer the dot product between the receiver and aPoint

[]{#index-grid_003a}

**grid: aPoint**

Answer a new point whose coordinates are rounded towards the nearest
multiple of aPoint

[]{#index-normal}

**normal**

Rotate the Point 90degrees clockwise and get the unit vector

[]{#index-transpose}

**transpose**

Answer a new point whose coordinates are the receiver's coordinates
exchanged (x becomes y, y becomes x)

[]{#index-truncatedGrid_003a}

**truncatedGrid: aPoint**

Answer a new point whose coordinates are rounded towards -infinity, to a
multiple of grid (which must be a Point)
