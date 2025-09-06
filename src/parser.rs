use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Recall(String),
    BinaryOp {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    IntentDeclaration {
        name: String,
        message: String,
    },
    ManifestCall {
        intent_name: String,
        with_message: Option<String>,
    },
    Calculate {
        name: String,
        expression: Expression,
    },
    Store {
        name: String,
        value: Expression,
    },
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        
        while self.current_token != Token::EOF {
            match self.current_token {
                Token::Intent => {
                    statements.push(self.parse_intent_declaration()?);
                }
                Token::Manifest => {
                    statements.push(self.parse_manifest_call()?);
                }
                Token::Calculate => {
                    statements.push(self.parse_calculate()?);
                }
                Token::Store => {
                    statements.push(self.parse_store()?);
                }
                _ => {
                    return Err(format!("Unexpected token: {:?}", self.current_token));
                }
            }
        }
        
        Ok(statements)
    }

    fn parse_intent_declaration(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip INTENT
        
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected identifier after INTENT"));
        };
        self.advance();
        
        let message = if let Token::StringLiteral(msg) = &self.current_token {
            msg.clone()
        } else {
            return Err(format!("Expected string literal after intent name"));
        };
        self.advance();
        
        Ok(Statement::IntentDeclaration { name, message })
    }

    fn parse_manifest_call(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip MANIFEST
        
        let intent_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected identifier after MANIFEST"));
        };
        self.advance();
        
        let with_message = if self.current_token == Token::With {
            self.advance(); // Skip WITH
            if let Token::StringLiteral(msg) = &self.current_token {
                let message = msg.clone();
                self.advance();
                Some(message)
            } else {
                return Err(format!("Expected string after WITH"));
            }
        } else {
            None
        };
        
        Ok(Statement::ManifestCall { intent_name, with_message })
    }

    fn parse_calculate(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip CALCULATE
        
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected identifier after CALCULATE"));
        };
        self.advance();
        
        let expression = self.parse_expression()?;
        
        Ok(Statement::Calculate { name, expression })
    }

    fn parse_store(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip STORE
        
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected identifier after STORE"));
        };
        self.advance();
        
        let value = self.parse_expression()?;
        
        Ok(Statement::Store { name, value })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_factor()?;
        
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let operator = self.current_token.clone();
            self.advance();
            let right = self.parse_factor()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;
        
        while matches!(self.current_token, Token::Star | Token::Slash) {
            let operator = self.current_token.clone();
            self.advance();
            let right = self.parse_primary()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match &self.current_token {
            Token::Number(n) => {
                let num = *n;
                self.advance();
                Ok(Expression::Number(num))
            }
            Token::Recall => {
                self.advance(); // Skip RECALL
                if let Token::Identifier(name) = &self.current_token {
                    let var_name = name.clone();
                    self.advance();
                    Ok(Expression::Recall(var_name))
                } else {
                    Err(format!("Expected identifier after RECALL"))
                }
            }
            Token::LeftParen => {
                self.advance(); // Skip (
                let expr = self.parse_expression()?;
                if self.current_token != Token::RightParen {
                    return Err(format!("Expected closing parenthesis"));
                }
                self.advance(); // Skip )
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.current_token))
        }
    }
}