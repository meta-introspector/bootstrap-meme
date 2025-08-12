use emojitape_interpreter::parser;
use emojitape_interpreter::types::token::Token;
use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let emojitape_path = "full_generated_emojitape.emojitape";

    println!("Reading Emojitape from: {}", emojitape_path);
    let mut file = fs::File::open(emojitape_path)?;
    let mut emojitape_content = String::new();
    file.read_to_string(&mut emojitape_content)?;

    println!("Parsing Emojitape...");
    let tokens = parser::tokenize(&emojitape_content);
    let emojitape = parser::parse_emojitape(tokens);

    println!("Extracting generated WAT block...");
    let wat_block = emojitape
        .generated_wat_block
        .iter()
        .map(|token| token.to_string_representation())
        .collect::<String>();

    println!("Generating Rust program...");
    let rust_program = format!(
        r#"\nfn main() {{\n    println!(\"--- Generated WAT Block ---\");\n    println!(\"{}\");\n    println!(\"--- End Generated WAT Block ---\");\n}}\n"#,
        wat_block.replace("\"", "\\\"")
    );

    let output_path = "generated_rust_program.rs";
    fs::write(output_path, rust_program)?;

    println!("Successfully generated {}!", output_path);

    Ok(())
}
