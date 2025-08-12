use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "📏";
pub const ASCII_EQUIVALENT: &str = "i32.const";

pub fn to_token() -> Token {
    Token::I32Const(0) // Placeholder value, will be replaced by actual value during parsing
}

pub fn execute_i32_const(
    i: &i32,
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    stack.push(*i);
    Ok(())
}