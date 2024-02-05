use crate::lexer::{self, Token};
use crate::{Data, DataType, FnArg};

#[derive(Debug, PartialEq, Clone)]
struct ParserError {
    line: usize,
    column: usize,
    message: String,
}

impl ParserError {
    fn new(line: usize, column: usize, message: String) -> Self {
        Self {
            line,
            column,
            message,
        }
    }
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error at line {} column {}: {}", self.line, self.column, self.message)
    }
}



pub struct Parser<'a> {
    current: usize,
}

impl<'a> Parser<'a> {
    
}

