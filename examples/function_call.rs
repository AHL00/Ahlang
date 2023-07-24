mod runner;

static SCRIPT: &str = r#"
let x: i32 = 12;
fn main(hello: i32, world: i32) {
    let x: i32 = 0;
}
"#;

fn main() {
    runner::run(SCRIPT, true);
}