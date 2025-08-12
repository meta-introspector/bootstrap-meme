use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "📦";

pub fn to_token() -> Token {
    Token::Box
}

pub fn execute_box(
    _stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    _tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    println!("Box (📦) token encountered. This signifies a WAT block or WASM binary emission.");
    Ok(())
}