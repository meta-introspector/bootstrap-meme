// RUST_CODE_BLOCK_UNPARSABLE:use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "K";

pub fn to_token() -> Token {
    Token::K
}

pub fn execute_k(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    // K x y = x
    // Requires at least 2 operands on the stack: y, x
    if stack.len() >= 2 {
        let _y = stack.pop().unwrap(); // Consume y
        let x = stack.pop().unwrap();
        stack.push(x);
        Ok(())
    } else {
        Err("Not enough operands for K combinator.".to_string())
    }
}