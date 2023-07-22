#[macro_use]
mod bencher;

const SCRIPT: &str = r#"
let a: i32 = 0;
while a < 1000000 {
    a = a + 1;
}
"#;

define_benchmark!(while_iter, SCRIPT);