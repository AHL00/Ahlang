use std::{error::Error, fmt::Display, iter::Peekable};

use crate::token::{Delimiter, Keyword, Literal, Operator, Punctuation, Token, TokenType};

struct Lexer<'a> {
    tokens: Vec<Token<'a>>,
    /// Zero-based index of the current character.
    current: usize,
    line: usize,
    column: usize,
    first_char: bool,
    chars: Peekable<std::str::Chars<'a>>,
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn consume_char(&mut self) -> Option<char> {
        let c = self.chars.next();

        if let Some(c) = c {
            if !self.first_char {
                self.current += 1;
            } else {
                self.first_char = false;
            }

            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }

        c
    }

    fn push_token(&mut self, token_type: TokenType<'a>) {
        let token = Token {
            token_type,
            line: self.line,
            column: self.column,
        };

        self.tokens.push(token);
    }

    pub fn lex(source: &'a String) -> Result<Vec<Token<'a>>, LexerError> {
        let mut lexer = Self {
            tokens: Vec::new(),
            current: 0,
            line: 1,
            column: 0,
            first_char: true,
            chars: source.chars().peekable(),
        };

        loop {
            if let Some(c) = lexer.consume_char() {
                if c.is_whitespace() {
                    continue;
                }

                if c.is_alphabetic() {
                    // If the character is a letter, it's either an identifier or a keyword.
                    let start = lexer.current;

                    // Consume the rest of the identifier.
                    // _ is the only special character allowed in an identifier.
                    while let Some(c) = lexer.peek() {
                        if c.is_alphanumeric() || *c == '_' {
                            lexer.consume_char();
                        } else {
                            break;
                        }
                    }

                    let identifier = &source[start..lexer.current + 1];

                    // Check if it is a keyword.
                    let keyword = Keyword::try_parse_token(identifier);

                    if let Some(keyword) = keyword {
                        lexer.push_token(TokenType::Keyword(keyword));
                    } else {
                        // If it's not a keyword, it's an identifier.
                        lexer.push_token(TokenType::Identifier(identifier));
                    }

                    continue;
                }

                if c.is_numeric() {
                    // If the first character is a number, it's a literal of some kind.
                    let start = lexer.current;

                    let mut contains_decimal = false;

                    // Consume the rest of the number.
                    while let Some(c) = lexer.peek() {
                        if c.is_numeric() || *c == '.' || *c == '_' {
                            if *c == '.' {
                                if contains_decimal {
                                    return Err(LexerError::new(
                                        lexer.line,
                                        lexer.column,
                                        "Invalid number literal, two decimals found.".to_string(),
                                    ));
                                }

                                contains_decimal = true;
                            }

                            lexer.consume_char();
                        } else {
                            break;
                        }
                    }

                    let number_str = &source[start..lexer.current + 1];

                    if contains_decimal {
                        lexer.push_token(TokenType::Literal(Literal::Float(number_str)));
                    } else {
                        lexer.push_token(TokenType::Literal(Literal::Int(number_str)));
                    }

                    continue;
                }

                if c == '"' {
                    // If the first character is a double quote, it's a string literal.
                    let start = lexer.current + 1;

                    // Consume the rest of the string.
                    while let Some(c) = lexer.peek() {
                        if *c == '"' {
                            lexer.consume_char();
                            break;
                        } else {
                            lexer.consume_char();
                        }
                    }

                    let string_literal = &source[start..lexer.current];

                    lexer.push_token(TokenType::Literal(Literal::Str(string_literal)));

                    continue;
                }

                if c == '\'' {
                    // If the first character is a single quote, it's a character literal.
                    let start = lexer.current + 1;

                    // Consume the rest of the character.
                    while let Some(c) = lexer.peek() {
                        if *c == '\'' {
                            lexer.consume_char();
                            break;
                        } else {
                            lexer.consume_char();
                        }
                    }

                    let char_literal = &source[start..lexer.current];

                    lexer.push_token(TokenType::Literal(Literal::Char(char_literal)));

                    continue;
                }


                // Try parsing as any of the rest.
                match c {
                    '(' => lexer.push_token(TokenType::Delimiter(Delimiter::OpenParen)),
                    ')' => lexer.push_token(TokenType::Delimiter(Delimiter::CloseParen)),
                    '{' => lexer.push_token(TokenType::Delimiter(Delimiter::OpenBrace)),
                    '}' => lexer.push_token(TokenType::Delimiter(Delimiter::CloseBrace)),
                    '[' => lexer.push_token(TokenType::Delimiter(Delimiter::OpenBracket)),
                    ']' => lexer.push_token(TokenType::Delimiter(Delimiter::CloseBracket)),
                    ',' => lexer.push_token(TokenType::Punctuation(Punctuation::Comma)),
                    '.' => lexer.push_token(TokenType::Punctuation(Punctuation::Dot)),
                    ':' => lexer.push_token(TokenType::Punctuation(Punctuation::Colon)),
                    ';' => lexer.push_token(TokenType::Punctuation(Punctuation::Semicolon)),
                    '+' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::AddAssign));
                            } else if *c == '+' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::Increment));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::Add));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::Add));
                        }
                    }
                    '-' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::SubAssign));
                            } else if *c == '>' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Punctuation(Punctuation::Arrow));
                            } else if *c == '-' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::Decrement));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::Sub));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::Sub));
                        }
                    }
                    '*' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::MulAssign));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::Mul));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::Mul));
                        }
                    }
                    '/' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::DivAssign));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::Div));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::Div));
                        }
                    }
                    '%' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::ModAssign));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::Modulus));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::Modulus));
                        }
                    }
                    '&' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '&' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::LogicalAnd));
                            } else if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::AndAssign));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::BitwiseAnd));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::BitwiseAnd));
                        }
                    }
                    '|' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '|' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::LogicalOr));
                            } else if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::OrAssign));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::BitwiseOr));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::BitwiseOr));
                        }
                    }
                    '^' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::XorAssign));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::BitwiseXor));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::BitwiseXor));
                        }
                    }
                    '~' => lexer.push_token(TokenType::Operator(Operator::BitwiseNot)),
                    '!' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::NotEqual));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::LogicalNot));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::LogicalNot));
                        }
                    }
                    '<' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '<' {
                                lexer.consume_char();
                                if let Some(c) = lexer.peek() {
                                    if *c == '=' {
                                        lexer.consume_char();
                                        lexer.push_token(TokenType::Operator(Operator::ShlAssign));
                                    } else {
                                        lexer.push_token(TokenType::Operator(Operator::ShiftLeft));
                                    }
                                } else {
                                    lexer.push_token(TokenType::Operator(Operator::ShiftLeft));
                                }
                            } else if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::LessThanOrEqual));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::LessThan));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::LessThan));
                        }
                    }
                    '>' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '>' {
                                lexer.consume_char();
                                if let Some(c) = lexer.peek() {
                                    if *c == '=' {
                                        lexer.consume_char();
                                        lexer.push_token(TokenType::Operator(Operator::ShrAssign));
                                    } else {
                                        lexer.push_token(TokenType::Operator(Operator::ShiftRight));
                                    }
                                } else {
                                    lexer.push_token(TokenType::Operator(Operator::ShiftRight));
                                }
                            } else if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::GreaterThanOrEqual));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::GreaterThan));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::GreaterThan));
                        }
                    }
                    '=' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Operator(Operator::Equal));
                            } else if *c == '>' {
                                lexer.consume_char();
                                lexer.push_token(TokenType::Punctuation(Punctuation::FatArrow));
                            } else {
                                lexer.push_token(TokenType::Operator(Operator::Assign));
                            }
                        } else {
                            lexer.push_token(TokenType::Operator(Operator::Assign));
                        }
                    }
                    _ => {
                        return Err(LexerError::new(
                            lexer.line,
                            lexer.column,
                            format!("Unexpected character: {}", c),
                        ));
                    }
                }
            } else {
                break;
            }
        }

        Ok(lexer.tokens)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct LexerError {
    line: usize,
    column: usize,
    message: String,
}

impl LexerError {
    fn new(line: usize, column: usize, message: String) -> Self {
        Self {
            line,
            column,
            message,
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lexer error at line {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl Error for LexerError {}

trait TryParseToken: Sized {
    fn try_parse_token(string: &str) -> Option<Self>;
}

impl TryParseToken for Keyword {
    fn try_parse_token(string: &str) -> Option<Self> {
        match string {
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "return" => Some(Keyword::Return),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "let" => Some(Keyword::Let),
            "mut" => Some(Keyword::Mut),
            "fn" => Some(Keyword::Fn),
            "struct" => Some(Keyword::Struct),
            "enum" => Some(Keyword::Enum),
            "type" => Some(Keyword::Type),
            "use" => Some(Keyword::Use),
            "static" => Some(Keyword::Static),
            "const" => Some(Keyword::Const),
            "mod" => Some(Keyword::Mod),
            "extern" => Some(Keyword::Extern),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let source = String::from("let a = 5;");
        let token_vec = Lexer::lex(&source).unwrap();

        let token_type_vec = token_vec
            .iter()
            .map(|token| token.token_type.clone())
            .collect::<Vec<TokenType>>();

        assert_eq!(
            token_type_vec,
            vec![
                TokenType::Keyword(Keyword::Let),
                TokenType::Identifier("a"),
                TokenType::Operator(Operator::Assign),
                TokenType::Literal(Literal::Int("5")),
                TokenType::Punctuation(Punctuation::Semicolon),
            ]
        );
    }

    #[test]
    fn test_multichar_operator() {
        let source = String::from("a += >>=");
        let token_vec = Lexer::lex(&source).unwrap();

        let token_type_vec = token_vec
            .iter()
            .map(|token| token.token_type.clone())
            .collect::<Vec<TokenType>>();

        assert_eq!(
            token_type_vec,
            vec![
                TokenType::Identifier("a"),
                TokenType::Operator(Operator::AddAssign),
                TokenType::Operator(Operator::ShrAssign),
            ]
        );
    }

    #[test]
    fn test_multiline() {
        let source = String::from("a\nb");
        let token_vec = Lexer::lex(&source).unwrap();

        let token_type_vec = token_vec
            .iter()
            .map(|token| token.token_type.clone())
            .collect::<Vec<TokenType>>();

        assert_eq!(token_vec[0].line, 1);

        assert_eq!(token_vec[1].line, 2);

        assert_eq!(
            token_type_vec,
            vec![TokenType::Identifier("a"), TokenType::Identifier("b"),]
        );
    }

    #[test]
    fn test_lex_error() {
        let source = String::from("let a = 5.5.5;");
        let lexer = Lexer::lex(&source);

        assert!(lexer.is_err());
    }

    #[test]
    fn test_string_literal() {
        let source = String::from("\"Hello, world!\";");
        let token_vec = Lexer::lex(&source).unwrap();

        let token_type_vec = token_vec
            .iter()
            .map(|token| token.token_type.clone())
            .collect::<Vec<TokenType>>();

        assert_eq!(
            token_type_vec,
            vec![
                TokenType::Literal(Literal::Str("Hello, world!")),
                TokenType::Punctuation(Punctuation::Semicolon),
            ]
        );
    }

    #[test]
    fn test_char_literal() {
        let source = String::from("'a';");
        let token_vec = Lexer::lex(&source).unwrap();

        let token_type_vec = token_vec
            .iter()
            .map(|token| token.token_type.clone())
            .collect::<Vec<TokenType>>();

        assert_eq!(
            token_type_vec,
            vec![
                TokenType::Literal(Literal::Char("a")),
                TokenType::Punctuation(Punctuation::Semicolon),
            ]
        );
    }
}
