// Benchmark macro, which runs a script and reports the time taken in each stage

macro_rules! define_benchmark {
    ($name:ident, $code:expr) => {
        use criterion::{criterion_group, criterion_main, Criterion};
        use ahlang::{*, lexer::Tokens, parser::Ast};
        fn lexer_bench(script_: &str) {
            let mut l = Lexer::new();
            l.set_input(script_);
            let _tokens = l.tokenize().unwrap();
        }

        fn parser_bench(tokens: &Tokens) {
            let mut p = Parser::new();
            p.set_tokens(tokens);
            let _ast = p.parse().unwrap();
        }

        fn eval_bench(ast: &Ast) {
            let mut e = Interpreter::new();
            e.run(ast).unwrap();
        }

        fn $name(c: &mut Criterion) {
            c.bench_function("lexer", |b| b.iter(|| lexer_bench($code)));

            let mut lexer = Lexer::new();
            lexer.set_input($code);
            let tokens = lexer.tokenize().unwrap();

            c.bench_function("parser", |b| b.iter(|| parser_bench(&tokens)));

            let mut parser = Parser::new();
            parser.set_tokens(&tokens);
            let ast = parser.parse().unwrap();

            c.bench_function("eval", |b| b.iter(|| eval_bench(&ast)));
        }

        criterion_group!(benches, $name);
        criterion_main!(benches);
    };
}
