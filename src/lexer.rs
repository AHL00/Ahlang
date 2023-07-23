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
    Function(&'a str),

    Type(&'a str),

    // Built-in functions
    BuiltInFunc(&'a str),

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
    While,
    Return,
}

pub(crate) const KEYWORDS: [&str; 9] = [
    "fn", "let", "const", "if", "else", "while", "return", "true", "false",
];
pub(crate) const KEYWORDS_TOKENS: [Token; 9] = [
    Token::Fn,
    Token::Let,
    Token::Const,
    Token::If,
    Token::Else,
    Token::While,
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

        'tokenizer: loop {
            // Get the next character of the input
            let i = char_iter.next();

            // If no more characters, end the loop
            if i.is_none() {
                break;
            }

            let (i, current_char) = i.unwrap();

            // Ignore whitespace characters
            if current_char.is_whitespace() {
                continue;
            }

            // Ignore comments (contents after //)
            if current_char == '/' {
                let peek = char_iter.peek();
                if peek.is_none() {
                    return Err("Unexpected EOF".to_string());
                }
                if peek.unwrap().1 == '/' {
                    // Consuming all characters until the end of the line (end of comment)
                    let mut next_char = char_iter.next();

                    while next_char.is_some() {
                        let (_, c) = next_char.unwrap();

                        // If new line character found, end consuming comment
                        if c == '\n' {
                            break;
                        }

                        next_char = char_iter.next();
                    }

                    continue;
                }
            }

            // Handle string literals within ""
            if current_char == '"' {
                let mut next_char = char_iter.next();

                if next_char.is_none() {
                    return Err("Unexpected EOF".to_string());
                }

                let mut char_count = 1;

                // Consume the characters inside the string literal
                while next_char.is_some() {
                    let (_, c) = next_char.unwrap();

                    // If closing double quote found, end of string literal
                    if c == '"' {
                        break;
                    }

                    char_count += 1;
                    next_char = char_iter.next();
                }

                // Add the string literal to the token list
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
                    let (_, c) = next_char.unwrap();

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

            // Peek
            let peek_opt = char_iter.peek();
            let mut peek = '\0';
            if peek_opt.is_some() {
                peek = peek_opt.unwrap().1;
            }

            if current_char.is_digit(10) {
                // If the current character is a digit, we start processing a number literal
                let mut next_char_digit = true;
                let mut char_count = 1;
                let mut float = false;
                let mut decimal_included = false;
            
                while next_char_digit {
                    // We keep processing characters as long as they are digits or special number characters
                    let next_char = char_iter.peek();
            
                    if next_char.is_none() {
                        // If there are no more characters, we break the loop
                        break;
                    }
            
                    let next_char = next_char.unwrap().1;
            
                    if next_char == '.' {
                        // If the character is a dot, we are processing a float
                        if decimal_included {
                            // If we have already included a decimal point, this is an error
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
                        // If the character is an underscore, we ignore it (it's allowed in number literals)
                        char_count += 1;
                        char_iter.next();
                    } else if next_char.is_digit(10) {
                        // If the character is a digit, we continue processing
                        char_count += 1;
                        char_iter.next();
                    } else {
                        // If the character is not a digit, dot, or underscore, we stop processing
                        next_char_digit = false;
                    }
                }
            
                if float {
                    // If we processed a float, we add a float token
                    self.tokens.vec.push(Token::Literal(Literal::Float(
                        &self.input[i..i + char_count],
                    )));
                } else {
                    // If we processed an integer, we add an integer token
                    self.tokens
                        .vec
                        .push(Token::Literal(Literal::Int(&self.input[i..i + char_count])));
                }
            
                continue;
            } else if current_char.is_alphabetic() {
                // If the current character is alphabetic, we start processing an identifier or keyword
                let mut next_char_alhpanum = true;
                let mut char_count = 1;
            
                while next_char_alhpanum {
                    // We keep processing characters as long as they are alphanumeric
                    let next_char = char_iter.peek();
            
                    if next_char.is_none() {
                        // If there are no more characters, we break the loop
                        break;
                    }
            
                    let next_char = next_char.unwrap().1;
            
                    if next_char.is_alphanumeric() {
                        // If the character is alphanumeric, we continue processing
                        char_count += 1;
                        char_iter.next();
                    } else {
                        // If the character is not alphanumeric, we stop processing
                        next_char_alhpanum = false;
                    }
                }
            
                let literal = &self.input[i..i + char_count];
            
                let found_kwd = KEYWORDS.iter().position(|&s| s == literal);
                if found_kwd.is_some() {
                    // If the literal is a keyword, we add a keyword token
                    self.tokens
                        .vec
                        .push(KEYWORDS_TOKENS[found_kwd.unwrap()].clone());
                    continue;
                } else if crate::BUILT_IN_TYPES.contains(&literal) {
                    // If the literal is a built-in type, we add a type token
                    self.tokens.vec.push(Token::Type(literal));
                    continue;
                } else if crate::BUILT_IN_FUNCS.contains(&literal) {
                    // If the literal is a built-in function, we add a function token
                    self.tokens.vec.push(Token::BuiltInFunc(literal));

                    continue;
                } else {
                    // Handled in parser now
                    // // If the literal is not a keyword, built-in type, or built-in function, it's an identifier
                    // if peek == '(' {
                    //     // If the next character is an open parenthesis, this is a function call
                    //     self.tokens.vec.push(Token::Function(literal));
                    //     continue;
                    // }
            
                    // Otherwise, it's a variable or other identifier
                    self.tokens.vec.push(Token::Ident(literal));
                    continue;
                }
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
                ('=', _) => match peek {
                    '=' => {
                        char_iter.next();
                        Token::Operator(Operator::Equals)
                    }
                    '>' => {
                        char_iter.next();
                        Token::FatArrow
                    }
                    _ => Token::Assign,
                },
                ('*', _) => Token::Operator(Operator::Asterisk),
                ('/', _) => Token::Operator(Operator::Slash),
                ('!', _) => match peek {
                    '=' => {
                        char_iter.next();
                        Token::Operator(Operator::NotEqual)
                    }
                    _ => Token::Operator(Operator::Not),
                },
                ('<', _) => match peek {
                    '=' => {
                        char_iter.next();
                        Token::Operator(Operator::LessThanOrEqual)
                    }
                    '<' => {
                        char_iter.next();
                        Token::Operator(Operator::LeftShift)
                    }
                    _ => Token::Operator(Operator::LessThan),
                },
                ('>', _) => match peek {
                    '=' => {
                        char_iter.next();
                        Token::Operator(Operator::GreaterThanOrEqual)
                    }
                    '>' => {
                        char_iter.next();
                        Token::Operator(Operator::RightShift)
                    }
                    _ => Token::Operator(Operator::GreaterThan),
                },
                ('&', _) => match peek {
                    '&' => {
                        char_iter.next();
                        Token::Operator(Operator::And)
                    }
                    _ => Token::Operator(Operator::BitwiseAnd),
                },
                ('|', _) => match peek {
                    '|' => {
                        char_iter.next();
                        Token::Operator(Operator::Or)
                    }
                    _ => Token::Operator(Operator::BitwiseOr),
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
                    | Token::BuiltInFunc(_)
                    | Token::Function(_)
            )
        {
            return true;
        } else {
            return false;
        }
    }
}