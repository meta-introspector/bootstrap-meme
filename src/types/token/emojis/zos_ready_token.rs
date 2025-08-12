use crate::types::token::Token;

pub const EMOJI: &str = "/zos ready";

pub fn to_token() -> Token {
    Token::ZosReady
}
