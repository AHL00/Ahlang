use super::*;

impl Ast {
    pub(crate) fn print_recursive(&self, node: &AstNode, indent: String, is_last: bool) {
        match node {
            AstNode::Expression(expr) => match expr {
                Expression::VarIdentifier(identifier) => {
                    println!("{}IDENTIFIER({})", &indent, identifier)
                }
                Expression::Literal(literal) => {
                    println!("{}LITERAL", &indent);
                    println!("{}├── type: {:?}", &indent, literal.type_);
                    println!("{}└── data: {:?}", &indent, literal.data);
                }
                Expression::Prefix { operator, right } => {
                    println!("{}PREFIX", &indent);
                    println!("{}├── op: {:?}", &indent, operator);
                    self.print_recursive(right, format!("{}│   ", &indent), true);
                }
                Expression::Postfix { left, operator } => {
                    println!("{}POSTFIX", &indent);
                    self.print_recursive(left, format!("{}├── ", &indent), false);
                    println!("{}└── op: {:?}", &indent, operator);
                }
                Expression::Infix {
                    left,
                    operator,
                    right,
                } => {
                    println!("{}INFIX", &indent);
                    self.print_recursive(left, format!("{}├── ", &indent), false);
                    println!("{}├── op: {:?}", &indent, operator);
                    self.print_recursive(right, format!("{}│   ", &indent), true);
                }
                Expression::FunctionCall {
                    function,
                    arguments,
                } => {
                    println!("{}CALL", &indent);
                    println!("{}├── function: {}", &indent, function);
                    for (i, arg) in arguments.iter().enumerate() {
                        let is_last_arg = i == arguments.len() - 1;
                        if is_last_arg {
                            self.print_recursive(arg, format!("{}└── ", &indent), true);
                        } else {
                            self.print_recursive(arg, format!("{}│   ", &indent), false);
                        }
                    }
                }
            },
            AstNode::Statement(stmt) => match stmt {
                Statement::Alloc {
                    identifier,
                    type_,
                    expr: value,
                    mut_,
                } => {
                    let current_indent = if is_last {
                        indent
                    } else {
                        format!("{}", &indent)
                    };
                    println!("{}ALLOC", &current_indent);
                    println!("{}├── identifier: {}", &current_indent, identifier);
                    println!("{}├── type: {:?}", &current_indent, type_);
                    println!("{}├── mut: {}", &current_indent, mut_);
                    self.print_recursive(value, format!("{}└── ", &current_indent), true);
                }
                Statement::Assign { identifier, expr } => {
                    let current_indent = if is_last {
                        indent
                    } else {
                        format!("{}", &indent)
                    };
                    println!("{}ASSIGN", &current_indent);
                    println!("{}├── identifier: {}", &current_indent, identifier);
                    self.print_recursive(expr, format!("{}└── ", &current_indent), true);
                }
                Statement::If { expr, block } => {
                    let current_indent = if is_last {
                        indent
                    } else {
                        format!("{}", &indent)
                    };
                    println!("{}IF", &current_indent);
                    self.print_recursive(expr, format!("{}├── ", &current_indent), false);
                    println!("{}└── [BLOCK]", &current_indent);
                    for node in &block.root {
                        self.print_recursive(node, format!("{}    ", &current_indent), false);
                    }
                }
                Statement::While { expr, block } => {
                    let current_indent = if is_last {
                        indent
                    } else {
                        format!("{}", &indent)
                    };
                    println!("{}WHILE", &current_indent);
                    self.print_recursive(expr, format!("{}├── ", &current_indent), false);
                    println!("{}└── [BLOCK]", &current_indent);
                    for node in &block.root {
                        self.print_recursive(node, format!("{}    ", &current_indent), false);
                    }
                }
                Statement::DefineFn {
                    identifier,
                    arguments,
                    block,
                } => {
                    let current_indent = if is_last {
                        indent
                    } else {
                        format!("{}", &indent)
                    };
                    println!("{}DEFINE FN", &current_indent);
                    println!("{}├── identifier: {}", &current_indent, identifier);
                    println!("{}├── arguments:", &current_indent);
                    for arg in arguments {
                        println!("{}│   ├── identifier: {}", &current_indent, arg.identifier);
                        println!("{}│   └── type: {:?}", &current_indent, arg.type_);
                    }
                    println!("{}└── [BLOCK]", &current_indent);
                    for node in &block.root {
                        self.print_recursive(node, format!("{}    ", &current_indent), false);
                    }
                }
                Statement::Else => println!("{}ELSE", &indent),
                Statement::Return => println!("{}RETURN", &indent),
                Statement::None => println!("{}NONE", &indent),
            },
        }
    }
}
