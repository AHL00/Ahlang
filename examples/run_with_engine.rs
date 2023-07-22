use ahlang::Engine;

static script: &'static str = r#"
let a: i32 = 10;
let b: i32 = -20;
"#;


fn main() {
    let mut engine = Engine::new();
    match engine.run(script) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}