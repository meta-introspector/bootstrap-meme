use crate::types::token::Token;

pub const EMOJI: &str = ".apply";

pub fn to_token() -> Token {
    Token::ApplyRulesLoop
}
