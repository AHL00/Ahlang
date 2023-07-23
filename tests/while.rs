#[macro_use]
mod tester;

#[test]
fn while_iter() {
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

#[test]
fn while_iter_nested_if() {
    use ahlang::Data;
    test! {
        r#"
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
        "#,
        [("x", Data::Int32 { val: 100 }), ("y", Data::Bool { val: false }), ("iters", Data::Int32 { val: 200 })]
    }
}
