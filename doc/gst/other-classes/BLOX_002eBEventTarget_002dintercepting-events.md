[]{#BLOX_002eBEventTarget_002dintercepting-events}

::: header
Up:
[BLOX.BEventTarget](BLOX_002eBEventTarget.html#BLOX_002eBEventTarget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::

------------------------------------------------------------------------

[]{#BLOX_002eBEventTarget_003a-intercepting-events}

#### 1.19.1 BLOX.BEventTarget: intercepting events {#blox.beventtarget-intercepting-events .subsection}

[]{#index-addEventSet_003a}

**addEventSet: aBEventSetSublass**

Add to the receiver the event handlers implemented by an instance of
aBEventSetSubclass. Answer the new instance of aBEventSetSublass.

[]{#index-onAsciiKeyEventSend_003ato_003a}

**onAsciiKeyEventSend: aSelector to: anObject**

When an ASCII key is pressed and the receiver has the focus, send the
1-argument message identified by aSelector to anObject, passing to it a
Character.

[]{#index-onDestroySend_003ato_003a}

**onDestroySend: aSelector to: anObject**

When the receiver is destroyed, send the unary message identified by
aSelector to anObject.

[]{#index-onFocusEnterEventSend_003ato_003a}

**onFocusEnterEventSend: aSelector to: anObject**

When the focus enters the receiver, send the unary message identified by
aSelector to anObject.

[]{#index-onFocusLeaveEventSend_003ato_003a}

**onFocusLeaveEventSend: aSelector to: anObject**

When the focus leaves the receiver, send the unary message identified by
aSelector to anObject.

[]{#index-onKeyEvent_003asend_003ato_003a}

**onKeyEvent: key send: aSelector to: anObject**

When the given key is pressed and the receiver has the focus, send the
unary message identified by aSelector to anObject. Examples for key are:
'Ctrl-1', 'Alt-X', 'Meta-plus', 'enter'. The last two cases include
example of special key identifiers; these include: 'backslash',
'exclam', 'quotedbl', 'dollar', 'asterisk', 'less', 'greater',
'asciicircum' (caret), 'question', 'equal', 'parenleft', 'parenright',
'colon', 'semicolon', 'bar' (pipe sign), 'underscore', 'percent',
'minus', 'plus', 'BackSpace', 'Delete', 'Insert', 'Return', 'End',
'Home', 'Prior' (Pgup), 'Next' (Pgdn), 'F1'..'F24', 'Caps_Lock',
'Num_Lock', 'Tab', 'Left', 'Right', 'Up', 'Down'. There are in addition
four special identifiers which map to platform-specific keys: '\<Cut\>',
'\<Copy\>', '\<Paste\>', '\<Clear\>' (all with the angular brackets!).

[]{#index-onKeyEventSend_003ato_003a}
[]{#index-onKeyEvent_003asend_003ato_003a-1} []{#index-eventTest}

**onKeyEventSend: aSelector to: anObject**

When a key is pressed and the receiver has the focus, send the
1-argument message identified by aSelector to anObject. The pressed key
will be passed as a String parameter; some of the keys will send special
key identifiers such as those explained in the documentation for
#onKeyEvent:send:to: Look at the #eventTest test program in the
BloxTestSuite to find out the parameters passed to such an event
procedure

[]{#index-onKeyUpEventSend_003ato_003a}
[]{#index-onKeyEvent_003asend_003ato_003a-2} []{#index-eventTest-1}

**onKeyUpEventSend: aSelector to: anObject**

When a key has been released and the receiver has the focus, send the
1-argument message identified by aSelector to anObject. The released key
will be passed as a String parameter; some of the keys will send special
key identifiers such as those explained in the documentation for
#onKeyEvent:send:to: Look at the #eventTest test program in the
BloxTestSuite to find out the parameters passed to such an event
procedure

[]{#index-onMouseDoubleEvent_003asend_003ato_003a}

**onMouseDoubleEvent: button send: aSelector to: anObject**

When the given button is double-clicked on the mouse, send the
1-argument message identified by aSelector to anObject. The mouse
position will be passed as a Point.

[]{#index-onMouseDoubleEventSend_003ato_003a}

**onMouseDoubleEventSend: aSelector to: anObject**

When a button is double-clicked on the mouse, send the 2-argument
message identified by aSelector to anObject. The mouse position will be
passed as a Point in the first parameter, the button number will be
passed as an Integer in the second parameter.

[]{#index-onMouseDownEvent_003asend_003ato_003a}

**onMouseDownEvent: button send: aSelector to: anObject**

When the given button is pressed on the mouse, send the 1-argument
message identified by aSelector to anObject. The mouse position will be
passed as a Point.

[]{#index-onMouseDownEventSend_003ato_003a}

**onMouseDownEventSend: aSelector to: anObject**

When a button is pressed on the mouse, send the 2-argument message
identified by aSelector to anObject. The mouse position will be passed
as a Point in the first parameter, the button number will be passed as
an Integer in the second parameter.

[]{#index-onMouseEnterEventSend_003ato_003a}

**onMouseEnterEventSend: aSelector to: anObject**

When the mouse enters the widget, send the unary message identified by
aSelector to anObject.

[]{#index-onMouseLeaveEventSend_003ato_003a}

**onMouseLeaveEventSend: aSelector to: anObject**

When the mouse leaves the widget, send the unary message identified by
aSelector to anObject.

[]{#index-onMouseMoveEvent_003asend_003ato_003a}

**onMouseMoveEvent: button send: aSelector to: anObject**

When the mouse is moved while the given button is pressed on the mouse,
send the 1-argument message identified by aSelector to anObject. The
mouse position will be passed as a Point.

[]{#index-onMouseMoveEventSend_003ato_003a}

**onMouseMoveEventSend: aSelector to: anObject**

When the mouse is moved, send the 1-argument message identified by
aSelector to anObject. The mouse position will be passed as a Point.

[]{#index-onMouseTripleEvent_003asend_003ato_003a}

**onMouseTripleEvent: button send: aSelector to: anObject**

When the given button is triple-clicked on the mouse, send the
1-argument message identified by aSelector to anObject. The mouse
position will be passed as a Point.

[]{#index-onMouseTripleEventSend_003ato_003a}

**onMouseTripleEventSend: aSelector to: anObject**

When a button is triple-clicked on the mouse, send the 2-argument
message identified by aSelector to anObject. The mouse position will be
passed as a Point in the first parameter, the button number will be
passed as an Integer in the second parameter.

[]{#index-onMouseUpEvent_003asend_003ato_003a}

**onMouseUpEvent: button send: aSelector to: anObject**

When the given button is released on the mouse, send the 1-argument
message identified by aSelector to anObject. The mouse position will be
passed as a Point.

[]{#index-onMouseUpEventSend_003ato_003a}

**onMouseUpEventSend: aSelector to: anObject**

When a button is released on the mouse, send the 2-argument message
identified by aSelector to anObject. The mouse position will be passed
as a Point in the first parameter, the button number will be passed as
an Integer in the second parameter.

[]{#index-onResizeSend_003ato_003a}

**onResizeSend: aSelector to: anObject**

When the receiver is resized, send the 1-argument message identified by
aSelector to anObject. The new size will be passed as a Point.

------------------------------------------------------------------------

::: header
Up:
[BLOX.BEventTarget](BLOX_002eBEventTarget.html#BLOX_002eBEventTarget){accesskey="u"
rel="up"}  
\[[Index](Class-index.html#Class-index "Index"){rel="index"}\]
:::
