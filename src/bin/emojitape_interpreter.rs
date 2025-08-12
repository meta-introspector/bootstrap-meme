// src/bin/emojitape_interpreter.rs
use emojitape_interpreter::tokenizer::tokenize;
use emojitape_interpreter::parser;
use emojitape_interpreter::interpreter;

use std::env;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_emojitape_file>", args[0]);
        return Err("No input file provided".into());
    }
    let emojitape_path = &args[1];

    let mut file = fs::File::open(emojitape_path)?;
    let mut emojitape_content = String::new();
    file.read_to_string(&mut emojitape_content)?;

    println!("Parsing Emojitape from: {}", emojitape_path);
    let tokens = tokenize(&emojitape_content);
    // println!("Tokens: {:?}", tokens); // For debugging

    let emojitape = parser::parse_emojitape(tokens);
    println!("Parsed Emojitape: {:?}", emojitape); // For debugging

    let rendered_emojitape = emojitape.render();
    println!("\n--- Rendered Emojitape ---\n{}", rendered_emojitape);

    interpreter::execute_emojitape(&emojitape)?;

    Ok(())
}