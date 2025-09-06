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
    For {
        variable: String,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        body: Vec<Statement>,
    },
    Assert {
        condition: Expression,
        message: Option<String>,
    },
    TryCatch {
        try_body: Vec<Statement>,
        catch_body: Vec<Statement>,
    },
    StringTransform {
        name: String,
        operation: String,
        source: String,
    },
    Switch {
        expression: Expression,
        cases: Vec<(Expression, Vec<Statement>)>,
        default_case: Option<Vec<Statement>>,
    },
    ArrayCreate {
        name: String,
    },
    ArrayPush {
        array_name: String,
        value: Expression,
    },
    ArrayPop {
        array_name: String,
        result_name: String,
    },
    ArrayGet {
        array_name: String,
        index: Expression,
        result_name: String,
    },
    ArraySet {
        array_name: String,
        index: Expression,
        value: Expression,
    },
    Import {
        filename: String,
    },
    Export {
        items: Vec<String>,
        filename: String,
    },
    Break,
    Continue,
    FunctionDefinition {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    FunctionCall {
        function_name: String,
        arguments: Vec<Expression>,
        result_name: Option<String>,
    },
    Return {
        value: Option<Expression>,
    },
    ArraySort {
        array_name: String,
        ascending: bool,
    },
    ArrayFilter {
        array_name: String,
        condition: Expression,
        result_array: String,
    },
    ArrayReverse {
        array_name: String,
    },
    ArrayMap {
        array_name: String,
        expression: Expression,
        result_array: String,
    },
    ArraySum {
        array_name: String,
        result_name: String,
    },
    ArrayJoin {
        array_name: String,
        separator: String,
        result_name: String,
    },
    DictCreate {
        name: String,
    },
    DictPut {
        dict_name: String,
        key: String,
        value: Expression,
    },
    DictFetch {
        dict_name: String,
        key: String,
        result_name: String,
    },
    DictKeys {
        dict_name: String,
        result_array: String,
    },
    DictValues {
        dict_name: String,
        result_array: String,
    },
    DictDelete {
        dict_name: String,
        key: String,
    },
    ReadFile {
        filename: String,
        result_name: String,
    },
    WriteFile {
        filename: String,
        content: String,
    },
    AppendFile {
        filename: String,
        content: String,
    },
    FileExists {
        filename: String,
        result_name: String,
    },
    Sleep {
        milliseconds: Expression,
    },
    Input {
        prompt: String,
        result_name: String,
    },
    GetType {
        variable: String,
        result_name: String,
    },
    ParseNumber {
        source: String,
        result_name: String,
    },
    Range {
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        result_array: String,
    },
    Fold {
        array_name: String,
        initial: Expression,
        operation: Expression,
        result_name: String,
    },
    Unique {
        array_name: String,
        result_array: String,
    },
    Concat {
        array1: String,
        array2: String,
        result_array: String,
    },
    Take {
        array_name: String,
        count: Expression,
        result_array: String,
    },
    Drop {
        array_name: String,
        count: Expression,
        result_array: String,
    },
    Zip {
        array1: String,
        array2: String,
        result_array: String,
    },
    Flatten {
        array_name: String,
        result_array: String,
    },
    Find {
        array_name: String,
        condition: Expression,
        result_name: String,
    },
    Count {
        array_name: String,
        condition: Expression,
        result_name: String,
    },
    Average {
        array_name: String,
        result_name: String,
    },
    Replace {
        text: String,
        pattern: String,
        replacement: String,
        result_name: String,
    },
    Split {
        text: String,
        delimiter: String,
        result_array: String,
    },
    Clear {
        target: String,
    },
    Shuffle {
        array_name: String,
    },
    Clone {
        source: String,
        destination: String,
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
                Token::For => {
                    statements.push(self.parse_for()?);
                }
                Token::Assert => {
                    statements.push(self.parse_assert()?);
                }
                Token::Try => {
                    statements.push(self.parse_try_catch()?);
                }
                Token::Uppercase | Token::Lowercase => {
                    statements.push(self.parse_string_transform()?);
                }
                Token::Switch => {
                    statements.push(self.parse_switch()?);
                }
                Token::Array => {
                    statements.push(self.parse_array_create()?);
                }
                Token::Push => {
                    statements.push(self.parse_array_push()?);
                }
                Token::Pop => {
                    statements.push(self.parse_array_pop()?);
                }
                Token::Get => {
                    statements.push(self.parse_array_get()?);
                }
                Token::Set => {
                    statements.push(self.parse_array_set()?);
                }
                Token::Import => {
                    statements.push(self.parse_import()?);
                }
                Token::Export => {
                    statements.push(self.parse_export()?);
                }
                Token::Break => {
                    self.advance();
                    statements.push(Statement::Break);
                }
                Token::Continue => {
                    self.advance();
                    statements.push(Statement::Continue);
                }
                Token::Function => {
                    statements.push(self.parse_function_definition()?);
                }
                Token::Call => {
                    statements.push(self.parse_function_call()?);
                }
                Token::Return => {
                    statements.push(self.parse_return()?);
                }
                Token::Sort => {
                    statements.push(self.parse_array_sort()?);
                }
                Token::Filter => {
                    statements.push(self.parse_array_filter()?);
                }
                Token::Reverse => {
                    statements.push(self.parse_array_reverse()?);
                }
                Token::Map => {
                    statements.push(self.parse_array_map()?);
                }
                Token::Sum => {
                    statements.push(self.parse_array_sum()?);
                }
                Token::Join => {
                    statements.push(self.parse_array_join()?);
                }
                Token::Dict => {
                    statements.push(self.parse_dict_create()?);
                }
                Token::Put => {
                    statements.push(self.parse_dict_put()?);
                }
                Token::Fetch => {
                    statements.push(self.parse_dict_fetch()?);
                }
                Token::Keys => {
                    statements.push(self.parse_dict_keys()?);
                }
                Token::Values => {
                    statements.push(self.parse_dict_values()?);
                }
                Token::Delete => {
                    statements.push(self.parse_dict_delete()?);
                }
                Token::ReadFile => {
                    statements.push(self.parse_read_file()?);
                }
                Token::WriteFile => {
                    statements.push(self.parse_write_file()?);
                }
                Token::AppendFile => {
                    statements.push(self.parse_append_file()?);
                }
                Token::Exists => {
                    statements.push(self.parse_file_exists()?);
                }
                Token::Sleep => {
                    statements.push(self.parse_sleep()?);
                }
                Token::Input => {
                    statements.push(self.parse_input()?);
                }
                Token::Type => {
                    statements.push(self.parse_get_type()?);
                }
                Token::Parse => {
                    statements.push(self.parse_parse_number()?);
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
                Token::Break => {
                    self.advance();
                    body.push(Statement::Break);
                },
                Token::Continue => {
                    self.advance();
                    body.push(Statement::Continue);
                },
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
                Token::Break => {
                    self.advance();
                    then_body.push(Statement::Break);
                },
                Token::Continue => {
                    self.advance();
                    then_body.push(Statement::Continue);
                },
                Token::Return => then_body.push(self.parse_return()?),
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
                    Token::Break => {
                        self.advance();
                        else_stmts.push(Statement::Break);
                    },
                    Token::Continue => {
                        self.advance();
                        else_stmts.push(Statement::Continue);
                    },
                    Token::Return => else_stmts.push(self.parse_return()?),
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
                Token::Break => {
                    self.advance();
                    body.push(Statement::Break);
                },
                Token::Continue => {
                    self.advance();
                    body.push(Statement::Continue);
                },
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

    fn parse_for(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip FOR
        
        let variable = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected variable name after FOR"));
        };
        self.advance();
        
        let start = self.parse_expression()?;
        
        if self.current_token != Token::To {
            return Err(format!("Expected TO after FOR start value"));
        }
        self.advance(); // Skip TO
        
        let end = self.parse_expression()?;
        
        let step = if self.current_token == Token::Step {
            self.advance(); // Skip STEP
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if self.current_token != Token::Do {
            return Err(format!("Expected DO after FOR parameters"));
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
                Token::For => body.push(self.parse_for()?),
                Token::Array => body.push(self.parse_array_create()?),
                Token::Push => body.push(self.parse_array_push()?),
                Token::Pop => body.push(self.parse_array_pop()?),
                Token::Get => body.push(self.parse_array_get()?),
                Token::Set => body.push(self.parse_array_set()?),
                Token::Break => {
                    self.advance();
                    body.push(Statement::Break);
                },
                Token::Continue => {
                    self.advance();
                    body.push(Statement::Continue);
                },
                _ => {
                    return Err(format!("Unexpected token in FOR body: {:?}", self.current_token));
                }
            }
        }
        
        if self.current_token != Token::End {
            return Err(format!("Expected END to close FOR"));
        }
        self.advance(); // Skip END
        
        Ok(Statement::For { variable, start, end, step, body })
    }
    
    fn parse_assert(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip ASSERT
        
        let condition = self.parse_expression()?;
        
        let message = if matches!(self.current_token, Token::StringLiteral(_)) {
            if let Token::StringLiteral(msg) = &self.current_token {
                let msg = msg.clone();
                self.advance();
                Some(msg)
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(Statement::Assert { condition, message })
    }
    
    fn parse_try_catch(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip TRY
        
        let mut try_body = Vec::new();
        
        while self.current_token != Token::Catch {
            match self.current_token {
                Token::EOF => return Err(format!("Expected CATCH after TRY")),
                Token::Intent => try_body.push(self.parse_intent_declaration()?),
                Token::Manifest => try_body.push(self.parse_manifest_call()?),
                Token::Calculate => try_body.push(self.parse_calculate()?),
                Token::Store => try_body.push(self.parse_store()?),
                Token::Combine => try_body.push(self.parse_combine()?),
                Token::Repeat => try_body.push(self.parse_repeat()?),
                Token::If => try_body.push(self.parse_if()?),
                Token::Print => try_body.push(self.parse_print()?),
                Token::While => try_body.push(self.parse_while()?),
                Token::Increment => try_body.push(self.parse_increment()?),
                Token::Decrement => try_body.push(self.parse_decrement()?),
                Token::For => try_body.push(self.parse_for()?),
                Token::Assert => try_body.push(self.parse_assert()?),
                _ => {
                    return Err(format!("Unexpected token in TRY body: {:?}", self.current_token));
                }
            }
        }
        
        self.advance(); // Skip CATCH
        
        let mut catch_body = Vec::new();
        
        while self.current_token != Token::End {
            match self.current_token {
                Token::EOF => return Err(format!("Expected END after CATCH")),
                Token::Intent => catch_body.push(self.parse_intent_declaration()?),
                Token::Manifest => catch_body.push(self.parse_manifest_call()?),
                Token::Calculate => catch_body.push(self.parse_calculate()?),
                Token::Store => catch_body.push(self.parse_store()?),
                Token::Combine => catch_body.push(self.parse_combine()?),
                Token::Repeat => catch_body.push(self.parse_repeat()?),
                Token::If => catch_body.push(self.parse_if()?),
                Token::Print => catch_body.push(self.parse_print()?),
                Token::While => catch_body.push(self.parse_while()?),
                Token::Increment => catch_body.push(self.parse_increment()?),
                Token::Decrement => catch_body.push(self.parse_decrement()?),
                Token::For => catch_body.push(self.parse_for()?),
                Token::Assert => catch_body.push(self.parse_assert()?),
                _ => {
                    return Err(format!("Unexpected token in CATCH body: {:?}", self.current_token));
                }
            }
        }
        
        if self.current_token != Token::End {
            return Err(format!("Expected END to close TRY/CATCH"));
        }
        self.advance(); // Skip END
        
        Ok(Statement::TryCatch { try_body, catch_body })
    }
    
    fn parse_string_transform(&mut self) -> Result<Statement, String> {
        let operation = match self.current_token {
            Token::Uppercase => "UPPERCASE".to_string(),
            Token::Lowercase => "LOWERCASE".to_string(),
            _ => return Err("Invalid string operation".to_string()),
        };
        self.advance(); // Skip operation token
        
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected identifier after {}", operation));
        };
        self.advance();
        
        let source = match &self.current_token {
            Token::StringLiteral(s) => {
                let result = s.clone();
                self.advance();
                result
            }
            Token::Identifier(src) => {
                let result = src.clone();
                self.advance();
                result
            }
            _ => {
                return Err(format!("Expected string literal or identifier after {}", operation));
            }
        };
        
        Ok(Statement::StringTransform { name, operation, source })
    }
    
    fn parse_switch(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip SWITCH
        
        let expression = self.parse_expression()?;
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        while self.current_token != Token::End {
            match self.current_token {
                Token::Case => {
                    self.advance(); // Skip CASE
                    
                    let case_value = self.parse_expression()?;
                    
                    if self.current_token != Token::Do {
                        return Err("Expected DO after CASE value".to_string());
                    }
                    self.advance(); // Skip DO
                    
                    let mut case_body = Vec::new();
                    while self.current_token != Token::Case 
                        && self.current_token != Token::Default 
                        && self.current_token != Token::End {
                        
                        match self.current_token {
                            Token::EOF => return Err("Expected CASE, DEFAULT, or END in SWITCH".to_string()),
                            Token::Intent => case_body.push(self.parse_intent_declaration()?),
                            Token::Manifest => case_body.push(self.parse_manifest_call()?),
                            Token::Calculate => case_body.push(self.parse_calculate()?),
                            Token::Store => case_body.push(self.parse_store()?),
                            Token::Combine => case_body.push(self.parse_combine()?),
                            Token::Repeat => case_body.push(self.parse_repeat()?),
                            Token::If => case_body.push(self.parse_if()?),
                            Token::Print => case_body.push(self.parse_print()?),
                            Token::While => case_body.push(self.parse_while()?),
                            Token::Increment => case_body.push(self.parse_increment()?),
                            Token::Decrement => case_body.push(self.parse_decrement()?),
                            Token::For => case_body.push(self.parse_for()?),
                            Token::Assert => case_body.push(self.parse_assert()?),
                            Token::Switch => case_body.push(self.parse_switch()?),
                            _ => {
                                return Err(format!("Unexpected token in CASE body: {:?}", self.current_token));
                            }
                        }
                    }
                    
                    cases.push((case_value, case_body));
                }
                Token::Default => {
                    self.advance(); // Skip DEFAULT
                    
                    if self.current_token != Token::Do {
                        return Err("Expected DO after DEFAULT".to_string());
                    }
                    self.advance(); // Skip DO
                    
                    let mut default_body = Vec::new();
                    while self.current_token != Token::End {
                        match self.current_token {
                            Token::EOF => return Err("Expected END after DEFAULT".to_string()),
                            Token::Intent => default_body.push(self.parse_intent_declaration()?),
                            Token::Manifest => default_body.push(self.parse_manifest_call()?),
                            Token::Calculate => default_body.push(self.parse_calculate()?),
                            Token::Store => default_body.push(self.parse_store()?),
                            Token::Combine => default_body.push(self.parse_combine()?),
                            Token::Repeat => default_body.push(self.parse_repeat()?),
                            Token::If => default_body.push(self.parse_if()?),
                            Token::Print => default_body.push(self.parse_print()?),
                            Token::While => default_body.push(self.parse_while()?),
                            Token::Increment => default_body.push(self.parse_increment()?),
                            Token::Decrement => default_body.push(self.parse_decrement()?),
                            Token::For => default_body.push(self.parse_for()?),
                            Token::Assert => default_body.push(self.parse_assert()?),
                            Token::Switch => default_body.push(self.parse_switch()?),
                            _ => {
                                return Err(format!("Unexpected token in DEFAULT body: {:?}", self.current_token));
                            }
                        }
                    }
                    
                    default_case = Some(default_body);
                    break;
                }
                _ => {
                    return Err(format!("Expected CASE or DEFAULT in SWITCH: {:?}", self.current_token));
                }
            }
        }
        
        if self.current_token != Token::End {
            return Err("Expected END to close SWITCH".to_string());
        }
        self.advance(); // Skip END
        
        Ok(Statement::Switch { expression, cases, default_case })
    }
    
    fn parse_array_create(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip ARRAY
        
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err("Expected identifier after ARRAY".to_string());
        };
        self.advance();
        
        Ok(Statement::ArrayCreate { name })
    }
    
    fn parse_array_push(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip PUSH
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err("Expected array name after PUSH".to_string());
        };
        self.advance();
        
        let value = self.parse_expression()?;
        
        Ok(Statement::ArrayPush { array_name, value })
    }
    
    fn parse_array_pop(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip POP
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err("Expected array name after POP".to_string());
        };
        self.advance();
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err("Expected result variable name after POP array".to_string());
        };
        self.advance();
        
        Ok(Statement::ArrayPop { array_name, result_name })
    }
    
    fn parse_array_get(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip GET
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err("Expected array name after GET".to_string());
        };
        self.advance();
        
        let index = self.parse_expression()?;
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err("Expected result variable name after GET index".to_string());
        };
        self.advance();
        
        Ok(Statement::ArrayGet { array_name, index, result_name })
    }
    
    fn parse_array_set(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip SET
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err("Expected array name after SET".to_string());
        };
        self.advance();
        
        let index = self.parse_expression()?;
        
        let value = self.parse_expression()?;
        
        Ok(Statement::ArraySet { array_name, index, value })
    }
    
    fn parse_import(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip IMPORT
        
        let filename = if let Token::StringLiteral(name) = &self.current_token {
            name.clone()
        } else {
            return Err("Expected filename string after IMPORT".to_string());
        };
        self.advance();
        
        Ok(Statement::Import { filename })
    }
    
    fn parse_export(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip EXPORT
        
        let mut items = Vec::new();
        
        // Parse list of identifiers to export
        while matches!(self.current_token, Token::Identifier(_)) {
            if let Token::Identifier(item) = &self.current_token {
                items.push(item.clone());
            }
            self.advance();
        }
        
        if items.is_empty() {
            return Err("Expected items to export".to_string());
        }
        
        let filename = if let Token::StringLiteral(name) = &self.current_token {
            name.clone()
        } else {
            return Err("Expected filename string after EXPORT items".to_string());
        };
        self.advance();
        
        Ok(Statement::Export { items, filename })
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
            Token::Min | Token::Max | Token::Floor | Token::Ceil | Token::Round | Token::Random | Token::Length | Token::Size => {
                let op = self.current_token.clone();
                self.advance();
                
                if self.current_token != Token::LeftParen {
                    return Err(format!("Expected ( after {:?}", op));
                }
                self.advance();
                
                match op {
                    Token::Min | Token::Max => {
                        // Two-argument functions
                        let first_arg = self.parse_primary()?;
                        let second_arg = self.parse_primary()?;
                        
                        if self.current_token != Token::RightParen {
                            return Err(format!("Expected ) after function arguments"));
                        }
                        self.advance();
                        
                        Ok(Expression::BinaryOp {
                            left: Box::new(first_arg),
                            operator: op,
                            right: Box::new(second_arg),
                        })
                    }
                    Token::Floor | Token::Ceil | Token::Round => {
                        // Single-argument functions
                        let arg = self.parse_primary()?;
                        
                        if self.current_token != Token::RightParen {
                            return Err(format!("Expected ) after function argument"));
                        }
                        self.advance();
                        
                        Ok(Expression::BinaryOp {
                            left: Box::new(Expression::Number(0.0)), // Dummy left operand
                            operator: op,
                            right: Box::new(arg),
                        })
                    }
                    Token::Random => {
                        // Zero-argument function
                        if self.current_token != Token::RightParen {
                            return Err(format!("Expected ) after RANDOM"));
                        }
                        self.advance();
                        
                        Ok(Expression::BinaryOp {
                            left: Box::new(Expression::Number(0.0)), // Dummy left operand
                            operator: op,
                            right: Box::new(Expression::Number(0.0)), // Dummy right operand
                        })
                    }
                    Token::Size | Token::Length => {
                        // String function - takes string literal or identifier
                        if let Token::StringLiteral(s) = &self.current_token {
                            let str_len = s.len() as f64;
                            self.advance();
                            
                            if self.current_token != Token::RightParen {
                                return Err(format!("Expected ) after LENGTH argument"));
                            }
                            self.advance();
                            
                            Ok(Expression::Number(str_len))
                        } else if let Token::Identifier(name) = &self.current_token {
                            let var_name = name.clone();
                            self.advance();
                            
                            if self.current_token != Token::RightParen {
                                return Err(format!("Expected ) after LENGTH argument"));
                            }
                            self.advance();
                            
                            // Store the identifier to be resolved at runtime
                            Ok(Expression::BinaryOp {
                                left: Box::new(Expression::Recall(var_name)),
                                operator: op,
                                right: Box::new(Expression::Number(0.0)), // Dummy
                            })
                        } else {
                            return Err(format!("LENGTH expects string literal or identifier"));
                        }
                    }
                    _ => unreachable!()
                }
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
            Token::Minus => {
                self.advance(); // Skip minus
                let expr = self.parse_primary()?;
                Ok(Expression::BinaryOp {
                    left: Box::new(Expression::Number(0.0)),
                    operator: Token::Minus,
                    right: Box::new(expr),
                })
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

    fn parse_function_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip FUNCTION
        
        let function_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected function name after FUNCTION"));
        };
        self.advance();
        
        let mut parameters = Vec::new();
        
        // Parse optional parameters
        if self.current_token == Token::LeftParen {
            self.advance(); // Skip (
            
            while self.current_token != Token::RightParen && self.current_token != Token::EOF {
                if let Token::Identifier(param_name) = &self.current_token {
                    parameters.push(param_name.clone());
                    self.advance();
                    
                    // Skip comma if present
                    if self.current_token == Token::Plus { // Using + as comma separator
                        self.advance();
                    }
                } else {
                    return Err(format!("Expected parameter name"));
                }
            }
            
            if self.current_token != Token::RightParen {
                return Err(format!("Expected ) after function parameters"));
            }
            self.advance(); // Skip )
        }
        
        if self.current_token != Token::Do {
            return Err(format!("Expected DO after function signature"));
        }
        self.advance(); // Skip DO
        
        let mut body = Vec::new();
        
        while self.current_token != Token::End && self.current_token != Token::EOF {
            match self.current_token {
                Token::Intent => body.push(self.parse_intent_declaration()?),
                Token::Manifest => body.push(self.parse_manifest_call()?),
                Token::Calculate => body.push(self.parse_calculate()?),
                Token::Store => body.push(self.parse_store()?),
                Token::Print => body.push(self.parse_print()?),
                Token::If => body.push(self.parse_if()?),
                Token::While => body.push(self.parse_while()?),
                Token::For => body.push(self.parse_for()?),
                Token::Return => body.push(self.parse_return()?),
                Token::Call => body.push(self.parse_function_call()?),
                _ => {
                    return Err(format!("Unexpected token in function body: {:?}", self.current_token));
                }
            }
        }
        
        if self.current_token != Token::End {
            return Err(format!("Expected END to close FUNCTION"));
        }
        self.advance(); // Skip END
        
        Ok(Statement::FunctionDefinition { 
            name: function_name, 
            parameters, 
            body 
        })
    }

    fn parse_function_call(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip CALL
        
        let function_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected function name after CALL"));
        };
        self.advance();
        
        let mut arguments = Vec::new();
        let mut result_name = None;
        
        // Parse optional arguments
        if self.current_token == Token::LeftParen {
            self.advance(); // Skip (
            
            while self.current_token != Token::RightParen && self.current_token != Token::EOF {
                arguments.push(self.parse_expression()?);
                
                // Skip comma if present
                if self.current_token == Token::Plus { // Using + as comma separator
                    self.advance();
                }
            }
            
            if self.current_token != Token::RightParen {
                return Err(format!("Expected ) after function arguments"));
            }
            self.advance(); // Skip )
        }
        
        // Check for result variable (INTO identifier)
        if let Token::Identifier(var_name) = &self.current_token {
            result_name = Some(var_name.clone());
            self.advance();
        }
        
        Ok(Statement::FunctionCall { 
            function_name, 
            arguments, 
            result_name 
        })
    }

    fn parse_return(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip RETURN
        
        let value = if self.current_token == Token::EOF 
            || self.current_token == Token::End {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        Ok(Statement::Return { value })
    }

    fn parse_array_sort(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip SORT
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected array name after SORT"));
        };
        self.advance();
        
        // Default to ascending, check for DESC keyword
        let mut ascending = true;
        if let Token::Identifier(order) = &self.current_token {
            if order == "DESC" {
                ascending = false;
                self.advance();
            } else if order == "ASC" {
                self.advance(); // Skip explicit ASC
            }
        }
        
        Ok(Statement::ArraySort { array_name, ascending })
    }

    fn parse_array_filter(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip FILTER
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected array name after FILTER"));
        };
        self.advance();
        
        let condition = self.parse_expression()?;
        
        let result_array = if let Token::Identifier(result_name) = &self.current_token {
            result_name.clone()
        } else {
            return Err(format!("Expected result array name for FILTER"));
        };
        self.advance();
        
        Ok(Statement::ArrayFilter { array_name, condition, result_array })
    }

    fn parse_array_reverse(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip REVERSE
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected array name after REVERSE"));
        };
        self.advance();
        
        Ok(Statement::ArrayReverse { array_name })
    }

    fn parse_array_map(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip MAP
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected array name after MAP"));
        };
        self.advance();
        
        let expression = self.parse_expression()?;
        
        let result_array = if let Token::Identifier(result_name) = &self.current_token {
            result_name.clone()
        } else {
            return Err(format!("Expected result array name for MAP"));
        };
        self.advance();
        
        Ok(Statement::ArrayMap { array_name, expression, result_array })
    }

    fn parse_array_sum(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip SUM
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected array name after SUM"));
        };
        self.advance();
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result variable name for SUM"));
        };
        self.advance();
        
        Ok(Statement::ArraySum { array_name, result_name })
    }

    fn parse_array_join(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip JOIN
        
        let array_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected array name after JOIN"));
        };
        self.advance();
        
        let separator = if let Token::StringLiteral(sep) = &self.current_token {
            sep.clone()
        } else {
            return Err(format!("Expected separator string for JOIN"));
        };
        self.advance();
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result variable name for JOIN"));
        };
        self.advance();
        
        Ok(Statement::ArrayJoin { array_name, separator, result_name })
    }

    fn parse_dict_create(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip DICT
        
        let name = if let Token::Identifier(n) = &self.current_token {
            n.clone()
        } else {
            return Err(format!("Expected dictionary name after DICT"));
        };
        self.advance();
        
        Ok(Statement::DictCreate { name })
    }

    fn parse_dict_put(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip PUT
        
        let dict_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected dictionary name after PUT"));
        };
        self.advance();
        
        let key = if let Token::StringLiteral(k) = &self.current_token {
            k.clone()
        } else if let Token::Identifier(k) = &self.current_token {
            k.clone()
        } else {
            return Err(format!("Expected key for PUT"));
        };
        self.advance();
        
        let value = self.parse_expression()?;
        
        Ok(Statement::DictPut { dict_name, key, value })
    }

    fn parse_dict_fetch(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip FETCH
        
        let dict_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected dictionary name after FETCH"));
        };
        self.advance();
        
        let key = if let Token::StringLiteral(k) = &self.current_token {
            k.clone()
        } else if let Token::Identifier(k) = &self.current_token {
            k.clone()
        } else {
            return Err(format!("Expected key for FETCH"));
        };
        self.advance();
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result variable name for FETCH"));
        };
        self.advance();
        
        Ok(Statement::DictFetch { dict_name, key, result_name })
    }

    fn parse_dict_keys(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip KEYS
        
        let dict_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected dictionary name after KEYS"));
        };
        self.advance();
        
        let result_array = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result array name for KEYS"));
        };
        self.advance();
        
        Ok(Statement::DictKeys { dict_name, result_array })
    }

    fn parse_dict_values(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip VALUES
        
        let dict_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected dictionary name after VALUES"));
        };
        self.advance();
        
        let result_array = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result array name for VALUES"));
        };
        self.advance();
        
        Ok(Statement::DictValues { dict_name, result_array })
    }

    fn parse_dict_delete(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip DELETE
        
        let dict_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected dictionary name after DELETE"));
        };
        self.advance();
        
        let key = if let Token::StringLiteral(k) = &self.current_token {
            k.clone()
        } else if let Token::Identifier(k) = &self.current_token {
            k.clone()
        } else {
            return Err(format!("Expected key for DELETE"));
        };
        self.advance();
        
        Ok(Statement::DictDelete { dict_name, key })
    }

    fn parse_read_file(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip READ_FILE
        
        let filename = if let Token::StringLiteral(f) = &self.current_token {
            f.clone()
        } else {
            return Err(format!("Expected filename string after READ_FILE"));
        };
        self.advance();
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result variable name for READ_FILE"));
        };
        self.advance();
        
        Ok(Statement::ReadFile { filename, result_name })
    }

    fn parse_write_file(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip WRITE_FILE
        
        let filename = if let Token::StringLiteral(f) = &self.current_token {
            f.clone()
        } else {
            return Err(format!("Expected filename string after WRITE_FILE"));
        };
        self.advance();
        
        let content = if let Token::StringLiteral(c) = &self.current_token {
            c.clone()
        } else if let Token::Identifier(var) = &self.current_token {
            format!("${{{}}}", var)
        } else {
            return Err(format!("Expected content for WRITE_FILE"));
        };
        self.advance();
        
        Ok(Statement::WriteFile { filename, content })
    }

    fn parse_append_file(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip APPEND_FILE
        
        let filename = if let Token::StringLiteral(f) = &self.current_token {
            f.clone()
        } else {
            return Err(format!("Expected filename string after APPEND_FILE"));
        };
        self.advance();
        
        let content = if let Token::StringLiteral(c) = &self.current_token {
            c.clone()
        } else if let Token::Identifier(var) = &self.current_token {
            format!("${{{}}}", var)
        } else {
            return Err(format!("Expected content for APPEND_FILE"));
        };
        self.advance();
        
        Ok(Statement::AppendFile { filename, content })
    }

    fn parse_file_exists(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip EXISTS
        
        let filename = if let Token::StringLiteral(f) = &self.current_token {
            f.clone()
        } else {
            return Err(format!("Expected filename string after EXISTS"));
        };
        self.advance();
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result variable name for EXISTS"));
        };
        self.advance();
        
        Ok(Statement::FileExists { filename, result_name })
    }

    fn parse_sleep(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip SLEEP
        
        let milliseconds = self.parse_expression()?;
        
        Ok(Statement::Sleep { milliseconds })
    }

    fn parse_input(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip INPUT
        
        let prompt = if let Token::StringLiteral(p) = &self.current_token {
            p.clone()
        } else {
            return Err(format!("Expected prompt string after INPUT"));
        };
        self.advance();
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result variable name for INPUT"));
        };
        self.advance();
        
        Ok(Statement::Input { prompt, result_name })
    }

    fn parse_get_type(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip TYPE
        
        let variable = if let Token::Identifier(v) = &self.current_token {
            v.clone()
        } else {
            return Err(format!("Expected variable name after TYPE"));
        };
        self.advance();
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result variable name for TYPE"));
        };
        self.advance();
        
        Ok(Statement::GetType { variable, result_name })
    }

    fn parse_parse_number(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip PARSE
        
        let source = if let Token::Identifier(s) = &self.current_token {
            s.clone()
        } else if let Token::StringLiteral(s) = &self.current_token {
            s.clone()
        } else {
            return Err(format!("Expected source for PARSE"));
        };
        self.advance();
        
        let result_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(format!("Expected result variable name for PARSE"));
        };
        self.advance();
        
        Ok(Statement::ParseNumber { source, result_name })
    }
}