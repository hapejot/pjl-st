# Squeak Smalltalk Grammar (Extracted from Reference)

## Based on: Squeak Language Reference v1.2 by Andrew P. Black

## Lexical Elements

### Comments
```
comment = '"' { any_character | '""' } '"'
```

### Identifiers
```
identifier = letter { letter | digit }
```
- Case-sensitive
- Convention: camelCase for instance variables, PascalCase for classes/globals

### Numbers
```
number = integer | float
integer = decimal_integer | radix_integer
decimal_integer = digit { digit } [ 'e' integer ]
radix_integer = digit { digit } 'r' { digit | letter } [ 'e' integer ]
float = digit { digit } '.' digit { digit } [ 'e' [ '+' | '-' ] integer ]

digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
letter = 'a'...'z' | 'A'...'Z'
```

Examples:
- `1234` (decimal integer)
- `8r177` (octal integer) 
- `16rFF` (hex integer)
- `2r1010` (binary integer)
- `123e2` (integer with exponent)
- `3.14e-10` (float with exponent)

### Characters
```
character = '$' any_character
```

Examples: `$x`, `$3`, `$<`, `$$`

### Strings
```
string = "'" { string_character | "''" } "'"
string_character = any_character_except_single_quote
```

### Symbols
```
symbol = '#' identifier | '#' string | '#' binary_selector
```

### Arrays
```
literal_array = '#(' { literal } ')'
literal = number | character | string | symbol | literal_array
```

## Pseudo-Variables
```
pseudo_variable = 'nil' | 'true' | 'false' | 'self' | 'super' | 'thisContext'
```

- `nil` - singleton instance of UndefinedObject
- `true` - singleton instance of True  
- `false` - singleton instance of False
- `self` - current object (message receiver)
- `super` - same object as self, but method lookup starts in superclass
- `thisContext` - active MethodContext or BlockContext

## Variables
```
variable = identifier | pseudo_variable
```

## Assignment
```
assignment = variable ':=' expression
```

## Messages

### Unary Messages
```
unary_message = unary_selector
unary_selector = identifier
```

Examples: `factorial`, `class`, `size`

### Binary Messages  
```
binary_message = binary_selector unary_object_description
binary_selector = binary_character { binary_character }
binary_character = '+' | '-' | '*' | '/' | '\\' | '=' | '>' | '<' | 
                  '~' | '|' | '&' | '?' | '!' | '@' | '%' | ','
```

Examples: `+`, `-`, `*`, `/`, `=`, `==`, `<=`, `>=`, `@`, `->`, `||`

### Keyword Messages
```
keyword_message = { keyword unary_object_description }
keyword = identifier ':'
```

Examples: `at:`, `at:put:`, `ifTrue:ifFalse:`

## Expressions

### Basic Expression
```
expression = assignment | basic_expression

basic_expression = primary { message }

primary = variable | literal | block | '(' expression ')' | brace_array
```

### Message Chain
```
unary_object_description = primary { unary_message }
binary_object_description = unary_object_description { binary_message }
keyword_object_description = binary_object_description [ keyword_message ]
```

### Cascade Expression  
```
cascade_expression = keyword_object_description { ';' { unary_message | binary_message | keyword_message } }
```

### Block Expression
```
block = '[' [ block_arguments ] [ temporaries ] expression_sequence ']'
block_arguments = { ':' identifier } '|'
temporaries = '|' { identifier } '|'
```

### Expression Sequence
```
expression_sequence = expression { '.' expression } [ '.' ]
```

### Brace Array
```
brace_array = '{' [ expression_sequence ] '}'
```

## Method Definition
```
method_definition = message_pattern [ temporaries ] expression_sequence

message_pattern = unary_pattern | binary_pattern | keyword_pattern

unary_pattern = unary_selector
binary_pattern = binary_selector identifier  
keyword_pattern = { keyword identifier }
```

## Class Definition
```
class_definition = superclass 'subclass:' '#' class_name
                  [ 'instanceVariableNames:' string ]
                  [ 'classVariableNames:' string ]
                  [ 'poolDictionaries:' string ]
                  [ 'category:' string ]
```

## Operator Precedence (Lowest to Highest)
1. Assignment (`:=`)
2. Cascade (`;`)
3. Keyword messages
4. Binary messages  
5. Unary messages
6. Parentheses `()`

## Control Structures (Message-Based)

### Conditionals
```
condition ifTrue: [ true_action ]
condition ifFalse: [ false_action ]
condition ifTrue: [ true_action ] ifFalse: [ false_action ]
```

### Loops
```
[ condition ] whileTrue: [ action ]
[ condition ] whileFalse: [ action ]
collection do: [ :each | action ]
start to: end do: [ :i | action ]
```

### Exception Handling (Squeak-specific)
```
[ risky_code ] on: ExceptionClass do: [ :exception | handler ]
```

## Notes
- All control structures are implemented as messages to blocks and objects
- No reserved keywords except pseudo-variables
- Everything is an object
- Method lookup is dynamic
- Single inheritance with mixin-like traits (in modern Squeak)