// RUST_CODE_BLOCK_UNPARSABLE:use super::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Rule {
    pub name: Option<String>,
    pub pattern: Vec<Token>,
    pub replacement: Vec<Token>,
}