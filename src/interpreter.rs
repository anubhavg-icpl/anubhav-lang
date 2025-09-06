use std::collections::HashMap;
use crate::parser::Statement;

pub struct Interpreter {
    intents: HashMap<String, String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            intents: HashMap::new(),
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
                    } else {
                        return Err(format!("Intent '{}' not found", intent_name));
                    }
                }
            }
        }
        Ok(())
    }
}