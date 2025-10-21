# === Page 8 ===

=== PAGE 8 ===
 "A character is any character (even unprintable ones), preceded by a dollar sign"
 "Don't be shy about characters that are digits"
 "or symbols"
 "or even the dollar sign"
$x
$3
$<
$$
Strings (Instances of class String)
a string comprises any sequence of characters, surrounded by single quotes
strings can include the "comment delimiting" character
and strings can include embedded single quote characters by doubling'' them
strings can contain embedded
newline characters
 "and don't forget the empty string"
''
''
''
'
'
''
A string is very much like ("isomorphic to") an array containing characters. Indexing a string answers characters at the corresponding 
position, staring with 1.
Symbols (Instances of class Symbol)
A string preceded by a hash sign is a Symbol
orAnyIdentifierPrefixedWithAHashSign
orAnIdentifierEndingWithAColon:
or:several:identifiers:each:ending:with:a:colon:
-     "A symbol can also be a hash followed by '-' or any special character"
+<  "or a hash followed by any pair of special characters"
#' '
#
#
#
#
#
Symbol is a subclass of String, and understands, in large part, the same messages.
The primary difference between a symbol and a string is that all symbols comprising the same sequence of characters are the same instance. 
Two different string instances can both have the characters 'test one two three', but every symbol having the characters #'test o ne two three' 
is the same instance. This "unique instance" property means that Symbols can be efficiently compared, because equality (=) is the same as 
identity (==).
"Identifier with colon" Symbols ( #a:keyword:selector:) are often referred to as keyword selectors, for reasons that will be made  clear
later.
e.g.,
"Single or dual symbol" Symbols (  #* or #++) are often referred to as binary selectors.e.g.,
The following are permissible special characters: +/*\~<=>@%|&?!
Note that #-- is not a symbol (or a binary selector). On the other hand, #'--'  a symbol (but not a binary selector).is
Constant Arrays (Instances of class Array)
 1 2 3 4 5  "An array of size 5 comprising five Integers (1 to 5)"
 'this' #is $a #'constant' array  "An array of size 5 comprising a String ('this'), a Symbol (#is), a Character ($a) and two Symb ols (#constant and 
#array)."
 1 2 ( 1 #(2) 3 ) 4  "An array of size 4 comprising two Integers (1 and 2), an Array of size 3, and another Integer (4)."
 1 + 2  "An array of size 3 comprising 1, #+, and 2. It is  the singleton array comprising 3."
#( )
#( )
#( )
#( ) not
Constant arrays are constants, and their elements must therefore be constants. "Expressions" are not evaluated, but are generally  parsed as 
sequences of symbols as in the example above.
Constant arrays may contain constant arrays. The hash sign for internal constant arrays is optional.
Identifiers and sequences of characters in constant arrays are treated as symbols; the hash sign for internal symbols is optional.
Arrays are indexed with the first element at index 1.
Assignments
identifier  expression
identifier :=  expression    " := is always a legal alternative to , but the pretty printer uses "
foo 100 factorial
foo bar 1000 factorial
The identifier (whether instance variable, class variable, temporary variable, or otherwise) will thereafter refer to the object answered by the
expression.
The " " glyph can be typed in Squeak by keying the underbar character (shift-hyphen).
Assignments are expressions; they answer the result of evaluating the right-hand-side.
Assignments can be cascaded as indicated above, resulting in the assignment of the same right-hand-side result to each variable.