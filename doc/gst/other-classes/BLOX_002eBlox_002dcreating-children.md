[]{#BLOX_002eBlox_002dcreating-children}

::: header
Next:
[BLOX.Blox-customization](BLOX_002eBlox_002dcustomization.html#BLOX_002eBlox_002dcustomization){accesskey="n"
rel="next"}, Previous:
[BLOX.Blox-basic](BLOX_002eBlox_002dbasic.html#BLOX_002eBlox_002dbasic){accesskey="p"
rel="prev"}, Up:
[BLOX.Blox](BLOX_002eBlox.html#BLOX_002eBlox){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBlox_003a-creating-children}

#### 1.26.7 BLOX.Blox: creating children {#blox.blox-creating-children .subsection}

[]{#index-make_003a} []{#index-width_003a-5} []{#index-height_003a-2}
[]{#index-backgroundColor_003a-15}

**make: array**

Create children of the receiver. Answer a Dictionary of the children.
Each element of array is an Array including: a string which becomes the
Dictionary's key, a binding like #{Blox.BWindow} identifying the class
name, an array with the parameters to be set (for example #(#width: 50
#height: 30 #backgroundColor: 'blue')), and afterwards the children of
the widget, described as arrays with this same format.

[]{#index-make_003aon_003a} []{#index-make_003a-1}

**make: array on: result**

Private - Create children of the receiver, adding them to result; answer
result. array has the format described in the comment to #make:

[]{#index-makeChild_003aon_003a} []{#index-make_003a-2}

**makeChild: each on: result**

Private - Create a child of the receiver, adding them to result; each is
a single element of the array described in the comment to #make:
