// src/main.rs

mod parser;
mod interpreter;
mod types;

use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let emojitape_path = "full_generated_emojitape.emojitape"; // Assuming the generated file name

    let mut file = fs::File::open(emojitape_path)?;
    let mut emojitape_content = String::new();
    file.read_to_string(&mut emojitape_content)?;

    println!("Parsing Emojitape from: {}", emojitape_path);
    let tokens = parser::tokenize(&emojitape_content);
    // println!("Tokens: {:?}", tokens); // For debugging

    let emojitape = parser::parse_emojitape(tokens);
    // println!("Parsed Emojitape: {:?}", emojitape); // For debugging

    interpreter::execute_emojitape(&emojitape)?;

    Ok(())
}