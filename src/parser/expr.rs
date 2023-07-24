use super::*;

fn binding_power(operator: &crate::Operator) -> (u8, u8) {
    match operator {
        crate::Operator::Plus | crate::Operator::Minus => (3, 4),

        crate::Operator::Asterisk | crate::Operator::Slash | crate::Operator::Modulo => (5, 6),

        crate::Operator::Caret => (7, 8),

        // Comparison operators
        crate::Operator::LessThan
        | crate::Operator::GreaterThan
        | crate::Operator::LessThanOrEqual
        | crate::Operator::GreaterThanOrEqual
        | crate::Operator::Equals
        | crate::Operator::NotEqual => (1, 2),

        // Logical operators
        crate::Operator::And | crate::Operator::Or => (9, 10),

        // Prefix
        crate::Operator::Not => (0, 7),
        crate::Operator::Identity => (0, 7),
        crate::Operator::Negation => (0, 7),

        _ => {
            panic!("Unknown operator: {:?}", operator);
        }
    }
}

#[derive(Debug)]
pub(crate) enum Expression {
    VarIdentifier(String),
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
    FunctionCall {
        // TODO: Optimization: Lexer immediately registers functions, replace string with function "pointer"
        function: String,
        arguments: Vec<Box<AstNode>>,
    },
}


/// Current token should be the token before the first token of the expression
/// Ends at the token before the end_token
impl<'a> Parser<'a> {
    pub(crate) fn parse_expr(
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
                    lexer::Token::Ident(ident) => {
                        let peek = self.peek();
                        if peek == &lexer::Token::LParen {
                            // Function call
                            self.current_token += 1;
                            let mut arguments: Vec<Box<AstNode>> = Vec::new();

                            loop {
                                self.current_token += 1;
                                if self.tokens.vec[self.current_token] == lexer::Token::RParen {
                                    break;
                                }
                                let expr = self.parse_expr(&lexer::Token::RParen, None)?;
                                arguments.push(expr);
                            }

                            Box::new(AstNode::Expression(Expression::FunctionCall {
                                function: (*ident).to_owned(),
                                arguments,
                            }))
                        } else {
                            Box::new(AstNode::Expression(Expression::VarIdentifier(
                                (*ident).to_owned(),
                            )))
                        }
                    }
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
                                literal.data = Data::Bool { val: *data };
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
                    return Err(format!("[Esmth] Expected operator, got {:?}", token).to_string());
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
}
