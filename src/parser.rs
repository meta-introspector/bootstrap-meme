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
                        let mut current_rule_pattern: Vec<Token> = Vec::new();
                        let mut current_rule_replacement: Vec<Token> = Vec::new();
                        let mut parsing_pattern = true;

                        // Loop until the end of the rules section or end of tokens
                        loop {
                            // Peek the next token to decide what to do
                            let peeked_token_opt = tokens.peek();

                            match peeked_token_opt {
                                Some(peeked_token) => {
                                    // Check for end of section
                                    if let Token::Comment(comment_text) = peeked_token {
                                        if comment_text.contains("---") {
                                            // If we have a partial rule, push it before breaking
                                            if !current_rule_pattern.is_empty() || !current_rule_replacement.is_empty() {
                                                emojitape.rules.push(Rule {
                                                    name: None,
                                                    pattern: current_rule_pattern.clone(),
                                                    replacement: current_rule_replacement.clone(),
                                                });
                                            }
                                            break; // End of rules section
                                        }
                                    }

                                    // Check for RuleEntry (🍃) - signifies a new rule
                                    if matches!(peeked_token, Token::RuleEntry) {
                                        // If we have a partial rule, push it before starting a new one
                                        if !current_rule_pattern.is_empty() || !current_rule_replacement.is_empty() {
                                            emojitape.rules.push(Rule {
                                                name: None,
                                                pattern: current_rule_pattern.clone(),
                                                replacement: current_rule_replacement.clone(),
                                            });
                                            current_rule_pattern.clear();
                                            current_rule_replacement.clear();
                                            parsing_pattern = true;
                                        }
                                        tokens.next(); // Consume RuleEntry
                                        continue; // Continue to parse the next token as part of the new rule
                                    }

                                    // Handle separator (->)
                                    if matches!(peeked_token, Token::FuncStart) {
                                        parsing_pattern = false;
                                        tokens.next(); // Consume FuncStart
                                        continue;
                                    }

                                    // Consume the token and add to pattern or replacement
                                    let consumed_token = tokens.next().unwrap();
                                    if parsing_pattern {
                                        current_rule_pattern.push(consumed_token);
                                    } else {
                                        current_rule_replacement.push(consumed_token);
                                    }
                                },
                                None => {
                                    // End of tokens - push any partial rule and break
                                    if !current_rule_pattern.is_empty() || !current_rule_replacement.is_empty() {
                                        emojitape.rules.push(Rule {
                                            name: None,
                                            pattern: current_rule_pattern.clone(),
                                            replacement: current_rule_replacement.clone(),
                                        });
                                    }
                                    break; // End of tokens
                                }
                            }
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
