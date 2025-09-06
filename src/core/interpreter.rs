use crate::lang::lexer::{Lexer, Token};
use crate::lang::parser::{Expression, Parser, Statement};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub struct Interpreter {
    pub(crate) intents: HashMap<String, String>,
    pub(crate) calculations: HashMap<String, f64>,
    pub(crate) variables: HashMap<String, f64>,
    pub(crate) arrays: HashMap<String, Vec<f64>>,
    pub(crate) dicts: HashMap<String, HashMap<String, f64>>, // Dictionary storage
    pub(crate) functions: HashMap<String, (Vec<String>, Vec<Statement>)>, // name -> (parameters, body)
    pub(crate) call_stack: Vec<HashMap<String, f64>>, // Stack of local variable scopes
    pub(crate) random_seed: u64,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            intents: HashMap::new(),
            calculations: HashMap::new(),
            variables: HashMap::new(),
            arrays: HashMap::new(),
            dicts: HashMap::new(),
            functions: HashMap::new(),
            call_stack: Vec::new(),
            random_seed: 12345, // Initial seed
        }
    }

    pub fn execute(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Statement::IntentDeclaration { name, message } => {
                    self.intents.insert(name, message);
                }
                Statement::ManifestCall {
                    intent_name,
                    with_message,
                } => {
                    if let Some(message) = self.intents.get(&intent_name) {
                        if let Some(context) = with_message {
                            println!("{message} {context}");
                        } else {
                            println!("{message}");
                        }
                    } else if let Some(result) = self.calculations.get(&intent_name) {
                        if let Some(context) = with_message {
                            println!("{result} {context}");
                        } else {
                            println!("{result}");
                        }
                    } else {
                        return Err(format!("Intent '{intent_name}' not found"));
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
                            let var_name = &part[2..part.len() - 1];
                            if let Some(msg) = self.intents.get(var_name) {
                                combined.push_str(msg);
                            } else if let Some(val) = self.calculations.get(var_name) {
                                combined.push_str(&val.to_string());
                            } else if let Some(val) = self.variables.get(var_name) {
                                combined.push_str(&val.to_string());
                            } else {
                                combined.push_str(&format!("<{var_name} not found>"));
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
                        match self.execute(body.clone()) {
                            Err(e) if e == "BREAK" => break,
                            Err(e) if e == "CONTINUE" => continue,
                            Err(e) => return Err(e),
                            Ok(_) => {}
                        }
                    }
                }
                Statement::If {
                    condition,
                    then_body,
                    else_body,
                } => {
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
                            let var_name = &item[2..item.len() - 1];
                            if let Some(msg) = self.intents.get(var_name) {
                                output.push_str(msg);
                            } else if let Some(val) = self.calculations.get(var_name) {
                                output.push_str(&val.to_string());
                            } else if let Some(val) = self.variables.get(var_name) {
                                output.push_str(&val.to_string());
                            } else {
                                output.push_str(&format!("<{var_name} not found>"));
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
                        match self.execute(body.clone()) {
                            Err(e) if e == "BREAK" => break,
                            Err(e) if e == "CONTINUE" => continue,
                            Err(e) => return Err(e),
                            Ok(_) => {}
                        }
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
                Statement::For {
                    variable,
                    start,
                    end,
                    step,
                    body,
                } => {
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
                            match self.execute(body.clone()) {
                                Err(e) if e == "BREAK" => break,
                                Err(e) if e == "CONTINUE" => {}
                                Err(e) => return Err(e),
                                Ok(_) => {}
                            }
                            current += step_val;
                        }
                    } else if step_val < 0.0 {
                        while current >= end_val {
                            self.variables.insert(variable.clone(), current);
                            match self.execute(body.clone()) {
                                Err(e) if e == "BREAK" => break,
                                Err(e) if e == "CONTINUE" => {}
                                Err(e) => return Err(e),
                                Ok(_) => {}
                            }
                            current += step_val;
                        }
                    }
                }
                Statement::Assert { condition, message } => {
                    let result = self.evaluate_expression(&condition)?;
                    if result == 0.0 {
                        let error_msg = if let Some(msg) = message {
                            format!("Assertion failed: {msg}")
                        } else {
                            "Assertion failed".to_string()
                        };
                        return Err(error_msg);
                    }
                    println!("✓ Assertion passed");
                }
                Statement::TryCatch {
                    try_body,
                    catch_body,
                } => {
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
                Statement::StringTransform {
                    name,
                    operation,
                    source,
                } => {
                    let source_string = if let Some(intent_str) = self.intents.get(&source) {
                        intent_str.clone()
                    } else {
                        // If not found as intent, treat as literal string
                        source.clone()
                    };

                    let result = match operation.as_str() {
                        "UPPERCASE" => source_string.to_uppercase(),
                        "LOWERCASE" => source_string.to_lowercase(),
                        _ => return Err(format!("Unknown string operation: {operation}")),
                    };

                    self.intents.insert(name.clone(), result);
                }
                Statement::Switch {
                    expression,
                    cases,
                    default_case,
                } => {
                    let switch_value = self.evaluate_expression(&expression)?;

                    let mut executed = false;

                    for (case_value, case_body) in cases {
                        let case_val = self.evaluate_expression(&case_value)?;
                        if switch_value == case_val {
                            self.execute(case_body.clone())?;
                            executed = true;
                            break;
                        }
                    }

                    if !executed {
                        if let Some(default_body) = default_case {
                            self.execute(default_body.clone())?;
                        }
                    }
                }
                Statement::ArrayCreate { name } => {
                    self.arrays.insert(name.clone(), Vec::new());
                }
                Statement::ArrayPush { array_name, value } => {
                    let val = self.evaluate_expression(&value)?;
                    if let Some(array) = self.arrays.get_mut(&array_name) {
                        array.push(val);
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::ArrayPop {
                    array_name,
                    result_name,
                } => {
                    if let Some(array) = self.arrays.get_mut(&array_name) {
                        if let Some(val) = array.pop() {
                            self.variables.insert(result_name.clone(), val);
                        } else {
                            return Err(format!("Array '{array_name}' is empty"));
                        }
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::ArraySize {
                    array_name,
                    result_name,
                } => {
                    if let Some(array) = self.arrays.get(&array_name) {
                        self.variables
                            .insert(result_name.clone(), array.len() as f64);
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::ArrayGet {
                    array_name,
                    index,
                    result_name,
                } => {
                    let idx = self.evaluate_expression(&index)? as usize;
                    if let Some(array) = self.arrays.get(&array_name) {
                        if idx < array.len() {
                            self.variables.insert(result_name.clone(), array[idx]);
                        } else {
                            return Err(format!(
                                "Array index {idx} out of bounds for array '{array_name}'"
                            ));
                        }
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::ArraySet {
                    array_name,
                    index,
                    value,
                } => {
                    let idx = self.evaluate_expression(&index)? as usize;
                    let val = self.evaluate_expression(&value)?;
                    if let Some(array) = self.arrays.get_mut(&array_name) {
                        if idx < array.len() {
                            array[idx] = val;
                        } else {
                            return Err(format!(
                                "Array index {idx} out of bounds for array '{array_name}'"
                            ));
                        }
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Import { filename } => {
                    // Read and execute the imported file
                    let content = fs::read_to_string(&filename)
                        .map_err(|e| format!("Failed to read file '{filename}': {e}"))?;

                    let lexer = Lexer::new(content);
                    let mut parser = Parser::new(lexer);
                    let imported_statements = parser
                        .parse()
                        .map_err(|e| format!("Parse error in '{filename}': {e}"))?;

                    // Execute the imported statements
                    self.execute(imported_statements)?;
                }
                Statement::Export { items, filename } => {
                    // Create export data
                    let mut export_content = String::new();
                    export_content.push_str("# Exported from Anubhav\n");

                    for item in &items {
                        if let Some(intent_value) = self.intents.get(item) {
                            export_content
                                .push_str(&format!("INTENT {item} \"{intent_value}\"\n"));
                        } else if let Some(calc_value) = self.calculations.get(item) {
                            export_content.push_str(&format!("STORE {item} {calc_value}\n"));
                        } else if let Some(var_value) = self.variables.get(item) {
                            export_content.push_str(&format!("STORE {item} {var_value}\n"));
                        } else if let Some(array_value) = self.arrays.get(item) {
                            export_content.push_str(&format!("ARRAY {item}\n"));
                            for value in array_value {
                                export_content.push_str(&format!("PUSH {item} {value}\n"));
                            }
                        }
                    }

                    // Write to file
                    fs::write(&filename, export_content)
                        .map_err(|e| format!("Failed to write to file '{filename}': {e}"))?;

                    println!("Exported {} items to {}", items.len(), filename);
                }
                Statement::Break => {
                    return Err("BREAK".to_string()); // Special error code for break
                }
                Statement::Continue => {
                    return Err("CONTINUE".to_string()); // Special error code for continue
                }
                Statement::FunctionDefinition {
                    name,
                    parameters,
                    body,
                } => {
                    self.functions
                        .insert(name.clone(), (parameters.clone(), body.clone()));
                    println!(
                        "Function '{}' defined with {} parameters",
                        name,
                        parameters.len()
                    );
                }
                Statement::FunctionCall {
                    function_name,
                    arguments,
                    result_name,
                } => {
                    if let Some((params, func_body)) = self.functions.get(&function_name).cloned() {
                        // Evaluate arguments
                        let mut arg_values = Vec::new();
                        for arg in &arguments {
                            arg_values.push(self.evaluate_expression(arg)?);
                        }

                        // Check parameter count
                        if arg_values.len() != params.len() {
                            return Err(format!(
                                "Function '{}' expects {} parameters, got {}",
                                function_name,
                                params.len(),
                                arg_values.len()
                            ));
                        }

                        // Create new local scope
                        let mut local_vars = HashMap::new();
                        for (i, param) in params.iter().enumerate() {
                            local_vars.insert(param.clone(), arg_values[i]);
                        }
                        self.call_stack.push(local_vars);

                        // Execute function body
                        let mut return_value = 0.0;
                        match self.execute(func_body) {
                            Ok(_) => {}
                            Err(e) if e.starts_with("RETURN:") => {
                                // Extract return value
                                if let Ok(val) = e[7..].parse::<f64>() {
                                    return_value = val;
                                }
                            }
                            Err(e) => {
                                self.call_stack.pop();
                                return Err(e);
                            }
                        }

                        // Pop local scope
                        self.call_stack.pop();

                        // Store result if specified
                        if let Some(result_var) = result_name {
                            self.variables.insert(result_var.clone(), return_value);
                        }
                    } else {
                        return Err(format!("Function '{function_name}' not found"));
                    }
                }
                Statement::Return { value } => {
                    let return_val = if let Some(expr) = value {
                        self.evaluate_expression(&expr)?
                    } else {
                        0.0
                    };
                    return Err(format!("RETURN:{return_val}")); // Special error code for return
                }
                Statement::ArraySort {
                    array_name,
                    ascending,
                } => {
                    if let Some(array) = self.arrays.get_mut(&array_name) {
                        if ascending {
                            array.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        } else {
                            array.sort_by(|a, b| b.partial_cmp(a).unwrap());
                        }
                        println!(
                            "Array '{}' sorted {}",
                            array_name,
                            if ascending { "ascending" } else { "descending" }
                        );
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::ArrayFilter {
                    array_name,
                    condition,
                    result_array,
                } => {
                    if let Some(source_array) = self.arrays.get(&array_name).cloned() {
                        let mut filtered_array = Vec::new();

                        for (index, &value) in source_array.iter().enumerate() {
                            // Set a temporary variable for the current array element
                            let old_item_value = self.variables.get("item").copied();
                            let old_index_value = self.variables.get("index").copied();

                            self.variables.insert("item".to_string(), value);
                            self.variables.insert("index".to_string(), index as f64);

                            // Evaluate condition
                            let condition_result = self.evaluate_expression(&condition)?;

                            if condition_result != 0.0 {
                                filtered_array.push(value);
                            }

                            // Restore old values
                            if let Some(old_val) = old_item_value {
                                self.variables.insert("item".to_string(), old_val);
                            } else {
                                self.variables.remove("item");
                            }
                            if let Some(old_val) = old_index_value {
                                self.variables.insert("index".to_string(), old_val);
                            } else {
                                self.variables.remove("index");
                            }
                        }

                        self.arrays.insert(result_array.clone(), filtered_array);
                        println!(
                            "Filtered {} into {} with {} elements",
                            array_name,
                            result_array,
                            self.arrays.get(&result_array).unwrap().len()
                        );
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::ArrayReverse { array_name } => {
                    if let Some(array) = self.arrays.get_mut(&array_name) {
                        array.reverse();
                        println!("Array '{array_name}' reversed");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::ArrayMap {
                    array_name,
                    expression,
                    result_array,
                } => {
                    if let Some(source_array) = self.arrays.get(&array_name).cloned() {
                        let mut mapped_array = Vec::new();

                        for (index, &value) in source_array.iter().enumerate() {
                            // Set temporary variables
                            let old_item_value = self.variables.get("item").copied();
                            let old_index_value = self.variables.get("index").copied();

                            self.variables.insert("item".to_string(), value);
                            self.variables.insert("index".to_string(), index as f64);

                            // Evaluate expression
                            let mapped_value = self.evaluate_expression(&expression)?;
                            mapped_array.push(mapped_value);

                            // Restore old values
                            if let Some(old_val) = old_item_value {
                                self.variables.insert("item".to_string(), old_val);
                            } else {
                                self.variables.remove("item");
                            }
                            if let Some(old_val) = old_index_value {
                                self.variables.insert("index".to_string(), old_val);
                            } else {
                                self.variables.remove("index");
                            }
                        }

                        self.arrays.insert(result_array.clone(), mapped_array);
                        println!(
                            "Mapped {} into {} with {} elements",
                            array_name,
                            result_array,
                            self.arrays.get(&result_array).unwrap().len()
                        );
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::ArraySum {
                    array_name,
                    result_name,
                } => {
                    if let Some(array) = self.arrays.get(&array_name) {
                        let sum: f64 = array.iter().sum();
                        self.variables.insert(result_name.clone(), sum);
                        println!("Sum of array '{array_name}' is {sum}");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::ArrayJoin {
                    array_name,
                    separator,
                    result_name,
                } => {
                    if let Some(array) = self.arrays.get(&array_name) {
                        let joined = array
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(&separator);
                        self.intents.insert(result_name.clone(), joined.clone());
                        println!("Joined array '{array_name}' into string: {joined}");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::DictCreate { name } => {
                    self.dicts.insert(name.clone(), HashMap::new());
                    println!("Dictionary '{name}' created");
                }
                Statement::DictPut {
                    dict_name,
                    key,
                    value,
                } => {
                    let val = self.evaluate_expression(&value)?;
                    if let Some(dict) = self.dicts.get_mut(&dict_name) {
                        dict.insert(key.clone(), val);
                        println!("Set {dict_name}['{key}'] = {val}");
                    } else {
                        return Err(format!("Dictionary '{dict_name}' not found"));
                    }
                }
                Statement::DictFetch {
                    dict_name,
                    key,
                    result_name,
                } => {
                    if let Some(dict) = self.dicts.get(&dict_name) {
                        if let Some(&value) = dict.get(&key) {
                            self.variables.insert(result_name.clone(), value);
                        } else {
                            return Err(format!(
                                "Key '{key}' not found in dictionary '{dict_name}'"
                            ));
                        }
                    } else {
                        return Err(format!("Dictionary '{dict_name}' not found"));
                    }
                }
                Statement::DictKeys {
                    dict_name,
                    result_array,
                } => {
                    if let Some(_dict) = self.dicts.get(&dict_name) {
                        let keys: Vec<f64> = Vec::new(); // Keys as array indices for now
                        self.arrays.insert(result_array.clone(), keys);
                        println!("Extracted keys from '{dict_name}'");
                    } else {
                        return Err(format!("Dictionary '{dict_name}' not found"));
                    }
                }
                Statement::DictValues {
                    dict_name,
                    result_array,
                } => {
                    if let Some(dict) = self.dicts.get(&dict_name) {
                        let values: Vec<f64> = dict.values().copied().collect();
                        self.arrays.insert(result_array.clone(), values);
                        println!(
                            "Extracted values from '{dict_name}' to array '{result_array}'"
                        );
                    } else {
                        return Err(format!("Dictionary '{dict_name}' not found"));
                    }
                }
                Statement::DictDelete { dict_name, key } => {
                    if let Some(dict) = self.dicts.get_mut(&dict_name) {
                        dict.remove(&key);
                        println!("Deleted key '{key}' from '{dict_name}'");
                    } else {
                        return Err(format!("Dictionary '{dict_name}' not found"));
                    }
                }
                Statement::ReadFile {
                    filename,
                    result_name,
                } => match fs::read_to_string(&filename) {
                    Ok(content) => {
                        self.intents.insert(result_name.clone(), content.clone());
                        println!("Read {} bytes from '{}'", content.len(), filename);
                    }
                    Err(e) => return Err(format!("Failed to read file '{filename}': {e}")),
                },
                Statement::WriteFile { filename, content } => {
                    let actual_content = if content.starts_with("${") && content.ends_with("}") {
                        let var_name = &content[2..content.len() - 1];
                        self.intents
                            .get(var_name)
                            .cloned()
                            .unwrap_or_else(|| content.clone())
                    } else {
                        content.clone()
                    };

                    match fs::write(&filename, actual_content.as_bytes()) {
                        Ok(_) => println!("Wrote {} bytes to '{}'", actual_content.len(), filename),
                        Err(e) => {
                            return Err(format!("Failed to write to file '{filename}': {e}"));
                        }
                    }
                }
                Statement::AppendFile { filename, content } => {
                    use std::fs::OpenOptions;
                    use std::io::Write;

                    let actual_content = if content.starts_with("${") && content.ends_with("}") {
                        let var_name = &content[2..content.len() - 1];
                        self.intents
                            .get(var_name)
                            .cloned()
                            .unwrap_or_else(|| content.clone())
                    } else {
                        content.clone()
                    };

                    match OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&filename)
                        .and_then(|mut file| file.write_all(actual_content.as_bytes()))
                    {
                        Ok(_) => {
                            println!("Appended {} bytes to '{}'", actual_content.len(), filename)
                        }
                        Err(e) => {
                            return Err(format!("Failed to append to file '{filename}': {e}"));
                        }
                    }
                }
                Statement::FileExists {
                    filename,
                    result_name,
                } => {
                    let exists = Path::new(&filename).exists();
                    self.variables
                        .insert(result_name.clone(), if exists { 1.0 } else { 0.0 });
                    println!("File '{filename}' exists: {exists}");
                }
                Statement::Sleep { milliseconds } => {
                    let ms = self.evaluate_expression(&milliseconds)? as u64;
                    println!("Sleeping for {ms} ms...");
                    thread::sleep(Duration::from_millis(ms));
                }
                Statement::Input {
                    prompt,
                    result_name,
                } => {
                    print!("{prompt}");
                    io::Write::flush(&mut io::stdout()).unwrap();

                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .map_err(|e| format!("Failed to read input: {e}"))?;

                    let trimmed = input.trim().to_string();

                    // Try to parse as number, otherwise store as string
                    if let Ok(num) = trimmed.parse::<f64>() {
                        self.variables.insert(result_name.clone(), num);
                    } else {
                        self.intents.insert(result_name.clone(), trimmed);
                    }
                }
                Statement::GetType {
                    variable,
                    result_name,
                } => {
                    let type_str = if self.variables.contains_key(&variable) {
                        "number"
                    } else if self.intents.contains_key(&variable) {
                        "string"
                    } else if self.arrays.contains_key(&variable) {
                        "array"
                    } else if self.dicts.contains_key(&variable) {
                        "dictionary"
                    } else {
                        "undefined"
                    };
                    self.intents
                        .insert(result_name.clone(), type_str.to_string());
                }
                Statement::ParseNumber {
                    source,
                    result_name,
                } => {
                    let value = if let Some(str_val) = self.intents.get(&source) {
                        str_val.parse::<f64>().unwrap_or(0.0)
                    } else {
                        source.parse::<f64>().unwrap_or(0.0)
                    };
                    self.variables.insert(result_name.clone(), value);
                }
                Statement::Range {
                    start,
                    end,
                    step,
                    result_array,
                } => {
                    let start_val = self.evaluate_expression(&start)?;
                    let end_val = self.evaluate_expression(&end)?;
                    let step_val = if let Some(s) = step {
                        self.evaluate_expression(&s)?
                    } else {
                        1.0
                    };

                    let mut range_array = Vec::new();
                    let mut current = start_val;
                    while current <= end_val {
                        range_array.push(current);
                        current += step_val;
                    }
                    self.arrays.insert(result_array.clone(), range_array);
                    println!("Generated range array '{result_array}'");
                }
                Statement::Unique {
                    array_name,
                    result_array,
                } => {
                    if let Some(array) = self.arrays.get(&array_name) {
                        let mut unique = Vec::new();
                        for &val in array {
                            if !unique.contains(&val) {
                                unique.push(val);
                            }
                        }
                        self.arrays.insert(result_array.clone(), unique);
                        println!("Created unique array '{result_array}'");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Concat {
                    array1,
                    array2,
                    result_array,
                } => {
                    if let (Some(arr1), Some(arr2)) =
                        (self.arrays.get(&array1), self.arrays.get(&array2))
                    {
                        let mut concatenated = arr1.clone();
                        concatenated.extend(arr2);
                        self.arrays.insert(result_array.clone(), concatenated);
                        println!("Concatenated arrays into '{result_array}'");
                    } else {
                        return Err("Array not found".to_string());
                    }
                }
                Statement::Take {
                    array_name,
                    count,
                    result_array,
                } => {
                    let n = self.evaluate_expression(&count)? as usize;
                    if let Some(array) = self.arrays.get(&array_name) {
                        let taken: Vec<f64> = array.iter().take(n).copied().collect();
                        self.arrays.insert(result_array.clone(), taken);
                        println!("Took {n} elements into '{result_array}'");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Drop {
                    array_name,
                    count,
                    result_array,
                } => {
                    let n = self.evaluate_expression(&count)? as usize;
                    if let Some(array) = self.arrays.get(&array_name) {
                        let dropped: Vec<f64> = array.iter().skip(n).copied().collect();
                        self.arrays.insert(result_array.clone(), dropped);
                        println!("Dropped {n} elements, result in '{result_array}'");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Find {
                    array_name,
                    condition,
                    result_name,
                } => {
                    // Clone the array to avoid borrowing issues
                    let array_clone = self.arrays.get(&array_name).cloned();
                    if let Some(array) = array_clone {
                        for &value in &array {
                            self.variables.insert("item".to_string(), value);
                            if self.evaluate_expression(&condition)? != 0.0 {
                                self.variables.insert(result_name.clone(), value);
                                println!("Found value: {value}");
                                break;
                            }
                        }
                        self.variables.remove("item");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Average {
                    array_name,
                    result_name,
                } => {
                    if let Some(array) = self.arrays.get(&array_name) {
                        if !array.is_empty() {
                            let avg: f64 = array.iter().sum::<f64>() / array.len() as f64;
                            self.variables.insert(result_name.clone(), avg);
                            println!("Average of '{array_name}' is {avg}");
                        } else {
                            self.variables.insert(result_name.clone(), 0.0);
                        }
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Clear { target } => {
                    if self.arrays.contains_key(&target) {
                        self.arrays.get_mut(&target).unwrap().clear();
                        println!("Cleared array '{target}'");
                    } else if self.dicts.contains_key(&target) {
                        self.dicts.get_mut(&target).unwrap().clear();
                        println!("Cleared dictionary '{target}'");
                    } else {
                        return Err(format!("Target '{target}' not found"));
                    }
                }
                Statement::Shuffle { array_name } => {
                    // Generate all random indices first to avoid borrowing issues
                    let len = self.arrays.get(&array_name).map(|a| a.len());
                    if let Some(len) = len {
                        let mut swaps = Vec::new();
                        for i in 0..len {
                            let j = (self.next_random() * len as f64) as usize;
                            swaps.push((i, j.min(len - 1)));
                        }

                        // Now apply the swaps
                        if let Some(array) = self.arrays.get_mut(&array_name) {
                            for (i, j) in swaps {
                                array.swap(i, j);
                            }
                        }
                        println!("Shuffled array '{array_name}'");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Clone {
                    source,
                    destination,
                } => {
                    if let Some(array) = self.arrays.get(&source).cloned() {
                        self.arrays.insert(destination.clone(), array);
                        println!("Cloned array '{source}' to '{destination}'");
                    } else if let Some(dict) = self.dicts.get(&source).cloned() {
                        self.dicts.insert(destination.clone(), dict);
                        println!("Cloned dictionary '{source}' to '{destination}'");
                    } else {
                        return Err(format!("Source '{source}' not found"));
                    }
                }
                Statement::Fold {
                    array_name,
                    initial,
                    operation,
                    result_name,
                } => {
                    // Simplified fold implementation - clone array to avoid borrowing issues
                    let array_clone = self.arrays.get(&array_name).cloned();
                    if let Some(array) = array_clone {
                        let mut accumulator = self.evaluate_expression(&initial)?;
                        for &value in &array {
                            self.variables.insert("acc".to_string(), accumulator);
                            self.variables.insert("item".to_string(), value);
                            accumulator = self.evaluate_expression(&operation)?;
                        }
                        self.variables.insert(result_name.clone(), accumulator);
                        self.variables.remove("acc");
                        self.variables.remove("item");
                        println!("Folded array '{array_name}' into result: {accumulator}");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Zip {
                    array1,
                    array2,
                    result_array,
                } => {
                    if let (Some(arr1), Some(arr2)) =
                        (self.arrays.get(&array1), self.arrays.get(&array2))
                    {
                        let mut zipped = Vec::new();
                        let len = arr1.len().min(arr2.len());
                        for i in 0..len {
                            zipped.push(arr1[i]);
                            zipped.push(arr2[i]);
                        }
                        self.arrays.insert(result_array.clone(), zipped);
                        println!(
                            "Zipped arrays '{array1}' and '{array2}' into '{result_array}'"
                        );
                    } else {
                        return Err("One or both arrays not found".to_string());
                    }
                }
                Statement::Flatten {
                    array_name,
                    result_array,
                } => {
                    // For simplicity, just copy the array (would need nested array support for true flatten)
                    if let Some(array) = self.arrays.get(&array_name).cloned() {
                        self.arrays.insert(result_array.clone(), array);
                        println!("Flattened array '{array_name}' into '{result_array}'");
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Count {
                    array_name,
                    condition,
                    result_name,
                } => {
                    let array_clone = self.arrays.get(&array_name).cloned();
                    if let Some(array) = array_clone {
                        let mut count = 0;
                        for &value in &array {
                            self.variables.insert("item".to_string(), value);
                            if self.evaluate_expression(&condition)? != 0.0 {
                                count += 1;
                            }
                        }
                        self.variables.remove("item");
                        self.variables.insert(result_name.clone(), count as f64);
                        println!(
                            "Counted {count} items matching condition in '{array_name}'"
                        );
                    } else {
                        return Err(format!("Array '{array_name}' not found"));
                    }
                }
                Statement::Replace {
                    text,
                    pattern,
                    replacement,
                    result_name,
                } => {
                    if let Some(target_str) = self.intents.get(&text) {
                        let replaced = target_str.replace(&pattern, &replacement);
                        self.intents.insert(result_name.clone(), replaced.clone());
                        println!("Replaced '{pattern}' with '{replacement}' in string");
                    } else {
                        return Err(format!("String '{text}' not found"));
                    }
                }
                Statement::Split {
                    text,
                    delimiter,
                    result_array,
                } => {
                    if let Some(string) = self.intents.get(&text) {
                        let parts: Vec<&str> = string.split(&delimiter).collect();
                        // Convert to array of indices (since we can't store strings in arrays)
                        let indices: Vec<f64> = (0..parts.len()).map(|i| i as f64).collect();
                        self.arrays.insert(result_array.clone(), indices);
                        println!(
                            "Split string '{}' by '{}' into {} parts",
                            text,
                            delimiter,
                            parts.len()
                        );
                    } else {
                        return Err(format!("String '{text}' not found"));
                    }
                }
            }
        }
        Ok(())
    }

    fn next_random(&mut self) -> f64 {
        // Linear congruential generator: (a * seed + c) % m
        // Using constants from Numerical Recipes
        self.random_seed =
            ((self.random_seed.wrapping_mul(1664525)).wrapping_add(1013904223)) % (1u64 << 32);
        (self.random_seed as f64) / ((1u64 << 32) as f64)
    }

    pub(crate) fn evaluate_expression(&mut self, expr: &Expression) -> Result<f64, String> {
        match expr {
            Expression::Number(n) => Ok(*n),
            Expression::Recall(name) => {
                // Check local scopes first (most recent first)
                if let Some(local_scope) = self.call_stack.last() {
                    if let Some(value) = local_scope.get(name) {
                        return Ok(*value);
                    }
                }

                // Fall back to global scope
                self.variables
                    .get(name)
                    .or_else(|| self.calculations.get(name))
                    .copied()
                    .ok_or_else(|| format!("Variable '{name}' not found"))
            }
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
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
                    Token::And => Ok(if left_val != 0.0 && right_val != 0.0 {
                        1.0
                    } else {
                        0.0
                    }),
                    Token::Or => Ok(if left_val != 0.0 || right_val != 0.0 {
                        1.0
                    } else {
                        0.0
                    }),
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
                                Err(format!("String '{name}' not found for LENGTH"))
                            }
                        } else {
                            Err("LENGTH function error".to_string())
                        }
                    }
                    Token::Size => {
                        // SIZE function - get array size
                        if let Expression::Recall(name) = &**left {
                            if let Some(array) = self.arrays.get(name) {
                                Ok(array.len() as f64)
                            } else {
                                Err(format!("Array '{name}' not found for SIZE"))
                            }
                        } else {
                            Err("SIZE function error".to_string())
                        }
                    }
                    _ => Err(format!("Invalid operator: {operator:?}")),
                }
            }
        }
    }
}
