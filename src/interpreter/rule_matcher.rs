use crate::types::token::Token;
use crate::types::rule::Rule;

pub fn match_and_replace(target: &[Token], rule: &Rule) -> Vec<Token> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < target.len() {
        if i + rule.pattern.len() <= target.len() && target[i..i + rule.pattern.len()] == rule.pattern[..] {
            // Match found, append replacement
            result.extend_from_slice(&rule.replacement);
            i += rule.pattern.len(); // Advance past the matched pattern
        } else {
            // No match, append current token
            result.push(target[i].clone());
            i += 1;
        }
    }
    result
}
