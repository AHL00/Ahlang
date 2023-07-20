mod lexer;
mod parser;
mod interpreter;

fn main() {
    let test_input = r#"
    let y: i32 = 12;
    let test: str = "Hello";
    let x: bool = true;
    let chr: char = 'c';
    let z: str = (50000 + 50000) * 2;
    let test: f64 = 12 * (12.4);
"#;

    let start = std::time::Instant::now();

    let mut l = lexer::Lexer::new(test_input);
    l.tokenize().expect("Failed to tokenize");
    
    let mut p = parser::Parser::new(l.get_tokens());
    let res = p.parse();

    let parse_end = std::time::Instant::now();

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

    println!("Lexing and parsing time: {:?}", parse_end - start);

    let i_start = std::time::Instant::now();

    let mut i = interpreter::Interpreter::new(p.get_ast());
    i.run();

    println!("\n\nInterpreter time: {:?}", std::time::Instant::now() - i_start);
    println!("Variables: {:?}", i.vars);
}
