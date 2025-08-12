use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "(¬)";

pub fn to_token() -> Token {
    Token::Not
}

pub fn execute_not(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if !stack.is_empty() {
        let a = stack.pop().unwrap();
        stack.push(if a == 0 { 1 } else { 0 });
        Ok(())
    } else {
        Err("Not enough operands for Not operation.".to_string())
    }
}