pub(crate) mod token;

pub mod interpreter;
pub mod lexer;
pub mod parser;

pub fn run(source: &str) -> Result<(), String> {
    // let tokens = lexer::Lexer::lex(source)?;
    // let mut parser = parser::Parser::new(&tokens);
    // let ast = parser.parse()?;

    // println!("{}", ast);

    Ok(())
}

#[test]
fn test_run() {
    let source = "
    let x = 10;
    let y = 20;
    ";
    // let z = x + y;

    let res = run(source);

    println!("{:?}", res);
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SourceLocation {
    line: usize,
    column: usize,
    file: smol_str::SmolStr,
}
