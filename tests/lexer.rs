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
