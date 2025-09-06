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
    Combine {
        name: String,
        parts: Vec<String>,
    },
    Repeat {
        count: Expression,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    Print {
        items: Vec<String>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Increment {
        variable: String,
    },
    Decrement {
        variable: String,
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
                Token::Combine => {
                    statements.push(self.parse_combine()?);
                }
                Token::Repeat => {
                    statements.push(self.parse_repeat()?);
                }
                Token::If => {
                    statements.push(self.parse_if()?);
                }
                Token::Print => {
                    statements.push(self.parse_print()?);
                }
                Token::While => {
                    statements.push(self.parse_while()?);
                }
                Token::Increment => {
                    statements.push(self.parse_increment()?);
                }
                Token::Decrement => {
                    statements.push(self.parse_decrement()?);
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

    fn parse_combine(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip COMBINE
        
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected identifier after COMBINE"));
        };
        self.advance();
        
        let mut parts = Vec::new();
        
        // Parse string literals and identifiers
        while matches!(self.current_token, Token::StringLiteral(_) | Token::Identifier(_)) {
            match &self.current_token {
                Token::StringLiteral(s) => {
                    parts.push(s.clone());
                    self.advance();
                }
                Token::Identifier(id) => {
                    parts.push(format!("${{{}}}", id));
                    self.advance();
                }
                _ => break,
            }
        }
        
        if parts.is_empty() {
            return Err(format!("Expected strings or identifiers after COMBINE name"));
        }
        
        Ok(Statement::Combine { name, parts })
    }

    fn parse_repeat(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip REPEAT
        
        let count = self.parse_expression()?;
        
        if self.current_token != Token::Times {
            return Err(format!("Expected TIMES after repeat count"));
        }
        self.advance(); // Skip TIMES
        
        if self.current_token != Token::Do {
            return Err(format!("Expected DO after TIMES"));
        }
        self.advance(); // Skip DO
        
        let mut body = Vec::new();
        
        while self.current_token != Token::End && self.current_token != Token::EOF {
            match self.current_token {
                Token::Intent => body.push(self.parse_intent_declaration()?),
                Token::Manifest => body.push(self.parse_manifest_call()?),
                Token::Calculate => body.push(self.parse_calculate()?),
                Token::Store => body.push(self.parse_store()?),
                Token::Combine => body.push(self.parse_combine()?),
                Token::Print => body.push(self.parse_print()?),
                _ => {
                    return Err(format!("Unexpected token in repeat body: {:?}", self.current_token));
                }
            }
        }
        
        if self.current_token != Token::End {
            return Err(format!("Expected END to close REPEAT"));
        }
        self.advance(); // Skip END
        
        Ok(Statement::Repeat { count, body })
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip IF
        
        let condition = self.parse_expression()?;
        
        if self.current_token != Token::Then {
            return Err(format!("Expected THEN after IF condition"));
        }
        self.advance(); // Skip THEN
        
        let mut then_body = Vec::new();
        
        while self.current_token != Token::Else && self.current_token != Token::End && self.current_token != Token::EOF {
            match self.current_token {
                Token::Intent => then_body.push(self.parse_intent_declaration()?),
                Token::Manifest => then_body.push(self.parse_manifest_call()?),
                Token::Calculate => then_body.push(self.parse_calculate()?),
                Token::Store => then_body.push(self.parse_store()?),
                Token::Combine => then_body.push(self.parse_combine()?),
                Token::Print => then_body.push(self.parse_print()?),
                _ => {
                    return Err(format!("Unexpected token in IF body: {:?}", self.current_token));
                }
            }
        }
        
        let else_body = if self.current_token == Token::Else {
            self.advance(); // Skip ELSE
            let mut else_stmts = Vec::new();
            
            while self.current_token != Token::End && self.current_token != Token::EOF {
                match self.current_token {
                    Token::Intent => else_stmts.push(self.parse_intent_declaration()?),
                    Token::Manifest => else_stmts.push(self.parse_manifest_call()?),
                    Token::Calculate => else_stmts.push(self.parse_calculate()?),
                    Token::Store => else_stmts.push(self.parse_store()?),
                    Token::Combine => else_stmts.push(self.parse_combine()?),
                    Token::If => else_stmts.push(self.parse_if()?),
                    Token::Print => else_stmts.push(self.parse_print()?),
                    _ => {
                        return Err(format!("Unexpected token in ELSE body: {:?}", self.current_token));
                    }
                }
            }
            Some(else_stmts)
        } else {
            None
        };
        
        if self.current_token != Token::End {
            return Err(format!("Expected END to close IF"));
        }
        self.advance(); // Skip END
        
        Ok(Statement::If { condition, then_body, else_body })
    }

    fn parse_print(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip PRINT
        
        let mut items = Vec::new();
        
        // Parse strings and identifiers for printing
        while matches!(self.current_token, Token::StringLiteral(_) | Token::Identifier(_)) {
            match &self.current_token {
                Token::StringLiteral(s) => {
                    items.push(s.clone());
                    self.advance();
                }
                Token::Identifier(id) => {
                    items.push(format!("${{{}}}", id));
                    self.advance();
                }
                _ => break,
            }
        }
        
        if items.is_empty() {
            return Err(format!("Expected items after PRINT"));
        }
        
        Ok(Statement::Print { items })
    }

    fn parse_while(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip WHILE
        
        let condition = self.parse_expression()?;
        
        if self.current_token != Token::Do {
            return Err(format!("Expected DO after WHILE condition"));
        }
        self.advance(); // Skip DO
        
        let mut body = Vec::new();
        
        while self.current_token != Token::End && self.current_token != Token::EOF {
            match self.current_token {
                Token::Intent => body.push(self.parse_intent_declaration()?),
                Token::Manifest => body.push(self.parse_manifest_call()?),
                Token::Calculate => body.push(self.parse_calculate()?),
                Token::Store => body.push(self.parse_store()?),
                Token::Combine => body.push(self.parse_combine()?),
                Token::Print => body.push(self.parse_print()?),
                Token::If => body.push(self.parse_if()?),
                Token::Increment => body.push(self.parse_increment()?),
                Token::Decrement => body.push(self.parse_decrement()?),
                _ => {
                    return Err(format!("Unexpected token in WHILE body: {:?}", self.current_token));
                }
            }
        }
        
        if self.current_token != Token::End {
            return Err(format!("Expected END to close WHILE"));
        }
        self.advance(); // Skip END
        
        Ok(Statement::While { condition, body })
    }

    fn parse_increment(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip INCREMENT
        
        let variable = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected variable name after INCREMENT"));
        };
        self.advance();
        
        Ok(Statement::Increment { variable })
    }

    fn parse_decrement(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip DECREMENT
        
        let variable = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected variable name after DECREMENT"));
        };
        self.advance();
        
        Ok(Statement::Decrement { variable })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_logical_and()?;
        
        while self.current_token == Token::Or {
            let operator = self.current_token.clone();
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_term()?;
        
        while self.current_token == Token::And {
            let operator = self.current_token.clone();
            self.advance();
            let right = self.parse_term()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison()?;
        
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let operator = self.current_token.clone();
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_factor()?;
        
        while matches!(self.current_token, 
            Token::Equal | Token::NotEqual | 
            Token::Less | Token::Greater | 
            Token::LessEqual | Token::GreaterEqual) {
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
        let mut left = self.parse_power()?;
        
        while matches!(self.current_token, Token::Star | Token::Slash | Token::Percent) {
            let operator = self.current_token.clone();
            self.advance();
            let right = self.parse_power()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;
        
        while self.current_token == Token::Power {
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
        // Handle NOT operator
        if self.current_token == Token::Not {
            self.advance();
            let expr = self.parse_primary()?;
            return Ok(Expression::BinaryOp {
                left: Box::new(Expression::Number(0.0)),
                operator: Token::Not,
                right: Box::new(expr),
            });
        }
        
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