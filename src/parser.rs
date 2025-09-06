use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum Statement {
    IntentDeclaration {
        name: String,
        message: String,
    },
    ManifestCall {
        intent_name: String,
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

    fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current_token))
        }
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
        
        Ok(Statement::ManifestCall { intent_name })
    }
}