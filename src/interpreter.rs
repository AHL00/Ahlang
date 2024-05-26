
/// Scope represents an environment in which variables and functions can be declared and used.
/// Every time a new scope is entered, the interpreter will add it to the stack of scopes.
/// When a scope is exited, it will be removed from the stack.
struct Scope<'a> {
    pub(crate) variables: Vec<Variable>,
    pub(crate) functions: Vec<()>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

struct Variable {
    name: String,
    value: VariableValue,
    mutable: bool,
}

enum VariableValue {
    I32(i32),
    F32(f32),
    String(String),
    Char(char),
    Boolean(bool),
}