use crate::types::token::Token;
use std::collections::HashMap;
use lazy_static::lazy_static; // Import lazy_static

pub mod build_token_map_function; // Import the new module
pub mod tokenize_function; // Import the new module
pub mod tests; // Import the new module

lazy_static! {
    static ref TOKEN_MAP: HashMap<&'static str, Token> = build_token_map_function::build_token_map();
}







