// src/bin/emojitape_interpreter.rs
use crate::parser::emojitape_parser::parse_emojitape;
use emojitape_interpreter::tokenizer::tokenize;
use emojitape_interpreter::parser;
use emojitape_interpreter::interpreter;
use emojitape_interpreter::cocci_converter; // Import the new module

use std::env;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_emojitape_file> | --cocci <path_to_cocci_file>", args[0]);
        return Err("No input file provided".into());
    }

    if args[1] == "--cocci" {
        if args.len() < 3 {
            eprintln!("Usage: {} --cocci <path_to_cocci_file>", args[0]);
            return Err("No .cocci file path provided".into());
        }
        let cocci_path = &args[2];
        let mut file = fs::File::open(cocci_path)?;
        let mut cocci_content = String::new();
        file.read_to_string(&mut cocci_content)?;

        println!("Converting .cocci file: {}", cocci_path);
        let emoji_tokens = cocci_converter::cocci_to_emojis(&cocci_content)?;
        println!("Converted to Emoji Tokens: {:?}", emoji_tokens);

        // Placeholder for converting back to .cocci and self-rewriting
        let regenerated_cocci = cocci_converter::emojis_to_cocci(&emoji_tokens)?;
        println!("\nRegenerated .cocci content:\n{}", regenerated_cocci);

        // For self-rewriting, we would ask for user confirmation here
        // and then write the regenerated_cocci back to cocci_path.
        // For now, we just print it.

        Ok(())
    } else {
        let emojitape_path = &args[1];

        let mut file = fs::File::open(emojitape_path)?;
        let mut emojitape_content = String::new();
        file.read_to_string(&mut emojitape_content)?;

        println!("Parsing Emojitape from: {emojitape_path}");
        let tokens = tokenize(&emojitape_content);
        // println!("Tokens: {:?}", tokens); // For debugging

        let emojitape = parse_emojitape(tokens);
        println!("Parsed Emojitape: {emojitape:?}"); // For debugging

        let rendered_emojitape = emojitape.render();
        println!("\n--- Rendered Emojitape ---\n{rendered_emojitape}");

        interpreter::execute_emojitape(&emojitape)?;

        Ok(())
    }
}
