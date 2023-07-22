const script: &str = r#"
    let x: i32 = 12 + -3 * -4;
    const y: str = "Hello, " + "world!";
    let z: i32 = x + (12 % 5);
    x = 512;    
    //y = "Can't mutate constants";
"#;
const debug: bool = false;

fn main() {
    let mut start = std::time::Instant::now();

    let mut l = ahlang::Lexer::new();
    l.set_input(script);
    let tokens = l.tokenize();

    if tokens.is_err() {
        println!("Error: {}", tokens.err().unwrap());
        panic!("Lexer error")
    }

    let tokens = tokens.unwrap();

    if debug {
        println!("Tokens: \n{:#?}", tokens);
    }

    let lexer_time = std::time::Instant::now() - start;
    start = std::time::Instant::now();

    let mut p = ahlang::Parser::new();
    p.set_tokens(&tokens);

    let ast = p.parse();
    if ast.is_err() {
        println!("Tokens: \n{:#?}\n", tokens);
        println!("{}", p.get_ast());
        panic!("Parser error");
    }
    let ast = ast.unwrap();

    if debug {
        println!("{}", &ast);
    }

    let parser_time = std::time::Instant::now() - start;
    start = std::time::Instant::now();
    
    let mut i = ahlang::Interpreter::new();
    match i.run(ast) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let run_time = std::time::Instant::now() - start;

    println!();

    println!("Lexer time: {:?}", lexer_time);
    println!("Parser time: {:?}", parser_time);
    println!("Run time: {:?}", run_time);
    println!(">> Total time: {:?}", lexer_time + parser_time + run_time);
    
    println!();


    println!("Vars: {:#?}", i.vars);

}