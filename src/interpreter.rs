use std::collections::HashMap;
use crate::parser::{Statement, Expression};
use crate::lexer::Token;

pub struct Interpreter {
    intents: HashMap<String, String>,
    calculations: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            intents: HashMap::new(),
            calculations: HashMap::new(),
        }
    }

    pub fn execute(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Statement::IntentDeclaration { name, message } => {
                    self.intents.insert(name, message);
                }
                Statement::ManifestCall { intent_name } => {
                    if let Some(message) = self.intents.get(&intent_name) {
                        println!("{}", message);
                    } else if let Some(result) = self.calculations.get(&intent_name) {
                        println!("{}", result);
                    } else {
                        return Err(format!("Intent '{}' not found", intent_name));
                    }
                }
                Statement::Calculate { name, expression } => {
                    let result = self.evaluate_expression(&expression)?;
                    self.calculations.insert(name, result);
                }
            }
        }
        Ok(())
    }

    fn evaluate_expression(&self, expr: &Expression) -> Result<f64, String> {
        match expr {
            Expression::Number(n) => Ok(*n),
            Expression::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                
                match operator {
                    Token::Plus => Ok(left_val + right_val),
                    Token::Minus => Ok(left_val - right_val),
                    Token::Star => Ok(left_val * right_val),
                    Token::Slash => {
                        if right_val == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    _ => Err(format!("Invalid operator: {:?}", operator))
                }
            }
        }
    }
}