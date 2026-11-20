[]{#BLOX_002eBlox-class_002devent-dispatching}

::: header
Next: [BLOX.Blox class-instance
creation](BLOX_002eBlox-class_002dinstance-creation.html#BLOX_002eBlox-class_002dinstance-creation){accesskey="n"
rel="next"}, Previous: [BLOX.Blox class-C
call-outs](BLOX_002eBlox-class_002dC-call_002douts.html#BLOX_002eBlox-class_002dC-call_002douts){accesskey="p"
rel="prev"}, Up:
[BLOX.Blox](BLOX_002eBlox.html#BLOX_002eBlox){accesskey="u" rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBlox-class_003a-event-dispatching}

#### 1.26.2 BLOX.Blox class: event dispatching {#blox.blox-class-event-dispatching .subsection}

[]{#index-dispatchEvents} []{#index-terminateMainLoop-1}
[]{#index-dispatchEvents-1}

**dispatchEvents**

If this is the outermost dispatching loop that is started, dispatch
events until the number of calls to #terminateMainLoop balances the
number of calls to #dispatchEvents; return instantly if this is not the
outermost dispatching loop that is started.

[]{#index-dispatchEvents_003a}

**dispatchEvents: mainWindow**

Dispatch some events; return upon destruction of the 'mainWindow' widget
(which can be any kind of BWidget, but will be typically a BWindow).

[]{#index-terminateMainLoop} []{#index-terminateMainLoop-2}
[]{#index-dispatchEvents-2}

**terminateMainLoop**

Terminate the event dispatching loop if this call to #terminateMainLoop
balances the number of calls to #dispatchEvents.

[]{#index-update_003a}

**update: aspect**

Initialize the Tcl and Blox environments; executed automatically on
startup.
