use emojitape_interpreter::parser;
//use emojitape_interpreter::types::token::Token;
use std::fs;
use std::io::Read;
use std::path::Path;

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

    let project_dir = Path::new("generated_project");
    if project_dir.exists() {
        fs::remove_dir_all(project_dir)?;
    }
    fs::create_dir(project_dir)?;
    fs::create_dir(project_dir.join("src"))?;

    let cargo_toml_content = r#"\n[package]\nname = \"generated_project\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nwat = \"1.0\"\n"#;
    fs::write(project_dir.join("Cargo.toml"), cargo_toml_content)?;

    let escaped_wat_block = wat_block.replace("\"", "\\\"");
    let main_rs_content = format!(
        "fn main() -> Result<(), Box<dyn std::error::Error>> {{    let wat_str = r#\"{{}}\"#;
    println!(\"--- Parsing WAT ---\");
    let wasm_binary = wat::parse_str(wat_str)?;
    println!(\"--- WAT Parsed Successfully ---\");
    let output_path = \"module.wasm\";
    std::fs::write(output_path, wasm_binary)?;
    println!(\"--- WASM binary written to {{}} ---\", output_path);
    Ok(())
}}",
        escaped_wat_block
    );
    fs::write(project_dir.join("src/main.rs"), main_rs_content)?;

    println!("Successfully generated Rust project in `generated_project` directory!");

    Ok(())
}
