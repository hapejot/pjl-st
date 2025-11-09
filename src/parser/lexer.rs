use std::{ffi::os_str::Display, ops::Index, rc::Rc, slice::SliceIndex};

use santiago::lexer::{LexerRules, Position};

pub struct Tokens(Vec<Rc<santiago::lexer::Lexeme>>);

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pos = Position { line: 0, column: 0 };
        for l in &self.0 {
            match &**l {
                santiago::lexer::Lexeme {
                    kind,
                    raw,
                    position: Position { line, column },
                } => {
                    if pos.line == 0 {
                        pos = Position {
                            line: *line,
                            column: *column,
                        };
                        write!(f, "{:width$}", "", width = *column - 1)?;
                    }
                    while pos.line < *line {
                        write!(f, "\n")?;
                        pos.line += 1;
                        pos.column = 1;
                    }
                    if pos.column < *column {
                        write!(f, "{:width$}", "", width = *column - pos.column)?;
                    }
                    write!(f, "{}", raw)?;
                    pos.column = *column + raw.len();
                }
            }
        }
        Ok(())
    }
}

impl Index<usize> for Tokens {
    type Output = Rc<santiago::lexer::Lexeme>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Tokens {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Rc<santiago::lexer::Lexeme>> {
        self.0.iter()
    }

    pub fn get(&self, index: usize) -> Option<&Rc<santiago::lexer::Lexeme>> {
        self.0.get(index)
    }
}

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "INT" = pattern r"-?[0-9]+";
        "DEFAULT" | "FLOAT" = pattern r"-?[0-9]+\.[0-9]+";
        "DEFAULT" | "IDENTIFIER" = pattern r"[a-zA-Z_][a-zA-Z_0-9]*";
        "DEFAULT" | "IDENTIFIER" = pattern r"[a-zA-Z_][a-zA-Z_0-9]*[.][a-zA-Z_][a-zA-Z_0-9]*";
        "DEFAULT" | "SYMBOL" = pattern r"#[a-zA-Z_][a-zA-Z_0-9:]*";
        "DEFAULT" | "SYMBOL" = pattern r"#[-%&,*+/<=>?@\\~!|][-%&,*+/<=>?@\\~!|]?";
        "DEFAULT" | "KEYWORD" = pattern r"[a-zA-Z_][a-zA-Z_0-9]*:";
        "DEFAULT" | "STRING" = pattern r"'[^']*'";
        // "DEFAULT" | "CATEGORY" = pattern r"<category: *'[^']*'>" => |lexer| lexer.take_and_map(|s| {
        //     let s = &s["<category:".len()..s.len() - 1].trim();
        //     let s = s.trim_matches('\'');
        //     s.to_string()
        // });
        // "DEFAULT" | "COMMENT" = pattern r"<comment: *'[^']*'>"=> |lexer| lexer.take_and_map(|s| {
        //     let s = &s["<comment:".len()..s.len() - 1].trim();
        //     let s = s.trim_matches('\'');
        //     s.to_string()
        // });
        "DEFAULT" | "PRAGMA" = pattern r"<[a-z]+: *'[^']*'>" => |lexer| lexer.take_and_map(|s| {
            let s = &s[1..s.len() - 1].trim();
            s.to_string()
        });
        "DEFAULT" | "PRAGMA" = pattern r"<[a-z]+:[^>]+>" => |lexer| lexer.take_and_map(|s| {
            let s = &s[1..s.len() - 1].trim();
            s.to_string()
        });
         // "DEFAULT" | "LOCAL" = pattern r":[a-zA-Z_][a-zA-Z_0-9]*";
        "DEFAULT" | "COMMENT" = pattern "\"[^\"]*\"" => |l| l.skip();
        "DEFAULT" | ":" = string ":";
        "DEFAULT" | "END_OF_CHUNK" = string "!";
        "DEFAULT" | "." = string ".";
        "DEFAULT" | "[" = string "[";
        "DEFAULT" | "]" = string "]";
        "DEFAULT" | "(" = string "(";
        "DEFAULT" | ")" = string ")";
        // "DEFAULT" | "|" = string "|";
        "DEFAULT" | "{" = string "{";
        "DEFAULT" | "}" = string "}";
        "DEFAULT" | "#" = string "#";
        "DEFAULT" | "SEMICOLON" = string ";";
        "DEFAULT" | "ASSIGN" = string ":=";
        "DEFAULT" | "ASSIGN" = string "<-";
        "DEFAULT" | "BINARY" = pattern r"[-%&,*+/<=>?@\\~!|][-%&,*+/<=>?@\\~!|]?";
        "DEFAULT" | "CHAR" = pattern r"\$.";
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
        "DEFAULT" | "RETURN" = string "^";
    )
}

pub fn tokenize(rules: &LexerRules, src: &str) -> Result<Tokens, Box<dyn std::error::Error>> {
    let lexemes = santiago::lexer::lex(rules, src).map_err(|x| x.to_string())?;
    Ok(Tokens(lexemes))
}
