pub mod lexer;
pub mod parser;
pub mod interpreter;

pub use lexer::Lexer;
pub use parser::Parser;
pub use interpreter::Interpreter;

pub(crate) static BUILT_IN_TYPES: phf::Map<&'static str, DataType> = phf::phf_map! {
    "i32" => DataType::Int32,
    "f64" => DataType::Float64,
    "str" => DataType::Str,
    "char" => DataType::Char,
    "bool" => DataType::Bool,
};

pub(crate) const BUILT_IN_FUNCS: [&str; 1] = ["print"];

pub struct Engine {
    pub lexer: Lexer,
    pub parser: Parser,
    pub interpreter: Interpreter,
}

impl Engine {
    pub fn new() -> Engine {
        let lexer = Lexer::new();
        let parser = Parser::new();
        let interpreter = Interpreter::new();

        Engine {
            lexer,
            parser,
            interpreter,
        }
    }

    pub fn run(&mut self, script: String) -> Result<(), String> {
        // TODO: Error handling 
        self.lexer.set_input(script);
        let tokens = self.lexer.tokenize()?;
        self.parser.set_tokens(tokens);
        self.interpreter.run(self.parser.parse()?)
    }
}

pub struct ReplEngine {
    pub lexer: Lexer,
    pub parser: Parser,
    pub interpreter: Interpreter,
    _line: String,
}

impl ReplEngine {
    pub fn new() -> ReplEngine {
        let lexer = Lexer::new();
        let parser = Parser::new();
        let interpreter = Interpreter::new();

        ReplEngine {
            lexer,
            parser,
            interpreter,
            _line: String::new(),
        }
    }

    pub fn eval(&mut self, statement: String) -> Result<(), String> {
        self.parser.reset();
        
        self.lexer.set_input(statement);
        let res = self.lexer.tokenize();
        let tokens: lexer::Tokens;

        if res.is_err() {
            return Err(format!("Lexer error: {}", res.unwrap_err()));
        } else {
            tokens = res.unwrap();
        }

        self.parser.set_tokens(tokens);
        let res = self.parser.parse();

        if res.is_err() {
            return Err(format!("Parser error: {}", res.unwrap_err()));
        }

        let res = self.interpreter.run(res.unwrap());
        if res.is_err() {
            return Err(format!("Interpreter error: {}", res.unwrap_err()));
        }

        Ok(())
    }
}

// Exposed to allow data in and out of the interpreter
#[derive(Debug, Clone)]
pub enum Data {
    Int32(i32),
    Float64(f64),
    Str(String),
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

