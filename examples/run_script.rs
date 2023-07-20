use ahlang::*;

static script: &'static str = r#"
let a: i32 = 10;
let b: i32 = -20 + a;
"#;

fn main() {
    let start = std::time::Instant::now();
    let mut l = Lexer::new();
    l.set_input(script.to_string());
    let tokens = l.tokenize().unwrap();

    let mut p = Parser::new();
    p.set_tokens(tokens);
    let ast = p.parse().unwrap();
    let end = std::time::Instant::now();
    println!("Time: {:?}", end - start);

    let mut i = Interpreter::new();
    match i.run(ast) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}