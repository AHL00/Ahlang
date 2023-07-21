#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Literal<'a> {
    Int(&'a str),
    Float(&'a str),
    Str(&'a str),
    Char(&'a str),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token<'a> {
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
    Operator(crate::Operator),

    // Delimiters
    Comma,
    Semicolon,
    Colon,
    DoubleQuote,
    SingleQuote,

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

pub(crate) const KEYWORDS: [&str; 7] = ["fn", "let", "if", "else", "return", "true", "false"];
pub(crate) const KEYWORDS_TOKENS: [Token; 7] = [
    Token::Fn,
    Token::Let,
    Token::If,
    Token::Else,
    Token::Return,
    Token::Literal(Literal::Bool(true)),
    Token::Literal(Literal::Bool(false)),
];

#[derive(Debug)]
pub struct Tokens<'a> {
    pub(crate) vec: Vec<Token<'a>>,
}

pub struct Lexer<'a> {
    input: &'a str,
    tokens: Tokens<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new() -> Lexer<'a> {
        Lexer {
            input: "",
            tokens: Tokens { vec: Vec::new() },
        }
    }

    pub(crate) fn get_tokens(&self) -> &Tokens {
        &self.tokens
    }

    pub fn set_input(&mut self, input: &'a str) {
        self.input = input;
    }

    pub fn tokenize(&mut self) -> Result<&Tokens, String> {
        if self.input == "" {
            return Err("Empty input string".to_string());
        }

        self.tokens.vec.clear();

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

            if current_char == '"' {
                // string literal
                let mut next_char = char_iter.next();

                if next_char.is_none() {
                    return Err("Unexpected EOF".to_string());
                }

                let mut char_count = 1;

                while next_char.is_some() {
                    let (i, c) = next_char.unwrap();

                    if c == '"' {
                        break;
                    }

                    char_count += 1;
                    next_char = char_iter.next();
                }

                if next_char.is_none() {
                    return Err("Unexpected EOF".to_string());
                }

                self.tokens.vec.push(Token::Literal(Literal::Str(
                    &self.input[i + 1..i + char_count],
                )));
                continue;
            }

            if current_char == '\'' {
                // char literal
                let mut next_char = char_iter.next();

                if next_char.is_none() {
                    return Err("Unexpected EOF".to_string());
                }

                let mut char_count = 1;

                while next_char.is_some() {
                    let (i, c) = next_char.unwrap();

                    if c == '\'' {
                        break;
                    }

                    char_count += 1;
                    next_char = char_iter.next();
                }

                if next_char.is_none() {
                    return Err("Unexpected EOF".to_string());
                }

                self.tokens.vec.push(Token::Literal(Literal::Char(
                    &self.input[i + 1..i + char_count],
                )));
                continue;
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
                    self.tokens.vec.push(Token::Literal(Literal::Float(
                        &self.input[i..i + char_count],
                    )));
                } else {
                    self.tokens.vec.push(Token::Literal(Literal::Int(
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

                let found_kwd = KEYWORDS.iter().position(|&s| s == literal);
                if found_kwd.is_some() {
                    self.tokens.vec.push(KEYWORDS_TOKENS[found_kwd.unwrap()].clone());
                    continue;
                } else if crate::BUILT_IN_TYPES.contains(&literal) {
                    self.tokens.vec.push(Token::Type(literal));
                    continue;
                } else if crate::BUILT_IN_FUNCS.contains(&literal) {
                    self.tokens.vec.push(Token::built_in_func(literal));
                    continue;
                } else {
                    self.tokens.vec.push(Token::Ident(literal));
                    continue;
                }
            }

            match current_char {
                '=' => self.tokens.vec.push(Token::Assign),
                '+' => {
                    if self.is_prefix() {
                        self.tokens.vec.push(Token::Operator(crate::Operator::Identity));
                    } else {
                        self.tokens.vec.push(Token::Operator(crate::Operator::Plus));
                    }
                },
                '-' => {
                    if self.is_prefix() {
                        self.tokens.vec.push(Token::Operator(crate::Operator::Negation));
                    } else {
                        self.tokens.vec.push(Token::Operator(crate::Operator::Minus));
                    }
                }
                '*' => self.tokens.vec.push(Token::Operator(crate::Operator::Asterisk)),
                '/' => self.tokens.vec.push(Token::Operator(crate::Operator::Slash)),
                '!' => {
                    self.tokens.vec.push(Token::Operator(crate::Operator::Not));
                }
                '^' => self.tokens.vec.push(Token::Operator(crate::Operator::Caret)),
                ',' => self.tokens.vec.push(Token::Comma),
                ';' => self.tokens.vec.push(Token::Semicolon),
                ':' => self.tokens.vec.push(Token::Colon),
                '(' => self.tokens.vec.push(Token::LParen),
                ')' => self.tokens.vec.push(Token::RParen),
                '{' => self.tokens.vec.push(Token::LBrace),
                '}' => self.tokens.vec.push(Token::RBrace),
                '[' => self.tokens.vec.push(Token::LSquare),
                ']' => self.tokens.vec.push(Token::RSquare),
                _ => self.tokens.vec.push(Token::Illegal),
            }
        }

        self.tokens.vec.push(Token::Eof);

        Ok(&self.tokens)
    }

    fn is_prefix(&self) -> bool {
        let last = self.tokens.vec.last();
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
