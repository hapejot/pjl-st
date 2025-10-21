# (  Identifier)   Expressionsequence  Block With Ar

  (: identifier)  | expressionSequence     "block with arguments"
(: identifier)  | | identifier  | expressionSequence      "block with arguments and local variables"
[ ]
[ + ]
[ ++ ] 
 1. 2. 3  "a block which, when evaluated, will answer the value 3"
 object doWithSideEffects. test  "a block which, when evaluated, will send #doWithSideEffects to object, and answer the object test"
 :param  param doSomething  "a block which, when evaluated with a parameter, will answer the result of sending #doSomething to the 
parameter.
[]
[]
[ | ]
A block represents a deferred sequence of actions.
The value of a block expression is an object that can execute the enclosed expressions at a later time, if requested to do so. Thus
 1. 2. 3  ==> [ ] in UndefinedObject>>DoIt
 1. 2. 3  value ==> 3
[]
[]
Language experts will note that blocks are rougly equivalent to lambda-expressions, anonymous functions, or closures.