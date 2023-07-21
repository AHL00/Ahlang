use std::collections::HashMap;

use crate::{parser, Data};

use crate::parser::{AstNode, Expression, Statement, Literal};

pub struct Interpreter {
    // TODO: Make vars private
    pub vars: HashMap<String, Data>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            vars: HashMap::new(),
        }
    }

    pub fn run(&mut self, ast: &parser::Ast) -> Result<(), String> {
        if ast.root.len() == 0 {
            return Err("AST is empty".to_string());
        }

        for node in &ast.root {
            let res = self.eval_statement(node);
            if res.is_err() {
                return Err(res.unwrap_err());
            }
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.vars.clear();
    }

    #[inline(always)]
    fn allocate_var(&mut self, identifier: &String, data: Data) {
        self.vars.insert(identifier.to_owned(), data);
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
            AstNode::Statement(stmt) => match stmt {
                Statement::Let {
                    identifier,
                    type_,
                    expr,
                } => {
                    let res = self.eval_stmt_let(identifier, type_, expr);
                    if res.is_err() {
                        return Err(res.unwrap_err());
                    }
                }
                _ => {
                    return Err("saaasdgSf".to_string());
                }
            },
            _ => {
                return Err("Expected statement".to_string());
            }
        }
        Ok(())
    }

    fn eval_stmt_let(
        &mut self,
        identifier: &String,
        type_: &crate::DataType,
        node: &Box<AstNode>,
    ) -> Result<(), String> {
        // evaluate expression
        let data = self.eval_expression(node)?.clone();

        // check if type matches
        if data.get_type() != *type_ {
            return Err("Expression type does not match variable type".to_string());
        }

        // allocate variable
        self.allocate_var(identifier, data);

        Ok(())
    }

    // Recursive for nested expressions
    fn eval_expression(&mut self, node: &AstNode) -> Result<Data, String> {
        match node {
            AstNode::Expression(expr) => match expr {
                Expression::Literal(literal) => {
                    return literal.data.clone().ok_or("Literal has no data".to_owned());
                }
                // Expression::Prefix { operator, right } => {
                //     return self.eval_prefix_expr(operator, right);
                // }
                _ => {
                    return Err("Expression type not implemented yet".to_string());
                }
            },
            _ => {
                return Err("Expected expression".to_string());
            }
        };
    }

    // fn eval_prefix_expr(
    //     &self,
    //     operator: &crate::Operator,
    //     right: &Box<AstNode>,
    // ) -> Result<&Data, String> {
    //     let data: &Data;

    //     // evaluate expression
    //     let res = self.eval_expression(right);

    //     Err("Not implemented".to_string())
    // }
}
