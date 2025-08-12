// RUST_CODE_BLOCK_UNPARSABLE:use super::Emojitape;
use crate::types::token::Token;

impl Emojitape {
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Prelude
        if !self.prelude.is_empty() {
            output.push_str("💬--- PRELUDE\n");
            for token in &self.prelude {
                output.push_str(&token.to_string());
            }
            output.push('\n');
        }

        // WASM Compiler Prelude
        if !self.wasm_compiler_prelude.is_empty() {
            output.push_str("💬--- WASM COMPILER PRELUDE\n");
            for token in &self.wasm_compiler_prelude {
                output.push_str(&token.to_string());
            }
            output.push('\n');
        }

        // Rules
        if !self.rules.is_empty() {
            output.push_str("💬--- RULES\n");
            for rule in &self.rules {
                // Simplified rendering for rules for now
                for token in &rule.pattern {
                    output.push_str(&token.to_string());
                }
                output.push('\n');
            }
        }

        // World Tape
        if !self.world_tape.is_empty() {
            output.push_str("💬--- WORLD TAPE\n");
            for token in &self.world_tape {
                output.push_str(&token.to_string());
                // Add space between tokens for readability, except for Newline/Whitespace
                if !matches!(token, Token::Newline | Token::Whitespace) {
                    output.push(' ');
                }
            }
            output.push('\n');
        }

        // Generated WAT Block
        if !self.generated_wat_block.is_empty() {
            output.push_str("💬--- GENERATED WAT BLOCK\n");
            for token in &self.generated_wat_block {
                output.push_str(&token.to_string());
            }
            output.push('\n');
        }

        // Clues & Keys
        if !self.clues_keys.is_empty() {
            output.push_str("💬--- CLUES & KEYS\n");
            for token in &self.clues_keys {
                output.push_str(&token.to_string());
            }
            output.push('\n');
        }

        // ZOS Export Definition
        if !self.zos_export_definition.is_empty() {
            output.push_str("💬--- /zos export definition\n");
            for token in &self.zos_export_definition {
                output.push_str(&token.to_string());
            }
            output.push('\n');
        }

        // ZOS Export Implementation
        if !self.zos_export_implementation.is_empty() {
            output.push_str("💬--- /zos export implementation\n");
            for token in &self.zos_export_implementation {
                output.push_str(&token.to_string());
            }
            output.push('\n');
        }

        // Self-Reproducing Footer
        if !self.self_reproducing_footer.is_empty() {
            output.push_str("💬--- SELF-REPRODUCING FOOTER\n");
            for token in &self.self_reproducing_footer {
                output.push_str(&token.to_string());
            }
            output.push('\n');
        }

        output
    }
}
