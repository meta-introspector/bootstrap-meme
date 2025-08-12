use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub trait ExecutableToken {
    fn execute(
        &self,
        stack: &mut Vec<i32>,
        locals: &mut HashMap<i32, i32>,
        tokens_iter: &mut Peekable<Iter<super::Token>>,
    ) -> Result<(), String>;
}
