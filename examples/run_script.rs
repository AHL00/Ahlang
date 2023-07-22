const script: &str = r#"
    // let x: i32 = 12 + -3 * -4;
    // const y: str = "Hello, " + "world!";
    let x: i32 = 512;
    let z: i32 = x + (12 % 5);

    if x == 512 {
        x = 12;
    }
"#;
const debug: bool = true;

fn main() {
    let mut start = std::time::Instant::now();

    let mut l = ahlang::Lexer::new();
    l.set_input(script);
    let tokens = l.tokenize();

    if tokens.is_err() {
        let mut l2 = ahlang::Lexer::new();
        l2.set_input(script);
        _ = l2.tokenize();
        println!("Tokens: \n");
        println!("{}", *l2.get_tokens());
        panic!("Lexer error: {}", tokens.err().unwrap());
    }

    let tokens = tokens.unwrap();

    if debug {
        println!("Tokens: \n{}", tokens);
    }

    let lexer_time = std::time::Instant::now() - start;
    start = std::time::Instant::now();

    let mut p = ahlang::Parser::new();
    p.set_tokens(&tokens);

    let ast = p.parse();
    if ast.is_err() {
        println!("Tokens: \n");
        
        println!("{}", tokens);

        let mut p2 = ahlang::Parser::new();
        p2.set_tokens(&tokens);
        _ = p2.parse();
        println!("{}", p2.get_ast_ref());
        panic!("Parser error: {}", &ast.err().unwrap());
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