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
    Assert,
    Try,
    Catch,
    Floor,
    Ceil,
    Round,
    Random,
    Length,
    Substring,
    Uppercase,
    Lowercase,
    Contains,
    Switch,
    Case,
    Default,
    Array,
    Push,
    Pop,
    Size,
    Get,
    Set,
    Import,
    Export,
    Break,
    Continue,
    Function,
    Call,
    Return,
    Sort,
    Filter,
    Reverse,
    Map,
    Reduce,
    Sum,
    Join,
    Dict,
    Put,
    Fetch,
    Keys,
    Values,
    ReadFile,
    WriteFile,
    AppendFile,
    Delete,
    Exists,
    Sleep,
    Input,
    Type,
    Parse,
    ToString,
    Lambda,
    Pipe,
    RangeOp,
    Fold,
    FindOp,
    AllOp,
    AnyOp,
    Zip,
    Unzip,
    Flatten,
    Unique,
    CountOp,
    GroupBy,
    Partition,
    TakeOp,
    DropOp,
    SliceOp,
    Concat,
    SplitOp,
    ReplaceOp,
    Trim,
    StartsWith,
    EndsWith,
    Includes,
    IndexOfOp,
    Pad,
    Eval,
    TypeOfOp,
    CloneOp,
    Merge,
    Diff,
    Intersection,
    UnionOp,
    ClearOp,
    SwapOp,
    Shuffle,
    Sample,
    MinOfOp,
    MaxOfOp,
    AverageOp,
    Median,
    ModeOp,
    StdDev,
    Variance,
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
                    "ASSERT" => Token::Assert,
                    "TRY" => Token::Try,
                    "CATCH" => Token::Catch,
                    "FLOOR" => Token::Floor,
                    "CEIL" => Token::Ceil,
                    "ROUND" => Token::Round,
                    "RANDOM" => Token::Random,
                    "LENGTH" => Token::Length,
                    "SUBSTRING" => Token::Substring,
                    "UPPERCASE" => Token::Uppercase,
                    "LOWERCASE" => Token::Lowercase,
                    "CONTAINS" => Token::Contains,
                    "SWITCH" => Token::Switch,
                    "CASE" => Token::Case,
                    "DEFAULT" => Token::Default,
                    "ARRAY" => Token::Array,
                    "PUSH" => Token::Push,
                    "POP" => Token::Pop,
                    "SIZE" => Token::Size,
                    "GET" => Token::Get,
                    "SET" => Token::Set,
                    "IMPORT" => Token::Import,
                    "EXPORT" => Token::Export,
                    "BREAK" => Token::Break,
                    "CONTINUE" => Token::Continue,
                    "FUNCTION" => Token::Function,
                    "CALL" => Token::Call,
                    "RETURN" => Token::Return,
                    "SORT" => Token::Sort,
                    "FILTER" => Token::Filter,
                    "REVERSE" => Token::Reverse,
                    "MAP" => Token::Map,
                    "REDUCE" => Token::Reduce,
                    "SUM" => Token::Sum,
                    "JOIN" => Token::Join,
                    "DICT" => Token::Dict,
                    "PUT" => Token::Put,
                    "FETCH" => Token::Fetch,
                    "KEYS" => Token::Keys,
                    "VALUES" => Token::Values,
                    "READ_FILE" => Token::ReadFile,
                    "WRITE_FILE" => Token::WriteFile,
                    "APPEND_FILE" => Token::AppendFile,
                    "DELETE" => Token::Delete,
                    "EXISTS" => Token::Exists,
                    "SLEEP" => Token::Sleep,
                    "INPUT" => Token::Input,
                    "TYPE" => Token::Type,
                    "PARSE" => Token::Parse,
                    "TO_STRING" => Token::ToString,
                    "LAMBDA" => Token::Lambda,
                    "PIPE" => Token::Pipe,
                    "RANGE" => Token::RangeOp,
                    "FOLD" => Token::Fold,
                    "FIND" => Token::FindOp,
                    "ALL" => Token::AllOp,
                    "ANY" => Token::AnyOp,
                    "ZIP" => Token::Zip,
                    "UNZIP" => Token::Unzip,
                    "FLATTEN" => Token::Flatten,
                    "UNIQUE" => Token::Unique,
                    "COUNT" => Token::CountOp,
                    "GROUP_BY" => Token::GroupBy,
                    "PARTITION" => Token::Partition,
                    "TAKE" => Token::TakeOp,
                    "DROP" => Token::DropOp,
                    "SLICE" => Token::SliceOp,
                    "CONCAT" => Token::Concat,
                    "SPLIT" => Token::SplitOp,
                    "REPLACE" => Token::ReplaceOp,
                    "TRIM" => Token::Trim,
                    "STARTS_WITH" => Token::StartsWith,
                    "ENDS_WITH" => Token::EndsWith,
                    "INCLUDES" => Token::Includes,
                    "INDEX_OF" => Token::IndexOfOp,
                    "PAD" => Token::Pad,
                    "EVAL" => Token::Eval,
                    "TYPE_OF" => Token::TypeOfOp,
                    "CLONE" => Token::CloneOp,
                    "MERGE" => Token::Merge,
                    "DIFF" => Token::Diff,
                    "INTERSECTION" => Token::Intersection,
                    "UNION" => Token::UnionOp,
                    "CLEAR" => Token::ClearOp,
                    "SWAP" => Token::SwapOp,
                    "SHUFFLE" => Token::Shuffle,
                    "SAMPLE" => Token::Sample,
                    "MIN_OF" => Token::MinOfOp,
                    "MAX_OF" => Token::MaxOfOp,
                    "AVERAGE" => Token::AverageOp,
                    "MEDIAN" => Token::Median,
                    "MODE" => Token::ModeOp,
                    "STDDEV" => Token::StdDev,
                    "VARIANCE" => Token::Variance,
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