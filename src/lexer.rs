use std::{error::Error, fmt::Display, iter::Peekable};

use crate::token::{Delimiter, Keyword, Literal, Operator, Punctuation, TokenType};

struct Lexer<'a> {
    tokens: Vec<TokenType<'a>>,
    /// Zero-based index of the current character.
    current: usize,
    line: usize,
    first_char: bool,
    chars: Peekable<std::str::Chars<'a>>,
}

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
            }
        }

        c
    }

    pub fn lex(source: &'a String) -> Result<Lexer<'a>, LexerError> {
        let mut lexer = Self {
            tokens: Vec::new(),
            current: 0,
            line: 1,
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
                        lexer.tokens.push(TokenType::Keyword(keyword));
                    } else {
                        // If it's not a keyword, it's an identifier.
                        lexer.tokens.push(TokenType::Identifier(identifier));
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
                                        lexer.current,
                                        start,
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
                        lexer
                            .tokens
                            .push(TokenType::Literal(Literal::Float(number_str)));
                    } else {
                        lexer
                            .tokens
                            .push(TokenType::Literal(Literal::Int(number_str)));
                    }

                    continue;
                }

                // Try parsing as any of the rest.
                match c {
                    '(' => lexer.tokens.push(TokenType::Delimiter(Delimiter::OpenParen)),
                    ')' => lexer.tokens.push(TokenType::Delimiter(Delimiter::CloseParen)),
                    '{' => lexer.tokens.push(TokenType::Delimiter(Delimiter::OpenBrace)),
                    '}' => lexer.tokens.push(TokenType::Delimiter(Delimiter::CloseBrace)),
                    '[' => lexer.tokens.push(TokenType::Delimiter(Delimiter::OpenBracket)),
                    ']' => lexer.tokens.push(TokenType::Delimiter(Delimiter::CloseBracket)),
                    ',' => lexer.tokens.push(TokenType::Punctuation(Punctuation::Comma)),
                    '.' => lexer.tokens.push(TokenType::Punctuation(Punctuation::Dot)),
                    ':' => lexer.tokens.push(TokenType::Punctuation(Punctuation::Colon)),
                    ';' => lexer.tokens.push(TokenType::Punctuation(Punctuation::Semicolon)),
                    '+' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::AddAssign));
                            } else if *c == '+' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::Increment));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::Add));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::Add));
                        }
                    }
                    '-' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::SubAssign));
                            } else if *c == '-' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::Decrement));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::Sub));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::Sub));
                        }
                    }
                    '*' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::MulAssign));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::Mul));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::Mul));
                        }
                    }
                    '/' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::DivAssign));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::Div));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::Div));
                        }
                    }
                    '%' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::ModAssign));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::Modulus));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::Modulus));
                        }
                    }
                    '&' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '&' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::LogicalAnd));
                            } else if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::AndAssign));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::BitwiseAnd));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::BitwiseAnd));
                        }
                    }
                    '|' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '|' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::LogicalOr));
                            } else if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::OrAssign));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::BitwiseOr));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::BitwiseOr));
                        }
                    }
                    '^' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::XorAssign));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::BitwiseXor));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::BitwiseXor));
                        }
                    }
                    '~' => lexer.tokens.push(TokenType::Operator(Operator::BitwiseNot)),
                    '!' => lexer.tokens.push(TokenType::Operator(Operator::LogicalNot)),
                    '<' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '<' {
                                lexer.consume_char();
                                if let Some(c) = lexer.peek() {
                                    if *c == '=' {
                                        lexer.consume_char();
                                        lexer.tokens.push(TokenType::Operator(Operator::ShlAssign));
                                    } else {
                                        lexer.tokens.push(TokenType::Operator(Operator::ShiftLeft));
                                    }
                                } else {
                                    lexer.tokens.push(TokenType::Operator(Operator::ShiftLeft));
                                }
                            } else if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::LessThanOrEqual));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::LessThan));
                            }
                        } else  {
                            lexer.tokens.push(TokenType::Operator(Operator::LessThan));
                        }
                    }
                    '>' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '>' {
                                lexer.consume_char();
                                if let Some(c) = lexer.peek() {
                                    if *c == '=' {
                                        lexer.consume_char();
                                        lexer.tokens.push(TokenType::Operator(Operator::ShrAssign));
                                    } else {
                                        lexer.tokens.push(TokenType::Operator(Operator::ShiftRight));
                                    }
                                } else {
                                    lexer.tokens.push(TokenType::Operator(Operator::ShiftRight));
                                }
                            } else if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::GreaterThanOrEqual));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::GreaterThan));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::GreaterThan));
                        }
                    }
                    '=' => {
                        if let Some(c) = lexer.peek() {
                            if *c == '=' {
                                lexer.consume_char();
                                lexer.tokens.push(TokenType::Operator(Operator::Equal));
                            } else {
                                lexer.tokens.push(TokenType::Operator(Operator::Assign));
                            }
                        } else {
                            lexer.tokens.push(TokenType::Operator(Operator::Assign));
                        }
                    }
                    _ => {
                        return Err(LexerError::new(
                            lexer.line,
                            lexer.current,
                            lexer.current,
                            format!("Unexpected character: {}", c),
                        ));
                    }
                }
            } else {
                break;
            }
        }

        Ok(lexer)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct LexerError {
    line: usize,
    column: usize,
    char: usize,
    message: String,
}

impl LexerError {
    fn new(line: usize, column: usize, char: usize, message: String) -> Self {
        Self {
            line,
            column,
            char,
            message,
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lexer error at line {}:{} (char {}): {}",
            self.line, self.column, self.char, self.message
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
        let lexer = Lexer::lex(&source).unwrap();

        print!("{:?}", lexer.tokens);

        assert_eq!(
            lexer.tokens,
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
        let lexer = Lexer::lex(&source).unwrap();

        print!("{:?}", lexer.tokens);

        assert_eq!(
            lexer.tokens,
            vec![
                TokenType::Identifier("a"),
                TokenType::Operator(Operator::AddAssign),
                TokenType::Operator(Operator::ShrAssign),
            ]
        );
    }

    #[test]
    fn test_lex_error() {
        let source = String::from("let a = 5.5.5;");
        let lexer = Lexer::lex(&source);

        assert!(lexer.is_err());
    }
}
