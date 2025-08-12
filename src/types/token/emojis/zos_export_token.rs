use crate::types::token::Token;

pub const EMOJI: &str = "/zos export";

pub fn to_token() -> Token {
    Token::ZosExport
}
