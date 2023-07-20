use std::collections::HashMap;

use ahlang::Data;

use crate::parser::{AstNode, Statement, Expression};

pub struct Interpreter<'a> {
    ast: &'a crate::parser::Ast,   
    pub vars: HashMap<String, Data>
}

impl<'a> Interpreter<'a> {
    pub fn new(ast: &'a crate::parser::Ast) -> Interpreter<'a> {
        Interpreter {
            ast,
            vars: HashMap::new()
        }
    }

    pub fn run(&mut self) {
        for node in &self.ast.root {
            let res = self.eval_statement(node);
            if res.is_err() {
                println!("Error: {}", res.unwrap_err());
            }
        }
    }

    fn allocate_var(&mut self, identifier: &String, data: &Data) {
        self.vars.insert(identifier.clone(), data.clone());
    }

    fn get_var(&self, identifier: &String) -> Option<&Data> {
        self.vars.get(identifier)
    }

    fn get_var_mut(&mut self, identifier: &String) -> Option<&mut Data> {
        self.vars.get_mut(identifier)
    }

    fn delete_var(&mut self, identifier: &String) {
        self.vars.remove(identifier);
    }

    // Recursive for nested statements
    fn eval_statement(&mut self, node: &AstNode) -> Result<(), String> {
        match node {
            AstNode::Statement(stmt) => {
                match stmt {
                    Statement::Let {identifier, type_, expr} => {
                        let res = self.eval_stmt_let(identifier, type_, expr);
                        if res.is_err() {
                            return Err(res.unwrap_err());
                        }
                    },
                    _ => {
                        return Err("saaasdgSf".to_string());
                    }
                }
            },
            _ => {
                return Err("Expected statement".to_string());}
        }

        

        Ok(())
    }

    fn eval_stmt_let(&mut self, identifier: &String, type_: &ahlang::DataType, expr: &Box<AstNode>) -> Result<(), String> {
        println!("Evaluating LET statement");
        let data: &Data;

        // evaluate expression
        let res = match expr.as_ref() {
            AstNode::Expression(expr) => {
                Interpreter::eval_expression(expr)
            },
            _ => {
                return Err("Expected expression".to_string());
            }
        };
    
        if res.is_err() {
            return Err(res.unwrap_err());
        } else {
            data = res.unwrap();
        }

        // check if type matches
        if data.get_type() != *type_ {
            return Err("Expression type does not match variable type".to_string());
        }

        // allocate variable
        self.allocate_var(identifier, data);

        Ok(())
    }

    // Recursive for nested expressions
    fn eval_expression(expr: &Expression) -> Result<&Data, String> {
        match *expr {
            Expression::Literal(_) => {
                return Interpreter::eval_literal(expr);
            },
            _ => {
                return Err("Expression type not implemented yet".to_string());
            }
        };
    }

    fn eval_literal(literal: &Expression) -> Result<&Data, String> {
        match literal {
            Expression::Literal(literal) => {
                if literal.data.is_none() {
                    return Err("Literal has no data".to_string());
                } else {
                    let data = literal.data.as_ref().unwrap();
                    return Ok(data)
                }
            },
            _ => {
                Err("Expected literal".to_string())
            }
        }
    }

}