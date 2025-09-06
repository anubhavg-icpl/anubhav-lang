use std::collections::HashMap;
use crate::parser::{Statement, Expression};
use crate::lexer::Token;

pub struct Interpreter {
    intents: HashMap<String, String>,
    calculations: HashMap<String, f64>,
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            intents: HashMap::new(),
            calculations: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Statement::IntentDeclaration { name, message } => {
                    self.intents.insert(name, message);
                }
                Statement::ManifestCall { intent_name, with_message } => {
                    if let Some(message) = self.intents.get(&intent_name) {
                        if let Some(context) = with_message {
                            println!("{} {}", message, context);
                        } else {
                            println!("{}", message);
                        }
                    } else if let Some(result) = self.calculations.get(&intent_name) {
                        if let Some(context) = with_message {
                            println!("{} {}", result, context);
                        } else {
                            println!("{}", result);
                        }
                    } else {
                        return Err(format!("Intent '{}' not found", intent_name));
                    }
                }
                Statement::Calculate { name, expression } => {
                    let result = self.evaluate_expression(&expression)?;
                    self.calculations.insert(name, result);
                }
                Statement::Store { name, value } => {
                    let result = self.evaluate_expression(&value)?;
                    self.variables.insert(name, result);
                }
                Statement::Combine { name, parts } => {
                    let mut combined = String::new();
                    for part in parts {
                        if part.starts_with("${") && part.ends_with("}") {
                            let var_name = &part[2..part.len()-1];
                            if let Some(msg) = self.intents.get(var_name) {
                                combined.push_str(msg);
                            } else if let Some(val) = self.calculations.get(var_name) {
                                combined.push_str(&val.to_string());
                            } else if let Some(val) = self.variables.get(var_name) {
                                combined.push_str(&val.to_string());
                            } else {
                                combined.push_str(&format!("<{} not found>", var_name));
                            }
                        } else {
                            combined.push_str(&part);
                        }
                    }
                    self.intents.insert(name, combined);
                }
                Statement::Repeat { count, body } => {
                    let times = self.evaluate_expression(&count)? as usize;
                    for _ in 0..times {
                        self.execute(body.clone())?;
                    }
                }
                Statement::If { condition, then_body, else_body } => {
                    let cond_value = self.evaluate_expression(&condition)?;
                    if cond_value != 0.0 {
                        self.execute(then_body)?;
                    } else if let Some(else_stmts) = else_body {
                        self.execute(else_stmts)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn evaluate_expression(&self, expr: &Expression) -> Result<f64, String> {
        match expr {
            Expression::Number(n) => Ok(*n),
            Expression::Recall(name) => {
                self.variables.get(name)
                    .or_else(|| self.calculations.get(name))
                    .copied()
                    .ok_or_else(|| format!("Variable '{}' not found", name))
            }
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
                    Token::Equal => Ok(if left_val == right_val { 1.0 } else { 0.0 }),
                    Token::NotEqual => Ok(if left_val != right_val { 1.0 } else { 0.0 }),
                    Token::Less => Ok(if left_val < right_val { 1.0 } else { 0.0 }),
                    Token::Greater => Ok(if left_val > right_val { 1.0 } else { 0.0 }),
                    Token::LessEqual => Ok(if left_val <= right_val { 1.0 } else { 0.0 }),
                    Token::GreaterEqual => Ok(if left_val >= right_val { 1.0 } else { 0.0 }),
                    Token::And => Ok(if left_val != 0.0 && right_val != 0.0 { 1.0 } else { 0.0 }),
                    Token::Or => Ok(if left_val != 0.0 || right_val != 0.0 { 1.0 } else { 0.0 }),
                    _ => Err(format!("Invalid operator: {:?}", operator))
                }
            }
        }
    }
}