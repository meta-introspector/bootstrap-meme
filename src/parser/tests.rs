use crate::types::token::Token;
use crate::tokenizer::tokenize_function::tokenize;
use crate::parser::emojitape_parser::parse_emojitape;

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