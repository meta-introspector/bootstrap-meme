// RUST_CODE_BLOCK_UNPARSABLE:use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "S";

pub fn to_token() -> Token {
    Token::S
}

pub fn execute_s(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    // S f g x = f x (g x)
    // Requires at least 3 operands on the stack: x, g, f
    if stack.len() >= 3 {
        let x = stack.pop().unwrap();
        let g = stack.pop().unwrap();
        let f = stack.pop().unwrap();
        // This is a simplified interpretation. In a real combinator system,
        // f, g, and x would be functions or values. Here, we're just
        // manipulating their integer representations on the stack.
        // A more complex implementation would involve a way to represent and apply functions.
        // For now, let's just push them back in the order of application.
        stack.push(f);
        stack.push(x);
        stack.push(g);
        stack.push(x);
        Ok(())
    } else {
        Err("Not enough operands for S combinator.".to_string())
    }
}