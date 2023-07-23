#[macro_use]
mod tester;

#[test]
fn ajksfj() {
    use ahlang::Data;
    test! {
        r#"
        let x: i32 = 0;
        while x < 10 {
            x = x + 1;
        }
        let y: str = "hello";
        "#,
        [("x", Data::Int32 { val: 10 }), ("y", Data::Str { val: Box::new("hello".to_string()) })]
    }
}
