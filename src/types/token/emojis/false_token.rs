use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "❌";
pub const ASCII_EQUIVALENT: &str = "false";

pub fn to_token() -> Token {
    Token::False
}

pub fn execute_false(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    stack.push(0);
    Ok(())
}