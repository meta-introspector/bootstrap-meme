use crate::types::token::Token;

pub const EMOJI: &str = "🧱";
pub const ASCII_EQUIVALENT: &str = "emit_wat_block";

pub fn to_token() -> Token {
    Token::EmitWatBlock
}