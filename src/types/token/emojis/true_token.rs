use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "✅";
pub const ASCII_EQUIVALENT: &str = "true";

pub fn to_token() -> Token {
    Token::True
}

pub fn execute_true(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    stack.push(1);
    Ok(())
}