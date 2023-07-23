mod runner;

static SCRIPT: &str = r#"

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

fn main() {
    runner::run(SCRIPT, true);
}