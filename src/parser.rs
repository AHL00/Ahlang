use crate::lexer::{self, Token};

// TODO: Proper errors and code cleanup

#[derive(Debug)]
pub struct Literal<'a> {
    data: Option<ahlang::Data<'a>>,
    type_: ahlang::DataType,
}

impl<'a> Literal<'a> {
    pub fn new(type_: ahlang::DataType) -> Literal<'a> {
        Literal { data: None, type_ }
    }

    pub fn set_data_from_str(&mut self, data_str: &'a str) {
        match self.type_ {
            ahlang::DataType::Int32 => {
                self.data = Some(ahlang::Data::Int32(
                    data_str.parse::<i32>().expect("Failed to parse int"),
                ));
            }
            ahlang::DataType::Float64 => {
                self.data = Some(ahlang::Data::Float64(
                    data_str.parse::<f64>().expect("Failed to parse float"),
                ));
            }
            ahlang::DataType::Str => {
                self.data = Some(ahlang::Data::Str(data_str));
            }
            ahlang::DataType::Bool => {
                self.data = Some(ahlang::Data::Bool(
                    data_str.parse::<bool>().expect("Failed to parse bool"),
                ));
            }
        }
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    Let {
        identifier: &'a str,
        type_: ahlang::DataType,
        expr: Box<AstNode<'a>>,
    },
    If,
    Else,
    Return,
}

fn binding_power(operator: &ahlang::Operator) -> (u8, u8) {
    match operator {
        // infix
        ahlang::Operator::Plus | ahlang::Operator::Minus => (3, 4),
        ahlang::Operator::Asterisk | ahlang::Operator::Slash => (5, 6),
        ahlang::Operator::Caret => (7, 8),

        // prefix
        ahlang::Operator::Not => (0, 7),
        ahlang::Operator::Identity => (0, 7),
        ahlang::Operator::Negation => (0, 7),
    }
}

fn is_prefix_operator(operator: &ahlang::Operator) -> bool {
    match operator {
        ahlang::Operator::Not | ahlang::Operator::Identity | ahlang::Operator::Negation => true,
        _ => false,
    }
}

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(&'a str),
    Literal(Literal<'a>),
    Prefix {
        operator: ahlang::Operator,
        right: Box<AstNode<'a>>,
    },
    Postfix {
        left: Box<AstNode<'a>>,
        operator: ahlang::Operator,
    },
    Infix {
        left: Box<AstNode<'a>>,
        operator: ahlang::Operator,
        right: Box<AstNode<'a>>,
    },
    Call {
        function: &'a str,
        arguments: Vec<AstNode<'a>>,
    },
}

#[derive(Debug)]
pub enum AstNode<'a> {
    Expression(Expression<'a>),

    Statement(Statement<'a>),
}

#[derive(Debug)]
pub struct Ast<'a> {
    pub root: Vec<AstNode<'a>>,
}

impl<'a> Ast<'a> {
    fn new(tokens: &'a Vec<lexer::Token<'a>>) -> Ast<'a> {
        Ast { root: Vec::new() }
    }

    fn print_recursive(&self, node: &AstNode, indent: String, is_last: bool) {
        match node {
            AstNode::Expression(expr) => match expr {
                Expression::Identifier(identifier) => println!("{}IDENTIFIER({})", &indent, identifier),
                Expression::Literal(literal) => {
                    println!("{}LITERAL", &indent);
                    println!("{}├── type: {:?}", &indent, literal.type_);
                    println!("{}└── data: {:?}", &indent, literal.data);
                },
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
                Expression::Infix { left, operator, right } => {
                    println!("{}INFIX", &indent);
                    self.print_recursive(left, format!("{}├── ", &indent), false);
                    println!("{}├── op: {:?}", &indent, operator);
                    self.print_recursive(right, format!("{}│   ", &indent), true);
                }
                Expression::Call { function, arguments } => {
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
                Statement::Let {
                    identifier,
                    type_,
                    expr: value,
                } => {
                    let current_indent = if is_last {
                        indent
                    } else {
                        format!("{}", &indent)
                    };
                    println!("{}LET", &current_indent);
                    println!("{}├── identifier: {}", &current_indent, identifier);
                    println!("{}├── type: {:?}", &current_indent, type_);
                    self.print_recursive(value, format!("{}└── ", &current_indent), true);
                }
                Statement::If => println!("{}IF", &indent),
                Statement::Else => println!("{}ELSE", &indent),
                Statement::Return => println!("{}RETURN", &indent),
            },
        }
    }
    
}

impl<'a> std::fmt::Display for Ast<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.root {
            self.print_recursive(node, String::from(""), false);
        }

        Ok(())
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
            lexer::Token::Let => self.parse_let(),
            lexer::Token::Eof => Ok(()),
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
        while self.tokens[self.current_token] != lexer::Token::RBrace {
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
        end_token: &Token<'a>,
        mut min_bp: Option<u8>,
    ) -> Result<Box<AstNode<'a>>, String> {
        // Current token is the first token of the expression
        self.current_token += 1;
        let mut lhs: Box<AstNode<'a>>;

        match self.tokens[self.current_token] {
            Token::Operator(op) => {
                // starts with an operator, so it's a prefix expression
                if is_prefix_operator(&op) {
                    let r_bp = binding_power(&op).1;
                    let right = self.parse_expr(end_token, Some(r_bp))?;
                    lhs = Box::new(AstNode::Expression(Expression::Prefix { operator: op, right }));
                } else {
                    return Err("[Esmth] Expected prefix operator".to_string());
                }
            }
            _ => {
                lhs = match self.tokens[self.current_token] {
                    lexer::Token::Ident(ident) => {
                        Box::new(AstNode::Expression(Expression::Identifier(ident)))
                    }
                    lexer::Token::Literal(lit) => {
                        use lexer::Literal as LexerLiteral;

                        match lit {
                            LexerLiteral::Int(data) => {
                                let mut literal = Literal::new(ahlang::DataType::Int32);
                                literal.set_data_from_str(data);
                                Box::new(AstNode::Expression(Expression::Literal(literal)))
                            },
                            LexerLiteral::Float(data) => {
                                let mut literal = Literal::new(ahlang::DataType::Float64);
                                literal.set_data_from_str(data);
                                Box::new(AstNode::Expression(Expression::Literal(literal)))
                            },
                        }
                    },
                    lexer::Token::LParen => {
                        let expr = self.parse_expr(&lexer::Token::RParen, None)?;
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
            let op = match self.peek().clone() {
                lexer::Token::Operator(op) => op,
                token => {
                    if token == *end_token {
                        break;
                    } else if token == lexer::Token::Eof {
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

            lhs = Box::new(AstNode::Expression(Expression::Infix {
                left: lhs,
                operator: op,
                right,
            }));
        }

        return Ok(lhs);
    }

    fn parse_let(&mut self) -> Result<(), String> {
        // start with let
        // do nothing and move on

        // next token should be identifier
        self.current_token += 1;
        let ident: &str = match self.tokens[self.current_token] {
            lexer::Token::Ident(ident) => ident,
            _ => {
                return Err("[E011] Expected identifier after let".to_string());
            }
        };

        // Next token is colon, parse type
        self.current_token += 1;
        let type_: ahlang::DataType;

        if self.tokens[self.current_token] == lexer::Token::Colon {
            self.current_token += 1;
            type_ = match self.tokens[self.current_token] {
                lexer::Token::Ident(type_) => {
                    todo!("Custom types are not yet supported");
                    // TODO: check if type exists
                },
                lexer::Token::Type(type_) => {
                    if ahlang::BUILT_IN_TYPES.contains_key(type_) {
                        ahlang::BUILT_IN_TYPES.get(type_).unwrap().clone()
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
        if self.tokens[self.current_token] != lexer::Token::Assign {
            return Err("[E014] Expected assign operator after type".to_string());
        }

        // Possible expressions:
        // let x: int = 1; <- literal
        // let x: int = y; <- identifier
        // let x: int = 1 + 2; <- binary expression
        // let x: int; <- uninitialized variable

        let expr: Box<AstNode<'a>>;

        let res = self.parse_expr(&lexer::Token::Semicolon, None);
        if res.is_err() {
            return Err(res.unwrap_err());
        } else {
            expr = res.unwrap();
        }

        // Next token is semicolon
        self.current_token += 1;
        if self.tokens[self.current_token] != lexer::Token::Semicolon {
            return Err("[E015] Expected semicolon after expression".to_string());
        }

        // match to phf hashmap ahlang::BUILT_IN_TYPES
        // if not found, it's a user defined type
        // if found, it's a built in type
        let type_ =

        // Add let statement to ast
        self.ast.root.push(AstNode::Statement(Statement::Let {
            identifier: ident,
            type_: type_,
            expr,
        }));

        Ok(())
    }
}