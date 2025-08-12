use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "#";

pub fn to_token() -> Token {
    Token::Comment(String::new())
}

pub fn execute_comment(
    _s: &String,
    _stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    // fix me, do something for comments
    Ok(())
}
