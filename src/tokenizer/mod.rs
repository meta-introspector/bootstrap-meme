use crate::types::token::Token;
use crate::types::token::emojis;
use std::collections::HashMap;

fn build_token_map() -> HashMap<&'static str, Token> {
    let mut m = HashMap::new();
    // Prelude
    m.insert(emojis::true_token::EMOJI, emojis::true_token::to_token());
    m.insert(emojis::false_token::EMOJI, emojis::false_token::to_token());
    m.insert(emojis::func_start_token::EMOJI, emojis::func_start_token::to_token());
    m.insert(emojis::forall_token::EMOJI, emojis::forall_token::to_token());
    m.insert(emojis::exists_token::EMOJI, emojis::exists_token::to_token());
    m.insert(emojis::up_arrow_token::EMOJI, emojis::up_arrow_token::to_token());
    m.insert(emojis::and_token::EMOJI, emojis::and_token::to_token());
    m.insert(emojis::or_token::EMOJI, emojis::or_token::to_token());
    m.insert(emojis::not_token::EMOJI, emojis::not_token::to_token());
    m.insert(emojis::implies_token::EMOJI, emojis::implies_token::to_token());
    m.insert(emojis::iff_token::EMOJI, emojis::iff_token::to_token());
    m.insert(emojis::s_token::EMOJI, emojis::s_token::to_token());
    m.insert(emojis::k_token::EMOJI, emojis::k_token::to_token());
    m.insert(emojis::i_token::EMOJI, emojis::i_token::to_token());
    m.insert(emojis::sparkle_token::EMOJI, emojis::sparkle_token::to_token());
    m.insert(emojis::lightning_token::EMOJI, emojis::lightning_token::to_token());
    m.insert(emojis::b_token::EMOJI, emojis::b_token::to_token());
    m.insert(emojis::c_token::EMOJI, emojis::c_token::to_token());
    m.insert(emojis::w_token::EMOJI, emojis::w_token::to_token());
    m.insert(emojis::y_token::EMOJI, emojis::y_token::to_token());
    m.insert(emojis::z_token::EMOJI, emojis::z_token::to_token());
    m.insert(emojis::omega_token::EMOJI, emojis::omega_token::to_token());
    m.insert(emojis::lambda_token::EMOJI, emojis::lambda_token::to_token());
    m.insert(emojis::top_token::EMOJI, emojis::top_token::to_token());
    m.insert(emojis::bottom_token::EMOJI, emojis::bottom_token::to_token());
    m.insert(emojis::maps_to_token::EMOJI, emojis::maps_to_token::to_token());
    m.insert(emojis::compose_token::EMOJI, emojis::compose_token::to_token());
    m.insert(emojis::equals_token::EMOJI, emojis::equals_token::to_token());
    m.insert(emojis::not_equals_token::EMOJI, emojis::not_equals_token::to_token());
    m.insert(emojis::identical_token::EMOJI, emojis::identical_token::to_token());
    m.insert(emojis::proves_token::EMOJI, emojis::proves_token::to_token());
    m.insert(emojis::entails_token::EMOJI, emojis::entails_token::to_token());

    // WASM Compiler Prelude (conceptual)
    m.insert(emojis::compiler_token::EMOJI, emojis::compiler_token::to_token());
    m.insert(emojis::optimizer_token::EMOJI, emojis::optimizer_token::to_token());
    m.insert(emojis::box_token::EMOJI, emojis::box_token::to_token()); // Re-used for emit_wat_block
    m.insert(emojis::check_trap_token::EMOJI, emojis::check_trap_token::to_token());

    // Rules (emoji -> token mapping)
    m.insert(emojis::return_token::EMOJI, emojis::return_token::to_token());
    m.insert(emojis::call_token::EMOJI, emojis::call_token::to_token());
    m.insert(emojis::local_get_token::EMOJI, emojis::local_get_token::to_token());
    m.insert(emojis::local_set_token::EMOJI, emojis::local_set_token::to_token());
    m.insert(emojis::spawn_token_token::EMOJI, emojis::spawn_token_token::to_token());
    m.insert(emojis::emit_wat_block_token::EMOJI, emojis::emit_wat_block_token::to_token());
    m.insert(emojis::rule_entry_token::EMOJI, emojis::rule_entry_token::to_token()); // This will need special handling
    m.insert(emojis::apply_rules_loop_token::EMOJI, emojis::apply_rules_loop_token::to_token());

    // World Tape
    m.insert(emojis::add_token::EMOJI, emojis::add_token::to_token());
    m.insert(emojis::sub_token::EMOJI, emojis::sub_token::to_token());
    m.insert(emojis::mul_token::EMOJI, emojis::mul_token::to_token());
    m.insert(emojis::div_s_token::EMOJI, emojis::div_s_token::to_token());
    m.insert(emojis::gt_s_token::EMOJI, emojis::gt_s_token::to_token());
    m.insert(emojis::zos_export_token::EMOJI, emojis::zos_export_token::to_token());
    m.insert(emojis::zos_ready_token::EMOJI, emojis::zos_ready_token::to_token());

    m
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let token_map = build_token_map();

    while let Some(c) = chars.peek() {
        // Handle comments
        if *c == '💬' || *c == '#' {
            let _comment_char = chars.next().unwrap(); // Consume 💬 or #
            let mut comment_content = String::new();
            while let Some(cc) = chars.next() {
                if cc == '\n' {
                    tokens.push(Token::Comment(comment_content.clone())); // Clone here
                    tokens.push(Token::Newline);
                    comment_content.clear(); // Clear for next potential comment
                    break;
                } else {
                    comment_content.push(cc);
                }
            }
            if comment_content.ends_with('\r') {
                comment_content.pop(); // Remove trailing carriage return if present
            }
            if chars.peek().is_none() && !comment_content.is_empty() {
                tokens.push(Token::Comment(comment_content));
            }
            continue;
        }

        // Handle newlines
        if *c == '\n' {
            chars.next();
            tokens.push(Token::Newline);
            continue;
        }

        // Handle whitespace
        if c.is_whitespace() {
            chars.next();
            tokens.push(Token::Whitespace);
            continue;
        }

        // Handle numbers (integers and floats)
        if c.is_ascii_digit() {
            let mut num_str = String::new();
            while let Some(&nc) = chars.peek() {
                if nc.is_ascii_digit() || nc == '.' {
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if num_str.contains('.') {
                if let Ok(f) = num_str.parse::<f32>() {
                    tokens.push(Token::Float(f));
                } else {
                    tokens.push(Token::Word(num_str)); // Fallback to word if parsing fails
                }
            } else {
                if let Ok(i) = num_str.parse::<i32>() {
                    tokens.push(Token::Integer(i));
                } else {
                    tokens.push(Token::Word(num_str)); // Fallback to word if parsing fails
                }
            }
            continue;
        }

        // Handle known emoji tokens (longest match first)
        let mut matched_emoji = false;
        let remaining: String = chars.clone().collect();
        for (emoji_str, token_type) in token_map.iter() {
            if remaining.starts_with(emoji_str) {
                for _ in 0..emoji_str.len() {
                    chars.next();
                }
                tokens.push(token_type.clone());
                matched_emoji = true;
                break; // Found the longest match, move to next character
            }
        }
        if matched_emoji {
            continue;
        }

        // Handle words and other characters
        let mut word_str = String::new();
        while let Some(&wc) = chars.peek() {
            // Stop if it's a known token start, whitespace, or newline
            if wc.is_whitespace() || wc == '\n' || wc == '💬' || wc == '#' || wc.is_ascii_digit() || token_map.contains_key(wc.to_string().as_str()) {
                break;
            }
            word_str.push(chars.next().unwrap());
        }

        if !word_str.is_empty() {
            tokens.push(Token::Word(word_str));
        } else {
            // If nothing else matched, it's an 'Other' token (single character)
            tokens.push(Token::Other(chars.next().unwrap().to_string()));
        }
    }

    tokens
}
