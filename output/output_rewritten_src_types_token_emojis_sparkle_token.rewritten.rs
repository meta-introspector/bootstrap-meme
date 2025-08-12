// RUST_CODE_BLOCK_UNPARSABLE:use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "✨";

pub fn to_token() -> Token {
    Token::Sparkle
}

pub fn execute_sparkle(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    // Expect next token to be an Integer
    if let Some(next_token) = tokens_iter.next() {
        if let Token::Integer(i) = next_token {
            stack.push(*i);
            Ok(())
        } else {
            Err(format!("Expected Integer after Sparkle, got {next_token:?}"))
        }
    } else {
        Err("Expected Integer after Sparkle, but found end of tape.".to_string())
    }
}