pub mod impl_emojitape;
//pub use impl_emojitape::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Emojitape {
    pub prelude: Vec<super::token::Token>,
    pub wasm_compiler_prelude: Vec<super::token::Token>,
    pub rules: Vec<super::rule::Rule>,
    pub world_tape: Vec<super::token::Token>,
    pub generated_wat_block: Vec<super::token::Token>,
    pub clues_keys: Vec<super::token::Token>,
    pub zos_export_definition: Vec<super::token::Token>,
    pub zos_export_implementation: Vec<super::token::Token>,
    pub self_reproducing_footer: Vec<super::token::Token>,
}
