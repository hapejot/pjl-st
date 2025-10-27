use std::{fs::File, io::Read};

use santiago::lexer::lex;
use st::parser::lexer::lexer_rules;

#[test]
fn regular() {
    let input = "([:x | [:y | x + y]] value: 2) value: 3";
    let rules = lexer_rules();
    let l = lex(&rules, input).unwrap();
    assert_eq!(l.len(), 19);

    let exp = vec![
        "(", "[", ":", "x", "|", "[", ":", "y", "|", "x", "+", "y", "]", "]", "value:", "2", ")",
        "value:", "3",
    ];
    for (i, e) in exp.iter().enumerate() {
        let x = l[i].clone();
        assert_eq!(&x.raw, e);
        println!("{}", &x.position);
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
    let l = lex(&rules, &src).unwrap();
    let expected = vec![
        "IDENTIFIER",
        "KEYWORD",
        "IDENTIFIER",
        "[",
        "|",
        "IDENTIFIER",
        "IDENTIFIER",
        "|",
        "CATEGORY",
        "COMMENT",
        "IDENTIFIER",
        "IDENTIFIER",
        "BINARY",
        "IDENTIFIER",
        "[",
        "CATEGORY"
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
