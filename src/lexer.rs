use crate::Operator;

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
    Built_in_func(&'a str),

    // Operators
    Assign,
    Operator(crate::Operator),
    FatArrow,

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
    Const,
    If,
    Else,
    Return,
}

pub(crate) const KEYWORDS: [&str; 8] = [
    "fn", "let", "const", "if", "else", "return", "true", "false",
];
pub(crate) const KEYWORDS_TOKENS: [Token; 8] = [
    Token::Fn,
    Token::Let,
    Token::Const,
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

impl<'a> std::fmt::Display for Tokens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for i in 0..self.vec.len() {
            s.push_str(&format!("{:3} > {:?}\n", i, self.vec[i]));
        }

        write!(f, "{}", s)
    }
}

// TODO: Bug, identifiers can't start with a digit, can't include _

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

    pub fn get_tokens(&self) -> &Tokens {
        &self.tokens
    }

    pub fn set_input(&mut self, input: &'a str) {
        self.input = input;
        self.reserve_tokens_vec(4)
    }

    /// Optimization: Reserve space for tokens vector
    pub fn reserve_tokens_vec(&mut self, average_token_len: usize) {
        self.tokens
            .vec
            .reserve(self.input.len() / average_token_len);
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
                    self.tokens
                        .vec
                        .push(Token::Literal(Literal::Int(&self.input[i..i + char_count])));
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
                    self.tokens
                        .vec
                        .push(KEYWORDS_TOKENS[found_kwd.unwrap()].clone());
                    continue;
                } else if crate::BUILT_IN_TYPES.contains(&literal) {
                    self.tokens.vec.push(Token::Type(literal));
                    continue;
                } else if crate::BUILT_IN_FUNCS.contains(&literal) {
                    self.tokens.vec.push(Token::Built_in_func(literal));
                    continue;
                } else {
                    self.tokens.vec.push(Token::Ident(literal));
                    continue;
                }
            }

            let peek_opt = char_iter.peek();
            // EOF char
            let mut peek = ' ';

            if peek_opt.is_some() {
                peek = peek_opt.unwrap().1;
            }

            self.tokens.vec.push(match (current_char, peek) {
                ('+', _) => {
                    if self.is_prefix() {
                        Token::Operator(Operator::Identity)
                    } else {
                        Token::Operator(Operator::Plus)
                    }
                }
                ('-', _) => {
                    if self.is_prefix() {
                        Token::Operator(Operator::Negation)
                    } else {
                        Token::Operator(Operator::Minus)
                    }
                }
                ('=', _) => {
                    match peek {
                        '=' => {
                            char_iter.next();
                            Token::Operator(Operator::Equals)
                        },
                        '>' => {
                            char_iter.next();
                            Token::FatArrow
                        }
                        _ => Token::Assign,
                    }
                },
                ('*', _) => Token::Operator(Operator::Asterisk),
                ('/', _) => Token::Operator(Operator::Slash),
                ('!', _) => {
                    match peek {
                        '=' => {
                            char_iter.next();
                            Token::Operator(Operator::NotEqual)
                        },
                        _ => Token::Operator(Operator::Not),
                    }
                },
                ('<', _) => {
                    match peek {
                        '=' => {
                            char_iter.next();
                            Token::Operator(Operator::LessThanEqual)
                        },
                        '<' => {
                            char_iter.next();
                            Token::Operator(Operator::LeftShift)
                        },
                        _ => Token::Operator(Operator::LessThan),
                    }
                },
                ('>', _) => {
                    match peek {
                        '=' => {
                            char_iter.next();
                            Token::Operator(Operator::GreaterThanEqual)
                        },
                        '>' => {
                            char_iter.next();
                            Token::Operator(Operator::RightShift)
                        },
                        _ => Token::Operator(Operator::GreaterThan),
                    }
                },
                ('&', _) => {
                    match peek {
                        '&' => {
                            char_iter.next();
                            Token::Operator(Operator::And)
                        },
                        _ => Token::Operator(Operator::BitwiseAnd),
                    }
                },
                ('|', _) => {
                    match peek {
                        '|' => {
                            char_iter.next();
                            Token::Operator(Operator::Or)
                        },
                        _ => Token::Operator(Operator::BitwiseOr),
                    }
                },
                ('%', _) => Token::Operator(Operator::Modulo),
                ('^', _) => Token::Operator(Operator::Caret),
                (',', _) => Token::Comma,
                (';', _) => Token::Semicolon,
                (':', _) => Token::Colon,
                ('(', _) => Token::LParen,
                (')', _) => Token::RParen,
                ('{', _) => Token::LBrace,
                ('}', _) => Token::RBrace,
                ('[', _) => Token::LSquare,
                (']', _) => Token::RSquare,
                _ => Token::Illegal,
            })
        }

        self.tokens.vec.push(Token::Eof);

        Ok(&self.tokens)
    }

    fn is_prefix(&self) -> bool {
        let last = self.tokens.vec.last();
        if last.is_none()
            || !matches!(
                last.unwrap(),
                Token::Literal(_)
                    | Token::Ident(_)
                    | Token::RParen
                    | Token::RSquare
                    | Token::RBrace
                    | Token::Built_in_func(_)
                    | Token::Func(_)
            )
        {
            return true;
        } else {
            return false;
        }
    }
}
