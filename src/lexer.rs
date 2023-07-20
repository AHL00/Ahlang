#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal<'a> {
    Int(&'a str),
    Float(&'a str),
}

// TODO: Move to operator.rs maybe lib.rs
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    // Infix operators
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    CARET,

    // Unary operators
    NOT,
    IDENTITY,
    NEGATION,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    ILLEGAL,
    // { TODO: Add line and column number to token to use in error messages
    //     line: usize,
    //     col: usize,
    // },
    EOF,

    // Identifiers / literals / funcs
    IDENT(&'a str),
    LITERAL(Literal<'a>),
    FUNC(&'a str),

    TYPE(&'a str),
    
    // Built-in functions
    BUILT_IN_FUNC(&'a str),

    // Operators
    ASSIGN,
    OPERATOR(Operator),

    // Delimiters
    COMMA,
    SEMICOLON,
    COLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LSQUARE,
    RSQUARE,

    // Keywords
    FN,
    LET,
}

pub struct Lexer<'a> {
    input: &'a str,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            tokens: Vec::new(),
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token<'a>> {
        &self.tokens
    }

    pub fn tokenize(&mut self) -> Result<&Vec<Token<'a>>, String> {
        // set token literal to slice of input str
        let mut char_iter = self.input.char_indices().peekable();

        #[allow(unused_labels)]
        'tokenizer: loop {
            let i = char_iter.next();

            if i.is_none() {
                break;
            }

            let (i, current_char) = i.unwrap();

            if current_char.is_whitespace() {
                continue;
            }

            if current_char == '/' {
                let peek = char_iter.peek();
                if peek.is_none() {
                    return Err("Unexpected EOF".to_string());
                }
                if peek.unwrap().1 == '/' {
                    // skip comment
                    let mut next_char = char_iter.next();

                    while next_char.is_some() {
                        let (i, c) = next_char.unwrap();

                        if c == '\n' {
                            break;
                        }

                        next_char = char_iter.next();
                    }

                    continue;
                }
            }

            if current_char.is_digit(10) {
                // check for int/float literal
                let mut next_char_digit = true;
                let mut char_count = 1;
                let mut float = false;
                let mut decimal_included = false;

                while next_char_digit {
                    let next_char = char_iter.peek();

                    if next_char.is_none() {
                        break;
                    }

                    let next_char = next_char.unwrap().1;

                    if next_char == '.' {
                        if decimal_included {
                            return Err(format!(
                                "Invalid float literal: {}",
                                &self.input[i..i + char_count + 1]
                            ));
                        }

                        float = true;
                        decimal_included = true;

                        char_count += 1;
                        char_iter.next();
                    } else if next_char == '_' {
                        char_count += 1;
                        char_iter.next();
                    } else if next_char.is_digit(10) {
                        char_count += 1;
                        char_iter.next();
                    } else {
                        next_char_digit = false;
                    }
                }

                if float {
                    self.tokens.push(Token::LITERAL(Literal::Float(
                        &self.input[i..i + char_count],
                    )));
                } else {
                    self.tokens.push(Token::LITERAL(Literal::Int(
                        &self.input[i..i + char_count],
                    )));
                }

                continue;
            } else if current_char.is_alphabetic() {
                // match keyword or identifier
                // identifiers can not start with a digit
                let mut next_char_alhpanum = true;
                let mut char_count = 1;

                while next_char_alhpanum {
                    let next_char = char_iter.peek();

                    if next_char.is_none() {
                        break;
                    }

                    let next_char = next_char.unwrap().1;

                    if next_char.is_alphanumeric() {
                        char_count += 1;
                        char_iter.next();
                    } else {
                        next_char_alhpanum = false;
                    }
                }

                let literal = &self.input[i..i + char_count];

                if ahlang::KEYWORDS.contains(&literal) {
                    match literal {
                        "fn" => self.tokens.push(Token::FN),
                        "let" => self.tokens.push(Token::LET),
                        _ => {
                            return Err(format!("Unknown keyword: {}", literal));
                        }
                    }

                    continue;
                } else if ahlang::BUILT_IN_TYPES.contains_key(&literal) {
                    self.tokens.push(Token::TYPE(literal));
                    continue;
                } else if ahlang::BUILT_IN_FUNCS.contains(&literal) {
                    self.tokens.push(Token::BUILT_IN_FUNC(literal));
                    continue;
                } else {
                    self.tokens.push(Token::IDENT(literal));
                    continue;
                }
            }

            match current_char {
                '=' => self.tokens.push(Token::ASSIGN),
                '+' => {
                    if self.is_prefix() {
                        self.tokens.push(Token::OPERATOR(Operator::IDENTITY));
                    } else {
                        self.tokens.push(Token::OPERATOR(Operator::PLUS));
                    }
                },
                '-' => {
                    if self.is_prefix() {
                        self.tokens.push(Token::OPERATOR(Operator::NEGATION));
                    } else {
                        self.tokens.push(Token::OPERATOR(Operator::MINUS));
                    }
                }
                '*' => self.tokens.push(Token::OPERATOR(Operator::ASTERISK)),
                '/' => self.tokens.push(Token::OPERATOR(Operator::SLASH)),
                '!' => {
                    self.tokens.push(Token::OPERATOR(Operator::NOT));
                }
                '^' => self.tokens.push(Token::OPERATOR(Operator::CARET)),
                ',' => self.tokens.push(Token::COMMA),
                ';' => self.tokens.push(Token::SEMICOLON),
                ':' => self.tokens.push(Token::COLON),
                '(' => self.tokens.push(Token::LPAREN),
                ')' => self.tokens.push(Token::RPAREN),
                '{' => self.tokens.push(Token::LBRACE),
                '}' => self.tokens.push(Token::RBRACE),
                '[' => self.tokens.push(Token::LSQUARE),
                ']' => self.tokens.push(Token::RSQUARE),
                _ => self.tokens.push(Token::ILLEGAL),
            }
        }

        self.tokens.push(Token::EOF);

        Ok(&self.tokens)
    }

    fn is_prefix(&self) -> bool {
        let last = self.tokens.last();
        if last.is_none()
            || !matches!(
                last.unwrap(),
                Token::LITERAL(_) | Token::IDENT(_) | Token::RPAREN | Token::RSQUARE | Token::RBRACE | Token::BUILT_IN_FUNC(_) | Token::FUNC(_)
            )
        {
            return true;
        } else {
            return false;
        }
    }
}
