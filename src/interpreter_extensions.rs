// Extension implementations for the interpreter
use crate::interpreter::Interpreter;
use crate::parser::Statement;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::path::Path;

impl Interpreter {
    pub fn execute_extension(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::DictCreate { name } => {
                self.dicts.insert(name.clone(), HashMap::new());
                println!("Dictionary '{}' created", name);
                Ok(())
            }
            Statement::DictPut { dict_name, key, value } => {
                let val = self.evaluate_expression(&value)?;
                if let Some(dict) = self.dicts.get_mut(&dict_name) {
                    dict.insert(key.clone(), val);
                    println!("Set {}['{}'] = {}", dict_name, key, val);
                } else {
                    return Err(format!("Dictionary '{}' not found", dict_name));
                }
                Ok(())
            }
            Statement::DictFetch { dict_name, key, result_name } => {
                if let Some(dict) = self.dicts.get(&dict_name) {
                    if let Some(&value) = dict.get(&key) {
                        self.variables.insert(result_name.clone(), value);
                        println!("Fetched {}['{}'] = {}", dict_name, key, value);
                    } else {
                        return Err(format!("Key '{}' not found in dictionary '{}'", key, dict_name));
                    }
                } else {
                    return Err(format!("Dictionary '{}' not found", dict_name));
                }
                Ok(())
            }
            Statement::ReadFile { filename, result_name } => {
                match fs::read_to_string(&filename) {
                    Ok(content) => {
                        self.intents.insert(result_name.clone(), content.clone());
                        println!("Read {} bytes from '{}'", content.len(), filename);
                    }
                    Err(e) => return Err(format!("Failed to read file '{}': {}", filename, e))
                }
                Ok(())
            }
            Statement::WriteFile { filename, content } => {
                // Resolve content if it's a variable reference
                let actual_content = if content.starts_with("${") && content.ends_with("}") {
                    let var_name = &content[2..content.len()-1];
                    self.intents.get(var_name).cloned()
                        .unwrap_or_else(|| content.clone())
                } else {
                    content.clone()
                };
                
                match fs::write(&filename, actual_content.as_bytes()) {
                    Ok(_) => println!("Wrote {} bytes to '{}'", actual_content.len(), filename),
                    Err(e) => return Err(format!("Failed to write to file '{}': {}", filename, e))
                }
                Ok(())
            }
            Statement::Sleep { milliseconds } => {
                let ms = self.evaluate_expression(&milliseconds)? as u64;
                println!("Sleeping for {} ms...", ms);
                thread::sleep(Duration::from_millis(ms));
                Ok(())
            }
            Statement::Input { prompt, result_name } => {
                print!("{}", prompt);
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .map_err(|e| format!("Failed to read input: {}", e))?;
                
                let trimmed = input.trim().to_string();
                
                // Try to parse as number, otherwise store as string
                if let Ok(num) = trimmed.parse::<f64>() {
                    self.variables.insert(result_name.clone(), num);
                } else {
                    self.intents.insert(result_name.clone(), trimmed);
                }
                
                Ok(())
            }
            _ => Err(format!("Extension statement not implemented"))
        }
    }
}