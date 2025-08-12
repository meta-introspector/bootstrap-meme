use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use super::super::Token;

pub fn execute_lightning(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    // Expect next token to be a Float
    if let Some(next_token) = tokens_iter.next() {
        if let Token::Float(f) = next_token {
            stack.push(*f as i32); // Convert float to int for stack
            Ok(())
        } else {
            Err(format!("Expected Float after Lightning, got {next_token:?}"))
        }
    } else {
        Err("Expected Float after Lightning, but found end of tape.".to_string())
    }
}
