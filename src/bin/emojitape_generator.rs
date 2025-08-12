use emojitape_interpreter::parser;
use std::fs;
use emojitape_interpreter::tokenizer::tokenize;
use std::io::Read;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_text_file> <output_emojitape_file>", args[0]);
        return Err("Missing arguments: input_text_file and output_emojitape_file are required.".into());
    }

    let input_path = &args[1];
    let output_path = &args[2];

    println!("Reading combined text from: {}", input_path);
    let mut file = fs::File::open(input_path)?;
    let mut combined_text = String::new();
    file.read_to_string(&mut combined_text)?;

    println!("Tokenizing combined text...");
    let tokens = tokenize(&combined_text);
    println!("Tokens: {:#?}", tokens); // For debugging token stream

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
    fs::write(output_path, emojitape.render())?;
    println!("Full Emojitape output written to: {}", output_path);

    Ok(())
}
