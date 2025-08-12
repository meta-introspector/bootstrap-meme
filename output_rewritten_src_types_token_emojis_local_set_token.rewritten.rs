// RUST_CODE_BLOCK_UNPARSABLE:use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "📤";

pub fn to_token() -> Token {
    Token::LocalSet
}

pub fn execute_local_set(
    stack: &mut Vec<i32>,
    locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if stack.len() >= 2 {
        let index = stack.pop().unwrap();
        let value = stack.pop().unwrap();
        locals.insert(index, value);
        Ok(())
    } else {
        Err("Not enough operands for LocalSet operation.".to_string())
    }
}