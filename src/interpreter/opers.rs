use crate::Data;


pub fn negation(a: Data) -> Result<Data, String> {
    match a {
        Data::Int32(a) => Ok(Data::Int32(-a)),
        Data::Float64(a) => Ok(Data::Float64(-a)),
        _ => Err("Cannot negate this type".to_string()),
    }
}

pub fn not(a: Data) -> Result<Data, String> {
    match a {
        Data::Bool(a) => Ok(Data::Bool(!a)),
        
        // TODO: Add bitwise not

        _ => Err("Type mismatch, can't use on this type".to_string()),
    }
}

pub fn addition(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        // Simple addition
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Int32(a + b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Float64(a + b)),
        
        // Concatenate strings
        // Allocates a new string
        (Data::Str(a), Data::Str(b)) => {
            let mut a = *a;
            a.push_str(&*b);
            Ok(Data::Str(Box::new(a)))
        }
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn subtraction(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Int32(a - b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Float64(a - b)),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn multiplication(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Int32(a * b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Float64(a * b)),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn division(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Int32(a / b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Float64(a / b)),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn modulo(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Int32(a % b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Float64(a % b)),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn equals(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Bool(a == b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Bool(a == b)),
        (Data::Str(a), Data::Str(b)) => Ok(Data::Bool(a == b)),
        (Data::Char(a), Data::Char(b)) => Ok(Data::Bool(a == b)),
        (Data::Bool(a), Data::Bool(b)) => Ok(Data::Bool(a == b)),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

#[inline(always)]
fn type_mismatch(a: &Data, b: &Data) -> String {
    format!("Type mismatch, [ {:?} | {:?} ]", a.get_type(), b.get_type())
}