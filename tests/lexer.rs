use st::parser::lexer::{lexer_rules, tokenize};
use std::{fs::File, io::Read};

#[test]
fn weird_identifier() {
    let input = "foo_bar. foo.bar";
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();
    let exp = vec!["foo_bar", ".", "foo.bar"];
    for (i, e) in exp.iter().enumerate() {
        let x = l[i].clone();
        assert_eq!(&x.raw, e, "{}", &x.position);
    }
}


#[test]
fn assignments_and_blocks() {
    let input =   "| adder1 adder2 |
        adder1 := [:x :y | x + y ].
        adder2 := [:x | adder1 value: 1 value: x].
        ^adder2 value: 2";
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();
    let exp = vec!["|", "adder1", "adder2", "|", 
                            "adder1", ":=", "[", ":", "x", ":", "y", "|", "x", "+", "y", "]", ".", 
                            "adder2", ":=", "[", ":", "x", "|", "adder1", "value:", "1", "value:", "x", "]", ".", 
                            "^", "adder2", "value:", "2"];
    for (i, e) in exp.iter().enumerate() {
        let x = l[i].clone();
        assert_eq!(&x.raw, e, "{}: {}", i, &x.position);
    }
}

#[test]
fn convert_to_string() {
    let input = "([:x |
                            [:y | x + y]]
                            value: 2)
                        value: 3";
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();
    let s2 = format!("{}", l);
    assert_eq!(input, s2);
}

#[test]
fn regular() {
    let input = "([:x | [:y | x + y]] value: 2) value: 3";
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();
    assert_eq!(l.len(), 19);

    let exp = vec![
        "(", "[", ":", "x", "|", "[", ":", "y", "|", "x", "+", "y", "]", "]", "value:", "2", ")",
        "value:", "3",
    ];
    for (i, e) in exp.iter().enumerate() {
        let x = l[i].clone();
        assert_eq!(&x.raw, e, "{}", &x.position);
    }
}

#[test]
fn read_point() {
    let src = {
        let file_path = st::get_smalltalk_file_path("kernel", "Point.st");
        let mut f =
            File::open(&file_path).expect(&format!("Failed to open file: {}", file_path.display()));
        let mut src = String::new();
        f.read_to_string(&mut src).expect("Failed to read file");
        src
    };
    let rules = lexer_rules();
    let l = tokenize(&rules, &src).unwrap();
    let expected = vec![
        "IDENTIFIER",
        "KEYWORD",
        "IDENTIFIER",
        "[",
        "BINARY",
        "IDENTIFIER",
        "IDENTIFIER",
        "BINARY",
        "PRAGMA",
        "PRAGMA",
        "IDENTIFIER",
        "IDENTIFIER",
        "BINARY",
        "IDENTIFIER",
        "[",
        "PRAGMA",
    ];

    for (i, x) in l
        .iter()
        .take(expected.len())
        .map(|t| t.kind.clone())
        .enumerate()
    {
        assert_eq!(x, expected[i]);
    }

    for token in l.iter().filter(|t| t.kind == "CATEGORY") {
        println!("{}: {}", token.position, token.raw);
    }

    for token in l.iter().filter(|t| t.kind == "COMMENT") {
        println!("{}: {}", token.position, token.raw);
    }
}

#[test]
fn test_pragma() {
    let input = r#"    
    <category: 'Language-Data types'>
    <comment: 'I am an abstract class.  My objects represent things that are discrete and 
map to a number line.  My instances can be compared with < and >.'>

   min: aMagnitude [
	"Returns the least object between the receiver and aMagnitude"

	<category: 'misc methods'>
	^self < aMagnitude ifTrue: [self] ifFalse: [aMagnitude]
    ]

    max: aMagnitude [
	"Returns the greatest object between the receiver and aMagnitude"

    <category: 'misc methods'>
	^self > aMagnitude ifTrue: [self] ifFalse: [aMagnitude]
    ]"#;
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();

    let exp = vec![
        "PRAGMA",
        "PRAGMA",
        "KEYWORD",
        "IDENTIFIER",
        "[",
        "PRAGMA",
        "RETURN",
        "IDENTIFIER",
        "BINARY",
        "IDENTIFIER",
        "KEYWORD",
        "[",
        "IDENTIFIER",
        "]",
        "KEYWORD",
        "[",
        "IDENTIFIER",
        "]",
        "]",
        "KEYWORD",
        "IDENTIFIER",
        "[",
        "PRAGMA",
        "RETURN",
        "IDENTIFIER",
        "BINARY",
        "IDENTIFIER",
        "KEYWORD",
        "[",
        "IDENTIFIER",
        "]",
        "KEYWORD",
        "[",
        "IDENTIFIER",
        "]",
        "]",
    ];
    for (i, e) in exp.iter().enumerate() {
        let x = l[i].clone();
        assert_eq!(&x.kind, e, "{}", &x.position);
    }
}

#[test]
fn read_magnitude() {
    let src = {
        let file_path = st::get_smalltalk_file_path("kernel", "Magnitude.st");
        let mut f =
            File::open(&file_path).expect(&format!("Failed to open file: {}", file_path.display()));
        let mut src = String::new();
        f.read_to_string(&mut src).expect("Failed to read file");
        src
    };
    let rules = lexer_rules();
    let _l = tokenize(&rules, &src).unwrap();
}

#[test]
fn test_escaped_quotes_in_strings() {
    // Test the specific case from the user's request
    let input = r#"'%1 [
    "Answer the receiver''s %1. Its default value is %2"
    %1 isNil ifTrue: [ %1 := %2 ].
    ^%1
    ]'"#;
    
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();
    
    // The string should be tokenized as a single STRING token
    assert_eq!(l.len(), 1);
    assert_eq!(l[0].kind, "STRING");
    
    // Verify the content includes the escaped quotes
    let expected_content = r#"'%1 [
    "Answer the receiver''s %1. Its default value is %2"
    %1 isNil ifTrue: [ %1 := %2 ].
    ^%1
    ]'"#;
    assert_eq!(l[0].raw, expected_content);
}

#[test]
fn test_simple_escaped_quotes() {
    let input = r#"'don''t'"#;
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();
    
    assert_eq!(l.len(), 1);
    assert_eq!(l[0].kind, "STRING");
    assert_eq!(l[0].raw, "'don''t'");
}

#[test]
fn test_multiple_escaped_quotes() {
    let input = r#"'He said ''Hello'' and ''Goodbye'''"#;
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();
    
    assert_eq!(l.len(), 1);
    assert_eq!(l[0].kind, "STRING");
    assert_eq!(l[0].raw, "'He said ''Hello'' and ''Goodbye'''");
}

#[test]
fn test_comments_with_escaped_quotes() {
    let input = r#""This is a comment with ""escaped quotes"" inside""#;
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();
    
    // Comments should be skipped, so no tokens
    assert_eq!(l.len(), 0);
}

#[test]
fn test_smalltalk_method_template() {
    // Test the actual Behavior.st method template from the user's request
    let input = r#"createGetMethod: what default: value [
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
]"#;
    
    let rules = lexer_rules();
    let l = tokenize(&rules, input).unwrap();
    
    // Verify that the string containing escaped quotes is parsed correctly
    let string_tokens: Vec<_> = l.iter()
        .filter(|token| token.kind == "STRING")
        .collect();
    
    // Should have exactly one string token (the template string)
    assert_eq!(string_tokens.len(), 1);
    
    let template_string = &string_tokens[0].raw;
    assert!(template_string.contains("receiver''s"));
    assert!(template_string.starts_with("'%1 ["));
    assert!(template_string.ends_with("    ]'"));
}
