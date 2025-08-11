use emojitape_interpreter::parser;
use emojitape_interpreter::types::{Emojitape, Token};
use std::fs;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "../bootstrap_docs.txt"; // Path to the combined text file

    println!("Reading combined text from: {}", input_path);
    let mut file = fs::File::open(input_path)?;
    let mut combined_text = String::new();
    file.read_to_string(&mut combined_text)?;

    println!("Tokenizing combined text...");
    let tokens = parser::tokenize(&combined_text);

    println!("Parsing Emojitape...");
    let emojitape = parser::parse_emojitape(tokens);

    // Now, implement the "small files" strategy.
    // For this iteration, I will simply print the structured Emojitape,
    // and then manually split it into smaller files in the next step.
    // This allows me to inspect the output and refine the chunking logic.

    println!("\n--- Generated Emojitape Structure ---");
    println!("Prelude: {:#?}", emojitape.prelude);
    println!("WASM Compiler Prelude: {:#?}", emojitape.wasm_compiler_prelude);
    println!("Rules: {:#?}", emojitape.rules);
    println!("World Tape (partial view): {:#?}", &emojitape.world_tape[0..std::cmp::min(emojitape.world_tape.len(), 50)]); // Print first 50 tokens
    println!("Full World Tape Length: {}", emojitape.world_tape.len());
    println!("Generated WAT Block: {:#?}", emojitape.generated_wat_block);
    println!("Clues & Keys: {:#?}", emojitape.clues_keys);
    println!("ZOS Export Definition: {:#?}", emojitape.zos_export_definition);
    println!("ZOS Export Implementation: {:#?}", emojitape.zos_export_implementation);
    println!("Self-Reproducing Footer: {:#?}", emojitape.self_reproducing_footer);
    println!("\n--- End Generated Emojitape Structure ---");

    // Optionally, write the full emojitape debug output to a file for inspection
    let full_output_path = "full_generated_emojitape_debug.txt";
    fs::write(full_output_path, format!("{:#?}", emojitape))?;
    println!("Full Emojitape debug output written to: {}", full_output_path);

    Ok(())
}
