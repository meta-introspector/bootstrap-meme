use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use crate::types::token::Token;
use crate::types::token::executable::ExecutableToken;

impl ExecutableToken for Token {
    fn execute(
        &self,
        stack: &mut Vec<i32>,
        _locals: &mut HashMap<i32, i32>,
        _tokens_iter: &mut Peekable<Iter<Token>>,
    ) -> Result<(), String> {
        match self {
            Token::Add => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b);
                    Ok(())
                } else {
                    Err("Not enough operands for Add operation.".to_string())
                }
            },
            _ => Err(format!("Attempted to execute non-Add token as Add: {:?}", self)),
        }
    }
}
