use crate::lexer::{self, Token};

// TODO: Proper errors and code cleanup

#[derive(Debug)]
pub(crate) struct Literal {
    pub data: Option<crate::Data>,
    pub type_: crate::DataType,
}

impl Literal {
    pub fn new(type_: crate::DataType) -> Literal {
        Literal { data: None, type_ }
    }
}

impl<'a> Literal {
    pub fn set_data_from_str(&mut self, data_str: &'a str) {
        match self.type_ {
            crate::DataType::Int32 => {
                self.data = Some(crate::Data::Int32(
                    data_str.parse::<i32>().expect("Failed to parse int"),
                ));
            }
            crate::DataType::Float64 => {
                self.data = Some(crate::Data::Float64(
                    data_str.parse::<f64>().expect("Failed to parse float"),
                ));
            }
            crate::DataType::Str => {
                self.data = Some(crate::Data::Str(Box::new(data_str.to_owned())));
            }
            crate::DataType::Char => {
                self.data = Some(crate::Data::Char(
                    data_str.chars().next().expect("Failed to parse char"),
                ));
            }
            crate::DataType::Bool => {
                panic!(
                    "Bool literals should be set directly as lexer::Literal::Bool is of type bool"
                )
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum Statement {
    Let {
        identifier: String,
        type_: crate::DataType,
        expr: Box<AstNode>,
    },
    If,
    Else,
    Return,
}

fn binding_power(operator: &crate::Operator) -> (u8, u8) {
    match operator {
        // infix
        crate::Operator::Plus | crate::Operator::Minus => (3, 4),
        crate::Operator::Asterisk | crate::Operator::Slash => (5, 6),
        crate::Operator::Caret => (7, 8),

        // prefix
        crate::Operator::Not => (0, 7),
        crate::Operator::Identity => (0, 7),
        crate::Operator::Negation => (0, 7),
    }
}

fn is_prefix_operator(operator: &crate::Operator) -> bool {
    match operator {
        crate::Operator::Not | crate::Operator::Identity | crate::Operator::Negation => true,
        _ => false,
    }
}

#[derive(Debug)]
pub(crate) enum Expression {
    Identifier(String),
    Literal(Literal),
    Prefix {
        operator: crate::Operator,
        right: Box<AstNode>,
    },
    Postfix {
        left: Box<AstNode>,
        operator: crate::Operator,
    },
    Infix {
        left: Box<AstNode>,
        operator: crate::Operator,
        right: Box<AstNode>,
    },
    Call {
        // TODO: Optimization: Lexer immediately registers functions, replace string with function "pointer"
        function: String,
        arguments: Vec<AstNode>,
    },
}

#[derive(Debug)]
pub(crate) enum AstNode {
    Expression(Expression),

    Statement(Statement),
}

#[derive(Debug)]
pub struct Ast {
    pub(crate) root: Vec<AstNode>,
}

impl Ast {
    pub const fn new() -> Ast {
        Ast { root: Vec::new() }
    }

    fn print_recursive(&self, node: &AstNode, indent: String, is_last: bool) {
        match node {
            AstNode::Expression(expr) => match expr {
                Expression::Identifier(identifier) => {
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
                Expression::Call {
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

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.root {
            self.print_recursive(node, String::from(""), false);
        }

        Ok(())
    }
}

static EMPTY_TOKENS: lexer::Tokens = lexer::Tokens { vec: Vec::new() };

pub struct Parser<'a> {
    ast: Ast,
    tokens: &'a lexer::Tokens<'a>,
    current_token: usize,
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser {
            ast: Ast::new(),
            tokens: &EMPTY_TOKENS,
            current_token: 0,
        }
    }

    pub fn set_tokens(&mut self, tokens: &'a lexer::Tokens) {
        self.tokens = tokens;
    }

    pub fn get_ast(&self) -> &Ast {
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

        while self.current_token < self.tokens.vec.len() {
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

    fn peek(&self) -> &lexer::Token {
        return &self.tokens.vec[self.current_token + 1];
    }

    fn parse_token(&mut self) -> Result<(), String> {
        // Used for both main and block statements
        let parse_res = match self.tokens.vec[self.current_token] {
            lexer::Token::Let => self.parse_let(),
            lexer::Token::Eof => Ok(()),
            _ => Err(format!(
                "[E001] Unexpected token: {:?}",
                self.tokens.vec[self.current_token]
            )),
        };

        return parse_res;
    }

    fn parse_block(&mut self) -> Result<(), String> {
        let mut block: Vec<AstNode> = Vec::new();

        // iterate until we find a closing brace
        while self.tokens.vec[self.current_token] != lexer::Token::RBrace {
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
        end_token: &Token,
        mut min_bp: Option<u8>,
    ) -> Result<Box<AstNode>, String> {
        // Current token is the first token of the expression
        self.current_token += 1;
        let mut lhs: Box<AstNode>;

        match self.tokens.vec[self.current_token] {
            Token::Operator(op) => {
                // starts with an operator, so it's a prefix expression
                if is_prefix_operator(&op) {
                    let r_bp = binding_power(&op).1;
                    let right = self.parse_expr(end_token, Some(r_bp))?;
                    lhs = Box::new(AstNode::Expression(Expression::Prefix {
                        operator: op,
                        right,
                    }));
                } else {
                    return Err("[Esmth] Expected prefix operator".to_string());
                }
            }
            _ => {
                lhs = match &self.tokens.vec[self.current_token] {
                    lexer::Token::Ident(ident) => Box::new(AstNode::Expression(
                        Expression::Identifier((*ident).to_owned()),
                    )),
                    lexer::Token::Literal(lit) => {
                        use lexer::Literal as LexerLiteral;

                        match lit {
                            LexerLiteral::Int(data) => {
                                let mut literal = Literal::new(crate::DataType::Int32);
                                literal.set_data_from_str(*data);
                                Box::new(AstNode::Expression(Expression::Literal(literal)))
                            }
                            LexerLiteral::Float(data) => {
                                let mut literal = Literal::new(crate::DataType::Float64);
                                literal.set_data_from_str(*data);
                                Box::new(AstNode::Expression(Expression::Literal(literal)))
                            }
                            LexerLiteral::Str(data) => {
                                let mut literal = Literal::new(crate::DataType::Str);
                                literal.set_data_from_str(*data);
                                Box::new(AstNode::Expression(Expression::Literal(literal)))
                            }
                            LexerLiteral::Char(data) => {
                                let mut literal = Literal::new(crate::DataType::Char);
                                literal.set_data_from_str(*data);
                                Box::new(AstNode::Expression(Expression::Literal(literal)))
                            }
                            LexerLiteral::Bool(data) => {
                                let mut literal = Literal::new(crate::DataType::Bool);
                                literal.data = Some(crate::Data::Bool(*data));
                                Box::new(AstNode::Expression(Expression::Literal(literal)))
                            }
                        }
                    }
                    lexer::Token::LParen => {
                        let expr = self.parse_expr(&lexer::Token::RParen, None)?;
                        self.current_token += 1;
                        expr
                    }
                    _ => {
                        return Err(format!(
                            "[E002] Unexpected token: {:?}",
                            self.tokens.vec[self.current_token]
                        ))
                    }
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
                lexer::Token::Ident(type_) => {
                    todo!("Custom types are not yet supported");
                    // TODO: check if type exists
                }
                lexer::Token::Type(type_) => {
                    let type_index = crate::BUILT_IN_TYPES
                        .iter()
                        .position(|&t| t == *type_);

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
        self.current_token += 1;
        if self.tokens.vec[self.current_token] != lexer::Token::Semicolon {
            return Err("[E015] Expected semicolon after expression".to_string());
        }

        // Add let statement to ast
        self.ast.root.push(AstNode::Statement(Statement::Let {
            identifier: ident.to_owned(),
            type_: type_,
            expr,
        }));

        Ok(())
    }
}
