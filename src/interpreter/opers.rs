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
        _ => Err("Can only apply this operator to booleans".to_string()),
    }
}

pub fn addition(a: Data, b: Data) -> Result<Data, String> {
    // TODO: Type mismatch?
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
        _ => Err("Cannot add these types".to_string()),
    }
}

pub fn subtraction(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Int32(a - b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Float64(a - b)),
        _ => Err("Type mismatch".to_string()),
    }
}

pub fn multiplication(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Int32(a * b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Float64(a * b)),
        _ => Err("Type mismatch".to_string()),
    }
}

pub fn division(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Int32(a / b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Float64(a / b)),
        _ => Err("Type mismatch".to_string()),
    }
}

pub fn modulo(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32(a), Data::Int32(b)) => Ok(Data::Int32(a % b)),
        (Data::Float64(a), Data::Float64(b)) => Ok(Data::Float64(a % b)),
        _ => Err("Type mismatch".to_string()),
    }
}