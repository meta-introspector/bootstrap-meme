use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "(↔)";

pub fn to_token() -> Token {
    Token::Iff
}

pub fn execute_iff(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(if (a != 0 && b != 0) || (a == 0 && b == 0) { 1 } else { 0 }); // (a AND b) OR (NOT a AND NOT b)
        Ok(())
    } else {
        Err("Not enough operands for Iff operation.".to_string())
    }
}