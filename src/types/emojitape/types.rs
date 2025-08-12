//pub mod impl_emojitape;
//pub use impl_emojitape::*;


#[derive(Debug, PartialEq, Clone)]
pub struct Emojitape {
    pub prelude: Vec<crate::types::token::Token>,
    pub wasm_compiler_prelude: Vec<crate::types::token::Token>,
    pub rules: Vec<crate::types::rule::Rule>,
    pub world_tape: Vec<crate::types::token::Token>,
    pub generated_wat_block: Vec<crate::types::token::Token>,
    pub clues_keys: Vec<crate::types::token::Token>,
    pub zos_export_definition: Vec<crate::types::token::Token>,
    pub zos_export_implementation: Vec<crate::types::token::Token>,
    pub self_reproducing_footer: Vec<crate::types::token::Token>,
    pub expected_output: Option<String>,
}