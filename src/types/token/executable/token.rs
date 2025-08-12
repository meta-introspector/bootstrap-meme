use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
//use super::Token; // Import Token from parent module
use crate::types::token::Token;
pub trait ExecutableToken {
    fn execute(
        &self,
        stack: &mut Vec<i32>,
        locals: &mut HashMap<i32, i32>,
        tokens_iter: &mut Peekable<Iter<Token>>,
    ) -> Result<(), String>;
}
