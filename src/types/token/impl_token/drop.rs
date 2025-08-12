use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use super::super::Token;

pub fn execute_drop(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if !stack.is_empty() {
        stack.pop();
        Ok(())
    } else {
        Err("Not enough operands for Drop operation.".to_string())
    }
}
