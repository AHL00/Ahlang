#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType<'a> {
    Identifier(&'a str),
    Literal(Literal<'a>),
    Operator(Operator),
    Keyword(Keyword),
    Punctuation(Punctuation),
    Delimiter(Delimiter),
}

/// The token only include slices to the original string. 
/// It doesn't do any sort of parsing of any kind.
/// It allows the parser to get the source code location of each token.
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token<'a> {
    pub(crate) token_type: TokenType<'a>,
    pub(crate) line: usize,
    pub(crate) column: usize,
}


#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Modulus,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    ShiftLeft,
    ShiftRight,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    ShlAssign,
    ShrAssign,
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Literal<'a> {
    /// Can contain underscores.
    Int(&'a str),
    /// Can contain underscores and always a decimal point.
    Float(&'a str),
    Str(&'a str),
    Char(&'a str),
    Bool(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Punctuation {
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    FatArrow,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Delimiter {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Keyword {
    Fn,
    Let,
    If,
    Else,
    While,
    For,
    Return,
    Break,
    Continue,
    Match,
    Case,
    Struct,
    Enum,
    Type,
    Use,
    Mod,
    Extern,
    Static,
    Mut,
    Const,
}