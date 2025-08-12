use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "⏎";
pub const ASCII_EQUIVALENT: &str = "\n"; // New: ASCII representation

pub fn to_token() -> Token {
    Token::Newline
}

pub fn execute_newline(
    _stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    // Do nothing for newline
    Ok(())
}

