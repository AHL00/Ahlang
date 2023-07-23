// Benchmark macro, which runs a script and reports the time taken in each stage
#[allow(unused_macros)]
macro_rules! benchmark {
    ($name:ident, $code:expr, [$(($var_name:expr, $expected:expr)),*]) => {
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
            
            let mut lexer = Lexer::new();
            lexer.set_input($code);
            let tokens = lexer.tokenize().expect("Lexer failed");
            
            let mut parser = Parser::new();
            parser.set_tokens(&tokens);
            let ast = parser.parse().expect("Parser failed");
            
            let mut interpreter = Interpreter::new();
            interpreter.run(&ast).unwrap();
            
            let vars = interpreter.dbg_get_vars();

            // validate results
            $(
                let actual_value = &vars.iter().find(|(k, _)| k == $var_name).unwrap().1;
                match ($expected, actual_value.clone()) {
                    (ahlang::Data::Int32 {val: expected}, ahlang::Data::Int32 {val: actual}) => {
                        assert_eq!(expected, actual, "Variable {} does not match expected value", $var_name);
                    },
                    (ahlang::Data::Float64 {val: expected}, ahlang::Data::Float64 {val: actual}) => {
                        assert_eq!(expected, actual, "Variable {} does not match expected value", $var_name);
                    },
                    (ahlang::Data::Bool {val: expected}, ahlang::Data::Bool {val: actual}) => {
                        assert_eq!(expected, actual, "Variable {} does not match expected value", $var_name);
                    },
                    (ahlang::Data::Char {val: expected}, ahlang::Data::Char {val: actual}) => {
                        assert_eq!(expected, actual, "Variable {} does not match expected value", $var_name);
                    },
                    (ahlang::Data::Str {val: expected}, ahlang::Data::Str {val: actual}) => {
                        assert_eq!(expected, actual, "Variable {} does not match expected value", $var_name);
                    },
                    _ => {
                        panic!("Variable {} has unexpected type", $var_name);
                    },
                };
            )*


            c.bench_function("lexer", |b| b.iter(|| lexer_bench($code)));

            c.bench_function("parser", |b| b.iter(|| parser_bench(&tokens)));

            c.bench_function("eval", |b| b.iter(|| eval_bench(&ast)));
        }

        criterion_group!(benches, $name);
        criterion_main!(benches);
    };
}
