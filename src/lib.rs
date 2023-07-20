pub static BUILT_IN_TYPES: phf::Map<&'static str, DataType> = phf::phf_map! {
    "i32" => DataType::Int32,
    "f64" => DataType::Float64,
    "str" => DataType::Str,
    "char" => DataType::Char,
    "bool" => DataType::Bool,
};

pub const BUILT_IN_FUNCS: [&str; 1] = ["print"];

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
pub enum Operator {
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

