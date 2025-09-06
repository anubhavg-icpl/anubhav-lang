use anubhav_lang::core::Interpreter;
use anubhav_lang::lang::{Lexer, Parser};
use std::env;
use std::fs;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file.anubhav>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .expect(&format!("Failed to read file: {}", filename));
    
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer);
    
    match parser.parse() {
        Ok(statements) => {
            let mut interpreter = Interpreter::new();
            if let Err(e) = interpreter.execute(statements) {
                eprintln!("Runtime error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }
}