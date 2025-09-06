use std::collections::HashMap;
use crate::parser::{Statement, Expression};
use crate::lexer::Token;

pub struct Interpreter {
    intents: HashMap<String, String>,
    calculations: HashMap<String, f64>,
    variables: HashMap<String, f64>,
    random_seed: u64,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            intents: HashMap::new(),
            calculations: HashMap::new(),
            variables: HashMap::new(),
            random_seed: 12345, // Initial seed
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
                Statement::Print { items } => {
                    let mut output = String::new();
                    for item in items {
                        if item.starts_with("${") && item.ends_with("}") {
                            let var_name = &item[2..item.len()-1];
                            if let Some(msg) = self.intents.get(var_name) {
                                output.push_str(msg);
                            } else if let Some(val) = self.calculations.get(var_name) {
                                output.push_str(&val.to_string());
                            } else if let Some(val) = self.variables.get(var_name) {
                                output.push_str(&val.to_string());
                            } else {
                                output.push_str(&format!("<{} not found>", var_name));
                            }
                        } else {
                            output.push_str(&item);
                        }
                        output.push(' ');
                    }
                    println!("{}", output.trim());
                }
                Statement::While { condition, body } => {
                    while self.evaluate_expression(&condition)? != 0.0 {
                        self.execute(body.clone())?;
                    }
                }
                Statement::Increment { variable } => {
                    if let Some(val) = self.variables.get(&variable) {
                        self.variables.insert(variable, val + 1.0);
                    } else {
                        self.variables.insert(variable, 1.0);
                    }
                }
                Statement::Decrement { variable } => {
                    if let Some(val) = self.variables.get(&variable) {
                        self.variables.insert(variable, val - 1.0);
                    } else {
                        self.variables.insert(variable, -1.0);
                    }
                }
                Statement::For { variable, start, end, step, body } => {
                    let start_val = self.evaluate_expression(&start)?;
                    let end_val = self.evaluate_expression(&end)?;
                    let step_val = if let Some(s) = step {
                        self.evaluate_expression(&s)?
                    } else {
                        1.0
                    };
                    
                    let mut current = start_val;
                    if step_val > 0.0 {
                        while current <= end_val {
                            self.variables.insert(variable.clone(), current);
                            self.execute(body.clone())?;
                            current += step_val;
                        }
                    } else if step_val < 0.0 {
                        while current >= end_val {
                            self.variables.insert(variable.clone(), current);
                            self.execute(body.clone())?;
                            current += step_val;
                        }
                    }
                }
                Statement::Assert { condition, message } => {
                    let result = self.evaluate_expression(&condition)?;
                    if result == 0.0 {
                        let error_msg = if let Some(msg) = message {
                            format!("Assertion failed: {}", msg)
                        } else {
                            "Assertion failed".to_string()
                        };
                        return Err(error_msg);
                    }
                    println!("✓ Assertion passed");
                }
                Statement::TryCatch { try_body, catch_body } => {
                    match self.execute(try_body.clone()) {
                        Ok(_) => {
                            // TRY block succeeded, continue normally
                        }
                        Err(_error) => {
                            // TRY block failed, execute CATCH block
                            self.execute(catch_body.clone())?;
                        }
                    }
                }
                Statement::StringTransform { name, operation, source } => {
                    let source_string = if let Some(intent_str) = self.intents.get(&source) {
                        intent_str.clone()
                    } else {
                        // If not found as intent, treat as literal string
                        source.clone()
                    };
                    
                    let result = match operation.as_str() {
                        "UPPERCASE" => source_string.to_uppercase(),
                        "LOWERCASE" => source_string.to_lowercase(),
                        _ => return Err(format!("Unknown string operation: {}", operation)),
                    };
                    
                    self.intents.insert(name.clone(), result);
                }
            }
        }
        Ok(())
    }
    
    fn next_random(&mut self) -> f64 {
        // Linear congruential generator: (a * seed + c) % m
        // Using constants from Numerical Recipes
        self.random_seed = ((self.random_seed.wrapping_mul(1664525)).wrapping_add(1013904223)) % (1u64 << 32);
        (self.random_seed as f64) / ((1u64 << 32) as f64)
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Result<f64, String> {
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
                    Token::Percent => {
                        if right_val == 0.0 {
                            Err("Modulo by zero".to_string())
                        } else {
                            Ok(left_val % right_val)
                        }
                    }
                    Token::Power => Ok(left_val.powf(right_val)),
                    Token::Equal => Ok(if left_val == right_val { 1.0 } else { 0.0 }),
                    Token::NotEqual => Ok(if left_val != right_val { 1.0 } else { 0.0 }),
                    Token::Less => Ok(if left_val < right_val { 1.0 } else { 0.0 }),
                    Token::Greater => Ok(if left_val > right_val { 1.0 } else { 0.0 }),
                    Token::LessEqual => Ok(if left_val <= right_val { 1.0 } else { 0.0 }),
                    Token::GreaterEqual => Ok(if left_val >= right_val { 1.0 } else { 0.0 }),
                    Token::And => Ok(if left_val != 0.0 && right_val != 0.0 { 1.0 } else { 0.0 }),
                    Token::Or => Ok(if left_val != 0.0 || right_val != 0.0 { 1.0 } else { 0.0 }),
                    Token::Not => {
                        // NOT is a unary operator, right_val contains the operand
                        Ok(if right_val == 0.0 { 1.0 } else { 0.0 })
                    }
                    Token::Min => Ok(left_val.min(right_val)),
                    Token::Max => Ok(left_val.max(right_val)),
                    Token::Floor => Ok(right_val.floor()),
                    Token::Ceil => Ok(right_val.ceil()),
                    Token::Round => Ok(right_val.round()),
                    Token::Random => Ok(self.next_random()),
                    Token::Length => {
                        // LENGTH function - get string from intents
                        if let Expression::Recall(name) = &**left {
                            if let Some(string_val) = self.intents.get(name) {
                                Ok(string_val.len() as f64)
                            } else {
                                Err(format!("String '{}' not found for LENGTH", name))
                            }
                        } else {
                            Err("LENGTH function error".to_string())
                        }
                    }
                    _ => Err(format!("Invalid operator: {:?}", operator))
                }
            }
        }
    }
}