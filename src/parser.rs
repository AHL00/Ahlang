use std::collections::HashMap;

use crate::lexer::{self, Token};

#[derive(Debug)]
pub enum Literal<'a> {
    INT(i32),
    FLOAT(f64),
    STRING(&'a str),
    BOOL(bool),
}

#[derive(Debug)]
pub enum Statement<'a> {
    LET {
        identifier: &'a str,
        type_: &'a str,
        built_in_type: bool,
        value: Box<AstNode<'a>>,
    },
    IF,
    ELSE,
    RETURN,
}

fn binding_power(operator: &lexer::Operator) -> (u8, u8) {
    match operator {
        // infix
        lexer::Operator::PLUS | lexer::Operator::MINUS => (3, 4),
        lexer::Operator::ASTERISK | lexer::Operator::SLASH => (5, 6),
        lexer::Operator::CARET => (7, 8),

        // prefix
        lexer::Operator::NOT => (0, 7),
        lexer::Operator::IDENTITY => (0, 7),
        lexer::Operator::NEGATION => (0, 7),
    }
}

fn is_prefix_operator(operator: &lexer::Operator) -> bool {
    match operator {
        lexer::Operator::NOT | lexer::Operator::IDENTITY | lexer::Operator::NEGATION => true,
        _ => false,
    }
}

#[derive(Debug)]
pub enum Expression<'a> {
    IDENTIFIER(&'a str),
    LITERAL(Literal<'a>),
    PREFIX {
        operator: lexer::Operator,
        right: Box<AstNode<'a>>,
    },
    POSTFIX {
        left: Box<AstNode<'a>>,
        operator: lexer::Operator,
    },
    INFIX {
        left: Box<AstNode<'a>>,
        operator: lexer::Operator,
        right: Box<AstNode<'a>>,
    },
    CALL {
        function: &'a str,
        arguments: Vec<AstNode<'a>>,
    },
}

#[derive(Debug)]
pub enum AstNode<'a> {
    EXPRESSION(Expression<'a>),

    STATEMENT(Statement<'a>),
}

#[derive(Debug)]
pub struct Ast<'a> {
    pub root: Vec<AstNode<'a>>,
}

impl<'a> Ast<'a> {
    fn new(tokens: &'a Vec<lexer::Token<'a>>) -> Ast<'a> {
        Ast { root: Vec::new() }
    }
}

pub struct Parser<'a> {
    tokens: &'a Vec<lexer::Token<'a>>,
    ast: Ast<'a>,
    current_token: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<lexer::Token<'a>>) -> Parser<'a> {
        Parser {
            tokens,
            ast: Ast::new(tokens),
            current_token: 0,
        }
    }

    pub fn get_ast(&self) -> &Ast<'a> {
        &self.ast
    }

    pub fn parse(&mut self) -> Result<&Ast<'a>, String> {
        while self.current_token < self.tokens.len() {
            let res = self.parse_token();

            if res.is_err() {
                return Err(res.unwrap_err());
            }

            // Go to next token, parse_{} functions should end
            // on the last token of their respective statements
            self.current_token += 1;
        }

        Ok(&self.ast)
    }

    fn peek(&self) -> &lexer::Token<'a> {
        return &self.tokens[self.current_token + 1];
    }

    fn parse_token(&mut self) -> Result<(), String> {
        // Used for both main and block statements
        let parse_res = match self.tokens[self.current_token] {
            lexer::Token::LET => self.parse_let(),
            lexer::Token::EOF => Ok(()),
            _ => Err(format!(
                "[E001] Unexpected token: {:?}",
                self.tokens[self.current_token]
            )),
        };

        return parse_res;
    }

    fn parse_block(&mut self) -> Result<(), String> {
        let mut block: Vec<AstNode<'a>> = Vec::new();

        // iterate until we find a closing brace
        while self.tokens[self.current_token] != lexer::Token::RBRACE {
            let res = self.parse_token();

            if res.is_err() {
                return Err(res.unwrap_err());
            }

            // Go to next token, parse_{} functions should end
            // on the last token of their respective statements
            self.current_token += 1;
        }

        Ok(())
    }

    /// Current token should be the token before the first token of the expression
    fn parse_expr(
        &mut self,
        end_token: Token<'a>,
        mut min_bp: Option<u8>,
    ) -> Result<Box<AstNode<'a>>, String> {
        // Current token is the first token of the expression
        self.current_token += 1;
        let mut lhs: Box<AstNode<'a>>;

        match self.tokens[self.current_token] {
            Token::OPERATOR(op) => {
                // starts with an operator, so it's a prefix expression
                if is_prefix_operator(&op) {
                    let r_bp = binding_power(&op).1;
                    let right = self.parse_expr(end_token, Some(r_bp))?;
                    lhs = Box::new(AstNode::EXPRESSION(Expression::PREFIX { operator: op, right }));
                } else {
                    return Err("[Esmth] Expected prefix operator".to_string());
                }
            }
            _ => {
                lhs = match self.tokens[self.current_token] {
                    lexer::Token::IDENT(ident) => {
                        Box::new(AstNode::EXPRESSION(Expression::IDENTIFIER(ident)))
                    }
                    lexer::Token::LITERAL(lit) => Box::new(AstNode::EXPRESSION(Expression::LITERAL(
                        Self::lexer_to_ast_literal(lit),
                    ))),
                    lexer::Token::LPAREN => {
                        let expr = self.parse_expr(lexer::Token::RPAREN, None)?;
                        self.current_token += 1;
                        expr
                    }
                    _ => {return Err(format!(
                        "[E002] Unexpected token: {:?}",
                        self.tokens[self.current_token]
                    ))},
                }
            }
        }
        
    
        if min_bp.is_none() {
            min_bp = Some(0);
        }

        loop {
            // This has to be peek and not current_token += 1 because
            // every time we exit a recursive call, we arrive here.
            // when parsing is done every layer of recursion will
            // arrive at the end_token and break their loops
            let op = match *self.peek() {
                lexer::Token::OPERATOR(op) => op,
                token => {
                    if token == end_token {
                        break;
                    } else if token == lexer::Token::EOF {
                        return Err("[Esmth] Unexpected EOF".to_string());
                    }
                    return Err("[Esmth] Expected operator".to_string());
                }
            };

            let (l_bp, r_bp) = binding_power(&op);

            if l_bp < min_bp.unwrap() {
                break;
            }

            self.current_token += 1;

            let right = self.parse_expr(end_token, Some(r_bp));

            if right.is_err() {
                return Err(right.unwrap_err());
            }

            let right = right.unwrap();

            lhs = Box::new(AstNode::EXPRESSION(Expression::INFIX {
                left: lhs,
                operator: op,
                right,
            }));
        }

        return Ok(lhs);
    }

    fn lexer_to_ast_literal(literal: lexer::LexerLiteral<'a>) -> Literal<'a> {
        match literal {
            lexer::LexerLiteral::INT(int) => {
                let int = int.replace("_", "");
                Literal::INT(int.parse::<i32>().expect("Failed to parse int"))
            }
            lexer::LexerLiteral::FLOAT(float) => {
                let float = float.replace("_", "");
                Literal::FLOAT(float.parse::<f64>().expect("Failed to parse float"))
            }
        }
    }

    fn parse_let(&mut self) -> Result<(), String> {
        // start with let
        // do nothing and move on

        // next token should be identifier
        self.current_token += 1;
        let ident: &str = match self.tokens[self.current_token] {
            lexer::Token::IDENT(ident) => ident,
            _ => {
                return Err("[E011] Expected identifier after let".to_string());
            }
        };

        // Next token is colon, parse type
        self.current_token += 1;
        let type_: &str;
        let mut built_in_type: bool = false;

        if self.tokens[self.current_token] == lexer::Token::COLON {
            self.current_token += 1;
            type_ = match self.tokens[self.current_token] {
                lexer::Token::IDENT(type_) => type_,
                lexer::Token::BUILT_IN_TYPE(type_) => {
                    built_in_type = true;
                    type_
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
        if self.tokens[self.current_token] != lexer::Token::ASSIGN {
            return Err("[E014] Expected assign operator after type".to_string());
        }

        // Possible expressions:
        // let x: int = 1; <- literal
        // let x: int = y; <- identifier
        // let x: int = 1 + 2; <- binary expression
        // let x: int; <- uninitialized variable

        let expr: Box<AstNode<'a>>;

        let res = self.parse_expr(lexer::Token::SEMICOLON, None);
        if res.is_err() {
            return Err(res.unwrap_err());
        } else {
            expr = res.unwrap();
            println!("{:?}", expr);
        }

        // Next token is semicolon
        self.current_token += 1;
        if self.tokens[self.current_token] != lexer::Token::SEMICOLON {
            return Err("[E015] Expected semicolon after expression".to_string());
        }

        // Add let statement to ast
        self.ast.root.push(AstNode::STATEMENT(Statement::LET {
            identifier: ident,
            type_: type_,
            built_in_type: built_in_type,
            value: expr,
        }));

        Ok(())
    }
}
