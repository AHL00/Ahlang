#[macro_use]
mod bencher;

#[macro_use]
mod tester;

const SCRIPT: &str = r#"
let x: i32 = 0;
let y: bool = false;
let iters: i32 = 0;

while x < 100 {
    iters = iters + 1;
    if y {
        x = x + 1;
    }
    y = !y;
}
"#;

benchmark!(while_iter, SCRIPT, [("x", Data::Int32 { val: 100 }), ("y", Data::Bool { val: false }), ("iters", Data::Int32 { val: 200 })]);
