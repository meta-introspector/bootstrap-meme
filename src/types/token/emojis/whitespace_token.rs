use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = " ";

pub fn to_token() -> Token {
    Token::Whitespace
}

pub fn execute_whitespace(
    _stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    // Do nothing for whitespace
    Ok(())
}
