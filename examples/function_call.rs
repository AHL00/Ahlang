mod run_script;

static SCRIPT: &str = r#"

let x: i32 = 12;

fn main() {
    print("Hello, world!");
}
"#;

fn main() {
    run_script::run(SCRIPT, true);
}