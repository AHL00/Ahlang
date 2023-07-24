use std::collections::HashMap;

use crate::interpreter::{Var, Function};
use crate::lexer::{self, Token};
use crate::{Data, DataType, FnArg};

mod print_ast;
mod expr;
pub(crate) use expr::Expression;

// TODO: Proper errors and code cleanup

#[derive(Debug)]
pub(crate) struct Literal {
    pub data: Data,
    pub type_: crate::DataType,
}

impl Literal {
    pub fn new(type_: crate::DataType) -> Literal {
        Literal {
            data: Data::Empty {},
            type_,
        }
    }
}

impl<'a> Literal {
    pub fn set_data_from_str(&mut self, data_str: &'a str) {
        match self.type_ {
            crate::DataType::Int32 => {
                self.data = Data::Int32 {
                    val: data_str.parse::<i32>().expect("Failed to parse int"),
                };
            }
            crate::DataType::Float64 => {
                self.data = Data::Float64 {
                    val: data_str.parse::<f64>().expect("Failed to parse float"),
                };
            }
            crate::DataType::Str => {
                self.data = Data::Str {
                    val: Box::new(data_str.to_owned()),
                };
            }
            crate::DataType::Char => {
                self.data = Data::Char {
                    val: data_str.chars().next().expect("Failed to parse char"),
                };
            }
            crate::DataType::Bool => {
                panic!(
                    "Bool literals should be set directly as lexer::Literal::Bool is of type bool"
                )
            }
            crate::DataType::Empty => {
                panic!(
                    "Empty literals should be set directly as lexer::Literal::Empty is of type ()"
                )
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum Statement {
    Alloc {
        identifier: String,
        type_: crate::DataType,
        expr: Box<AstNode>,
        mut_: bool,
    },
    Assign {
        identifier: String,
        expr: Box<AstNode>,
    },
    If {
        expr: Box<AstNode>,
        block: Box<Ast>,
    },
    While {
        expr: Box<AstNode>,
        block: Box<Ast>,
    },
    Else,
    Return,
    DefineFn {
        identifier: String,
        arguments: Vec<FnArg>,
        block: Box<Ast>,
    },
    None,
}

fn is_prefix_operator(operator: &crate::Operator) -> bool {
    match operator {
        crate::Operator::Not | crate::Operator::Identity | crate::Operator::Negation => true,
        _ => false,
    }
}

#[derive(Debug)]
pub(crate) enum AstNode {
    Expression(Expression),
    Statement(Statement),
}

/// The var and fn identifiers will be stored as usize's in the ast
/// These refer to the index of the variable in the scope's variables vector
#[derive(Debug)]
pub(crate) struct Scope {
    pub(crate) variables: Vec<Var>,
    pub(crate) functions: Vec<Function>
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn get_var(&self, index: usize) -> &Var {
        &self.variables[index]
    }

    pub fn get_fn(&self, index: usize) -> &Function {
        &self.functions[index]
    }

    pub fn get_var_mut(&mut self, index: usize) -> &mut Var {
        &mut self.variables[index]
    }

    pub fn drop_var(&mut self, index: usize) {
        // set index to empty var
        self.variables[index] = Var {
            data: Data::Empty {},
            type_: crate::DataType::Empty,
            mut_: false,
        };
    }
}

#[derive(Debug)]
pub struct Ast {
    pub(crate) root: Vec<AstNode>,
    pub(crate) scope: Scope,
}

impl Ast {
    pub fn new() -> Ast {
        Ast { root: Vec::new(), scope: Scope::new() }
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.root {
            self.print_recursive(node, String::from(""), false);
        }

        Ok(())
    }
}

static EMPTY_TOKENS: lexer::Tokens = lexer::Tokens { vec: Vec::new() };

pub struct Parser<'a> {
    ast: Box<Ast>,
    tokens: &'a lexer::Tokens<'a>,
    current_token: usize,
}


impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser {
            ast: Box::new(Ast::new()),
            tokens: &EMPTY_TOKENS,
            current_token: 0,
        }
    }

    pub fn set_tokens(&mut self, tokens: &'a lexer::Tokens) {
        self.tokens = tokens;
    }

    pub fn get_ast_ref(&self) -> &Ast {
        &self.ast
    }

    pub fn reset(&mut self) {
        self.ast.root.clear();
        self.tokens = &EMPTY_TOKENS;
        self.current_token = 0;
    }

    /// Parses the tokens into an AST
    pub fn parse(&mut self) -> Result<&Ast, String> {
        if self.tokens.vec.len() == 0 {
            return Err("[E000] No tokens to parse".to_string());
        }

        self.ast = self.parse_block()?;

        Ok(&self.ast)
    }

    fn peek(&self) -> &lexer::Token {
        return &self.tokens.vec[self.current_token + 1];
    }

    /// Starts at left brace and ends *after* right brace or semicolon if included
    /// Used for main script and also blocks
    fn parse_block(&mut self) -> Result<Box<Ast>, String> {
        if self.tokens.vec[self.current_token] != lexer::Token::LBrace {
            return Err("[E009] Expected left brace at start of block".to_string());
        }
        self.current_token += 1;

        let var_id_map: HashMap<String, usize> = HashMap::new();
        let fn_id_map: HashMap<String, usize> = HashMap::new();

        let mut block = Box::new(Ast::new());

        // while not at right brace, self.parse_token()
        while self.tokens.vec[self.current_token] != lexer::Token::RBrace {
            let res = self.parse_token();

            if res.is_err() {
                return Err(res.unwrap_err());
            }

            block.root.push(res.unwrap());
        }

        if self.tokens.vec[self.current_token] != lexer::Token::RBrace {
            return Err("[E008] Expected right brace after if block".to_string());
        }

        self.current_token += 1;

        if self.tokens.vec[self.current_token] == lexer::Token::Semicolon {
            self.current_token += 1;
        }

        Ok(block)
    }

    /// Should end after the last token of the statement, including the semicolon
    fn parse_token(&mut self) -> Result<AstNode, String> {
        // Used for both main and block statements
        match self.tokens.vec[self.current_token] {
            lexer::Token::Fn => self.parse_fn_def(),
            lexer::Token::Let => self.parse_alloc(true),
            lexer::Token::Const => self.parse_alloc(false),
            lexer::Token::Ident(_) => {
                // check if next token is an assign operator
                if self.peek() == &lexer::Token::Assign {
                    self.parse_assign()
                }
                // move to next token
                else {
                    self.current_token += 1;
                    Ok(AstNode::Statement(Statement::None))
                }
            }
            lexer::Token::If => self.parse_if(),
            lexer::Token::While => self.parse_while(),
            // lexer::Token::Eof => {
            //     // End the parser by moving the current token to the end
            //     self.current_token += 1;
            //     Ok(AstNode::Statement(Statement::None))
            // }
            _ => Err(format!(
                "[E001] Unexpected token: {:?}",
                self.tokens.vec[self.current_token]
            )),
        }
    }

    fn parse_fn_def(&mut self) -> Result<AstNode, String> {
        // start with fn
        // ignore and move on
        self.current_token += 1;

        // next token is identifier
        let identifier: String = match &self.tokens.vec[self.current_token] {
            lexer::Token::Ident(ident) => (*ident).to_owned(),
            _ => {
                return Err("[E010] Expected identifier after fn".to_string());
            }
        };

        // next token is left paren
        self.current_token += 1;
        if self.tokens.vec[self.current_token] != lexer::Token::LParen {
            return Err("[E005] Expected left paren after function identifier".to_string());
        }

        let mut args_exist = false;
        if self.peek() != &lexer::Token::RParen {
            args_exist = true;
        }

        let mut arguments = Vec::new();
        if args_exist {
            loop {
                self.current_token += 1;
                // check if next token is right paren

                // next token is identifier
                let ident: String = match &self.tokens.vec[self.current_token] {
                    lexer::Token::Ident(ident) => (*ident).to_owned(),
                    _ => {
                        return Err(format!(
                            "Expected identifier, got {:?}",
                            self.tokens.vec[self.current_token]
                        ));
                    }
                };

                // next token is colon
                self.current_token += 1;
                if self.tokens.vec[self.current_token] != lexer::Token::Colon {
                    return Err("[E015] Expected colon after identifier".to_string());
                }

                // next token is type
                self.current_token += 1;
                let type_: DataType = match &self.tokens.vec[self.current_token] {
                    lexer::Token::Type(type_) => {
                        DataType::from_str(*type_).expect("Failed to parse type")
                    }
                    _ => {
                        return Err("[E012] Expected type after colon".to_string());
                    }
                };

                // if there is a comma, continue
                self.current_token += 1;
                if self.tokens.vec[self.current_token] == lexer::Token::Comma {
                    continue;
                } else if self.tokens.vec[self.current_token] == lexer::Token::RParen {
                    break;
                }

                arguments.push(FnArg {
                    identifier: Box::new(ident),
                    type_,
                });
            }
        }

        // next token is left brace
        self.current_token += 1;
        if self.tokens.vec[self.current_token] != lexer::Token::LBrace {
            return Err("[E007] Expected left brace after right paren".to_string());
        }

        // parse block
        let block = self.parse_block()?;

        // finishes after } or ;
        Ok(AstNode::Statement(Statement::DefineFn {
            identifier,
            arguments,
            block,
        }))
    }

    fn parse_assign(&mut self) -> Result<AstNode, String> {
        // start with identifier
        let ident: &str = match &self.tokens.vec[self.current_token] {
            lexer::Token::Ident(ident) => *ident,
            _ => {
                return Err("[E003] Expected identifier after assign operator".to_string());
            }
        };

        // next token is assign operator
        self.current_token += 1;

        // all the tokens until the semicolon are the expression
        let expr = self.parse_expr(&lexer::Token::Semicolon, None)?;

        // next token is semicolon
        self.expect_semicolon_and_next()?;

        // Add assign statement to ast
        Ok(AstNode::Statement(Statement::Assign {
            identifier: ident.to_owned(),
            expr,
        }))
    }

    fn parse_alloc(&mut self, mut_: bool) -> Result<AstNode, String> {
        // start with let or const
        // ignore and move on
        self.current_token += 1;

        // next token should be identifier
        let ident: String = match &self.tokens.vec[self.current_token] {
            lexer::Token::Ident(ident) => (*ident).to_owned(),
            _ => {
                return Err("[E011] Expected identifier after let".to_string());
            }
        };

        // Next token is colon, parse type
        self.current_token += 1;
        let type_: crate::DataType;

        if self.tokens.vec[self.current_token] == lexer::Token::Colon {
            self.current_token += 1;
            type_ = match &self.tokens.vec[self.current_token] {
                lexer::Token::Ident(_) => {
                    todo!("Custom types are not yet supported");
                    // TODO: check if type exists
                }
                lexer::Token::Type(type_) => {
                    let type_index = crate::BUILT_IN_TYPES.iter().position(|&t| t == *type_);

                    if type_index.is_some() {
                        crate::BUILT_IN_TYPES_DATA_TYPES[type_index.unwrap()]
                    } else {
                        return Err(format!("[E016] Unknown type: {}", type_));
                    }
                }
                _ => {
                    return Err("[E012] Expected type after colon".to_string());
                }
            };
        } else {
            // TODO: infer type if colon is not present
            return Err("[E013] Expected a colon, type inference is not yet supported".to_string());
        }

        // Next token is equals
        self.current_token += 1;
        if self.tokens.vec[self.current_token] != lexer::Token::Assign {
            return Err("[E014] Expected assign operator after type".to_string());
        }

        let expr: Box<AstNode>;

        let res = self.parse_expr(&lexer::Token::Semicolon, None);
        if res.is_err() {
            return Err(res.unwrap_err());
        } else {
            expr = res.unwrap();
        }

        // Next token is semicolon
        self.expect_semicolon_and_next()?;

        Ok(AstNode::Statement(Statement::Alloc {
            identifier: ident.to_owned(),
            type_: type_,
            expr,
            mut_: mut_,
        }))
    }

    fn parse_if(&mut self) -> Result<AstNode, String> {
        // start with if
        // ignore and move on
        self.current_token += 1;

        // next token is expression
        // parse_expr requires the token before the expression
        self.current_token -= 1;
        let expr = self.parse_expr(&lexer::Token::LBrace, None)?;

        // next token is block
        self.current_token += 1;
        let block = self.parse_block()?;

        Ok(AstNode::Statement(Statement::If { expr, block }))
    }

    fn parse_while(&mut self) -> Result<AstNode, String> {
        // start with while
        // ignore and move on
        self.current_token += 1;

        // next token is expression
        // parse_expr requires the token before the expression
        self.current_token -= 1;
        let expr = self.parse_expr(&lexer::Token::LBrace, None)?;

        // next token is block
        self.current_token += 1;
        let block = self.parse_block()?;

        Ok(AstNode::Statement(Statement::While { expr, block }))
    }
    /// Expects the next token to be a semicolon, and moves to the next token
    fn expect_semicolon_and_next(&mut self) -> Result<(), String> {
        self.current_token += 1;
        if self.tokens.vec[self.current_token] != lexer::Token::Semicolon {
            return Err(format!(
                "[E004] Expected semicolon, got: {:?}",
                self.tokens.vec[self.current_token]
            )
            .to_string());
        }
        self.current_token += 1;

        Ok(())
    }
}
