#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Intent,
    Manifest,
    Identifier(String),
    StringLiteral(String),
    EOF,
}

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.current_char = lexer.input.chars().nth(0);
        lexer
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.chars().nth(self.position);
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> String {
        let mut result = String::new();
        self.advance(); // Skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip closing quote
                break;
            }
            result.push(ch);
            self.advance();
        }
        result
    }

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        match self.current_char {
            None => Token::EOF,
            Some('"') => {
                let string_val = self.read_string();
                Token::StringLiteral(string_val)
            }
            Some(ch) if ch.is_alphabetic() => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "INTENT" => Token::Intent,
                    "MANIFEST" => Token::Manifest,
                    _ => Token::Identifier(identifier),
                }
            }
            _ => {
                self.advance();
                self.next_token()
            }
        }
    }
}