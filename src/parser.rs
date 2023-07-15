use crate::lexer;

#[derive(Debug)]
pub enum LiteralType<'a> {
    INT(i32),
    FLOAT(f64),
    STRING(&'a str),
    BOOL(bool),
}

#[derive(Debug)]
pub enum StatementType<'a> {
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

#[derive(Debug)]
pub enum InfixOperator {
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    CARET,
}

fn infix_precedence(operator: &InfixOperator) -> u8 {
    match operator {
        InfixOperator::PLUS => 1,
        InfixOperator::MINUS => 1,
        InfixOperator::ASTERISK => 2,
        InfixOperator::SLASH => 2,
        InfixOperator::CARET => 3,
    }
}

#[derive(Debug)]
pub enum PrefixOperator {
    NOT,
    MINUS,
}

fn prefix_precedence(operator: &PrefixOperator) -> u8 {
    match operator {
        PrefixOperator::NOT => 4,
        PrefixOperator::MINUS => 4,
    }
}

#[derive(Debug)]
pub enum PostfixOperator {
    INCREMENT,
    DECREMENT,
}

#[derive(Debug)]
pub enum ExpressionType<'a> {
    IDENTIFIER(&'a str),
    LITERAL(LiteralType<'a>),
    PREFIX {
        operator: PrefixOperator,
        right: Box<AstNode<'a>>,
    },
    POSTFIX {
        left: Box<AstNode<'a>>,
        operator: PostfixOperator,
    },
    INFIX {
        left: Box<AstNode<'a>>,
        operator: InfixOperator,
        right: Box<AstNode<'a>>,
    },
    CALL {
        function: &'a str,
        arguments: Vec<AstNode<'a>>,
    },
}

#[derive(Debug)]
pub enum AstNode<'a> {
    EXPRESSION(ExpressionType<'a>),

    STATEMENT(StatementType<'a>),
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

    pub fn ast_to_string(&self) -> String {
        // TODO: Implement
        String::new()
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

    /// Current token should be the first token of an expression
    fn parse_expr(&mut self) -> Result<Box<AstNode<'a>>, String> {
        // Current token is the first token of the expression
        let mut expr: Option<Box<AstNode<'a>>> = None;

        // // Find the distance to the next semicolon or RPAREN than concerns this expression
        // let mut distance_to_semicolon: usize = 0;
        // for token in &self.tokens[self.current_token..] {
        //     if *token == lexer::Token::SEMICOLON {
        //         break;
        //     }

        //     distance_to_semicolon += 1;
        // }

        // If this is an inner expression, the end is the next RPAREN
        // If the number of LPARENs and RPARENs up to a point is equal, and another RPAREN is found,
        // then the expression has ended
        println!("Current token: {:?} {{\n", &self.tokens[self.current_token]);
        let starting_token = self.current_token;

        let mut distance_to_end: usize = 0;
        let mut lparen_count: usize = 0;
        let mut rparen_count: usize = 0;
        println!("Distance count\n-------------------");
        for token in &self.tokens[self.current_token..] {
            println!("Token: {:?}", token);
            if *token == lexer::Token::LPAREN {
                lparen_count += 1;
            } else if *token == lexer::Token::RPAREN {
                if lparen_count == rparen_count {
                    break;
                }
                rparen_count += 1;
            }

            if *token == lexer::Token::SEMICOLON {
                break;
            }

            distance_to_end += 1;
        }
        println!("distance to end: {}\n--------------------\n", distance_to_end);


        if distance_to_end == 0 {
            return Err("[E002] Expected an expression".to_string());
        } else if distance_to_end == 1 {
            // If the distance is 1, then the expression is a literal or identifier
            expr = match &self.tokens[self.current_token] {
                lexer::Token::IDENT(ident) => Some(Box::new(AstNode::EXPRESSION(ExpressionType::IDENTIFIER(ident)))),
                lexer::Token::LITERAL(literal) => {
                    use crate::lexer::LexerLiteralType;
                    match literal {
                        LexerLiteralType::FLOAT(f) => {
                            // remove _ from float
                            let f = f.replace("_", "");
                            let expr = ExpressionType::LITERAL(LiteralType::FLOAT(
                                f.parse::<f64>().unwrap(),
                            ));
                            Some(Box::new(AstNode::EXPRESSION(expr)))
                        }
                        LexerLiteralType::INT(i) => {
                            // remove _ from int
                            let i = i.replace("_", "");
                            let expr = ExpressionType::LITERAL(LiteralType::INT(
                                i.parse::<i32>().unwrap(),
                            ));
                            Some(Box::new(AstNode::EXPRESSION(expr)))
                        }
                    }
                }
                _ => {
                    return Err("[E003] Expected an expression".to_string());
                }
            }
        } else if distance_to_end == 2 {
            // Unary expression
            // Current token is the operator
            match &self.tokens[self.current_token] {
                lexer::Token::OPERATOR(op) => {
                    expr = match op {
                        lexer::OperatorType::MINUS => {
                            // Current token is the operator
                            // Next token is the expression
                            self.current_token += 1;
                            let expr = ExpressionType::PREFIX {operator: PrefixOperator::MINUS, right: self.parse_expr()? };
                            Some(Box::new(AstNode::EXPRESSION(expr)))
                        }
                        lexer::OperatorType::NOT => {
                            // Current token is the operator
                            // Next token is the expression
                            self.current_token += 1;
                            let expr = ExpressionType::PREFIX {operator: PrefixOperator::NOT, right: self.parse_expr()? };
                            Some(Box::new(AstNode::EXPRESSION(expr)))
                        }
                        _ => {
                            return Err("[E004] Expected an expression".to_string());
                        }
                    }
                }
                _ => {
                    return Err("[E005] Expected an expression".to_string());
                }
            } 

        } else {
            // Pratt parser

            // Current token is the first token of the expression

            // Order of precedence:
            // B   1. Parentheses and grouping
            // !-  2. Unary prefix operators
            // I   3. Exponentiation
            // DM  4. Multiplication and division
            // AS  5. Addition and subtraction

            // To go top down the AST, start with the last step and work our way up

            // Recursive descent parser
            // Current token is the first token of the expression

            // Pseudo code:
            // expr = parse_expression(precedence=0)
            // while current_token != SEMICOLON:
            //     expr = parse_infix(expr, precedence=0)
            // return expr            

            match &self.tokens[self.current_token] {
                // starts with an operator
                // Unary expression
                lexer::Token::OPERATOR(op) => {
                    expr = match op {
                        lexer::OperatorType::MINUS => {
                            // Current token is the operator
                            // Next token is the expression
                            self.current_token += 1;
                            let expr = ExpressionType::PREFIX {operator: PrefixOperator::MINUS, right: self.parse_expr()? };
                            Some(Box::new(AstNode::EXPRESSION(expr)))
                        }
                        lexer::OperatorType::NOT => {
                            // Current token is the operator
                            // Next token is the expression
                            self.current_token += 1;
                            let expr = ExpressionType::PREFIX {operator: PrefixOperator::NOT, right: self.parse_expr()? };
                            Some(Box::new(AstNode::EXPRESSION(expr)))
                        }
                        _ => {
                            return Err("[E006] Expected an expression".to_string());
                        }
                    }
                }

                // starts with a LPAREN
                // Inner expression
                lexer::Token::LPAREN => {
                    // Current token is the LPAREN
                    // Next token is the expression
                    self.current_token += 1;
                    
                    let inner_expr = self.parse_expr()?;

                    // Next token is the RPAREN
                    // Functions should go to the end of their respective expression
                    self.current_token += 1;

                    expr = Some(inner_expr);
                }

                _ => {
                    return Err("[E009] Expected an expression".to_string());
                }
            }

        }

        println!("}} [{:?}] Current expr: {:?}", self.tokens[starting_token], expr);

        if expr.is_none() {
            return Err("[E010] Failed to parse expression".to_string());
        } else {
            return Ok(expr.unwrap());
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
        if self.tokens[self.current_token] != lexer::Token::OPERATOR(lexer::OperatorType::ASSIGN) {
            return Err("[E014] Expected assign operator after type".to_string());
        }

        // Possible expressions:
        // let x: int = 1; <- literal
        // let x: int = y; <- identifier
        // let x: int = 1 + 2; <- binary expression
        // let x: int; <- uninitialized variable

        self.current_token += 1;
        let expr: Box<AstNode<'a>>;
        let res = self.parse_expr();
        if res.is_err() {
            return Err(res.unwrap_err());
        } else {
            expr = res.unwrap();
        }

        // Next token is semicolon
        self.current_token += 1;
        if self.tokens[self.current_token] != lexer::Token::SEMICOLON {
            return Err("[E015] Expected semicolon after expression".to_string());
        }

        // Add let statement to ast
        self.ast.root.push(AstNode::STATEMENT(StatementType::LET {
            identifier: ident,
            type_: type_,
            built_in_type: built_in_type,
            value: expr,
        }));

        Ok(())
    }
}
