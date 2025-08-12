use std::fs;
use std::path::Path;

pub fn create_project_structure() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = Path::new("generated_project");
    if project_dir.exists() {
        fs::remove_dir_all(project_dir)?;
    }
    fs::create_dir(project_dir)?;
    fs::create_dir(project_dir.join("src"))?;

    let cargo_toml_content = r#"
[package]
name = "generated_project"
version = "0.1.0"
edition = "2021"

[dependencies]
wat = "1.0"
"#;
    fs::write(project_dir.join("Cargo.toml"), cargo_toml_content)?;

    Ok(())
}
