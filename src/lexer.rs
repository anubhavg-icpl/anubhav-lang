#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Intent,
    Manifest,
    Calculate,
    With,
    Store,
    Recall,
    Combine,
    Repeat,
    Times,
    Do,
    End,
    If,
    Then,
    Else,
    Print,
    While,
    Increment,
    Decrement,
    For,
    To,
    Step,
    Min,
    Max,
    Identifier(String),
    StringLiteral(String),
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    LeftParen,
    RightParen,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
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
            } else if ch == '#' {
                // Skip comment line
                while let Some(c) = self.current_char {
                    if c == '\n' {
                        self.advance();
                        break;
                    }
                    self.advance();
                }
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

    fn read_number(&mut self) -> f64 {
        let mut result = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_numeric() || ch == '.' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        result.parse().unwrap_or(0.0)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        match self.current_char {
            None => Token::EOF,
            Some('"') => {
                let string_val = self.read_string();
                Token::StringLiteral(string_val)
            }
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('*') => {
                self.advance();
                if self.current_char == Some('*') {
                    self.advance();
                    Token::Power
                } else {
                    Token::Star
                }
            }
            Some('/') => {
                self.advance();
                Token::Slash
            }
            Some('%') => {
                self.advance();
                Token::Percent
            }
            Some('=') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::Equal
                } else {
                    Token::Equal
                }
            }
            Some('!') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::NotEqual
                } else {
                    self.next_token()
                }
            }
            Some('<') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            Some('>') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            Some('(') => {
                self.advance();
                Token::LeftParen
            }
            Some(')') => {
                self.advance();
                Token::RightParen
            }
            Some(ch) if ch.is_numeric() => {
                let num = self.read_number();
                Token::Number(num)
            }
            Some(ch) if ch.is_alphabetic() => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "INTENT" => Token::Intent,
                    "MANIFEST" => Token::Manifest,
                    "CALCULATE" => Token::Calculate,
                    "WITH" => Token::With,
                    "STORE" => Token::Store,
                    "RECALL" => Token::Recall,
                    "COMBINE" => Token::Combine,
                    "REPEAT" => Token::Repeat,
                    "TIMES" => Token::Times,
                    "DO" => Token::Do,
                    "END" => Token::End,
                    "IF" => Token::If,
                    "THEN" => Token::Then,
                    "ELSE" => Token::Else,
                    "AND" => Token::And,
                    "OR" => Token::Or,
                    "NOT" => Token::Not,
                    "PRINT" => Token::Print,
                    "WHILE" => Token::While,
                    "INCREMENT" => Token::Increment,
                    "DECREMENT" => Token::Decrement,
                    "FOR" => Token::For,
                    "TO" => Token::To,
                    "STEP" => Token::Step,
                    "MIN" => Token::Min,
                    "MAX" => Token::Max,
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