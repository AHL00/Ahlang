const script: &str = r#"
    let x: i32 = 12 + -3 * -4;
    let y: str = "Hello, " + "world!";
    let z: i32 = x + (12 % 5);
"#;

fn main() {
    let mut start = std::time::Instant::now();

    let mut l = ahlang::Lexer::new();
    l.set_input(script);
    let tokens = l.tokenize().unwrap();

    let lexer_time = std::time::Instant::now() - start;
    start = std::time::Instant::now();

    let mut p = ahlang::Parser::new();
    p.set_tokens(&tokens);

    let parser_time = std::time::Instant::now() - start;
    start = std::time::Instant::now();
    
    let mut i = ahlang::Interpreter::new();
    match i.run(&p.parse().unwrap()) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let run_time = std::time::Instant::now() - start;
    println!("Lexer time: {:?}", lexer_time);
    println!("Parser time: {:?}", parser_time);
    println!("Run time: {:?}", run_time);
    println!(">> Total time: {:?}", lexer_time + parser_time + run_time);
    
    println!();

    println!("Vars: {:#?}", i.vars);

}