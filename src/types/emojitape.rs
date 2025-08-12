use crate::types::token::Token;

#[derive(Debug, Default)]
pub struct Emojitape {
    pub prelude: Vec<Token>,
    pub wasm_compiler_prelude: Vec<Token>,
    pub rules: Vec<Token>,
    pub world_tape: Vec<Token>,
    pub generated_wat_block: Vec<Token>,
    pub clues_keys: Vec<Token>,
    pub zos_export_definition: Vec<Token>,
    pub zos_export_implementation: Vec<Token>,
    pub self_reproducing_footer: Vec<Token>,
}

impl Emojitape {
    pub fn render(&self) -> String {
        let mut output = String::new();
        output.push_str("// PRELUDE\n");
        for token in &self.prelude {
            output.push_str(&token.to_string());
        }
        output.push_str("\n// WASM COMPILER PRELUDE\n");
        for token in &self.wasm_compiler_prelude {
            output.push_str(&token.to_string());
        }
        output.push_str("\n// RULES\n");
        for token in &self.rules {
            output.push_str(&token.to_string());
        }
        output.push_str("\n// WORLD TAPE\n");
        for token in &self.world_tape {
            output.push_str(&token.to_string());
        }
        output.push_str("\n// GENERATED WAT BLOCK\n");
        for token in &self.generated_wat_block {
            output.push_str(&token.to_string());
        }
        output.push_str("\n// CLUES & KEYS\n");
        for token in &self.clues_keys {
            output.push_str(&token.to_string());
        }
        output.push_str("\n// ZOS EXPORT DEFINITION\n");
        for token in &self.zos_export_definition {
            output.push_str(&token.to_string());
        }
        output.push_str("\n// ZOS EXPORT IMPLEMENTATION\n");
        for token in &self.zos_export_implementation {
            output.push_str(&token.to_string());
        }
        output.push_str("\n// SELF-REPRODUCING FOOTER\n");
        for token in &self.self_reproducing_footer {
            output.push_str(&token.to_string());
        }
        output
    }
}
