use crate::types::token::Token;
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub const EMOJI: &str = "🧱";
pub const ASCII_EQUIVALENT: &str = "emit_wat_block";

pub fn to_token() -> Token {
    Token::EmitWatBlock
}

pub fn execute_emit_wat_block(
    _stack: &mut Vec<i32>,
    _locals: &mut HashMap<i32, i32>,
    tokens_iter: &mut Peekable<Iter<Token>>,
) -> Result<(), String> {
    let mut wat_content = String::new();
    let mut paren_count = 0;
    // Consume tokens until matching closing parenthesis
    while let Some(next_token) = tokens_iter.next() {
        if let Token::Other(s) = next_token {
            if s == "(" {
                paren_count += 1;
            } else if s == ")" {
                if paren_count == 0 {
                    break; // Found matching closing parenthesis
                } else {
                    paren_count -= 1;
                }
            }
            wat_content.push_str(s);
        } else {
            wat_content.push_str(&next_token.to_string());
        }
    }
    println!("Emitting WAT Block: {}", wat_content);
    Ok(())
}
