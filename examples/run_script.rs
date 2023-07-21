use ahlang::*;

static script: &'static str = r#"
let a: str = "Hello";
"#;

fn main() {
    let start = std::time::Instant::now();

    let mut l = Lexer::new();
    l.set_input(script);
    let tokens = l.tokenize().unwrap();

    let mut p = Parser::new();
    p.set_tokens(&tokens);
    
    let mut i = Interpreter::new();
    match i.run(&p.parse().unwrap()) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let end = std::time::Instant::now();
    println!("Parse time: {:?}", end - start);
    println!("Vars: {:?}", i.vars);
}