use criterion::{black_box, criterion_group, criterion_main, Criterion};

static short_script: &'static str = r#"
let x: i32 = 12 + -3 * -4;
const y: str = "Hello, " + "world!";
let z: i32 = x + (12 % 5);
x = 512;    
"#;

use ahlang::{*, lexer::Tokens};

fn lexer_bench(script: &str) {
    let mut l = Lexer::new();
    l.set_input(script);
    let tokens = l.tokenize().unwrap();
}

fn parser_bench(tokens: &Tokens) {
    let mut p = Parser::new();
    p.set_tokens(tokens);
    let ast = p.parse().unwrap();
}

fn eval_bench(ast: &parser::Ast) {
    let mut e = Interpreter::new();
    e.run(ast).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    // copy string 250 times
    let mut script = String::new();
    for _ in 0..250 {
        script.push_str(&*short_script);
    }

    c.bench_function("lexer", |b| b.iter(|| lexer_bench(script.as_str())));

    let mut lexer = Lexer::new();
    lexer.set_input(script.as_str());
    let tokens = lexer.tokenize().unwrap();

    c.bench_function("parser", |b| b.iter(|| parser_bench(tokens)));

    let mut parser = Parser::new();
    parser.set_tokens(&tokens);
    let ast = parser.parse().unwrap();

    c.bench_function("eval", |b| b.iter(|| eval_bench(ast)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);