[]{#BLOX_002eBPopupWindow_002dgeometry-management}

::: header
Up:
[BLOX.BPopupWindow](BLOX_002eBPopupWindow.html#BLOX_002eBPopupWindow){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBPopupWindow_003a-geometry-management}

#### 1.34.1 BLOX.BPopupWindow: geometry management {#blox.bpopupwindow-geometry-management .subsection}

[]{#index-addChild_003a-2} []{#index-basicAddChild_003a-3}

**addChild: w**

Private - The widget identified by child has been added to the receiver.
This method is public not because you can call it, but because it can be
useful to override it, not forgetting the call to either the superclass
implementation or #basicAddChild:, to perform some initialization on the
children just added. Answer the new child.

[]{#index-child_003aheight_003a-1}

**child: child height: value**

Set the given child's height. This is done by setting its parent
window's (that is, our) height.

[]{#index-child_003aheightOffset_003a-1}

**child: child heightOffset: value**

This method should not be called for instances of this class.

[]{#index-child_003awidth_003a-1}

**child: child width: value**

Set the given child's width. This is done by setting its parent window's
(that is, our) width.

[]{#index-child_003awidthOffset_003a-1}

**child: child widthOffset: value**

This method should not be called for instances of this class.

[]{#index-child_003ax_003a-1}

**child: child x: value**

Set the x coordinate of the given child's top-left corner. This is done
by setting its parent window's (that is, our) x.

[]{#index-child_003axOffset_003a-1}

**child: child xOffset: value**

This method should not be called for instances of this class.

[]{#index-child_003ay_003a-1}

**child: child y: value**

Set the y coordinate of the given child's top-left corner. This is done
by setting its parent window's (that is, our) y.

[]{#index-child_003ayOffset_003a-1}

**child: child yOffset: value**

This method should not be called for instances of this class.

[]{#index-heightChild_003a-1}

**heightChild: child**

Answer the given child's height, which is the height that was imposed on
the popup window.

[]{#index-widthChild_003a-1}

**widthChild: child**

Answer the given child's width in pixels, which is the width that was
imposed on the popup window.

[]{#index-xChild_003a-1}

**xChild: child**

Answer the x coordinate of the given child's top-left corner, which is
desumed by the position of the popup window.

[]{#index-yChild_003a-1}

**yChild: child**

Answer the y coordinate of the given child's top-left corner, which is
desumed by the position of the popup window.

------------------------------------------------------------------------

::: header
Up:
[BLOX.BPopupWindow](BLOX_002eBPopupWindow.html#BLOX_002eBPopupWindow){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
