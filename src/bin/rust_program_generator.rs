//use emojitape_interpreter::parser;
use emojitape_interpreter::parser::emojitape_parser::parse_emojitape;
use emojitape_interpreter::generator::{rust_project, main_rs};
use std::fs;
use std::io::Read;
use emojitape_interpreter::tokenizer::tokenize;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_emojitape_file> <output_project_directory>", args[0]);
        return Err("Missing arguments: input_emojitape_file and output_project_directory are required.".into());
    }

    let emojitape_path = &args[1];
    let output_dir = &args[2];

    println!("Reading Emojitape from: {emojitape_path}");
    let mut file = fs::File::open(emojitape_path)?;
    let mut emojitape_content = String::new();
    file.read_to_string(&mut emojitape_content)?;

    println!("Parsing Emojitape...");
    let tokens = tokenize(&emojitape_content);
    let emojitape = parse_emojitape(tokens);

    println!("Extracting generated WAT block...");
    let wat_block = emojitape
        .generated_wat_block
        .iter()
        .map(|token| token.to_string())
        .collect::<String>();

    println!("Generating Rust project in {output_dir}...");
    rust_project::create_project_structure(output_dir)?;
    main_rs::generate_main_rs(output_dir, &wat_block)?;

    println!("Successfully generated Rust project in `{output_dir}` directory!");

    Ok(())
}
