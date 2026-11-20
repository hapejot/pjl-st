## Lexical Analysis for Escaped Quotes in Smalltalk

This document describes how the lexer handles escaped quotes within strings and comments in Smalltalk source code.

### String Literals with Escaped Quotes

In Smalltalk, single quotes are used to delimit string literals. To include a literal single quote within a string, it must be escaped by doubling it (`''`).

#### Examples:

```smalltalk
'simple string'                    → Single STRING token
'don''t'                          → Single STRING token with escaped quote
'He said ''Hello'' world'         → Single STRING token with two escaped quotes
'Answer the receiver''s value'    → Single STRING token (common in method templates)
```

#### Lexer Rule:
```rust
"DEFAULT" | "STRING" = pattern r"'([^']|'')*'";
```

This regex pattern means:
- `'` - Literal single quote (start of string)
- `([^']|'')*` - Zero or more of either:
  - `[^']` - Any character except a single quote
  - `''` - Two consecutive single quotes (escaped quote)
- `'` - Literal single quote (end of string)

### Comments with Escaped Quotes

Smalltalk uses double quotes for comments. Similar to strings, escaped quotes within comments are handled by doubling them (`""`).

#### Examples:

```smalltalk
"This is a comment"                     → Skipped (no tokens)
"A comment with ""quoted"" text"        → Skipped (no tokens) 
"The ""quick"" brown fox"               → Skipped (no tokens)
```

#### Lexer Rule:
```rust
"DEFAULT" | "COMMENT" = pattern "\"([^\"]|\"\")*\"" => |l| l.skip();
```

This pattern works similarly to strings but for double quotes, and the `=> |l| l.skip()` part tells the lexer to ignore these tokens completely.

### Real-world Example

The `createGetMethod:default:` method template from `Behavior.st` demonstrates practical usage:

```smalltalk
createGetMethod: what default: value [
    "Create a method accessing the variable `what', with a default value
     of `value', using lazy initialization"

    <category: 'method dictionary'>
    ^self 
        compile: '%1 [
    "Answer the receiver''s %1. Its default value is %2"
    %1 isNil ifTrue: [ %1 := %2 ].
    ^%1
        ]' 
            % {what. value}
]
```

In this example:
- The comment `"Create a method..."` is skipped by the lexer
- The multi-line string starting with `'%1 [` contains:
  - A comment `"Answer the receiver''s %1..."`
  - An escaped quote in `receiver''s` 
  - Smalltalk code with proper syntax

The lexer correctly identifies this as a single STRING token, preserving all the internal content including the escaped quotes and nested comment structure.

### Testing

The lexer implementation includes comprehensive tests covering:
- Simple escaped quotes: `'don''t'`
- Multiple escaped quotes: `'He said ''Hello'' and ''Goodbye'''`
- Complex multi-line strings with embedded comments
- Method templates from actual Smalltalk code

All tests pass, confirming that the lexer correctly handles these edge cases while maintaining compatibility with existing Smalltalk syntax.