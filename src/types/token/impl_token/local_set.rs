use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use super::super::Token;

pub fn execute_local_set(
    stack: &mut Vec<i32>,
    locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if stack.len() >= 2 {
        let value = stack.pop().unwrap(); // Pop value first
        let index = stack.pop().unwrap(); // Then pop index
        locals.insert(index, value);
        Ok(())
    } else {
        Err("Not enough operands for LocalSet operation.".to_string())
    }
}
