use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use super::super::Token;

pub fn execute_local_get(
    stack: &mut Vec<i32>,
    locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    if !stack.is_empty() {
        let index = stack.pop().unwrap();
        if let Some(&value) = locals.get(&index) {
            stack.push(value);
            Ok(())
        } else {
            Err(format!("Local variable at index {index} not found."))
        }
    } else {
        Err("Not enough operands for LocalGet operation.".to_string())
    }
}
