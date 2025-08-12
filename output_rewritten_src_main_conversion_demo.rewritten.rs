// RUST_CODE_BLOCK_UNPARSABLE:// src/main_conversion_demo.rs

use std::fs;
use std::io::Read;
use std::env;

mod rust_to_emoji_workaround;
mod emoji_to_rust_standalone;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Rust to Emoji and Back Conversion Demo ---");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <rust_file1> <rust_file2> ...", args[0]);
        return Err("No input Rust files provided.".into());
    }

    for i in 1..args.len() {
        let input_rust_file = &args[i];
        let output_rust_file = format!("output_rewritten_{}", input_rust_file.replace("/", "_").replace(".rs", ".rewritten.rs"));

        println!("\n--- Processing file: {} ---", input_rust_file);

        // 1. Read original Rust code
        let mut file = fs::File::open(input_rust_file)?;
        let mut original_rust_code = String::new();
        file.read_to_string(&mut original_rust_code)?;
        println!("\nOriginal Rust Code:\n{}", original_rust_code);

        // 2. Convert Rust to Emojis
        println!("\nConverting Rust to Emojis...");
        let emoji_tokens = rust_to_emoji_workaround::rust_code_to_emojis(&original_rust_code)?;
        println!("Converted Emoji Tokens: {:?}", emoji_tokens);

        // 3. Convert Emojis back to Rust
        println!("\nConverting Emojis back to Rust...");
        let regenerated_rust_code = emoji_to_rust_standalone::emojis_to_rust_code(&emoji_tokens)?;
        println!("Regenerated Rust Code:\n{}", regenerated_rust_code);

        // Save Emojitape to file
        let emojitape_output_file = format!("tapes/output_emojitape_{}", input_rust_file.replace("/", "_").replace(".rs", ".emojitape"));
        let emojitape_string = emoji_to_rust_standalone::emojitape_to_string(&emoji_tokens);
        fs::write(&emojitape_output_file, &emojitape_string)?;
        println!("Emojitape written to: {}", emojitape_output_file);

        // 4. Write regenerated Rust code to a new file
        fs::write(&output_rust_file, &regenerated_rust_code)?;
        println!("\nRegenerated Rust code written to: {}", output_rust_file);
    }

    println!("\n--- Demo Complete ---");

    Ok(())
}

