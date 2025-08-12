#[test]
fn test_tokenize_simple() {
    let input = "🌱✅❌";
    let expected = vec![
        crate::types::token::Token::SpawnToken,
        crate::types::token::Token::True,
        crate::types::token::Token::False,
    ];
    assert_eq!(crate::tokenizer::tokenize_function::tokenize(input), expected);
}

#[test]
fn test_tokenize_with_numbers() {
    let input = "📏42 📐3.14";
    let expected = vec![
        crate::types::token::Token::I32Const(42),
        crate::types::token::Token::Whitespace,
        crate::types::token::Token::F32Const(3.14),
    ];
    assert_eq!(crate::tokenizer::tokenize_function::tokenize(input), expected);
}

#[test]
fn test_tokenize_with_words() {
    let input = "hello world";
    let expected = vec![
        crate::types::token::Token::Word("hello".to_string()),
        crate::types::token::Token::Whitespace,
        crate::types::token::Token::Word("world".to_string()),
    ];
    assert_eq!(crate::tokenizer::tokenize_function::tokenize(input), expected);
}

#[test]
fn test_tokenize_with_comment() {
    let input = "💬 this is a comment\n🌱";
    let expected = vec![
        crate::types::token::Token::Comment("this is a comment".to_string()),
        crate::types::token::Token::Newline,
        crate::types::token::Token::SpawnToken,
    ];
    assert_eq!(crate::tokenizer::tokenize_function::tokenize(input), expected);
}

#[test]
fn test_tokenize_mixed() {
    let input = "🌱 📏42 ➕ 📏58 🎯";
    let expected = vec![
        crate::types::token::Token::SpawnToken,
        crate::types::token::Token::Whitespace,
        crate::types::token::Token::I32Const(42),
        crate::types::token::Token::Whitespace,
        crate::types::token::Token::Add,
        crate::types::token::Token::Whitespace,
        crate::types::token::Token::I32Const(58),
        crate::types::token::Token::Whitespace,
        crate::types::token::Token::Return,
    ];
    assert_eq!(crate::tokenizer::tokenize_function::tokenize(input), expected);
}