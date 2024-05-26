use crate::SourceLocation;

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Identifier(&'a str),
    Literal(Literal<'a>),
    Operator(Operator),
    Keyword(Keyword),
    Punctuation(Punctuation),
    Delimiter(Delimiter),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
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
pub enum Literal<'a> {
    /// Can contain underscores.
    Integer(&'a str),
    /// Can contain underscores and always a decimal point.
    Float(&'a str),
    String(&'a str),
    Char(&'a str),
    Boolean(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuation {
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    FatArrow,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Delimiter {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
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
