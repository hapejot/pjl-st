[]{#BLOX_002eBExtended_002dcustomization}

::: header
Previous:
[BLOX.BExtended-accessing](BLOX_002eBExtended_002daccessing.html#BLOX_002eBExtended_002daccessing){accesskey="p"
rel="prev"}, Up:
[BLOX.BExtended](BLOX_002eBExtended.html#BLOX_002eBExtended){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBExtended_003a-customization}

#### 1.20.2 BLOX.BExtended: customization {#blox.bextended-customization .subsection}

[]{#index-create-1} []{#index-newPrimitive-1} []{#index-create-10}
[]{#index-newPrimitive-2}

**create**

After this method is called (the call is made automatically) the
receiver will be attached to a 'primitive' widget (which can be in turn
another extended widget). This method is public not because you can call
it, but because it can be useful to override it, not forgetting the call
to super (which only calls #newPrimitive and saves the result), to
perform some initialization on the primitive widget just created;
overriding #create is in fact more generic than overriding
#newPrimitive. For an example of this, see the implementation of
BButtonLike.

[]{#index-newPrimitive}

**newPrimitive**

Create and answer a new widget on which the implementation of the
receiver will be based. You should not call this method directly;
instead you must override it in BExtended's subclasses.
