use ahlang::*;

fn main() {
    // Binary entry point
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: ahlang <file>");
        return;
    }

    if args[1] == "help" {
        println!("Usage: ahlang <file>");
        println!("Version: ahlang version");
        println!("REPL: ahlang repl");
        return;
    }

    if args[1] == "version" {
        println!("ahlang {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args[1] == "repl" {
        repl();
        return;
    }

    let file = std::fs::read_to_string(&args[1]).unwrap();
    
    let mut l = Lexer::new();
    l.set_input(file);

    let res = l.tokenize();
    let tokens: lexer::Tokens;
    
    if res.is_err() {
        println!("Error: {}", res.unwrap_err());
        return;
    } else {
        tokens = res.unwrap();
    }
    
    let mut p = Parser::new();
    p.set_tokens(tokens);
    let res = p.parse();

    if res.is_err() {
        println!("Error: {}", res.unwrap_err());
        return;
    }

    let mut i = Interpreter::new();
    match i.run(res.unwrap()) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}

fn repl() {
    use std::io::Write;
    
    let mut repl_engine = ReplEngine::new();

    println!("Ahlang REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("Type 'exit' to exit, 'help' for more information");
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if input == "exit" {
            return;
        }

        if input == "help" {
            println!("Type 'exit' to exit, 'help' for more information");
            continue;
        }

        let res = repl_engine.eval(input);

        if res.is_err() {
            println!("{}", res.unwrap_err());
            continue;
        }

    }
}
