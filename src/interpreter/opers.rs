use crate::Data;


pub fn negation(a: Data) -> Result<Data, String> {
    match a {
        Data::Int32 { val: a } => Ok(Data::Int32 { val: -a }),
        Data::Float64 { val: a } => Ok(Data::Float64 { val: -a }),
        _ => Err("Cannot negate this type".to_string()),
    }
}

pub fn not(a: Data) -> Result<Data, String> {
    match a {
        Data::Bool { val: a } => Ok(Data::Bool { val: !a }),
        
        // TODO: Add bitwise not

        _ => Err("Type mismatch, can't use on this type".to_string()),
    }
}

pub fn addition(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        // Simple addition
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Int32 { val: a + b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Float64 { val: a + b }),
        
        // Concatenate strings
        // Allocates a new string
        (Data::Str { val: a }, Data::Str { val: b }) => {
            let mut a = *a;
            a.push_str(&*b);
            Ok(Data::Str { val: Box::new(a) })
        }
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn subtraction(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Int32 { val: a - b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Float64 { val: a - b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn multiplication(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Int32 { val: a * b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Float64 { val: a * b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn division(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Int32 { val: a / b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Float64 { val: a / b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn modulo(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Int32 { val: a % b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Float64 { val: a % b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn power(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Int32 { val: a.pow(b as u32) }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Float64 { val: a.powf(b) }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn equals(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Bool { val: a == b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Bool { val: a == b }),
        (Data::Str { val: a }, Data::Str { val: b }) => Ok(Data::Bool { val: a == b }),
        (Data::Char { val: a }, Data::Char { val: b }) => Ok(Data::Bool { val: a == b }),
        (Data::Bool { val: a }, Data::Bool { val: b }) => Ok(Data::Bool { val: a == b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn not_equal(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Bool { val: a != b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Bool { val: a != b }),
        (Data::Str { val: a }, Data::Str { val: b }) => Ok(Data::Bool { val: a != b }),
        (Data::Char { val: a }, Data::Char { val: b }) => Ok(Data::Bool { val: a != b }),
        (Data::Bool { val: a }, Data::Bool { val: b }) => Ok(Data::Bool { val: a != b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn less_than(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Bool { val: a < b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Bool { val: a < b }),
        (Data::Str { val: a }, Data::Str { val: b }) => Ok(Data::Bool { val: a < b }),
        (Data::Char { val: a }, Data::Char { val: b }) => Ok(Data::Bool { val: a < b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn less_than_or_equal(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Bool { val: a <= b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Bool { val: a <= b }),
        (Data::Str { val: a }, Data::Str { val: b }) => Ok(Data::Bool { val: a <= b }),
        (Data::Char { val: a }, Data::Char { val: b }) => Ok(Data::Bool { val: a <= b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn greater_than(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Bool { val: a > b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Bool { val: a > b }),
        (Data::Str { val: a }, Data::Str { val: b }) => Ok(Data::Bool { val: a > b }),
        (Data::Char { val: a }, Data::Char { val: b }) => Ok(Data::Bool { val: a > b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn greater_than_or_equal(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Bool { val: a >= b }),
        (Data::Float64 { val: a }, Data::Float64 { val: b }) => Ok(Data::Bool { val: a >= b }),
        (Data::Str { val: a }, Data::Str { val: b }) => Ok(Data::Bool { val: a >= b }),
        (Data::Char { val: a }, Data::Char { val: b }) => Ok(Data::Bool { val: a >= b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn and(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Bool { val: a }, Data::Bool { val: b }) => Ok(Data::Bool { val: a && b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn or(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Bool { val: a }, Data::Bool { val: b }) => Ok(Data::Bool { val: a || b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn bitwise_and(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Int32 { val: a & b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn bitwise_or(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Int32 { val: a | b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn bitwise_xor(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) => Ok(Data::Int32 { val: a ^ b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn bitwise_left_shift(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) if b >= 0 => Ok(Data::Int32 { val: a << b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

pub fn bitwise_right_shift(a: Data, b: Data) -> Result<Data, String> {
    match (a, b) {
        (Data::Int32 { val: a }, Data::Int32 { val: b }) if b >= 0 => Ok(Data::Int32 { val: a >> b }),
        (a, b) => Err(type_mismatch(&a, &b)),
    }
}

#[inline(always)]
fn type_mismatch(a: &Data, b: &Data) -> String {
    format!("Type mismatch, [ {:?} | {:?} ]", a.get_type(), b.get_type())
}