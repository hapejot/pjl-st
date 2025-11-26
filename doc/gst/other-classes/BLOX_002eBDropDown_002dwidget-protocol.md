[]{#BLOX_002eBDropDown_002dwidget-protocol}

::: header
Previous: [BLOX.BDropDown-list box
accessing](BLOX_002eBDropDown_002dlist-box-accessing.html#BLOX_002eBDropDown_002dlist-box-accessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BDropDown](BLOX_002eBDropDown.html#BLOX_002eBDropDown){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBDropDown_003a-widget-protocol}

#### 1.12.5 BLOX.BDropDown: widget protocol {#blox.bdropdown-widget-protocol .subsection}

[]{#index-dropRectangle}

**dropRectangle**

Answer the rectangle in which the list widget will pop-up. If possible,
this is situated below the drop-down widget's bottom side, but if the
screen space there is not enough it could be above the drop-down
widget's above side. If there is no screen space above as well, we pick
the side where we can offer the greatest number of lines in the pop-up
widget.

[]{#index-dropdown}

**dropdown**

Force the pop-up list widget to be visible.

[]{#index-isDropdownVisible}

**isDropdownVisible**

Answer whether the pop-up widget is visible

[]{#index-toggle}

**toggle**

Toggle the visibility of the pop-up widget.

[]{#index-unmapList}

**unmapList**

Unmap the pop-up widget from the screen, transfer its selected item to
the always visible text widget, and generate a callback.
