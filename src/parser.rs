// src/parser.rs

use super::types::{Emojitape, Rule, Token};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref TOKEN_MAP: HashMap<&'static str, Token<'static>> = {
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
        m.insert("💬", Token::Comment);
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

    static ref TOKEN_REGEX: Regex = Regex::new(r"(\s+|\n|✅|❌|➡️|∀|∃|⏫⏫⏫|\(∧\)|\(∨\)|\(¬\)|\(→\)|\(↔\)|S|K|I|✨|⚡|B|C|W|Y|Z|Ω|Λ|⊤|⊥|↦|∘|=|≠|≡|⊢|⊨️|⚗️|⚙️|📦|↩️|📞|📥|📤|🌱|💬|\(\.\.\. \)|️\.apply|➕|➖|✖️|➗|≻|/zos export|/zos ready)").unwrap();
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut last_index = 0;

    for mat in TOKEN_REGEX.find_iter(input) {
        let pre_match = &input[last_index..mat.start()];
        if !pre_match.is_empty() {
            tokens.push(Token::Word(pre_match));
        }

        let matched_str = mat.as_str();
        if matched_str == "\n" {
            tokens.push(Token::Newline);
        } else if matched_str.trim().is_empty() {
            tokens.push(Token::Whitespace);
        } else if let Some(token_type) = TOKEN_MAP.get(matched_str) {
            tokens.push(token_type.clone());
        } else {
            // Handle numbers
            if let Ok(i) = matched_str.parse::<i32>() {
                tokens.push(Token::Integer(i));
            } else if let Ok(f) = matched_str.parse::<f32>() {
                tokens.push(Token::Float(f));
            } else {
                tokens.push(Token::Other(matched_str));
            }
        }
        last_index = mat.end();
    }

    let remaining = &input[last_index..];
    if !remaining.is_empty() {
        tokens.push(Token::Word(remaining));
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
            Token::Comment(c) => {
                if c.starts_with("---") {
                    if c.starts_with("--- PRELUDE") {
                        current_section = "prelude";
                    } else if c.starts_with("--- WASM COMPILER PRELUDE") {
                        current_section = "wasm_compiler_prelude";
                    } else if c.starts_with("--- RULES") {
                        current_section = "rules";
                    } else if c.starts_with("--- WORLD TAPE") {
                        current_section = "world_tape";
                    } else if c.starts_with("--- GENERATED WAT BLOCK") {
                        current_section = "generated_wat_block";
                    } else if c.starts_with("--- CLUES & KEYS") {
                        current_section = "clues_keys";
                    } else if c.starts_with("--- /zos export definition") {
                        current_section = "zos_export_definition";
                    } else if c.starts_with("--- /zos export implementation") {
                        current_section = "zos_export_implementation";
                    } else if c.starts_with("--- SELF-REPRODUCING FOOTER") {
                        current_section = "self_reproducing_footer";
                    }
                }
                // Comments within sections are ignored for now, or can be added to the respective Vecs
            },
            _ => {
                match current_section {
                    "prelude" => emojitape.prelude.push(token),
                    "wasm_compiler_prelude" => emojitape.wasm_compiler_prelude.push(token),
                    "rules" => {
                        // Simplified rule parsing: just push all tokens for now
                        // Real rule parsing would involve identifying RuleEntry tokens and their content
                        emojitape.rules.push(Rule {
                            name: None,
                            pattern: vec![token.clone()],
                            replacement: Vec::new(),
                        });
                    },
                    "world_tape" => emojitape.world_tape.push(token),
                    "generated_wat_block" => emojitape.generated_wat_block.push(token),
                    "clues_keys" => emojitape.clues_keys.push(token),
                    "zos_export_definition" => emojitape.zos_export_definition.push(token),
                    "zos_export_implementation" => emojitape.zos_export_implementation.push(token),
                    "self_reproducing_footer" => emojitape.self_reproducing_footer.push(token),
                    _ => {},
                }
            }
        }
    }

    emojitape
}
