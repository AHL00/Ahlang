mod lexer;

fn main() {
    let test_input = 
"fn main() {
    let test: int = 50000.00;
}";

    let mut l = lexer::Lexer::new(test_input);
    l.tokenize().expect("Failed to tokenize");

    println!("Source:\n--------");
    println!("{}\n", test_input);

    println!("Tokens:\n--------");
    for token in l.tokens {
        println!("{:?}", token);
    }

}
