// RUST_CODE_BLOCK_UNPARSABLE:use crate::types::token::Token;
use crate::tokenizer::tokenize;
use crate::parser::parse_emojitape;

#[test]
fn test_tokenize_empty() {
    let tokens = tokenize("");
    assert!(tokens.is_empty());
}

#[test]
fn test_tokenize_simple_emojis() {
    let tokens = tokenize("✅❌➡️");
    assert_eq!(tokens, vec![Token::True, Token::False, Token::FuncStart]);
}

#[test]
fn test_tokenize_with_words_and_numbers() {
    let tokens = tokenize("hello 123 world 4.5");
    assert_eq!(tokens, vec![
        Token::Word("hello".to_string()),
        Token::Whitespace,
        Token::Integer(123),
        Token::Whitespace,
        Token::Word("world".to_string()),
        Token::Whitespace,
        Token::Float(4.5),
    ]);
}

#[test]
fn test_tokenize_with_comments() {
    let tokens = tokenize("💬 This is a comment\n✅");
    assert_eq!(tokens, vec![
        Token::Comment("This is a comment".to_string()), // Removed leading space
        Token::Newline,
        Token::True,
    ]);
}

#[test]
fn test_tokenize_with_section_comments() {
    let tokens = tokenize("💬--- PRELUDE\n✅");
    assert_eq!(tokens, vec![
        Token::Comment("--- PRELUDE".to_string()),
        Token::Newline,
        Token::True,
    ]);
}

#[test]
fn test_tokenize_with_other_token() {
    let tokens = tokenize("hello$world");
    assert_eq!(tokens, vec![
        Token::Word("hello".to_string()),
        Token::Other("$".to_string()),
        Token::Word("world".to_string()),
    ]);
}

#[test]
fn test_parse_emojitape_prelude() {
    let tokens = vec![
        Token::Comment("--- PRELUDE".to_string()),
        Token::Newline,
        Token::True,
        Token::False,
        Token::Comment("--- WORLD TAPE".to_string()),
        Token::Newline,
        Token::Integer(1),
    ];
    let emojitape = parse_emojitape(tokens);
    assert_eq!(emojitape.prelude, vec![Token::True, Token::False]);
    assert_eq!(emojitape.world_tape, vec![Token::Newline, Token::Integer(1)]);
    assert!(emojitape.rules.is_empty());
}

#[test]
fn test_parse_emojitape_rules() {
    let tokens = vec![
        Token::Comment("--- RULES".to_string()),
        Token::Newline,
        Token::RuleEntry, // ➡️
        Token::Integer(1),
        Token::Integer(1),
        Token::Add,
                    Token::FuncStart, // ➡️
        Token::Integer(2),
        Token::Comment("--- WORLD TAPE".to_string()),
    ];
    let emojitape = parse_emojitape(tokens);
    assert_eq!(emojitape.rules.len(), 1);
    assert_eq!(emojitape.rules[0].pattern, vec![Token::Integer(1), Token::Integer(1), Token::Add]);
    assert_eq!(emojitape.rules[0].replacement, vec![Token::Integer(2)]);
    assert!(emojitape.prelude.is_empty());
    assert!(emojitape.world_tape.is_empty());
}

#[test]
fn test_parse_emojitape_all_sections() { // Renaming this test to avoid confusion later
    let tokens = vec![
        Token::Comment("--- PRELUDE".to_string()),
        Token::Newline,
        Token::True,
        Token::Comment("--- WASM COMPILER PRELUDE".to_string()),
        Token::Newline,
        Token::Compiler,
    ];
    let emojitape = parse_emojitape(tokens);
    assert_eq!(emojitape.prelude, vec![Token::True]);
    assert_eq!(emojitape.wasm_compiler_prelude, vec![Token::Compiler]);
}

#[test]
fn test_parse_prelude_and_wasm_compiler_prelude() {
    let tokens = vec![
        Token::Comment("--- PRELUDE".to_string()),
        Token::Newline,
        Token::True,
        Token::Comment("--- WASM COMPILER PRELUDE".to_string()),
        Token::Newline,
        Token::Compiler,
    ];
    let emojitape = parse_emojitape(tokens);
    assert_eq!(emojitape.prelude, vec![Token::True]);
    assert_eq!(emojitape.wasm_compiler_prelude, vec![Token::Compiler]);
}