# Pseudo-Variables

Pseudo-Variables
Pseudo-variable Description
nil The singleton instance of Class UndefinedObject
true The singleton instance of Class True
false The singleton instance of Class False
self The current object, that is, the receiver of the current message.
super
As the receiver of a message, refers to the same object as . However, when a message is 
sent to , the search for a suitable method starts in the superclass of the class whose method 
definition contains the word .
super self
super
super
thisContext The active context, that is, the "currently executing" MethodContext or BlockContext.
Pseudo-variables are reserved identifiers that are similar to keywords in other languages.
,  and  are constants.nil true false
,  and  vary dynamically as code is executed.self super thisContext