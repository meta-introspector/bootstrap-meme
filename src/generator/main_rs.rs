use std::fs;
use std::path::Path;

pub fn generate_main_rs(wat_block: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = Path::new("generated_project");
    let template_path = Path::new("src/generator/main.rs.template");

    let template_content = fs::read_to_string(template_path)?;

    let escaped_wat_block = wat_block.replace("\"", "\\\"");
    let main_rs_content = template_content.replace("{{WAT_BLOCK}}", &escaped_wat_block);

    fs::write(project_dir.join("src/main.rs"), main_rs_content)?;

    Ok(())
}
