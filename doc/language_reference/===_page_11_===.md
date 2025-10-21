# === Page 11 ===

=== PAGE 11 ===
Control Structures
Alternative Control Structures (Receiver is Boolean)
Message Description Notes
 alternativeBlockifTrue:
Answer nil if the receiver is false. Signal an Error if the receiver is 
nonBoolean. Otherwise, answer the result of evaluating
alternativeBlock
1,2
 alternativeBlockifFalse:
Answer nil if the receiver is true. Signal an Error if the receiver is 
nonBoolean. Otherwise answer the result of evaluating the argument,
alternativeBlock.
1,2
 trueAlternativeBlock
falseAlternativeBlock
ifTrue: ifFalse: Answer the value of trueAlternativeBlock if the receiver is true. 
Answer the value of falseAlternativeBlock if the receiver is false.
Otherwise, signal an Error.
1,2
falseAlternativeBlock
trueAlternativeBlock
ifFalse: ifTrue: Same as ifTrue:ifFalse:. 1,2
Notes
1. These are not technically control structures, since they can be understood as keyword messages that are sent to boolean objects . (See the 