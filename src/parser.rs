// src/parser.rs

use crate::types::{Emojitape, Rule, Token};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref TOKEN_MAP: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        // Prelude
        m.insert("✅", Token::True);
        m.insert("❌", Token::False);
        m.insert("➡️", Token::FuncStart);
        m.insert("∀", Token::Forall);
        m.insert("∃", Token::Exists);
        m.insert("⏫⏫⏫", Token::UpArrow);
        m.insert("(∧)", Token::And);
        m.insert("(∨)", Token::Or);
        m.insert("(¬)", Token::Not);
        m.insert("(→)", Token::Implies);
        m.insert("(↔)", Token::Iff);
        m.insert("S", Token::S);
        m.insert("K", Token::K);
        m.insert("I", Token::I);
        m.insert("✨", Token::Sparkle);
        m.insert("⚡", Token::Lightning);
        m.insert("B", Token::B);
        m.insert("C", Token::C);
        m.insert("W", Token::W);
        m.insert("Y", Token::Y);
        m.insert("Z", Token::Z);
        m.insert("Ω", Token::Omega);
        m.insert("Λ", Token::Lambda);
        m.insert("⊤", Token::Top);
        m.insert("⊥", Token::Bottom);
        m.insert("↦", Token::MapsTo);
        m.insert("∘", Token::Compose);
        m.insert("=", Token::Equals);
        m.insert("≠", Token::NotEquals);
        m.insert("≡", Token::Identical);
        m.insert("⊢", Token::Proves);
        m.insert("⊨️", Token::Entails);

        // WASM Compiler Prelude (conceptual)
        m.insert("⚗️", Token::Compiler);
        m.insert("⚙️", Token::Optimizer);
        m.insert("📦", Token::Box); // Re-used for emit_wat_block

        // Rules (emoji -> token mapping)
        m.insert("↩️", Token::Return);
        m.insert("📞", Token::Call);
        m.insert("📥", Token::LocalGet);
        m.insert("📤", Token::LocalSet);
        m.insert("🌱", Token::SpawnToken);
        // m.insert("💬", Token::Comment); // Removed as Comment now holds a String
        m.insert("(... )", Token::RuleEntry); // This will need special handling
        m.insert("️.apply", Token::ApplyRulesLoop);

        // World Tape
        m.insert("➕", Token::Add);
        m.insert("➖", Token::Sub);
        m.insert("✖️", Token::Mul);
        m.insert("➗", Token::DivS);
        m.insert("≻", Token::GtS);
        m.insert("/zos export", Token::ZosExport);
        m.insert("/zos ready", Token::ZosReady);

        m
    };

    static ref TOKEN_REGEX: Regex = Regex::new(r"(\s+|\n|✅|❌|➡️|∀|∃|⏫⏫⏫|\(∧\)|\(∨\)|\(¬\)|\(→\)|\(↔\)|S|K|I|✨|⚡|B|C|W|Y|Z|Ω|Λ|⊤|⊥|↦|∘|=|≠|≡|⊢|⊨️|⚗️|⚙️|📦|↩️|📞|📥|📤|🌱|(?P<comment>💬[^\n]*)|\(\.\.\. \)|️\.apply|➕|➖|✖️|➗|≻|/zos export|/zos ready|\d+\.\d+|\d+)").unwrap();
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut last_index = 0;

    for caps in TOKEN_REGEX.captures_iter(input) {
        let full_match = caps.get(0).unwrap();
        let pre_match = &input[last_index..full_match.start()];
        if !pre_match.is_empty() {
            tokens.push(Token::Word(pre_match.to_string()));
        }

        let matched_str = full_match.as_str();

        if let Some(comment_match) = caps.name("comment") {
            // The comment includes the '💬' emoji, so we strip it for the content
            let comment_content = comment_match.as_str().strip_prefix("💬").unwrap_or("");
            tokens.push(Token::Comment(comment_content.to_string()));
        } else if matched_str == "\n" {
            tokens.push(Token::Newline);
        } else if matched_str.trim().is_empty() {
            tokens.push(Token::Whitespace);
        } else if let Some(token_type) = TOKEN_MAP.get(matched_str) {
            tokens.push(token_type.clone());
        } else if let Ok(i) = matched_str.parse::<i32>() {
            tokens.push(Token::Integer(i));
        } else if let Ok(f) = matched_str.parse::<f32>() {
            tokens.push(Token::Float(f));
        } else {
            tokens.push(Token::Other(matched_str.to_string()));
        }
        last_index = full_match.end();
    }

    let remaining = &input[last_index..];
    if !remaining.is_empty() {
        tokens.push(Token::Word(remaining.to_string()));
    }

    tokens
}



// This parser is highly simplified and will need significant expansion
// to handle the full complexity of the emojitape format, especially rules.
pub fn parse_emojitape(tokens: Vec<Token>) -> Emojitape {
    let mut emojitape = Emojitape {
        prelude: Vec::new(),
        wasm_compiler_prelude: Vec::new(),
        rules: Vec::new(),
        world_tape: Vec::new(),
        generated_wat_block: Vec::new(),
        clues_keys: Vec::new(),
        zos_export_definition: Vec::new(),
        zos_export_implementation: Vec::new(),
        self_reproducing_footer: Vec::new(),
    };

    let mut current_section = "";
    for token in tokens {
        match &token {
            Token::Comment(comment_text) => {
                if comment_text.starts_with("---") {
                    if comment_text.starts_with("--- PRELUDE") {
                        current_section = "prelude";
                    } else if comment_text.starts_with("--- WASM COMPILER PRELUDE") {
                        current_section = "wasm_compiler_prelude";
                    } else if comment_text.starts_with("--- RULES") {
                        current_section = "rules";
                    } else if comment_text.starts_with("--- WORLD TAPE") {
                        current_section = "world_tape";
                    } else if comment_text.starts_with("--- GENERATED WAT BLOCK") {
                        current_section = "generated_wat_block";
                    } else if comment_text.starts_with("--- CLUES & KEYS") {
                        current_section = "clues_keys";
                    } else if comment_text.starts_with("--- /zos export definition") {
                        current_section = "zos_export_definition";
                    } else if comment_text.starts_with("--- /zos export implementation") {
                        current_section = "zos_export_implementation";
                    } else if comment_text.starts_with("--- SELF-REPRODUCING FOOTER") {
                        current_section = "self_reproducing_footer";
                    }
                }
                // Comments within sections are ignored for now, or can be added to the respective Vecs
            },
            _ => {
                match current_section {
                    "prelude" => {
                        if !matches!(token, Token::Newline | Token::Whitespace) {
                            emojitape.prelude.push(token.clone())
                        }
                    },
                    "wasm_compiler_prelude" => {
                        if !matches!(token, Token::Newline | Token::Whitespace) {
                            emojitape.wasm_compiler_prelude.push(token.clone())
                        }
                    },
                    "rules" => {
                        // Simplified rule parsing: just push all tokens for now
                        // Real rule parsing would involve identifying RuleEntry tokens and their content
                        if !matches!(token, Token::Newline | Token::Whitespace) {
                            emojitape.rules.push(Rule {
                                name: None,
                                pattern: vec![token.clone()],
                                replacement: Vec::new(),
                            });
                        }
                    },
                    "world_tape" => {
                        if !matches!(token, Token::Newline | Token::Whitespace) {
                            emojitape.world_tape.push(token.clone())
                        }
                    },
                    "generated_wat_block" => {
                        if !matches!(token, Token::Newline | Token::Whitespace) {
                            emojitape.generated_wat_block.push(token.clone())
                        }
                    },
                    "clues_keys" => {
                        if !matches!(token, Token::Newline | Token::Whitespace) {
                            emojitape.clues_keys.push(token.clone())
                        }
                    },
                    "zos_export_definition" => {
                        if !matches!(token, Token::Newline | Token::Whitespace) {
                            emojitape.zos_export_definition.push(token.clone())
                        }
                    },
                    "zos_export_implementation" => {
                        if !matches!(token, Token::Newline | Token::Whitespace) {
                            emojitape.zos_export_implementation.push(token.clone())
                        }
                    },
                    "self_reproducing_footer" => {
                        if !matches!(token, Token::Newline | Token::Whitespace) {
                            emojitape.self_reproducing_footer.push(token.clone())
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    emojitape
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Token;

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
            Token::Comment(" This is a comment".to_string()),
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
        assert_eq!(emojitape.world_tape, vec![Token::Integer(1)]);
        assert!(emojitape.rules.is_empty());
    }

    #[test]
    fn test_parse_emojitape_rules() {
        let tokens = vec![
            Token::Comment("--- RULES".to_string()),
            Token::Newline,
            Token::FuncStart, // ➡️
            Token::Return, // ↩️
            Token::Comment("--- WORLD TAPE".to_string()),
        ];
        let emojitape = parse_emojitape(tokens);
        assert_eq!(emojitape.rules.len(), 2);
        assert_eq!(emojitape.rules[0].pattern, vec![Token::FuncStart]);
        assert_eq!(emojitape.rules[1].pattern, vec![Token::Return]);
        assert!(emojitape.prelude.is_empty());
        assert!(emojitape.world_tape.is_empty());
    }

    #[test]
    fn test_parse_emojitape_all_sections() {
        let tokens = vec![
            Token::Comment("--- PRELUDE".to_string()),
            Token::Newline,
            Token::True,
            Token::Comment("--- WASM COMPILER PRELUDE".to_string()),
            Token::Newline,
            Token::Compiler,
            Token::Comment("--- RULES".to_string()),
            Token::Newline,
            Token::FuncStart,
            Token::Comment("--- WORLD TAPE".to_string()),
            Token::Newline,
            Token::Integer(1),
            Token::Comment("--- GENERATED WAT BLOCK".to_string()),
            Token::Newline,
            Token::Box,
            Token::Comment("--- CLUES & KEYS".to_string()),
            Token::Newline,
            Token::Sparkle,
            Token::Comment("--- /zos export definition".to_string()),
            Token::Newline,
            Token::ZosExport,
            Token::Comment("--- /zos export implementation".to_string()),
            Token::Newline,
            Token::ZosReady,
            Token::Comment("--- SELF-REPRODUCING FOOTER".to_string()),
            Token::Newline,
            Token::Omega,
        ];

        let emojitape = parse_emojitape(tokens);

        assert_eq!(emojitape.prelude, vec![Token::True]);
        assert_eq!(emojitape.wasm_compiler_prelude, vec![Token::Compiler]);
        assert_eq!(emojitape.rules.len(), 1);
        assert_eq!(emojitape.rules[0].pattern, vec![Token::FuncStart]);
        assert_eq!(emojitape.world_tape, vec![Token::Integer(1)]);
        assert_eq!(emojitape.generated_wat_block, vec![Token::Box]);
        assert_eq!(emojitape.clues_keys, vec![Token::Sparkle]);
        assert_eq!(emojitape.zos_export_definition, vec![Token::ZosExport]);
        assert_eq!(emojitape.zos_export_implementation, vec![Token::ZosReady]);
        assert_eq!(emojitape.self_reproducing_footer, vec![Token::Omega]);
    }
}