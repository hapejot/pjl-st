use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
            "DEFAULT" | "INT" = pattern r"[0-9]+";
            "DEFAULT" | "IDENTIFIER" = pattern r"[a-zA-Z_][a-zA-Z_0-9]*";
            "DEFAULT" | "SYMBOL" = pattern r"#[a-zA-Z_][a-zA-Z_0-9]*";
            "DEFAULT" | "KEYWORD" = pattern r"[a-zA-Z_][a-zA-Z_0-9]*:";
            "DEFAULT" | "STRING" = pattern r"'[^']*'";
            "DEFAULT" | "CATEGORY" = pattern r"<category: *'[^']*'>" => |lexer| lexer.take_and_map(|s| {
                let s = &s["<category:".len()..s.len() - 1].trim();
                let s = s.trim_matches('\'');
                s.to_string()
            });
            "DEFAULT" | "COMMENT" = pattern r"<comment: *'[^']*'>"=> |lexer| lexer.take_and_map(|s| {
                let s = &s["<comment:".len()..s.len() - 1].trim();
                let s = s.trim_matches('\'');
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
            "DEFAULT" | "|" = string "|";
            "DEFAULT" | "{" = string "{";
            "DEFAULT" | "}" = string "}";
            "DEFAULT" | "#" = string "#";
            "DEFAULT" | "SEMICOLON" = string ";";
            "DEFAULT" | "ASSIGN" = string ":=";
            "DEFAULT" | "ASSIGN" = string "<-";
            "DEFAULT" | "BINARY" = pattern r"[-%&,*+/<=>?@\~!][-%&,*+/<=>?@\~!]?";
            "DEFAULT" | "CHAR" = pattern r"\$.";
            "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
            "DEFAULT" | "RETURN" = string "^";
        )
}
