# Evaluation Messages For Blockcontext

Evaluation Messages for BlockContext
Message Description Notes
value Evaluate the block represented by the receiver and answer the result. 1
 argvalue: Evaluate the block represented by the receiver, passing it the value of the 
argument, arg. 2
anArray
valueWithArguments:
Evaluate the block represented by the receiver. The argument is an Array 
whose elements are the arguments for the block. Signal an error if the length of 
the Array is not the same as the the number of arguments that the block was
expecting.
3
Notes
1. The message , sent to a block, causes the block to be executed and answers the result. The block must require zero arguments.#value
2. The message , causes the block to be executed. The block must require exactly one argument; the corresponding parameter is 
initialized to .
 #value: arg
arg
3. Squeak also recognizes ,  and . If you have a block with more than 
four parameters, you must use
#value:value: #value:value:value: #value:value:value:value:
#valueWithArguments: