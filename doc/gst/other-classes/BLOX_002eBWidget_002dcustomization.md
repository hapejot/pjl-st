[]{#BLOX_002eBWidget_002dcustomization}

::: header
Next: [BLOX.BWidget-geometry
management](BLOX_002eBWidget_002dgeometry-management.html#BLOX_002eBWidget_002dgeometry-management){accesskey="n"
rel="next"}, Previous:
[BLOX.BWidget-accessing](BLOX_002eBWidget_002daccessing.html#BLOX_002eBWidget_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BWidget](BLOX_002eBWidget.html#BLOX_002eBWidget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBWidget_003a-customization}

#### 1.49.3 BLOX.BWidget: customization {#blox.bwidget-customization .subsection}

[]{#index-addChild_003a-3}

**addChild: child**

The widget identified by child has been added to the receiver. This
method is public not because you can call it, but because it can be
useful to override it, not forgetting the call to basicAddChild, to
perform some initialization on the children just added. Answer the new
child.

[]{#index-create-2}

**create**

Make the receiver able to respond to its widget protocol. This method is
public not because you can call it, but because it can be useful to
override it, not forgetting the call to super, to perform some
initialization on the primitive widget just created; for an example of
this, see the implementation of BButtonLike.

[]{#index-initialize_003a-2} []{#index-new_003a-11}

**initialize: parentWidget**

This is called by #new: to initialize the widget (as the name says\...).
The default implementation calls all the other methods in the
'customization' protocol and some private ones that take care of making
the receiver's status consistent, so you should usually call it instead
of doing everything by hand. This method is public not because you can
call it, but because it might be useful to override it. Always answer
the receiver.

[]{#index-setInitialSize} []{#index-initialize_003a-4}

**setInitialSize**

This is called by #initialize: to set the widget's initial size. The
whole area is occupied by default. This method is public not because you
can call it, but because it can be useful to override it.
