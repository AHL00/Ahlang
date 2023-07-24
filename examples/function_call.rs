mod runner;

static SCRIPT: &str = r#"

fn main(hello: i32, world: i32) {
    let x: i32 = 0;
}
"#;

fn main() {
    runner::run(SCRIPT, true);
}