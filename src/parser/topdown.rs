use std::{collections::HashMap, rc::Rc};

use pjl_static_strings::StringTable;
use santiago::lexer::{Lexeme, lex};
use tracing::{error, instrument, trace};

use crate::{memory::Value, parser::lexer::{Tokens, lexer_rules, tokenize}};

pub fn parse(src: &str) -> Result<Vec<SmalltalkNode>, Box<dyn std::error::Error>> {
    let mut parser = SmalltalkParser::new(src)?;
    Ok(parser.parse_file()?)
}

pub fn parse_eval(src: &str) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
    let mut parser = SmalltalkParser::new(src)?;
    Ok(parser.parse_eval()?)
}

/// Simplified Smalltalk AST node
#[derive(Debug, Clone)]
pub enum SmalltalkNode {
    /// Variable identifier
    Identifier(&'static str),
    /// Symbol literal (#symbol)
    Symbol(&'static str),
    /// Character literal ($a)
    Value(Value),
    /// Array literal #[...]
    Array(Vec<SmalltalkNode>),
    /// Block literal [arg | body]
    Block {
        parameters: Vec<&'static str>,
        temps: Vec<&'static str>,
        body: Box<SmalltalkNode>,
    },
    /// Message expression (receiver selector: arg1 with: arg2)
    MessageInvoke {
        receiver: Box<SmalltalkNode>,
        messages: Vec<SmalltalkNode>,
    },
    Message {
        selector: &'static str,
        arguments: Vec<SmalltalkNode>,
    },
    /// Variable assignment (var := value)
    Assignment {
        variable: &'static str,
        value: Box<SmalltalkNode>,
    },
    /// Sequence of statements (stmt1. stmt2. stmt3) with temporary variables
    Statements(Vec<&'static str>, Vec<SmalltalkNode>),
    /// Return statement (^expression)
    Return(Box<SmalltalkNode>),
    /// Method definition (selector [temporaries] body)
    MethodDefinition {
        classname: &'static str,
        is_static: bool,
        selector: &'static str,
        parameters: Vec<&'static str>,
        temporaries: Vec<&'static str>,
        body: Box<SmalltalkNode>,
    },
    /// Null/nil value
    Nil,
    ClassDefinition {
        name: &'static str,
        attr: Value,
        superclass: Option<&'static str>,
    },
    CascadeReceiver,
}
/// Parser error with location information
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
    pub line: usize,
    pub column: usize,
    pub context: String,
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}\nPosition: {}\nContext:\n{}",
            self.line + 1,   // Make line numbers 1-based
            self.column + 1, // Make column numbers 1-based
            self.message,
            self.position,
            self.context
        )
    }
}

impl std::error::Error for ParseError {}

impl From<ParseError> for String {
    fn from(error: ParseError) -> String {
        error.to_string()
    }
}

pub struct SmalltalkParser {
    tokens: Tokens,
    current_kind: &'static str,
    current_raw: &'static str,
    position: usize,
}

type Token = Rc<Lexeme>;

impl SmalltalkParser {
    pub fn new(src: &str) -> Result<Self, String> {
        let lexer = lexer_rules();
        let lexemes = tokenize(&lexer, src).map_err(|x| x.to_string())?;
        let mut p = SmalltalkParser {
            tokens: lexemes,
            position: 0,
            current_kind: "",
            current_raw: "",
        };
        p.advance();
        Ok(p)
    }

    pub fn current_token(&self) -> &'static str {
        self.current_kind
    }

    pub fn current_value(&self) -> &'static str {
        self.current_raw
    }

    // #[instrument(skip(self))]
    pub fn advance(&mut self) -> &'static str {
        if self.position >= self.tokens.len() {
            self.current_kind = "EOF";
            self.current_raw = "EOF";
            return self.current_kind;
        }
        let t = self.tokens[self.position].clone();
        self.current_kind = StringTable::get(t.kind.as_str());
        self.current_raw = StringTable::get(t.raw.as_str());
        self.position += 1;
        trace!("{}", self.current_kind);
        self.current_kind
    }

    pub fn expect(&mut self, expected: &str) -> Result<(), String> {
        if self.current_token() == expected {
            self.advance();
            Ok(())
        } else {
            Err(self.error_msg(&format!(
                "Expected {:?}, found {:?}",
                expected,
                self.current_token()
            )))
        }
    }

    /// parsing a statement to evaluate
    pub fn parse_eval(&mut self) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        let _pragmas = self.pragmas().unwrap();
        let temporaries = self.parse_temporaries()?;
        let _pragmas = self.pragmas().unwrap();
        let r = self.parse_statements();
        if let Ok(SmalltalkNode::Statements(_, s)) = r {
            return Ok(SmalltalkNode::Statements(temporaries, s));
        }
        r
    }

    /// Create an error message with location information
    fn error_msg(&self, message: &str) -> String {
        let (line, column) = self.get_line_column(self.position);
        let context = self.get_context(5);

        format!(
            "{} at line {}, column {} (position {})\nContext:\n{}",
            message,
            line + 1,
            column + 1,
            self.position,
            context
        )
    }

    /// Parse a sequence of statements separated by dots
    #[instrument(skip(self))]
    fn parse_statements(&mut self) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        let mut statements = Vec::new();

        while !matches!(self.current_token(), "EOF" | "]") {
            if matches!(self.current_token(), "RETURN") {
                self.advance();
                let expr = self.parse_basic_expression()?;
                statements.push(SmalltalkNode::Return(Box::new(expr)));
                break;
            } else {
                let stmt = self.parse_expression()?;
                statements.push(stmt);
            }
            // Optional dot separator
            if matches!(self.current_token(), ".") {
                self.advance();
                // Allow trailing dots
                if matches!(self.current_token(), "EOF" | "]") {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(SmalltalkNode::Statements(vec![], statements))
    }

    /// Parse a single expression (assignment, return, or message expression)
    #[instrument(skip(self))]
    fn parse_expression(&mut self) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        // Check for assignment
        if "IDENTIFIER" == self.current_token() {
            let var_name = self.current_raw;
            let checkpoint = self.position;
            self.advance();

            if matches!(self.current_token(), "ASSIGN") {
                self.advance();
                let value = self.parse_expression()?;
                return Ok(SmalltalkNode::Assignment {
                    variable: var_name,
                    value: Box::new(value),
                });
            } else {
                // Not an assignment, backtrack
                self.position = checkpoint - 1;
                self.advance();
            }
        } else {
            trace!("no assignment");
        }

        self.parse_basic_expression()
    }

    /// Parse a message expression with optional cascading
    #[instrument(skip(self))]
    fn parse_basic_expression(&mut self) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        let receiver = self.parse_primary()?;

        let r = self.parse_messages(receiver.clone())?;
        // Parse message chain
        let rs = self.parse_cascade_message(receiver)?;
        if rs.len() > 0 {
            if let SmalltalkNode::MessageInvoke {
                receiver,
                mut messages,
            } = r
            {
                for x in rs {
                    if let SmalltalkNode::MessageInvoke {
                        receiver: _,
                        messages: mut ms,
                    } = x
                    {
                        messages.append(&mut ms);
                    }
                }
                return Ok(SmalltalkNode::MessageInvoke { receiver, messages });
            } else {
                todo!("Should not reach here in parse_basic_expression")
            }
        }
        Ok(r)
    }

    /// Check if current position starts a message send
    #[allow(dead_code)]
    fn is_message_start(&self) -> bool {
        matches!(self.current_token(), "IDENTIFIER" | "BINARY" | "KEYWORD")
    }

    #[instrument(skip(self))]
    fn parse_cascade_message(
        &mut self,
        receiver: SmalltalkNode,
    ) -> Result<Vec<SmalltalkNode>, Box<dyn std::error::Error>> {
        let mut messages = vec![];
        while matches!(self.current_token(), "SEMICOLON") {
            self.advance(); // consume ';'
            let next_message = self.parse_messages(SmalltalkNode::CascadeReceiver)?;
            messages.push(next_message);
        }
        Ok(messages)
    }

    /// Parse a message send (unary, binary, or keyword)
    #[allow(dead_code)]
    #[instrument(skip(self))]
    fn parse_message_send(
        &mut self,
        receiver: SmalltalkNode,
    ) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        match self.current_token() {
            "IDENTIFIER" => {
                let selector = self.current_raw;
                // Unary message
                self.advance();
                Ok(SmalltalkNode::MessageInvoke {
                    receiver: Box::new(receiver),
                    messages: vec![SmalltalkNode::Message {
                        selector,
                        arguments: vec![],
                    }],
                })
            }
            "BINARY" => {
                let selector = self.current_raw;
                // Binary message
                self.advance();
                let argument = self.parse_primary()?;
                Ok(SmalltalkNode::MessageInvoke {
                    receiver: Box::new(receiver),
                    messages: vec![SmalltalkNode::Message {
                        selector,
                        arguments: vec![argument],
                    }],
                })
            }
            "KEYWORD" => {
                // Keyword message
                let mut selector = self.current_raw.to_string();
                let mut arguments = vec![];

                self.advance();
                arguments.push(self.parse_primary()?);

                // Handle multiple keyword parts
                while "KEYWORD" == self.current_token() {
                    selector.push_str(&self.current_raw);
                    self.advance();
                    arguments.push(self.parse_primary()?);
                }

                Ok(SmalltalkNode::MessageInvoke {
                    receiver: Box::new(receiver),
                    messages: vec![SmalltalkNode::Message {
                        selector: StringTable::get(&selector),
                        arguments,
                    }],
                })
            }
            _ => Err(self.error_msg("Expected message selector").into()),
        }
    }

    /// Parse primary expressions (literals, identifiers, blocks, parentheses)
    #[instrument(skip(self))]
    pub fn parse_primary(&mut self) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        match self.current_token() {
            "IDENTIFIER" => {
                let r = SmalltalkNode::Identifier(self.current_raw);
                self.advance();
                Ok(r)
            }
            "INT" => {
                let v = Value::Integer(self.current_raw.parse::<i64>().unwrap());
                self.advance();
                Ok(SmalltalkNode::Value(v))
            }
            "FLOAT" => {
                let v = Value::Float(self.current_raw.parse::<f64>().unwrap());
                self.advance();
                Ok(SmalltalkNode::Value(v))
            }
            "STRING" => {
                let s = &self.current_raw[1..self.current_raw.len() - 1]; // Remove quotes
                let v = Value::String(StringTable::get(s));
                self.advance();
                Ok(SmalltalkNode::Value(v))
            }
            "SYMBOL" => {
                let r = SmalltalkNode::Symbol(self.current_raw);
                self.advance();
                Ok(r)
            }
            "CHAR" => {
                let r = SmalltalkNode::Value(Value::Character(
                    self.current_raw.chars().nth(1).unwrap(),
                ));
                self.advance();
                Ok(r)
            }
            "(" => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(")")?;
                Ok(expr)
            }
            "[" => self.parse_block(),
            "#" => {
                // Array literal
                self.advance();
                match self.current_token() {
                    "[" => self.parse_array_literal("[", "]"), // byte array
                    "(" => self.parse_array_literal("(", ")"), // regular array
                    _ => todo!(),
                }
            }
            "{" => {
                // array literal with dynmic elements separated by dots
                self.parse_array_literal_dyn()
            }
            _ => {
                for idx in self.position - 5..self.position + 5 {
                    if let Some(token) = self.tokens.get(idx) {
                        error!("Token[{}]: {:?}", idx, token);
                    }
                }

                Err(format!(
                    "Unexpected token in primary: {:}:[{}] in {}",
                    self.current_token(),
                    self.current_raw,
                    self.position
                )
                .into())
            }
        }
    }

    #[instrument(skip(self))]
    fn parse_array_literal_dyn(&mut self) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        self.expect("{")?;
        let mut elements = vec![];
        while self.current_token() != "}" {
            elements.push(self.parse_basic_expression()?);
            if self.current_token() == "." {
                self.advance();
            }
        }
        self.expect("}")?;
        Ok(SmalltalkNode::Array(elements))
    }

    #[instrument(skip(self))]
    fn parse_array_literal(
        &mut self,
        open: &str,
        close: &str,
    ) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        self.expect(open)?;
        let mut elements = vec![];
        while self.current_token() != close {
            elements.push(self.parse_primary()?);
            //     if self.current_token() == "." {
            //         self.advance();
            //     }
        }
        self.expect(close)?;
        Ok(SmalltalkNode::Array(elements))
    }

    /// Parse a block literal [arg | body]
    #[instrument(skip(self))]
    fn parse_block(&mut self) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        self.expect("[")?;

        let mut parameters = Vec::new();

        // Parse parameters if present
        while self.current_token() == ":" {
            self.advance();
            if "IDENTIFIER" == self.current_token() {
                parameters.push(self.current_raw);
                self.advance();
            } else {
                return Err("Expected parameter name after ':'".into());
            }
        }

        let temps = if matches!(self.current_raw, "|") {
            self.advance();
            self.parse_temporaries()?
        } else {
            vec![]
        };

        // Parse body
        let body = if matches!(self.current_token(), "]") {
            SmalltalkNode::Nil
        } else {
            self.parse_statements()?
        };
        if matches!(self.current_raw, "|") {
            return Err(self
                .error_msg("parameter names need to be prefixed with ':'")
                .into());
        }
        self.expect("]")?;

        Ok(SmalltalkNode::Block {
            parameters,
            temps,
            body: Box::new(body),
        })
    }

    pub fn tokens(&self) -> &Tokens {
        &self.tokens
    }

    pub fn position(&self) -> usize {
        self.position
    }

    /// Create a ParseError with location information
    fn error(&self, message: String) -> ParseError {
        let (line, column) = self.get_line_column(self.position);
        let context = self.get_context(5);

        ParseError {
            message,
            position: self.position,
            line,
            column,
            context,
        }
    }

    /// Get line and column for a given position
    fn get_line_column(&self, pos: usize) -> (usize, usize) {
        let t = self.tokens.get(pos).unwrap().clone();
        (t.position.line, t.position.column)
    }

    /// Get context around current position showing tokens
    pub fn get_context(&self, radius: usize) -> String {
        let start = self.position.saturating_sub(radius);
        let end = (self.position + radius).min(self.tokens.len());

        let mut context = String::new();
        for i in start..end {
            let marker = if i == self.position { " --> " } else { "     " };
            if let Some(token) = self.tokens.get(i) {
                context.push_str(&format!("{}{}: {:?}\n", marker, i, token));
            }
        }
        context
    }

    fn parse_keyword_message(
        &mut self,
        receiver: SmalltalkNode,
    ) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        let r = receiver;
        if matches!(self.current_token(), "KEYWORD") {
            let mut new_arguments = vec![];
            let mut selector = String::new();
            while "KEYWORD" == self.current_token() {
                selector.push_str(self.current_raw);
                self.advance();
                let mut arg = self.parse_primary()?;
                while matches!(self.current_token(), "IDENTIFIER") {
                    arg = self.parse_unary_message(arg);
                }
                while matches!(self.current_token(), "BINARY") {
                    arg = self.parse_binary_message(arg)?;
                }
                new_arguments.push(arg);
            }
            Ok(SmalltalkNode::MessageInvoke {
                receiver: Box::new(r),
                messages: vec![SmalltalkNode::Message {
                    selector: StringTable::get(&selector),
                    arguments: new_arguments,
                }],
            })
        } else {
            todo!("Should not reach here in parse_keyword_message")
        }
    }

    fn parse_unary_message(&mut self, r: SmalltalkNode) -> SmalltalkNode {
        if "IDENTIFIER" == self.current_token() {
            let selector = self.current_raw;
            self.advance();
            let r = SmalltalkNode::MessageInvoke {
                receiver: Box::new(r.clone()),
                messages: vec![SmalltalkNode::Message {
                    selector,
                    arguments: vec![],
                }],
            };
            r
        } else {
            todo!("Should not reach here in parse_unary_message")
        }
    }

    fn parse_binary_message(
        &mut self,
        receiver: SmalltalkNode,
    ) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        if "BINARY" == self.current_token() {
            let selector = self.current_raw;
            self.advance();
            let mut argument = self.parse_primary()?;
            while matches!(self.current_token(), "IDENTIFIER") {
                argument = self.parse_unary_message(argument);
            }
            let r = SmalltalkNode::MessageInvoke {
                receiver: Box::new(receiver),
                messages: vec![SmalltalkNode::Message {
                    selector,
                    arguments: vec![argument],
                }],
            };
            Ok(r)
        } else {
            todo!("Should not reach here in parse_binary_message")
        }
    }

    fn parse_messages(
        &mut self,
        receiver: SmalltalkNode,
    ) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        let mut r = receiver;
        match self.current_token() {
            "IDENTIFIER" => {
                while matches!(self.current_token(), "IDENTIFIER") {
                    r = self.parse_unary_message(r);
                }
                while matches!(self.current_token(), "BINARY") {
                    r = self.parse_binary_message(r)?;
                }
                if matches!(self.current_token(), "KEYWORD") {
                    r = self.parse_keyword_message(r)?;
                }
                Ok(r)
            }
            "BINARY" => {
                while matches!(self.current_token(), "BINARY") {
                    r = self.parse_binary_message(r)?;
                }
                if matches!(self.current_token(), "KEYWORD") {
                    r = self.parse_keyword_message(r)?;
                }
                Ok(r)
            }
            "KEYWORD" => {
                r = self.parse_keyword_message(r)?;
                Ok(r)
            }
            _ => return Ok(r),
        }
    }

    /// Parse a method definition
    /// Syntax: selector [parameters] [| temporaries |] body
    pub fn parse_method_definition(&mut self) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        // Parse method pattern (selector and parameters)
        let (selector, parameters) = self.parse_method_pattern()?;

        // Parse temporaries if present
        let temporaries = self.parse_temporaries()?;

        // Parse method body
        let body = self.parse_statements()?;

        Ok(SmalltalkNode::MethodDefinition {
            selector,
            parameters,
            temporaries,
            body: Box::new(body),
            classname: "<classname>",
            is_static: false,
        })
    }

    /// Parse method pattern (selector and parameters)
    /// Handles unary, binary, and keyword methods
    pub fn parse_method_pattern(&mut self) -> Result<(&'static str, Vec<&'static str>), String> {
        match self.current_token() {
            "IDENTIFIER" => {
                let selector = self.current_raw;
                self.advance();
                Ok((selector, vec![]))
            }
            "KEYWORD" => {
                // Handle keyword method that starts with keyword token
                let mut selector = self.current_raw.to_string();
                let mut parameters = vec![];
                self.advance();

                // Parse first parameter
                if "IDENTIFIER" == self.current_token() {
                    parameters.push(self.current_raw);
                    self.advance();
                } else {
                    return Err(self.error_msg("Expected parameter after keyword"));
                }

                // Parse additional keyword parts
                while "KEYWORD" == self.current_token() {
                    selector.push_str(self.current_raw);
                    self.advance();

                    // Parse parameter
                    if "IDENTIFIER" == self.current_token() {
                        parameters.push(self.current_raw);
                        self.advance();
                    } else {
                        return Err(self.error_msg("Expected parameter after keyword"));
                    }
                }

                Ok((StringTable::get(&selector), parameters))
            }
            "BINARY" => {
                // Binary method
                let selector = self.current_raw;
                self.advance();

                if "IDENTIFIER" == self.current_token() {
                    let parameter = self.current_raw;
                    self.advance();
                    Ok((selector, vec![parameter]))
                } else {
                    Err(self.error_msg("Expected parameter for binary method"))
                }
            }
            _ => Err(self.error_msg("Expected method selector")),
        }
    }

    /// Parse temporaries declaration: | temp1 temp2 |
    pub fn parse_temporaries(&mut self) -> Result<Vec<&'static str>, ParseError> {
        if self.current_raw == "|" {
            self.advance(); // consume '|'

            let mut temporaries = vec![];

            while "IDENTIFIER" == self.current_token() {
                temporaries.push(self.current_raw);
                self.advance();
            }

            if matches!(self.current_raw, "|") {
                self.advance(); // consume closing '|'
                Ok(temporaries)
            } else {
                Err(self.error("Expected '|' to close temporaries declaration".into()))
            }
        } else {
            Ok(vec![]) // No temporaries
        }
    }

    fn parse_def_expression(&mut self) -> Result<Vec<Value>, String> {
        match self.current_token() {
            "INT" => {
                let v = Value::Integer(self.current_raw.parse::<i64>().unwrap());
                self.advance(); // consume 'method'
                return Ok(vec![v]);
            }
            "[" => {
                let mut values = vec![];
                self.advance(); // consume '['
                while matches!(self.current_token(), "INT") {
                    values.push(Value::Integer(self.current_raw.parse::<i64>().unwrap()));
                    self.advance(); // consume value
                    if matches!(self.current_raw, ",") {
                        self.advance(); // consume ','
                    } else {
                        break;
                    }
                }
                self.expect("]")?;
                return Ok(values);
            }
            x => return Err(format!("expected string {:?}", x)),
        }
    }

    pub(crate) fn parse_file(&mut self) -> Result<Vec<SmalltalkNode>, Box<dyn std::error::Error>> {
        let mut _attributes = None;
        let mut result = vec![];
        while self.current_token() != "EOF" {
            let mut is_static = false;
            match self.current_token() {
                "{" => {
                    println!("parsing attributes");
                    _attributes = Some(self.parse_attributes()?);
                }
                "IDENTIFIER" if self.current_raw == "Class" => {
                    println!("parsing class definition");
                    // Skip class definition header
                    self.advance();
                    let _attr = self.parse_attributes()?;
                    // let name = StringTable::get(attr.get("name").unwrap().as_str().unwrap());
                    let name = "UnnamedClass";
                    result.push(SmalltalkNode::ClassDefinition {
                        name,
                        attr: Value::Undefined,
                        superclass: None,
                    });
                }
                "IDENTIFIER" => {
                    // Parse class name and method pattern: ClassName >> methodPattern
                    let classname = self.current_raw;
                    self.advance();
                    match self.current_raw {
                        ">>" => {
                            self.advance();
                        }
                        "subclass:" => {
                            println!("parsing class {}", classname);
                            self.advance();
                            match self.current_token() {
                                "SYMBOL" => {
                                    let _subclass = self.current_raw;
                                    self.advance();
                                    result.push(SmalltalkNode::ClassDefinition {
                                        name: _subclass,
                                        attr: Value::Undefined,
                                        superclass: Some(classname),
                                    });
                                    continue;
                                }
                                _ => {
                                    return Err(self
                                        .error_msg("Expected superclass name after 'subclass'")
                                        .into());
                                }
                            }
                        }
                        "class" => {
                            is_static = true;
                            self.advance();
                            if ">>" == self.current_raw {
                                self.advance();
                            } else {
                                return Err(self.error_msg("Expected '>>' after 'class'").into());
                            }
                        }
                        _ => {
                            return Err(self.error_msg("Expected '>>' after class name").into());
                        }
                    }
                    let (selector, parameters) = self.parse_pattern()?;
                    println!("parsing method {}", classname);
                    println!("selector: {}, params: {:?}", selector, parameters);
                    // Parse method body (quoted string or block)
                    let body = self.parse_method_body()?;

                    // Parse temporaries if present
                    let temporaries = vec![]; // TODO: Extract from body if needed
                    result.push(SmalltalkNode::MethodDefinition {
                        selector,
                        parameters,
                        temporaries,
                        body: Box::new(body),
                        classname,
                        is_static,
                    });
                }
                _ => {
                    return Err(self
                        .error_msg(&format!(
                            "Unexpected token in file: {:?}",
                            self.current_token()
                        ))
                        .into());
                }
            }
        }

        Ok(result)
    }

    fn parse_attributes(&mut self) -> Result<HashMap<String, Vec<Value>>, String> {
        let mut attributes = HashMap::new();
        self.expect("{")?;
        while "SYMBOL" == self.current_token() {
            let key = self.current_raw.to_string();
            self.advance();
            if ":" == self.current_token() {
                self.advance();
                let x = self.parse_def_expression()?;
                attributes.insert(key, x);
                if "," == self.current_raw {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect("}")?;
        println!("attributes: {:?}", attributes);
        Ok(attributes)
    }

    fn parse_pattern(&mut self) -> Result<(&'static str, Vec<&'static str>), String> {
        // Reuse the existing parse_method_pattern logic
        self.parse_method_pattern()
    }

    /// Parse method metadata from Pharo format: { #category : 'category-name' }
    #[allow(dead_code)]
    fn parse_method_metadata(&mut self) -> Result<String, String> {
        self.expect("{")?;

        // Expect #category
        if "SYMBOL" == self.current_token() {
            if self.current_raw != "#category" {
                return Err(
                    self.error_msg(&format!("Expected #category, found {}", self.current_raw))
                );
            }
            self.advance();
        } else {
            return Err(self.error_msg("Expected #category in method metadata"));
        }

        self.expect(":")?;

        // Parse category string
        let category = if "STRING" == self.current_token() {
            let result = self.current_raw.to_string();
            self.advance();
            result
        } else {
            return Err(self.error_msg("Expected category string"));
        };

        self.expect("}")?;
        Ok(category)
    }

    /// Parse class reference: ClassName [class] >>
    pub fn parse_class_reference(&mut self) -> Result<(String, bool), String> {
        // Parse class name
        let class_name = if "IDENTIFIER" == self.current_token() {
            let result = self.current_raw.to_string();
            self.advance();
            result
        } else {
            return Err(self.error_msg("Expected class name"));
        };

        // Check for 'class' keyword (indicates class-side method)
        let is_class_side = if "IDENTIFIER" == self.current_token() {
            if self.current_raw == "class" {
                self.advance();
                true
            } else {
                false
            }
        } else {
            false
        };

        if self.current_kind == "BINARY" && self.current_raw == ">>" {
            self.advance();
        } else {
            return Err(self.error_msg("Expected '>>' after class reference"));
        }

        Ok((class_name, is_class_side))
    }

    /// Parse method body (Pharo format: actual Smalltalk code)
    fn parse_method_body(&mut self) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        self.expect("[")?;
        let _temporaries = self.parse_temporaries()?;

        println!(
            "Parsing method body starting at token: {:?}",
            self.current_token()
        );
        // Then parse the method body as statements
        let r = self.parse_statements()?;
        self.expect("]")?;
        Ok(r)
    }

    pub(crate) fn parse_class_definition(
        &mut self,
    ) -> Result<SmalltalkNode, Box<dyn std::error::Error>> {
        self.parse_expression()
    }

    pub(crate) fn pragmas(&mut self) -> Result<Vec<String>, String> {
        let mut pragmas = Vec::new();
        while self.current_token() == "PRAGMA" {
            let result = self.current_raw.to_string();
            self.advance();
            trace!("pragma {result}");
            pragmas.push(result);
        }
        return Ok(pragmas);
    }
}
