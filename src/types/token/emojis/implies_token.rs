use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "(→)";

pub fn to_token() -> Token {
    Token::Implies
}

pub fn execute_implies(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(if a == 0 || b != 0 { 1 } else { 0 }); // NOT a OR b
        Ok(())
    } else {
        Err("Not enough operands for Implies operation.".to_string())
    }
}