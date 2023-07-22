use crate::parser::{AstNode, Expression, Statement, Ast};
use crate::{parser, Data, Operator};
use std::collections::HashMap;

mod opers;

#[derive(Debug)]
pub(crate) struct Var {
    pub data: Data,
    pub type_: crate::DataType,
    pub mut_: bool,
}

pub struct Interpreter {
    pub(crate) vars: HashMap<String, Var>,
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
    fn allocate_var(&mut self, identifier: &String, var: Var) {
        self.vars.insert(identifier.to_owned(), var);
    }

    fn get_var(&self, identifier: &String) -> Option<&Var> {
        self.vars.get(identifier)
    }

    fn get_var_mut(&mut self, identifier: &String) -> Option<&mut Var> {
        self.vars.get_mut(identifier)
    }

    // fn delete_var(&mut self, identifier: &String) {
    //     self.vars.remove(identifier);
    // }

    fn eval_block(&mut self, block: &Box<Ast>) -> Result<(), String> {
        for node in &block.root {
            let res = self.eval_statement(node);
            if res.is_err() {
                return Err(res.unwrap_err());
            }
        }

        Ok(())
    }

    // Recursive for nested statements
    fn eval_statement(&mut self, node: &AstNode) -> Result<(), String> {
        match node {
            AstNode::Statement(stmt) => match stmt {
                Statement::Alloc { identifier, type_, expr, mut_ } => {
                    self.eval_stmt_alloc(identifier, type_, expr, *mut_)?;
                },
                Statement::Assign { identifier, expr } => {
                    self.eval_stmt_assign(identifier, expr)?;
                },
                Statement::If { expr, block } => {
                    self.eval_stmt_if(expr, block)?;
                },
                Statement::None => {
                    // do nothing
                },
                _ => {
                    return Err("[DEV] Statement type not implemented yet".to_string());
                },
            },
            _ => {
                return Err("Expected statement".to_string());
            }
        }
        Ok(())
    }

    fn eval_stmt_if(&mut self, expr: &Box<AstNode>, block: &Box<Ast>) -> Result<(), String> {
        let data = self.eval_expression(expr)?;

        match data {
            Data::Bool(true) => {
                self.eval_block(block)?;
            },
            Data::Bool(false) => {
                // do nothing
            },
            _ => {
                return Err(format!("Expression evaluates to {:?}, should be bool", data.get_type()).to_string());
            }
        }

        Ok(())
    }

    fn eval_stmt_assign(&mut self, identifier: &String, expr: &Box<AstNode>) -> Result<(), String> {
        let data = self.eval_expression(expr)?;

        let var_mut = self.get_var_mut(identifier).ok_or("Variable not found".to_string())?;

        if data.get_type() != var_mut.type_ {
            return Err("Expression type does not match variable type".to_string());
        }

        if !var_mut.mut_ {
            return Err("Can't mutate a constant".to_string());
        }

        var_mut.data = data;

        Ok(())
    }

    //fn eval_stmt_fn_call(&mut self) -> Result<(), String> {}

    fn eval_stmt_alloc(
        &mut self,
        identifier: &String,
        type_: &crate::DataType,
        node: &Box<AstNode>,
        mut_: bool,
    ) -> Result<(), String> {
        // evaluate expression
        let data = self.eval_expression(node)?;

        // check if type matches
        if data.get_type() != *type_ {
            return Err("Expression type does not match variable type".to_string());
        }

        // allocate variable
        self.allocate_var(identifier, Var {
            data,
            type_: type_.clone(),
            mut_,
        });

        Ok(())
    }

    // Recursive for nested expressions
    /// Evaluate, then return the data. Rust has RVO, so this should be fine.
    /// The Data is allocated in the caller's stack frame.
    fn eval_expression(&mut self, node: &AstNode) -> Result<Data, String> {
        match node {
            AstNode::Expression(expr) => match expr {
                Expression::Literal(literal) => {
                    return literal.data.clone().ok_or("Literal has no data".to_owned());
                }
                Expression::Prefix { operator, right } => {
                    return self.eval_prefix_expr(operator, right);
                }
                Expression::Infix {
                    operator,
                    left,
                    right,
                } => {
                    return self.eval_infix_expr(operator, left, right);
                }
                Expression::VarIdentifier(identifier) => {
                    return self
                        .get_var(identifier)
                        .ok_or("Variable not found".to_string())
                        .map(|x| x.data.clone());
                }
                _ => {
                    return Err("Expression type not implemented yet".to_string());
                }
            },
            _ => {
                return Err("Expected expression".to_string());
            }
        };
    }

    fn eval_infix_expr(
        &mut self,
        operator: &crate::Operator,
        left: &Box<AstNode>,
        right: &Box<AstNode>,
    ) -> Result<Data, String> {
        let data: Data;

        // evaluate expressions
        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

        match operator {
            Operator::Plus => {
                data = opers::addition(left, right)?;
            },
            Operator::Minus => {
                data = opers::subtraction(left, right)?;
            },
            Operator::Asterisk => {
                data = opers::multiplication(left, right)?;
            },
            Operator::Slash => {
                data = opers::division(left, right)?;
            },
            Operator::Modulo => {
                data = opers::modulo(left, right)?;
            },
            Operator::Caret => {
                data = opers::power(left, right)?;
            },
            Operator::Equals => {
                data = opers::equals(left, right)?;
            },
            Operator::NotEqual => {
                data = opers::not_equal(left, right)?;
            },
            Operator::LessThan => {
                data = opers::less_than(left, right)?;
            },
            Operator::LessThanOrEqual => {
                data = opers::less_than_or_equal(left, right)?;
            },
            Operator::GreaterThan => {
                data = opers::greater_than(left, right)?;
            },
            Operator::GreaterThanOrEqual => {
                data = opers::greater_than_or_equal(left, right)?;
            },
            Operator::And => {
                data = opers::and(left, right)?;
            },
            Operator::Or => {
                data = opers::or(left, right)?;
            },
            Operator::BitwiseAnd => {
                data = opers::bitwise_and(left, right)?;
            },
            Operator::BitwiseOr => {
                data = opers::bitwise_or(left, right)?;
            },
            Operator::LeftShift => {
                data = opers::bitwise_left_shift(left, right)?;
            },
            Operator::RightShift => {
                data = opers::bitwise_right_shift(left, right)?;
            },
            _ => {
                return Err("This operator can't be used as an infix".to_string());
            }
        };

        Ok(data)
    }

    fn eval_prefix_expr(
        &mut self,
        operator: &crate::Operator,
        right: &Box<AstNode>,
    ) -> Result<Data, String> {
        let data: Data;

        // evaluate expression
        let res = self.eval_expression(right)?;

        match operator {
            Operator::Negation => {
                data = opers::negation(res)?;
            }
            Operator::Not => {
                data = opers::not(res)?;
            }
            _ => {
                return Err("This operator can't be used as a prefix".to_string());
            }
        };

        Ok(data)
    }
}
