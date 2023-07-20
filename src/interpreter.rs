use std::{any::Any, collections::HashMap};

use ahlang::Data;

use crate::parser::{AstNode, Statement};

pub struct Interpreter<'a> {
    ast: &'a crate::parser::Ast<'a>,   
    pub vars: HashMap<&'a str, Data<'a>>
}

impl<'a> Interpreter<'a> {
    pub fn new(ast: &'a crate::parser::Ast<'a>) -> Interpreter<'a> {
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

    fn allocate_var(&mut self, identifier: &'a str, data: Data<'a>) {
        self.vars.insert(identifier, data);
    }

    fn get_var(&self, identifier: &'a str) -> Option<&'a Data> {
        self.vars.get(identifier)
    }

    fn get_var_mut(&mut self, identifier: &'a str) -> Option<&'a mut Data> {
        self.vars.get_mut(identifier)
    }

    fn delete_var(&mut self, identifier: &'a str) {
        self.vars.remove(identifier);
    }

    // Recursive for nested statements
    fn eval_statement(&mut self, node: &AstNode<'a>) -> Result<(), String> {
        match node {
            AstNode::STATEMENT(stmt) => {
                match stmt {
                    Statement::LET {identifier, type_, expr} => {
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

    fn eval_stmt_let(&mut self, identifier: &&'a str, type_: &ahlang::DataType, expr: &Box<AstNode<'a>>) -> Result<(), String> {
        println!("Evaluating LET statement");
        let data: Data<'a>;

        // evaluate expression
        let res = self.eval_expression(expr);
        
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

    fn eval_expression(&mut self, node: &AstNode<'a>) -> Result<Data<'a>, String> {
        // match node {
        //     AstNode::EXPRESSION(expr) => {
        //         match expr {
        //             crate::parser::Expression(expr) => {
        //                 match expr {
                            
        //                     _ => {
        //                         return Err("Expected expression".to_string());
        //                     }
        //                 }
        //             },
        //             _ => {
        //                 return Err("Expected expression".to_string());
        //             }
        //         }
        //     },
        //     _ => {
        //         return Err("Expected expression".to_string());
        //     }
        // }

        // Ok(Box::new(()))
        let data = Data::Int32(12);
        Ok(data)
    }

}