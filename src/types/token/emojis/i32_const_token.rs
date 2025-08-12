use crate::types::token::Token;

pub const EMOJI: &str = "📏";
pub const ASCII_EQUIVALENT: &str = "i32.const";

pub fn to_token() -> Token {
    Token::I32Const(0) // Placeholder value, will be replaced by actual value during parsing
}

