# Definitions Of These Methods In Classes True And F

definitions of these methods in classes True and False, respectively).
2. However, these expressions play the same role as control structures in other languages.
Alternative Control Structures (Receiver is any Object)
Message Description Notes
 nilBlockifNil: Answer the result of evaluating nilblock if the receiver is nil. Otherwise 
answer the receiver.  
ifNotNilBlockifNotNil: Answer the result of evaluating ifNotNilBlock if the receiver is not nil. 
Otherwise answer nil.  
nilBlock
ifNotNilBlock
ifNil: ifNotNil: Answer the result of evaluating nilBlock if the receiver is nil. Otherwise 
answer the result of evaluating ifNotNilBlock.  
ifNotNilBlock
nilBlock
ifNotNil: ifNil: Same as #ifNil:ifNotNil:  
Iterative Control Structures (receiver is aBlockContext)
Message Description Notes
whileTrue Evaluate the receiver. Continue to evaluate the receiver for so long as the result is true.  
aBlockwhileTrue: Evaluate the receiver. If true, evaluate aBlock and repeat.  
whileFalse Evaluate the receiver. Continue to evaluate the receiver for so long as the result is false.  
aBlockwhileFalse: Evaluate the receiver. If false, evaluate aBlockand repeat.  