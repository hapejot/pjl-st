[]{#BLOX_002eBExtended}

::: header
Next: [BLOX.BForm](BLOX_002eBForm.html#BLOX_002eBForm){accesskey="n"
rel="next"}, Previous:
[BLOX.BEventTarget](BLOX_002eBEventTarget.html#BLOX_002eBEventTarget){accesskey="p"
rel="prev"}, Up: [BLOX
package](BLOX-package.html#BLOX-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBExtended-1}

### 1.20 BLOX.BExtended {#blox.bextended .section}

[]{#index-BLOX_002eBExtended}

**Defined in namespace BLOX**\
**Superclass: BLOX.BWidget**\
**Category: Graphics-Windows**

:   Just like Gui, I serve as a base for complex objects which expose an
    individual protocol but internally use a Blox widget for creating
    their user interface. Unlike Gui, however, the instances of my
    subclasses understand the standard widget protocol. Just override my
    newPrimitive method to return another widget, and you'll get a class
    which interacts with the user like that widget (a list box, a text
    box, or even a label) but exposes a different protocol.

  ---------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [BLOX.BExtended-accessing](BLOX_002eBExtended_002daccessing.html#BLOX_002eBExtended_002daccessing){accesskey="1"}:                    (instance)
  • [BLOX.BExtended-customization](BLOX_002eBExtended_002dcustomization.html#BLOX_002eBExtended_002dcustomization){accesskey="2"}:        (instance)
  ---------------------------------------------------------------------------------------------------------------------------------- ---- ------------
