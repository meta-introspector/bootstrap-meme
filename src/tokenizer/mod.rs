use crate::types::token::Token;
use std::collections::HashMap;
use lazy_static::lazy_static; // Import lazy_static

pub mod build_token_map_function; // Import the new module
pub mod tokenize_function; // Import the new module

lazy_static! {
    static ref TOKEN_MAP: HashMap<&'static str, Token> = build_token_map_function::build_token_map();
}






#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_tokenize_simple() {
        let input = "🌱✅❌";
        let expected = vec![
            Token::SpawnToken,
            Token::True,
            Token::False,
        ];
        assert_eq!(tokenize_function::tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_with_numbers() {
        let input = "📏42 📐3.14";
        let expected = vec![
            Token::I32Const(42),
            Token::Whitespace,
            Token::F32Const(3.14),
        ];
        assert_eq!(tokenize_function::tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_with_words() {
        let input = "hello world";
        let expected = vec![
            Token::Word("hello".to_string()),
            Token::Whitespace,
            Token::Word("world".to_string()),
        ];
        assert_eq!(tokenize_function::tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_with_comment() {
        let input = "💬 this is a comment
🌱";
        let expected = vec![
            Token::Comment("this is a comment".to_string()),
            Token::Newline,
            Token::SpawnToken,
        ];
        assert_eq!(tokenize_function::tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_mixed() {
        let input = "🌱 📏42 ➕ 📏58 🎯";
        let expected = vec![
            Token::SpawnToken,
            Token::Whitespace,
            Token::I32Const(42),
            Token::Whitespace,
            Token::Add,
            Token::Whitespace,
            Token::I32Const(58),
            Token::Whitespace,
            Token::Return,
        ];
        assert_eq!(tokenize_function::tokenize(input), expected);
    }
}
