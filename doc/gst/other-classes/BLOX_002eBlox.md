[]{#BLOX_002eBlox}

::: header
Next: [BLOX.BMenu](BLOX_002eBMenu.html#BLOX_002eBMenu){accesskey="n"
rel="next"}, Previous:
[BLOX.BList](BLOX_002eBList.html#BLOX_002eBList){accesskey="p"
rel="prev"}, Up: [BLOX
package](BLOX-package.html#BLOX-package){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBlox-1}

### 1.26 BLOX.Blox {#blox.blox .section}

[]{#index-BLOX_002eBlox}

**Defined in namespace BLOX**\
**Superclass: BLOX.BEventTarget**\
**Category: Graphics-Windows**

:   I am the superclass for every visible user interface object
    (excluding canvas items, which are pretty different). I provide
    common methods and a simple Tcl interface for internal use. In
    addition, I expose class methods that do many interesting
    event-handling things.

    NOTE: some of the methods (notably geometry methods) may not be
    suitable for all Blox subclasses and may be included only for
    backwards compatibility towards 1.1.5 BLOX. You should use geometry
    methods only for subclasses of BWidget.

  ------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
  • [BLOX.Blox class-C call-outs](BLOX_002eBlox-class_002dC-call_002douts.html#BLOX_002eBlox-class_002dC-call_002douts){accesskey="1"}:                  (class)
  • [BLOX.Blox class-event dispatching](BLOX_002eBlox-class_002devent-dispatching.html#BLOX_002eBlox-class_002devent-dispatching){accesskey="2"}:        (class)
  • [BLOX.Blox class-instance creation](BLOX_002eBlox-class_002dinstance-creation.html#BLOX_002eBlox-class_002dinstance-creation){accesskey="3"}:        (class)
  • [BLOX.Blox class-utility](BLOX_002eBlox-class_002dutility.html#BLOX_002eBlox-class_002dutility){accesskey="4"}:                                      (class)
  • [BLOX.Blox-accessing](BLOX_002eBlox_002daccessing.html#BLOX_002eBlox_002daccessing){accesskey="5"}:                                                  (instance)
  • [BLOX.Blox-basic](BLOX_002eBlox_002dbasic.html#BLOX_002eBlox_002dbasic){accesskey="6"}:                                                              (instance)
  • [BLOX.Blox-creating children](BLOX_002eBlox_002dcreating-children.html#BLOX_002eBlox_002dcreating-children){accesskey="7"}:                          (instance)
  • [BLOX.Blox-customization](BLOX_002eBlox_002dcustomization.html#BLOX_002eBlox_002dcustomization){accesskey="8"}:                                      (instance)
  • [BLOX.Blox-widget protocol](BLOX_002eBlox_002dwidget-protocol.html#BLOX_002eBlox_002dwidget-protocol){accesskey="9"}:                                (instance)
  ------------------------------------------------------------------------------------------------------------------------------------------------- ---- ------------
