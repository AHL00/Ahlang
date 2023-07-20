pub const KEYWORDS: [&str; 2] = ["fn", "let"];
pub static BUILT_IN_TYPES: phf::Map<&'static str, DataType> = phf::phf_map! {
    "i32" => DataType::Int32,
    "f64" => DataType::Float64,
    "str" => DataType::Str,
    "bool" => DataType::Bool,
};
pub const BUILT_IN_FUNCS: [&str; 1] = ["print"];

#[derive(Debug, Clone)]
pub enum Data<'a> {
    Int32(i32),
    Float64(f64),
    Str(&'a str),
    Bool(bool),
}

impl<'a> Data<'a> {
    pub fn get_type(&self) -> DataType {
        match self {
            Data::Int32(_) => DataType::Int32,
            Data::Float64(_) => DataType::Float64,
            Data::Str(_) => DataType::Str,
            Data::Bool(_) => DataType::Bool,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Int32,
    Float64,
    Str,
    Bool,
}
