use crate::types::token::Token;

pub const EMOJI: &str = "✅";
pub const ASCII_EQUIVALENT: &str = "true";

pub fn to_token() -> Token {
    Token::True
}
