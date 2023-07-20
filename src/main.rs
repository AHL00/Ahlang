mod lexer;
mod parser;
mod interpreter;

fn main() {
    let test_input = "
let y: i32 = x;
let z: i32 = (-50000 + 50000) * 2;
let test: f64 = 12 * (12.4 - x);
";

    let start = std::time::Instant::now();

    let mut l = lexer::Lexer::new(test_input);
    l.tokenize().expect("Failed to tokenize");
    
    let mut p = parser::Parser::new(l.get_tokens());
    let res = p.parse();

    let end = std::time::Instant::now();

    
    println!("Source:\n--------");
    println!("{}\n", test_input);

    println!("Tokens:\n--------");
    for token in l.get_tokens() {
        println!("{:?}", token);
    }
    println!("\n");

    if res.is_err() {
        println!("\n\nParser error: {}", res.unwrap_err());
        println!("\n--------");
        println!("{}", p.get_ast());
    } else {
        println!("\n\nParsed successfully!\n");
        // iterate ast
        println!("AST:\n--------------------");
        println!("{}\n\n", p.get_ast())
    }

    println!("Lexing and parsing time: {:?}", end - start);

    let mut i = interpreter::Interpreter::new(p.get_ast());
    i.run();
}
