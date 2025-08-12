use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use super::super::Token;

pub fn execute_i(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    // I x = x
    // Requires at least 1 operand on the stack: x
    if !stack.is_empty() {
        let x = stack.pop().unwrap();
        stack.push(x);
        Ok(())
    } else {
        Err("Not enough operands for I combinator.".to_string())
    }
}
