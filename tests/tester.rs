macro_rules! test {
    ($script:literal, [$(($var_name:expr, $expected:expr)),*]) => {
        let mut engine = ahlang::Engine::new();
        let _ = engine.run($script).unwrap();

        let vars = engine.interpreter.dbg_get_vars();

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
    };
}
