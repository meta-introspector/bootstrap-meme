use emojitape_interpreter::parser;
use emojitape_interpreter::generator::{rust_project, main_rs};
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

    println!("Generating Rust project...");
    rust_project::create_project_structure()?;
    main_rs::generate_main_rs(&wat_block)?;

    println!("Successfully generated Rust project in `generated_project` directory!");

    Ok(())
}