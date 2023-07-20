#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal<'a> {
    Int(&'a str),
    Float(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Illegal,
    // { TODO: Add line and column number to token to use in error messages
    //     line: usize,
    //     col: usize,
    // },
    Eof,

    // Identifiers / literals / funcs
    Ident(&'a str),
    Literal(Literal<'a>),
    Func(&'a str),

    Type(&'a str),
    
    // Built-in functions
    built_in_func(&'a str),

    // Operators
    Assign,
    Operator(ahlang::Operator),

    // Delimiters
    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LSquare,
    RSquare,

    // Keywords
    Fn,
    Let,
    If,
    Else,
    Return,
}

pub static KEYWORDS: phf::Map<&'static str, Token> = phf::phf_map! {
    "fn" => Token::Fn,
    "let" => Token::Let,
    "if" => Token::If,
    "else" => Token::Else,
    "return" => Token::Return,
};

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
                    self.tokens.push(Token::Literal(Literal::Float(
                        &self.input[i..i + char_count],
                    )));
                } else {
                    self.tokens.push(Token::Literal(Literal::Int(
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

                if KEYWORDS.contains_key(&literal) {
                    self.tokens.push(KEYWORDS[&literal].clone());
                    continue;
                } else if ahlang::BUILT_IN_TYPES.contains_key(&literal) {
                    self.tokens.push(Token::Type(literal));
                    continue;
                } else if ahlang::BUILT_IN_FUNCS.contains(&literal) {
                    self.tokens.push(Token::built_in_func(literal));
                    continue;
                } else {
                    self.tokens.push(Token::Ident(literal));
                    continue;
                }
            }

            match current_char {
                '=' => self.tokens.push(Token::Assign),
                '+' => {
                    if self.is_prefix() {
                        self.tokens.push(Token::Operator(ahlang::Operator::Identity));
                    } else {
                        self.tokens.push(Token::Operator(ahlang::Operator::Plus));
                    }
                },
                '-' => {
                    if self.is_prefix() {
                        self.tokens.push(Token::Operator(ahlang::Operator::Negation));
                    } else {
                        self.tokens.push(Token::Operator(ahlang::Operator::Minus));
                    }
                }
                '*' => self.tokens.push(Token::Operator(ahlang::Operator::Asterisk)),
                '/' => self.tokens.push(Token::Operator(ahlang::Operator::Slash)),
                '!' => {
                    self.tokens.push(Token::Operator(ahlang::Operator::Not));
                }
                '^' => self.tokens.push(Token::Operator(ahlang::Operator::Caret)),
                ',' => self.tokens.push(Token::Comma),
                ';' => self.tokens.push(Token::Semicolon),
                ':' => self.tokens.push(Token::Colon),
                '(' => self.tokens.push(Token::LParen),
                ')' => self.tokens.push(Token::RParen),
                '{' => self.tokens.push(Token::LBrace),
                '}' => self.tokens.push(Token::RBrace),
                '[' => self.tokens.push(Token::LSquare),
                ']' => self.tokens.push(Token::RSquare),
                _ => self.tokens.push(Token::Illegal),
            }
        }

        self.tokens.push(Token::Eof);

        Ok(&self.tokens)
    }

    fn is_prefix(&self) -> bool {
        let last = self.tokens.last();
        if last.is_none()
            || !matches!(
                last.unwrap(),
                Token::Literal(_) | Token::Ident(_) | Token::RParen | Token::RSquare | Token::RBrace | Token::built_in_func(_) | Token::Func(_)
            )
        {
            return true;
        } else {
            return false;
        }
    }
}
