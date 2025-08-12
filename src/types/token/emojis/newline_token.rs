use crate::types::token::Token;

pub const EMOJI: &str = "⏎";
pub const ASCII_EQUIVALENT: &str = "\n"; // New: ASCII representation

pub fn to_token() -> Token {
    Token::Newline
}