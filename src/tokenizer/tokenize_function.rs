use crate::types::token::Token;
use crate::types::token::emojis;
use crate::tokenizer::TOKEN_MAP; // Import TOKEN_MAP from tokenizer module

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < input.len() {
        let remaining_slice = &input[i..];

        // 1. Comments
        if remaining_slice.starts_with("💬") || remaining_slice.starts_with("#") {
            let start_char_len = if remaining_slice.starts_with("💬") { "💬".len() } else { "#".len() };
            i += start_char_len;
            let comment_end = remaining_slice[start_char_len..].find(emojis::newline_token::ASCII_EQUIVALENT).unwrap_or(remaining_slice[start_char_len..].len());
            let comment_content = &remaining_slice[start_char_len..start_char_len + comment_end];
            tokens.push(Token::Comment(comment_content.trim().to_string()));
            i += comment_end;
            continue;
        }

        // 2. Newlines
        if remaining_slice.starts_with(emojis::newline_token::ASCII_EQUIVALENT) {
            i += emojis::newline_token::ASCII_EQUIVALENT.len();
            tokens.push(Token::Newline);
            continue;
        }

        // 3. Whitespace
        if remaining_slice.chars().next().unwrap().is_whitespace() {
            i += remaining_slice.chars().next().unwrap().len_utf8();
            tokens.push(Token::Whitespace);
            continue;
        }

        // 4. Longest Emoji Match
        let mut longest_match_emoji: Option<&'static str> = None;
        let mut longest_match_token_variant: Option<Token> = None; // Store the actual token variant

        for (emoji_str, token_type) in TOKEN_MAP.iter() {
            if remaining_slice.starts_with(emoji_str)
                && (longest_match_emoji.is_none() || emoji_str.len() > longest_match_emoji.unwrap().len()) {
                    longest_match_emoji = Some(emoji_str);
                    longest_match_token_variant = Some(token_type.clone()); // Clone the token
                }
        }

        if let Some(emoji_str) = longest_match_emoji {
            i += emoji_str.len();
            let matched_token = longest_match_token_variant.unwrap();

            match matched_token {
                Token::I32Const(_) => { // 📏
                    // Expect an integer immediately after
                    let (num_token, len) = parse_number_from_slice(&input[i..]);
                    if let Some(Token::Integer(val)) = num_token {
                        tokens.push(Token::I32Const(val));
                        i += len;
                    } else {
                        // If no valid integer follows, push I32Const(0) as is
                        tokens.push(Token::I32Const(0));
                    }
                },
                Token::F32Const(_) => { // 📐
                    // Expect a float immediately after
                    let (num_token, len) = parse_number_from_slice(&input[i..]);
                    if let Some(Token::Float(val)) = num_token {
                        tokens.push(Token::F32Const(val));
                        i += len;
                    } else {
                        // If no valid float follows, push F32Const(0.0) as is
                        tokens.push(Token::F32Const(0.0));
                    }
                },
                _ => {
                    tokens.push(matched_token);
                }
            }
            continue;
        }

        // Helper function to parse numbers from a slice
        fn parse_number_from_slice(s: &str) -> (Option<Token>, usize) {
            let mut current_len = 0;
            let mut has_decimal = false;
            for ch in s.chars() {
                if ch.is_ascii_digit() {
                    current_len += ch.len_utf8();
                } else if ch == '.' && !has_decimal {
                    has_decimal = true;
                    current_len += ch.len_utf8();
                } else {
                    break;
                }
            }
            if current_len > 0 {
                let num_str = &s[..current_len];
                if has_decimal {
                    if let Ok(f) = num_str.parse::<f32>() {
                        return (Some(Token::Float(f)), current_len);
                    }
                } else if let Ok(i_val) = num_str.parse::<i32>() {
                    return (Some(Token::Integer(i_val)), current_len);
                }
            }
            (None, 0)
        }

        // 5. Numbers (only if not prefixed by Sparkle/Lightning)
        let (num_token, len) = parse_number_from_slice(remaining_slice);
        if let Some(token) = num_token {
            tokens.push(token);
            i += len;
            continue;
        }

        // 6. Words
        if remaining_slice.chars().next().unwrap().is_alphanumeric() {
            let mut current_len = 0;
            for ch in remaining_slice.chars() {
                if ch.is_alphanumeric() {
                    current_len += ch.len_utf8();
                } else {
                    break;
                }
            }
            let word_str = &remaining_slice[..current_len];
            tokens.push(Token::Word(word_str.to_string()));
            i += current_len;
            continue;
        }

        // 7. Other (single character fallback)
        let ch = remaining_slice.chars().next().unwrap();
        tokens.push(Token::Other(ch.to_string()));
        i += ch.len_utf8();
    }

    tokens
}
