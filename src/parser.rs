// src/parser.rs

use crate::types::emojitape::Emojitape;
use crate::types::rule::Rule;
use crate::types::token::Token;

// Removed: use std::vec::IntoIter; // This was unused

pub fn parse_emojitape(tokens_vec: Vec<Token>) -> Emojitape {
    let mut tokens = tokens_vec.into_iter().peekable();
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
        expected_output: None, // Initialize new field
    };

    let mut current_section = "";
    let mut expected_output_buffer = String::new(); // Buffer for expected output

    while let Some(token) = tokens.next() {
        match &token {
            Token::Comment(comment_text) => {
                if comment_text.contains("---") {
                    // If we were in expected_output section, finalize it
                    if current_section == "expected_output" {
                        emojitape.expected_output = Some(expected_output_buffer.trim().to_string());
                        expected_output_buffer.clear();
                    }

                    if comment_text.contains("WASM COMPILER PRELUDE") {
                        current_section = "wasm_compiler_prelude";
                    }
                    else if comment_text.contains("PRELUDE") {
                        current_section = "prelude";
                    }
                    else if comment_text.contains("RULES") {
                        current_section = "rules";
                    }
                    else if comment_text.contains("WORLD TAPE") {
                        current_section = "world_tape";
                    }
                    else if comment_text.contains("GENERATED WAT BLOCK") {
                        current_section = "generated_wat_block";
                    }
                    else if comment_text.contains("CLUES & KEYS") {
                        current_section = "clues_keys";
                    }
                    else if comment_text.contains("/zos export definition") {
                        current_section = "zos_export_definition";
                    }
                    else if comment_text.contains("/zos export implementation") {
                        current_section = "zos_export_implementation";
                    }
                    else if comment_text.contains("SELF-REPRODUCING FOOTER") {
                        current_section = "self_reproducing_footer";
                    }
                    else if comment_text.contains("EXPECTED OUTPUT") { // New section
                        current_section = "expected_output";
                    }
                }
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
                        if matches!(token, Token::Newline | Token::Whitespace) {
                            continue;
                        }

                        if matches!(token, Token::RuleEntry) {
                            let mut pattern_tokens = Vec::new();
                            let mut replacement_tokens = Vec::new();
                            let mut parsing_pattern = true;

                            loop {
                                let next_token_opt = tokens.peek();

                                match next_token_opt {
                                    Some(peeked_token) => {
                                        if matches!(peeked_token, Token::RuleEntry) {
                                            break;
                                        }
                                        if let Token::Comment(comment_text) = peeked_token {
                                            if comment_text.contains("---") {
                                                break;
                                            }
                                        }

                                        if matches!(peeked_token, Token::Newline | Token::Whitespace) {
                                            tokens.next();
                                            continue;
                                        } else if matches!(peeked_token, Token::FuncStart) { // Changed from Token::Return
                                            parsing_pattern = false;
                                            tokens.next();
                                            continue;
                                        } else {
                                            let consumed_token = tokens.next().unwrap();
                                            if parsing_pattern {
                                                pattern_tokens.push(consumed_token);
                                            } else {
                                                replacement_tokens.push(consumed_token);
                                            }
                                        }
                                    },
                                    None => {
                                        break;
                                    }
                                }
                            }
                            emojitape.rules.push(Rule {
                                name: None,
                                pattern: pattern_tokens,
                                replacement: replacement_tokens,
                            });
                        } else {
                            emojitape.rules.push(Rule {
                                name: None,
                                pattern: vec![token.clone()],
                                replacement: Vec::new(),
                            });
                        }
                    },
                    "world_tape" => {
                        emojitape.world_tape.push(token.clone())
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
                    "expected_output" => {
                        let s = match token {
                            Token::Integer(i) => i.to_string(),
                            Token::Float(f) => f.to_string(),
                            Token::Word(w) => w.clone(),
                            Token::Newline => "\n".to_string(),
                            Token::Whitespace => " ".to_string(),
                            // Use the default to_string for other tokens, which might be the desired behavior
                            _ => token.to_string(),
                        };
                        expected_output_buffer.push_str(&s);
                    },
                    _ => {},
                }
            }
        }
    }

    // Finalize expected_output if it was the last section
    if current_section == "expected_output" && !expected_output_buffer.is_empty() {
        emojitape.expected_output = Some(expected_output_buffer.trim().to_string());
    }

    emojitape
}

#[cfg(test)]
mod tests;
