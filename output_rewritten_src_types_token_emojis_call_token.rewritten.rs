// RUST_CODE_BLOCK_UNPARSABLE:use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "📞";

pub fn to_token() -> Token {
    Token::Call
}

pub fn execute_call(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if !stack.is_empty() {
        let func_id = stack.pop().unwrap();
        match func_id {
            0 => { // Example: print top of stack
                if let Some(value) = stack.pop() {
                    println!("Call (func_id 0): {value}");
                    Ok(())
                } else {
                    Err("Stack empty for print function.".to_string())
                }
            },
            _ => Err(format!("Unknown function ID: {func_id}")),
        }
    } else {
        Err("Not enough operands for Call operation.".to_string())
    }
}