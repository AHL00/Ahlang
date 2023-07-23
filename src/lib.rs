#![allow(dead_code)]
#![allow(unused_labels)]

pub mod interpreter;
pub mod lexer;
pub mod parser;
mod stdlib;


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

pub struct Engine {
    pub interpreter: Interpreter,
}

impl Engine {
    pub fn new() -> Engine {
        let interpreter = Interpreter::new();

        Engine {
            interpreter,
        }
    }

    pub fn run(&mut self, script: &str) -> Result<(), String> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();

        lexer.set_input(script);
        let tokens = lexer.tokenize()?;
        parser.set_tokens(tokens);
        let ast = parser.parse()?;
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

    pub fn get_vars(&self) -> Vec<(String, Data)> {
        self.interpreter
            .vars
            .iter()
            .map(|(k, v)| (k.clone(), v.data.clone()))
            .collect()
    }

    pub fn eval(&mut self, line: &str) -> Result<(), String> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();

        lexer.set_input(line);
        let tokens = lexer.tokenize().map_err(|e| format!("[Lexer error] {}", e))?;

        parser.set_tokens(tokens);
        let ast = parser.parse().map_err(|e| format!("[Parser error] {}", e))?;

        self.interpreter.run(ast).map_err(|e| format!("[Interpreter error] {}", e))
    }
}

// NOTE: use get_type() for now, if performance becomes an issue, redesign data enum to be wrapped in a struct that contains the data type as well 

// pub struct FunctionReturn {
//     pub data: Data,
//     pub data_type: DataType,
// }

// pub struct FunctionArg {
//     pub name: String,
//     pub data_type: DataType,
// }

trait Format {
    fn format(&self) -> String;
}

// Exposed to allow data in and out of the interpreter
#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Int32 {
        val: i32,
    },
    Float64 {
        val: f64,
    },
    Str {
        // Box to reduce mem size of enum
        val: Box<String>,
    },
    Char {
        val: char,
    },
    Bool {
        val: bool,
    },
    // NOTE:
    // Class {
    //     data: Vec<Data>, // store the index of the vars in a hashmap somewhere 
    //     class_type: String, // store the member functions in a hashmap somewhere
    // },
    Empty {

    },
}

impl Data {
    fn get_type(&self) -> DataType {
        match self {
            Data::Int32 { .. } => DataType::Int32,
            Data::Float64 { .. } => DataType::Float64,
            Data::Str { .. } => DataType::Str,
            Data::Char { .. } => DataType::Char,
            Data::Bool { .. } => DataType::Bool,
            Data::Empty { .. } => DataType::Empty,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Data::Empty { .. } => true,
            _ => false,
        }
    }
}


// TODO: Remove and use Data(_) instead?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataType {
    Int32,
    Float64,
    Str,
    Char,
    Bool,
    Empty,
}

impl DataType {
    pub fn from_str(s: &str) -> Option<DataType> {
        match s {
            "i32" => Some(DataType::Int32),
            "f64" => Some(DataType::Float64),
            "str" => Some(DataType::Str),
            "char" => Some(DataType::Char),
            "bool" => Some(DataType::Bool),
            "()" => Some(DataType::Empty),
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
            DataType::Empty => "()",
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
    /// Power and bitwise xor
    Caret,
    Modulo,

    BitwiseAnd,
    BitwiseOr,
    LeftShift,
    RightShift,

    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equals,
    NotEqual,

    And,
    Or,

    // Prefix operators
    /// Bools and bitwise not
    Not,
    Identity,
    Negation,
}