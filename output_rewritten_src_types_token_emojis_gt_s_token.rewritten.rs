// RUST_CODE_BLOCK_UNPARSABLE:use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "≻";

pub fn to_token() -> Token {
    Token::GtS
}

pub fn execute_gt_s(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(if a > b { 1 } else { 0 }); // Push 1 for true, 0 for false
        Ok(())
    } else {
        Err("Not enough operands for GtS operation.".to_string())
    }
}