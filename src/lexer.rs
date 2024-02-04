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
                // If they all return None, then keep consuming the string until it is recognized.
                // If it goes through the whole string and doesn't recognize it, then it's an error.
                let start = lexer.current;

                // TODO: Fix this ugly code.
                // If there is something like "===" in the source code, it will be parsed as three
                // "=". This is not the correct behavior.
                // I think a better way to do this would be to pass the lexer to the try_parse_token
                // function and let it consume the characters as needed.
                // Maybe it shouldn't break immediately after finding a token, but keep consuming
                // characters until it finds a token or reaches the end of the string.
                // That's crazy inefficient though.
                loop {
                    let operator = Operator::try_parse_token(&source[start..lexer.current + 1]);

                    if let Some(operator) = operator {
                        lexer.tokens.push(TokenType::Operator(operator));
                        break;
                    }

                    let punctuation =
                        Punctuation::try_parse_token(&source[start..lexer.current + 1]);

                    if let Some(punctuation) = punctuation {
                        lexer.tokens.push(TokenType::Punctuation(punctuation));
                        break;
                    }

                    let delimiter = Delimiter::try_parse_token(&source[start..lexer.current + 1]);

                    if let Some(delimiter) = delimiter {
                        lexer.tokens.push(TokenType::Delimiter(delimiter));
                        break;
                    }

                    if let None = lexer.consume_char() {
                        break;
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

impl TryParseToken for Punctuation {
    fn try_parse_token(string: &str) -> Option<Self> {
        match string {
            "," => Some(Punctuation::Comma),
            "." => Some(Punctuation::Dot),
            ":" => Some(Punctuation::Colon),
            ";" => Some(Punctuation::Semicolon),
            "->" => Some(Punctuation::Arrow),
            "=>" => Some(Punctuation::FatArrow),
            _ => None,
        }
    }
}

impl TryParseToken for Delimiter {
    fn try_parse_token(string: &str) -> Option<Self> {
        match string {
            "(" => Some(Delimiter::OpenParen),
            ")" => Some(Delimiter::CloseParen),
            "{" => Some(Delimiter::OpenBrace),
            "}" => Some(Delimiter::CloseBrace),
            "[" => Some(Delimiter::OpenBracket),
            "]" => Some(Delimiter::CloseBracket),
            _ => None,
        }
    }
}

impl TryParseToken for Operator {
    fn try_parse_token(string: &str) -> Option<Self> {
        match string {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            "%" => Some(Operator::Modulus),
            "&&" => Some(Operator::LogicalAnd),
            "||" => Some(Operator::LogicalOr),
            "!" => Some(Operator::LogicalNot),
            "&" => Some(Operator::BitwiseAnd),
            "|" => Some(Operator::BitwiseOr),
            "^" => Some(Operator::BitwiseXor),
            "~" => Some(Operator::BitwiseNot),
            "<<" => Some(Operator::ShiftLeft),
            ">>" => Some(Operator::ShiftRight),
            "==" => Some(Operator::Equal),
            "!=" => Some(Operator::NotEqual),
            "<" => Some(Operator::LessThan),
            ">" => Some(Operator::GreaterThan),
            "<=" => Some(Operator::LessThanOrEqual),
            ">=" => Some(Operator::GreaterThanOrEqual),
            "=" => Some(Operator::Assign),
            "+=" => Some(Operator::AddAssign),
            "-=" => Some(Operator::SubAssign),
            "*=" => Some(Operator::MulAssign),
            "/=" => Some(Operator::DivAssign),
            "%=" => Some(Operator::ModAssign),
            "&=" => Some(Operator::AndAssign),
            "|=" => Some(Operator::OrAssign),
            "^=" => Some(Operator::XorAssign),
            "<<=" => Some(Operator::ShlAssign),
            ">>=" => Some(Operator::ShrAssign),
            "++" => Some(Operator::Increment),
            "--" => Some(Operator::Decrement),
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
    fn test_lex_error() {
        let source = String::from("let a = 5.5.5;");
        let lexer = Lexer::lex(&source);

        assert!(lexer.is_err());
    }
}
