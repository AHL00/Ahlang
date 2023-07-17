mod lexer;
mod parser;

fn main() {
    let test_input = 
"
let x: int = -2^2 + 2;
";
// let y: int = x;
// let z: int = (-50000 + 50000) * 2;
//let test: float = 12 * (12.4 - x);

    let mut l = lexer::Lexer::new(test_input);
    l.tokenize().expect("Failed to tokenize");

    println!("Source:\n--------");
    println!("{}\n", test_input);

    println!("Tokens:\n--------");
    for token in l.get_tokens() {
        println!("{:?}", token);
    }
    println!("\n");

    let mut p = parser::Parser::new(l.get_tokens());
    let res = p.parse();

    if res.is_err() {
        println!("\n\nParser error: {}", res.unwrap_err());
        println!("\n--------");
        println!("{:?}", p.get_ast());
    } else {
        println!("\n\nParsed successfully!");
        println!("\n{:#?}", p.get_ast());
    }
}