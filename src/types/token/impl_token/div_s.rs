use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use super::super::Token;

pub fn execute_div_s(
    stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        if b == 0 {
            Err("Division by zero.".to_string())
        } else {
            stack.push(a / b);
            Ok(())
        }
    } else {
        Err("Not enough operands for DivS operation.".to_string())
    }
}
