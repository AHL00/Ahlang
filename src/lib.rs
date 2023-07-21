pub mod interpreter;
pub mod lexer;
pub mod parser;

pub use interpreter::Interpreter;
pub use lexer::Lexer;
pub use parser::Parser;

pub(crate) const BUILT_IN_TYPES: [&str; 5] = ["i32", "f64", "str", "char", "bool"];
pub(crate) const BUILT_IN_TYPES_DATA_TYPES: [DataType; 5] = [
    DataType::Int32,
    DataType::Float64,
    DataType::Str,
    DataType::Char,
    DataType::Bool,
];

pub(crate) const BUILT_IN_FUNCS: [&str; 1] = ["print"];

pub struct Engine<'a> {
    pub lexer: Lexer<'a>,
    pub parser: Parser<'a>,
    pub interpreter: Interpreter,
}

impl<'a> Engine<'a> {
    pub fn new() -> Engine<'a> {
        let lexer = Lexer::new();
        let parser = Parser::new();
        let interpreter = Interpreter::new();

        Engine {
            lexer,
            parser,
            interpreter,
        }
    }

    pub fn run(&'a mut self, script: &'a str) -> Result<(), String> {
        self.lexer.set_input(script);
        let tokens = self.lexer.tokenize()?;
        self.parser.set_tokens(tokens);
        let ast = self.parser.parse()?;
        self.interpreter.run(ast)
    }
}

pub struct ReplEngine {
    pub interpreter: Interpreter,
}

impl ReplEngine {
    pub fn new() -> ReplEngine {
        let interpreter = Interpreter::new();

        ReplEngine { interpreter }
    }

    pub fn eval(&mut self, line: &str) -> Result<(), String> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();

        lexer.set_input(line);
        let tokens = lexer.tokenize().map_err(|e| format!("[Lexer error] {}", e))?;

        parser.set_tokens(tokens);
        let ast = parser.parse().map_err(|e| format!("[Parser error] {}", e))?;

        self.interpreter.run(ast)
    }
}

// Exposed to allow data in and out of the interpreter
#[derive(Debug, Clone)]
pub enum Data {
    Int32(i32),
    Float64(f64),
    Str(Box<String>),
    Char(char),
    Bool(bool),
}

impl Data {
    pub fn get_type(&self) -> DataType {
        match self {
            Data::Int32(_) => DataType::Int32,
            Data::Float64(_) => DataType::Float64,
            Data::Str(_) => DataType::Str,
            Data::Char(_) => DataType::Char,
            Data::Bool(_) => DataType::Bool,
        }
    }

    /// Clones the data into a Box<dyn Any>, helpful for interfacing between the interpreter and Rust
    pub fn to_rust_type(&self) -> Box<dyn std::any::Any> {
        match self {
            Data::Int32(i) => Box::new(*i),
            Data::Float64(f) => Box::new(*f),
            Data::Str(s) => Box::new(s.clone()),
            Data::Char(c) => Box::new(c.clone()),
            Data::Bool(b) => Box::new(*b),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataType {
    Int32,
    Float64,
    Str,
    Char,
    Bool,
}

impl DataType {
    pub fn from_str(s: &str) -> Option<DataType> {
        match s {
            "i32" => Some(DataType::Int32),
            "f64" => Some(DataType::Float64),
            "str" => Some(DataType::Str),
            "char" => Some(DataType::Char),
            "bool" => Some(DataType::Bool),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            DataType::Int32 => "i32",
            DataType::Float64 => "f64",
            DataType::Str => "str",
            DataType::Char => "char",
            DataType::Bool => "bool",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Operator {
    // Infix operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,

    // Unary operators
    Not,
    Identity,
    Negation,
}