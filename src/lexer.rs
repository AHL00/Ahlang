#[derive(Debug, PartialEq, Clone)]
pub enum LexerLiteralType<'a> {
    INT(&'a str),
    FLOAT(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorType {
    ASSIGN,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    CARET,
    NOT,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    ILLEGAL,
    // { TODO: Add line and column number to token to use in error messages
    //     line: usize,
    //     col: usize,
    // },
    EOF,

    // Identifiers
    IDENT(&'a str),
    BUILT_IN_TYPE(&'a str),

    // Literals
    LITERAL(LexerLiteralType<'a>),

    // Built-in functions
    BUILT_IN_FUNC(&'a str),

    // Operators
    OPERATOR(OperatorType),

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

static KEYWORDS: [&str; 2] = ["fn", "let"];
static BUILT_IN_TYPES: [&str; 2] = ["int", "float"];
static BUILT_IN_FUNCS: [&str; 1] = ["print"];

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

            let (i, char) = i.unwrap();

            if char.is_whitespace() {
                continue;
            }

            // check for int/float literal
            if char.is_digit(10) {
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
                    self.tokens
                        .push(Token::LITERAL(LexerLiteralType::FLOAT(&self.input[i..i + char_count])));
                } else {
                    self.tokens.push(Token::LITERAL(LexerLiteralType::INT(&self.input[i..i + char_count])));
                }

                continue;
            } else if char.is_alphabetic() {
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

                if KEYWORDS.contains(&literal) {
                    match literal {
                        "fn" => self.tokens.push(Token::FN),
                        "let" => self.tokens.push(Token::LET),
                        _ => {
                            return Err(format!("Unknown keyword: {}", literal));
                        }
                    }

                    continue;
                } else if BUILT_IN_TYPES.contains(&literal) {
                    self.tokens.push(Token::BUILT_IN_TYPE(literal));
                    continue;
                } else if BUILT_IN_FUNCS.contains(&literal) {
                    self.tokens.push(Token::BUILT_IN_FUNC(literal));
                    continue;
                } 
                else {
                    self.tokens.push(Token::IDENT(literal));
                    continue;
                }
            }

            match char {
                '=' => self.tokens.push(Token::OPERATOR(OperatorType::ASSIGN)),
                '+' => self.tokens.push(Token::OPERATOR(OperatorType::PLUS)),
                '-' => self.tokens.push(Token::OPERATOR(OperatorType::MINUS)),
                '*' => self.tokens.push(Token::OPERATOR(OperatorType::ASTERISK)),
                '/' => self.tokens.push(Token::OPERATOR(OperatorType::SLASH)),
                '!' => self.tokens.push(Token::OPERATOR(OperatorType::NOT)),
                '^' => self.tokens.push(Token::OPERATOR(OperatorType::CARET)),
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
}
