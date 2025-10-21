# Superclass Variablesubclass  #Nameofclass

SuperClass variableSubclass: #NameOfClass
    instanceVariableNames: 'instVarName1 instVarName2'
    classVariableNames: 'ClassVarName1 ClassVarName2'
    poolDictionaries: ''
    category: 'Major-Minor'
SuperClass variableByteSubclass: #NameOfClass
    instanceVariableNames: 'instVarName1 instVarName2'
    classVariableNames: 'ClassVarName1 ClassVarName2'
    poolDictionaries: ''
    category: 'Major-Minor'
SuperClass variableWordSubclass: #NameOfClass
    instanceVariableNames: 'instVarName1 instVarName2'
    classVariableNames: 'ClassVarName1 ClassVarName2'
    poolDictionaries: ''
    category: 'Major-Minor'
Method Definition
All methods answer a value; there is an implicit at the end of every method to make sure that this is the case. Here is an examp le (from 
class String).
^ self 
"Answer the number of lines represented by the receiver, where every
     cr adds one line."
    | cr count |
    cr  Character cr.
    count  1  min: self size.
    self do:
        [:c | c == cr ifTrue: [count  count + 1]].
    ^ count
lllliiiinnnneeeeCCCCoooouuuunnnntttt