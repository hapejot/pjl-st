# Answer Expressions

Answer Expressions
answerExpression ::= ^ expression
^ aTemporary
^ 2+3
Inside the body of a method, an answer expression is used to terminate the execution of the method and deliver the expression as the 
method's answer.
Answer expressions inside a nested block expression will terminate the enclosing method.
Class Definition
Ordinary Class Definition
SuperClass subclass: #NameOfClass
    instanceVariableNames: 'instVarName1 instVarName2'
    classVariableNames: 'ClassVarName1 ClassVarName2'
    poolDictionaries: ''
    category: 'Major-Minor'
Variable Class Definition
These forms of class definition are used to create indexable objects, , those like Array, ByteArray and WordArray. They are included here for 
completeness, but are not normally used directly; instead, use an ordinary object with an instance variable whose value is an approriate Array (or 
other collection) object.
i.e.