# === Page 12 ===

=== PAGE 12 ===
Enumeration Control Structures (Receiver is anInteger)
Message Description Notes
aBlocktimesRepeat: Evaluate the argument, aBlock, the number of times represented by the receiver.  
 stop  aBlockto: do: Evaluate aBlock with each element of the interval (self to: stop by: 1) as the 
argument.  
 stop  step 
aBlock
to: by: do: Evaluate aBlock with each element of the interval (self to: stop by: step) as the
argument.  
Enumeration Control Structures (Receiver is Collection)
Message Description Notes
aBlockdo: For each element of the receiver, evaluate aBlock with that element as the argument. 1
Note
1. Squeak collections provide a very substantial set of enumeration operators. See the section  on the Classes 
Reference.
EnumeratingCollections
Case Structures (Receiver is any Object)
Message Description Notes
aBlockAssociationCollectioncaseOf:
Answer the evaluated value of the first association in 
aBlockAssociationCollection whose evaluated key equals the 
receiver. If no match is found, signal an Error.
1
aBlockAssociationCollection
 aBlock
caseOf:
otherwise:
Answer the evaluated value of the first association in 
aBlockAssociationCollection whose evaluated key equals the 
receiver. If no match is found, answer the result of evaluating 
aBlock.
1
Note
1. aBlockAssociationCollection is a collection of Associations (key/value pairs).
Example: aSymbol caseOf: {[#a]->[1+1]. ['b' asSymbol]->[2+2]. [#c]->[3+3]}
Expression "Brace" Arrays