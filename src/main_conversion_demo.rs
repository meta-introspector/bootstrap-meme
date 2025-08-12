// src/main_conversion_demo.rs

use std::fs;
use std::io::Read;

mod rust_to_emoji_workaround;
mod emoji_to_rust_standalone;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Rust to Emoji and Back Conversion Demo ---");

    let input_rust_file = "temp_test_conversion.rs";
    let output_rust_file = "output_rewritten_rust.rs";

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

    // 4. Write regenerated Rust code to a new file
    fs::write(output_rust_file, &regenerated_rust_code)?;
    println!("\nRegenerated Rust code written to: {}", output_rust_file);

    println!("\n--- Demo Complete ---");

    Ok(())
}

