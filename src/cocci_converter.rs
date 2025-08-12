// src/cocci_converter.rs

use crate::types::token::Token;
use crate::cocci_parser::{self, CocciElement};
use crate::rust_to_emoji;

pub fn cocci_to_emojis(cocci_content: &str) -> Result<Vec<Token>, String> {
    println!("Converting .cocci to emojis...");

    let mut tokens: Vec<Token> = Vec::new();
    let cocci_elements = cocci_parser::parse_cocci_content(cocci_content);

    for element in cocci_elements {
        match element {
            CocciElement::RuleHeader(name) => {
                tokens.push(Token::Comment(format!("COC_RULE_START:{}", name)));
            },
            CocciElement::Separator => {
                tokens.push(Token::Comment("COC_SEPARATOR".to_string()));
            },
            CocciElement::Identifier(id) => {
                tokens.push(Token::Comment(format!("COC_IDENTIFIER:{}", id)));
            },
            CocciElement::AddLine(code) => {
                tokens.push(Token::Comment(format!("COC_ADD_LINE_START:{}", code)));
                match rust_to_emoji::rust_code_to_emojis(&code) {
                    Ok(mut emoji_code_tokens) => {
                        tokens.append(&mut emoji_code_tokens);
                    },
                    Err(e) => {
                        tokens.push(Token::Comment(format!("RUST_TO_EMOJI_ERROR:{}", e)));
                    }
                }
                tokens.push(Token::Comment("COC_ADD_LINE_END".to_string()));
            },
            CocciElement::RemoveLine(code) => {
                tokens.push(Token::Comment(format!("COC_REMOVE_LINE_START:{}", code)));
                match rust_to_emoji::rust_code_to_emojis(&code) {
                    Ok(mut emoji_code_tokens) => {
                        tokens.append(&mut emoji_code_tokens);
                    },
                    Err(e) => {
                        tokens.push(Token::Comment(format!("RUST_TO_EMOJI_ERROR:{}", e)));
                    }
                }
                tokens.push(Token::Comment("COC_REMOVE_LINE_END".to_string()));
            },
            CocciElement::Comment(text) => {
                tokens.push(Token::Comment(format!("COC_COMMENT:{}", text)));
            },
            CocciElement::ContextLine(code) => {
                tokens.push(Token::Comment(format!("COC_CONTEXT_LINE_START:{}", code)));
                match rust_to_emoji::rust_code_to_emojis(&code) {
                    Ok(mut emoji_code_tokens) => {
                        tokens.append(&mut emoji_code_tokens);
                    },
                    Err(e) => {
                        tokens.push(Token::Comment(format!("RUST_TO_EMOJI_ERROR:{}", e)));
                    }
                }
                tokens.push(Token::Comment("COC_CONTEXT_LINE_END".to_string()));
            },
        }
    }

    Ok(tokens)
}

pub fn emojis_to_cocci(emoji_tokens: &[Token]) -> Result<String, String> {
    // Placeholder for converting emoji tokens back to .cocci content
    // This will involve:
    // 1. Mapping emoji tokens back to SmPL constructs.
    // 2. Reconstructing the .cocci file format.

    println!("Converting emojis to .cocci...");
    println!("Emoji tokens: {:?}", emoji_tokens);

    let mut cocci_content = String::new();

    // Example: Simple reconstruction
    cocci_content.push_str("@generated_cocci_rule@\n");
    cocci_content.push_str("identifier some_id;\n");
    cocci_content.push_str("@@\n");

    // TODO: Implement actual reconstruction logic here

    Ok(cocci_content)
}

